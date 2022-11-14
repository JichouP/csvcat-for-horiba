# csvcat-for-horiba

堀場分光器向けの csv 結合ツール

Cyclic モードで積算回数を重ねたときに出力される `csv` ファイルを各 CH ごとにまとめます

## 使い方

1. [GitHub Release](https://github.com/JichouP/csvcat-for-horiba/releases/latest) から最新版の実行ファイルをダウンロード
1. 解凍した実行ファイルを `csv` が出力されている場所に保存
1. 実行ファイルを実行する
1. `aggregated` フォルダが生成され、その中に合計値と平均値の `csv` ファイルが出力される
