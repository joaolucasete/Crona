use super::Module;
use std::sync::Arc;

#[derive(Debug,PartialEq)]
pub enum Instruction {
    Halt,
    Const(u32),
    Add
}

// Bools can be treated as numbers.
pub enum StackValue {
    Number(u32),
    Str(String)
}

// The struct Process hold the state of a single isolated green thread.
pub struct Process {
    stack: Vec<StackValue>,
    return_stack: Vec<usize>,
    module: Arc<Module>,
    inst_pointer: usize
}

impl Process {
    pub fn new(module: Arc<Module>) -> Process{
        Process {
            stack: Vec::new(),
            return_stack: Vec::new(),
            module,
            inst_pointer: 0
        }
    }
}
