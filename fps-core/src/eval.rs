use crate::parser::Expr;
use crate::fps::Fps;
use num_traits::ToPrimitive;

pub fn eval(expr: &Expr, degree: usize) -> Fps {
    let mut fps = eval_expr(expr, degree);
    fps.truncate(degree);
    fps
}

fn eval_expr(expr: &Expr, degree: usize) -> Fps {
    match expr {
        Expr::Num(n) => Fps::new(vec![n.clone()]),
        Expr::Variable(_) => Fps::from_variable(),
        Expr::Add(a, b) => &eval_expr(a, degree) + &eval_expr(b, degree),
        Expr::Sub(a, b) => &eval_expr(a, degree) - &eval_expr(b, degree),
        Expr::Mul(a, b) => &eval_expr(a, degree) * &eval_expr(b, degree),
        Expr::Div(a, b) => eval_expr(a, degree).div(&eval_expr(b, degree), degree),
        Expr::Pow(a, b) => {
            let base = eval_expr(a, degree);
            if let Expr::Num(n) = &**b {
                if let Some(exp) = n.to_integer().to_u32() {
                    let mut res = Fps::one();
                    if exp == 0 {
                        return res;
                    }
                    let mut cur = base;
                    let mut exp = exp;
                    while exp > 0 {
                        if exp % 2 == 1 {
                            res = &res * &cur;
                        }
                        cur = &cur * &cur;
                        exp /= 2;
                    }
                    return res
                }
            }
            panic!("Exponent must be a non-negative integer");
        }
        Expr::Neg(a) => -&eval_expr(a, degree),
    }
}
