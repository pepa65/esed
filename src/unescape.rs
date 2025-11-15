use std::{char, str::Chars};

/// Convert a string with backslash escapes to a string with the proper escaped characters
pub fn unescape(input: &str) -> String {
	let mut chars = input.chars();
	let mut s = String::new();
	while let Some(c) = chars.next() {
		if c != '\\' {
			s.push(c);
			continue;
		}
		let Some(char) = chars.next() else {
			// This means that the last char is a `\\`
			assert_eq!(c, '\\');
			s.push('\\');
			break;
		};

		let escaped: Option<char> = match char {
			'n' => Some('\n'),
			'r' => Some('\r'),
			't' => Some('\t'),
			'\'' => Some('\''),
			'\"' => Some('\"'),
			'\\' => Some('\\'),
			'u' => escape_n_chars(&mut chars, 4),
			'x' => escape_n_chars(&mut chars, 2),
			_ => None,
		};
		if let Some(char) = escaped {
			// Successfully escaped a sequence
			s.push(char);
		} else {
			// User didn't meant to escape that
			s.push('\\');
			s.push(char);
		}
	}
	s
}

/// This is for sequences such as `\x08` or `\u1234`
fn escape_n_chars(chars: &mut Chars<'_>, length: usize) -> Option<char> {
	let s = chars.as_str().get(0..length)?;
	let u = u32::from_str_radix(s, 16).ok()?;
	let ch = char::from_u32(u)?;
	_ = chars.nth(length);
	Some(ch)
}
