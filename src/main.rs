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

use std::io::Write;

//extern crate libc;

trait ResultExt <T> {
    fn unwr(self) -> T;
}

impl<T> ResultExt<T> for Result<T, String>{
    fn unwr(self) -> T{
        match self {
            Ok(v) => v,
            Err(e) => {
                //std::io::stdout().write(e.to_string().as_bytes());
                println!("{}", e.to_string());
                
                std::process::exit(1);
            }
        }
    }
}

#[no_mangle]
pub fn main(_argc: i32, _argv: *const *const u8) {
//fn main() {
    
    if _argc < 2 {
        //println!("Usage: ncvm_asm <file>");
        std::process::exit(1);
    }
    let filename = unsafe {
        let a = *_argv.offset(1);
        let s = std::ffi::CStr::from_ptr(a as *const _);
        let s = s.to_str().unwrap();
        s
    };
    
    let src = std::fs::read_to_string(filename).unwrap();
    let bytecode = ncvm_asm::compile_asm(src).unwr();
    
    if _argc < 3 {
        std::fs::write("a.ncvm", bytecode).unwrap();
    } else {
        let mut filename = unsafe {
            let a = *_argv.offset(2);
            let s = std::ffi::CStr::from_ptr(a as *const _);
            let s = s.to_str().unwrap();
            s.to_string()
        };
        
        if !filename.ends_with(".ncvm") {
            filename += ".ncvm";
        }
        
        std::fs::write(filename, bytecode).unwrap();
    }
}
