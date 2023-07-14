use std::cell::RefCell;
use std::rc::Rc;

use crate::ops::{*, Inst::*};
use crate::err::*;
use crate::stack::FixedStack;


// new(chunk), execute()->Result



pub struct VM<'c> {
    chunk:Chunk<'c>, // 'c: lifetime of Chunk
    ip:usize, // index of next op to execute,
    value_stack:FixedStack<Value<'c>> // vals come from chunk
}

impl<'c>  VM<'c> {
    pub fn new(chunk:Chunk<'c> )->Self {
        VM {
            chunk:chunk,
            ip:0,
            value_stack:FixedStack::new()
        }
    }

    // get current instruction and increase ip
    pub fn get_curr_inst(&self)->Option<&Inst> {
        let curr=self.chunk.get_op(self.ip);
        // self.ip+=1;
        curr
    }

    // &'c mut VM<'c> - the excl. ref must live as long as the object => we can't take any other refs once the 
        // ref is created
    // &mut VM<'c> -> an exclusive ref to VM that has its own lifetime
    pub fn run(&'c mut self)->Result<Value> {
        // reset
        self.ip=0;
        self.value_stack.clear();

        // numeric bin op
        macro_rules! bin_op {
            ($op:tt) => {
                {
                    let stack=&mut self.value_stack;
                    let right=stack.pop()?.expect_int()?;
                    let left=stack.pop()?.expect_int()?;
                    stack.push(Value::num(left $op right))?;
                }
            };
        }

        loop {
            let curr=self.get_curr_inst();
            if curr.is_none() {
                break Ok(Value::Number(1)) // exit code 1
            }  

            let curr=curr.unwrap();

            match curr { 
                // print top of stack and break   
                OpReturn => {
                    let res=self.value_stack.pop()?;
                    break Ok(res);
                },
                // get constant at idx in chunk, push onto stack
                OpConstant(idx) => {
                    let i=*idx;

                    let ct=self.chunk.get_constant(10);
                    
                    let get:Result<Value>=ct
                        .ok_or(errc_i!("Invalid index for constant:{}", i));

                    let get=get?;
                    self.value_stack.push(get)?;
                },
                OpNegate => {
                    let mut stack=&mut self.value_stack;
                    let top=stack.pop()?.expect_int()?;
                    stack.push(Value::num(top*-1))?;
                },
                OpAdd => bin_op!(+),
                OpSub => bin_op!(-),   
                OpMul => bin_op!(*),
                OpDiv => bin_op!(/),    
                OpConcat => {
                    let mut stack=&mut self.value_stack;
                    let left=stack.pop()?;
                    let left=left.expect_string()?;
                    let mut left=left.clone();

                    left.push_str("hi");
                    
                    // let new_val=Value::ObjString(&left);
                    // stack.push(new_val);
                    
                }     
            }

            // advance ip - may cause issue since ip advanced before match (unavoidable)
            self.ip+=1;
        }
    }
}