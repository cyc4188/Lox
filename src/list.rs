use std::fmt::Display;

use crate::Object;

/// list can store any Object
#[derive(Debug, Clone)]
pub struct List {
    pub inner: Vec<Object>,
}

impl List {
    pub fn new() -> Self {
        Self { inner: vec![] }
    }

    pub fn push(&mut self, obj: Object) {
        self.inner.push(obj);
    }

    pub fn slice(&self, start: usize, end: usize) -> Self {
        Self {
            inner: self.inner[start..end].to_vec(),
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        let mut new_list = self.clone();
        new_list.inner.extend(other.inner.clone());
        new_list
    }
}

impl From<Vec<Object>> for List {
    fn from(v: Vec<Object>) -> Self {
        Self { inner: v }
    }
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = self
            .inner
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "[{}]", s)
    }
}
