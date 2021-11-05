# csvtoron
Project to automatically convert csv files to ron. Partialy bases on the project toml_to_ron. 

Just call
> csvtoron <filename.csv>

The output file will be named
> filname.ron

# Output
The the cells will be interpreted as f64, u64 and string and put into the output file acordingly. 

The Output format is as follows:

> (
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

# Version 1
* rewrite to library and move main into examples
* test example with fixed field sizes
* writing tests for the variants - fixed cell, comma and semicolon separated lists
* after transformation of one line the output should be deseriazable as a structure, this means no quotation marks on the name of structure elements 

# Version 2 
* modify the output so that the cell header will be written without a hyphen so that you can import a record as a struct.
* allow json output, then this project needs to be renamed :)