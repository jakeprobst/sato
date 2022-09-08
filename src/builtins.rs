use crate::renderer::{Attributes, Renderer, RenderError, basic_html_tag};
use crate::context::{ContextValue, RenderContext};
use crate::template::{TemplateExprNode, TemplateTag};


pub(crate) fn do_html(attrs: &Attributes, expr: &[&TemplateExprNode], renderer: &Renderer, context: &RenderContext) -> Result<Vec<String>, RenderError> {
    let mut v = vec!["<!doctype html5>".into()];
    v.append(&mut basic_html_tag("html".into(), attrs, expr, renderer, context)?);
    Ok(v)
}

pub(crate) fn do_is_set(_: &Attributes, expr: &[&TemplateExprNode], _render: &Renderer, context: &RenderContext) -> Result<Vec<String>, RenderError> {
    match expr.get(0) {
        Some(TemplateExprNode::Identifier(ident)) => {
            Ok(match context.get(ident) {
                Some(ContextValue::String(value)) => {
                    if *value == "'null".to_owned() {
                        vec!["'false".into()]
                    }
                    else {
                        vec!["'true".into()]
                    }
                },
                _ => vec!["'false".into()],
            })
        },
        _ => Err(RenderError::IsSet("expected identifier".into(), expr.iter().cloned().cloned().collect()))
    }
}

pub(crate) fn do_eq(_: &Attributes, expr: &[&TemplateExprNode], renderer: &Renderer, context: &RenderContext) -> Result<Vec<String>, RenderError> {
    let exp1 = expr.get(0)
        .and_then(|e| renderer.evaluate(e, context).ok())
        .map(|e| e.join(""))
        .ok_or_else(|| RenderError::Mod("missing expr 1".into(), expr.iter().cloned().cloned().collect()))?;

    let exp2 = expr.get(1)
        .and_then(|e| renderer.evaluate(e, context).ok())
        .map(|e| e.join(""))
        .ok_or_else(|| RenderError::Mod("missing expr 1".into(), expr.iter().cloned().cloned().collect()))?;

    Ok(if exp1 == exp2 {
        vec!["'true".into()]
    }
    else {
        vec!["'false".into()]
    })
}

pub(crate) fn do_mod(_: &Attributes, expr: &[&TemplateExprNode], renderer: &Renderer, context: &RenderContext) -> Result<Vec<String>, RenderError> {
    let exp1 = expr.get(0)
        .and_then(|e| renderer.evaluate(e, context).ok())
        .map(|e| e.join(""))
        .and_then(|e| e.parse::<i32>().ok())
        .ok_or_else(|| RenderError::Mod("missing expr 1".into(), expr.iter().cloned().cloned().collect()))?;

    let exp2 = expr.get(1)
        .and_then(|e| renderer.evaluate(e, context).ok())
        .map(|e| e.join(""))
        .and_then(|e| e.parse::<i32>().ok())
        .ok_or_else(|| RenderError::Mod("missing expr 1".into(), expr.iter().cloned().cloned().collect()))?;
    Ok(vec![(exp1 % exp2).to_string()])
}

pub(crate) fn do_add(_: &Attributes, expr: &[&TemplateExprNode], renderer: &Renderer, context: &RenderContext) -> Result<Vec<String>, RenderError> {
    let exp1 = expr.get(0)
        .and_then(|e| renderer.evaluate(e, context).ok())
        .map(|e| e.join(""))
        .and_then(|e| e.parse::<i32>().ok())
        .ok_or_else(|| RenderError::Mod("missing expr 1".into(), expr.iter().cloned().cloned().collect()))?;

    let exp2 = expr.get(1)
        .and_then(|e| renderer.evaluate(e, context).ok())
        .map(|e| e.join(""))
        .and_then(|e| e.parse::<i32>().ok())
        .ok_or_else(|| RenderError::Mod("missing expr 1".into(), expr.iter().cloned().cloned().collect()))?;
    Ok(vec![(exp1 + exp2).to_string()])
}

