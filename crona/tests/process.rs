use std::io::Error;
use crona::Module;
use crona::Process;
use crona::vm::process::StackValue;
use std::sync::Arc;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process() -> Result<(),Error>{
        use StackValue::*;
        
        let module = Arc::new(Module::from_file(&"bin_tests/add.crn".to_string())?);
        let mut process = Process::new(module.clone());
        process.execute();
        process.execute();
        assert_eq!(process.stack,vec![Num(4),Num(12)]);
        process.execute();
        assert_eq!(process.stack,vec![Num(16)]);
        Ok(())
    }

}