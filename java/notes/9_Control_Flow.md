# 9. Control Flow

- if, 論理演算子, while, forの制御構文を実装していくチャプター
- チューリングマシンについての概要
  - チューリングマシンとラムダ計算
  - > 必要なのは、関数をチューリング マシンに変換し、それをシミュレーターで実行することだけです。

## if

- if文の定義が文の定義に追加される
  - ifはelseを伴ったり伴わなかったりする
  ```
  statement     -> exprStmt
                  | ifStmt
                  | printStmt
                  | block ;
  ifStmt        -> "if" "(" expression ")" statement
                  ( "else" statement )? ;
  ```
- GenerateAstに定義を追加して、`class If extends Stmt`をつくる
- Parserに処理を追加する
  - `"if"` にマッチしたら `ifStatement` を呼び出す
  - `condition` を取り出す
  - `thenBranch` を取り出す
  - `elseBranch` を取り出す 当然 `null` になることもある
  - `new Stmt.If` する
- ダングリング else 問題
  - `if (first) if (second) whenTrue(); else whenFalse();` のelseは最初のif条件に対するものか、二番目に対するものか？
    - Loxの場合はelseにもっとも近いifに束縛される
      - というか再帰下降パーサーはだいたいそうなる
- Interpreterに処理を追加する

## 論理演算子
- 