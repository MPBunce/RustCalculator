use std::env;

fn main() {
    
    let args: Vec<String> = env::args().collect();

    for n in &args{
        println!("{}", n)
    }

    println!("Hello, world!");
}
