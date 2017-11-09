
extern crate julia;
extern crate colored;
extern crate rustyline;

use rustyline::Editor;
use rustyline::error::ReadlineError;
use colored::*;

use julia::api::Julia;
use julia::error::Error;

macro_rules! errprintln {
    ($fmt:expr) => { eprintln!($fmt); };
    ($fmt:expr, $err:expr) => {
        use std::error::Error;
        match $err.cause() {
            None        => eprintln!(concat!($fmt, "\n > {}"), $err, $err.description()),
            Some(cause) => eprintln!(concat!($fmt, "\n > {}\n >> {}"), $err, $err.description(), cause),
        }
    }
}

fn greet() {
    println!(
        r#"
               {}
   {}       _ {}{}{}
  {}     | {} {}
   _ _   _| |_  __ _     _  _  __
  | | | | | | |/ _` |   | |/ // _)
  | | |_| | | | (_| | {} |  ,/ \_ \
 _/ |\__'_|_|_|\__'_|{}|_|   (__/
|__/
"#,
        "_".bright_green(),
        "_".bright_blue(),
        "_".bright_red(),
        "(_)".bright_green(),
        "_".bright_magenta(),
        "(_)".bright_blue(),
        "(_)".bright_red(),
        "(_)".bright_magenta(),
        "_".bright_yellow(),
        "(_)".bright_yellow()
    );
}

fn main() {
    let mut jl = match Julia::new() {
        Ok(jl) => jl,
        Err(err) => {
            errprintln!("An error occurred while initializing Julia:\n{}", err);
            return;
        }
    };
    let mut rl = Editor::<()>::new();

    greet();

    loop {
        let line = rl.readline("julia.rs> ");
        let line = match line {
            Ok(line) => line,
            Err(ReadlineError::Eof) => break,
            Err(ReadlineError::Interrupted) => continue,
            Err(err) => {
                errprintln!("Error: {}", err);
                continue;
            }
        };

        let ret = jl.eval_string(line);
        let ret = match ret {
            Ok(ret) => ret,
            Err(Error::UnhandledException(ex)) => {
                errprintln!("Exception: {}", ex);
                continue;
            }
            Err(err) => {
                errprintln!("Error: {}", err);
                continue;
            }
        };

        if !ret.is_nothing() {
            println!("{}", ret);
        }
    }
}
