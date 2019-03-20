use core::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

pub struct QueryResult {
    text: Rc<Vec<String>>,
    result: Rc<RefCell<HashSet<usize>>>,
    query: String,
}

impl QueryResult {
    pub fn new() -> QueryResult {
        unimplemented!() // todo remove this
    }

    pub fn from(
        text: Rc<Vec<String>>,
        result: Rc<RefCell<HashSet<usize>>>,
        query: String,
    ) -> QueryResult {
        QueryResult {
            text,
            result,
            query,
        }
    }
}
