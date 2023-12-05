use std::io::Read;

fn process(input: &str) -> u32 {
    input.lines()
        .map(|line| {
            let text_converter: [(&str, &str); 9] = [
                ("one", "1"),
                ("two", "2"),
                ("three", "3"),
                ("four", "4"),
                ("five", "5"),
                ("six", "6"),
                ("seven", "7"),
                ("eight", "8"),
                ("nine", "9"),
            ];

            let mut converted_line = line.to_string();
            for &(from, to) in &text_converter {
                converted_line = converted_line.replace(from, to);
            }

            let mut digits = line.chars().filter_map(|char| char.to_digit(10));

            let first = digits.next().expect("must be string").to_string();
            let last = digits.last().map(|char| char.to_string()).unwrap_or_else(|| first.clone().to_string());

            format!("{first}{last}").parse::<u32>().expect("a number")
        })
        .sum::<u32>()
}

#[test]
fn test_part1_1() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = std::fs::File::open("test_files/target_part1_1.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    assert_eq!(142, process(&buffer));
    Ok(())
}

#[test]
fn test_part1_2() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = std::fs::File::open("test_files/target_part1_2.txt")?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;
    dbg!(&process(&buffer));
    assert_eq!(56397, process(&buffer));
    Ok(())
}
