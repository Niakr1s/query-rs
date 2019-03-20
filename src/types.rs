use core::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

pub type Text = Vec<String>;
pub type Set = HashSet<usize>;
pub type Words = HashMap<String, Rc<RefCell<Set>>>;
