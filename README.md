# EZInput - For user keyboar input ⌨️

## About

EZInput is a library that implements an easy and fast way to take user input from terminal.

## Use

1.- You must add this to your cargo.toml, in **dependencies** section

```
keyezinput = "0.1.2"
````

2.- And finally implements in your code like this:

```rust
extern crate keyezinput;

fn main(){
    let input: String = keyezinput::input::input_line("Foo input: ");

    println!("Foo output: {}", input);
}
```

The output would be like this:

````
Foo input: bar keyboard
Foo output: bar keyboard
````




I hope you liked ❤️

Special greeting to you, that watch this repo 😄
