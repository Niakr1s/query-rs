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

#[cfg(test)]
mod test {
    use crate::prelude::*;
    #[test]
    fn word_query() {
        let tq = TextQuery::from(
            r###"Hello this, 
        wonderful world!!!!?
        And bye xD"###,
        );
        let wq = WordQuery::new("world");
        let nq = NoQuery::new(wq);
        let res = nq.eval(&tq);
        assert!(res.result.contains(&0) && res.result.contains(&2) && res.result.len() == 2);
    }
}
