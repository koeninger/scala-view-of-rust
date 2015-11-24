use std::ops::Add;
use std::mem::size_of;

#[derive(Debug)]
struct Inches(i32);

impl Add for Inches {
    type Output = Inches;

    fn add(self, other: Inches) -> Inches {
        Inches(self.0 + other.0)
    }
}

#[derive(Debug)]
struct Feet(i32);

impl Add for Feet {
    type Output = Feet;

    fn add(self, other: Feet) -> Feet {
        Feet(self.0 + other.0)
    }
}

fn main() {
    let i = Inches(1);
    let i2 = Inches(2);
    let f = Feet(1);
    let f2 = Feet(2);
    println!("f + f2 = {:?}", f + f2);
    println!("i + i2 = {:?}", i + i2);
    // error: mismatched types
    //println!("f + i = {:?}", f + i);
    println!("size of i32 {}", size_of::<i32>());
    println!("size of &i32 {}", size_of::<&i32>());
    println!("size of [i32;4] {}", size_of::<[i32;4]>());
    println!("size of Feet {}", size_of::<Feet>());
    println!("size of &Feet {}", size_of::<&Feet>());
    println!("size of [Feet;4] {}", size_of::<[Feet;4]>());
}
