mod text_query;

use std::path::Path;

use text_query::TextQuery;

fn main() {
    let tq = TextQuery::from(Path::new("alice.txt"));
}
