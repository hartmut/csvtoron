extern crate csv;

use csv::{ReaderBuilder, StringRecord, Trim};
use ron::ser::{to_writer_pretty, PrettyConfig};
use serde::Serialize;
use std::collections::HashMap;
use std::format;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read};

fn get_file_content(filename: &str) -> Result<String, String> {
    let file = File::open(filename);
    if file.is_err() {
        return Err(format!("Could not open file {}", filename));
    }
    let mut buffer = Vec::new();
    // read the whole file
    let f = BufReader::new(file.unwrap()).read_to_end(&mut buffer);
    if f.is_err() {
        return Err(format!("Failed to read the file {}", filename));
    }
    match std::str::from_utf8(&*buffer) {
        Ok(v) => Ok(v.to_owned()),
        Err(e) => Err(format!("Invalid UTF-8 sequence: {}", e)),
    }
}

#[derive(Debug)]
struct Csvinput {
    pub header: StringRecord,
    pub records: Vec<StringRecord>,
}

// load csv file and export to Vec
// delimiter is determinde automatically
fn csvreader(content: &str) -> Csvinput {
    // create csv reader to interpret input string
    let mut rdr = ReaderBuilder::new()
        .trim(Trim::All)
        .from_reader(content.as_bytes());

    // extract headers
    let header = rdr.headers().unwrap().clone();

    // extract list of lines/records
    let records = rdr
        .records()
        .collect::<Result<Vec<StringRecord>, csv::Error>>()
        .unwrap();

    // Output
    Csvinput { header, records }
}

// create writer from for the ron output file
fn create_ron_file(filename: &str) -> Result<BufWriter<File>, String> {
    let ron_filename = filename.split('.').next();
    if ron_filename.is_none() {
        return Err(format!("Failed to get the ron filename {}", filename));
    }
    let ron_filename = ron_filename.unwrap().to_owned() + ".ron";

    let ron_file = File::create(&ron_filename);
    if ron_file.is_err() {
        return Err(format!("Failed to create the ron file {}", ron_filename));
    }
    Ok(BufWriter::new(ron_file.unwrap()))
}

// definition of content of a cell
#[derive(Debug, Serialize)]
#[serde(untagged)]
enum OutType {
    Str(String),
    U64(u64),
    F64(f64),
}

// default implementation of cell content
impl Default for OutType {
    fn default() -> Self {
        OutType::Str("".to_string())
    }
}

// TODO serialize so that the field name is not written as String but as a structure element name
// one line of a csv file as a Hashmap
type OutRecord = HashMap<String, OutType>;

// whole content of a csv file which can be written as a ron file
#[derive(Default, Debug, Serialize)]
struct Ronfile {
    content: Vec<OutRecord>,
}

// determine content of a cell (f64, u64 or String)
fn matcher(element: String) -> OutType {
    if let Ok(output) = element.parse::<f64>() {
        OutType::F64(output)
    } else if let Ok(output) = element.parse::<u64>() {
        OutType::U64(output)
    } else {
        OutType::Str(element.to_string())
    }
}

// ron converter
fn convert(csv: Csvinput) -> Ronfile {
    let mut res = Ronfile::default();
    for record in csv.records {
        let mut field = csv.header.iter();
        let mut outrecord = OutRecord::default();
        for element in record.iter() {
            outrecord.insert(
                field.next().unwrap().to_string(),
                matcher(element.to_string()),
            );
        }
        res.content.push(outrecord);
    }
    res
}

/// Converts a csv file to a ron file. The csv filename will not be touched
/// but a new file with the file type .ron will be created in the same directory
/// as the csv file.
///
/// # Example Code
///
/// ```
/// use std::env;
/// use csvtoron::to_ron;
///
/// fn main() -> Result<(), String> {
///    to_ron("this_is_a_csv_file.csv")
/// }
/// ```
/// Output will be in the file **this_is_a_csv_file.ron**

pub fn to_ron(filename: &str) -> Result<(), String> {
    let content = get_file_content(filename)?;
    let doc = csvreader(&content);
    let converted = convert(doc);
    let ron_buffer = create_ron_file(filename)?;
    let pretty = PrettyConfig::new()
        .with_separate_tuple_members(true)
        .with_enumerate_arrays(true);
    let r = to_writer_pretty(ron_buffer, &converted, pretty);
    if r.is_err() {
        return Err(format!("Serialization failed for {:?}", &converted));
    }
    Ok(())
}
