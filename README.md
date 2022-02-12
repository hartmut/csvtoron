# csvtoron

Project to automatically convert csv files to ron. Inspired by the project toml_to_ron.

For usage take a look into the example directory or just call

```rust
to_ron(filename)
```

in the library. The return value is

```rust
Result<(), String>
```

The Result Strings only contains whether the creation of the ron file has been successfull. The library currently automatically writes a file in the same directory and filename as the source but with the extension .ron instead of .csv .  

For issues and problems please go to [github](https://github.com/hartmut/csvtoron).

## Example

```rust
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
```

## Input

- a header line is needed
- the program determines the seperator, in later versions it will be possible to define it

## Output

The the cells will be interpreted as f64, u64 and string and put into the output file acordingly.

The Output format is as follows:

```ron
(
    content: [
        { 
            "head of column 1": "String value of line 1 column 1",
            "head of column 2": 0.426, //f64
            "head of column 3": "String Value",
            "head of column 4": 713, //u64
        }
        {
            content of second line
        }
        ..
    ]
)
```

## Version 1

- currently the field name is serialized as a string but not  as a structure element name, so it is more difficult to automatically consume one ron element/csv line into a record
- create documentation
- test example with fixed field sizes
- writing tests for the variants - fixed cell, comma and semicolon separated lists
- after transformation of one line the output should be deseriazable as a structure, this means no quotation marks on the name of structure elements

## Version 2

- modify the output so that the cell header will be written without a hyphen so that you can import a record as a struct.
- allow json output, then this project needs to be migrated to anonther project
