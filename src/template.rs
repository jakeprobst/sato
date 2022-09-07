use std::io::Read;
use crate::renderer::{Attribute, Attributes};

#[derive(thiserror::Error, Debug)]
pub enum ParseExprError {
    #[error("expr is not an item")]
    NotAnItem,
    #[error("expr is not a list")]
    NotAList,
    #[error("@ attribute is not a list {0:?} {1:?}")]
    NotAnAttribute(sexp::Sexp, Vec<sexp::Sexp>),
    #[error("html attribute is missing an element {0:?}")]
    AttributeMissingElement(Vec<sexp::Sexp>),
}


#[derive(Debug, Clone)]
pub struct TemplateTag {
    pub tag: String,
    pub attrs: Attributes,
    pub children: Vec<TemplateExprNode>,
}

#[derive(Debug, Clone)]
pub enum TemplateExprNode {
    Identifier(String),
    //Integer(i64),
    Tag(TemplateTag)
}

impl TryFrom<String> for TemplateExprNode {
    type Error = TemplateError;
    
    fn try_from(other: String) -> Result<TemplateExprNode, Self::Error> {
        Ok(parse_expr(&sexp::parse(&other)?)?)
    }
}


pub enum TemplateNode {
    Html{
        attrs: Attributes,
        children: Vec<TemplateNode>,
    },
    IsSet,
    If,
    Switch,
    Case,
    For,
    Eq,
    Mod,

    Other {
        tag: String,
        attrs: Attributes,
        children: Vec<TemplateNode>,
    }
}


fn parse_attrs(attrs: &Vec<sexp::Sexp>) -> Result<Vec<Attribute>, ParseExprError> {
    attrs.iter().skip(1)
        .map(|attr| {
            match attr {
                sexp::Sexp::List(list) => {
                    let name = list
                        .get(0)
                        .ok_or_else(|| ParseExprError::AttributeMissingElement(attrs.clone()))?
                        .to_string();
                    let value = list
                        .get(1)
                        .ok_or_else(|| ParseExprError::AttributeMissingElement(attrs.clone()))?
                        .to_string();
                    Ok(Attribute(name, value))
                }
                _ => Err(ParseExprError::NotAnAttribute(attr.clone(), attrs.clone()))
            }
        })
        .collect::<Result<Vec<_>, ParseExprError>>()
}

fn parse_expr(expr: &sexp::Sexp) -> Result<TemplateExprNode, ParseExprError> {
    Ok(match expr {
        sexp::Sexp::Atom(atom) => {
            match atom {
                sexp::Atom::S(s) => TemplateExprNode::Identifier(s.to_string()),
                sexp::Atom::I(i) => TemplateExprNode::Identifier(i.to_string()),
                //sexp::Atom::I(i) => TemplateExprNode::Integer(i),
                _ => panic!("expr is not an atom {:?}", atom),
            }
        },
        sexp::Sexp::List(list) => {
            let tag = match list[0] {
                sexp::Sexp::Atom(sexp::Atom::S(ref s)) => s.clone(),
                _ => panic!("expr is not a list")
            };
            let (attrs, attr_index) = match &list.get(1) {
                Some(sexp::Sexp::List(list)) if list.get(0) == Some(&sexp::Sexp::Atom(sexp::Atom::S("@".into()))) => (parse_attrs(&list)?, 2),
                _ => (Vec::new(), 1)
            };

            let children = list.iter().skip(attr_index).map(|l| {
                parse_expr(l)
            }).collect::<Result<Vec<_>, ParseExprError>>()?;

            TemplateExprNode::Tag(TemplateTag {
                tag,
                attrs: Attributes::new(attrs),
                children,
            })
        }
    })
}


#[derive(thiserror::Error, Debug)]
pub enum TemplateError {
    #[error("could not find template file")]
    NoFile,
    #[error("invalid file")]
    InvalidFile,
    #[error("error parsing template")]
    ParseError(#[from] Box<sexp::Error>),
    #[error("error parsing template expression")]
    ParseExprError(#[from] ParseExprError),
}


#[derive(Clone, Debug)]
pub struct Template {
    pub(crate) expr: TemplateExprNode,
}


impl Template {
    pub fn from_str(template: &str) -> Result<Template, TemplateError> {
        Ok(Template {
            expr: parse_expr(&sexp::parse(template)?)?
        })
    }

    pub fn from_path<P: AsRef<std::path::Path>>(template: P) -> Result<Template, TemplateError> {
        let mut f = std::fs::File::open(template).map_err(|_| TemplateError::NoFile)?;
        let mut s = String::new();
        f.read_to_string(&mut s).map_err(|_| TemplateError::InvalidFile)?;
        Template::from_str(&s)
    }
}
