import re

def gen_opcode_impl():
    regex = re.compile(r"pub enum Opcode \{[\w\d\,=\s]*\}", re.MULTILINE)
    file_src = open("deps/ncvm-rust/src/clib_ncvm.rs", "r").read()
    enum = regex.search(file_src).group(0)
    regex_match = enum.replace(",", "").split("\n")[1:-1]
    regex_match = list(map(lambda x: x.strip().split(" ")[0], regex_match))
    
    impl = "impl Opcode {\n\tpub fn from_str(opcode: &str) -> Option<Opcode> {\n\t\tmatch opcode {\n"

    for match in regex_match:
        impl += f"\t\t\t\"{match.lower()}\" => Some(Opcode::{match}),\n"

    impl += "\t\t\t_ => None"
    impl += "\n\t\t}\n\t}\n}"
    enum = "#[derive(Debug)]\n" + enum
    enum += "\n\n" + impl
    open("src/opcodes.rs", "w").write(enum)




def main():
    gen_opcode_impl()

if __name__ == "__main__":
    main()
