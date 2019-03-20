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

impl QueryCommon for WordQuery {
    fn eval(&self, tq: &TextQuery) -> QueryResult {
        let text = Rc::clone(&tq.text);
        let result = tq.words[&self.word].clone();
        QueryResult::from(text, result, self.rep())
    }
    fn rep(&self) -> String {
        self.word.clone()
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
        let res = wq.eval(&tq);
        assert!(res.result.contains(&1) && res.result.len() == 1);
    }
}
