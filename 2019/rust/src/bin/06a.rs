use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/06.txt").unwrap();
    let objects: Vec<(&str, &str)> = input
        .lines()
        .map(|l| l.split(")").collect::<Vec<&str>>())
        .map(|x| (x[0], x[1]))
        .collect();

    let mut orbit: HashMap<&str, Vec<&str>> = HashMap::new();
    for (from, to) in objects {
        if let Some(children) = orbit.get_mut(from) {
            children.push(to);
        } else {
            orbit.insert(from, vec![to]);
        }
    }

    fn walk(orbit: &HashMap<&str, Vec<&str>>, name: &str, lv: usize) -> usize {
        let mut acc: usize = lv;
        if let Some(children) = orbit.get(name) {
            for child in children {
                acc += walk(orbit, child, lv + 1);
            }
        }
        acc
    }
    let sum = walk(&orbit, "COM", 0);
    println!("sum: {}", sum);
}
