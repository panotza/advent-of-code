use std::fs;

const ADD: usize = 1;
const MUL: usize = 2;
const HALT: usize = 99;

struct Machine {
    ops: Vec<usize>,
}

impl Machine {
    fn new(ops: Vec<usize>) -> Self {
        Machine { ops }
    }

    fn run(&mut self) -> usize {
        let mut ip: usize = 0;
        while ip < self.ops.len() {
            let op = self.ops[ip];
            match op {
                ADD => {
                    let p1 = self.load(ip + 1);
                    let p2 = self.load(ip + 2);
                    let p3 = self.load(ip + 3);

                    let a = self.load(p1);
                    let b = self.load(p2);

                    self.store(p3, a + b);
                }
                MUL => {
                    let p1 = self.load(ip + 1);
                    let p2 = self.load(ip + 2);
                    let p3 = self.load(ip + 3);

                    let a = self.load(p1);
                    let b = self.load(p2);

                    self.store(p3, a * b);
                }
                HALT => break,
                _ => {
                    panic!("unreachable")
                }
            }

            ip += 4;
        }
        self.ops[0]
    }

    fn load(&self, ip: usize) -> usize {
        if let Some(v) = self.ops.get(ip) {
            *v
        } else {
            panic!("ip: {} is out of rage", ip)
        }
    }

    fn store(&mut self, ip: usize, v: usize) {
        self.ops[ip] = v
    }
}

fn main() {
    let mut program: Vec<usize> = fs::read_to_string("input/02.txt")
        .unwrap()
        .split_terminator(",")
        .map(|x| x.parse().unwrap())
        .collect();
    program[1] = 12;
    program[2] = 2;
    let mut mac = Machine::new(program);
    let r = mac.run();
    println!("result: {}", r);
}
