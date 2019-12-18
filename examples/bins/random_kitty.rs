use rand::{thread_rng, seq::SliceRandom};

const CATS: [&str; 24] = [
    "Angel",
    "Bella",
    "Beyonce",
    "Casper",
    "Cher",
    "Cupcake",
    "Dexter",
    "Dusty",
    "Felix",
    "Gracie",
    "Jasmine",
    "Milo",
    "Misty",
    "Oscar",
    "Precious",
    "Rusty",
    "Sadie",
    "Shakira",
    "Simon",
    "Smokey",
    "Snickers",
    "Toby",
    "Zeus",
    "Zoe",
];

fn main() {
    let cat = CATS.choose(&mut thread_rng());
    println!("{}", cat.unwrap());
}
