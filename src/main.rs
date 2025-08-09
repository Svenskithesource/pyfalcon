pub mod disassemble;
mod v310;

use crate::disassemble::disassemble_code;
use clap::{Arg, ArgMatches, Command, crate_version};
use std::{io::BufReader, path::Path};
use yansi::Paint;

fn main() {
    let matches = Command::new("pyfalcon")
        .version(crate_version!())
        .author("svenskithesource")
        .about("A Python bytecode disassembler")
        .arg(
            Arg::new("input")
                .help("Input file to disassemble")
                .required(true)
                .index(1)
                .value_name("FILE"),
        )
        .arg(
            Arg::new("python-version")
                .short('v')
                .long("python-version")
                .value_name("VERSION")
                .help("Python version (e.g., 3.10, 3.11) - required for non-pyc files (marshal format)")
                .value_parser(validate_python_version),
        )
        .arg(
            Arg::new("no-colors")
                .short('n')
                .long("no-colors")
                .action(clap::ArgAction::SetTrue)
                .help("Disable coloring"),
        )
        .get_matches();

    if let Err(e) = validate_args(&matches) {
        eprintln!("Error: {}", e.red().bold());
        std::process::exit(1);
    }

    let input_file_path = matches.get_one::<String>("input").unwrap();
    let file = std::fs::File::open(input_file_path)
        .map_err(|e| {
            eprintln!("Failed to open input file: {}", e);
            std::process::exit(1);
        })
        .unwrap();

    let reader = BufReader::new(file);

    let python_version = matches.get_one::<python_marshal::magic::PyVersion>("python-version");

    let code_object = match python_version {
        Some(version) => pyc_editor::load_code(reader, *version),
        None => pyc_editor::load_pyc(reader).map(|pyc| match pyc {
            pyc_editor::PycFile::V310(pyc_file) => {
                pyc_editor::CodeObject::V310(pyc_file.code_object)
            }
        }),
    }
    .map_err(|e| {
        eprintln!("Failed to parse file: {}", e.red().bold());
        std::process::exit(1);
    })
    .unwrap();

    print!("{}", disassemble_code(&code_object, true));
}

/// Validate Python version format (e.g., 3.8, 3.9, 3.10, 3.11, etc.)
fn validate_python_version(version: &str) -> Result<python_marshal::magic::PyVersion, String> {
    let parts: Vec<&str> = version.split('.').collect();

    if parts.len() != 2 {
        return Err("Python version must be in format X.Y (e.g., 3.10)".to_string());
    }

    let major: u8 = parts[0]
        .parse()
        .map_err(|_| "Invalid major version number")?;
    let minor: u8 = parts[1]
        .parse()
        .map_err(|_| "Invalid minor version number")?;

    Ok((major, minor).into())
}

fn validate_args(matches: &ArgMatches) -> Result<(), String> {
    let input_file = matches.get_one::<String>("input").unwrap();
    let python_version = matches.get_one::<python_marshal::magic::PyVersion>("python-version");
    let no_colors = matches.get_one::<bool>("no-colors");

    if no_colors == Some(&true) {
        yansi::disable();
    }

    if !Path::new(input_file).exists() {
        return Err(format!("Input file '{}' does not exist", input_file));
    }

    let path = Path::new(input_file);
    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

    match extension.to_lowercase().as_str() {
        "pyc" => {
            // .pyc files don't require python version
            if python_version.is_some() {
                eprintln!("{}",
                    "Warning: Python version specified for .pyc file will be ignored (version is read from file header)".rgb(255, 110, 78)
                );
            }
        }
        _ => {
            if extension.to_lowercase().as_str() == "py" {
                eprintln!("{}",
                    "Warning: Python source files are not supported. Only the marshal format, either raw or as a .pyc file.
                    Will try to continue by treating the file as a binary file.".rgb(255, 110, 78)
                );
            }

            // Other file types require python version
            if python_version.is_none() {
                return Err(format!(
                    "Python version must be specified for '{}' files. Use --python-version or -v flag.",
                    if extension.is_empty() {
                        "files without extension"
                    } else {
                        extension
                    }
                ));
            }
        }
    }

    Ok(())
}
