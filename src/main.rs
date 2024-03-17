use clap::Parser;
use crossterm::style::Stylize;
use ctrlc;
use inquire::CustomType;
use regex::Regex;
use std::io::Write;

#[derive(Parser)]
#[command(name = "regexp")]
#[command(version = "0.1")]
#[command(about="a commandline regular expression tester", long_about=None)]
#[command(arg_required_else_help(true))]
struct Cli {
    /// Regular expression
    regexp: Option<String>,

    /// Display captured groups
    #[arg(short, long, default_value_t = false)]
    capture: bool,
}

fn main() {
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
    })
    .expect("Error setting Ctrl-C handler");

    let cli = Cli::parse();

    let regexp = cli
        .regexp
        .as_deref()
        .expect("clap configured to show help if arguments absent");

    let cap = cli.capture;

    let re = match Regex::new(regexp) {
        Ok(re) => re,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    loop {
        let test_string = CustomType::<String>::new("Enter a test string:")
            .with_help_message("CTRL-C to exit")
            .prompt();

        match test_string.as_deref() {
            Ok(test_string) => match_regexp(re.clone(), test_string, cap),
            Err(_) => {
                std::io::stdout().flush().unwrap();
                println!("\n");
                return;
            }
        }
    }
}

/// Matches regular expression, printing info to stdout
fn match_regexp(re: Regex, test_string: &str, cap: bool) {
    let count = re.find_iter(test_string).count();
    if count > 0 {
        if count == 1 {
            println!("  {} Found {} match:", "✓".green(), count);
        } else {
            println!("  {} Found {} matches:", "✓".green(), count);
        }
        for captures in re.captures_iter(test_string) {
            println!(
                "      {}",
                test_string.replace(&captures[0], &format!("{}", &captures[0].bold().green()))
            );
            if cap {
                for (j, capture) in captures.iter().skip(1).enumerate() {
                    match capture {
                        Some(capture) => {
                            println!("        {}) {}", j + 1, capture.as_str());
                        }
                        None => (),
                    }
                }
            }
        }
    } else {
        println!("  {} Found {} matches", "✗".red(), count);
    }
}
