use std::fmt;

fn gcd(a: i64, b: i64) -> i64 {
    // TODO
    let mut max = a;
    let mut min = b;
    if min > max {
        let temp = max;
        max = min;
        min = temp;
    }

    loop {
        let gcd_val = max % min;
        if gcd_val == 0 {
            return min;
        }
        max = min;
        min = gcd_val;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rational {
    // TODO
    n: i64,
    d: i64,
}

impl Rational {
    pub fn new(n: i64, d: i64) -> Rational {
        // TODO
        Rational {
            n: n,
            d: d,
        }
    }
    // TODO: the reduce method
    pub fn reduce(&mut self) -> Rational {
        let divide = gcd(self.n, self.d);
        self.n = self.n/divide;
        self.d = self.d/divide;
        Rational {
            n: self.n,
            d: self.d,
        }
    }
}

impl From<i64> for Rational {
    // TODO
    fn from(src: i64) -> Rational {
        Rational {
            n: src,
            d: 1,
        }
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.n, self.d)
    }
}

impl From<Rational> for f64 {
    fn from(src: Rational) -> f64 {
        src.n as f64/src.d as f64
    }
}