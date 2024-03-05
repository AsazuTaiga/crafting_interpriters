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
- `and`, `or` を作る
  - これらは短絡評価である点で、他の２項演算子と異なる
  - `false and sideEffect();` みたいな例では右側を評価しないよね
- BNF
  ```
  expression  -> assignment ;
  assingment  -> IDENTIFIER "=" assignment
               | logic_or ;
  logic_or    -> logic_and ( "or" logic_and )* ;
  logic_and   -> equality ( "and" equality )* ;
  ```
  - `assignment` と `equality` の間に `and`, `or` が来る
- `and`, `or`を表すために`Expr.Binary` クラスを再利用する？（フィールドが同じなため）
  - 再利用しない
  - `visitBinaryExpr()`で論理演算子かどうかチェックして短絡評価のハンドリングに分岐する必要があると考えると、クラスを分けてそれぞれのvisitメソッドを作った方がよい
- GenerateAstに追記して生成、Parserに処理を追加する
- Interpreterに処理を追加する
  - 短絡評価になっていることに注意せよ
    - `visitBinaryExpr` と `visitLogicalOperator` を比較するとよりよくわかる
    - 前者がとりあえず左辺と右辺を`evaluate`して、演算子の種別ごとに評価しているのに対し、後者は `evaluate(expr.right)` をぎりぎりまで呼ばないようにしてる
  
## While ループ
- for ループより簡単らしい（それはそう）
- BNF
  ```
  statement  -> exprStmt
              | ifStmt
              | printStmt
              | whileStmt
              | block ;
  whileStmt  -> "while" "(" expression ")" statement ;
  ```
- ルールをGenerateAst.javaに追加する
- > ここで、式と文に個別の基本クラスを用意することがなぜ良いのかがわかります。フィールド宣言により、条件が式であり本文が文であることが明確になります。
  - まあそれはそうだが、それはそうだろという感じ（個別の基本クラスを用意しないわけないだろと思っていたので）
- Parser.javaに以下を追加
  - 先頭のキーワード `while` を検出して判定し、`whileStatement()`に分岐させる
  - `whileStatement()` で以下の順にトークンを消費
    - `(`
    - `expression()` で条件を取り出す
    - `)`
    - `statement()` で本文を取り出す
    - `return new Stmt.While(condition, body)` する
- Interpreter.javaに以下を追加
  - `visitWhileStmt(Stmt.While stmt)` で、Javaのwhile文をそのまま使って、条件がtruthyであるかどうか評価し、その場合には本文を実行