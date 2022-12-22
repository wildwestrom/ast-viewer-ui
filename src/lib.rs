use quote::ToTokens;

#[must_use]
pub fn ast_to_string(ast: syn::File) -> String {
	let string_from_ast_shebang: String = if let Some(shebang) = ast.shebang {
		shebang
	} else {
		String::new()
	};
	let string_from_ast_attrs: String = ast
		.attrs
		.iter()
		.map(|attr| attr.to_token_stream().to_string())
		.collect();
	let string_from_ast_items: String = ast
		.items
		.iter()
		.map(|item| item.to_token_stream().to_string())
		.collect();

	let mut new_code_string = String::new();
	new_code_string.push_str(&string_from_ast_shebang);
	new_code_string.push_str(&string_from_ast_attrs);
	new_code_string.push_str(&string_from_ast_items);

	new_code_string
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

	fn test_round_trip(testfile: &str) {
		let mut filepath = "test-inputs/".to_string();
		filepath.push_str(testfile);
		let contents = std::fs::read_to_string(filepath.clone())
			.expect(&format!("failed to read file from {filepath}"));
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
