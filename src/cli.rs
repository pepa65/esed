use clap::Parser;

#[derive(Parser, Debug)]
#[command(
	version,
	about,
	help_template = "\
{before-help}{name} v{version} - {about-with-newline}
{usage-heading} {usage}
{all-args}{after-help}"
)]
pub struct Options {
	#[arg(short, long)]
	/// Modify file(s) in-place instead of output to stdout
	pub in_place: bool,

	#[arg(short = 's', long = "strings", value_name = "STRINGS")]
	/// Treat SEARCH and REPLACEMENT as literal strings [default: regex]
	pub literal_mode: bool,

	#[arg(short, long)]
	/// Use regex case-insensitive [default: case-sensitive]
	pub case_insensitive: bool,

	#[arg(short, long)]
	/// Match the SEARCH regex as a full word [default: match anywhere]
	pub word: bool,

	#[arg(short, long)]
	/// Dots also match newlines [default: newlines not matched by dots]
	pub dot_match_newline: bool,

	#[arg(short, long)]
	/// No multiline: '^' and '$' anchor only to the very beginning and end [default: multiline]
	pub no_multiline: bool,

	#[arg(allow_hyphen_values = true, value_name = "SEARCH")]
	/// SEARCH: regexp or (if using '-s'/'--strings') string
	pub find: String,

	#[arg(short = 'l', long = "limit", value_name = "MAX", default_value_t)]
	/// Limit the number of replacements per file (0: unlimited)
	pub replacements: usize,

	#[arg(allow_hyphen_values = true, value_name = "REPLACEMENT")]
	/// REPLACEMENT (regex captured values like $1 can be used)
	pub replace_with: String,

	#[arg(value_name = "FILE")]
	/// Optional path(s) to file(s)
	pub files: Vec<std::path::PathBuf>,
}
