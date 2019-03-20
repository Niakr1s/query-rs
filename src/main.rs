#![allow(dead_code)]
#![allow(unused_variables)]
mod query;
mod query_result;
mod text_query;

use std::path::Path;

use text_query::TextQuery;

fn main() {
    let tq = TextQuery::from(Path::new("alice.txt"));
}
