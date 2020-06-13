use super::Instruction;
use std::io::{Error, ErrorKind, Read};
use std::fs::File;
use std::collections::HashMap;

use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

// Module is a struct that stores the code of a single MOD so it can easily hot reload in the future.
pub struct Module{
    pub code: Vec<Instruction>,
    pub data: Vec<u8>,
    pub jump_table: HashMap<u32,u32>
}

impl Module {
    pub fn from_file(file_name: &String) -> Result<Module,Error> {
        let mut file = File::open(file_name)?;

        // 60350 is the magic number of each Crona Module.
        // if the file not contains it so it's not considered a executable file.
        if file.read_u16::<LittleEndian>()? != 60350 {
            return Err(Error::new(ErrorKind::Other, "The file not contains a crona binary!"));
        }

        // This is the header of a Crona file.
        let table_len = file.read_u32::<LittleEndian>()?;
        let data_len = file.read_u32::<LittleEndian>()?;
        let code_len = file.read_u32::<LittleEndian>()?;

        // Basically all the data that we need to execute a code in Crona
        let mut bin_table: Vec<u8> = vec![0; table_len as usize];
        let mut bin_data: Vec<u8> = vec![0; data_len as usize];
        let mut bin_code: Vec<u8> = vec![0; code_len as usize];

        file.read_exact(&mut bin_table)?;
        file.read_exact(&mut bin_data)?;
        file.read_exact(&mut bin_code)?;

        let jump_table = Module::bin_to_table(bin_table)?;
        let code = Module::bin_to_instructions(bin_code)?;

        Ok(Module {
            code,
            data: bin_data,
            jump_table
        })
    }

    // This function converts the raw binary data of jump_table in a jump_table
    fn bin_to_table(bin_table: Vec<u8>) -> Result<HashMap<u32,u32>,Error>{
        let mut reader = Cursor::new(bin_table);
        let mut jump_table = HashMap::new();
        while let Ok(hash) = reader.read_u32::<LittleEndian>() {
            let val = reader.read_u32::<LittleEndian>()?;
            jump_table.insert(hash,val);
        }
        Ok(jump_table)
    }

    // This function converts the binary code to a internal representation to avoid errors on run
    fn bin_to_instructions(bin_code: Vec<u8>) -> Result<Vec<Instruction>,Error> {
        let mut reader = Cursor::new(bin_code);
        let mut code = Vec::new();
        while let Ok(instruction) = reader.read_u8() {
            code.push(match instruction {
                0 => Instruction::Halt,
                1 => Instruction::Const(reader.read_u32::<LittleEndian>()?),
                2 => Instruction::Add,
                _ => return Err(Error::new(ErrorKind::Other, "Not Recognized"))
            });
        };
        Ok(code)
    }
}