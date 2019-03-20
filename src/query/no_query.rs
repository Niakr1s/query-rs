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
        let qr = self.query.eval(&tq);
        let all = tq.all_lines();
        let res = &all - &qr.result;
        QueryResult::from(Rc::clone(&qr.text), res, self.rep())
    }
    fn rep(&self) -> String {
        format!("~{}", self.query.rep())
    }
}
