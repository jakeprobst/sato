use std::collections::HashMap;
use std::convert::{From, Into};

use crate::context::{ContextValue, RenderContext};
use crate::template::{Template, TemplateExprNode};
use crate::builtins;

type NodeHandler = dyn Fn(&Attributes, &[&TemplateExprNode], &Renderer, &RenderContext) -> Result<Vec<String>, RenderError>;

static VAR_REGEX: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(r#"\$[a-zA-Z0-9_\-\.]+"#).unwrap()
});


#[derive(Debug, Clone)]
pub struct Attribute(pub String, pub String);

#[derive(Debug, Clone)]
pub struct Attributes(Vec<Attribute>);


impl Attributes {
    pub fn new(attrs: Vec<Attribute>) -> Attributes {
        Attributes(attrs)
    }

    pub fn push(&mut self, attr: Attribute) {
        self.0.push(attr)
    }

    pub fn get(&self, name: &str) -> Option<&String> {
        self.0
            .iter()
            .find(|a| a.0 == name)
            .map(|a| &a.1)
    }
}

impl<'a> IntoIterator for &'a Attributes {
    type Item = &'a Attribute;
    type IntoIter = std::slice::Iter<'a, Attribute>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}



#[derive(thiserror::Error, Debug)]
pub enum RenderError {
    #[error("expected a variable, found {0}")]
    ExpectedVariable(String),
    #[error("error expanding variable: {0} ({1:?})")]
    ExpandVariable(String, String),
    #[error("error in `is-set`: {0} ({1:?})")]
    IsSet(String, Vec<TemplateExprNode>),
    #[error("error in `eq`: {0} ({1:?})")]
    Eq(String, Vec<TemplateExprNode>),
    #[error("error in `if`: {0} ({1:?})")]
    If(String, Vec<TemplateExprNode>),
    #[error("error in `case`: {0} ({1:?})")]
    Case(String, Vec<TemplateExprNode>),
    #[error("error in `switch`: {0} ({1:?})")]
    Switch(String, Vec<TemplateExprNode>),
    #[error("error in `for`: {0} {1:?} ({2:?})")]
    For(String, Attributes, Vec<TemplateExprNode>),

    #[error("error in `if`: {0} ({1:?})")]
    Mod(String, Vec<TemplateExprNode>),

