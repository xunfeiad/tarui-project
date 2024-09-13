use serde::{Deserialize, Serialize};
use serde_json::{self, Value};

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Person(String);

fn main() {
    let s = Person(String::from("123"));

    let ss: Value = serde_json::to_value(&s).unwrap();

    println!("{:?}", ss);
}
