use std::env;
extern crate rustyard;

fn main() {
    
    let args: Vec<String> = env::args().collect();

    for n in args.clone(){
        println!("{}", n)
    }
    println!("Calulcating result of: {}", &args[1]);
    let res = parse_string(args[1].to_string());

    println!("Result: {}", res);
}

fn parse_string(input: String) -> f64{
    let mut yard = rustyard::ShuntingYard::new();
    let a:f64 = yard.calculate(&input).unwrap();
    return a
}