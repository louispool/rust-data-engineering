use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ord;
use std::collections::{BinaryHeap, HashMap};
use clap::Parser;

#[derive(Parser)]
struct Opts {
    #[clap(short, long)]
    /// Name of a fruit to remove from the list
    remove_fruit: Option<String>,
}

#[derive(Eq, PartialEq)]
enum Fruit {
    Fig,
    Other(String),
}

// We define Figs as the highest priority by implementing Ord
impl Ord for Fruit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Fruit::Fig, Fruit::Fig) => std::cmp::Ordering::Equal,
            (Fruit::Fig, Fruit::Other(_)) => std::cmp::Ordering::Greater,
            (Fruit::Other(_), Fruit::Fig) => std::cmp::Ordering::Less,
            (Fruit::Other(_), Fruit::Other(_)) => std::cmp::Ordering::Equal,
        }
    }
}

impl PartialOrd for Fruit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(unused_parens)]
fn generate_fruit_salad(freq_map: &mut HashMap<String, usize>) -> BinaryHeap<Fruit> {

    let mut rng = thread_rng();
    let fruits = vec![
        "Apple",
        "Orange",
        "Pear",
        "Peach",
        "Banana",
        "Fig",
        "Fig",
        "Fig",
        "Fig",
    ];
    let mut fruit_salad = BinaryHeap::new();

    loop {

        let fruit = fruits.choose(&mut rng).unwrap();
        let count = freq_map.entry(fruit.to_string()).or_insert(0);
        *count += 1;
    
        if (*fruit == "Fig") {
            
            fruit_salad.push(Fruit::Fig);
            if (*count == 2) {
                break;
            }
        } 
        else {
            fruit_salad.push(Fruit::Other(fruit.to_string()));
        }
    }
    fruit_salad
}

fn main() {
    
    let opts: Opts = Opts::parse();
    
    //Map of fruits to their count
    let mut fruit_map = std::collections::HashMap::new();        

    let mut fruit_salad = generate_fruit_salad(&mut fruit_map);
    // Remove a fruit if specified
    if let Some(fruit) = opts.remove_fruit {
        fruit_salad.retain(|f| match f {
            Fruit::Fig => fruit.to_lowercase() != "fig",
            Fruit::Other(fruit_name) => fruit_name.to_lowercase() != fruit.to_lowercase(),
        });
    }

    println!("Random Fruit Salad With Two Servings of Figs:");
    for fruit in fruit_salad.into_sorted_vec().iter().rev() {
        match fruit {
            Fruit::Fig => println!("Fig"),
            Fruit::Other(fruit_name) => println!("{}", fruit_name),
        }
    }

    //Print the count of each fruit
    for (fruit, count) in fruit_map.iter() {
        println!("Generated {}, {} times", fruit, count);
    }
}
