use crate::prelude::*;

pub mod no_query;
pub mod traits;
pub mod word_query;

pub struct Query<Q>
where
    Q: QueryCommon,
{
    query: Q,
}

impl<Q> Query<Q>
where
    Q: QueryCommon,
{
    pub fn new(s: &str) -> Query<Q> {
        unimplemented!() // todo rewrite parsing from c++ query
    }
}

impl<Q> QueryCommon for Query<Q>
where
    Q: QueryCommon,
{
    fn eval(&self, tq: &TextQuery) -> QueryResult {
        self.query.eval(&tq)
    }
    fn rep(&self) -> String {
        self.query.rep()
    }
}
