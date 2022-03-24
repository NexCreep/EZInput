extern crate ezinput-4-keyboard;

fn main(){
    let input: String = ezinput::input::input_line("Caca");

    println!("{}", input);
}