/*
As with the VecDeque example, this code starts by creating a LinkedList of fruits,
converts it to a Vec for shuffling, and then converts it back to a LinkedList.
After the shuffling, it adds "Pomegranate", "Fig", and "Cherry" to the end of the list.
Finally, it prints out the final fruit salad.

This example shows how to use a LinkedList, but remember that LinkedList
has a higher memory overhead and worse cache locality than Vec or VecDeque,
so it's typically not the best choice unless you have a specific need for the properties 
of a linked list. In Rust, it's usually better to use a Vec or VecDeque.

A LinkedList is a doubly-linked list, which means that each element in the list
has a pointer to the next element and the previous element.
A great example of when to use a LinkedList is when you need to insert or remove elements
from the middle of the list.
*/

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;
use std::collections::LinkedList;
use clap::{Args, Parser, Subcommand};

#[derive(Subcommand)]
enum FruitCommands {

    /// Add a fruit to the salad
    Add(AddFruitArgs),

    /// Remove a fruit from the salad
    Remove(RemoveFruitArgs),

    // Just list the fruit in the salad
    List
}

#[derive(Args)]
struct AddFruitArgs{
    
    /// Name of the fruit to add
    name: String,

    /// Position to add the fruit at
    position: Option<usize>
}

#[derive(Args)]
struct RemoveFruitArgs{

    /// Position of the fruit to remove
    position: usize
}

#[derive(Parser)]
struct Opts {

    #[command(subcommand)]
    command: FruitCommands,
}

fn insert_at(list: &mut LinkedList<String>, idx: usize, value: String) {
    
    let mut tail = list.split_off(idx);
    list.push_back(value);
    list.append(&mut tail);
}

fn remove_from(list: &mut LinkedList<String>, idx: usize) {
    
    let mut tail = list.split_off(idx + 1);
    list.pop_back();
    list.append(&mut tail); 
}

fn print_salad(salad: &LinkedList<String>) {
    
    println!("Fruit Salad:");
    for (i, item) in salad.iter().enumerate() {
        if i != salad.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}

fn main() {

    let opts = Opts::parse();

    let mut fruit: LinkedList<String> = LinkedList::new();
    fruit.push_back("Arbutus".to_string());
    fruit.push_back("Loquat".to_string());
    fruit.push_back("Strawberry Tree Berry".to_string());

    /*
    Please note that converting a LinkedList to a Vec and back to a LinkedList 
    isn't a common operation in practice. I included 
    it in this example to keep the code as similar as possible 
    to the original VecDeque example.
     */

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    let mut salad: Vec<_> = fruit.into_iter().collect();
    salad.shuffle(&mut rng);

    // Convert it back to LinkedList
    let mut salad: LinkedList<_> = salad.into_iter().collect();

    // Add fruits to the both ends of the list after shuffling
    salad.push_front("Pomegranate".to_string());
    salad.push_back("Fig".to_string());
    salad.push_back("Cherry".to_string());

    // Print out the fruit salad
    print_salad(&salad);

    // Get a random fruit from the salad
    let salad: Vec<_> = salad.into_iter().collect();
    let random_fruit = salad.choose(&mut rng).unwrap().to_string();

    println!("Random fruit: {}", random_fruit);

    // Convert back to LinkedList
    let mut salad: LinkedList<_> = salad.into_iter().collect();

    match opts.command {
        
        FruitCommands::Add(args) => {
            if let Some(position) = args.position {
                println!("Adding fruit {} at position: {}", args.name, position);
                insert_at(&mut salad, position, args.name);
            }
            else {
                println!("Adding fruit {} at the end", args.name);
                salad.push_back(args.name);
            }
            print_salad(&salad);
            
        },
        FruitCommands::Remove(args) => {
            println!("Removing fruit at position: {}", args.position);
            remove_from(&mut salad, args.position);
            print_salad(&salad);
        }
        FruitCommands::List => {
            print_salad(&salad);
        }
    }    
}
