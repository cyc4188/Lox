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

    pub fn get(&self, index: usize) -> &Object {
        self.inner.get(index).unwrap()
    }

    pub fn slice(&self, start: usize, end: usize) -> Self {
        Self {
            inner: self.inner[start..end].to_vec(),
        }
    }

    pub fn slice_change(&mut self, start: usize, end: usize, new: &Self) {
        self.inner.splice(start..end, new.inner.clone());
    }

    pub fn slice_change_obj(&mut self, start: usize, end: usize, new: Object) {
        self.inner.splice(start..end, vec![new]);
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
        let s = self
            .inner
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "[{}]", s)
    }
}
