// TODO: Add to_lower function

use std::collections::HashMap;

use crate::opcodes;
use crate::types;
use crate::types::Block;


use types::{
    ArgumentType,
    //Argument,
    Command,
};


enum StaticMemoryType {
    Raw,
    //String
}

impl StaticMemoryType {
    pub fn from_str(s: &str) -> Option<StaticMemoryType> {
        match s {
            "raw" => Some(StaticMemoryType::Raw),
            _ => None
        }
    }

    /*pub fn as_str(&self) -> &'static str {
        match self {
            StaticMemoryType::Raw     => "raw",
            //StaticMemoryType::String  => ""
        }
    }*/

    pub fn default() -> StaticMemoryType {
        return StaticMemoryType::Raw;
    }
}


pub fn parse(
    src: String
) -> Result<(HashMap<String, Vec<u8>>, Vec<Block>), String> {
    let mut static_memory_var_map = HashMap::new();
    //let mut block_map = HashMap::new();
    let mut block_vec: Vec<Block> = Vec::new();

    // Separate asm by new lines and filter empty strings
    let src_lower = src.to_lowercase();
    let lines = src_lower
        .split("\n")
        .filter(|s| s.len() > 0)
        .map(|s| s.trim())
        .collect::<Vec<&str>>();

    let mut is_static_memory_block = false;
    let mut static_memory_block_name = String::new();
    let mut static_memory_type = StaticMemoryType::default();
    let mut block_name = String::new();

    for line in lines {
        let formated_line_split_com = line
            .replace(".", " . ")
            .replace("<", " < ")
            .replace(">", " > ")
            .replace("(", " ( ")
            .replace(")", " ) ")
            //.replace("#", " # ")
            //.replace("+", " + ")
            //.replace("-", " - ")
            .replace(":", " : ")
            .split(";")
            .map(|s| s.to_string())
            .next().unwrap();
            //.collect::<Vec<String>>()[0].clone();

        //let formated_line = formated_line_split_com.clone();

        if formated_line_split_com.chars().count() == 0 {
            continue;
        }

        // Separate line by spaces and filter empty strings
        let words = formated_line_split_com
            .trim()
            .split(" ")
            .filter(|s| s.len() > 0)
            .map(|s| s.trim())
            .collect::<Vec<&str>>();


        // Static memory block definition
        if words.len() >= 2 && words[0] == "." {
            is_static_memory_block = true;
            static_memory_block_name = words[1].to_string();

            if static_memory_var_map.contains_key(&static_memory_block_name) {
                return Err("Static memory block already defined".to_string());
            }

            static_memory_var_map.insert(
                static_memory_block_name.clone(),
                vec![]
            );
            #[cfg(debug_assertions)]
            //println!("Static memory definition found: {}", words[1]);

            if words.len() == 3 && words[2] == "(" {
                static_memory_type = StaticMemoryType::default()
            }
            else if words.len() == 6 && words[2] == "<" && words[4] == ">" && words[5] == "(" {
                //println!("Static memory type: {}", words[3]);
                match StaticMemoryType::from_str(words[3]) {
                    Some(t) => static_memory_type = t,
                    None => return Err("Unknown static memory block type".to_string())
                }
            }
            else {
                return Err("Unexplained line".to_string());
            }
            continue;
        }

        // End of static memory block
        if words.len() == 1 && words[0] == ")" {
            if is_static_memory_block {
                is_static_memory_block = false;
                static_memory_type = StaticMemoryType::default();
                static_memory_block_name.clear();
            }
            else {
                return Err("Unknown static memory block".to_string())
            }
            continue;
        }

        // Static memory block data
        if is_static_memory_block {
            match static_memory_type {
                StaticMemoryType::Raw => {
                    let data_line = words
                        .iter()
                        .map(|s|String::from(*s))
                        .collect::<String>();

                    if data_line.chars().count() % 2 == 1 {
                        return Err("Static memory must be an integer number of bytes".to_string());
                    }

                    let mut cur = data_line.as_str();
                    while !cur.is_empty() {
                        let (chunk, rest) = cur.split_at(2);
                        static_memory_var_map.get_mut(&static_memory_block_name).unwrap().push(u8::from_str_radix(chunk, 16).unwrap());
                        cur = rest;
                    }
                }
            }
            continue;
        }



        // Block definition
        if words.len() == 2 && words[1] == ":" {
            block_name = words[0].to_string();
            //println!("Block name: {}", block_name);

            if block_vec.iter().find(|x| x.name == block_name).is_some() {
                return Err("Block already defined".to_string());
            }

            /*block_map.insert(
                block_name.clone(),
                vec![]
            );*/
            block_vec.push(Block {
                name: block_name.clone(),
                commands: vec![]
            });
            continue;
        }

        let command = opcodes::Opcode::from_str(words[0].to_lowercase().as_str());
        if command.is_some() {
            if block_name.is_empty() {
                return Err("Command found outside of block".to_string());
            }
            //println!("Command found: {:?}",  line);
            let mut cmd = Command::new(command.unwrap());
            for i in 1..words.len() {
                let arg = words[i];
                if arg.starts_with("r") {
                    let index = arg[1..].parse::<u8>().unwrap();
                    cmd.add_arg(ArgumentType::Register(index));
                }
                else if arg.starts_with("#") {
                    match arg.chars().filter(|c| *c == '+').count() {
                        0 => {
                            cmd.add_arg(ArgumentType::Memory(String::from(&arg[1..]), 0));
                        }
                        1 => {
                            let split = arg
                                .split("+")
                                .collect::<Vec<&str>>();
                            let offset = split[1].parse::<u64>().unwrap();
                            cmd.add_arg(ArgumentType::Memory(String::from(&split[0][1..]), offset));
                        }
                        _ => {
                            return Err("Invalid memory address".to_string());
                        }
                    }              
                }
                else if let Ok(a) = arg.parse::<i64>() {
                    cmd.add_arg(ArgumentType::Immediate(a as u64));
                }
                else if let Ok(a) = arg.parse::<u64>() {
                    cmd.add_arg(ArgumentType::Immediate(a));
                }
                else {
                    cmd.add_arg(ArgumentType::Label(String::from(arg)));
                }
            }
            //block_map.get_mut(&block_name).unwrap().push(cmd);
            //block_vec.into_iter().find(|x| x.name == block_name).unwrap().commands.push(cmd);
            block_vec.iter_mut().find(|x| x.name == block_name).unwrap().commands.push(cmd);
            continue;
        }
        else if command.is_none() && !block_name.is_empty() {
            //println!("Command not found: {}", line);
            return Err("Unknown command".to_string());
        }

        return Err("Unknown line".to_string());
    };
    Ok((
        static_memory_var_map,
        block_vec
    ))
}