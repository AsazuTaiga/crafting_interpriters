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

## For ループ
- `for (var i = 0; i < 10; i = i + 1) print i;` みたいな感じ
- BNF
  ```
  statement   -> exprStmt
               | forStmt
               | ifStmt
               | printStmt
               | whileStmt
               | block ;
  forStmt     -> "for" "(" ( varDecl | exprStmt | ";" )
                 expression? ";"
                 expression? ")" statement ;
  ```
- カッコの中にセミコロンで分割された三つの句がある
  1. initializer
     - 一度だけ実行される
     - 通常は式だが、便宜上変数宣言も許可する。その場合、変数のスコープはforループの残りの部分（他の2つの句と本体）になる
  2. condition
     - ループを終了するタイミングを制御する
     - 各反復の開始時に1回評価される
     - 結果が真の場合、ループ本体が実行される  
  3. increment
     - 各ループ反復の最後になんらかの処理を実行する任意の式
     - 式の結果は破棄されるため、有用であるためには副作用が必要
     - 通常は変数をインクリメントする
- これらの句はいずれも省略できる
- Loxには上記の機能はいずれも存在するので、forは必要ない
  - **糖衣構文**である
- **desugaring**（脱糖？）処理をすることで、forループを変数宣言+whileループとして処理する方針でいく
  - なので、Ast上には`forStmt`のようなものは登場しない
- Parser.javaに以下を追加する
  - `statement()`
    - `for`にマッチしたら`forStatement()`に分岐
  - `forStatement()`
    - `initializer`, `condition`, `increment`, `body` をそれぞれ読み取る
    - 逆順 (`incremnt`, `condition`, `initializer`)の順にdesugarしていく（その方がシンプルになるらしい）
      - 書いてみてわかったが実際そう。（`body`変数の使い方な気もするが）
- テストを追加