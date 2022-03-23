//! Fast and easy way to ask keyboard user input
//! 
//! Provides an easy way to take user input from the std buffer

// The module :)
pub mod input{
    use std::io::{self, Write};

    // This function takes all buffer line. It return a String object.
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
}