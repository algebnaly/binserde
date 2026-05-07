use minibserde::{Decode, Encode};

#[allow(unused)]
#[derive(Encode, Decode)]
struct Person {
    name: String,
    age: u32,
}

#[allow(unused)]
#[derive(Encode, Decode)]
struct Point(i32, i32);

fn main() {}
