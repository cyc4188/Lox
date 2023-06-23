use std::{cell::RefCell, rc::Rc};

use crate::Object;

pub type ObjectRef = Rc<RefCell<Object>>;

/// list can store any Object
#[derive(Debug)]
pub struct List {
    pub inner: Vec<ObjectRef>,
}

impl List {}
