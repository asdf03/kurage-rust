# RSUTのaxumでルーティング設定を別ファイルで管理する方法

## ゴール
- main.rsに書いているルーティング設定を別ファイルに移行する

## 状況
管理しやすくなるよう、ルーティング設定を別ファイルへ移動しようとしましたが、日本語の記事自体が少なく探すのに手間取ってしまったので、メモに残したいと思います。

## nestを使用してルーティングのネストを行う

「nest」を使用する事でルーティングを親ルート、子ルートという風に分ける事ができます

```
let app = Router::new()
    .route("/hoge/hoge", get(hogeHoge))
    .route("/hoge/fuga", get(hogeFuga))
```

↓

```
// hogeのルーティングを受け止め、hogeRoute()に渡す
let app = Router::new()
    .nest("/hoge", post(hogerRouter))    // 親ルート
    ... // その他設定

// hoge以降のルーティングを設定する
pub fn hogeRouter() -> Router {
  .route("/hoge", get(hogeHoge))    // 子ルート
  .route("/fuga", get(hogeFuga))    // 子ルート
}
```

## 全てのルーティング設定を別ファイルで管理するする

nestを活用し、別ファイルにルーティング設定を全て書いていきます。

```
// main.rs

use crate::routers::route;

let app = Router::new()
    .nest("/", route)
    .... // その他設定
```

```
// routers.rs

pub fn route() -> Router {
  .route("/route1", get(getHoge1))
  .route("/route2", get(getHoge2))
  .route("/route3", get(getHoge3))
  .route("/route4/fuga", get(getFuga1))
  .route("/route5/fuag", get(getFuga2))
}
```