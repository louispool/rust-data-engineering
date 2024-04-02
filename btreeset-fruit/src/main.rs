use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::BTreeSet;
use clap::Parser;

#[derive(Parser)]
struct Opts {
    #[clap(short, long)]
    /// Name of a fruit to remove from the list
    remove_fruit: Option<String>,
}

fn main() {

    let opts: Opts = Opts::parse();
    
    let mut fruits = vec![
        "apple",
        "banana",
        "cherry",
        "date",
        "elderberry",
        "fig",
        "grape",
        "honeydew",
    ];

    //Remove a fruit if specified
    if let Some(fruit) = opts.remove_fruit {
        fruits.retain(|&f| f != fruit);
    }
    
    let amounts = [1, 3, 5, 7, 9];
    let mut rng = thread_rng();

    //Map of fruits to their count
    let mut fruit_map = std::collections::HashMap::new();        

    for amount in amounts.iter() {

        let mut fruit_set = BTreeSet::new();
        let mut shuffled_fruits = fruits.clone();
        shuffled_fruits.shuffle(&mut rng);

        for fruit in shuffled_fruits {

            //Increment the count of the fruit
            let count = fruit_map.entry(fruit).or_insert(0);
            *count += 1;

            fruit_set.insert(fruit);
            if fruit_set.len() >= *amount {
                break;
            }
        }

        print!("{}: ", amount);
        for fruit in fruit_set.iter().rev() {
            print!("{} ", fruit);
        }
        println!();
    }

    //Print the count of each fruit
    for (fruit, count) in fruit_map.iter() {
        println!("Generated {}, {} times", fruit, count);
    }        

}
