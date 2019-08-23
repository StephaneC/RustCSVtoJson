# RustCSVtoJson
A simple Rust cli to transform CSV to json. 

## use

`./target/release/RustCsvToJson -i test2.csv`
`./target/release/RustCsvToJson -i test.csv --limiter ,`

### Cli params
* `-i`, `--input`: input csv filename. *required*
* `-o`, `--output`: output csv filename
* `-l`, `--limiter`: csv limiter. Default *;*

## TODO
* manage empty value
* manage output filename depending on input filename
* manage errors

