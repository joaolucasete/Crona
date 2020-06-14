use std::io::Error;
use crona::Module;
use crona::Process;
use std::sync::Arc;

fn main() -> Result<(),Error> {
    let module = Arc::new(Module::from_file(&"crona/bin_tests/add.crn".to_string())?);
    let mut process = Process::new(module.clone());
    process.execute();
    process.execute();
    process.execute();
    println!("{:?}",process.stack);
    Ok(())
}

