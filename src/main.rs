use std::{env, fs::File, io::Read, error::Error, process};
mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error : {}", err);
        process::exit(1);
    });

    //filename 이름을 가진 파일을 file 변수에 저장
    let mut file:File = lib::openFile(&config.filename);

    //파일 내용을 담을 contents String 변수 생성
    let mut contents = String::new();
    
    //read_to_string 함수를 통해 String 변수에 파일 내용을 담음
    file.read_to_string(&mut contents).expect("Something Err Occured!...");

    println!("file contents : {}", contents);
}


struct  Config{
    query : String,
    filename : String
}

impl Config{
    fn new(args : &[String]) -> Result<Config, &'static str>{
        if args.len() < 3{
            return Err("Not sufficent Parameter");
        }
        Ok(Config { query: args[1].clone(), filename: args[2].clone()})
    }
}
