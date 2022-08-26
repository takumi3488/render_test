# Rails renderメソッドのシンボル/文字列比較

## 結果

5000回ずつ計測しました。

||シンボル[μs]|文字列[μs]|比(シンボルを100とする)|
|---|---|---|---|
|誤差込み平均値|2183.26±5149.94|2134.57±4505.70|97.77|
|中央値|1655.92|1645.04|100.66|

平均値は文字列の方が速いですが、中央値はシンボルの方が速いです。

誤差が大きいので比の部分は何度が計測すると簡単に逆転しますが、これくらいの差であれば速度差は全くないと結論付けて良さそうです。

## 計測方法

本リポジトリをクローンして、フォルダ内で以下のコマンドを実行してください。

```sh
bash bin/bench
```

`result.csv`の結果が更新されます。

実行には、Ruby(3.1.2)と2021editionのCargoが必要です。
