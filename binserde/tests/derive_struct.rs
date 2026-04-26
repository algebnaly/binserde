use binserde::{Decode, Encode};

#[allow(unused)]
#[derive(Encode, Decode)]
struct Person {
    name: String,
    age: u32,
}

fn main() {}
