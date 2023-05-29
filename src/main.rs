use std::{env, fs::File, io::Read};

fn main() {
    let args : Vec<String> = env::args().collect();
    //cargo run [params, ...]
    println!("{:?}", args);
    let query = &args[1];
    let filename = &args[2];
    

    //filename 이름을 가진 파일을 file 변수에 저장
    let mut file:File = File::open(filename).expect("file not found...");

    //파일 내용을 담을 contents String 변수 생성
    let mut contents = String::new();
    
    //read_to_string 함수를 통해 String 변수에 파일 내용을 담음
    file.read_to_string(&mut contents).expect("Something Err Occured!...");

    println!("file contents : {}", contents);
}
