use core::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::types::*;

pub struct QueryResult {
    pub text: Rc<Text>,
    pub result: Rc<RefCell<Set>>,
    query: String,
}

impl QueryResult {
    pub fn from(text: Rc<Text>, result: Rc<RefCell<Set>>, query: String) -> QueryResult {
        QueryResult {
            text,
            result,
            query,
        }
    }
}

impl fmt::Display for QueryResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}:", self.query,)?;
        let mut sorted = self
            .result
            .borrow()
            .clone()
            .into_iter()
            .collect::<Vec<usize>>();
        sorted[..].sort();
        for line_no in sorted {
            writeln!(f, "{}: {}", line_no, self.text[line_no])?;
        }
        Ok(())
    }
}
