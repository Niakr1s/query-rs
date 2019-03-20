use core::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::io::Result;
use std::path::Path;
use std::rc::Rc;

use crate::types::*;

pub struct TextQuery {
    pub text: Rc<Text>,
    pub words: Words,
}

impl TextQuery {
    pub fn from(p: &Path) -> Result<TextQuery> {
        let text = {
            let mut f = File::open(&p)?;
            let mut content = String::new();
            f.read_to_string(&mut content)?;
            content.lines().map(String::from).collect::<Text>()
        };
        let text = Rc::new(text);
        let mut words = Words::new();
        for (counter, line) in text.iter().enumerate() {
            for word in line.split_whitespace() {
                let word = trim_word(word).to_owned();
                words
                    .entry(word)
                    .or_insert_with(|| Rc::new(RefCell::new(Set::new())))
                    .borrow_mut()
                    .insert(counter);
            }
        }
        Ok(TextQuery { text, words })
    }
    pub fn all_lines(&self) -> Set {
        (0..self.text.len()).collect::<Set>()
    }
}

fn trim_word(s: &str) -> &str {
    s.trim_matches(|c: char| !c.is_alphanumeric())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn alice_txt_len() {
        let p = Path::new("alice.txt");
        let tq = TextQuery::from(&p);
        assert_eq!(tq.unwrap().text.len(), 3736);
    }

    #[test]
    fn get_word_test() {
        let s = "!alice,";
        assert_eq!(trim_word(&s), "alice");
    }
}
