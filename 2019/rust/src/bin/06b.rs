use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/06.txt").unwrap();
    let objects: Vec<(&str, &str)> = input
        .lines()
        .map(|l| l.split(")").collect::<Vec<&str>>())
        .map(|x| (x[0], x[1]))
        .collect();

    let mut orbit: HashMap<&str, &str> = HashMap::new();
    for (from, to) in objects {
        orbit.insert(to, from);
    }

    fn path_to_com<'a>(orbit: &HashMap<&'a str, &'a str>, from: &str) -> Vec<&'a str> {
        let mut from = orbit.get(from).unwrap();

        let mut paths: Vec<&str> = vec![];
        while let Some(to) = orbit.get(from) {
            paths.push(to);
            from = to;
        }
        paths
    }

    // basically, traverse back to COM (root node) and find the junction then sum the length between 2 paths
    let a = path_to_com(&orbit, "YOU");
    let b = path_to_com(&orbit, "SAN");
    'outer: for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            if x == y {
                println!("min orbital transfers: {}", i + j + 2);
                break 'outer;
            }
        }
    }
}
