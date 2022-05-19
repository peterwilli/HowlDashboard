use std::fmt;
use std::ops::{Add, AddAssign};

#[derive(Copy, Clone, Default)]
pub struct UniversalNumber {
    pub(crate) n: Option<i64>,
    pub(crate) f: Option<f64>
}

impl UniversalNumber {
    pub fn zero() -> Self {
        return Self {
            n: Some(0),
            ..Default::default()
        };
    }

    pub fn from_str(number_str: &str) -> Result<Self, &'static str> {
        let n = number_str.parse::<i64>();
        if n.is_ok() {
            return Ok(Self {
                n: Some(n.unwrap()),
                f: None
            });
        }
        let f = number_str.parse::<f64>();
        if f.is_ok() {
            return Ok(Self {
                n: None,
                f: Some(f.unwrap())
            });
        }
        return Err("Parse error");
    }
}

impl AddAssign for UniversalNumber {
    fn add_assign(&mut self, other: Self) {
        *self += other
    }
}

impl Add for UniversalNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new_num = self.clone();
        if self.n.is_some() {
            if rhs.n.is_some() {
                new_num.n = Some(self.n.unwrap() + rhs.n.unwrap());
            }
            else if rhs.f.is_some() {
                new_num.f = Some((self.n.unwrap() as f64) + rhs.f.unwrap());
                new_num.n = None;
            }
        }
        else if self.f.is_some() {
            if rhs.n.is_some() {
                new_num.f = Some((rhs.n.unwrap() as f64) + self.f.unwrap());
            }
            else if rhs.f.is_some() {
                new_num.f = Some(self.f.unwrap() + rhs.f.unwrap());
            }
        }
        return new_num;
    }
}

impl fmt::Display for UniversalNumber {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        if self.n.is_some() {
            write!(f, "{}", self.n.unwrap())
        }
        else {
            write!(f, "{}", self.f.unwrap())
        }
    }
}