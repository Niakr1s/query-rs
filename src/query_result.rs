use core::cell::RefCell;
use std::collections::HashSet;
use std::fmt;
use std::rc::Rc;

pub struct QueryResult {
    text: Rc<Vec<String>>,
    result: Rc<RefCell<HashSet<usize>>>,
    query: String,
}

impl QueryResult {
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
