use crate::query_result::QueryResult;
use crate::text_query::TextQuery;

use std::rc::Rc;

pub trait Query {
    fn eval(&self, tq: &TextQuery) -> QueryResult;
    fn rep(&self) -> String;
}

pub struct WordQuery {
    word: String,
}

impl WordQuery {
    pub fn new(word: &str) -> WordQuery {
        let word = word.to_owned();
        WordQuery { word }
    }
}

impl Query for WordQuery {
    fn eval(&self, tq: &TextQuery) -> QueryResult {
        let text = Rc::clone(&tq.text);
        let result = Rc::clone(&tq.words[&self.word]);
        QueryResult::from(text, result, self.rep())
    }
    fn rep(&self) -> String {
        self.word.clone()
    }
}
