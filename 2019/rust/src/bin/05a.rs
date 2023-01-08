use std::fmt::Display;
use std::fs;

use num_traits::{PrimInt, ToPrimitive};

const ADD: u32 = 1;
const MUL: u32 = 2;
const INPUT: u32 = 3;
const OUTPUT: u32 = 4;
const HALT: u32 = 99;

const POS: u8 = 0;
const IMMED: u8 = 1;

struct Machine<T> {
    ops: Vec<T>,
    ip: usize,
}

impl<T> Machine<T>
where
    T: PrimInt + Display + ToPrimitive,
{
    fn new(ops: Vec<T>) -> Self {
        Machine { ops, ip: 0 }
    }

    fn get_opcode(&self, v: usize) -> Instruction {
        Instruction {
            op: (v % 100) as u32,
            p1: ((v / 100) % 10) as u8,
            p2: ((v / 1000) % 10) as u8,
            p3: ((v / 1000) % 10) as u8,
        }
    }

    fn inc_ip(&mut self) {
        self.ip += 1;
    }

    fn run(&mut self, input: T) -> T {
        while self.ip < self.ops.len() {
            let inst = self.get_opcode(self.ops[self.ip].to_usize().unwrap());
            match inst.op {
                ADD => {
                    let p1 = self.load_next(inst.p1);
                    let p2 = self.load_next(inst.p2);
                    let p3 = self.load_next(IMMED);
                    self.store(p3.to_usize().unwrap(), p1 + p2);
                }
                MUL => {
                    let p1 = self.load_next(inst.p1);
                    let p2 = self.load_next(inst.p2);
                    let p3 = self.load_next(IMMED);
                    self.store(p3.to_usize().unwrap(), p1 * p2);
                }
                INPUT => {
                    let p1 = self.load_next(IMMED);
                    self.store(p1.to_usize().unwrap(), input);
                }
                OUTPUT => {
                    let p1 = self.load_next(POS);
                    println!("{}", p1);
                }
                HALT => break,
                _ => {
                    panic!("unreachable")
                }
            }

            self.inc_ip();
        }
        self.ops[0]
    }

    fn load_next(&mut self, mode: u8) -> T {
        self.inc_ip();
        self.load(self.ip, mode)
    }

    fn load(&self, ip: usize, mode: u8) -> T {
        match mode {
            0 => {
                if let Some(v) = self.ops.get(ip) {
                    self.load((*v).to_usize().unwrap(), 1)
                } else {
                    panic!("ip: {} is out of rage", ip)
                }
            }
            1 => {
                if let Some(v) = self.ops.get(ip) {
                    *v
                } else {
                    panic!("ip: {} is out of rage", ip)
                }
            }
            _ => {
                panic!("unreachable")
            }
        }
    }

    fn store(&mut self, ip: usize, v: T) {
        self.ops[ip] = v
    }
}

struct Instruction {
    op: u32,
    p1: u8,
    p2: u8,
    p3: u8,
}

fn main() {
    let program: Vec<i64> = fs::read_to_string("input/05.txt")
        .unwrap()
        .split_terminator(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut mac = Machine::new(program);
    let _ = mac.run(1);
}
