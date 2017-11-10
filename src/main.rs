
extern crate julia;
extern crate colored;
extern crate rustyline;

use std::env;

use rustyline::{Config, Editor, EditMode, KeyPress, Cmd, CompletionType};
use rustyline::error::ReadlineError;
use colored::*;

use julia::api::{Julia, Value};
use julia::error::Error;

macro_rules! errprintln {
    ($msg:expr) => { eprintln!("{}", msg.bright_red().bold()); };
    ($fmt:expr, $err:expr) => {
        use std::fmt::Write;
        use std::error::Error;

        let mut msg = String::new();
        let err = match $err.cause() {
            None        => {
                write!(msg, concat!($fmt, "\n > {}"), $err, $err.description())
                    .and_then(|_| {
                        eprintln!("{}", msg.bright_red().bold());
                        Ok(())
                    })
            },
            Some(cause) => {
                write!(msg, concat!($fmt, "\n > {}\n >> {}"), $err, $err.description(), cause)
                    .and_then(|_| {
                        eprintln!("{}", msg.bright_red().bold());
                        Ok(())
                    })
            },
        };
        err.expect("Couldn't write error");
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
        "_".bright_green().bold(),
        "_".bright_blue().bold(),
        "_".bright_red().bold(),
        "(_)".bright_green().bold(),
        "_".bright_magenta().bold(),
        "(_)".bright_blue().bold(),
        "(_)".bright_red().bold(),
        "(_)".bright_magenta().bold(),
        "_".bright_yellow().bold(),
        "(_)".bright_yellow().bold()
    );
}

fn set_history(jl: &mut Julia, ret: &Value) -> Result<(), usize> {
    let ans = jl.main().global("ans").unwrap_or_else(|_| Value::nothing());
    let ans1 = jl.main().global("ans1").unwrap_or_else(
        |_| Value::nothing(),
    );
    let ans2 = jl.main().global("ans2").unwrap_or_else(
        |_| Value::nothing(),
    );
    let ans3 = jl.main().global("ans3").unwrap_or_else(
        |_| Value::nothing(),
    );
    let ans4 = jl.main().global("ans4").unwrap_or_else(
        |_| Value::nothing(),
    );
    let ans5 = jl.main().global("ans5").unwrap_or_else(
        |_| Value::nothing(),
    );
    let ans6 = jl.main().global("ans6").unwrap_or_else(
        |_| Value::nothing(),
    );
    let ans7 = jl.main().global("ans7").unwrap_or_else(
        |_| Value::nothing(),
    );
    let ans8 = jl.main().global("ans8").unwrap_or_else(
        |_| Value::nothing(),
    );
    jl.main().set("ans", ret).map_err(|_| 0_usize)?;
    jl.main().set("ans1", &ans).map_err(|_| 1_usize)?;
    jl.main().set("ans2", &ans1).map_err(|_| 2_usize)?;
    jl.main().set("ans3", &ans2).map_err(|_| 3_usize)?;
    jl.main().set("ans4", &ans3).map_err(|_| 4_usize)?;
    jl.main().set("ans5", &ans4).map_err(|_| 5_usize)?;
    jl.main().set("ans6", &ans5).map_err(|_| 6_usize)?;
    jl.main().set("ans7", &ans6).map_err(|_| 7_usize)?;
    jl.main().set("ans8", &ans7).map_err(|_| 8_usize)?;
    jl.main().set("ans9", &ans8).map_err(|_| 9_usize)?;
    Ok(())
}

fn main() {
    let mut jl = match Julia::new() {
        Ok(jl) => jl,
        Err(err) => {
            errprintln!("An error occurred while initializing Julia:\n{}", err);
            return;
        }
    };

    greet();

    let config = Config::builder()
        .completion_type(CompletionType::List)
        .history_ignore_space(true)
        .edit_mode(EditMode::Emacs)
        .build();
    let mut rl = Editor::<()>::with_config(config);
    rl.bind_sequence(KeyPress::Up, Cmd::PreviousHistory);
    rl.bind_sequence(KeyPress::Down, Cmd::NextHistory);

    let home = env::var("HOME").unwrap();
    let history_path = format!("{}/.julia-rs_history", home);

    rl.load_history(&history_path).ok();

    let ps1 = format!("{} ", "julia.rs>".bright_green().bold());
    loop {
        let line = rl.readline(&ps1);
        let line = match line {
            Ok(line) => {
                rl.add_history_entry(&*line);
                line
            }
            Err(ReadlineError::Eof) => {
                println!("^D");
                break;
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
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

        if let Err(i) = set_history(&mut jl, &ret) {
            eprintln!("Warning: couldn't set answer history at {}", i);
        }
    }
    rl.save_history(&history_path).ok();
}
