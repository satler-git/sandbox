use anyhow::Result;

use crate::{parse::Parser, token::Tokenizer};

mod parse;
mod token;

#[derive(Debug, Default)]
pub struct EvalPipeline;

#[derive(Debug)]
pub enum Arg {
    V,
}

impl EvalPipeline {
    pub fn eval(buf: &str, args: Vec<Arg>) -> Result<()> {
        let tokens = Tokenizer::tokenize(buf);
        let parsed = Parser::parse(tokens)?;

        // evaler

        Ok(())
    }
}
