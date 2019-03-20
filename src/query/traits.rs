use crate::prelude::*;

pub trait QueryCommon {
    fn eval(&self, tq: &TextQuery) -> QueryResult;
    fn rep(&self) -> String;
}
