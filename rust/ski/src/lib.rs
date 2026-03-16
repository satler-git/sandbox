use anyhow::Result;

use crate::{eval::Value, parse::Parser, token::Tokenizer};

mod eval;
mod parse;
mod token;

pub struct EvalPipeline;

impl EvalPipeline {
    pub fn eval<T>(buf: &str, args: Vec<Value<T>>) -> Result<Value<T>> {
        let tokens = Tokenizer::tokenize(buf);
        let parsed = Parser::parse(tokens)?;

        dbg!(&parsed);

        Ok(eval::Evaluator::eval(parsed, args))
    }
}
