use std::{io::{self, ErrorKind, Write}, process::ExitCode};

const fn js_intconv(x : f64) -> f64 {
    ((x as i64) as i32) as f64
}

const TWO_B_32 : f64 = 4294967296.0;
const TWO_B_MINUS32 : f64 = 2.3283064365386963e-10;

struct Mash {
    n : f64
}

impl Mash {
    fn add (&mut self, data : &str) -> f64 {

        for chr in data.chars()  {
            let code = u32::from(chr);
            self.n += code as f64;
            let mut h = 0.02519603282416938 * self.n;
            self.n = js_intconv(h);
            h -= self.n;
            h *= self.n;
            self.n = js_intconv(h);
            h -= self.n;
            self.n += h * TWO_B_32;
        }
        js_intconv(self.n) * TWO_B_MINUS32
    }
    const fn new() -> Mash {
        Mash { n: 4022871197.0 }
    }
 }


struct Alea {
    s0 : f64,
    s1 : f64,
    s2 : f64,
    c : f64
}

impl Alea {
    fn next_f64(self) -> (f64, Alea) {
        let t = 2091639.0 * self.s0 + self.c * TWO_B_MINUS32;
        let s0 = self.s1;
        let s1 = self.s2;
        let c = js_intconv(t) as f64;
        let s2 = t - c;
        (s2, Alea {s0, s1, s2, c})
    }
    fn next_u32(self) -> (u32, Alea) {
        let result = self.next_f64();
        ((result.0 * TWO_B_32) as u32, result.1)
    }
    fn new(strings: &[&str]) -> Alea {
        let mut m = Mash::new();
        let c = 1.0;
        let mut s0 = m.add(" ");
        let mut s1 = m.add(" ");
        let mut s2 = m.add(" ");

        fn mash_adjust ( m: &mut Mash, arg: &str, x: &mut f64) {
            *x -= m.add(arg);
            if *x < 0.0 {*x += 1.0};
        }

        for arg in strings {
            mash_adjust(&mut m, arg, &mut s0);
            mash_adjust(&mut m, arg, &mut s1);
            mash_adjust(&mut m, arg, &mut s2);
        }
        Alea {s0, s1, s2, c}
    }
}

const BUFFER_VALUES : usize = 1024*1024;

fn main() -> ExitCode {
    let mut stdout = io::stdout().lock();
    let strs = ["my", "3", "seeds"];
    let mut cur_alea = Alea::new(&strs);
    loop {
        let mut buf = [0 as u8; BUFFER_VALUES*4];
        for produced_val in 0..BUFFER_VALUES {
            let (value, new_alea) = cur_alea.next_u32();
            let value_bytes = value.to_le_bytes();
            let base = produced_val * 4;
            let destination = &mut buf[base..base+4];
            destination.clone_from_slice(&value_bytes);
            cur_alea = new_alea;
        }
        if let Err(i) = stdout.write_all(&buf) {
            return match i.kind() {
                ErrorKind::BrokenPipe | ErrorKind::Interrupted => {ExitCode::SUCCESS}
                _ => {ExitCode::FAILURE}
            }
        }
    }
}
