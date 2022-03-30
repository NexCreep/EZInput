# EZInput - For user keyboar input ⌨️

## About

EZInput is a library that implements an easy and fast way to take user input from terminal.

## Use

1.- You must add this to your cargo.toml, in **dependencies** section

```
keyezinput = "0.1.3"
````

2.- And finally implements in your code like this:

```rust
use keyezinput;

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

## Change Log
**[v0.1.3 - 22.03]**
- Patched user input that have to extra chars that difficulties the parse to another type




I hope you liked ❤️

Special greeting to you, that watch this repo 😄
