//! This is a simple program to calculate a checksum from the command line

use crc_fast::{checksum, checksum_file, CrcAlgorithm};
use std::env;
use std::process::ExitCode;
use std::str::FromStr;

#[derive(Debug)]
struct Config {
    algorithm: String,
    file: Option<String>,
    string: Option<String>,
    format: OutputFormat,
}

#[derive(Debug, Clone)]
enum OutputFormat {
    Hex,
    Decimal,
}

fn print_usage() {
    println!("Usage: checksum -a algorithm [-f file] [-s string] [--format hex|decimal]");
    println!();
    println!("Example: checksum -a CRC-32/ISCSI -f myfile.txt");
    println!("Example: checksum -a CRC-64/NVME -s 'Hello, world!' --format decimal");
    println!();
    println!("Options:");
    println!("  -a algorithm        Specify the checksum algorithm (required)");
    println!("  -f file             Calculate checksum for the specified file");
    println!("  -s string           Calculate checksum for the specified string");
    println!("  --format hex|decimal Output format (default: hex)");
    println!("  -h, --help          Show this help message");
    println!();
    println!("Note: Either -f or -s must be provided, but not both.");
}

fn parse_args() -> Result<Config, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        return Err("No arguments provided".to_string());
    }

    // Check for help flag
    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        return Err("help".to_string());
    }

    let mut algorithm: Option<String> = None;
    let mut file: Option<String> = None;
    let mut string: Option<String> = None;
    let mut format = OutputFormat::Hex; // Default to hex

    let mut i = 1; // Skip program name
    while i < args.len() {
        match args[i].as_str() {
            "-a" => {
                if i + 1 >= args.len() {
                    return Err("Missing algorithm after -a flag".to_string());
                }
                algorithm = Some(args[i + 1].clone());
                i += 2;
            }
            "-f" => {
                if i + 1 >= args.len() {
                    return Err("Missing filename after -f flag".to_string());
                }
                if string.is_some() {
                    return Err("Cannot specify both -f and -s flags".to_string());
                }
                file = Some(args[i + 1].clone());
                i += 2;
            }
            "-s" => {
                if i + 1 >= args.len() {
                    return Err("Missing string after -s flag".to_string());
                }
                if file.is_some() {
                    return Err("Cannot specify both -f and -s flags".to_string());
                }
                string = Some(args[i + 1].clone());
                i += 2;
            }
            "--format" => {
                if i + 1 >= args.len() {
                    return Err("Missing format after --format flag".to_string());
                }
                match args[i + 1].as_str() {
                    "hex" => format = OutputFormat::Hex,
                    "decimal" => format = OutputFormat::Decimal,
                    invalid => {
                        return Err(format!(
                            "Invalid format '{}'. Use 'hex' or 'decimal'",
                            invalid
                        ))
                    }
                }
                i += 2;
            }
            arg => {
                return Err(format!("Unknown argument: {}", arg));
            }
        }
    }

    // Validate required arguments
    let algorithm = algorithm.ok_or("Algorithm (-a) is required")?;

    if file.is_none() && string.is_none() {
        return Err("Either -f (file) or -s (string) must be provided".to_string());
    }

    Ok(Config {
        algorithm,
        file,
        string,
        format,
    })
}

fn calculate_checksum(config: &Config) -> Result<(), String> {
    let algorithm = CrcAlgorithm::from_str(&config.algorithm)
        .map_err(|_| format!("Invalid algorithm: {}", config.algorithm))?;

    let checksum = if let Some(ref filename) = config.file {
        checksum_file(algorithm, filename, None).unwrap()
    } else if let Some(ref text) = config.string {
        checksum(algorithm, text.as_bytes())
    } else {
        return Err("No input provided for checksum calculation".to_string());
    };

    match config.format {
        OutputFormat::Hex => println!("{:#x?}", checksum),
        OutputFormat::Decimal => println!("{}", checksum),
    }

    Ok(())
}

fn main() -> ExitCode {
    match parse_args() {
        Ok(config) => {
            if let Err(e) = calculate_checksum(&config) {
                eprintln!("Error: {}", e);
                return ExitCode::from(1);
            }
        }
        Err(msg) => {
            if msg == "help" {
                print_usage();
                return ExitCode::SUCCESS;
            } else {
                eprintln!("Error: {}", msg);
                println!();
                print_usage();
                return ExitCode::from(1);
            }
        }
    }

    ExitCode::SUCCESS
}
