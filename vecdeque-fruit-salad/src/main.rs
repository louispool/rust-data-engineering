/*
This code starts with an initial VecDeque,
converts it to a Vec for shuffling, and then converts it back to a VecDeque.
After that, it pushes "Pomegranate" to the front of the deque, and "Fig" and "Cherry"
to the back of the deque. Finally, it prints out the final fruit salad.

A VecDeque is a double-ended queue, which means that you can push and pop from both ends
of the queue.
*/

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;
use std::collections::VecDeque;
use clap::Parser;

#[derive(clap::ValueEnum, Clone)]
enum WhichEnd {
    Start,
    End
}

#[derive(Parser)]
struct Opts {

    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    /// List of space delimited names of the fruits to include 
    /// at the start of the salad : `--start-fruit Apple Pear Orange`
    start_fruit: Option<Vec<String>>,

    /// List of space delimited names of the fruits to include 
    /// at the end of the salad : `--end-fruit Apple Pear Orange`
    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    end_fruit: Option<Vec<String>>,    

    /// Which end to remove fruit from
    #[clap(short, long)]
    remove: Option<WhichEnd>
}

fn main() {

    let Opts{start_fruit, end_fruit, remove} = Opts::parse();

    let mut salad: VecDeque<String> = VecDeque::new();
    salad.push_back("Arbutus".to_string());
    salad.push_back("Loquat".to_string());
    salad.push_back("Strawberry Tree Berry".to_string());

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    let mut salad: Vec<_> = salad.into_iter().collect();
    salad.shuffle(&mut rng);

    // Convert it back to VecDeque
    let mut salad: VecDeque<_> = salad.into_iter().collect();

    // Add fruits to the both ends of the queue after shuffling
    salad.push_front("Pomegranate".to_string());
    salad.push_back("Fig".to_string());
    salad.push_back("Cherry".to_string());

    if let Some(fruits) = start_fruit {
        for fruit in fruits.into_iter().rev() { //into_iter().rev() iterates in reverse order and moves the elements, as opposed to copying them
            salad.push_front(fruit);
        }
    }

    if let Some(fruits) = end_fruit {
        for fruit in fruits {
            salad.push_back(fruit);
        }
    }

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in salad.iter().enumerate() {
        if i != salad.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }

    //Convert Deque to Vec for random trait to work
    let salad: Vec<String> = salad.into_iter().collect();
    let random_fruit = salad.choose(&mut rng).unwrap().to_string();

    println!("Random fruit: {}", random_fruit);

    // Convert it back to VecDeque
    let mut salad: VecDeque<_> = salad.into_iter().collect();
    if let Some(WhichEnd::Start) = remove {
       println!("Remove from start {}", salad.pop_front().unwrap_or("nothing".to_string()));
    } 
    else if let Some(WhichEnd::End) = remove {
       println!("Remove from end {}", salad.pop_back().unwrap_or("nothing".to_string()));
    }
}
