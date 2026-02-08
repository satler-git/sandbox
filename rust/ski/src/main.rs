/*
* Syntax reference
*
* ** Lambda
*
* \<ident>. <expr>
*
* <ident> = _ | "(a-z|0-9)+"
*
* ** <expr>
*
* (<expr>)
* <expr> <expr> <- function apply
* <ident>
* <lambda>
*
* f x y = (f x) y
* $
*
* lambda
*
* ** Const
*
* <ident> = expr
*
* main = ...
*
*
* main
* 最後の行に(\x. x + 1) 0を適用した結果を表示する
*
* 行ごとに処理する
*/

use std::{fs::File, io::Read, path::PathBuf};

use clap::Parser;

use anyhow::Result;

use ski::EvalPipeline;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::try_parse()?;

    let script = if let Some(path) = args.file {
        let mut file = File::open(path)?;
        let mut buf = String::new();

        file.read_to_string(&mut buf)?;

        buf
    } else {
        std::io::read_to_string(std::io::stdin())?
    };

    EvalPipeline::eval(&script, vec![])?;

    Ok(())
}
