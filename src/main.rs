/*trait ResultExt <T> {
    fn unwr(self) -> T;
}

impl<T, E> ResultExt<T> for Result<T, E>
    where E: std::fmt::Debug
{
    fn unwr(self) -> T{
        match self {
            Ok(v) => v,
            Err(e) => {
                println!("Error: {:?}", e);
                std::process::exit(1);
            }
        }
    }
}*/

//use std::io::Write;

/*trait ResultExt <T> {
    fn unwr(self) -> T;
}

impl<T> ResultExt<T> for Result<T, String>{
    fn unwr(self) -> T{
        match self {
            Ok(v) => v,
            Err(e) => {
                std::io::stdout().write(e.to_string().as_bytes());
                std::process::exit(1);
            }
        }
    }
} */


//fn main() {
    /*let src = match std::fs::read_to_string("examples/12.asm") {
        Ok(v) => v,
        Err(e) => {
            
            /*match e.kind() {
                std::io::ErrorKind::NotFound => {
                    std::io::stdout().write("File not found".as_bytes());
                },
                _ => {
                    std::io::stdout().write("Error".as_bytes());
                }
            }*/
            println!("Error: {:?}", e);

            std::io::stdout().write("Hello".as_bytes());
            std::process::exit(1);
        }
    };
    std::io::stdout().write(src.as_bytes());*/
    /*let src = std::fs::read_to_string("examples/1.asm").unwrap();
    ncvm_asm::compile_asm(src).unwr();*/    
    //ncvm_asm::compile_asm(String::from("value"));
//}

#![no_main]

//use std::fs::File;
//use std::io::Write;
//use std::os::unix::io::FromRawFd;

extern crate libc;

#[no_mangle]
pub fn main(_argc: i32, _argv: *const *const u8) {
    //unsafe {
        let src = std::fs::read_to_string("examples/1.asm").unwrap();
        ncvm_asm::compile_asm(src).unwrap();
        //let s = ncvm_asm::compile_asm(String::from("")).unwrap();
        //libc::printf(a.as_ptr() as *const _);
    //}
    /*let mut stdout = stdout();
    stdout.write(b"Hello, world!\n").unwrap();*/
    //std::io::stdout().write("Hello\n".as_bytes());
}