/*
This program creates a fruit salad by scrambling (shuffling) a list of fruit.
A vector is a growable array. It can grow or shrink in size and is one of the most
useful data structures in Rust. A vector is represented using the Vec<T> type.
*/
use clap::Parser;

#[derive(Parser)]
struct Opts {

    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    /// List of space delimited names of the fruits to include 
    /// in the salad: `--fruit Apple Pear Orange`
    fruit: Option<Vec<String>>,
    
    /// Number of random fruits to include in the salad
    #[clap(short, long)]
    random: Option<usize>
}

fn main() {

    let Opts{mut fruit, random} = Opts::parse();

    let fruit = vector_fruit_salad::create_fruit_salad(random.unwrap_or(0), fruit.as_mut());

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
