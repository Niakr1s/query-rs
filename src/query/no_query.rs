use crate::prelude::*;
use std::rc::Rc;

pub struct NoQuery<Q>
where
    Q: Query,
{
    query: Q,
}

impl<Q> NoQuery<Q>
where
    Q: Query,
{
    pub fn new(query: Q) -> NoQuery<Q> {
        NoQuery { query }
    }
}

impl<Q> Query for NoQuery<Q>
where
    Q: Query,
{
    fn eval(&self, tq: &TextQuery) -> QueryResult {
        let result = self.query.eval(&tq);
        unimplemented!()
        // QueryResult::from(text, result, self.rep())
    }
    fn rep(&self) -> String {
        format!("~{}", self.query.rep())
    }
}
