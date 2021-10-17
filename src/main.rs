extern crate csv;

use csv::{ReaderBuilder, StringRecord, Trim};
use ron::ser::{to_writer_pretty, PrettyConfig};
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read};
use std::{env, format};

pub fn get_file_content(filename: &str) -> Result<String, String> {
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
pub struct Csvinput {
    pub header: StringRecord,
    pub records: Vec<StringRecord>,
}

// load csv file and export to Vec
// TODO define delimeter in input
pub fn csvreader(content: &str) -> Csvinput {
    let mut rdr = ReaderBuilder::new()
        .trim(Trim::All)
        .from_reader(content.as_bytes());

    let header = rdr.headers().unwrap().clone();

    let records = rdr
        .records()
        .collect::<Result<Vec<StringRecord>, csv::Error>>()
        .unwrap();

    Csvinput { header, records }
}

pub fn create_ron_file(filename: &str) -> Result<BufWriter<File>, String> {
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

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum OutType {
    Str(String),
    U64(u64),
    F64(f64),
}

impl Default for OutType {
    fn default() -> Self {
        OutType::Str("".to_string())
    }
}

// TODO serialize so that the field name is not written as String but as a structure element name
type OutRecord = HashMap<String, OutType>;
type OutVec = Vec<OutRecord>;

#[derive(Default, Debug, Serialize)]
pub struct Ronfile {
    pub content: OutVec,
}

pub fn matcher(element: String) -> OutType {
    if let Ok(output) = element.parse::<f64>() {
        OutType::F64(output)
    } else if let Ok(output) = element.parse::<u64>() {
        OutType::U64(output)
    } else {
        OutType::Str(element.to_string())
    }
}

pub fn convert(csv: Csvinput) -> Ronfile {
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

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err("Usage is : ./csvtoron <filename>".to_string());
    }
    let filename = args.get(1).unwrap();
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
