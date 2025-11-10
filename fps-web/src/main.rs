use yew::prelude::*;


// TODO: map_errを使ってキレイに書き直す(FPS計算がフロントエンドで動くようになったら)
fn calc_fps(expression: &str, max_deg: usize) -> Option<String> {

    let tokens = fps_core::tokenizer::tokenize(expression);

    if let Err(e) = tokens {
        //eprintln!("Tokenization error: {:?}", e);
        web_sys::console::log_1(&format!("Tokenization error: {:?}", e).into());
        return None;
    }
    let tokens: Vec<fps_core::tokenizer::Token>  = tokens.unwrap();
    let expr = fps_core::parser::parse(&tokens);

    if let Err(e) = expr {
        //eprintln!("Parsing error: {:?}", e);
        web_sys::console::log_1(&format!("Parsing error: {:?}", e).into());
        return None;
    }
    let expr = expr.unwrap();

    let series = fps_core::evaluator::evaluate(&expr, max_deg);

    if let Err(e) = series {
        //eprintln!("Evaluation error: {:?}", e);
        web_sys::console::log_1(&format!("Evaluation error: {:?}", e).into());
        return None;
    }
    let series = series.unwrap();

    return Some(format!("{}", series));

}

#[function_component]
fn App() -> Html {
    let expression: UseStateHandle<String> = use_state(|| String::from("1/(1-x)"));
    let max_degree: UseStateHandle<usize> = use_state(|| 5);
    let result: UseStateHandle<String> = use_state(|| String::new());


    // 入力欄をバインド(バインドってどういうことだよ)
    // 「入力欄をバインドする」というのは、UIコンポーネントと状態(State)を双方向でつないで、入力内容とアプリ内部の値が常に一致するようにすること。
    // YewはReactと同じ単方向データフローなので(といってもReactを知らないのでわからないが)、
    // 状態→inputの valueと inputからのイベント→状態更新の2本を書けばバインディング完了
    use web_sys::HtmlInputElement; // web-sysは、WebAssemblyでブラウザ向けに書くときDOMやWindowなどのブラウザのWebAPIをRustから型型安に操作するためのバインディング集(codex談)

    // 入力が変わったときに新しい文字列をstateに入れるコールバック
    let on_expr_change: Callback<InputEvent> = {
        let expression: UseStateHandle<String> = expression.clone();
        let result: UseStateHandle<String> = result.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into::<HtmlInputElement>();
            expression.set(input.value());
            // console.logと同じ感じ(たぶん)
            web_sys::console::log_1(&format!("Expression changed: {}", input.value()).into());

            let res: Option<String> = calc_fps(&input.value(), *max_degree);
            let res: String = res.unwrap_or("error".to_string());
            result.set(res);
            web_sys::console::log_1(&format!("Calculation result: {:?}", result).into());
        })
    };


    html! {
        <>
        <input value={(*expression).clone()} oninput={on_expr_change} />

        // 計算結果
        {
            html! { <pre>{(*result).clone()}</pre> }

        }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}