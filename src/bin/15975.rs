use std::io::{stdin, stdout, BufWriter, Write};

#[derive(Default)]
struct Scanner {
    buf: Vec<String>,
}

impl Scanner {
    fn next<T>(&mut self) -> T
    where
        T: std::str::FromStr,
    {
        loop {
            if let Some(token) = self.buf.pop() {
                return token.parse().ok().expect("parse failed");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("read failed");
            self.buf = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = scan.next::<i32>();
    let mut dots = Vec::new();
    dots.push((0, 0));
    dots.push((n + 1, 0));
    for _ in 0..n {
        let x = scan.next::<i32>();
        let color = scan.next::<i32>();
        dots.push((color, x));
    }
    dots.sort();

    let mut sum = 0i64;
    for i in 1..=n as usize {
        let len = match (dots[i].0 == dots[i - 1].0, dots[i].0 == dots[i + 1].0) {
            (true, true) => i32::min(dots[i].1 - dots[i - 1].1, dots[i + 1].1 - dots[i].1),
            (true, false) => dots[i].1 - dots[i - 1].1,
            (false, true) => dots[i + 1].1 - dots[i].1,
            (false, false) => 0,
        };
        sum += len as i64;
    }
    writeln!(out, "{}", sum).ok();
}
