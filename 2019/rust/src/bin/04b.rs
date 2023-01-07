fn main() {
    let mut cnt = 0;

    'outer: for n in 367479..893698 + 1 {
        let mut double = false;
        let size = f64::log10(n as f64).floor() as u32 + 1;
        for i in 0..size - 1 {
            let a = digit_at(n, size, i);
            let b = digit_at(n, size, i + 1);
            let l = {
                if i == 0 {
                    None
                } else {
                    Some(digit_at(n, size, i - 1))
                }
            };
            let r = {
                if size - 2 <= i {
                    None
                } else {
                    Some(digit_at(n, size, i + 2))
                }
            };

            if a > b {
                continue 'outer;
            }

            // basically look for - ab - pattern
            double = double
                || match (l, r) {
                    (None, Some(r)) => a == b && a != r,
                    (Some(l), Some(r)) => l != b && a == b && a != r,
                    (Some(l), None) => l != b && a == b,
                    _ => panic!("unreachable"),
                };
        }

        if double {
            cnt += 1;
        }
    }
    println!("cnt: {}", cnt);
}

fn digit_at(n: u32, size: u32, i: u32) -> u32 {
    let n = n / (10u32.pow((size - 1) - i));
    n % 10
}
