use std::process::Command;

fn run_cli(expression: &str) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_fps-cli"))
        .arg(expression)
        .output()
        .expect("failed to run fps-cli");

    assert!(output.status.success(), "CLI exited with failure");
    String::from_utf8(output.stdout)
        .expect("stdout should be valid UTF-8")
        .trim()
        .to_string()
}

#[test]
fn prints_series_for_sin() {
    let output = run_cli("sin(3x)");
    assert_eq!(output, "3 x - 9/2 x^3 + O(x^4)");
}

#[test]
fn prints_series_for_exp() {
    let output = run_cli("exp(5x)");
    assert_eq!(output, "1 + 5 x + 25/2 x^2 + 125/6 x^3 + O(x^4)");
}

#[test]
fn prints_series_for_log() {
    let output = run_cli("log(1 + 7x)");
    assert_eq!(output, "7 x - 49/2 x^2 + 343/3 x^3 + O(x^4)");
}
