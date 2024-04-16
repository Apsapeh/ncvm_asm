#![no_main]

mod opcodes;
mod types;
mod parser;
mod compiler;


pub fn compile_asm(src: String) -> Result<(), String> {
    

    //println!("{:?}", static_memory_var_map);
    //println!("{:?}", block_map);
    let (static_memory_var_map, block_map) = parser::parse(src)?;
    compiler::compile(
        static_memory_var_map,
        block_map
    );
    Ok(())
}