pub(crate) fn do_sub(_: &Attributes, expr: &[&TemplateExprNode], renderer: &Renderer, context: &RenderContext) -> Result<Vec<String>, RenderError> {
    let exp1 = expr.get(0)
        .and_then(|e| renderer.evaluate(e, context).ok())
        .map(|e| e.join(""))
        .and_then(|e| e.parse::<i32>().ok())
        .ok_or_else(|| RenderError::Mod("missing expr 1".into(), expr.iter().cloned().cloned().collect()))?;

    let exp2 = expr.get(1)
        .and_then(|e| renderer.evaluate(e, context).ok())
        .map(|e| e.join(""))
        .and_then(|e| e.parse::<i32>().ok())
        .ok_or_else(|| RenderError::Mod("missing expr 1".into(), expr.iter().cloned().cloned().collect()))?;
    Ok(vec![(exp1 - exp2).to_string()])
}

pub(crate) fn do_mul(_: &Attributes, expr: &[&TemplateExprNode], renderer: &Renderer, context: &RenderContext) -> Result<Vec<String>, RenderError> {
    let exp1 = expr.get(0)
        .and_then(|e| renderer.evaluate(e, context).ok())
        .map(|e| e.join(""))
        .and_then(|e| e.parse::<i32>().ok())
        .ok_or_else(|| RenderError::Mod("missing expr 1".into(), expr.iter().cloned().cloned().collect()))?;

    let exp2 = expr.get(1)
        .and_then(|e| renderer.evaluate(e, context).ok())
        .map(|e| e.join(""))
        .and_then(|e| e.parse::<i32>().ok())
        .ok_or_else(|| RenderError::Mod("missing expr 1".into(), expr.iter().cloned().cloned().collect()))?;
    Ok(vec![(exp1 * exp2).to_string()])
}

pub(crate) fn do_div(_: &Attributes, expr: &[&TemplateExprNode], renderer: &Renderer, context: &RenderContext) -> Result<Vec<String>, RenderError> {
    let exp1 = expr.get(0)
        .and_then(|e| renderer.evaluate(e, context).ok())
        .map(|e| e.join(""))
        .and_then(|e| e.parse::<i32>().ok())
        .ok_or_else(|| RenderError::Mod("missing expr 1".into(), expr.iter().cloned().cloned().collect()))?;

    let exp2 = expr.get(1)
        .and_then(|e| renderer.evaluate(e, context).ok())
        .map(|e| e.join(""))
        .and_then(|e| e.parse::<i32>().ok())
        .ok_or_else(|| RenderError::Mod("missing expr 1".into(), expr.iter().cloned().cloned().collect()))?;
    Ok(vec![(exp1 / exp2).to_string()])
}


pub(crate) fn do_if(_: &Attributes, expr: &[&TemplateExprNode], renderer: &Renderer, context: &RenderContext) -> Result<Vec<String>, RenderError> {
    let conditional = expr.get(0)
        .ok_or_else(|| RenderError::If("condition not found".into(), expr.iter().cloned().cloned().collect()))
        .cloned()?;

    let result = renderer.evaluate(&conditional, context)?.join("");

    Ok(
        if result == "'false" || result == "'null" {
            match expr.get(2) {
                Some(e) => renderer.evaluate(e, context)?,
                None => Vec::new(),
            }
        }
        else {
            renderer.evaluate(expr
                              .get(1)
                              .ok_or_else(|| RenderError::If("code block not found".into(), expr.iter().cloned().cloned().collect()))?,
                              context)?
        })
}

pub(crate) fn do_case(_: &Attributes, expr: &[&TemplateExprNode], renderer: &Renderer, context: &RenderContext) -> Result<Vec<String>, RenderError> {
    let condition = expr.get(0)
        .ok_or_else(|| RenderError::Case("variant not found".into(), expr.iter().cloned().cloned().collect()))?;

    let switch_value = context.get("__switch")
        .ok_or_else(|| RenderError::Case("builtin switch variable not found?".into(), expr.iter().cloned().cloned().collect()))?;

    let body = expr.get(1..)
        .unwrap_or_default();

    match (condition, switch_value) {
        (TemplateExprNode::Identifier(condition_str), ContextValue::String(switch_str)) if condition_str == switch_str => {
            renderer.evaluate_multiple(body, context)
        },
        _ => return Ok(Vec::new())
    }
}

