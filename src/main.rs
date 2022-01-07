pub use clap::Parser as ArgParser;
use log_analyzer::{parse_type_name, Args, TableOutput};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};
fn main() {
    let args = Args::parse();
    let file = File::open(&args.path_in).expect("Failed to find and read input file");
    let reader = BufReader::new(file).lines();
    let mut type_data: HashMap<String, (usize, usize)> = HashMap::new();
    let mut upsert_type_data = |type_name, msg_byte_size| {
        let updated_value = type_data
            .get(&type_name)
            .map_or((1, msg_byte_size), |p| (p.0 + 1, p.1 + msg_byte_size));
        type_data.insert(type_name, updated_value);
    };

    let mut warnings: Vec<usize> = vec![];
    let mut line_num = 0;

    for line in reader {
        line_num += 1;

        if let Ok(line) = line {
            let byte_size = line.len();
            match parse_type_name(&line) {
                Ok((_, type_name)) => upsert_type_data(type_name.into(), byte_size),
                Err(_) => {
                    warnings.push(line_num);
                    continue;
                }
            }
        }
    }

    if warnings.len() > 0 {
        println!(
            "\n  ({}) Warning: ParseError on line(s): {:?}\n",
            warnings.len(),
            warnings
        );
    }

    let mut results: Vec<_> = type_data
        .into_iter()
        .map(|(name, (num_instances, total_byte_size))| (name, num_instances, total_byte_size))
        .collect();

    results.sort_unstable_by(|&(_, _, ref s1), &(_, _, s2)| s2.cmp(s1));

    println!("{}", TableOutput { results });
}
