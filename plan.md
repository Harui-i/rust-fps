了解です。
では、ここまでのディスカッション内容を、**実装者に伝えるための技術設計メモ**として整理したものを以下にまとめます。

````{"id":"49201","variant":"standard","title":"形式的べき級数展開ツール（CLI＋WASMフロントエンド）設計概要"}
# 形式的べき級数展開ツール（CLI＋WASMフロントエンド）設計概要

## 🎯 プロジェクト概要

入力された式（例: `2exp(3x/(2!)) / ((1-x)^3)`）を形式的べき級数（FPS: Formal Power Series）として展開し、  
係数を有理数で出力するツールを開発する。

最終出力例：
```
2 + 9x + 113/4x^2 + O(x^3)
```

FPSの演算部分では、高速化を目的とはせず、O(N^2)程度の単純実装で十分とする。
C++の参考実装: `cpp_ref_lib.md` (ただし、シグネチャレベルで真似する必要はない)
本プロジェクトでは主に **CLI** と **Web（WASM）** インタフェースを提供する。

---

## 🧩 全体構成（Cargo workspace）

```
fps-project/
├─ Cargo.toml (workspace root)
├─ fps-core/   ← 共通ライブラリ（トークン化・構文解析・FPS展開）
├─ fps-cli/    ← CLIバイナリ
└─ fps-web/    ← WASM + Yew フロントエンド
```

---

## 🧱 各クレートの役割

### 1. `fps-core`
形式的べき級数エンジン。

#### 主なモジュール構成
- `tokenizer.rs`: 文字列 → トークン列
- `parser.rs`: トークン列 → 構文木 (`Expr`)
- `fps.rs`: 構文木 → FPS展開（外部ライブラリ利用可）
- `lib.rs`: 公開APIまとめ

#### 主な公開関数
```rust
fn tokenize(input: &str) -> Result<Vec<Token>, Error>;
fn parse(tokens: &[Token]) -> Result<Expr, Error>;
fn expand(expr: &Expr, order: usize) -> FormalPowerSeries;
fn to_pretty_string(series: &FormalPowerSeries) -> String;
```

#### 主な依存クレート
- `num-rational`（有理数計算）
- `serde` / `serde_json`（構造体のシリアライズ）
- `thiserror`（エラー管理）

---

### 2. `fps-cli`
コマンドラインインタフェース。

#### 使用例
```bash
cargo run -p fps-cli -- "2exp(3x/(2!)) / ((1-x)^3)" --order 3
```

#### 主な処理フロー
1. 引数から入力式・展開次数を取得
2. `fps-core` の API を呼び出し
3. 結果を標準出力に表示（またはJSON形式で出力）

#### 主な依存クレート
- `clap`（コマンドライン引数パーサ）
- `anyhow`（エラーハンドリング）

---

### 3. `fps-web`
ブラウザ上で動作する WASM アプリ。  
入力フォームから式を受け取り、リアルタイムに展開結果を表示。

#### 技術スタック
- `yew`: フロントエンドフレームワーク（React類似のRust製）
- `wasm-bindgen` / `wasm-pack`: WASMビルド
- `fps-core`: 計算ロジックをWASM側で再利用

#### UIイメージ
```
┌───────────────────────────────┐
│ 入力式: [ 2exp(3x/(2!)) / ((1-x)^3) ]       │
│ 展開次数: [ 3 ]                             │
│ [計算]                                       │
├───────────────────────────────┤
│ 出力: 2 + 9x + 113/4x^2 + O(x^3)            │
└───────────────────────────────┘
```

#### 主なコンポーネント
- `App`: 全体管理
- `InputPanel`: 式・次数の入力フォーム
- `OutputPanel`: 結果表示領域

---

## 🧠 処理の流れ（共通）

```
入力文字列
   ↓
[Tokenizer] : "2exp(3x/(2!)) / ((1-x)^3)"
   ↓
[Parser] : Expr { Mul(Div(Exp(...))) }
   ↓
[FPS展開モジュール] : FPSオブジェクト
   ↓
[Formatter] : "2 + 9x + 113/4x^2 + O(x^3)"
```

---

## 🔧 拡張予定

- `log`, `sin`, `cos` などの解析的展開をサポート
- 出力形式に LaTeX / MathJax 対応を追加
- FPS計算精度の設定（次数や有理数→小数変換）
- Web側でのリアルタイム更新（入力中に自動再計算）

---

## 🧪 実装順序（推奨）

| ステップ | 内容 |
|----------|------|
| ① | `fps-core`: `tokenize`, `parse`, `expand` のダミー実装 |
| ② | `fps-cli`: CLI入力→出力確認 |
| ③ | `fps-web`: Yewアプリ（WASM）で入力フォーム作成 |
| ④ | `fps-core`: 実際のFPS計算アルゴリズムを統合 |
| ⑤ | 出力整形・エラーメッセージ強化 |

---

## ✅ 完成イメージ

- CLI版：即時評価
  ```
  $ cargo run -p fps-cli -- "exp(x)/(1-x)" --order 4
  1 + 2x + 5/2x^2 + 4/3x^3 + O(x^4)
  ```
- Web版：ブラウザで即時計算・表示（完全Rust+WASM）

---

**開発メモ:**  
この設計では、CLI・Webのどちらも `fps-core` を共通利用するため、  
1箇所の実装を更新すれば両方に反映される。  
WASMビルド時は`cd fps-web && trunk serve` などで動作確認可能。
````

このメモをドキュメントとして共有すれば、チームメンバーがすぐに実装に入れます。
次に進めるなら、「`fps-core` の `Expr`（構文木）設計」または「`fps-web` の最小Yew UI構成」から着手するのが自然です。どちらからやりますか？
