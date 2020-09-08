use stacc::interpreter::Interpreter;
use std::io::Write;

fn main() -> anyhow::Result<()> {
    ctrlc::set_handler(move || {
        std::process::exit(0);
    })
    .unwrap();

    let mut interpreter = Interpreter::new();
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    // print!("> ");
    loop {
        interpreter.print_stack();
        print!("> ");
        stdout.flush()?;
        let mut code = String::new();
        stdin.read_line(&mut code)?;
        interpreter.eval(code)?;
    }

    Ok(())
}
