use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::env;

fn main() -> io::Result<()> {
    let current_dir = env::current_dir()?;

    let log_file_path = current_dir.join("idea.log");

    if !log_file_path.exists() {
        eprintln!("Error: File 'idea.log' not found in the current directory.");
        return Ok(());
    }

    let log_file = File::open(log_file_path)?;
    let reader = io::BufReader::new(log_file);

    let mut block = Vec::new();
    let mut block_number = 1;
    let mut in_block = false;

    for line in reader.lines() {
        let line = line?;

        if line.contains("IDE STARTED") {
            if in_block && !block.is_empty() {
                save_block_to_file(&block, block_number, &current_dir)?;
                block.clear();
                block_number += 1;
            }
            in_block = true;
        }

        if in_block {
            block.push(line.clone());
        }

        if line.contains("IDE SHUTDOWN") {
            save_block_to_file(&block, block_number, &current_dir)?;
            block.clear();
            block_number += 1;
            in_block = false;
        }
    }


    if !block.is_empty() {
        save_block_to_file(&block, block_number, &current_dir)?;
    }

    Ok(())
}

fn save_block_to_file(block: &Vec<String>, block_number: usize, dir: &Path) -> io::Result<()> {
    let file_name = format!("idea_block_{}.log", block_number);
    let file_path = dir.join(file_name);
    let mut file = File::create(file_path)?;
    for line in block {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}
