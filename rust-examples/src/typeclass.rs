trait ToJson {
    fn to_json_str(&self) -> String;
}

impl ToJson for i32 {
    fn to_json_str(&self) -> String {
        self.to_string()
    }
}

impl<T: ToJson> ToJson for Vec<T> {
    fn to_json_str(&self) -> String {
        format!("[{}]",
                self.iter()
                    .map(|x| x.to_json_str())
                    .collect::<Vec<_>>()
                    .join(", "))
    }
}

fn main() {
    let v = vec![1, 2, 3, 4];
    println!("{}", v.to_json_str());
}
