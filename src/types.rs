
use crate::opcodes;

#[derive(Clone)]
pub enum ArgumentType {
    Register(u8),
    /*RegisterU32,
    RegisterU64,
    RegisterF32,
    RegisterF64,*/
    Memory(String, u64),
    Immediate(u64),
    Label(String)
}

impl ArgumentType {
    pub fn unwrap_register(&self) -> u8 {
        match self {
            ArgumentType::Register(v) => *v,
            _ => panic!("Expected Register")
        }
    }
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

//#[derive(Debug)]
//#[derive(Clone,)]
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

pub struct Block {
    pub name: String,
    pub commands: Vec<Command>
}


#[derive(Clone, PartialEq)]
pub struct Label{
    pub name: String,
    pub block_size: u64,
    pub full_addr: u64,
    pub used_labels: Vec<String>
}

