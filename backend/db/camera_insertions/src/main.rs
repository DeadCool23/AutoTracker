use rand::Rng;

fn main() {
    let mut rng = rand::rng();
    println!("{}", rng.random_range(1..=3))
}