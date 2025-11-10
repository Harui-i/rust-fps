use crate::{parser::Expr, series::Series};
use num_traits::ToPrimitive;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EvalError {
    #[error("Division by series with zero constant term")]
    DivisionByZero,
    #[error("Exponent must be a constant integer")]
    NonIntegerExponent,
    #[error("Exponent magnitude is too large")]
    ExponentTooLarge,
    #[error("{0} requires series with zero constant term")]
    FunctionRequiresZeroConstant(&'static str),
    #[error("log requires series with constant term equal to 1")]
    LogRequiresUnitConstant,
}

pub fn evaluate(expr: &Expr, max_degree: usize) -> Result<Series, EvalError> {
    match expr {
        Expr::Num(value) => Ok(Series::constant(value.clone(), max_degree)),
        Expr::Variable(_) => Ok(Series::variable(max_degree)),
        Expr::Add(lhs, rhs) => {
            let left = evaluate(lhs, max_degree)?;
            let right = evaluate(rhs, max_degree)?;
            Ok(left.add(&right))
        }
        Expr::Sub(lhs, rhs) => {
            let left = evaluate(lhs, max_degree)?;
            let right = evaluate(rhs, max_degree)?;
            Ok(left.sub(&right))
        }
        Expr::Mul(lhs, rhs) => {
            let left = evaluate(lhs, max_degree)?;
            let right = evaluate(rhs, max_degree)?;
            Ok(left.mul(&right))
        }
        Expr::Div(lhs, rhs) => {
            let left = evaluate(lhs, max_degree)?;
            let right = evaluate(rhs, max_degree)?;
            left.div(&right)
        }
        Expr::Pow(base, exponent) => {
            let base_series = evaluate(base, max_degree)?;
            let exponent_series = evaluate(exponent, max_degree)?;

            if !exponent_series.is_constant() {
                return Err(EvalError::NonIntegerExponent);
            }

            let exponent_value = exponent_series.constant_term();
            if !exponent_value.is_integer() {
                return Err(EvalError::NonIntegerExponent);
            }

            let exponent_bigint = exponent_value.to_integer();
            let exponent = exponent_bigint
                .to_i64()
                .ok_or(EvalError::ExponentTooLarge)?;

            base_series.powi(exponent)
        }
        Expr::Neg(inner) => {
            let series = evaluate(inner, max_degree)?;
            Ok(series.neg())
        }
        Expr::Sin(inner) => {
            let series = evaluate(inner, max_degree)?;
            series.sin()
        }
        Expr::Exp(inner) => {
            let series = evaluate(inner, max_degree)?;
            series.exp()
        }
        Expr::Log(inner) => {
            let series = evaluate(inner, max_degree)?;
            series.log()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parser, tokenizer};

    fn evaluate_str(expr: &str, max_degree: usize) -> Result<Series, EvalError> {
        let tokens = tokenizer::tokenize(expr).expect("tokenize");
        let ast = parser::parse(&tokens).expect("parse");
        evaluate(&ast, max_degree)
    }

    #[test]
    fn evaluates_geometric_series() {
        let series = evaluate_str("1 / (1 - x)", 3).expect("evaluate");
        assert_eq!(format!("{}", series), "1 + x + x^2 + x^3 + O(x^4)");
    }

    #[test]
    fn rejects_non_integer_exponent() {
        let error = evaluate_str("(1 + x)^(x)", 3).unwrap_err();
        assert_eq!(error, EvalError::NonIntegerExponent);
    }

    #[test]
    fn rejects_large_exponent() {
        let error = evaluate_str("(1 + x)^(100000000000000000000)", 3).unwrap_err();
        assert_eq!(error, EvalError::ExponentTooLarge);
    }
}
