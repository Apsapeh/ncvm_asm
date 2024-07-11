
use std::collections::HashMap;

use crate::types::{ArgumentType, Block, Command, Label};
use crate::opcodes::Opcode;

pub fn precompile(
    sm_map: HashMap<String, Vec<u8>>,
    block_map: Vec<Block>,
) -> (Vec<Vec<u8>>, Vec<u8>, Vec<Command>) {
    let mut lib_functions = vec![];

    let mut static_memory_var_map = HashMap::with_capacity(sm_map.len());
    let mut compiled_static_memory: Vec<u8> = Vec::with_capacity(
        (&sm_map).into_iter().map(|(_, v)| v.len()).sum()
    );
    
    for (k, v) in sm_map.iter() {
        static_memory_var_map.insert(k, compiled_static_memory.len() as u64);
        compiled_static_memory.extend(v);
    }

    let mut labels: Vec<Label> = Vec::new();
    let mut compiled_blocks_map = HashMap::with_capacity(block_map.len());
    let mut compiled_blocks: Vec<Command> = Vec::with_capacity(
        (&block_map).into_iter().map(|v| v.commands.len()).sum()
    );
    for block in block_map.iter() {
        let mut used_labels = vec![];
        let commands = &block.commands;
        compiled_blocks_map.insert(block.name.clone(), compiled_blocks.len() as u64);
        let start_block_addr = compiled_blocks.len() as u64;
        for command in commands.iter() {
            let mut skip_push = false;
            let mut new_command = Command::new(command.opcode);
            for (i, arg) in command.args.iter().enumerate() {
                match arg {
                    ArgumentType::Register(v) => {
                        new_command.add_arg(ArgumentType::Register(v.clone()));
                    },
                    ArgumentType::Memory(l, o) => {
                        match command.opcode {
                            Opcode::LSR => {
                                let mut clear_addr_reg = Command::new(Opcode::LRCLR);
                                clear_addr_reg.add_arg(new_command.args[0].clone());
                                compiled_blocks.push(clear_addr_reg);

                                let mem_addr = static_memory_var_map.get(l).unwrap().clone() as u64 + o;
                                if mem_addr > 0 {
                                    split_on_bytes(
                                        mem_addr, Opcode::LSR, new_command.args[0].unwrap_register(),
                                        Opcode::LLSI, &mut compiled_blocks, 4
                                    );
                                }
                                skip_push = true;
                            }
                            _ => {
                                let mut clear_addr_reg = Command::new(Opcode::LRCLR);
                                clear_addr_reg.add_arg(ArgumentType::Register(0));
                                compiled_blocks.push(clear_addr_reg);

                                let mem_addr = static_memory_var_map.get(l).unwrap().clone() as u64 + o;
                                if mem_addr > 0 {
                                    split_on_bytes(
                                        mem_addr, Opcode::LSR, 0,
                                        Opcode::LLSI, &mut compiled_blocks, 4
                                    );
                                }
                            }
                        }

                        
                    },
                    ArgumentType::Immediate(v) => {
                        if *v <= 0xFFFFFF && command.opcode == Opcode::POPI {
                            new_command.add_arg(ArgumentType::Register(v.clone() as u8));
                            new_command.add_arg(ArgumentType::Register((v >> 8) as u8));
                            new_command.add_arg(ArgumentType::Register((v >> 16) as u8));
                        }
                        else if command.opcode == Opcode::ISR {
                            split_on_bytes(
                                v.clone(), Opcode::ISR, command.args[0].unwrap_register(),
                                Opcode::ILSI, &mut compiled_blocks, 2
                            );
                            skip_push = true;
                        }
                        else if command.opcode == Opcode::LSR {
                            split_on_bytes(
                                v.clone(), Opcode::LSR, command.args[0].unwrap_register(),
                                Opcode::LLSI, &mut compiled_blocks, 4
                            );
                            skip_push = true;
                        }
                        else if *v <= 0xFF {
                            new_command.add_arg(ArgumentType::Register(v.clone() as u8));
                        }
                        else {
                            //return Err("Invalid immediate value".to_string());
                        }
                    },
                    ArgumentType::Label(l) => {
                        let mut clear_addr_reg = Command::new(Opcode::LRCLR);
                        clear_addr_reg.add_arg(ArgumentType::Register(0));
                        compiled_blocks.push(clear_addr_reg);
                        
                        match command.opcode {
                            Opcode::LIBCALL => {
                                let addr = match lib_functions.iter().position(|x| x == l) {
                                    Some(i) => i,
                                    None => {
                                        lib_functions.push(l.clone());
                                        lib_functions.len() - 1
                                    }
                                };

                                split_on_bytes(
                                    addr as u64,
                                    Opcode::LSR,
                                    0,
                                    Opcode::LLSI,
                                    &mut compiled_blocks,
                                    4
                                );
                            },
                            _ => {
                                new_command.add_arg(arg.clone());
                                used_labels.push(l.clone());
                            }
                        }
                    }
                }
            }
            
            if !skip_push {
                compiled_blocks.push(new_command);
            }
        }

        let end_block_addr = compiled_blocks.len() as u64;
        labels.push(Label {
            name: block.name.clone(),
            block_size: end_block_addr - start_block_addr,
            full_addr: 0,
            used_labels: used_labels
        });
    }

    // Calculate label address
    while {
        let labels_clone = labels.clone();
        for (idx, label) in labels.iter_mut().enumerate() {
            if idx > 0 {
                let mut prev_labels_size: u64 = 0;
                for used_label in labels_clone[idx-1].used_labels.iter() {
                    let addr = labels_clone.iter().find(|x| x.name == *used_label).unwrap().full_addr;
                    prev_labels_size += get_label_size(addr);
                }
                label.full_addr = labels_clone[idx-1].full_addr + labels_clone[idx-1].block_size + prev_labels_size;
            }
            else {
                label.full_addr = 0;
            }
        }
        labels_clone != labels
    } {}

    let mut final_commands = vec![];
    for cmd in compiled_blocks.iter_mut() {
        let mut new_command = Command::new(cmd.opcode);
        for arg in cmd.args.iter() {
            match arg {
                ArgumentType::Label(l) => {
                    let addr = labels.iter().find(|x| x.name == *l).unwrap().full_addr;
                    split_on_bytes(addr, Opcode::LSR, 0, Opcode::LLSI, &mut final_commands, 4);
                },
                _ => {
                    new_command.add_arg(arg.clone());
                }
            }
        }
        final_commands.push(new_command);
    }

    let compiled_lib_functions = lib_functions.into_iter()
        .map(|x| {
            let mut v = x.into_bytes();
            v.push(0);
            v
        })
        .collect::<Vec<Vec<u8>>>();

    // Print final commands
    for (i, cmd) in final_commands.iter().enumerate() {
        println!("{i}: {:?}", cmd);
    }

    (compiled_lib_functions, compiled_static_memory, final_commands)
}



