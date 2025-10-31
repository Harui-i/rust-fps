use anyhow::Result;
use clap::Parser;
use fps_core::{tokenizer, parser};

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
    println!("{:#?}", ast);
    Ok(())
}
