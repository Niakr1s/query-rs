#![allow(dead_code)]
#![allow(unused_variables)]
pub mod prelude;
mod query;
mod query_result;
mod text_query;
mod types;

use std::path::Path;

use prelude::*;

fn main() {
    let tq = TextQuery::from(Path::new("alice.txt")).expect("no alice.txt file");
    let query = WordQuery::new("Alice");
    let res = query.eval(&tq);
    println!("{}", res);
}
