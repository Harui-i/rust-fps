use fps_core::{evaluator, parser, tokenizer};

fn evaluate(expr: &str, max_degree: usize) -> String {
    let tokens = tokenizer::tokenize(expr).expect("tokenize");
    let ast = parser::parse(&tokens).expect("parse");
    let series = evaluator::evaluate(&ast, max_degree).expect("evaluate");
    format!("{}", series)
}

#[test]
fn geometric_series_matches_expected_output() {
    let formatted = evaluate("1 / (1 - x)", 3);
    assert_eq!(formatted, "1 + x + x^2 + x^3 + O(x^4)");
}

#[test]
fn sin_series_matches_expected_output() {
    let formatted = evaluate("sin(3x)", 3);
    assert_eq!(formatted, "3 x - 9/2 x^3 + O(x^4)");
}

#[test]
fn cos_series_matches_expected_output() {
    let formatted = evaluate("cos(5x)", 4);
    assert_eq!(formatted, "1 - 25/2 x^2 + 625/24 x^4 + O(x^5)");
}

#[test]
fn exp_series_matches_expected_output() {
    let formatted = evaluate("exp(5x)", 3);
    assert_eq!(formatted, "1 + 5 x + 25/2 x^2 + 125/6 x^3 + O(x^4)");
}

#[test]
fn log_series_matches_expected_output() {
    let formatted = evaluate("log(1 + 7x)", 3);
    assert_eq!(formatted, "7 x - 49/2 x^2 + 343/3 x^3 + O(x^4)");
}
