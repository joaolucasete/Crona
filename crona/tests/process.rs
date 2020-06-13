use std::io::Error;
use crona::Module;
use crona::Process;
use crona::vm::Instruction;
use std::sync::Arc;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process() -> Result<(),Error>{
        let module = Arc::new(Module::from_file(&"bin_tests/add.crn".to_string())?);
        let process = Process::new(module.clone());
        
        Ok(())
    }

}