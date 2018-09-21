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

    sp: u16,
    pc: u16,
}

impl Registers {
    fn new() -> Registers {
        Registers {
            a: 0,
            f: 0,

            b: 0,
            c: 0,

            d: 0,
            e: 0,

            h: 0,
            l: 0,

            sp: 0,
            pc: 0,
        }
    }
    
    fn set_zero_flag(&mut self, value: bool) {
        if value {
            self.f |= 1 << 7;
        } else {
            self.f &= 0 << 7;
        }
    }
}

fn run(program: &Vec<u8>) {
    let mut registers = Registers::new();    
    let mut memory: [u8; 0x10000] = [0; 0x10000];
    for (i, byte) in memory.iter_mut().enumerate() {
        *byte = (i as u8);
    }

    loop {
        if (registers.pc as usize) < program.len() {
            let inst = program[registers.pc as usize];
            registers.pc += 1;
            match inst {
                // LD HL,d16
                0x21u8 => {
                    registers.l = program[registers.pc as usize];
                    registers.h = program[registers.pc as usize + 1];
                    registers.pc += 2;
                    println!("new value of H: {:02X} L: {:02X}", registers.h, registers.l);
                },
                // LD SP,d16
                0x31u8 => {
                    let word: u16 = (program[registers.pc as usize] as u16) | (program[registers.pc as usize + 1] as u16) << 8;
                    registers.pc += 2;
                    registers.sp = word;
                    println!("new value of SP: {:04X} ", registers.sp);
                },
                // LDD (HL),A
                0x32u8 => {
                    let mut address = (registers.h as u16) << 8 | (registers.l as u16);
                    registers.a = memory[address as usize];
                    address -= 1;
                    registers.h = (address >> 8) as u8;
                    registers.l = address as u8;
                    println!("new value of H: {:02X} L: {:02X} A: {:02X}", registers.h, registers.l, registers.a);
                },

                // XOR A
                0xAFu8 => {
                    registers.a ^= registers.a;
                    let zero_flag = registers.a == 0;
                    registers.set_zero_flag(zero_flag);
                    println!("new value of A: {:02X} F: {:08b}", registers.a, registers.f);
                },
                _ => break,
            }
        }
    }
    println!("teh end");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let filename = &args[1];
        println!("Hello, {}!", filename);

        let mut f = File::open(filename).expect("File not found");

        let mut buf = Vec::new();
        f.read_to_end(&mut buf).expect("couldn't read file");

        run(&buf);

        if args.len() > 2 {
            if args[2] == "--print" {
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
    }
}
