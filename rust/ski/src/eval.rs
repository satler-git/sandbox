use crate::parse::{File, Ident};

use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Evaluator<'a> {
    curr: Vec<Env<'a>>,
}

pub enum Value<T> {
    Value(T),
    Fun(Box<dyn Fn(Value<T>) -> Value<T>>),
}

impl Evaluator<'_> {
    pub fn eval<T>(script: File<'_>, args: Vec<Value<T>>) -> Value<T> {
        todo!()
    }
}

#[derive(Debug)]
struct Env<'a> {
    namespace: HashMap<Ident<'a>, usize>,
}
