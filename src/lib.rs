extern crate rustyline;
#[macro_use]
extern crate lazy_static;

pub mod data;
pub mod utils;
pub mod vm;
pub mod compiler;
pub mod scanner;
pub mod parser;

use vm::VM;
use rustyline::{DefaultEditor, error::ReadlineError};

use utils::constants::*;
use utils::err::*;

use log::{Level, LevelFilter};
use env_logger::Env;

pub fn init_logger() {
    env_logger::Builder::from_default_env()
        .filter_module("nova", LevelFilter::Debug)
        .format_timestamp(None)
        .init();
}

pub fn nova_repl(mut vm:VM)->Result<()> {
    let mut rl = DefaultEditor::new().unwrap();

    println!();
    println!("Welcome to Nova: a highly expressive, dynamically typed functional programming language.\nType an expression to get started.\n");

    loop {
        let readline = rl.readline(">>> ");

        match readline {
            Ok(inp) => {
                let inp = inp.trim().to_string();
                if inp.len() == 0 {
                    continue;
                }

                if QUIT_STRINGS.contains(&inp.as_str()) {
                    println!("See you again!");
                    break;
                }

                if ["cl", "clear"].contains(&inp.as_str()) {
                    let _ = rl.clear_screen();
                    continue;
                }

                rl.add_history_entry(inp.clone().trim()).unwrap();

                if inp.starts_with(CMD_PREFIX) {
                    println!("Command: {}", &inp[CMD_PREFIX.len()..]);
                    continue;
                }


                match vm.interpret(&inp) {
                    Ok(val) => {
                        println!("{}", val.to_string());
                    },

                    Err(err) => {
                        println!("{}", err.to_string());
                    }
                }               
            }



            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("See you again!");
                break;
            }
            _ => (),
        }
    }
    Ok(())
}  

#[cfg(test)]
pub mod tests {
    use crate::data::ops::*;
    use crate::vm::VM;

    #[test]
    fn test_stack_ops() {
        let mut c2=Chunk::new();
    
        let idx=c2.add_constant(Value::Number(2), 1);
        let idx2=c2.add_constant(Value::Number(3), 1);
        let idx3=c2.add_constant(Value::Number(5), 1);
    
        c2.write_op(Inst::OpConstant(idx), 1);
        c2.write_op(Inst::OpConstant(idx2), 1);
        c2.write_op(Inst::OpAdd, 1); // 2+3
    
        c2.write_op(Inst::OpConstant(idx3), 1);
        c2.write_op(Inst::OpDiv, 1); // 5/5 = 1
    
        c2.write_op(Inst::OpNegate, 1); // -1
    
        c2.write_op(Inst::OpConstant(idx3), 1);
        c2.write_op(Inst::OpMul, 1); // -1*5=-5
        
    
        c2.write_op(Inst::OpReturn, 1);
    
        println!("{}", c2);
        
        let mut vm=VM::new();
        let res=vm.run(c2, true).unwrap();
    
        assert_eq!(res.to_string(), "-5");
        
    }
    

    #[test]
    fn test_concat() {
        let mut c2=Chunk::new();
    
        // Value::ValObj(Object::new("hi"))
        // Value::ValObj(Object::new(Function{...}))
    
        let string1="hi".to_string(); 
        let string2="hello".to_string();
    
        let idx=c2.add_constant(Value::ObjString(string1), 1);
        let idx2=c2.add_constant(Value::ObjString(string2), 1);
    
        c2.write_op(Inst::OpConstant(idx), 1);
        c2.write_op(Inst::OpConstant(idx2), 1);
        
        c2.write_op(Inst::OpConcat, 1);
        c2.write_op(Inst::OpReturn, 1);
        
        let mut vm=VM::new();
        let res=vm.run(c2, true).unwrap();
    
        assert_eq!(res.to_string(), "hihello");

        let mut c3=Chunk::new();
        let idx=c3.add_constant(Value::Bool(true), 1);
        c3.write_op(Inst::OpConstant(idx), 1);
        c3.write_op(Inst::OpReturn, 2);

        let res=vm.run(c3, true).unwrap(); // re-use possible
        assert_eq!(res.to_string(), "true");
    }

    #[test]
    pub fn test() {
        let mut chunk=Chunk::new();
        chunk.write_constant(Value::Number(1), 1);
        chunk.write_constant(Value::Number(2), 1);

        chunk.write_op(Inst::OpReturn, 1);

        let mut vm=VM::new();
        let res=vm.run(chunk, true);

        log::debug!("{:?}", res);
    }
}
