use super::inst::Instructions;
use std::io::Error;
use std::env;
use std::fs::File;

// Module is a struct that stores the code of a single MOD so it can easily hot reload in the future.
pub struct Module{
    code: Vec<Instructions>,
    data: Vec<u8>
}

impl Module {
    pub fn from_file(file_name: &String) -> Result<Module,Error> {
        let file = File::open(file_name)?;
        Ok(Module {
            code: Vec::new(),
            data: Vec::new()
        })
    }
}