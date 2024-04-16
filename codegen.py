import re

def gen_opcode_impl():
    file_src = open("deps/ncvm/include/ncvm.h", "r").read()
    regex = re.compile(r"enum _packed OPCODE \{[\w\d\,\.\[\]\=\s\*\\\/\>\<\|\-\'\+\*\%\(\)\!]*\}", re.MULTILINE)
    enum = regex.search(file_src).group(0)
    regex = re.compile(r"(\w+),?\s+(\/\*)([\w\d\,\.\[\]\=\ \t\*\\\/\>\<\|\-\'\+\*\%\(\)\!]*)(\*\/)", re.MULTILINE)

    rust_code = '''#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Opcode {\n'''
    
    addr_commands = []
    i = 0
    for match in regex.finditer(enum):
        command = match.group(1)
        doc = match.group(3).split("|")
        r1 = doc[0]
        r2 = doc[1]
        r3 = doc[2]
        command_type = doc[3]

        if command_type == "a":
            addr_commands.append({"command":command, "r1":r1, "r2":r2, "r3":r3, "type":command_type})

        rust_code += f'    {command} = {i},\n'
        i += 1
    rust_code += "}\n\n"

    rust_code += f'''struct CommandMapNode {{
    key: &'static str,
    val: Opcode
}}

impl CommandMapNode {{
    pub const fn new(key: &'static str, val: Opcode) -> CommandMapNode {{
        CommandMapNode {{key, val}}
    }}
}}

static COMMANDS: [CommandMapNode; {i}] = [\n'''

    for match in regex.finditer(enum):
        command = match.group(1)
        rust_code += f'    CommandMapNode::new("{command.lower()}", Opcode::{command}),\n'
    
    if rust_code[-1] == ",":
        rust_code = rust_code[:-1]
    rust_code += "];\n\n"

    # Generate address commands array
    rust_code += f"static ADDRESS_COMMANDS: [Opcode; {len(addr_commands)}] = [\n"
    for addr_command in addr_commands:
        rust_code += f"    Opcode::{addr_command['command']},\n"
    if rust_code[-1] == ",":
        rust_code = rust_code[:-1]
    rust_code += "];\n\n"

    rust_code += '''impl Opcode {
    pub fn from_str(opcode: &str) -> Option<Opcode> {
        for c in &COMMANDS {
            if c.key == opcode {
                return Some(c.val)
            }
        };
        None
    }

    pub fn is_address_command(&self) -> bool {
        for c in &ADDRESS_COMMANDS {
            if *c == *self {
                return true
            }
        };
        false
    }
}'''

    open("src/opcodes.rs", "w").write(rust_code)
    



def main():
    gen_opcode_impl()

if __name__ == "__main__":
    main()