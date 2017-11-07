
extern crate julia;
extern crate colored;
extern crate rustyline;

use rustyline::Editor;
use rustyline::error::ReadlineError;
use colored::*;

use julia::Julia;
use julia::error::Error;


fn greet() {
    println!(
        r#"
               {}
   {}       _ {}{}{}
  {}     | {} {}
   _ _   _| |_  __ _     _  _  __
  | | | | | | |/ _` | _ | |/ // _)
  | | |_| | | | (_| |/ \|  ,/ \_ \
 _/ |\__'_|_|_|\__'_|\_/|_|   (__/
|__/
"#,
        "_".bright_green(),
        "_".bright_blue(),
        "_".bright_red(),
        "(_)".bright_green(),
        "_".bright_magenta(),
        "(_)".bright_blue(),
        "(_)".bright_red(),
        "(_)".bright_magenta()
    );
}

fn main() {
    let mut jl = match Julia::new() {
        Ok(jl) => jl,
        Err(err) => {
            eprintln!("An error occured while initializing Julia:\n{:?}", err);
            return;
        }
    };
    let mut rl = Editor::<()>::new();

    // TODO: replace by implementing Display on Value
    let println = match jl.base().function("println") {
        Ok(fun) => fun,
        Err(err) => {
            eprintln!(
                "An error occured while getting the `println` function:\n{:?}",
                err
            );
            return;
        }
    };

    greet();

    loop {
        let line = rl.readline("julia.rs> ");
        let line = match line {
            Ok(line) => line,
            Err(ReadlineError::Eof) => break,
            Err(ReadlineError::Interrupted) => continue,
            Err(err) => {
                eprintln!("Error: {:?}", err);
                continue;
            }
        };

        let ret = jl.eval_string(line);
        let ret = match ret {
            Ok(ret) => ret,
            Err(Error::UnhandledException(ex)) => {
                eprintln!("Exception occured: {:?}", ex);
                continue;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                continue;
            }
        };

        if !ret.is_nothing() {
            println.call1(&ret).expect("Fatal error occured!");
        }
    }
}
