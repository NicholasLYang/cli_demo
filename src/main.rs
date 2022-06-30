use chrono::{DateTime, Utc};
use clap::{arg, command};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Bond {
    cusip: String,
    date_created: DateTime<Utc>,
    price: u32,
}

fn main() -> Result<(), anyhow::Error> {
    let matches = command!()
        .arg(arg!([input_file_name] "input file").required(true))
        .arg(arg!([output_file_name] "output file").required(true))
        .get_matches();

    let input_file_name = matches.get_one::<String>("input_file_name").unwrap();
    let output_file_name = matches.get_one::<String>("output_file_name").unwrap();

    let mut reader = csv::Reader::from_path(input_file_name)?;
    let mut writer = csv::Writer::from_path(output_file_name)?;

    for line in reader.deserialize() {
        let bond: Bond = line?;
        println!("{:?}", bond);
        writer.serialize(bond)?;
    }

    Ok(())
}
