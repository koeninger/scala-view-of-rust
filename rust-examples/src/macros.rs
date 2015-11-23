use std::ops::Add;

macro_rules! newtype_number {
    ($i:ident $t:ty) => {
        #[derive(Debug)]
        struct $i($t);
        
        impl Add for $i {
            type Output = $i;

            fn add(self, other: $i) -> $i {
                $i(self.0 + other.0)
            }
        }
    }
}

newtype_number!(Inches i32);
newtype_number!(Feet i32);

fn main() {
    let i = Inches(1);
    let i2 = Inches(2);
    let f = Feet(1);
    let f2 = Feet(2);
    println!("f + f2 = {:?}", f + f2);
    println!("i + i2 = {:?}", i + i2);
}
