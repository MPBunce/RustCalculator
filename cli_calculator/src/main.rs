use std::env;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    println!("Calulcating result of: {}", &args[1]);
    let res = args[1].to_string();
    let mut tokens: Vec<char> = Vec::new();

    for n in res.chars() {
        tokens.push(n);
    }

    for n in tokens {
        println!("{}", n)
    };

    println!("Result: ");
}
