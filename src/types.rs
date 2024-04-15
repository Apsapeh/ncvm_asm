
use crate::opcodes;

#[derive(Debug)]
pub enum ArgumentType {
    Register(u8),
    /*RegisterU32,
    RegisterU64,
    RegisterF32,
    RegisterF64,*/
    Memory(String, u64),
    Immediate(i64),
}


//#[derive(Debug)]
/*pub struct Argument {
    arg_type: ArgumentType,
    value: String
}

impl Argument {
    pub fn new(arg_type: ArgumentType, value: String) -> Argument {
        Argument {
            arg_type,
            value
        }
    }
}*/

#[derive(Debug)]
pub struct Command {
    pub opcode: opcodes::Opcode,
    pub args : Vec<ArgumentType>
}

impl Command {
    pub fn new(opcode: opcodes::Opcode) -> Command {
        Command {
            opcode,
            args: vec![]
        }
    }

    pub fn add_arg(&mut self, arg: ArgumentType) {
        self.args.push(arg);
    }
}