use std::io::Error;
use crona::Module;
use crona::vm::Instruction;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_module() -> Result<(),Error>{
        let module = Module::from_file(&"bin_tests/add.crn".to_string())?;
        
        assert_eq!(module.code, vec![
            Instruction::Const(0),
            Instruction::Const(4),
            Instruction::Add,
            Instruction::Halt
        ]);

        assert_eq!(*module.jump_table.get(&0).unwrap(),0);
        Ok(())
    }

}