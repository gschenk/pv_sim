use serde::Deserialize;

pub mod consume;

#[derive(Deserialize, Debug)]
pub struct Data {
    time: usize,
    power: f64,
}

fn main() {
    let foo = |x| println!("{:?}", x);
    let _ = consume::receive(&foo);
}
