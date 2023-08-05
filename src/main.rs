use std::fmt::Debug;
use std::io::{self, BufRead};
use std::collections::HashSet;
use std::env;
use std::fs;

mod rule;
mod string_reader;
use rule::Rule;
use string_reader::StringReader;

fn main() {
    let mut debug = false;

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
            "-h" => {
                eprintln!("Usage: {} [options]", args[0]);
                eprintln!("Options:");
                eprintln!("  -r <path to rule file>    Specify a rule file to use.");
                eprintln!("  -h                        Display this help message.");
                std::process::exit(0);
            },
            "-d" => {
                eprintln!("Debug mode enabled.");
                debug = true;
            },
            _ => {
                eprintln!("No such argument {}", args[i]);
                std::process::exit(1);
            }
        }
        i += 1;
    }

    if rule_paths.is_empty() {
        eprintln!("No rules specified, use -r <path to rule file>.");

        eprintln!("Usage: {} [options]", args[0]);
        eprintln!("Options:");
        eprintln!("  -r <path to rule file>    Specify a rule file to use.");
        eprintln!("  -h                        Display this help message.");
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
                let result: Option<Rule> = match c {
                    ':' => { Some(Rule::Nothing) },
                    '$' => {
                        match rule_reader.read() {
                            Ok(c) => { Some(Rule::Append(c)) },
                            _ => None
                        }
                    },
                    '^' => {
                        match rule_reader.read() {
                            Ok(c) => Some(Rule::Prepend(c)),
                            _ => None
                        }
                    },
                    's' => {
                        match rule_reader.read() {
                            Ok(c1) => {
                                match rule_reader.read() {
                                    Ok(c2) => Some(Rule::Replace(c1, c2)),
                                    _ => None
                                }
                            },
                            _ => {let _ = rule_reader.read(); None}
                        }
                    },
                    'l' => { Some(Rule::Lowercase()) },
                    'u' => { Some(Rule::Uppercase()) },
                    'c' => { Some(Rule::Capitalize()) },
                    'C' => { Some(Rule::InvertCapitalize()) },
                    'r' => { Some(Rule::Reverse()) },
                    'd' => { Some(Rule::Duplicate()) },
                    '@' => {
                        match rule_reader.read() {
                            Ok(c) => Some(Rule::Purge(c)),
                            _ => None
                        }
                    },
                    '{' => { Some(Rule::RotateLeft()) },
                    '}' => { Some(Rule::RotateRight()) },
                    't' => { Some(Rule::ToggleCase()) },
                    'T' => {
                        match rule_reader.read_usize() {
                            Ok(c) => Some(Rule::ToggleAt(c)),
                            _ => None
                        }
                    },
                    '[' => { Some(Rule::TruncateLeft()) },
                    ']' => { Some(Rule::TruncateRight()) },
                    'p' => {
                        match rule_reader.read_usize() {
                            Ok(c) => Some(Rule::DuplicateN(c)),
                            _ => None
                        }
                    },
                    '\'' => {
                        match rule_reader.read_usize() {
                            Ok(c) => Some(Rule::TruncateAt(c)),
                            _ => None
                        }
                    },
                    'D' =>  {
                        match rule_reader.read_usize() {
                            Ok(c) => Some(Rule::DeleteAt(c)),
                            _ => None
                        }
                    },
                    'O' => {
                        match rule_reader.read_usize() {
                            Ok(c1) => {
                                match rule_reader.read_usize() {
                                    Ok(c2) => Some(Rule::OmitRange(c1, c2)),
                                    _ => None
                                }
                            },
                            _ => {let _ = rule_reader.read(); None}
                        }
                    },
                    'f' => { Some(Rule::Reflect()) },
                    'o' => {
                        match rule_reader.read_usize() {
                            Ok(c1) => {
                                match rule_reader.read() {
                                    Ok(c2) => Some(Rule::OverwriteAt(c1, c2)),
                                    _ => None
                                }
                            },
                            _ => {let _ = rule_reader.read(); None}
                        }
                    },
                    'z' => {
                        match rule_reader.read_usize() {
                            Ok(c) => Some(Rule::DuplicateFirstN(c)),
                            _ => None
                        }
                    },
                    'Z' => {
                        match rule_reader.read_usize() {
                            Ok(c) => Some(Rule::DuplicateLastN(c)),
                            _ => None
                        }
                    },
                    'i' => {
                        match rule_reader.read_usize() {
                            Ok(c1) => {
                                match rule_reader.read() {
                                    Ok(c2) => Some(Rule::InsertAt(c1, c2)),
                                    _ => None
                                }
                            },
                            _ => {let _ = rule_reader.read(); None}
                        }
                    },
                    'q' => {
                        Some(Rule::DuplicateAll())
                    },
                    'x' => {
                        match rule_reader.read_usize() {
                            Ok(c1) => {
                                match rule_reader.read_usize() {
                                    Ok(c2) => Some(Rule::ExtractRange(c1, c2)),
                                    _ => None
                                }
                            },
                            _ => {let _ = rule_reader.read(); None}
                        }
                    }

                    // These rules have not been implemented, but will be handled
                    // gracefully, so that other rules are not affected
                    'E' => { if debug { eprintln!("{} Rule E (Title) not implemented! (handled gracefully)", rules.len() + 1); } None },
                    'k' => { if debug { eprintln!("{} Rule k (Swap first two) not implemented! (handled gracefully)", rules.len() + 1); } None },
                    'K' => { if debug { eprintln!("{} Rule K (Swap last two) not implemented! (handled gracefully)", rules.len() + 1); } None },
                    'y' => { if debug { eprintln!("{} Rule y (Duplicate block front) not implemented! (handled gracefully)", rules.len() + 1); } let _ = rule_reader.read(); None },
                    'Y' => { if debug { eprintln!("{} Rule Y (Duplicate block back) not implemented! (handled gracefully)", rules.len() + 1); } let _ = rule_reader.read(); None },
                    '.' => { if debug { eprintln!("{} Rule . (Replace N + 1) not implemented! (handled gracefully)", rules.len() + 1); } let _ = rule_reader.read(); None },
                    ',' => { if debug { eprintln!("{} Rule , (Replace N - 1) not implemented! (handled gracefully)", rules.len() + 1); } let _ = rule_reader.read(); None },
                    '-' => { if debug { eprintln!("{} Rule - (Ascii Decrement) not implemented! (handled gracefully)", rules.len() + 1); } let _ = rule_reader.read(); None },
                    '+' => { if debug { eprintln!("{} Rule + (Ascii Increment) not implemented! (handled gracefully)", rules.len() + 1); } let _ = rule_reader.read(); None },
                    'L' => { if debug { eprintln!("{} Rule L (Bitwise shift left) not implemented! (handled gracefully)", rules.len() + 1); } let _ = rule_reader.read(); None },
                    'R' => { if debug { eprintln!("{} Rule R (Bitwise shift right) not implemented! (handled gracefully)", rules.len() + 1); } let _ = rule_reader.read(); None },
                    'e' => { if debug { eprintln!("{} Rule e (Title w/separator) not implemented! (handled gracefully)", rules.len() + 1); } let _ = rule_reader.read(); None },
                    '*' => { if debug { eprintln!("{} Rule * (Swap @ N) not implemented! (handled gracefully)", rules.len() + 1); } let _ = rule_reader.read(); let _ = rule_reader.read(); None },
                    '3' => { if debug { eprintln!("{} Rule 3 (Toggle w/Nth separator) not implemented! (handled gracefully)", rules.len() + 1); } let _ = rule_reader.read(); let _ = rule_reader.read(); None },
                    
                    // Line done
                    '\n' => { rules.push(mangler); mangler = Vec::new(); None },
                    '\r' | ' ' | '\t' => { None },
                    '#' if mangler.is_empty() => { let _ = rule_reader.skip_line(); None },
                    
                    // This rule is totally unknown and may cause issues
                    // because it's potential arguments could be interpreted as
                    // other rules
                    _ => {
                        if debug {
                            eprintln!("[line {}] Unknown rule {} not implemented! This may cause other rules to fail", rules.len()+1, c);
                        }
                        None
                    }
                };
                
                if let Some(rule) = result {
                    match rule {
                        Rule::Invalid(c) => {
                            if debug {
                                eprintln!("[line {}] Invalid rule {}", rules.len()+1, c);
                            }
                        },
                        _ => { mangler.push(rule); }
                    } 
                }

                // eprintln!("[line {}]", rules.len());
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
    // For every line in the input
    for line in io::stdin().lock().lines() {
        // For every rule / every line in the rule file
        let line = line.unwrap();
        // Required on windows cmd, not implemented
        // let line = line.trim_end();

        let mut line_out = HashSet::new();

        for rule in &rules {
            let mut out = line.clone();
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
                    Rule::Replace(c, o) => { out = out.replace(*c, &o.to_string().to_owned()[..]) }
                    Rule::Lowercase() => { out = out.to_lowercase() }
                    Rule::Uppercase() => { out = out.to_uppercase() }
                    Rule::Capitalize() => {
                        // Thank you stackoverflow!
                        let mut temp: Vec<char> = out.chars().collect();
                        temp[0] = temp[0].to_uppercase().next().unwrap();
                        out = temp.into_iter().collect();
                    },
                    Rule::InvertCapitalize() => {
                        let mut temp: Vec<char> = out.to_uppercase().chars().collect();
                        temp[0] = temp[0].to_lowercase().next().unwrap();
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
                    },
                    Rule::RotateLeft() => {
                        out = out.chars().skip(1).chain(out.chars().take(1)).collect();
                    },
                    Rule::RotateRight() => {
                        out = out.chars().rev().take(1).chain(out.chars().take(out.chars().count() - 1)).collect();
                    }
                    Rule::Nothing => { }
                    Rule::ToggleCase() => {
                        out = out.chars().map(|c| {
                            if c.is_lowercase() {
                                c.to_uppercase().next().unwrap()
                            } else {
                                c.to_lowercase().next().unwrap()
                            }
                        }).collect();
                    },
                    Rule::ToggleAt(pos) => {
                        out = out.chars().enumerate().map(|(i, c)| {
                            if i == *pos {
                                if c.is_lowercase() {
                                    c.to_uppercase().next().unwrap()
                                } else {
                                    c.to_lowercase().next().unwrap()
                                }
                            } else {
                                c
                            }
                        }).collect();
                    },
                    Rule::DuplicateN(n) => {
                        let mut tmp = String::new();
                        for _ in 0..*n {
                            tmp.push_str(&out);
                        }
                        out = tmp;
                    },
                    Rule::Reflect() => {
                        let mut tmp = out.clone();
                        tmp.push_str(&out.chars().rev().collect::<String>());
                        out = tmp;
                    },
                    Rule::TruncateLeft() => {
                        out = out.chars().skip(1).collect();
                    },
                    Rule::TruncateRight() => {
                        out = out.chars().take(out.chars().count() - 1).collect();
                    },
                    Rule::DeleteAt(pos) => {
                        out = out.chars().enumerate().filter(|(i, _)| i != pos).map(|(_, c)| c).collect();
                    },
                    Rule::ExtractRange(pos, count) => {
                        out = out.chars().enumerate().filter(|(i, _)| i >= pos && i < &(pos + count)).map(|(_, c)| c).collect();
                    },
                    Rule::OmitRange(pos, count) => {
                        out = out.chars().enumerate().filter(|(i, _)| i < pos || i >= &(pos + count)).map(|(_, c)| c).collect();
                    },
                    Rule::InsertAt(pos, c) => {
                        let mut tmp = String::new();
                        for (i, ch) in out.chars().enumerate() {
                            if i == *pos {
                                tmp.push(*c);
                            }
                            tmp.push(ch);
                        }
                        out = tmp;
                    },
                    Rule::OverwriteAt(pos, c) => {
                        let mut tmp = String::new();
                        for (i, ch) in out.chars().enumerate() {
                            if i == *pos {
                                tmp.push(*c);
                            } else {
                                tmp.push(ch);
                            }
                        }
                        out = tmp;
                    },
                    Rule::TruncateAt(pos) => {
                        out = out.chars().enumerate().filter(|(i, _)| i < pos).map(|(_, c)| c).collect();
                    },
                    Rule::DuplicateFirstN(n) => {
                        let mut tmp = String::new();
                        for _ in 0..*n {
                            tmp.push(out.chars().next().unwrap());
                        }
                        tmp.push_str(&out);
                        out = tmp;
                    },
                    Rule::DuplicateLastN(n) => {
                        let mut tmp = String::new();
                        tmp.push_str(&out);
                        for _ in 0..*n {
                            tmp.push(out.chars().last().unwrap());
                        }
                        out = tmp;
                    },
                    Rule::DuplicateAll() => {
                        let mut tmp = String::new();
                        for c in out.chars() {
                            tmp.push(c);
                            tmp.push(c);
                        }
                        out = tmp;
                    },
                    Rule::Invalid(_) => todo!(),
                };

            }
            line_out.insert(out);
        }
        for o in &line_out {
            println!("{}", o);
        }
    }
}

