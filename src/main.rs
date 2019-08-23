extern crate clap;

use clap::{Arg, App};
use std::fs::File;
use std::io::{Write, BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let matches = App::new("RustCsvToJson")
        .version("0.1.0")
        .author("Stéphane Castrec <stephane.castrec@gmail.com>")
        .about("transform csv file to json")
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .value_name("FILE")
            .help("Sets the input file to transform")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("FILE")
            .help("Sets the output file result")
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("limiter")
            .short("l")
            .long("limiter")
            .value_name("limiter")
            .help("Sets the csv delimiter")
            .required(false)
            .takes_value(true))
        .get_matches();
        
    let input = matches.value_of("input");
    if let Some(i) = matches.value_of("input") {
        println!("Input: {}", i);
    }

    let output = matches.value_of("output");
    let output_file;
    if let Some(o) = output {
        println!("Output: {}", o);
        output_file = String::from(o);
    } else {
        // TODO : get the input filename and change csv to json
        output_file = String::from("output.json")
    }

    let limiter = matches.value_of("limiter").unwrap_or(";");;
    println!("Limiter: {}", limiter);

    let f = File::open(input.unwrap())?;
    let buff = BufReader::new(f);

    // open a JSON array
    let mut result = String::from("[");

    // first read headers
    let mut cursor = buff.lines();
    let headers = cursor.next().unwrap().unwrap();
    let keys: Vec<&str> = headers.split(limiter).collect();

    let mut i = 0;
    for line in cursor { 
        if i > 0 {
            result.push_str(",");
        }  
        let line_unwrapped = line.unwrap();  
        let values: Vec<&str> = line_unwrapped.split(limiter).collect();
        // values
        //println!("values {}", line.unwrap());
        let line_res = transform_line(&keys, values);
        result.push_str(&line_res);
        i += 1;
    }
    // close the array
    result.push_str("]");
    println!("result {}", result);
    println!("nbObject {}", i);
    return write_file_result(result, output_file);
}

fn write_file_result(result: String, filename: String) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(result.as_bytes())?;
    Ok(())
}

fn transform_line(keys: &Vec<&str>, values: std::vec::Vec<&str>) -> String {
    let mut result = String::from("");
    let mut i = 0;
    result.push_str("{\"");
    for x in keys {
        if i > 0 {
            result.push_str(",\"");
        } 
        result.push_str(x);
        result.push_str("\":\"");
        result.push_str(values[i]);
        result.push_str("\"");
        i = i + 1;
    }
    result.push_str("}");
    return result;
}
