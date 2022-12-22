use std::{ffi::OsString, fs};

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
	let args = Cli::parse();
	let contents = fs::read_to_string(args.input)?;
	let ast = syn::parse_file(&contents)?;
	dbg!(&ast);
	let roundtripped_code = ast_viewer_ui::ast_to_string(ast);
	fs::write(args.output, roundtripped_code)?;
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
