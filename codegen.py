import re

def gen_opcode_impl():
    file_src = open("deps/ncvm/include/ncvm.h", "r").read()
    regex = re.compile(r"enum _packed OPCODE \{[\w\d\,\.\[\]\=\s\*\\\/\>\<\|\-\'\+\*\%\(\)\!]*\}", re.MULTILINE)
    enum = regex.search(file_src).group(0)
    regex = re.compile(r"(\w+),?\s+(\/\*)([\w\d\,\.\[\]\=\ \t\*\\\/\>\<\|\-\'\+\*\%\(\)\!]*)(\*\/)", re.MULTILINE)

    rust_code = '''#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Opcode {\n'''
    
    i = 0
    for match in regex.finditer(enum):
        command = match.group(1)
        doc = match.group(3).split("|")
        r1 = doc[0]
        r2 = doc[1]
        r3 = doc[2]
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

    rust_code += '''impl Opcode {
    pub fn from_str(opcode: &str) -> Option<Opcode> {
        for c in &COMMANDS {
            if c.key == opcode {
                return Some(c.val)
            }
        };
        None
    }
}'''

    open("src/opcodes.rs", "w").write(rust_code)
    



def main():
    gen_opcode_impl()

if __name__ == "__main__":
    main()
