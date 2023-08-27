use anyhow::{anyhow, Result};
use quote::ToTokens;
pub use syn;

#[must_use]
pub fn ast_to_string(ast: syn::File) -> String {
	let shebang = ast.shebang.unwrap_or_default();
	let attrs = ast
		.attrs
		.iter()
		.map(|attr| attr.to_token_stream().to_string());
	let items = ast
		.items
		.iter()
		.map(|item| item.to_token_stream().to_string());

	let mut ast_string = shebang;
	ast_string.extend(attrs);
	ast_string.extend(items);

	ast_string
}

/// # Errors
///
/// This function fails if there was an error parsing the original source code.
pub fn round_trip(input_code: &str) -> syn::Result<String> {
	let ast = syn::parse_file(input_code)?;
	Ok(ast_to_string(ast))
}

/// This was partly copy pasted from rust-analyzer.
///
/// # Warning
///
/// This function relies on a shell command and thus any environment in which the program executes.
pub fn format_rust_code(input_code: &str) -> Result<String> {
	let mut cmd = std::process::Command::new("rustfmt");

	let mut fmt = cmd
		.stdin(std::process::Stdio::piped())
		.stdout(std::process::Stdio::piped())
		.stderr(std::process::Stdio::piped())
		.spawn()?;

	std::io::Write::write_all(
		&mut fmt.stdin.as_mut().ok_or(anyhow!("No Stdin"))?,
		input_code.as_bytes(),
	)?;

	let output = fmt.wait_with_output()?;
	let captured_stdout = String::from_utf8(output.stdout)?;

	Ok(captured_stdout)
}

#[cfg(test)]
mod tests {
	use super::*;

	const TEST_CASES: &[&str] = &[
		"hello.rs",
		"quicksort.rs",
		"intrinsics.rs",
		"find-crate-roots.rs",
		"macros.rs",
	];

	#[test]
	fn round_trip_generates_equivalent_ast() {
		for case in TEST_CASES {
			let filepath = format!("../test-inputs/{}", case);
			let input = std::fs::read_to_string(filepath.clone())
				.unwrap_or_else(|_| panic!("failed to read file from {}", filepath));
			let roundtripped_code = round_trip(&input).expect("failed to read rust source string");
			assert_eq!(round_trip(&roundtripped_code).unwrap(), roundtripped_code);
		}
	}
	#[test]
	fn output_from_ast_is_equal_to_input() {
		for case in TEST_CASES {
			let filepath = format!("../test-inputs/{}", case);
			let input = std::fs::read_to_string(filepath.clone())
				.unwrap_or_else(|_| panic!("failed to read file from {}", filepath));
			let output = format_rust_code(&round_trip(&input).unwrap()).unwrap();
			assert_eq!(input, output);
		}
	}
}
