use std::fs::read_to_string;
use std::path::Path;

#[must_use]
pub fn read_input(day: usize, example: bool, part: u8) -> Option<String> {
    read_to_string(
        get_possible_paths(day, example, part)
            .iter()
            .find(|path| Path::new(path).exists())?,
    )
    .ok()
    .map(|s| s.replace('\r', ""))
}

fn get_possible_paths(day: usize, example: bool, part: u8) -> Vec<String> {
    let mut paths = Vec::new();
    if example {
        paths.push(format!("inputs/{day:02}-example-{part}.txt"));
        paths.push(format!("inputs/{day:02}-example.txt"));
    }
    paths.push(format!("inputs/{day:02}.txt"));
    paths
}
