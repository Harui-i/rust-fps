use yew::prelude::*;

// TODO: map_errを使ってキレイに書き直す(FPS計算がフロントエンドで動くようになったら)
fn calc_fps(expression: &str, max_deg: usize) -> Option<String> {
    let tokens = fps_core::tokenizer::tokenize(expression);

    if let Err(e) = tokens {
        web_sys::console::log_1(&format!("Tokenization error: {:?}", e).into());
        return None;
    }
    let tokens: Vec<fps_core::tokenizer::Token> = tokens.unwrap();
    let expr = fps_core::parser::parse(&tokens);

    if let Err(e) = expr {
        web_sys::console::log_1(&format!("Parsing error: {:?}", e).into());
        return None;
    }
    let expr = expr.unwrap();

    let series = fps_core::evaluator::evaluate(&expr, max_deg);

    if let Err(e) = series {
        web_sys::console::log_1(&format!("Evaluation error: {:?}", e).into());
        return None;
    }
    let series = series.unwrap();

    Some(format!("{}", series))
}

#[function_component]
fn App() -> Html {
    let expression: UseStateHandle<String> = use_state(|| String::from("1/(1-x)"));
    let max_degree: UseStateHandle<usize> = use_state(|| 5);
    let result: UseStateHandle<String> = use_state(String::new);

    {
        let result = result.clone();
        use_effect_with(
            ((*expression).clone(), *max_degree),
            move |(expr_value, deg_value): &(String, usize)| {
                let computed = calc_fps(expr_value, *deg_value)
                    .unwrap_or_else(|| "Unable to evaluate expression".to_string());
                result.set(computed);
                || ()
            },
        );
    }

    use web_sys::HtmlInputElement;

    let on_expr_change: Callback<InputEvent> = {
        let expression = expression.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into::<HtmlInputElement>();
            expression.set(input.value());
            web_sys::console::log_1(&format!("Expression changed: {}", input.value()).into());
        })
    };

    let on_degree_change: Callback<InputEvent> = {
        let max_degree = max_degree.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into::<HtmlInputElement>();
            let parsed = input.value().parse::<usize>().unwrap_or(1).clamp(1, 32);
            max_degree.set(parsed);
            web_sys::console::log_1(&format!("Max degree changed: {}", parsed).into());
        })
    };

    let preset_expressions = vec![
        ("Geometric Series", "1/(1-x)"),
        ("Exponential", "exp(x)"),
        ("Sine", "sin(x)"),
        ("Cosine", "cos(x)"),
        ("Logarithm", "log(1+x)"),
    ];

    let preset_buttons = preset_expressions.into_iter().map(|(label, value)| {
        let expression = expression.clone();
        let value_string = value.to_string();
        let label_text = label.to_string();
        html! {
            <button class="preset-chip" type="button" onclick={{
                let expression = expression.clone();
                let value_string = value_string.clone();
                Callback::from(move |_| {
                    expression.set(value_string.clone());
                })
            }}>
                <span class="preset-label">{ label_text }</span>
                <span class="preset-value">{ value }</span>
            </button>
        }
    });

    html! {
        <div class="app-shell">
            <main class="glass-card">
                <header class="hero">
                    <p class="eyebrow">{"FPS Explorer"}</p>
                    <h1>{"Formal Power Series Playground"}</h1>
                    <p class="hero-copy">
                        {"Experiment with expressions, tweak truncation degrees, and inspect the resulting series with snappy WASM-backed evaluation."}
                    </p>
                </header>

                <section class="control-panel">
                    <div class="field">
                        <label for="expression">{"Expression"}</label>
                        <div class="input-wrapper">
                            <input
                                id="expression"
                                class="text-input"
                                value={(*expression).clone()}
                                oninput={on_expr_change.clone()}
                                placeholder="e.g. 1/(1-x)"
                            />
                        </div>
                        <p class="hint">{"Supports +, -, *, /, sin, cos, exp, log, and custom fps_core functions."}</p>
                    </div>

                    <div class="field">
                        <label for="max-degree">{"Max degree"}</label>
                        <div class="input-wrapper number">
                            <input
                                id="max-degree"
                                class="text-input"
                                type="number"
                                min="1"
                                max="32"
                                value={(*max_degree).to_string()}
                                oninput={on_degree_change.clone()}
                            />
                            <span class="suffix">{"deg"}</span>
                        </div>
                        <p class="hint">{"Clamp stays between 1 and 32 to keep evaluation responsive."}</p>
                    </div>
                </section>

                <section class="preset-panel">
                    <p class="hint">{"Need inspiration? Try a preset expression:"}</p>
                    <div class="preset-grid">
                        { for preset_buttons }
                    </div>
                </section>

                <section class="result-panel">
                    <div class="result-header">
                        <h2>{"Series output"}</h2>
                        <span class="status-pill">{format!("deg ≤ {}", *max_degree)}</span>
                    </div>
                    <pre class="result-block">{(*result).clone()}</pre>
                </section>
            </main>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
