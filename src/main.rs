use std::{ffi::OsString, error::Error};
use clap::Parser;
use quote::ToTokens;
// proc-macro2 crate might come in handy

fn main() -> Result<(), Box<dyn Error>> {
	let args = Cli::parse();
	let contents = std::fs::read_to_string(args.input)?;
    // turn file into ast
    let ast = syn::parse_file(&contents)?;
    // turn ast back into a file
    let str_from_ast: String = ast.items.iter().map(|item| {
        let token_stream = item.to_token_stream().to_string();
        token_stream
    }).collect();
    std::fs::write(args.output, str_from_ast)?;
    Ok(())
}

#[derive(Parser)]
struct Cli {
	/// File to browse ast of.
	input: OsString,
    /// File to output to.
    #[clap(short, long, default_value = "out.rs")]
    output: OsString,
}

#[cfg(test)]
mod tests {}
