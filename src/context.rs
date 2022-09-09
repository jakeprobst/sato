use std::collections::{HashMap, BTreeMap};
use crate::renderer::RenderValue;
use crate::template::Template;


#[derive(Clone, Debug)]
pub enum ContextValue {
    Integer(i64),
    Boolean(bool),
    String(String),
    Vec(Vec<ContextValue>),
    Object(RenderContext),
    Template(Template),
}

impl From<&str> for ContextValue {
    fn from(other: &str) -> Self {
        ContextValue::String(other.into())
    }
}

impl From<String> for ContextValue {
    fn from(other: String) -> Self {
        ContextValue::String(other)
    }
}

impl From<&String> for ContextValue {
    fn from(other: &String) -> Self {
        ContextValue::String(other.clone())
    }
}

impl<T: Into<ContextValue>> From<Vec<T>> for ContextValue {
    fn from(other: Vec<T>) -> Self {
        ContextValue::Vec(other.into_iter().map(|k| k.into()).collect())
    }
}

impl From<RenderContext> for ContextValue {
    fn from(other: RenderContext) -> Self {
        ContextValue::Object(other)
    }
}

impl From<BTreeMap<String, ContextValue>> for ContextValue {
    fn from(other: BTreeMap<String, ContextValue>) -> Self {
        ContextValue::Object(RenderContext(other))
    }
}

impl From<HashMap<String, ContextValue>> for ContextValue {
    fn from(other: HashMap<String, ContextValue>) -> Self {
        ContextValue::Object(RenderContext(other.into_iter().collect()))
    }
}

impl From<Template> for ContextValue {
    fn from(other: Template) -> Self {
        ContextValue::Template(other)
    }
}

impl From<bool> for ContextValue {
    fn from(other: bool) -> Self {
        ContextValue::Boolean(other)
    }
}

// TODO: somehow make this generic over all numbers
impl From<usize> for ContextValue {
    fn from(other: usize) -> Self {
        ContextValue::Integer(other as i64)
    }
}

impl From<i32> for ContextValue {
    fn from(other: i32) -> Self {
        ContextValue::Integer(other as i64)
    }
}

impl From<i64> for ContextValue {
    fn from(other: i64) -> Self {
        ContextValue::Integer(other)
    }
}

impl PartialEq for ContextValue {
    fn eq(&self, other: &ContextValue) -> bool {
        match (self, other) {
            (ContextValue::Integer(a), ContextValue::Integer(b)) => a == b,
            (ContextValue::Boolean(a), ContextValue::Boolean(b)) => a == b,
            (ContextValue::String(a), ContextValue::String(b)) => a == b,
            (ContextValue::Vec(a), ContextValue::Vec(b)) => a == b,
            _ => false,
        }
    }
}

impl PartialOrd for ContextValue {
    fn partial_cmp(&self, other: &ContextValue) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (ContextValue::Integer(a), ContextValue::Integer(b)) => a.partial_cmp(b),
            (ContextValue::Boolean(a), ContextValue::Boolean(b)) => a.partial_cmp(b),
            (ContextValue::String(a), ContextValue::String(b)) => a.partial_cmp(b),
            (ContextValue::Vec(a), ContextValue::Vec(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl From<&RenderValue> for ContextValue {
    fn from(other: &RenderValue) -> ContextValue {
        match other {
            RenderValue::String(s) => ContextValue::String(s.clone()),
            RenderValue::Integer(i) => ContextValue::Integer(*i),
            RenderValue::Boolean(b) => ContextValue::Boolean(*b),
            RenderValue::Vec(v) => ContextValue::Vec(v.iter().map(|e| e.into()).collect()),
            RenderValue::Empty => ContextValue::String("".into()),
        }
    }
}

impl From<RenderValue> for ContextValue {
    fn from(other: RenderValue) -> ContextValue {
        match other {
            RenderValue::String(s) => ContextValue::String(s.clone()),
            RenderValue::Integer(i) => ContextValue::Integer(i),
            RenderValue::Boolean(b) => ContextValue::Boolean(b),
            RenderValue::Vec(v) => ContextValue::Vec(v.iter().map(|e| e.into()).collect()),
            RenderValue::Empty => ContextValue::String("".into()),
        }
    }
}


#[derive(Default, Clone, Debug)]
pub struct RenderContext(pub(crate) BTreeMap<String, ContextValue>);


impl RenderContext {
    pub fn builder() -> RenderContextBuilder {
        RenderContextBuilder::default()
    }
    
    pub fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: std::convert::Into<String>,
        V: std::convert::Into<ContextValue>,
    {
        self.0.insert(key.into().trim_start_matches('$').into(), value.into());
    }

    pub fn get<K>(&self, key: K) -> Option<&ContextValue>
    where
        K: std::convert::Into<String>,
    {
        self.0.get(key.into().trim_start_matches('$'))
    }
}


#[derive(Default, Clone)]
pub struct RenderContextBuilder(pub(crate) BTreeMap<String, ContextValue>);

impl RenderContextBuilder {
    pub fn insert<K, V>(mut self, key: K, value: V) -> Self
    where
        K: std::convert::Into<String>,
        V: std::convert::Into<ContextValue>,
    {
        self.0.insert(key.into(), value.into());
        self
    }

    pub fn build(self) -> RenderContext {
        RenderContext(self.0)
    }
}
