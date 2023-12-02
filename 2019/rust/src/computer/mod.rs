use std::fmt::Display;
use std::io::Read;

use num_traits::{PrimInt, ToPrimitive};

const ADD: u32 = 1;
const MUL: u32 = 2;
const INPUT: u32 = 3;
const OUTPUT: u32 = 4;
const JIFT: u32 = 5;
const JIFF: u32 = 6;
const LT: u32 = 7;
const EQ: u32 = 8;
const HALT: u32 = 99;

const POS: u8 = 0;
const IMMED: u8 = 1;

pub struct Computer<T, I: Read> {
    input: I,
    ops: Vec<T>,
    ip: usize,
    modified_ip: bool,
}

impl<T, I: Read> Computer<T, I>
where
    T: PrimInt + Display + ToPrimitive,
{
    pub fn new(ops: Vec<T>, input: I) -> Self {
        Computer {
            ops,
            ip: 0,
            modified_ip: false,
            input,
        }
    }

    fn decode_opcode(&self, v: usize) -> OpCode {
        OpCode {
            op: (v % 100) as u32,
            p1: ((v / 100) % 10) as u8,
            p2: ((v / 1000) % 10) as u8,
            p3: ((v / 1000) % 10) as u8,
        }
    }

    fn inc_ip(&mut self) {
        if self.modified_ip {
            self.modified_ip = false;
        } else {
            self.ip += 1;
        }
    }

    fn set_ip(&mut self, n: usize) {
        self.ip = n;
        self.modified_ip = true;
    }

    pub fn run(&mut self) -> T {
        while self.ip < self.ops.len() {
            let inst = self.decode_opcode(self.ops[self.ip].to_usize().unwrap());
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
                    let mut buf = String::new();
                    self.input.read_to_string(&mut buf).unwrap();
                    let i: i64 = buf.parse().unwrap();
                    self.store(p1.to_usize().unwrap(), T::from(i).unwrap());
                }
                OUTPUT => {
                    let p1 = self.load_next(POS);
                    println!("{}", p1);
                }
                JIFT => {
                    let p1 = self.load_next(inst.p1);
                    let p2 = self.load_next(inst.p2);
                    if !p1.is_zero() {
                        self.set_ip(p2.to_usize().unwrap());
                    }
                }
                JIFF => {
                    let p1 = self.load_next(inst.p1);
                    let p2 = self.load_next(inst.p2);
                    if p1.is_zero() {
                        self.set_ip(p2.to_usize().unwrap());
                    }
                }
                LT => {
                    let p1 = self.load_next(inst.p1);
                    let p2 = self.load_next(inst.p2);
                    let p3 = self.load_next(IMMED);
                    if p1.lt(&p2) {
                        self.store(p3.to_usize().unwrap(), T::from(1).unwrap());
                    } else {
                        self.store(p3.to_usize().unwrap(), T::from(0).unwrap());
                    }
                }
                EQ => {
                    let p1 = self.load_next(inst.p1);
                    let p2 = self.load_next(inst.p2);
                    let p3 = self.load_next(IMMED);
                    if p1.eq(&p2) {
                        self.store(p3.to_usize().unwrap(), T::from(1).unwrap());
                    } else {
                        self.store(p3.to_usize().unwrap(), T::from(0).unwrap());
                    }
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

struct OpCode {
    op: u32,
    p1: u8,
    p2: u8,
    p3: u8,
}
