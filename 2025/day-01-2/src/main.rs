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
        password += count_zero_clicks(pointer, args.size as i32, rotation).abs();
        pointer = (pointer + rotation).rem_euclid(args.size as i32);
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

fn count_zero_clicks(pointer: i32, size: i32, rotation: i32) -> i32 {
    let previous_group = calculate_group_for_pointer(pointer, size, rotation);
    let new_group = calculate_group_for_pointer(pointer + rotation, size, rotation);
    (new_group - previous_group).abs()
}

/// Calculates the group for a given pointer position, size, and rotation.
///
/// The group is a useful concept for determining whether the point `0` is clicked
/// or not: we just need to check if the group changed or not.
/// The trick with subtracting 1 if rotation is negative enables us to quickly account
/// for situations like "we are currently at zero, but will move to -1" - this doesn't
/// click `0`.
fn calculate_group_for_pointer(pointer: i32, size: i32, rotation: i32) -> i32 {
    (if rotation < 0 { pointer - 1 } else { pointer }).div_euclid(size)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_rotation() {
        assert_eq!(parse_rotation("L90"), Ok(-90));
        assert_eq!(parse_rotation("R180"), Ok(180));
        assert_eq!(parse_rotation("U270"), Err("Invalid direction".to_string()));
    }

    #[test]
    fn test_description_logic() {
        // The dial starts by pointing at 50.
        // The dial is rotated L68 to point at 82; during this rotation, it points at 0 once.
        // The dial is rotated L30 to point at 52.
        // The dial is rotated R48 to point at 0.
        // The dial is rotated L5 to point at 95.
        // The dial is rotated R60 to point at 55; during this rotation, it points at 0 once.
        // The dial is rotated L55 to point at 0.
        // The dial is rotated L1 to point at 99.
        // The dial is rotated L99 to point at 0.
        // The dial is rotated R14 to point at 14.
        // The dial is rotated L82 to point at 32; during this rotation, it points at 0 once.
        let size = 100;
        let mut pointer = 50;
        let mut password = 0;
        let rotations = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ];
        let expected_passwords = vec![1, 1, 2, 2, 3, 4, 4, 5, 5, 6];
        for (line, expected_password) in rotations.into_iter().zip(expected_passwords.into_iter()) {
            let rotation = parse_rotation(line).unwrap();
            password += count_zero_clicks(pointer, size, rotation);
            pointer = (pointer + rotation).rem_euclid(size as i32);
            assert_eq!(password, expected_password);
        }
    }
}
