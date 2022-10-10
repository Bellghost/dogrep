use std::collections::HashSet;
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub file_path: String, // File path
    pub pattern: String,   // Matched pattern
    pub ignore_case: bool, // Ignore case
    pub reversed: bool,    // Reverse matching
    pub line_number: bool, // Gives the line number
    pub count: bool,       // Count the number of lines which matching the pattern
}

impl Config {
    pub fn build(mut args: std::env::Args) -> Result<Config, &'static str> {
        // At least three parameters:(*.exe pattern filename)
        if args.len() < 3 {
            return Err("Not enough parameters.");
        }
        args.next(); // Skip the first parameter.
        let mut args = args.rev();

        // Must provide filepath and pattern.
        let file_path = args.next().ok_or("No file path provided")?;
        let pattern = args.next().ok_or("No search pattern provided")?;

        // Other parameters
        let other_parameters: HashSet<_> = args.collect();

        Ok(Config {
            file_path,
            pattern,
            ignore_case: other_parameters.contains("-i"),
            reversed: other_parameters.contains("-v"),
            line_number: other_parameters.contains("-n"),
            count: other_parameters.contains("-c"),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_path)?; // Read contents from file
    if config.count {
        // Count mode
        let total = count_mode(config, &content);
        println!("There is(are) {total} line(s) in the file that match the pattern.")
    } else {
        // Search mode
        let res = search_mode(config, &content);
        for line in res.iter() {
            println!("{line}");
        }
    }
    Ok(())
}

fn matcher<'a>(config: &'a Config, content: &'a str) -> impl Iterator<Item = (usize, &'a str)> {
    content.lines().enumerate().filter(|item| {
        let line: &str = item.1;
        let flag;
        if config.ignore_case {
            // Ignore case
            let pattern = config.pattern.to_lowercase();
            flag = line.to_lowercase().contains(&pattern);
        } else {
            // Case insensitive
            flag = line.contains(&config.pattern);
        }
        if config.reversed {
            !flag
        } else {
            flag
        }
    })
}

fn count_mode(config: Config, content: &str) -> usize {
    matcher(&config, content).count()
}

fn search_mode(config: Config, content: &str) -> Vec<String> {
    if config.line_number {
        matcher(&config, content)
            .map(|(index, line)| (index + 1).to_string() + " : " + line)
            .collect()
    } else {
        matcher(&config, content)
            .map(|(_, line)| String::from(line))
            .collect()
    }
}