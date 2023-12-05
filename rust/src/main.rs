// このコードでは crafting interpretersという本の内容を写経しています。 
// しかし、本ではサンプルコードがJavaで書かれているので、Rustで書き直しています。

use std::env;
use std::process;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::cell::RefCell;
use std::rc::Rc;

mod token;

struct Lox {
    had_error: Rc<RefCell<bool>>,
}

impl Lox {
    /// このコードは、ファイルを読み込んでrun関数に渡している。
    /// `?`は、Result型を返す関数の呼び出し時にエラーが発生した場合に、
    /// そのエラーを呼び出し元に返すためのもの。
    /// `Path::new(&path)`は、与えられた`path`（String型）を参照し、
    /// Pathオブジェクトを生成している。
    /// `fs::read_to_string`は、Pathオブジェクトを元にファイルの内容を文字列として読み込む。
    fn run_file(&self, path: &String) -> Result<(), io::Error> {

        let bytes = fs::read_to_string(Path::new(&path))?;
        self.run(&bytes);
        Ok(())
    }

    fn run_prompt(&self) -> Result<(), io::Error> {
        let stdin = io::stdin();
        let mut reader = stdin.lock();
        let mut buffer = String::new();
    
        loop {
            print!("> ");
            io::stdout().flush()?;

            if reader.read_line(&mut buffer)? == 0 {
                break;
            }
            
            self.run(&buffer);
            buffer.clear();
        }
    
        Ok(())
    }

    // Q. このコードは何をしているのか？
    // A. このコードは、エラーが発生した場合に、
    //   そのエラーを呼び出し元に返すためのもの。
    // Rcは、参照カウントを行うための型。
    // RefCellは、可変の値を持つことができる型。
    fn new () -> Lox {
        Lox {
            had_error: Rc::new(RefCell::new(false)),
        }
    }

    fn error(&self, line: usize, message: &str) {
        self.report(line, "", message);
    }
    
    fn report(&self, line: usize, location: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, location, message);
        *self.had_error.borrow_mut() = true;
    }

    /// 与えられたソースコードからトークンをスキャンし、それらを一つずつ表示する。
    /// TODO: トークンを表示する構造体とそれらのトークンを生成するScanaer構造体を作成する。
    fn run(&self, source: &String) {
        // 後で実装する
        // 
}


    
}



// Javaコード
// private static void run(String source) {
//     Scanner scanner = new Scanner(source);
//     List<Token> tokens = scanner.scanTokens();

//     // For now, just print the tokens.
//     for (Token token : tokens) {
//       System.out.println(token);
//     }
//   }

// Rustコード
struct Scanner {
    
}
struct Token {
    
}


/// このコードは、コマンドライン引数を受け取り、
/// 引数が1つの場合はrun_file関数を呼び出し、
/// 引数がない場合はrun_prompt関数を呼び出している。
fn main() {
    let args: Vec<String> = env::args().collect();
    let lox = Lox::new();
    
    match args.len() {
        2 => {
            if let Err(e) = lox.run_file(&args[1]) {
                eprintln!("Error running file: {}", e);
                process::exit(65); // エラーコード65で終了
            }
        }
        1 => {
            if let Err(e) = lox.run_prompt() {
                eprintln!("Error running prompt: {}", e);
                process::exit(66); // エラーコード66で終了
            }
        }
        _ => {
            println!("Usage: rlox [script]");
            process::exit(64); // エラーコード64で終了
        }
    }
    
    if *lox.had_error.borrow() {
        process::exit(65); // スキャンまたはパースエラーがあった場合の終了コード
    }
}
