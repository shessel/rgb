use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Registers {
    a: u8,
    f: u8,

    b: u8,
    c: u8,

    d: u8,
    e: u8,

    h: u8,
    l: u8,

    sp: u8,
    pc: u8,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let filename = &args[1];
        println!("Hello, {}!", filename);

        let mut f = File::open(filename).expect("File not found");

        let mut buf = Vec::new();
        f.read_to_end(&mut buf).expect("couldn't read file");

        println!("File contents:");
        for (i, x) in buf.iter().enumerate() {
            print!("{:02X} ", x);
            if i > 0 && i % 32 == 0 {
                println!();
                if i == 1024 {
                    break;
                }
            }
        }
    }
}
