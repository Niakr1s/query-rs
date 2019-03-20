use crate::query_result::QueryResult;
use crate::text_query::TextQuery;

trait Query {
    fn eval(&self, tq: &TextQuery) -> QueryResult;
    fn rep(&self) -> String;
}

struct WordQuery {
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
        QueryResult::new() // todo real eval
    }
    fn rep(&self) -> String {
        self.word.clone()
    }
}
