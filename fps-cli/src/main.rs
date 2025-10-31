use anyhow::Result;
use clap::Parser;
use fps_core::{eval, fps::Fps, parser, tokenizer};
use num_traits::{One, Signed, Zero};
use std::fmt::Write;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The expression to parse
    expression: String,

    /// The maximum degree of the output
    #[arg(short, long, default_value_t = 10)]
    degree: usize,
}

fn format_fps(fps: &Fps, degree: usize) -> String {
    let mut s = String::new();
    let mut is_first_term = true;

    for i in 0..=degree {
        let coeff = fps.get_coeff(i);
        if coeff.is_zero() {
            continue;
        }

        let abs_coeff = if coeff.is_negative() {
            -coeff.clone()
        } else {
            coeff.clone()
        };

        if !is_first_term {
            if coeff.is_positive() {
                write!(s, " + ").unwrap();
            } else {
                write!(s, " - ").unwrap();
            }
        } else if coeff.is_negative() {
            write!(s, "-").unwrap();
        }

        let is_one = abs_coeff.is_one();
        if i == 0 || !is_one {
            write!(s, "{}", abs_coeff).unwrap();
            if i > 0 {
                write!(s, "*").unwrap();
            }
        }

        if i > 0 {
            write!(s, "x").unwrap();
            if i > 1 {
                write!(s, "^{}", i).unwrap();
            }
        }

        is_first_term = false;
    }

    if is_first_term {
        write!(s, "0").unwrap();
    }

    write!(s, " + O(x^{})", degree + 1).unwrap();

    s
}

fn main() -> Result<()> {
    let args = Args::parse();
    let tokens = tokenizer::tokenize(&args.expression)?;
    let ast = parser::parse(&tokens)?;
    let fps = eval::eval(&ast, args.degree + 2);
    println!("{}", format_fps(&fps, args.degree));
    Ok(())
}
