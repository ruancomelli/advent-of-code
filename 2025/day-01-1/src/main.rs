use clap::Parser;
use std::io::{self, BufRead};
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    #[clap(short, long, default_value_t = 50)]
    position: i32,
    #[clap(short, long, default_value_t = 100)]
    size: u32,
    #[clap(short, long)]
    input_file: PathBuf,
}

fn main() {
    let args = Args::parse();
    let mut pointer = args.position;
    let file = std::fs::File::open(args.input_file).expect("Failed to open file");
    let reader = io::BufReader::new(file);
    let mut password: i32 = 0;
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let rotation = parse_rotation(&line).expect("Failed to parse line");
        pointer = (pointer + rotation).rem_euclid(args.size as i32);
        if pointer == 0 {
            password += 1;
        }
    }
    println!("Password: {}", password);
}

fn parse_rotation(line: &str) -> Result<i32, String> {
    if let Some(amount_str) = line.strip_prefix("L") {
        let amount: i32 = amount_str.parse().map_err(|_| "Invalid amount")?;
        Ok(-amount)
    } else if let Some(amount_str) = line.strip_prefix("R") {
        let amount = amount_str.parse().map_err(|_| "Invalid amount")?;
        Ok(amount)
    } else {
        Err("Invalid direction".to_string())
    }
}
