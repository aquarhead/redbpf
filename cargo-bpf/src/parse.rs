use redbpf::Module;
use std::fs;
use std::path::PathBuf;

use crate::CommandError;

pub fn parse(program: &PathBuf) -> Result<(), CommandError> {
    let module = Module::parse(&fs::read(program)?).unwrap();
    println!("{:?}", module);

    Ok(())
}
