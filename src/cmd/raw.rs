use csv;

use CliResult;
use config::{Config, Delimiter};
use std::io::{self, Write};
use util;

static USAGE: &'static str = "
Write each CSV record as a raw line of plaintext with no formatting.
Generally you do not want to do this but one example use case is when you
want to use the values of a single column as input to a non-csv plaintext
utility.

Usage:
    xsv raw [options] [<input>]

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
";

#[derive(Deserialize)]
struct Args {
    arg_input: Option<String>,
    flag_delimiter: Option<Delimiter>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(true);

    let mut rdr = rconfig.reader()?;
    let mut r = csv::ByteRecord::new();
    let mut stdout = io::stdout();
    while rdr.read_byte_record(&mut r)? {
        for field in r.iter() {
            stdout.write(field)?;
        }
        stdout.write(b"\n")?;
    }
    stdout.flush()?;
    Ok(())
}
