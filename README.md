# csvcat-for-horiba

堀場分光器向けの csv 結合ツール

Cyclic モードで積算回数を重ねたときに出力される `csv` ファイルを各 CH ごとにまとめます

## 使い方

1. [GitHub Release](./releases/latest) から最新版の実行ファイルをダウンロード
1. 実行ファイルを `csv` が出力されている場所に保存
1. 実行ファイルを実行する
1. `aggregated` フォルダが生成され、その中に合計値と平均値の `csv` ファイルが出力される

## Apple Silicon 向け

GitHub Actions で arm 向けのビルドを行うのが困難なので、手元でビルドする必要があります

1. Rust をインストールする
   - [Rustup](https://www.rust-lang.org/ja/tools/install) を使用する
1. このリポジトリをクローンする
1. `cargo b -r` を実行してコンパイルする
1. `target/release/csvcat-for-horiba` が生成される
1. 実行ファイルを `csv` が出力されている場所に保存
1. 実行ファイルを実行する
1. `aggregated` フォルダが生成され、その中に合計値と平均値の `csv` ファイルが出力される