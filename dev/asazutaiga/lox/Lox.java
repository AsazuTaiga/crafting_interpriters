package dev.asazutaiga.lox;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.nio.charset.Charset;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;

public class Lox {
  static boolean hadError = false;

  public static void main(String[] args) throws IOException {
    // 引数１個より多いなら終了
    if (args.length > 1) {
      System.out.println("Usage: jlox [script]");
      System.exit(64);
      // 引数１個ならファイル読み取りモードで動かす
    } else if (args.length == 1) {
      runFile(args[0]);
      // 引数なしならプロンプトモード
    } else {
      runPrompt();
    }
  }

  /**
   * FileモードでRunします
   * 
   * @param path
   * @throws IOException
   */
  private static void runFile(String path) throws IOException {
    byte[] bytes = Files.readAllBytes(Paths.get(path));
    run(new String(bytes, Charset.defaultCharset()));

    // エラーがあることをExit codeで知らせる
    if (hadError)
      System.exit(65);
  }

  /**
   * PromptモードでRunします
   * 
   * @throws IOException
   */
  private static void runPrompt() throws IOException {
    InputStreamReader input = new InputStreamReader(System.in);
    BufferedReader reader = new BufferedReader(input);

    for (;;) {
      System.err.println("> ");
      String line = reader.readLine();
      if (line == null)
        break;
      run(line);
      hadError = false; // エラーフラグは立てるが、実行は止めない。その後の部分にエラーがある場合にユーザーに報告したいので。
    }
  }

  /**
   * Run の実体
   * 
   * @param string
   */
  private static void run(String source) {
    Scanner scanner = new Scanner(source);
    List<Token> tokens = scanner.scanTokens();

    // とりあえずコンソールにはく
    for (Token token : tokens) {
      System.out.println(token);
    }
  }

  /**
   * エラーハンドリング
   * 
   * @param line
   * @param message
   */
  static void error(int line, String message) {
    report(line, "", message);
  }

  /**
   * エラーハンドリング（行番号付き）
   * 
   * @param line
   * @param where
   * @param message
   */
  private static void report(int line, String where, String message) {
    System.err.println(
        "[line " + line + "] Error" + where + ": " + message);
    hadError = true;
  }
}
