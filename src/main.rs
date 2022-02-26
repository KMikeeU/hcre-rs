use std::io::{self, BufRead};
use std::collections::HashSet;
use std::env;
use std::fs;

mod rule;
mod string_reader;
use rule::Rule;
use string_reader::StringReader;

fn main() {
    // Compiling all the rules
    let mut rule_paths: Vec<String> = Vec::new();
    
    let args: Vec<String> = env::args().collect();
    let mut i: usize = 1;
    loop {
        if i >= args.len() {
            break;
        }
        match args[i].as_ref() {
            "-r" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("Missing rule location. Usage: -r <path to rule file>.");
                    std::process::exit(1);
                }

                rule_paths.push(args[i].clone());
            },
            _ => {
                eprintln!("No such argument {}", args[i]);
                std::process::exit(1);
            }
        }
        i += 1;
    }

    if rule_paths.len() == 0 {
        eprintln!("No rules specified, use -r <path to rule file>.");
        std::process::exit(1);
    }

    let mut rule_string = String::new();
    
    for rule_path in rule_paths {
        let contents = fs::read_to_string(&rule_path);
        match contents {
            Ok(content) => { rule_string.push_str(&content); rule_string.push('\n'); },
            Err(_) => { eprintln!("Could not read file {}", rule_path) }
        }
    }

    let mut rule_reader = StringReader::from_string(&rule_string);

    // All rules, line by line
    let mut rules: Vec<Vec<Rule>> = Vec::new();

    // The current rule / current line
    let mut mangler: Vec<Rule> = Vec::new();
    loop {
        // Do we have another char to read?
        match rule_reader.read() {
            Ok(c) => {
                // Match rules
                let result = match c {
                    ':' => { Some(Rule::Nothing) },
                    '$' => { Some(Rule::Append(rule_reader.read().unwrap())) },
                    '^' => { Some(Rule::Prepend(rule_reader.read().unwrap())) },
                    's' => { Some(Rule::Substitute(rule_reader.read().unwrap(), rule_reader.read().unwrap())) },
                    'l' => { Some(Rule::Lowercase()) },
                    'u' => { Some(Rule::Uppercase()) },
                    'c' => { Some(Rule::Capitalize()) },
                    'C' => { Some(Rule::InvertCapitalize()) },
                    'r' => { Some(Rule::Reverse()) },
                    'd' => { Some(Rule::Duplicate()) },
                    '@' => { Some(Rule::Purge(rule_reader.read().unwrap())) },
                    '\n' => { rules.push(mangler); mangler = Vec::new(); None },
                    '\r' | ' ' | '\t' => { None },
                    _ => { eprintln!("Rule {} not implemented!", c); None }
                };
                match result {
                    Some(rule) => { mangler.push(rule) },
                    None => {}
                }
            },
            Err(_) => {
                // We must be at the end of the string, add the latest mangler to the list, this is
                // causing some shit
                rules.push(mangler);
                break;
            }
        }
    }

    // Applying compiled rules to stdin
    let stdin = io::stdin();

    // For every line in the input
    for line in stdin.lock().lines() {
        // For every rule / every line in the rule file
        let line = line.unwrap();
        let mut line_out = HashSet::new();

        for rule in &rules {
            let mut out = String::from(line.clone());
            // For every operation in the rule
            for mangler in rule {
                match mangler {
                    Rule::Append(c) => { out.push(*c) }, 
                    Rule::Prepend(c) => {
                        let mut temp = String::new();
                        temp.push(*c);
                        temp.push_str(&out);
                        out = temp;
                    },
                    Rule::Substitute(c, o) => { out = out.replace(*c, &o.to_string().to_owned()[..]) }
                    Rule::Lowercase() => { out = out.to_lowercase() }
                    Rule::Uppercase() => { out = out.to_uppercase() }
                    Rule::Capitalize() => {
                        // Thank you stackoverflow!
                        let mut temp: Vec<char> = out.chars().collect();
                        temp[0] = temp[0].to_uppercase().nth(0).unwrap();
                        out = temp.into_iter().collect();
                    },
                    Rule::InvertCapitalize() => {
                        let mut temp: Vec<char> = out.to_uppercase().chars().collect();
                        temp[0] = temp[0].to_lowercase().nth(0).unwrap();
                        out = temp.into_iter().collect();
                    },
                    Rule::Duplicate() => {
                        let tmp = out.clone();
                        out.push_str(&tmp);
                    },
                    Rule::Reverse() => {
                        out = out.chars().rev().collect::<String>();
                    },
                    Rule::Purge(c) => {
                        out = out.replace(*c, "");
                    }
                    Rule::Nothing => { }
                };

            }
            line_out.insert(out);
        }
        for o in &line_out {
            println!("{}", o);
        }
    }
}

