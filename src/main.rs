extern crate keyezinput;

fn main(){
    let input: String = keyezinput::input::input_line("Foo input: ");

    println!("Foo output: {}", input);
}