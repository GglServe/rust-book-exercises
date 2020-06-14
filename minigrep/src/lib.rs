use std::fs;
use std::error::Error;

pub struct Config {
    search_str: String,
    filename: String,
    case_insensitive: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let content = fs::read_to_string(&config.filename)?;

    for line in search(&content, &config.search_str, config.case_insensitive).iter() {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(text: &'a str, substr: &str, case_insensitive: bool) -> Vec<&'a str> {
    let mut result_lines: Vec<&str> = Vec::new();

    if case_insensitive {
        for line in text.trim().lines() {
            if line.to_lowercase().contains(&substr.to_lowercase()) {
                result_lines.push(line)
            }
        }
    } else {
        for line in text.trim().lines() {
            if line.contains(substr) {
                result_lines.push(line)
            }
        }
    }

    result_lines
}


impl Config {
    pub fn new(args: Vec<String>, case_insensitive: bool) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("requires 2 arguments - search_string and filename\n");
        }

        let search_str: String = args[0].clone();
        let filename: String = args[1].trim().to_string();

        if search_str == "" {
            return Err("search string cannot be empty\n");
        } else if filename == "" {
            return Err("filename cannot be empty\n");
        }

        Ok(Config{search_str, filename, case_insensitive})
    }
}

#[cfg(test)]
mod test {
    use super::search;

    #[test]
    fn multiple_matches_case_sensitive() {
        let search_str = "you";
        let text = "\
I’m nobody! Who are you?
Are you nobody, too?
Then there’s a pair of us - don’t tell!
They’d banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!";
        let expected = "\
I’m nobody! Who are you?
Are you nobody, too?
They’d banish us, you know.
To tell your name the livelong day";

        assert_eq!(expected.lines().collect::<Vec<&str>>(), search(&text, &search_str, false));
    }

    #[test]
    fn no_match_case_sensitive() {
        let search_str = "abcde";
        let text = "\
I’m nobody! Who are you?
Are you nobody, too?
Then there’s a pair of us - don’t tell!
They’d banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!";
        let expected = "";

        assert_eq!(expected.lines().collect::<Vec<&str>>(), search(&text, &search_str, false));
    }

    #[test]
    fn case_insensitive() {
        let search_str = "To";
        let text = "\
I’m nobody! Who are you?
Are you nobody, too?
Then there’s a pair of us - don’t tell!
They’d banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!";
        let expected = "\
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!";

        assert_eq!(expected.lines().collect::<Vec<&str>>(), search(&text, &search_str, true));
    }
}

