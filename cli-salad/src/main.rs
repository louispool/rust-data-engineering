use clap::Parser;
use cli_salad::create_fruit_salad;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Your Name <your.email@example.com>",
    about = "Number of fruits to include in the salad"
)]
struct Opts {
    #[clap(short, long)]
    /// Number of fruits to include in the salad
    number: usize,

    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    /// List of space delimited names of the fruits to include 
    /// in the salad: `--fruit Apple Pear Orange`
    fruit: Option<Vec<String>>,    
}

fn main() {
    let opts: Opts = Opts::parse();

    // Get the number of fruits the user requested
    let num_fruits = opts.number;

    // Create the fruit salad
    let mut salad = if opts.fruit.is_some() {
        opts.fruit.unwrap()
    } 
    else {
        create_fruit_salad(num_fruits)
    };

    // Print the fruit salad in human readable format with a count of fruits used
    println!("Created Fruit salad with {} fruits: {:?}", num_fruits, salad);

    salad.sort();

    // Print the fruit salad in human readable format with a count of fruits used
    println!("Fruit salad sorted alphabetically {:?}", salad);


}
