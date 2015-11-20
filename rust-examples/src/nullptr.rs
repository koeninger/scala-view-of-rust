use std::mem::size_of;

fn no_blowup(opt: &Option<u32>) {
    opt.map(|x| println!("I got an {:?}", x));
}

fn main() {
    let x = Some(23);
    // COMPILE warning: single-assignment variable doesn't need to be mutable
    let mut y;
    
    no_blowup(&x);

    // COMPILE error: use of possibly uninitialized variable: `y`
    // no_blowup(&y);

    // COMPILE error: mismatched types
    // y = std::ptr::null();
    // no_blowup(&y);

    y = None;
    no_blowup(&y);

    // because references cant be null, enums with only 1 non-empty variant are optimized to a single pointer
    println!("size of u8 {}", size_of::<u8>());
    println!("size of &u8 {}", size_of::<&u8>());
    println!("size of Option<&u8> {}", size_of::<Option<&u8>>());
    println!("size of Option<Box<&u8>> {}", size_of::<Option<Box<&u8>>>());
}
