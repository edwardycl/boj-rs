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
    let n = scan.next::<i64>();
    let mut prime_factors = Vec::new();

    if n == 1 {
        return;
    }
    solve(n, &mut prime_factors);
    prime_factors.sort();
    for p in prime_factors {
        writeln!(out, "{}", p).ok();
    }
}

fn solve(n: i64, pf: &mut Vec<i64>) {
    // 1. Miller-Rabin
    if is_prime(n) {
        pf.push(n);
        return;
    }
    // 2. Pollard's rho
    if let Some(f) = pollard_rho(n) {
        solve(f, pf);
        solve(n / f, pf);
        return;
    }
    // 3. Last check
    if n < 10_000_000_000_000_000 {
        let mut n = n;
        while n % 2 == 0 {
            pf.push(2);
            n /= 2;
        }
        let mut p = 3;
        while p * p <= n {
            while n % p == 0 {
                pf.push(p);
                n /= p;
            }
            p += 2;
        }
        if n != 1 {
            pf.push(n);
        }
    } else {
        pf.push(n);
    }
}

fn pollard_rho(n: i64) -> Option<i64> {
    let mut x = 2;
    let mut y = 2;
    let mut d = 1;
    for i in 0..1_000_000 {
        x = (mul_mod(x, x, n) + 1) % n;
        y = (mul_mod(y, y, n) + 1) % n;
        y = (mul_mod(y, y, n) + 1) % n;
        d = gcd((x - y).abs(), n);
        if d != 1 {
            break;
        }
    }
    if d == n || d == 1 {
        None
    } else {
        Some(d)
    }
}

// Miller-Rabin primality test
fn is_prime(n: i64) -> bool {
    // Can't use external crate `rand`
    const RAND_NUMS: [i64; 40] = [
        670164287199246182,
        9214203182926399135,
        8238735229412258825,
        8783036834712653088,
        3632401852918111531,
        4653790133352716947,
        301288656486692421,
        1053897433974994545,
        1088626352980885378,
        5124540380291114005,
        6441826821314206526,
        7737725465542501759,
        93941849963833195,
        2902337700218540281,
        6446696805595903953,
        511437195186410309,
        8746580920441952294,
        4194696683413847307,
        6793621620346521673,
        8871787357814061464,
        3362055597079691981,
        6137650417480879178,
        3858520158788760169,
        5454717743025623992,
        2068666025231616072,
        8704873964487880200,
        6539302550254140943,
        7209711146047234478,
        6062077406642949848,
        5797289710958127013,
        8442753075163742250,
        864943653867854397,
        7402118734563795546,
        1171957278509710719,
        5046042612462883978,
        1392197784164700683,
        6887119869800048351,
        6615193218694171362,
        1969769536485897683,
        5331986024940722592,
    ];
    match n {
        1 => false,
        2 | 3 => true,
        n if n % 2 == 0 => false,
        _ => {
            let mut r = 0;
            let mut d = n - 1;
            while d & 1 == 0 {
                r += 1;
                d /= 2;
            }
            'witness: for random_num in RAND_NUMS.iter() {
                let a = random_num % (n - 3) + 2;
                let mut x = pow_mod(a, d, n);
                if x == 1 || x == n - 1 {
                    continue 'witness;
                }
                for _ in 0..(r - 1) {
                    x = mul_mod(x, x, n);
                    if x == n - 1 {
                        continue 'witness;
                    }
                }
                return false;
            }
            true
        }
    }
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let rem = a % b;
        a = b;
        b = rem;
    }
    a
}

// a * b (mod m)
fn mul_mod(a: i64, b: i64, m: i64) -> i64 {
    ((a as i128 * b as i128) % m as i128) as i64
}

// a ^ b (mod m)
fn pow_mod(mut a: i64, mut b: i64, m: i64) -> i64 {
    let mut result = 1;
    a = a % m;
    while b > 0 {
        if b & 1 != 0 {
            result = mul_mod(result, a, m);
        }
        a = mul_mod(a, a, m);
        b >>= 1;
    }
    result
}
