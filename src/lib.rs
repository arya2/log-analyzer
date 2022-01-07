pub use clap::Parser as ArgParser;
use nom::IResult;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Counts the number of objects of each type that are in the log, and the total byte size of all messages for each type in a log of JSON objects
#[derive(ArgParser, Debug)]
#[clap(about)]
pub struct Args {
    /// Path to input file
    pub path_in: String,
}

pub fn parse_type_name(line: &String) -> IResult<&str, &str> {
    const TYPE_FIELD_NAME: &'static str = "\"type\"";

    use nom::{
        bytes::complete::{escaped, tag, take_until},
        character::complete::{alphanumeric1, char, multispace0 as space0, one_of},
        sequence::{delimited, preceded, tuple},
    };

    preceded(
        tuple((
            take_until(TYPE_FIELD_NAME),
            tag(TYPE_FIELD_NAME),
            delimited(space0, char(':'), space0),
        )),
        delimited(
            char('"'),
            escaped(alphanumeric1, '\\', one_of("\"n\\")),
            char('"'),
        ),
    )(line)
}

pub struct TableOutput {
    pub results: Vec<(String, usize, usize)>,
}

impl Display for TableOutput {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        use comfy_table::{presets::UTF8_FULL, Cell, CellAlignment, Table};
        let mut table = Table::new();
        table.set_header(["Type", "Number of Instances", "Total Byte Size"]);
        for row in &self.results {
            let row = [&row.0, &format!("{}", row.1), &format!("{}", row.2)];
            table.add_row(row.map(|s| Cell::new(s).set_alignment(CellAlignment::Center)));
        }
        write!(formatter, "{}", table.load_preset(UTF8_FULL))
    }
}