    #[error("error in `{0}`: {1} ({1:?})")]
    UserDefined(String, String, Vec<TemplateExprNode>),
}

pub struct Renderer {
    functions: HashMap<String, Box<NodeHandler>>,
}

// stolen from https://github.com/rust-lang/regex/issues/648#issuecomment-590072186
// to get around the default replace_all not handlong errors
fn replace_all<E>(re: &regex::Regex, haystack: &str, replacement: impl Fn(&regex::Captures) -> Result<String, E>) -> Result<String, E> {
    let mut new = String::with_capacity(haystack.len());
    let mut last_match = 0;
    for caps in re.captures_iter(haystack) {
        let m = caps.get(0).unwrap();
        new.push_str(&haystack[last_match..m.start()]);
        new.push_str(&replacement(&caps)?);
        last_match = m.end();
    }
    new.push_str(&haystack[last_match..]);
    Ok(new)
}

fn context_to_string(cvalue: &ContextValue, renderer: &Renderer, context: &RenderContext) -> Result<String, RenderError> {
    Ok(match cvalue {
        ContextValue::String(s) => s.clone(),
        ContextValue::Integer(i) => i.to_string(),
        ContextValue::Template(t) => renderer.render(t, context)?,
        ContextValue::Vec(v) => {
            v.iter()
                .map(|e| match e {
                    ContextValue::String(s) => renderer.evaluate_string(s, context),
                    ContextValue::Integer(i) => Ok(i.to_string()),
                    ContextValue::Template(t) => renderer.render(t, context),
                    ContextValue::Vec(_) => context_to_string(e, renderer, context),
                    _ => Ok(String::new())
                })
                .collect::<Result<Vec<_>, _>>()?
                .join("")
        },
        _ => String::new()
    })
}

// TODO: stop being lazy and not use a regex
pub(crate) fn expand_variable(expr: String, renderer: &Renderer, context: &RenderContext) -> Result<String, RenderError> {
    replace_all(&VAR_REGEX, &expr, |c: &regex::Captures| {
        let variable = c
            .get(0)
            .ok_or_else(|| RenderError::ExpandVariable("could not get variable".into(), expr.clone()))?
            .as_str()
            .to_string();
        Ok(if variable.contains(".") {
            let elements = variable
                .get(1..)
                .ok_or_else(|| RenderError::ExpandVariable("invalid variable name".into(), expr.clone()))?
                .split('.')
                .collect::<Vec<_>>();
            let obj = elements
                .get(0)
                .ok_or_else(|| RenderError::ExpandVariable("could not find object name".into(), expr.clone()))?;
            let vars = elements
                .get(1..)
                .ok_or_else(|| RenderError::ExpandVariable("could not get object variable".into(), expr.clone()))?;

            // TODO: fold?
            let mut sub_context = context.0.get(*obj);
            for var in vars.iter() {
                match sub_context {
                    Some(ContextValue::Object(o)) => {
                        sub_context = o.0.get::<str>(var)
                    },
                    _ => return Ok(variable),
                }
            }
            sub_context
                .and_then(|c| context_to_string(c, renderer, context).ok())
                .unwrap_or(variable)
        }
        else {
            context.0.get(&variable[1..])
                .and_then(|c| context_to_string(c, renderer, context).ok())
                .unwrap_or(variable)
        })
    })
}

pub(crate) fn basic_html_tag(tag: String, attrs: &Attributes, expr: &[&TemplateExprNode], renderer: &Renderer, context: &RenderContext) -> Result<Vec<String>, RenderError> {
    let mut l = Vec::new();
    let attr_str = attrs.0.iter()
        .map(|attr| {
            let key = renderer.evaluate_string(&attr.0, context)?;
            let value = renderer.evaluate_string(&attr.1, context)?;
            Ok(format!(" {}=\"{}\"", key, value))
        })
        .collect::<Result<Vec<_>, RenderError>>()?
        .join("");
    if expr.len() == 0 {
        l.push(format!("<{}{} />", tag, attr_str));
    }
    else {
        l.push(format!("<{}{}>", tag, attr_str));
        l.append(&mut renderer.evaluate_multiple(expr, context)?);
        l.push(format!("</{}>", tag));
    }
    Ok(l)
}


fn standard_issue_functions() -> HashMap<String, Box<NodeHandler>> {
    let mut functions = HashMap::new();
    functions.insert("html".into(), Box::new(builtins::do_html) as Box<NodeHandler>);
    functions.insert("is-set".into(), Box::new(builtins::do_is_set) as Box<NodeHandler>);
    functions.insert("if".into(), Box::new(builtins::do_if) as Box<NodeHandler>);
    functions.insert("switch".into(), Box::new(builtins::do_switch) as Box<NodeHandler>);
    functions.insert("case".into(), Box::new(builtins::do_case) as Box<NodeHandler>);
    functions.insert("for".into(), Box::new(builtins::do_for) as Box<NodeHandler>);
    functions.insert("eq".into(), Box::new(builtins::do_eq) as Box<NodeHandler>);
    functions.insert("+".into(), Box::new(builtins::do_add) as Box<NodeHandler>);
    functions.insert("-".into(), Box::new(builtins::do_sub) as Box<NodeHandler>);
    functions.insert("*".into(), Box::new(builtins::do_mul) as Box<NodeHandler>);
    functions.insert("/".into(), Box::new(builtins::do_div) as Box<NodeHandler>);
    functions.insert("%".into(), Box::new(builtins::do_mod) as Box<NodeHandler>);
    functions
}

impl Renderer {
    pub fn builder() -> RendererBuilder {
        RendererBuilder::new()
    }

    pub fn evaluate_multiple(&self, expr: &[&TemplateExprNode], context: &RenderContext) -> Result<Vec<String>, RenderError> {
        Ok(expr
            .iter()
            .map(|e| self.evaluate(e, context))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect())
    }

    pub fn evaluate(&self, expr: &TemplateExprNode, context: &RenderContext) -> Result<Vec<String>, RenderError> {
        Ok(match expr {
            TemplateExprNode::Identifier(ident) => {
                vec![expand_variable(ident.clone(), self, context)?]
            },
            TemplateExprNode::Integer(i) => {
                vec![i.to_string()]
            },
            TemplateExprNode::Tag(tag) => {
                match self.functions.get(&tag.tag) {
                    Some(op_func) => op_func(&tag.attrs, &tag.children.iter().collect::<Vec<_>>(), self, context)?,
                    None => basic_html_tag(tag.tag.clone(), &tag.attrs, &tag.children.iter().collect::<Vec<_>>(), self, context)?,
                }
            },
        })
    }

    pub fn evaluate_string(&self, expr: &String, context: &RenderContext) -> Result<String, RenderError> {
        Ok(
            if let Ok(expr) = TemplateExprNode::try_from(expr.clone()) {
                self.evaluate(&expr, context)?.join("")
            }
            else {
                expand_variable(expr.clone(), self, context)?
            })
    }

    pub fn render(&self, template: &Template, context: &RenderContext) -> Result<String, RenderError> {
        Ok(Vec::from(self.evaluate(&template.expr, context)?).join(""))
    }
}


pub struct RendererBuilder {
    functions: HashMap<String, Box<NodeHandler>>,
}

impl RendererBuilder {
    fn new() -> Self {
        RendererBuilder {
            functions: standard_issue_functions(),
        }
    }

    pub fn function<S>(mut self, name: S, func: Box<NodeHandler>) -> Self
    where
        S: std::convert::Into<String>
    {
        self.functions.insert(name.into(), func);
        self
    }

    pub fn build(self) -> Renderer {
        Renderer {
            functions: self.functions,
        }
    }
}

