/*
This example code counts the frequency of each number in the vector.
 */
use std::collections::HashMap;
use std::hash::Hash;
use clap::Parser;

#[derive(Parser)]
struct Opts {

    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ',')]
    /// Sequence of comma delimited numbers : `--numbers 1,2,3,4,5,6,7,8,9,1,3`
    numbers: Option<Vec<i32>>,
    
    #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ',')]
    /// Sentence to count the frequency of each word : `--sentence "That that is is that that is not"`
    sentence: Option<String>
}

/// Determines the frequency of each value in the input vector. 
/// 
/// Value can be any type that implements Eq, Hash and Copy.
/// 
/// The Return value is a vector of tuples where the first element is the value 
/// and the second element is the frequency.
fn logic<T: Eq + Hash + Copy>(values: &Vec<T>) -> Vec<(T, u32)> {
    
    let mut frequencies = HashMap::new();

    for &val in values {
        let frequency = frequencies.entry(val).or_insert(0);
        *frequency += 1;
    }

    let mut result = Vec::new();

    for (num, frequency) in frequencies {
        result.push((num, frequency));
    }

    result
}

fn main() {

    let opts = Opts::parse();
    
    let numbers = if let Some(numbers) = opts.numbers {
        numbers
    } else {
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 3]
    };
    let mut result = logic(&numbers);

    //Sort such that higher frequencies are first
    result.sort_by(|a, b| b.1.cmp(&a.1));
    
    //print the results in a human readable format that explains what the result is.
    println!("The frequency of each number in the sequence:\n {:?} is:\n {:?}", numbers, result);

    let sentence = if let Some(sentence) = opts.sentence {
        sentence
    } else {
        "That that is is that that is not".to_string()
    };

    let words: Vec<&str> = sentence.split_whitespace().collect();
    let mut result = logic(&words);

    //Sort such that higher frequencies are first
    result.sort_by(|a, b| b.1.cmp(&a.1));

    //print the results in a human readable format that explains what the result is.
    println!("The frequency of each word in the sentence:\n {:?} is:\n {:?}", words, result);
}
