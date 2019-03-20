use crate::prelude::*;
use std::rc::Rc;

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
        let result = tq.words[&self.word].clone();
        QueryResult::from(text, result, self.rep())
    }
    fn rep(&self) -> String {
        self.word.clone()
    }
}
