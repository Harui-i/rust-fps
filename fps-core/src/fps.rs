use std::ops::{Add, Sub, Mul, Neg};
use num_rational::BigRational;
use num_traits::{One, Zero};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fps {
    coeffs: Vec<BigRational>,
}

impl Fps {
    pub fn new(coeffs: Vec<BigRational>) -> Self {
        Self { coeffs }
    }

    pub fn coeffs(&self) -> &[BigRational] {
        &self.coeffs
    }

    pub fn get_coeff(&self, i: usize) -> BigRational {
        self.coeffs.get(i).cloned().unwrap_or_else(BigRational::zero)
    }

    pub fn zero() -> Self {
        Self { coeffs: vec![] }
    }

    pub fn one() -> Self {
        Self {
            coeffs: vec![BigRational::one()],
        }
    }

    pub fn from_variable() -> Self {
        Self {
            coeffs: vec![BigRational::zero(), BigRational::one()],
        }
    }

    pub fn degree(&self) -> usize {
        self.coeffs.len()
    }

    pub fn truncate(&mut self, degree: usize) {
        self.coeffs.truncate(degree);
    }

    pub fn div(&self, rhs: &Self, precision: usize) -> Self {
        if rhs.coeffs.iter().all(|c| c.is_zero()) {
            panic!("Division by zero");
        }

        let l = rhs.coeffs.iter().position(|c| !c.is_zero()).unwrap_or(0);

        if self.degree() < l {
            return Fps::zero();
        }

        let mut a = self.clone();
        a.coeffs.drain(0..l);
        let mut b = rhs.clone();
        b.coeffs.drain(0..l);

        let inv_b0 = BigRational::one() / b.get_coeff(0);

        let mut c = vec![BigRational::zero(); precision];
        for i in 0..precision {
            let mut s = BigRational::zero();
            for j in 1..=i {
                s += b.get_coeff(j) * c[i - j].clone();
            }
            c[i] = (a.get_coeff(i) - s) * inv_b0.clone();
        }
        Fps::new(c)
    }
}

impl Add for &Fps {
    type Output = Fps;

    fn add(self, rhs: Self) -> Self::Output {
        let max_degree = self.coeffs.len().max(rhs.coeffs.len());
        let mut coeffs = Vec::with_capacity(max_degree);
        for i in 0..max_degree {
            coeffs.push(self.get_coeff(i) + rhs.get_coeff(i));
        }
        Fps::new(coeffs)
    }
}

impl Sub for &Fps {
    type Output = Fps;

    fn sub(self, rhs: Self) -> Self::Output {
        let max_degree = self.coeffs.len().max(rhs.coeffs.len());
        let mut coeffs = Vec::with_capacity(max_degree);
        for i in 0..max_degree {
            coeffs.push(self.get_coeff(i) - rhs.get_coeff(i));
        }
        Fps::new(coeffs)
    }
}

impl Mul for &Fps {
    type Output = Fps;

    fn mul(self, rhs: Self) -> Self::Output {
        let degree = self.degree() + rhs.degree();
        let mut coeffs = vec![BigRational::zero(); degree];
        for i in 0..self.degree() {
            for j in 0..rhs.degree() {
                if i + j < degree {
                    coeffs[i + j] += self.get_coeff(i) * rhs.get_coeff(j);
                }
            }
        }
        Fps::new(coeffs)
    }
}

impl Neg for &Fps {
    type Output = Fps;

    fn neg(self) -> Self::Output {
        let coeffs = self.coeffs.iter().map(|c| -c.clone()).collect();
        Fps::new(coeffs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_div() {
        // 1 / (1 - x) = 1 + x + x^2 + ...
        let one = Fps::one();
        let one_minus_x = Fps::new(vec![BigRational::one(), -BigRational::one()]);
        let actual = one.div(&one_minus_x, 10);

        let expected = Fps::new(vec![BigRational::one(); 10]);
        assert_eq!(actual, expected);
    }
}
