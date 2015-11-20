#[derive(Debug)]
struct Entity {
    id: u32
}

impl Entity {
    fn new(id: u32) -> Entity {
        Entity { id: id }
    }
}

impl Drop for Entity {
    fn drop(&mut self) {
        println!("Dropping {}", self.id);
    }
}

fn main() {
    let three = outer();
    println!("Leaving main"); // Dropping three
}

fn outer() -> Entity {
    let one = Entity::new(1);
    let two = Entity::new(2);
    let three = Entity::new(3);
    
    inner(two);

    // COMPILE error: use of moved value
    //println!("this would be use-after-free {:?}", two);

    println!("Leaving Outer");        
    three // Dropping one
}

fn inner(e: Entity) {
    println!("Leaving Inner"); // Dropping two
}

// COMPILE error: s does not live long enough
//fn as_str(e: &Entity) -> &str {
//    let s = format!("{:?}", e);
//    &s
//}
