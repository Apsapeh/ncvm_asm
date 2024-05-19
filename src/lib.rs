#![no_main]

mod compiler;
mod opcodes;
mod parser;
mod precompiler;
mod types;

pub fn compile_asm(src: String) -> Result<Vec<u8>, String> {
    //println!("{:?}", static_memory_var_map);
    //println!("{:?}", block_map);
    let (static_memory_var_map, block_map) = parser::parse(src)?;
    let (lib_functions, compiled_static_mem, compiled_commands) =
        precompiler::precompile(static_memory_var_map, block_map);

    let bytecode = compiler::compile(lib_functions, compiled_static_mem, compiled_commands);

    //std::fs::write("foo.bin", bytecode).unwrap();
    Ok(bytecode)
}
