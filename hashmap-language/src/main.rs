use std::collections::HashMap;
use std::str::FromStr;
use clap::Parser;
use std::error::Error;

#[derive(Parser)]
struct Opts {
    
    /// A list of languages with their creation year as tuples separated by a comma. 
    /// e.g: `--languages "JavaScript,1995" "Python,1991" "C,1972"`
    #[clap(short, long, value_parser = parse_key_val::<String, u32>, num_args = 1.., value_delimiter = ' ')]
    languages: Option<Vec<(String, u32)>>,
}

// Parse a string of the form `key,value` into a tuple of the parsed key and value.
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: FromStr,
    U::Err: Error + Send + Sync + 'static,
{   
    // Find the position of the comma
    let pos = s.find(',').ok_or_else(|| format!("Expected tuple as `key,value` instead got {s}"))?;

    // Return the parsed tuple         
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

fn init_languages() -> HashMap<String, i32> {
    let mut languages = HashMap::new();
    languages.insert("JavaScript".to_string(), 1995);
    languages.insert("HTML/CSS".to_string(), 1990);
    languages.insert("Python".to_string(), 1991);
    languages.insert("SQL".to_string(), 1974);
    languages.insert("TypeScript".to_string(), 2012);
    languages.insert("Bash/Shell".to_string(), 1989);
    languages.insert("Java".to_string(), 1995);
    languages.insert("C#".to_string(), 2000);
    languages.insert("C++".to_string(), 1985);
    languages.insert("C".to_string(), 1972);
    languages.insert("PHP".to_string(), 1995);
    languages.insert("PowerShell".to_string(), 2006);
    languages.insert("Go".to_string(), 2007);
    languages.insert("Rust".to_string(), 2010);

    languages
}

fn calculate_weights(years_active: &mut HashMap<String, i32>) -> HashMap<String, i32> {
    
    // Subtract the creation year from 2024 to get the number of years active.
    for year in years_active.values_mut() {
        *year = 2024 - *year;
    }

    let min_year = *years_active.values().min().unwrap_or(&0);
    let max_year = *years_active.values().max().unwrap_or(&0);

    let mut weights = HashMap::new();

    for (language, &year) in years_active.iter() {
        let normalized_year = (year - min_year) as f64 / (max_year - min_year) as f64;
        let weight = (normalized_year * 99.0) as i32 + 1; // weight between 1 and 100
        weights.insert(language.to_string(), weight);
    }

    weights
}

fn main() {

    let opts = Opts::parse();

    let mut languages = init_languages();
    if let Some(input_langs) = opts.languages {

        for (language, year) in input_langs {
            languages.insert(language, year as i32);
        }
    }

    let weights = calculate_weights(&mut languages);

    // Convert to a vector of tuples for sorting
    let mut weights: Vec<_> = weights.into_iter().collect();
    weights.sort_by_key(|k| k.1);

    println!("Language weighing from 1-100 by age (1 is newest and 100 is oldest):");
    for (language, weight) in &weights {
        println!("{}: {}", language, weight);
    }
}
