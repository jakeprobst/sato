use std::collections::HashMap;
use std::convert::{From, Into};

use crate::context::{ContextValue, RenderContext};
use crate::template::{Template, TemplateExprNode, TemplateAttribute};
use crate::builtins;

type NodeHandler = dyn for<'a> Fn(Attributes, &[TemplateExprNode], &'a Renderer, &'a RenderContext) -> Result<RenderValue, RenderError> + Send + Sync;

#[derive(Debug, Clone)]
pub enum RenderValue {
    String(String),
    Integer(i64),
    Boolean(bool),
    Vec(Vec<RenderValue>),
    Object(HashMap<String, RenderValue>),
    Template(Template),
    Empty,
}

impl RenderValue {
    pub fn finalize(self) -> String {
        match self {
            RenderValue::String(s) => s,
            RenderValue::Integer(i) => i.to_string(),
            RenderValue::Boolean(b) => b.to_string(),
            RenderValue::Vec(v) => v.into_iter().map(|e| e.finalize()).collect::<Vec<_>>().join(""),
            RenderValue::Object(o) => o.into_iter().map(|(_k, v)| v.finalize()).collect::<Vec<_>>().join(""),
            RenderValue::Template(_t) => "".into(),
            RenderValue::Empty => "".into(),
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            RenderValue::String(s) => Some(s),
            _ => None
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            RenderValue::Integer(i) => Some(*i),
            _ => None
        }
    }

    pub fn join(&self) -> RenderValue {
        match self {
            RenderValue::String(_) => self.clone(),
            RenderValue::Integer(_) => self.clone(),
            RenderValue::Boolean(_) => self.clone(),
            RenderValue::Vec(v) => RenderValue::String(v.iter().map(|e| e.clone().finalize()).collect::<Vec<_>>().join("")),
            RenderValue::Object(o) => RenderValue::String(o.iter().map(|(_k, v)| v.clone().finalize()).collect::<Vec<_>>().join("")),
            RenderValue::Template(_) => self.clone(),
            RenderValue::Empty => self.clone(),
        }
    }
}

impl From<&str> for RenderValue {
    fn from(other: &str) -> Self {
        RenderValue::String(other.into())
    }
}

impl From<String> for RenderValue {
    fn from(other: String) -> Self {
        RenderValue::String(other)
    }
}

impl<T: Into<RenderValue>> From<Vec<T>> for RenderValue {
    fn from(other: Vec<T>) -> Self {
        RenderValue::Vec(other.into_iter().map(|k| k.into()).collect())
    }
}

impl From<i64> for RenderValue {
    fn from(other: i64) -> Self {
        RenderValue::Integer(other)
    }
}

impl From<bool> for RenderValue {
    fn from(other: bool) -> Self {
        RenderValue::Boolean(other)
    }
}

impl From<&ContextValue> for RenderValue {
    fn from(other: &ContextValue) -> Self {
        match other {
            ContextValue::Integer(i) => RenderValue::Integer(*i),
            ContextValue::Boolean(b) => RenderValue::Boolean(*b),
            ContextValue::String(s) => RenderValue::String(s.clone()),
            ContextValue::Vec(v) => RenderValue::Vec(v.iter().map(|e| RenderValue::from(e)).collect::<Vec<_>>()),
            ContextValue::Object(o) => {
                RenderValue::Object(o.0.iter()
                                       .map(|(k, v)| (k.clone(), RenderValue::from(v)))
                                       .collect::<HashMap<String, RenderValue>>())
            },
            ContextValue::Template(t) => RenderValue::Template(t.clone()),
        }
    }
}

impl PartialEq for RenderValue {
    fn eq(&self, other: &RenderValue) -> bool {
        match (self, other) {
            (RenderValue::Integer(a), RenderValue::Integer(b)) => a == b,
            (RenderValue::Boolean(a), RenderValue::Boolean(b)) => a == b,
            (RenderValue::String(a), RenderValue::String(b)) => a == b,
            (RenderValue::Vec(a), RenderValue::Vec(b)) => a == b,
            _ => false,
        }
    }
}

