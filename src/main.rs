use clap::{ArgAction, Parser};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::exit;

#[derive(Parser)]
#[command(name = "fsort")]
#[command(author = "TechHara")]
#[command(version = "0.1")]
#[command(about = "Sort fields within each line", long_about = None)]
struct Cli {
    /// Field delimiter character
    #[arg(short, long, default_value_t = '\t')]
    delim: char,
    /// Separate fields by whitespace.
    /// Specify output delimiter with -d option
    #[arg(short, long, action = ArgAction::SetTrue)]
    white_space: bool,
    /// Fold to upper case when comparing
    #[arg(short, long, action = ArgAction::SetTrue)]
    fold_case: bool,
    /// TODO: Compare according to string numerical value
    #[arg(short, long, action = ArgAction::SetTrue)]
    numeric: bool,
    /// Reverse the result of comparisons
    #[arg(short, long, action = ArgAction::SetTrue)]
    reverse: bool,
    /// Check each line is sorted
    #[arg(short, long, action = ArgAction::SetTrue)]
    check: bool,
    /// Input file
    input: Option<String>,
    /// Output file
    output: Option<String>,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let input_file = match cli.input.is_some() && cli.input != Some("-".to_string()) {
        true => cli.input.unwrap(),
        false => "/dev/stdin".to_string(),
    };
    let output_file = match cli.output.is_some() && cli.output != Some("-".to_string()) {
        true => cli.output.unwrap(),
        false => "/dev/stdout".to_string(),
    };

    let ifs = BufReader::new(File::open(input_file)?);
    let mut ofs = BufWriter::new(File::create(output_file)?);

    let compare = match cli.fold_case {
        false => |a: &&str, b: &&str| a.cmp(b),
        true => |a: &&str, b: &&str| a.to_uppercase().cmp(&b.to_uppercase()),
    };

    for (linenum, line) in ifs.lines().enumerate() {
        let linenum = linenum + 1;
        let line = line?;
        let mut fields: Vec<&str> = match cli.white_space {
            false => line.split(cli.delim).collect(),
            true => line.split_whitespace().collect(),
        };

        if cli.check {
            let sorted = match cli.reverse {
                false => is_sorted_by(fields, compare),
                true => is_sorted_by(fields, |a, b| compare(b,a)),
            };
            if !sorted {
                eprintln!("not sorted at line #{}", linenum);
                exit(255);
            }
        } else {
            match cli.reverse {
                false => fields.sort(),
                true => fields.sort_by(|a, b| compare(b, a)),
            }
    
            writeln!(ofs, "{}", fields.join(&cli.delim.to_string()))?;
        }
    }

    Ok(())
}

fn is_sorted_by<T, F>(xs: Vec<T>, f: F) -> bool
where
    T: std::cmp::Ord,
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    for idx in 1..xs.len() {
        if f(&xs[idx - 1], &xs[idx]) == std::cmp::Ordering::Greater {
            return false;
        }
    }
    true
}
