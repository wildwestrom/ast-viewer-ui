use std::ffi::OsString;

use anyhow::Result;
use clap::Parser;
use quote::ToTokens;
// proc-macro2 crate might come in handy

fn round_trip(input_code: &str) -> Result<String> {
	let ast = syn::parse_file(&input_code)?;
	let string_from_ast_shebang: String = if let Some(shebang) = ast.shebang {
		shebang
	} else {
		"".to_string()
	};
	let string_from_ast_attrs: String = ast
		.attrs
		.iter()
		.map(|attr| {
			let token_stream = attr.to_token_stream().to_string();
			token_stream
		})
		.collect();
	let string_from_ast_items: String = ast
		.items
		.iter()
		.map(|item| {
			let token_stream = item.to_token_stream().to_string();
			token_stream
		})
		.collect();

	let mut roundtripped_code = String::new();
	roundtripped_code.push_str(&string_from_ast_shebang);
	roundtripped_code.push_str(&string_from_ast_attrs);
	roundtripped_code.push_str(&string_from_ast_items);
	Ok(roundtripped_code)
}

fn main() -> Result<()> {
	let args = Cli::parse();
	let contents = std::fs::read_to_string(args.input)?;
	let roundtripped_code = round_trip(&contents)?;
	assert_eq!(round_trip(&roundtripped_code)?, roundtripped_code);
	std::fs::write(args.output, roundtripped_code)?;
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
mod tests {
	use super::*;

	fn test_round_trip(testfile: &str) {
		let mut filepath = "test-inputs/".to_string();
		filepath.push_str(testfile);
		let contents = std::fs::read_to_string(filepath.clone()).expect(&format!("failed to read file from {filepath}"));
		let roundtripped_code = round_trip(&contents).expect("failed to read rust source string");
		assert_eq!(round_trip(&roundtripped_code).unwrap(), roundtripped_code);
	}

	#[test]
	fn hello() {
		test_round_trip("hello.rs")
	}

	#[test]
	fn quicksort() {
		test_round_trip("quicksort.rs")
	}

	#[test]
	fn top_level_attrs() {
		test_round_trip("intrinsics.rs")
	}

	#[test]
	fn shebang() {
		test_round_trip("find-crate-roots.rs")
	}

	#[test]
	fn macros() {
		test_round_trip("macros.rs")
	}
}
