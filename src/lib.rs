use quote::ToTokens;

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
	fn test_round_trip() {
		for case in TEST_CASES {
			let filepath = format!("test-inputs/{}", case);
			let contents = std::fs::read_to_string(filepath.clone())
				.unwrap_or_else(|_| panic!("failed to read file from {}", filepath));
			let roundtripped_code =
				round_trip(&contents).expect("failed to read rust source string");
			assert_eq!(round_trip(&roundtripped_code).unwrap(), roundtripped_code);
		}
	}
}