pub(crate) fn do_switch(_: &Attributes, expr: &[&TemplateExprNode], renderer: &Renderer, context: &RenderContext) -> Result<Vec<String>, RenderError> {
    let variable = renderer.evaluate(expr.get(0)
                                     .ok_or_else(|| RenderError::Switch("variable not found".into(),
                                                                        expr.iter().cloned().cloned().collect()))?,
                                     context)?.join("");
    let cases = expr.get(1..);
    let mut context = context.clone();
    context.insert("__switch", variable);
    Ok(cases.iter()
       .map(|case| {
           renderer.evaluate_multiple(&case, &context)
       })
       .collect::<Result<Vec<Vec<_>>, _>>()?
       .iter()
       .flatten()
       .cloned()
       .collect::<Vec<_>>())
}

fn parse_range(tag: &TemplateTag) -> Option<ContextValue> {
    let min = tag.children.get(0)
        .and_then(TemplateExprNode::as_integer)?;
    let max = tag.children.get(1)
        .and_then(TemplateExprNode::as_integer)?;
    let step = tag.children.get(2)
        .and_then(TemplateExprNode::as_integer)
        .unwrap_or(1) as usize;

    let range = (min..max)
        .step_by(step)
        .map(Into::into)
        .collect();

    Some(ContextValue::Vec(range))
}

