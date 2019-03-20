#![allow(dead_code)]
#![allow(unused_variables)]
mod query;
mod query_result;
mod text_query;

use std::path::Path;

use query::{Query, WordQuery};
use text_query::TextQuery;

fn main() {
    let tq = TextQuery::from(Path::new("alice.txt")).expect("no alice.txt file");
    let query = WordQuery::new("Alice");
    let res = query.eval(&tq);
    println!("{}", res);
}
