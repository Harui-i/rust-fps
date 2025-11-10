use anyhow::Result;
use clap::Parser;
use fps_core::{evaluator, parser, tokenizer};

const MAX_DEGREE: usize = 3;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The expression to parse
    expression: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let tokens = tokenizer::tokenize(&args.expression)?;
    let ast = parser::parse(&tokens)?;
    let series = evaluator::evaluate(&ast, MAX_DEGREE)?;
    println!("{}", series);
    Ok(())
}
