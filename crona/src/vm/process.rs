use crate::Module;
use std::sync::Arc;
use std::convert::TryInto;
use crate::operation;

/*
 * This module represents a single process that in theory is a 
 * isolated unit of green thread that will run in the VM instance to achieve
 * the max usage of the host machine.
*/


// The instructions that the machine will execute
#[derive(Debug,PartialEq,Copy,Clone)]
pub enum Instruction {
    Halt,
    Const(u32),
    Add,
    Sub,
    Mul,
    Div
}

// Bools can be treated as numbers. so it's not necessary here.
#[derive(Debug)]
pub enum StackValue {
    Num(u32),
    Str(String)
}

// The struct Process hold the state of a single isolated green thread.
pub struct Process {
    pub stack: Vec<StackValue>,
    module: Arc<Module>,
    instruction_ptr: usize,
    halted: bool
}

impl Process {
    pub fn new(module: Arc<Module>) -> Process{
        Process {
            stack: Vec::new(),
            module,
            instruction_ptr: 0,
            halted: false
        }
    }

    pub fn pop(&mut self) -> StackValue{
        self.stack.pop().expect("Cannot pop a empty stack!")
    }

    // TODO: Improve the error handling of this part.
    pub fn push_u32_from_data(&mut self, place: u32){
        let raw_bytes: [u8;4] = self.module.data[place as usize..place as usize+4].try_into().unwrap();
        let number = u32::from_le_bytes(raw_bytes);
        self.stack.push(StackValue::Num(number));
    }

    pub fn current(&mut self) -> Option<Instruction>{
        if self.instruction_ptr < self.module.code.len() {
            Some(self.module.code[self.instruction_ptr])
        }else{
            None
        }
    }

    pub fn execute(&mut self) -> Option<u8>{
        use StackValue::*;
        use Instruction::*;

        match self.next()? {
            Halt => return Some(0),
            Const(place) => self.push_u32_from_data(place),
            Add => operation!(self, (Num(x),Num(y)) => push Num(x+y)),
            Sub => operation!(self, (Num(x),Num(y)) => push Num(x-y)),
            Mul => operation!(self, (Num(x),Num(y)) => push Num(x*y)),
            Div => operation!(self, (Num(x),Num(y)) => push Num(x/y)),
            _ => None
        };
    }
}