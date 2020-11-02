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

    let n = scan.next::<usize>();
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        let age = scan.next::<i32>();
        let name = scan.next::<String>();
        v.push((age, name));
    }
    v.sort_by_key(|x| x.0);

    for x in v {
        writeln!(out, "{} {}", x.0, x.1).ok();
    }
}
