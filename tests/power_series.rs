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