pub(crate) fn do_for(attrs: &Attributes, expr: &[&TemplateExprNode], renderer: &Renderer, context: &RenderContext) -> Result<Vec<String>, RenderError> {
    let in_position = expr.iter()
        .position(|b| {
            match b {
                TemplateExprNode::Identifier(ident) if ident == "in" => true,
                _ => false,
            }
        });

    if let Some(in_position) = in_position {
        let iterable = expr.get(in_position+1)
            .map(|e| {
                match e {
                    TemplateExprNode::Identifier(ident) => {
                        context
                            .get(ident)
                            .cloned()
                            .ok_or_else(|| RenderError::For("iterable is not a variable".into(), attrs.clone(), expr.iter().cloned().cloned().collect()))
                    },
                    TemplateExprNode::Tag(tag) if tag.tag == "range" => {
                        parse_range(tag)
                            .ok_or_else(|| RenderError::For("invalid range".into(), attrs.clone(), expr.iter().cloned().cloned().collect()))
                    },
                    _ => Err(RenderError::For("iteration variable is not a valid type".into(), attrs.clone(), expr.iter().cloned().cloned().collect()))
                }
            })
            .ok_or_else(|| RenderError::For("no iteration variable specified".into(), attrs.clone(), expr.iter().cloned().cloned().collect()))??;
        let body = expr.get(in_position+2..)
            .unwrap_or_default();

        match iterable {
            ContextValue::Vec(v) => {
                enum IterType {
                    Normal(String),
                    Enum(String, String),
                }

                let val = expr.get(in_position-1)
                    .and_then(|e| {
                        match e {
                            TemplateExprNode::Identifier(_) => renderer.evaluate(e, context).map(|e| IterType::Normal(e.join(""))).ok(),
                            TemplateExprNode::Tag(tag) if tag.tag == "enumerate" => {
                                let iter = tag.children.get(0)
                                    .and_then(TemplateExprNode::as_identifier)?;
                                let index = tag.children.get(1)
                                    .and_then(TemplateExprNode::as_identifier)?;
                                Some(IterType::Enum(iter.clone(), index.clone()))
                            },
                            _ => None
                        }
                    })
                    .ok_or_else(|| RenderError::For("missing variable to iterate over".into(), attrs.clone(), expr.iter().cloned().cloned().collect()))?;

                let mut second_context = context.clone();
                Ok(v.iter()
                   .enumerate()
                   .map(|(i, value)| {
                       match &val {
                           IterType::Normal(val) => {
                               second_context.insert(val.clone(), value.clone());
                           }
                           IterType::Enum(iter, index) => {
                               second_context.insert(iter.clone(), value.clone());
                               second_context.insert(index.clone(), i);
                           }
                       }
                       renderer.evaluate_multiple(body, &second_context)
                   })
                   .collect::<Result<Vec<_>, RenderError>>()?
                   .iter()
                   .flatten()
                   .cloned()
                   .collect())
            },
            ContextValue::Object(o) => {
                let key_var = expr.get(in_position-2)
                    .and_then(|a| renderer.evaluate(a, context).ok())
                    .map(|e| e.join(""))
                    .ok_or_else(|| RenderError::For("missing key variable to iterate over".into(), attrs.clone(), expr.iter().cloned().cloned().collect()))?;
                let value_var = expr.get(in_position-1)
                    .and_then(|a| renderer.evaluate(a, context).ok())
                    .map(|e| e.join(""))
                    .ok_or_else(|| RenderError::For("missing value variable to iterate over".into(), attrs.clone(), expr.iter().cloned().cloned().collect()))?;
                let mut second_context = context.clone();
                Ok(o.0.iter()
                   .map(|(key, value)| {
                       second_context.insert(key_var.clone(), ContextValue::String(key.clone()));
                       second_context.insert(value_var.clone(), value.clone());
                       renderer.evaluate_multiple(body, &second_context)
                   })
                   .collect::<Result<Vec<_>, RenderError>>()?
                   .iter()
                   .flatten()
                   .cloned()
                   .collect())
            },
            _ => Err(RenderError::For("element is not iterable".into(), attrs.clone(), expr.iter().cloned().cloned().collect()))
        }
    }
    else {
        let body = expr.get(0..)
            .unwrap_or_default();

        let max = attrs.get("max")
            .and_then(|a| renderer.evaluate_string(a, context).ok())
            .and_then(|a| a.parse::<i32>().ok());
        let min = attrs.get("min")
            .and_then(|a| renderer.evaluate_string(a, context).ok())
            .and_then(|a| a.parse::<i32>().ok());
        let step = attrs.get("step")
            .and_then(|a| renderer.evaluate_string(a, context).ok())
            .and_then(|a| a.parse::<i32>().ok());

        if let (Some(min), Some(max)) = (min, max) {
            let val = attrs.get("var")
                .ok_or_else(|| RenderError::For("missing var attribute for range iteration".into(), attrs.clone(), expr.iter().cloned().cloned().collect()))?;

            let mut second_context = context.clone();
            Ok((min..max)
               .step_by(step.unwrap_or(1) as usize)
               .map(|value| {
                   second_context.insert(val, value.to_string());
                   renderer.evaluate_multiple(body, &second_context)
               })
               .collect::<Result<Vec<_>, RenderError>>()?
               .iter()
               .flatten()
               .cloned()
               .collect())
        }
        else {
            let iterable = context
                .get(attrs.get("iterate")
                     .ok_or_else(|| RenderError::For("missing iterate attribute".into(), attrs.clone(), expr.iter().cloned().cloned().collect()))?)
                .ok_or_else(|| RenderError::For("iterate attribute variable is not set ".into(), attrs.clone(), expr.iter().cloned().cloned().collect()))?;

            match iterable {
                ContextValue::Vec(v) => {
                    let val = attrs.get("var")
                        .ok_or_else(|| RenderError::For("missing var attribute for array iteration".into(), attrs.clone(), expr.iter().cloned().cloned().collect()))?;
                    let index = attrs.get("index");

                    let mut second_context = context.clone();
                    Ok(v.iter()
                       .enumerate()
                       .map(|(i, value)| {
                           second_context.insert(val, value.clone());
                           if let Some(index) = index {
                               second_context.insert(index, i);
                           }
                           renderer.evaluate_multiple(body, &second_context)
                       })
                       .collect::<Result<Vec<_>, RenderError>>()?
                       .iter()
                       .flatten()
                       .cloned()
                       .collect())
                }
                ContextValue::Object(o) => {
                    let key_var = attrs.get("key")
                        .ok_or_else(|| RenderError::For("missing key attribute for object iteration".into(), attrs.clone(), expr.iter().cloned().cloned().collect()))?;
                    let value_var = attrs.get("value")
                        .ok_or_else(|| RenderError::For("missing value attribute for object iteration".into(), attrs.clone(), expr.iter().cloned().cloned().collect()))?;
                    let mut second_context = context.clone();
                    Ok(o.0.iter()
                       .map(|(key, value)| {
                           second_context.insert(key_var, ContextValue::String(key.clone()));
                           second_context.insert(value_var, value.clone());
                           renderer.evaluate_multiple(body, &second_context)
                       })
                       .collect::<Result<Vec<_>, RenderError>>()?
                       .iter()
                       .flatten()
                       .cloned()
                       .collect())
                }
                _ => Err(RenderError::For("iterate attribute is not an array or object".into(), attrs.clone(), expr.iter().cloned().cloned().collect()))
            }
        }
    }
}
