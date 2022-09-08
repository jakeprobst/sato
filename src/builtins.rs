use crate::renderer::{Attributes, Renderer, RenderError, basic_html_tag};
use crate::context::{ContextValue, RenderContext};
use crate::template::TemplateExprNode;


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
            .and_then(|e| {
                match e {
                    TemplateExprNode::Identifier(ident) => context.get(ident),
                    _ => None
                }
            })
            .ok_or_else(|| RenderError::For("no iterable specified".into(), attrs.clone(), expr.iter().cloned().cloned().collect()))?;
        let body = expr.get(in_position+2..)
            .unwrap_or_default();
        match iterable {
            ContextValue::Vec(v) => {
                let val = expr.get(in_position-1)
                    .and_then(|a| renderer.evaluate(a, context).ok())
                    .map(|e| e.join(""))
                    .ok_or_else(|| RenderError::For("missing variable to iterate over".into(), attrs.clone(), expr.iter().cloned().cloned().collect()))?;

                let mut second_context = context.clone();
                Ok(v.iter()
                   .map(|value| {
                       second_context.insert(val.clone(), value.clone());
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
