use std::fmt;
use std::ops::{Add, AddAssign};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{EnumAccess, Error, MapAccess, SeqAccess, Visitor};

#[derive(Clone, Default, Copy, Debug)]
pub struct UniversalNumber {
    pub(crate) n: Option<i64>,
    pub(crate) f: Option<f64>
}

impl Serialize for UniversalNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        if self.n.is_some() {
            serializer.serialize_str(&format!("{}", self.n.unwrap()))
        }
        else {
            serializer.serialize_str(&format!("{}", self.f.unwrap()))
        }
    }
}

struct UNVisitor;

impl<'de> Visitor<'de> for UNVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string containing an integer between -2^31 and 2^31 or a float of the same range.")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        return Ok(v.to_string());
    }
}

impl<'de> Deserialize<'de> for UniversalNumber {
    fn deserialize<D>(deserializer: D) -> Result<UniversalNumber, D::Error>
        where
            D: Deserializer<'de>,
    {
        Ok(UniversalNumber::from_str(&deserializer.deserialize_string(UNVisitor).unwrap()).unwrap())
    }
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
                ..Default::default()
            });
        }
        let f = number_str.parse::<f64>();
        if f.is_ok() {
            return Ok(Self {
                f: Some(f.unwrap()),
                ..Default::default()
            });
        }
        return Err("Parse error");
    }
}

impl AddAssign for UniversalNumber {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
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

impl PartialEq<Self> for UniversalNumber {
    fn eq(&self, other: &Self) -> bool {
        return if self.n.is_some() {
            if other.n.is_some() {
                self.n.unwrap() == other.n.unwrap()
            } else {
                (self.n.unwrap() as f64) == other.f.unwrap()
            }
        } else {
            if other.n.is_some() {
                self.f.unwrap() == (other.n.unwrap() as f64)
            } else {
                self.f.unwrap() == other.f.unwrap()
            }
        }
    }
}

impl Eq for UniversalNumber {

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