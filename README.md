## 概要

rustを使用した高速なcmsを開発中

## 特徴

- markdownをDB代わりとする
- backendはRustを使用してRESTサーバーを立ち上げる
- front側はnext.jsを使用しSSRを行う
- CI/CDを構築

## 使用

- backendフォルダ ... Rust製のAPIサーバー (port: 8000番)
- frontendフォルダ ... Next.jsのSSR用フロントエンドサーバー (port: 3000番)

## ローカルでの確認方法 (docker不使用)

まずはbackendサーバーを立ち上げる

```
$ cd backend
$ cargo build
$ cargo run
```

プロジェクトのルートディレクトリに戻る

```
$ cd ..
```

次にfrontendサーバーを立ち上げる

```
$ cd frontend
$ yarn dev
```

ローカルのブラウザからアクセスする

```
http://localhost:3000
```