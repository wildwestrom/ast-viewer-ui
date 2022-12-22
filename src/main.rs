use std::{ffi::OsString, fs};

use anyhow::Result;
use ast_viewer_ui::round_trip;
use clap::Parser;
// proc-macro2 crate might come in handy

fn main() -> Result<()> {
	let args = Cli::parse();
	let contents = fs::read_to_string(args.input)?;
	let roundtripped_code = round_trip(&contents)?;
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
