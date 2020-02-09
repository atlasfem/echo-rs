use std::env;

fn main() {
    let mut args = env::args().skip(1);

    if let Some(arg) = args.next() {
        println!("{}", arg);

        for arg in args {
            println!(" {}", arg);
        }
    }

    println!();
}