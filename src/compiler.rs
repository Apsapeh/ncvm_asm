
use std::collections::HashMap;

use crate::types::Command;

pub fn compile(
    sm_map: HashMap<String, Vec<u8>>,
    block_map: HashMap<String, Vec<Command>>,
) {
    let mut static_memory_var_map = HashMap::with_capacity(sm_map.len());
    let mut compiled_static_memory: Vec<u8> = Vec::with_capacity(
        (&sm_map).into_iter().map(|(_, v)| v.len()).sum()
    );
    
    for (k, v) in sm_map.iter() {
        static_memory_var_map.insert(k, compiled_static_memory.len());
        compiled_static_memory.extend(v);
    }

    for (_, commands) in block_map.iter() {
        for command in commands.iter() {
            
        }
    }

    /*let mut compiled_blocks_map = HashMap::with_capacity(block_map.len());
    let mut compiled_blocks: Vec<Command> = Vec::with_capacity(
        (&block_map).into_iter().map(|(_, v)| v.len()).sum()
    );*/
    //drop(sm_map);
    //println!("{:?}", static_memory_var_map);
    //let static_memory_var_map = 

    /*for (block_name, commands) in block_map.iter() {
        println!("Block: {}", block_name);
        for command in commands.iter() {
            //println!("{:?}", command.opcode);
            println!("{:?}", command);
        }
    }*/
}

// cmd[8] reg1[8] reg2[8] imm[64]