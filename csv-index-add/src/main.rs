use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let mut file = File::open("wordle.csv").expect("failed to open csv");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("failed to read to buffer");


    let mut index = 0;
    let rows: Vec<String> = buffer.lines().map(|word| {
        index += 1;
        format!("{}, {}", index, word)
    })
        .collect();

    let mut output = File::create("words_with_index.csv")?;
    output.write_all(rows.join("\n").as_bytes());

    Ok(())
}
