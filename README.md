# csvtoron

Project to automatically convert csv files to ron. Partialy bases on the project toml_to_ron.

For usage take a look into the example directory or just call

```rust
master(filename)
```

in the library. The return value is

```rust
Result<(), String>
```

For issues and problems please go to [github](https://github.com/hartmut/csvtoron).

## example

```rust
> use std::env;
use csvtoron::master;

> fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err("Usage is : ./csvtoron <filename>".to_string());
    }
    let filename = args.get(1).unwrap();
    master(filename)
}
```

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

* test example with fixed field sizes
* writing tests for the variants - fixed cell, comma and semicolon separated lists
* after transformation of one line the output should be deseriazable as a structure, this means no quotation marks on the name of structure elements

## Version 2

* modify the output so that the cell header will be written without a hyphen so that you can import a record as a struct.
* allow json output, then this project needs to be renamed :)
