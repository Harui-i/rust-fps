use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};
use std::fmt;

use crate::evaluator::EvalError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Series {
    coeffs: Vec<BigRational>,
}

impl Series {
    pub fn zero(max_degree: usize) -> Self {
        Self {
            coeffs: vec![BigRational::zero(); max_degree + 1],
        }
    }

    pub fn one(max_degree: usize) -> Self {
        let mut series = Self::zero(max_degree);
        series.coeffs[0] = BigRational::one();
        series
    }

    pub fn constant(value: BigRational, max_degree: usize) -> Self {
        let mut series = Self::zero(max_degree);
        series.coeffs[0] = value;
        series
    }

    pub fn variable(max_degree: usize) -> Self {
        let mut series = Self::zero(max_degree);
        if max_degree >= 1 {
            series.coeffs[1] = BigRational::one();
        }
        series
    }

    pub fn coeffs(&self) -> &[BigRational] {
        &self.coeffs
    }

    pub fn max_degree(&self) -> usize {
        self.coeffs.len().saturating_sub(1)
    }

    pub fn constant_term(&self) -> BigRational {
        self.coeffs
            .get(0)
            .cloned()
            .unwrap_or_else(BigRational::zero)
    }

    pub fn is_constant(&self) -> bool {
        self.coeffs.iter().skip(1).all(|c| c.is_zero())
    }

    pub fn add(&self, other: &Series) -> Series {
        assert_eq!(self.coeffs.len(), other.coeffs.len());
        let mut result = self.clone();
        for (r, o) in result.coeffs.iter_mut().zip(other.coeffs.iter()) {
            *r += o;
        }
        result
    }

    pub fn neg(&self) -> Series {
        let mut result = self.clone();
        for coeff in &mut result.coeffs {
            *coeff = -coeff.clone();
        }
        result
    }

    pub fn sub(&self, other: &Series) -> Series {
        self.add(&other.neg())
    }

    pub fn mul(&self, other: &Series) -> Series {
        assert_eq!(self.coeffs.len(), other.coeffs.len());
        let max_degree = self.max_degree();
        let mut result = Series::zero(max_degree);

        for i in 0..=max_degree {
            for j in 0..=max_degree {
                if i + j > max_degree {
                    break;
                }
                result.coeffs[i + j] += self.coeffs[i].clone() * other.coeffs[j].clone();
            }
        }

        result
    }

    pub fn scale(&self, scalar: &BigRational) -> Series {
        let mut result = self.clone();
        for coeff in &mut result.coeffs {
            *coeff *= scalar.clone();
        }
        result
    }

    pub fn inverse(&self) -> Result<Series, EvalError> {
        let max_degree = self.max_degree();
        let a0 = self.constant_term();

        if a0.is_zero() {
            return Err(EvalError::DivisionByZero);
        }

        let mut result = Series::zero(max_degree);
        result.coeffs[0] = BigRational::one() / a0.clone();

        for n in 1..=max_degree {
            let mut sum = BigRational::zero();
            for k in 1..=n {
                sum += self.coeffs[k].clone() * result.coeffs[n - k].clone();
            }
            result.coeffs[n] = -sum / a0.clone();
        }

        Ok(result)
    }

    pub fn div(&self, other: &Series) -> Result<Series, EvalError> {
        let inverse = other.inverse()?;
        Ok(self.mul(&inverse))
    }

    pub fn powi(&self, exponent: i64) -> Result<Series, EvalError> {
        let max_degree = self.max_degree();

        if exponent == 0 {
            return Ok(Series::one(max_degree));
        }

        if exponent < 0 {
            return self.inverse()?.powi(-exponent);
        }

        let mut result = Series::one(max_degree);
        let mut base = self.clone();
        let mut exp = exponent;

        while exp > 0 {
            if exp % 2 == 1 {
                result = result.mul(&base);
            }
            exp /= 2;
            if exp > 0 {
                base = base.mul(&base);
            }
        }

        Ok(result)
    }

    pub fn sin(&self) -> Result<Series, EvalError> {
        if !self.constant_term().is_zero() {
            return Err(EvalError::FunctionRequiresZeroConstant("sin"));
        }

        let max_degree = self.max_degree();
        let mut result = Series::zero(max_degree);

        if max_degree == 0 {
            return Ok(result);
        }

        let mut factorial = BigInt::one();
        for n in 0..=((max_degree - 1) / 2) {
            if n > 0 {
                let two_n = 2 * (n as i64);
                factorial *= BigInt::from(two_n);
                factorial *= BigInt::from(two_n + 1);
            }

            let power = self.powi((2 * n + 1) as i64)?;
            let sign = if n % 2 == 0 {
                BigInt::one()
            } else {
                -BigInt::one()
            };
            let coeff = BigRational::new(sign, factorial.clone());
            result = result.add(&power.scale(&coeff));
        }

        Ok(result)
    }

