use crate::error::Error;
use crate::instruction::Instruction;
use crate::instruction::Instruction::Dec;

pub struct Translator {
    pub output: String
}

impl Translator {
    pub fn new(output: String) -> Self {
        Self {
            output
        }
    }

    /*pub fn run_instructions(&self) -> Vec<Instruction> {
        self.run_string()
            .into_bytes()
            .into_iter()
            .map(|c| Instruction::from(c as char))
            .collect()
    }

    pub fn run_string(&self) -> String {
        let mut buf = String::new();

        for (idx, c) in self.output.chars().enumerate() {
            if idx == 0 {
                buf.push_str(&Self::r#char(c));
            } else {
                let delta = (c as i32) - (self.output.as_bytes()[idx - 1] as i32);
                let to_push = {
                    let mut clone = buf.clone();
                    clone.push_str(&Self::delta(delta as f32));
                    clone
                };
                buf.push_str(&to_push)
            }
        }

        buf
    }

    fn delta(delta: f32) -> String {
        let mut buf = String::new();

        for _ in 0..Self::floor(delta.abs()) {
            buf.push('+');
        }

        if delta > 0.0 {
            buf.push_str("[>++++++++++<-]>");
        } else {
            buf.push_str("[>----------<-]>");
        }

        for _ in 0..(delta.abs() % 10.0) as usize {
            if delta > 0.0 {
                buf.push('+');
            } else {
                buf.push('-');
            }
        }

        buf.push_str(".<");

        buf
    }

    fn r#char(c: char) -> String {
        let mut buf = String::from("[-]>[-]<");

        for _ in 0..Self::floor((c as u8 as f32) / 10.0) {
            buf.push('+');
        }

        buf.push_str("[>++++++++++<-]>");

        for _ in 0..(c as usize) % 10 {
            buf.push('+');
        }

        buf.push_str(".<");

        buf
    }

    fn floor(item: f32) -> u8 {
        (item / 10.0).floor() as u8
    }*/

    pub fn run_string(&self) -> String {
        let mut chars = self.output.chars();
        let mut last_c = 0u8;
        let g = Self::default_arr();
        let mut out = String::new();

        for c in chars {
            let a = &g[last_c.u()][(c as u8).u()];
            let b = &g[0][(c as u8).u()];

            if a.len() <= b.len() {
                out.push_str(a);
            } else {
                out.push_str(&format!(">{b}"));
            }
            out.push('.');
            last_c = c as u8;
        }

        out
    }

    fn repeat(s: String, times: isize) -> String {
        let mut b = String::new();
        for _ in 0..times {
            b.push_str(&s)
        }

        b
    }

    fn default_arr() -> &'static [[&'static str; 256]; 256] {
        crate::bfdef::DEFAULT_ARRAY
    }

    pub fn calculate_default_arr() -> Vec<Vec<String>> {
        let mut v = vec![vec![String::new(); 256]; 256];

        for x in 0..256isize {
            for y in 0..256isize {
                let mut delta = y-x;
                if delta > 128 {
                    delta -= 256;
                }
                if delta < -128 {
                    delta += 256;
                }

                if delta >= 0 {
                    v[x.u()][y.u()] = Self::repeat("+".into(), delta)
                } else {
                    v[x.u()][y.u()] = Self::repeat("-".into(), -delta)
                }
            }
        }

        let mut iter = true;

        while iter {
            iter = false;

            for x in 0..256 {
                for n in 1..40 {
                    for d in 1..40 {
                        let mut j = x;
                        let mut y = 0;

                        for _ in 0..256 {
                            if j == 0 {
                                break;
                            }
                            j = (j - d + 256) & 255;
                            y = (y + n) & 255;
                        }

                        if j == 0 {
                            let mut s = format!(
                                "[{}>{}<]>",
                                Self::repeat("-".into(), d),
                                Self::repeat("+".into(), n)
                            );

                            if s.len() < v[x.u()][y.u()].len() {
                                v[x.u()][y.u()] = s;
                                iter = true;
                            }
                        }

                        j = x;
                        y = 0;

                        for _ in 0..256 {
                            if j == 0 {
                                break;
                            }

                            j = (j + d) & 255;
                            y = (y - n + 256) & 255;
                        }

                        if j == 0 {
                            let s = format!(
                                "[{}>{}<]>",
                                Self::repeat("+".into(), d),
                                Self::repeat("-".into(), n)
                            );

                            if s.len() < v[x.u()][y.u()].len() {
                                v[x.u()][y.u()] = s;
                                iter = true;
                            }
                        }
                    }
                }
            }

            for x in 0..256 {
                for y in 0..256 {
                    for z in 0..256 {
                        if v[x.u()][z.u()].len() + v[z.u()][y.u()].len() < v[x.u()][y.u()].len() {
                            v[x.u()][y.u()] = format!("{}{}", v[x.u()][z.u()], v[z.u()][y.u()]);
                            iter = true;
                        }
                    }
                }
            }
        }

        v
    }

}

trait ToUsize {
    fn u(self) -> usize;
}

impl ToUsize for isize {
    fn u(self) -> usize {
        self as usize
    }
}

impl ToUsize for u8 {
    fn u(self) -> usize {
        self as usize
    }
}

impl ToUsize for i32 {
    fn u(self) -> usize {
        self as usize
    }
}