impl PartialOrd for RenderValue {
    fn partial_cmp(&self, other: &RenderValue) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (RenderValue::Integer(a), RenderValue::Integer(b)) => a.partial_cmp(b),
            (RenderValue::Boolean(a), RenderValue::Boolean(b)) => a.partial_cmp(b),
            (RenderValue::String(a), RenderValue::String(b)) => a.partial_cmp(b),
            (RenderValue::Vec(a), RenderValue::Vec(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}


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
    Cmp(String, Vec<TemplateExprNode>),
    #[error("error in `if`: {0} ({1:?})")]
    If(String, Vec<TemplateExprNode>),
    #[error("error in `case`: {0} ({1:?})")]
    Case(String, Vec<TemplateExprNode>),
    #[error("error in `switch`: {0} ({1:?})")]
    Switch(String, Vec<TemplateExprNode>),
    #[error("error in `for`: {0} {1:?} ({2:?})")]
    For(String, Attributes, Vec<TemplateExprNode>),
    #[error("error in `get`: {0} {1:?}")]
    Get(String, Vec<TemplateExprNode>),

    #[error("error in math operator: {0} ({1:?})")]
    Math(String, Vec<TemplateExprNode>),

    #[error("error in `{0}`: {1} ({2:?})")]
    UserDefined(String, String, Vec<TemplateExprNode>),

    #[error("error in `eval`: {0}")]
    Evaluate(String),

}

pub struct Renderer {
    functions: HashMap<String, Box<NodeHandler>>,
}

pub(crate) fn expand_variable(expr: &String, renderer: &Renderer, context: &RenderContext) -> Result<RenderValue, RenderError> {
    Ok(
        if expr.starts_with('$') {
            if expr.contains(".") {
                expr[1..].split('.').try_fold((context.clone(), None), |(mut context, output), expr| {
                    if output.is_some() {
                        return Ok((context, output))
                    }

                    match context.get(expr) {
                        Some(ContextValue::Object(o)) => {
                            context = o.clone();
                            Ok((context, output))
                        },
                        Some(item) => {
                            let item = item.into();
                            Ok((context, Some(item)))
                        },
                        None => Ok((context, Some(RenderValue::Boolean(false))))
                    }
                })?
                    .1
                    .unwrap_or_else(|| expr.clone().into())
            }
            else {
                context.get(&expr[1..])
                    .map(|e| e.try_into())
                    .unwrap_or(Ok(RenderValue::String(expr.clone())))
                    .map(|e| {
                        match e {
                            RenderValue::Vec(v) => {
                                Ok(RenderValue::Vec(v.iter()
                                                    .map(|v| {
                                                        match v {
                                                            RenderValue::String(s) => expand_variable(s, renderer, context),
                                                            _ => Ok(v.clone())
                                                        }
                                                    })
                                                    .collect::<Result<Vec<_>, _>>()?))
                            },
                            RenderValue::Template(t) => {
                                Ok(RenderValue::String(renderer.render(&t, context)?))
                            }
                            _ => Ok(e)
                        }
                    })
                    .unwrap_or(Ok(RenderValue::String(expr.clone())))?
            }
        }
        else {
            RenderValue::String(expr.clone())
        }
    )
}

pub(crate) fn basic_html_tag(tag: String, attrs: &Attributes, expr: &[TemplateExprNode], renderer: &Renderer, context: &RenderContext) -> Result<RenderValue, RenderError> {
    let mut l = Vec::<RenderValue>::new();
    let attr_str = attrs.0.iter()
        .map(|attr| {
            Ok(format!(" {}=\"{}\"", attr.0, attr.1))
        })
        .collect::<Result<Vec<_>, RenderError>>()?
        .join("");
    if expr.len() == 0 {
        l.push(format!("<{}{} />", tag, attr_str).into());
    }
    else {
        l.push(format!("<{}{}>", tag, attr_str).into());
        l.push(renderer.evaluate_multiple(expr, context)?.into());
        l.push(format!("</{}>", tag).into());
    }
    Ok(l.into())
}


fn standard_issue_functions() -> HashMap<String, Box<NodeHandler>> {
    let mut functions = HashMap::new();
    functions.insert("html".into(), Box::new(builtins::do_html) as Box<NodeHandler>);
    functions.insert("is-set".into(), Box::new(builtins::do_is_set));
    functions.insert("if".into(), Box::new(builtins::do_if));
    functions.insert("switch".into(), Box::new(builtins::do_switch));
    functions.insert("case".into(), Box::new(builtins::do_case));
    functions.insert("for".into(), Box::new(builtins::do_for));
    functions.insert("get".into(), Box::new(builtins::do_get));

    functions.insert("eq".into(), Box::new(|a,e,r,c| builtins::do_cmp_op(a,e,r,c, |q, w| q == w)));
    functions.insert("lt".into(), Box::new(|a,e,r,c| builtins::do_cmp_op(a,e,r,c, |q, w| q < w)));
    functions.insert("gt".into(), Box::new(|a,e,r,c| builtins::do_cmp_op(a,e,r,c, |q, w| q > w)));
    functions.insert("lte".into(), Box::new(|a,e,r,c| builtins::do_cmp_op(a,e,r,c, |q, w| q <= w)));
    functions.insert("gte".into(), Box::new(|a,e,r,c| builtins::do_cmp_op(a,e,r,c, |q, w| q >= w)));
    functions.insert("ne".into(), Box::new(|a,e,r,c| builtins::do_cmp_op(a,e,r,c, |q, w| q != w)));

    functions.insert("+".into(), Box::new(|a,e,r,c| builtins::do_math_op(a,e,r,c, |q, w| q + w)));
    functions.insert("-".into(), Box::new(|a,e,r,c| builtins::do_math_op(a,e,r,c, |q, w| q - w)));
    functions.insert("*".into(), Box::new(|a,e,r,c| builtins::do_math_op(a,e,r,c, |q, w| q * w)));
    functions.insert("/".into(), Box::new(|a,e,r,c| builtins::do_math_op(a,e,r,c, |q, w| q / w)));
    functions.insert("%".into(), Box::new(|a,e,r,c| builtins::do_math_op(a,e,r,c, |q, w| q % w)));

    functions
}

impl Renderer {
    pub fn builder() -> RendererBuilder {
        RendererBuilder::new()
    }

