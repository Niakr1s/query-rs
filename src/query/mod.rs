use crate::query_result::QueryResult;
use crate::text_query::TextQuery;

pub mod no_query;
pub mod word_query;

pub trait Query {
    fn eval(&self, tq: &TextQuery) -> QueryResult;
    fn rep(&self) -> String;
}
