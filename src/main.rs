use num_cpus::*;
use rand::random;

fn main() {
    let cpus = num_cpus::get();

    println!("Hello, world! cpu {}", cpus);
    let x = 5 + 5;
    println!("Is `x` 10 or 100? x = {}", x);
    let random_boolean = rand::random();
    println!("You {}!", if random_boolean { "win" } else { "lose" });
}
