use std::io::{self, Write};

pub fn input_line(msg: &str) -> String {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut input = String::new();

    print!("{}", msg);

    match stdout.flush(){
        Ok(_) => {
            match stdin.read_line(&mut input){
                Ok(_)=>{}
                Err(e) => panic!("Oops! An error ocurrs in read_line::lib.rs::15 : {}", e)
            };
        }
        Err(e) => println!("Oops! An error ocurrs in flush::lib.rs::15 : {}", e)
    }
    

    return input;
}