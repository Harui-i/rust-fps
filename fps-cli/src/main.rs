use anyhow::Result;
use clap::Parser;
use fps_core::{evaluator, parser, tokenizer};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The expression to parse
    expression: String,

    /// Maximum degree of the resulting power series (default: 5)
    #[arg(short, long)]
    maxdeg: Option<usize>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let tokens = tokenizer::tokenize(&args.expression)?;
    let ast = parser::parse(&tokens)?;
    let series = evaluator::evaluate(&ast, args.maxdeg.unwrap_or(5))?;
    println!("{}", series);
    Ok(())
}
