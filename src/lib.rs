pub use utf::{parse_arg_utf, parse_token_utf};

#[cfg(feature = "utf")]
mod utf;

#[cfg(feature = "ascii")]
mod ascii;

const ESCAPE_CHAR: char = '\\';

#[derive(PartialEq)]
enum LoopOption {
	NOTHING,
	CONTINUE,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[cfg(feature = "utf")]
	fn test_parse_utf() {
		let res = parse_arg_utf(r#"\'\"\n\r\t\v\061\u1F600\\\\A 'word with space' also\ space"#);
		assert_eq!(res, vec![String::from("\'\"\n\r\t\x0b\x61😀\\\\A"), String::from("word with space"), String::from("also space")])
	}
}
