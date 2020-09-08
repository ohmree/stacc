use crate::lexer::{Lexer, Token};
use std::collections::HashMap;

// TODO: handle pointers (maybe?).
// On one hand we need floats for math, on the other hand
// we can't store addresses in them (at least I don't think).
type Stack = Vec<f64>;
type Function = Box<dyn Fn(&mut Stack)>;

pub struct Interpreter {
    stack: Stack,
    words: HashMap<String, Function>,
}

impl Interpreter {
    pub fn new() -> Self {
        let stack = Stack::new();
        let mut words = HashMap::new();

        let add: Function = Box::new(|stack: &mut Stack| {
            let n1 = stack.pop().unwrap();
            let n2 = stack.pop().unwrap();
            stack.push(n1 + n2);
        });

        let sub: Function = Box::new(|stack: &mut Stack| {
            let n1 = stack.pop().unwrap();
            let n2 = stack.pop().unwrap();
            stack.push(n2 - n1);
        });

        let mul: Function = Box::new(|stack: &mut Stack| {
            let n1 = stack.pop().unwrap();
            let n2 = stack.pop().unwrap();
            stack.push(n1 * n2);
        });

        let div: Function = Box::new(|stack: &mut Stack| {
            let n1 = stack.pop().unwrap();
            let n2 = stack.pop().unwrap();
            stack.push(n1 / n2);
        });

        let print: Function = Box::new(|stack: &mut Stack| {
            println!("{}", stack.pop().unwrap());
        });

        let exit: Function = Box::new(|_| {
            std::process::exit(0);
        });

        // TODO: implement stack manipulation words

        words.insert("+".to_string(), add);
        words.insert("-".to_string(), sub);
        words.insert("*".to_string(), mul);
        words.insert("/".to_string(), div);
        words.insert(".".to_string(), print);
        words.insert("exit".to_string(), exit);
        // TODO: maybe use a reference counted pointer to store functions?
        // words.insert("quit".to_string(), exit);
        // words.insert("bye".to_string(), exit);

        Self { stack, words }
    }

    pub fn eval(&mut self, code: String) -> anyhow::Result<()> {
        let lexer = Lexer::new(&code);
        for token in lexer {
            match token {
                Token::Number(num) => self.stack.push(num),
                Token::Word(word) => self.words[&word](&mut self.stack),
                _ => {}
            }
        }

        Ok(())
    }

    pub fn print_stack(&self) {
        if self.stack.len() > 0 {
            for (idx, num) in self.stack.iter().enumerate() {
                println!("[{}] {}", idx, num);
            }
        }
    }
}
