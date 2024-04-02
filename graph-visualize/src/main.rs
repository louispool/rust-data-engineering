extern crate rasciigraph;

use std::error::Error;
use rasciigraph::{plot, Config};
use clap::Parser;

#[derive(Parser)]
struct Opts {
    #[clap(short, long)]
    /// Data File to use
    file: Option<String>
}

struct Record {
    city: String,
    distance: f64
}

fn read_csv(file: String) -> Result<Vec<Record>, Box<dyn Error>> {
    
    //Read CSV file, deserialize to Record and return a vector
    let mut rdr = csv::Reader::from_path(file)?;
    let mut records = Vec::new();

    for result in rdr.records() {
        
        let record = result?;
        records.push(Record {
            city: record[0].to_string(),
            distance: record[1].trim().parse()?
        });
    }
    
    Ok(records)
}

fn main() {

    let opts: Opts = Opts::parse();

    let records = match opts.file {
        Some(file) => read_csv(file).unwrap(),
        None => vec![
            Record { city: "Lisbon".to_string(), distance: 0.0 },
            Record { city: "Madrid".to_string(), distance: 502.56 },
            Record { city: "Paris".to_string(), distance: 1053.36 },
            Record { city: "Berlin".to_string(), distance: 2187.27 },
            Record { city: "Copenhagen".to_string(), distance: 2636.42 },
            Record { city: "Stockholm".to_string(), distance: 3117.23 },
            Record { city: "Moscow".to_string(), distance: 4606.35 }
        ]
    };

    //Print the city strings in the records
    let cities: Vec<String> = records.iter().map(|r| r.city.clone()).collect();
    println!("{}", cities.join(" > "));

    //Plot the distances    
    let distances_travelled: Vec<f64> = records.iter().map(|r| r.distance).collect();
    println!(
        "{}",
        plot(
            distances_travelled.into_iter().map(|d| d as f64).collect(),
            Config::default()
                .with_offset(10)
                .with_height(10)
                .with_caption("Travelled distances (km)".to_string())
        )
    );
}
