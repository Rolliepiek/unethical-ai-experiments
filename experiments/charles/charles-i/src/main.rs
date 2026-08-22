use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use std::vec;
use rand::RngExt;
#[test]
fn main() {
    let mut rng = rand::rng();
    struct Senses {
        eyes: f64,
        nose: f64,
        ears: f64,
        skin: f64,
        mouth: f64,
    }
    struct Brain {
        eyes: Vec<f64>,
        nose: Vec<f64>,
        ears: Vec<f64>,
        skin: Vec<f64>,
        mouth: Vec<f64>,
    }
    let mut input = Senses {
        eyes: 0.0,
        nose: 0.0,
        ears: 0.0,
        skin: 0.0,
        mouth: 0.0,
    };
    let mut charles = Brain {
        eyes: vec![rng.random_range(0.0..2.0), 0.0],
        nose: vec![rng.random_range(0.0..2.0), 0.0],
        ears: vec![rng.random_range(0.0..2.0), 0.0],
        skin: vec![rng.random_range(0.0..2.0), 0.0],
        mouth: vec![rng.random_range(0.0..2.0), 0.0],
    };
    let mut temp: Vec<f64> = vec![0.0; 5];
    let mut emotions: f64;
    let lifetime: i32 = rng.random_range(1..10000);
    for _i in 0..lifetime {
        input.eyes = rng.random_range(0.0..2.0);
        input.nose = rng.random_range(0.0..2.0);
        input.ears = rng.random_range(0.0..2.0);
        input.skin = rng.random_range(0.0..2.0);
        input.mouth = rng.random_range(0.0..2.0);
        charles.eyes[1] = rng.random_range(0.0..2.0);
        charles.nose[1] = rng.random_range(0.0..2.0);
        charles.ears[1] = rng.random_range(0.0..2.0);
        charles.skin[1] = rng.random_range(0.0..2.0);
        charles.mouth[1] = rng.random_range(0.0..2.0);
        temp[0] = input.eyes * charles.eyes[0] * charles.eyes[1];
        temp[1] = input.nose * charles.nose[0] * charles.nose[1];
        temp[2] = input.ears * charles.ears[0] * charles.ears[1];
        temp[3] = input.skin * charles.skin[0] * charles.skin[1];
        temp[4] = input.mouth * charles.mouth[0] * charles.mouth[1];
        emotions = (temp[0] + temp[1] + temp[2] + temp[3] + temp[4]) / 5.0;
        print!("\r{}", emotions);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(10));
    }
    println!("\nCharles I died.");
}