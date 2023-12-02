use std::fs;
use std::io::BufReader;

use advent_of_code2019::Computer;

fn main() {
    let program: Vec<i64> = fs::read_to_string("input/05.txt")
        .unwrap()
        .split_terminator(",")
        .map(|x| x.parse().unwrap())
        .collect();

    BufReader::new(String::from("abc"));

    let mut mac = Computer::new(program);
    let _ = mac.run();
}
