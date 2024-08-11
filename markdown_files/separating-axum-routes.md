# RSUTのaxumでルーティング設定を別ファイルで管理する方法

## ゴール
- main.rsに書いているルーティング設定を別ファイルに移行する

## 状況
管理しやすくなるよう、ルーティング設定を別ファイルへ移動しようとしましたが、日本語の記事自体が少なく探すのに手間取ってしまったので、メモに残したいと思います。

## main.rsの状態

以下のようなmain.rsの状態があったとします。

```
use axum::{routing::get, Router};

async fn root_page() {
    // 実装
}

async fn blog_page() {
    // 実装
}

fn main() {
    let app = Router::new()
        .route("/blog", get(root_page))
        .route("/blog/:file_name", get(blog_page));

    // サーバーの起動など
}
```

このように、main.rs にはすべてのルーティング設定が直接書かれており、ルーティングが増えるにつれて管理が難しくなってきます。

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

// hoge以降のルーティングを設定する
pub fn hogeRouter() -> Router {
  .route("/hoge", get(hogeHoge))    // 子ルート
  .route("/fuga", get(hogeFuga))    // 子ルート
}
```

## 子ルートをルーティング専用ファイルへ移動させる

nestを活用し、別ファイルにルーティング設定を移動します。

```
// main.rs

use crate::router::hogeRouter;    // 別ファイルのルーティングをインポート

let app = Router::new()
    .nest("/hoge", hogeRouter)
```

```
// router.rs

pub fn hogeRouter() -> Router {
  .route("/hoge", get(hogeHoge))
  .route("/fuga", get(hogeFuga))
}
``` 