use crate::types::{ArgumentType, Command, Label};

pub fn compile(
    lib_functions: Vec<Vec<u8>>,
    mut compiled_static_mem: Vec<u8>,
    compiled_commands: Vec<Command>,
    public_labels: Vec<Label>,
) -> Vec<u8> {
    let mut result = Vec::with_capacity(
        compiled_static_mem.len() + compiled_commands.len() * 4
    );

    // Version
    let version = 1u32;
    result.extend_from_slice(&version.to_le_bytes());

    // Setings
    result.push(8u8); // u32 reg
    result.push(8u8); // u64 reg
    result.push(8u8); // f32 reg
    result.push(8u8); // f64 reg
    let stack_size: u64 = 1024 * 1024 * 1; // 1MB
    result.extend_from_slice(&stack_size.to_le_bytes()); // stack size
    let call_stack_size: u64 = 1024 * 128; // 512KB (x32) / 1MB (x64)   
    result.extend_from_slice(&call_stack_size.to_le_bytes()); // call stack size

    // Blocks info
    let public_labels_count = public_labels.len() as u64;
    result.extend_from_slice(&public_labels_count.to_le_bytes()); // public labels count
    let lib_functions_count = lib_functions.len() as u64;
    result.extend_from_slice(&lib_functions_count.to_le_bytes()); // lib functions count
    let lib_functions_size = lib_functions.iter().map(|v| v.len()).sum::<usize>() as u64;
    result.extend_from_slice(&lib_functions_size.to_le_bytes()); // lib functions size
    let static_mem_size = compiled_static_mem.len() as u64;
    result.extend_from_slice(&static_mem_size.to_le_bytes()); // static memory index
    let block_size = compiled_commands.len() as u64;
    result.extend_from_slice(&block_size.to_le_bytes()); // block index
    
    // Public labels
    for label in public_labels {
        let mut byte_name = label.name.into_bytes();
        /*if byte_name[byte_name.len() - 1] != 0 {
            byte_name.push(0);
        }*/
        byte_name.push(0);
        if byte_name.len() > 255 {
            panic!("Label name is too long");
        }
        result.push(byte_name.len() as u8);
        result.extend_from_slice(&byte_name);
        result.extend_from_slice(&label.full_addr.to_le_bytes());
    }

    // Lib functions
    for lib_function in lib_functions {
        result.extend_from_slice(&lib_function);
    }

    // Static memory
   result.append(&mut compiled_static_mem);

    // Blocks
    for command in compiled_commands {
        if command.args.len() > 3 {
            panic!("Too many arguments for command");
        }
        
        let mut byte_command: [u8; 4] = [0; 4];

        // Set command
        byte_command[0] = (command.opcode as u8).to_le();

        // Set arguments
        for i in 0..3 {
            if command.args.len() > i {
                if let ArgumentType::Register(v) = &command.args[i] {
                    byte_command[i + 1] = v.to_le();
                } else {
                    panic!("Invalid argument type");
                }
            }
        }

        result.extend_from_slice(&byte_command);
    }

    //println!("{:?}", result);
    //println!("{:?}", result.len() as u64);

    result
}