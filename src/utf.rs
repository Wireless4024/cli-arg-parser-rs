use std::borrow::Cow;

use anyhow::{anyhow, bail, Result};

use crate::{
	ESCAPE_CHAR,
	LoopOption,
	LoopOption::{
		CONTINUE,
		NOTHING,
	},
};

macro_rules! parse_escape_char {
    ($ch:ident,$buf:ident,$($char:literal,$output:stmt),*) => {
	    match $ch {
			$($char => {
				$buf.push({$output});
				return Ok(($ch, CONTINUE));
			}),*
			_ => bail!("Invalid escape token {}", $ch)
		}
    };
}


fn parse_escape_utf<'a>(cli: &mut impl Iterator<Item=char>, buf: &mut Vec<char>) -> Result<(char, LoopOption)> {
	let mut cur = ESCAPE_CHAR;
	let mut escape_len = 1;
	while let Some(ch) = cli.next() {
		if ch != ESCAPE_CHAR {
			cur = ch;
			break;
		} else {
			escape_len += 1;
			if escape_len & 1 == 0 {
				buf.push(ESCAPE_CHAR);
			}
		}
	}
	return if escape_len & 1 == 1 {
		parse_escape_char!(
					cur, buf,
					'n', '\n',
					'r', '\r',
					't', '\t',
					'v', '\x0B',
					'0', {
						let hex = String::from_iter(cli.take(2));
						u8::from_str_radix(&hex, 8)? as char
					},
					'x', {
						let hex = String::from_iter(cli.take(2));
						u8::from_str_radix(&hex, 16)? as char
					},
					'u', {
						let mut hex = String::from_iter(cli.take(4));
						let emoji = if hex.starts_with(&['1','e','E']) {
							hex.extend(cli.take(1));
							true
						} else { false };
						let first = u32::from_str_radix(&hex, 16)?;
						match char::from_u32(first as u32) {
							None => {
								if emoji { hex.extend(cli.take(3)); }
								let second = u32::from_str_radix(&hex,16)?;
								char::from_u32(second as u32).ok_or_else(||anyhow!("Invalid utf!"))?
							}
							Some(ch) => ch
						}
					},
					'"', '"',
					'\'', '\'',
					' ', ' ');
	} else {
		Ok((cur, NOTHING))
	};
}

pub fn parse_token_utf<'a>(cli: &mut impl Iterator<Item=char>) -> Result<Cow<'a, str>> {
	let mut head = if let Some(ch) = cli.next() {
		ch
	} else {
		bail!("Ends of token!")
	};

	while head.is_ascii_whitespace() {
		head = match cli.next() {
			Some(ch) => ch,
			None => {
				return Ok(Cow::Owned(String::new()));
			}
		};
	}

	let mut buf = Vec::new();
	let quoted = head == '"' || head == '\'';

	if !quoted && head != ESCAPE_CHAR {
		buf.push(head);
	}

	while head == ESCAPE_CHAR {
		let (ch, option) = parse_escape_utf(cli, &mut buf)?;

		if option == NOTHING {
			head = ch;
		} else if option == CONTINUE {
			break;
		}
	}

	while let Some(mut cur) = cli.next() {
		if cur.is_ascii_whitespace() && !quoted {
			break;
		}
		if cur == ESCAPE_CHAR {
			let (ch, option) = parse_escape_utf(cli, &mut buf)?;
			match option {
				CONTINUE => {
					continue;
				}
				_ => {
					cur = ch;
				}
			}
		}
		// end of quote
		if quoted && cur == head {
			break;
		};
		buf.push(cur);
	}

	Ok(Cow::Owned(String::from_iter(buf)))
}

pub fn parse_arg_utf(cli: &str) -> Vec<String> {
	let mut chars = cli.chars();
	let mut res = Vec::new();
	while let Ok(token) = parse_token_utf(&mut chars) {
		res.push(String::from(token));
	}
	res
}