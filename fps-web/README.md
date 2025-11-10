# これはなに

FPS計算をWebブラウザ上でできるようにするための、フロントエンド部分。 Yew(v 0.2.1)を使っている。

# 環境構築

yewではwasmを使うので、すこし準備が必要。
https://yew.rs/docs/getting-started/introduction を参考にしている。

## wasm ターゲットを追加する。

```
rustup target add wasm32-unknown-unknown
```

## trunk

trunkをインストールする。

```
cargo install --locked trunk
```


# 動かす

```
trunk serve
```