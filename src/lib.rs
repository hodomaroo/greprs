use core::panic;
use std::{fs::File, error::Error, io::ErrorKind};


//특정 기능을 담당하는 로직은 lib.rs에 선언

//파일 이름을 입력받아 해당 파일 반환
pub fn openFile(fileName : &String) -> File{
    let file = File::open(fileName);
    
    match  file {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            panic!("File Not Found!");
        },
        Err(ref error) if error.kind() == ErrorKind::OutOfMemory => {
            panic!("Params Not efficient!");
        }
        Err(ref error) => {
            panic!("{}", error.to_string());
        }
    }
}