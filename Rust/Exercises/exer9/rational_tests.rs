#[cfg(test)]
use crate::rational::Rational;

#[test]
fn rational_test() {
    let mut r = Rational::new(6, 8);
    assert_eq!(format!("{:?}", r), "Rational { n: 6, d: 8 }");
    r.reduce();
    assert_eq!(format!("{:?}", r), "Rational { n: 3, d: 4 }");
    let four1 = Rational::from(4);
    let four2 = Rational::new(4, 1);
    assert_eq!(four1, four2);

    let s = format!("Format: {}", r);
    assert_eq!(format!("{}", s), "Format: 3/4");
    let t = format!("Format: {}", four2);
    assert_eq!(format!("{}", t), "Format: 4/1");

    let test = Rational::new(5, 10);
    let f: f64 = test.into();
    assert_eq!(format!("i64 to f64: {}", f), "i64 to f64: 0.5");
}