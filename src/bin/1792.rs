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

const M: usize = 50000;

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    // least prime factor
    let mut lpf = [0; M + 10];
    let mut mobius = [0; M + 10];
    let mut mobius_sum = [0; M + 10];

    for i in 2..=M {
        if lpf[i] == 0 {
            let mut j = i;
            while j <= M {
                if lpf[j] == 0 {
                    lpf[j] = i;
                }
                j += i;
            }
        }
    }

    mobius[1] = 1;
    mobius_sum[1] = 1;
    for i in 2..=M {
        if lpf[i / lpf[i]] == lpf[i] {
            mobius[i] = 0;
        } else {
            mobius[i] = -1 * mobius[i / lpf[i]];
        }
        mobius_sum[i] = mobius_sum[i - 1] + mobius[i];
    }

    let n: i32 = scan.next();
    for _ in 0..n {
        let mut a: i32 = scan.next();
        let mut b: i32 = scan.next();
        let d: i32 = scan.next();
        a /= d;
        b /= d;

        let mut result = 0;
        let mut pos = a.min(b);
        while pos > 0 {
            let ak = a / pos;
            let bk = b / pos;
            let next_pos = i32::max(a / (ak + 1), b / (bk + 1));
            result += (mobius_sum[pos as usize] - mobius_sum[next_pos as usize]) * ak * bk;
            pos = next_pos;
        }

        writeln!(out, "{}", result).ok();
    }
}
