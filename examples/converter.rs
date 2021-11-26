use std::env;
use csvtoron::to_ron;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err("Usage is : ./csvtoron <filename>".to_string());
    }
    let filename = args.get(1).unwrap();
    to_ron(filename)
}