fn split_on_bytes(
    mut value: u64,
    sr_opcode: Opcode, 
    sr_number: u8,
    shift_opcode: Opcode, 
    compiled_blocks: &mut Vec<Command>, 
    count: u8
) {
    let mut tmp_vec = vec![];
    let mut i = 0;
    while {
        let mut load_command = Command::new(sr_opcode);
        load_command.add_arg(ArgumentType::Register(sr_number));
        load_command.add_arg(ArgumentType::Register(value as u8));
        value >>= 8;
        load_command.add_arg(ArgumentType::Register(value as u8));
        value >>= 8;
        tmp_vec.push(load_command);
        value > 0 && i < count - 1
    } {i += 1}
    let mut i = tmp_vec.len()-1;
    for c in tmp_vec.into_iter().rev() {
        compiled_blocks.push(c);
        if i != 0 {
            let mut load_command = Command::new(shift_opcode);
            load_command.add_arg(ArgumentType::Register(sr_number));
            load_command.add_arg(ArgumentType::Register(sr_number));
            load_command.add_arg(ArgumentType::Register(16));
            compiled_blocks.push(load_command);
            i -= 1;
        }
    }
}

fn get_label_size(addr: u64) -> u64 {
    if addr <= 0xFF {
        1
    }
    else if addr <= 0xFFFF {
        3
    }
    else if addr <= 0xFFFFFF {
        5
    }
    else {
        7
    }
}