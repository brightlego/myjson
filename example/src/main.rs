use std::io;
use std::io::Read;
use myjson::{parse, stringify};

fn main() -> (){
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    
    handle.read_to_string(&mut buffer).unwrap();
    let obj = parse(buffer.chars()).unwrap();
    println!("{}", stringify(&obj));
}
