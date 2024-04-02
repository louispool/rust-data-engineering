use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
use clap::Parser;

#[derive(Parser)]
struct Opts {
    #[clap(short, long, default_value = "100")]
    /// Number of random fruits to generate
    count: Option<usize>,
}

fn generate_fruit() -> &'static str {
    let fruits = [
        "Apple",
        "Banana",
        "Cherry",
        "Date",
        "Elderberry",
        "Fig",
        "Grape",
        "Honeydew",
    ];
    let mut rng = thread_rng();
    fruits.choose(&mut rng).unwrap()
}

fn main() {

    let opts: Opts = Opts::parse();
    let count = opts.count.unwrap_or(100);

    //Map of fruits to their count
    let mut fruit_map = std::collections::HashMap::new();

    let mut fruit_set = HashSet::new();
    println!("Generating {} random fruits...", count);
    for _ in 0..count {

        //Generate a random fruit
        let fruit = generate_fruit();
        fruit_set.insert(fruit);

        //Increment the count of the fruit
        let count = fruit_map.entry(fruit).or_insert(0);
        *count += 1;
    }

    println!("Number of unique fruits generated: {}", fruit_set.len());

    //Print the unique fruits
    for fruit in fruit_set.iter() {
        println!("{}", fruit);
    }

    //Print the count of each fruit
    for (fruit, count) in fruit_map.iter() {
        println!("Generated {}, {} times", fruit, count);
    }
}
