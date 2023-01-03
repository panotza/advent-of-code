const INPUT: [usize; 121] = [
    1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 9, 19, 1, 19, 5, 23, 2, 6, 23, 27, 1, 6,
    27, 31, 2, 31, 9, 35, 1, 35, 6, 39, 1, 10, 39, 43, 2, 9, 43, 47, 1, 5, 47, 51, 2, 51, 6, 55, 1,
    5, 55, 59, 2, 13, 59, 63, 1, 63, 5, 67, 2, 67, 13, 71, 1, 71, 9, 75, 1, 75, 6, 79, 2, 79, 6,
    83, 1, 83, 5, 87, 2, 87, 9, 91, 2, 9, 91, 95, 1, 5, 95, 99, 2, 99, 13, 103, 1, 103, 5, 107, 1,
    2, 107, 111, 1, 111, 5, 0, 99, 2, 14, 0, 0,
];

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
                    let p1 = self.load_value(ip + 1);
                    let p2 = self.load_value(ip + 2);
                    let p3 = self.load_value(ip + 3);

                    let a = self.load_value(p1);
                    let b = self.load_value(p2);

                    self.store_value(p3, a + b);
                }
                MUL => {
                    let p1 = self.load_value(ip + 1);
                    let p2 = self.load_value(ip + 2);
                    let p3 = self.load_value(ip + 3);

                    let a = self.load_value(p1);
                    let b = self.load_value(p2);

                    self.store_value(p3, a * b);
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

    fn load_value(&self, ip: usize) -> usize {
        if let Some(v) = self.ops.get(ip) {
            *v
        } else {
            panic!("ip: {} is out of rage", ip)
        }
    }

    fn store_value(&mut self, ip: usize, v: usize) {
        self.ops[ip] = v
    }
}

fn main() {
    let program = INPUT.to_vec();

    for noun in 0usize..100 {
        for verb in 0usize..100 {
            let mut program = program.clone();
            program[1] = noun;
            program[2] = verb;
            let mut mac = Machine::new(program);
            let r = mac.run();
            if r == 19690720 {
                println!("noun: {}, verb: {}; ans: {}", noun, verb, 100 * noun + verb);
            }
        }
    }
}