    pub fn cos(&self) -> Result<Series, EvalError> {
        if !self.constant_term().is_zero() {
            return Err(EvalError::FunctionRequiresZeroConstant("cos"));
        }

        let max_degree = self.max_degree();
        let mut result = Series::zero(max_degree);

        if max_degree == 0 {
            result.coeffs[0] = BigRational::one();
            return Ok(result);
        }

        let mut factorial = BigInt::one();
        for n in 0..=(max_degree / 2) {
            if n > 0 {
                let two_n = 2 * (n as i64);
                factorial *= BigInt::from(two_n - 1);
                factorial *= BigInt::from(two_n);
            }

            let power = if n == 0 {
                Series::one(max_degree)
            } else {
                self.powi((2 * n) as i64)?
            };
            let sign = if n % 2 == 0 {
                BigInt::one()
            } else {
                -BigInt::one()
            };
            let coeff = BigRational::new(sign, factorial.clone());
            result = result.add(&power.scale(&coeff));
        }

        Ok(result)
    }

    pub fn exp(&self) -> Result<Series, EvalError> {
        if !self.constant_term().is_zero() {
            return Err(EvalError::FunctionRequiresZeroConstant("exp"));
        }

        let max_degree = self.max_degree();
        let mut result = Series::zero(max_degree);
        let mut factorial = BigInt::one();

        for n in 0..=max_degree {
            if n > 0 {
                factorial *= BigInt::from(n as i64);
            }
            let power = self.powi(n as i64)?;
            let coeff = BigRational::new(BigInt::one(), factorial.clone());
            result = result.add(&power.scale(&coeff));
        }

        Ok(result)
    }

    pub fn log(&self) -> Result<Series, EvalError> {
        if self.constant_term() != BigRational::one() {
            return Err(EvalError::LogRequiresUnitConstant);
        }

        let max_degree = self.max_degree();
        let mut result = Series::zero(max_degree);

        if max_degree == 0 {
            return Ok(result);
        }

        let adjustment = self.sub(&Series::one(max_degree));

        for n in 1..=max_degree {
            let power = adjustment.powi(n as i64)?;
            let sign = if n % 2 == 1 {
                BigInt::one()
            } else {
                -BigInt::one()
            };
            let coeff = BigRational::new(sign, BigInt::from(n as i64));
            result = result.add(&power.scale(&coeff));
        }

        Ok(result)
    }
}

impl fmt::Display for Series {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;

        for (degree, coeff) in self.coeffs.iter().enumerate() {
            if coeff.is_zero() {
                continue;
            }

            let is_negative = coeff.is_negative();
            let abs_coeff = if is_negative {
                -coeff.clone()
            } else {
                coeff.clone()
            };

            if first {
                if is_negative {
                    write!(f, "-{}", format_term(&abs_coeff, degree))?;
                } else {
                    write!(f, "{}", format_term(&abs_coeff, degree))?;
                }
                first = false;
            } else if is_negative {
                write!(f, " - {}", format_term(&abs_coeff, degree))?;
            } else {
                write!(f, " + {}", format_term(&abs_coeff, degree))?;
            }
        }

        if first {
            write!(f, "0")?;
        }

        write!(f, " + O(x^{})", self.max_degree() + 1)
    }
}

fn format_term(coeff: &BigRational, degree: usize) -> String {
    match degree {
        0 => format_rational(coeff),
        1 => {
            if coeff.is_one() {
                "x".to_string()
            } else {
                format!("{} x", format_rational(coeff))
            }
        }
        _ => {
            if coeff.is_one() {
                format!("x^{}", degree)
            } else {
                format!("{} x^{}", format_rational(coeff), degree)
            }
        }
    }
}

fn format_rational(rational: &BigRational) -> String {
    if rational.is_integer() {
        rational.to_integer().to_string()
    } else {
        format!("{}/{}", rational.numer(), rational.denom())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_rational::BigRational;
    use num_traits::{One, Zero};

    fn series_from_coeffs(coeffs: &[i64], max_degree: usize) -> Series {
        let mut result = Series::zero(max_degree);
        for (idx, coeff) in coeffs.iter().enumerate() {
            if idx >= result.coeffs.len() {
                break;
            }
            result.coeffs[idx] = BigRational::from_integer((*coeff).into());
        }
        result
    }

    #[test]
    fn display_includes_o_term() {
        let series = series_from_coeffs(&[1, 1, 1, 1], 3);
        assert_eq!(format!("{}", series), "1 + x + x^2 + x^3 + O(x^4)");
    }

    #[test]
    fn multiplication_truncates_to_degree() {
        let a = series_from_coeffs(&[1, 1], 3); // 1 + x
        let b = series_from_coeffs(&[1, -1], 3); // 1 - x
        let product = a.mul(&b);

        let expected = series_from_coeffs(&[1, 0, -1], 3); // 1 - x^2
        assert_eq!(product, expected);
    }

    #[test]
    fn inverse_requires_non_zero_constant_term() {
        let zero_constant = series_from_coeffs(&[0, 1], 3);
        assert_eq!(zero_constant.inverse(), Err(EvalError::DivisionByZero));
    }

    #[test]
    fn powi_handles_negative_exponents() {
        let series = series_from_coeffs(&[1, 1], 3); // 1 + x
        let inverse = series.powi(-1).unwrap();

        // 1 - x + x^2 - x^3 + O(x^4)
        let mut expected = Series::zero(3);
        expected.coeffs[0] = BigRational::one();
        expected.coeffs[1] = -BigRational::one();
        expected.coeffs[2] = BigRational::one();
        expected.coeffs[3] = -BigRational::one();

        assert_eq!(inverse, expected);
    }

    #[test]
    fn zero_series_checks() {
        let zero = Series::zero(2);
        assert!(zero.is_constant());
        assert!(zero.constant_term().is_zero());
    }
}