    pub fn evaluate_multiple(&self, expr: &[TemplateExprNode], context: &RenderContext) -> Result<RenderValue, RenderError> {
        Ok(expr
           .into_iter()
           .map(|e| self.evaluate(&e, context))
           .collect::<Result<Vec<_>, _>>()?
           .into())
    }

    pub fn evaluate_attrs(&self, attrs: &Vec<TemplateAttribute>, context: &RenderContext) -> Result<Attributes, RenderError> {
        Ok(Attributes(attrs
                      .iter()
                      .map(|attr| {
                          Ok(Attribute(self.evaluate(&attr.0, context)?.finalize(), self.evaluate_multiple(&attr.1, context)?.finalize()))
                      })
                      .collect::<Result<Vec<_>, _>>()?))
    }

    pub fn evaluate(&self, expr: &TemplateExprNode, context: &RenderContext) -> Result<RenderValue, RenderError> {
        Ok(match expr {
            TemplateExprNode::Identifier(ident) => {
                expand_variable(&ident, self, context)?
            },
            TemplateExprNode::Integer(i) => {
                (*i).into()
            },
            TemplateExprNode::Tag(tag) => {
                let eval_attrs = self.evaluate_attrs(&tag.attrs, context)?;
                match self.functions.get(&tag.tag) {
                    Some(op_func) => op_func(eval_attrs, &tag.children, self, context)?,
                    None => basic_html_tag(tag.tag.clone(), &eval_attrs, &tag.children, self, context)?,
                }
            },
        })
    }

    pub fn render(&self, template: &Template, context: &RenderContext) -> Result<String, RenderError> {
        Ok(self.evaluate(&template.expr, context)?.finalize())
    }
}

impl Default for Renderer {
    fn default() -> Renderer {
        Renderer::builder().build()
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

