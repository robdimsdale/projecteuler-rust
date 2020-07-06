use projecteuler::*;

fn main() {
    println!("Problem 1: {}", p1());
    println!("Problem 2: {}", p2());
    println!("Problem 3: {}", p3());
    println!("Problem 4: {}", p4());
    println!("Problem 5: {}", p5());
    println!("Problem 6: {}", p6());
    println!("Problem 7: {}", p7());

    println!("Problem 76: {}", p76());

    println!(
        "Pascal triangle row 13: {:?}",
        PascalTriangle::build(13).row(12).unwrap()
    );
}
