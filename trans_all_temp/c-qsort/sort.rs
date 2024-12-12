use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::env;
use std::cmp::Ordering;

static mut PV: Vec<String> = Vec::new();

fn display(option_u: bool) {
    let mut prev_line = String::new();
    unsafe {
        for line in PV.iter() {
            if option_u && prev_line == *line {
                continue;
            }
            println!("{}", line);
            prev_line = line.clone();
        }
    }
}

fn sort_main() -> io::Result<i32> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path> [options]", args[0]);
        return Ok(-1);
    }

    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);

    unsafe {
        for line in reader.lines() {
            let line = line?;
            PV.push(line);
        }
    }

    let mut option_u = false;
    let mut option_r = false;
    let mut option_n = false;

    if args.len() > 2 {
        match args[2].chars().nth(1) {
            Some('u') => option_u = true,
            Some('r') => option_r = true,
            Some('n') => option_n = true,
            _ => {}
        }
    }

    display(option_u);
    println!("--- after sort ---");

    unsafe {
        if option_r {
            PV.sort_by(|a, b| b.cmp(a));
        } else if option_n {
            PV.sort_by(|a, b| a.parse::<i32>().unwrap_or_default().cmp(&b.parse::<i32>().unwrap_or_default()));
        } else {
            PV.sort();
        }
    }

    display(option_u);
    println!("");
    Ok(0)
}

fn main() {
    if let Err(e) = sort_main() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}