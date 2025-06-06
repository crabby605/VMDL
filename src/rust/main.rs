use std::env;
use std::fs;
use std::path::Path;
use std::process;
use vmdl_rs::{parse, to_json, to_text};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file_path = "examples/config.vmdl";
    let mut format = "json";

    // simple command line argument parsing
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-f" | "--format" => {
                if i + 1 < args.len() {
                    format = &args[i + 1];
                    i += 2;
                } else {
                    eprintln!("Error: Format option requires a value");
                    process::exit(1);
                }
            },
            path if !path.starts_with("-") => {
                file_path = path;
                i += 1;
            },
            _ => {
                eprintln!("Unknown option: {}", args[i]);
                i += 1;
            }
        }
    }

    println!("Parsing VMDL file: {}", file_path);
    
    if !Path::new(file_path).exists() {
        eprintln!("File does not exist: {}", file_path);
        process::exit(1);
    }

    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            process::exit(1);
        }
    };

    match parse(&content) {
        Ok(data) => {
            println!("\nParsed data as {}:", format);
            
            //output in the requested format
            match format {
                "json" => println!("{}", to_json(&data)),
                "text" => println!("{}", to_text(&data, 0)),
                _ => {
                    eprintln!("Unsupported format: {}. Using JSON instead.", format);
                    println!("{}", to_json(&data));
                }
            }
            
            //example of accessing data
            println!("\nAccessing specific values:");
            if let Some(project) = data.get_string("Project") {
                println!("Project: {}", project);
            }
            
            if let Some(route) = data.get_string("Route") {
                println!("Route: {}", route);
            }
            
            if let Some(envs) = data.get_object("Environments") {
                if let Some(staging) = envs.get("Staging") {
                    if let Some(staging_route) = staging.get_string("Route") {
                        println!("Staging Route: {}", staging_route);
                    }
                }
            }
        },
        Err(e) => {
            eprintln!("Error parsing VMDL: {}", e);
            process::exit(1);
        }
    }
}
