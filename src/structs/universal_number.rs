use std::ops::Add;

#[derive(Copy, Clone)]
pub struct UniversalNumber {
    n: Option<i64>,
    f: Option<f64>
}

impl UniversalNumber {
    pub fn from_str(number_str: &str) -> Result<Self, &'static str> {
        let f = number_str.parse::<f64>();
        if f.is_ok() {
            return Ok(Self {
                n: None,
                f: Some(f.unwrap())
            });
        }
        let n = number_str.parse::<i64>();
        if n.is_ok() {
            return Ok(Self {
                n: Some(n.unwrap()),
                f: None
            });
        }
        return Err("Parse error");
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
                new_num.f = Some((self.n.unwrap() as f64) + rhs.n.unwrap());
            }
            else if rhs.f.is_some() {
                new_num.f = Some(self.f.unwrap() + rhs.f.unwrap());
            }
        }
        return new_num;
    }
}