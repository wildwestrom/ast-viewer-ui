use std::{ffi::OsString, error::Error};

use clap::Parser;

// proc-macro-2 crate might come in handy

fn main() -> Result<(), Box<dyn Error>> {
	let args = Cli::parse();
	let filename = args.file;
	let contents = std::fs::read_to_string(filename)?;
    let ast = syn::parse_file(&contents)?;
    println!("{:#?}", ast);
    Ok(())
}

#[derive(Parser)]
struct Cli {
	/// File to browse ast of.
	file: OsString,
}

#[cfg(test)]
mod tests {}
