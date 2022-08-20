use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

mod vm;

fn main() -> Result<()> {
    // MVP: parse our prefix calculator
    // (+ 1 (+ 2 3))

    let mut rl = Editor::<()>::new()?;
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match vm::parse(&line) {
                    Ok(value) => {
                        let mut buffer = String::new();
                        vm::print_value(&mut buffer, &value);
                        println!("{}", buffer);
                    }
                    Err(err) => {
                        println!("Error: {:?}", err);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt")
}
