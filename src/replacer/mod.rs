use std::borrow::Cow;

use crate::{Result, unescape};

use regex::bytes::Regex;

mod validate;

pub use validate::{InvalidReplaceCapture, validate_replace};

pub(crate) struct Replacer {
	regex: Regex,
	replace_with: Vec<u8>,
	is_literal: bool,
	replacements: usize,
}

impl Replacer {
	#[allow(clippy::too_many_arguments)]
	pub(crate) fn new(
		look_for: String, replace_with: String, is_literal: bool, no_multiline: bool, dot_match_newline: bool, case_insensitive: bool, words: bool,
		replacements: usize,
	) -> Result<Self> {
		let (look_for, replace_with) = if is_literal {
			(regex::escape(&look_for), replace_with.into_bytes())
		} else {
			validate_replace(&replace_with)?;
			(look_for, unescape::unescape(&replace_with).into_bytes())
		};
		let mut regex = regex::bytes::RegexBuilder::new(&look_for);
		regex.multi_line(true);
		if no_multiline {
			regex.multi_line(false);
		};
		if dot_match_newline {
			regex.dot_matches_new_line(true);
		};
		regex.case_insensitive(false);
		if case_insensitive {
			regex.case_insensitive(true);
		};
		if words {
			regex = regex::bytes::RegexBuilder::new(&format!("\\b{}\\b", look_for));
		};
		Ok(Self { regex: regex.build()?, replace_with, is_literal, replacements })
	}

	pub(crate) fn replace<'a>(&'a self, content: &'a [u8]) -> Cow<'a, [u8]> {
		let regex = &self.regex;
		let limit = self.replacements;
		let use_color = false;
		if self.is_literal {
			Self::replacen(regex, limit, content, use_color, regex::bytes::NoExpand(&self.replace_with))
		} else {
			Self::replacen(regex, limit, content, use_color, &*self.replace_with)
		}
	}

	/// A modified form of [`regex::bytes::Regex::replacen`] that supports coloring replacements
	pub(crate) fn replacen<'haystack, R: regex::bytes::Replacer>(
		regex: &regex::bytes::Regex, limit: usize, haystack: &'haystack [u8], _use_color: bool, mut rep: R,
	) -> Cow<'haystack, [u8]> {
		let mut it = regex.captures_iter(haystack).enumerate().peekable();
		if it.peek().is_none() {
			return Cow::Borrowed(haystack);
		}
		let mut new = Vec::with_capacity(haystack.len());
		let mut last_match = 0;
		for (i, cap) in it {
			// unwrap on 0 is OK because captures only reports matches
			let m = cap.get(0).unwrap();
			new.extend_from_slice(&haystack[last_match..m.start()]);
			rep.replace_append(&cap, &mut new);
			last_match = m.end();
			if limit > 0 && i >= limit - 1 {
				break;
			}
		}
		new.extend_from_slice(&haystack[last_match..]);
		Cow::Owned(new)
	}
}
