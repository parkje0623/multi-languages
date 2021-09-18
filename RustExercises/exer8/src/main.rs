pub mod find;
pub mod hailstone;
pub mod rational;

fn main() {
    // nothing is required here, but you may want to use it for testing.
    println!("Hailstone: {}", hailstone::hailstone(17));
    println!("Hailstone: {}", hailstone::hailstone(18));
    println!("Hailstone: {}", hailstone::hailstone(1));

    println!("Hailstone_Append: {:?}", hailstone::hailstone_sequence_append(5));

    println!("Hailstone_Prealloc: {:?}", hailstone::hailstone_sequence_prealloc(5));

    let v1: Vec<i32> = Vec::from([4, 5, 2, 8, 7, 3, 1]);
    println!("FindElt: {:?}", find::find_elt(&v1, 8));
    println!("FindElt: {:?}", find::find_elt(&v1, 6));
    let v2: Vec<char> = "Hello World!".chars().collect();
    println!("FindElt: {:?}", find::find_elt(&v2, 'o'));
    println!("FindElt: {:?}", find::find_elt(&v2, 'q'));

    let mut r = rational::Rational::new(6, 8);
    println!("New Rational: {:?}", r);
    r.reduce();
    println!("Reduce Rational: {:?}", r);
    println!("From Rational: {:?}", rational::Rational::from(4_i64));
    println!("Boolean Rational: {}", rational::Rational::from(4_i64) == rational::Rational::new(4, 1));
}
