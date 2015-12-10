/// A product type
#[derive(Debug)]
pub struct Point {
    x: f64,
    y: f64,
}
impl Point {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// /// A sum type
// pub enum Option<T> {
//     None,
//     Some(T)
// }
// impl<T> Option<T> {
//     #[inline]
//     pub fn is_some(&self) -> bool {
//         match *self {
//             Some(_) => true,
//             None => false,
//         }
//     }
// }

fn main() {
    let o = Some(Point { x: 23.0, y: 42.0 });
    println!("{:?}", o.map(|p| p.distance_from_origin()));
}
