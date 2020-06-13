#[derive(Debug,PartialEq)]
pub enum Instruction {
    Halt,
    Const(u32),
    Add
}