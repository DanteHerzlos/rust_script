use std::{
    cell::RefCell,
    io::{self, BufRead, Write},
    rc::Rc,
};

use crate::{ast::Node, evaluator::eval, lexer::Lexer, object::Environment, parser::Parser};

const PROMT: &'static str = ">>";

pub fn start() {
    let env = Rc::new(RefCell::new(Environment::new()));
    loop {
        print!("{} ", PROMT);
        let _ = io::stdout().flush();

        let mut line = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_line(&mut line).unwrap();

        let lexer = Lexer::new(line);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        let errors = parser.get_errors();
        if errors.len() > 0 {
            print_parse_errors(errors);
            continue;
        }

        let evaluated = eval(program.as_node(), env.clone());
        if evaluated.is_some() {
            println!("{}", evaluated.unwrap().inspect())
        }
    }
}

fn print_parse_errors(errors: Vec<String>) {
    for msg in errors {
        println!("parser errors:");
        println!("\t{}", msg)
    }
}
