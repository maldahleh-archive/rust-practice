use text_colorizer::*;

use std::env;
use std::fs;
use std::process;
use regex::Regex;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn main() {
    let args = parse_args();

    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            let error_message = format!("failed to read from file '{}': {:?}",
                                        args.filename,
                                        e);

            encountered_error(error_message);
        }
    };

    let replaced_data = match replace(&args.target, &args.replacement, &data) {
        Ok(v) => v,
        Err(e) => {
            let error_message = format!("failed to replace text: {:?}", e);

            encountered_error(error_message);
        }
    };

    match fs::write(&args.output, &replaced_data) {
        Ok(_) => {},
        Err(e) => {
            let error_message = format!("failed to write to file '{}': {:?}",
                                                args.filename,
                                                e);

            encountered_error(error_message);
        }
    }
}

fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_incorrect_args(args.len());
    }

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}

fn print_incorrect_args(received: usize) {
    let error_message = format!("wrong number of arguments: expected 4, got {}.", received);

    print_usage();
    encountered_error(error_message);
}

fn encountered_error(message: String) {
    eprintln!("{} {}",
              "Error:".red().bold(),
              message);
    process::exit(1);
}

fn print_usage() {
    eprintln!("{} - change occurrences of one string into another", "quickreplace".green());
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}
