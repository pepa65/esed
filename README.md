[![version](https://img.shields.io/crates/v/esed.svg)](https://crates.io/crates/esed)
[![build](https://github.com/pepa65/esed/actions/workflows/ci.yml/badge.svg)](https://github.com/pepa65/esed/actions/workflows/ci.yml)
[![dependencies](https://deps.rs/repo/github/pepa65/esed/status.svg)](https://deps.rs/repo/github/pepa65/esed)
[![docs](https://img.shields.io/badge/docs-esed-blue.svg)](https://docs.rs/crate/esed/latest)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/pepa65/esed/blob/master/LICENSE)
[![downloads](https://img.shields.io/crates/d/esed.svg)](https://crates.io/crates/esed)

# esed 1.1.1
**Easy sed**
* Repo: <https://github.com/pepa65/esed>
* After: <https://github.com/chmln/sd> but changed the CLI
* License: MIT
* Small standalone single binary in 100% Rust

## The Pitch
Why use it over any existing tools?
* **Painless regular expressions** - `esed` uses regex syntax that is the same as JavaScript and Python.
  Forget about dealing with quirks of `sed` or `awk` and get productive immediately.
* **String-literal mode** - Non-regex search & replace: no more backslashes,
  no more remembering which characters are special and need to be escaped.
* **Easy to read, easy to write** - Search & replace expressions are split up, which makes them easy to read and write.
  No more messing with unclosed and escaped slashes.
* **Easy to modify multiple files** - No for-loops needed, just list all files.
* **Smart, common-sense defaults** - Defaults follow common sense and are tailored for typical daily use.
* **Still need `-i`/`--in-place` to modify files** - Defaults to output to stdout

### Comparison to `sed`
While `sed` does a whole lot more, `esed` focuses on doing just one thing and doing it well.
Here are some cherry-picked examples where esed shines:
* Simpler syntax for replacing all occurrences:
  - **esed**: `esed SEARCH REPLACEMENT`
  - **sed**: `sed s/SEARCH/REPLACEMENT/g`
* Replace newlines with comma-space:
  - **esed**: `esed '\n' ', '`
  - **sed**: `sed ':a;N;$!ba;s/\n/, /g'`
* Extracting stuff surrounded ny slashes out of string:
  - **esed**: `echo "Example with /path/, etc." |esed '.*(/.*/)' '$1'`
  - **sed**: `echo "Example with /path/, etc." |sed -E 's/.*(\\/.*\\/)/\1/g'`
  - Or `sed` with a different delimiter (still messy): `echo "Example with /path/ etc." |sed -E 's@.*(/.*/)@\1@g'`
* In place modification of file:
  - **esed**: `esed -i SEARCH REPLACEMENT FILE`
  - **sed**: `sed -i -e 's/SEARCH/REPLACEMENT/g' FILE` (some platforms do not need the `-e`)
* Replacement in multiple files:
  - **esed**: `esed -i SEARCH REPLACEMENT FILE* XFILE STHFILE`
  - **sed**: `for file in FILE* XFILE STHFILE; do sed -i -e 's/SEARCH/REPLACEMENT/g' "$file"; done`

#### Notes on regex syntax
* Common regex features like start & end anchors `^ #`, dot `.` wildcard (unicode) character matching,
  character class matching between square brackets `[ ]` and character exclusion between `[^` and `]`.
* Quantifiers (after the pattern): `*`: 0 or more, `+`: 1 or more, `?`: 0 or 1, `{n,m}`: between n and m times
* Alternation: `|` matches either the pattern on the left or the right.
* Escape special characters (`. * & $`) by prefixing with `\\`.
* Special matchings: `\s`: whitespace, `\S`: non-whitespace, `\d`: digit, `\pL`: letter (any unicode class)
  `\n`: newline, `\r`: carriagereturn, `\t`: tab
* Non-unicode matching between `(?-u:` and `)`
* Grouping between parentheses `( )` enables capturing patterns in the replacement and combining/grouping.
* Named grouping between `(?P<NAME>` and `)`.
* For full documentation, see: https://docs.rs/regex/latest/regex

### Benchmarks
#### Simple replacement on ~1.5 gigabytes of JSON files
```sh
hyperfine --warmup 3 --export-markdown out.md \
  'sed -E "s/\"/'"'"'/g" *.json >/dev/null' \
  'sed "s/\"/'"'"'/g" *.json >/dev/null' \
  'esed -i "\"" "'"'"'" *.json >/dev/null'
```

| Command | Mean [s] | Min…Max [s] |
|:---|---:|---:|
| `sed -E "s/\"/'/g" *.json > /dev/null` | 2.338 ± 0.008 | 2.332…2.358 |
| `sed "s/\"/'/g" *.json > /dev/null` | 2.365 ± 0.009 | 2.351…2.378 |
| `esed "\"" "'" *.json > /dev/null` | **0.997 ± 0.006** | 0.987…1.007 |

Result: **~2.35 times faster**

#### Regex replacement on a ~55M json file
```sh
hyperfine --warmup 3 --export-markdown out.md \
  'sed -E "s:(\w+):\1\1:g" dump.json >/dev/null' \
  'sed "s:\(\w\+\):\1\1:g" dump.json >/dev/null' \
  'esed "(\w+)" "$1$1" dump.json >/dev/null'
```

| Command | Mean [s] | Min…Max [s] |
|:---|---:|---:|
| `sed -E "s:(\w+):\1\1:g" dump.json >/dev/null` | 11.315 ± 0.215 | 11.102…11.725 |
| `sed "s:\(\w\+\):\1\1:g" dump.json >/dev/null` | 11.239 ± 0.208 | 11.057…11.762 |
| `esed "(\w+)" "$1$1" dump.json >/dev/null` | **0.942 ± 0.004** | 0.936…0.951 |

Result: **~11.93 times faster**

## Install
### Download and install static single-binary
```
wget https://github.com/pepa65/esed/releases/download/1.1.1/esed
sudo mv esed /usr/local/bin
sudo chown root:root /usr/local/bin/esed
sudo chmod +x /usr/local/bin/esed
```

### Install with cargo
If not installed yet, install a **Rust toolchain**, see <https://www.rust-lang.org/tools/install>

The binary `esed` will be installed into `~/.cargo/bin/` (might need to be added to `PATH`!)

#### Direct from crates.io
`cargo install esed`

#### Direct from repo
`cargo install --git https://github.com/pepa65/esed`

#### Static build for Linux (avoiding GLIBC incompatibilities)
```
git clone https://github.com/pepa65/esed
cd esed
rustup target add x86_64-unknown-linux-musl
cargo inst  # Alias defined in .cargo/config.toml
```

### Install with cargo-binstall
Even without a full Rust toolchain, rust binaries can be installed with the static binary `cargo-binstall`:
```
# Install cargo-binstall for Linux x86_64
# (Other versions are available at https://crates.io/crates/cargo-binstall)
wget github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
tar xf cargo-binstall-x86_64-unknown-linux-musl.tgz
sudo chown root:root cargo-binstall
sudo mv cargo-binstall /usr/local/bin/
```

Only a linux-x86_64 (musl) binary available: `cargo-binstall esed`

This will also be installed into `~/.cargo/bin/` and might need to be added to `PATH`!

## Quick Guide
1. **String-literal mode**: By default, expressions are treated as regex. Use `-s`/`--strings` to disable regex.
   ```sh
   # Remove string with special characters
   echo 'lots((([]))) of special chars' |esed -s '((([])))' ''
   # Result: "lots of special chars"
   ```
2. **Basic regex use**: Trim trailing whitespace.
   ```sh
   # Trim whitespace before the end of input
   echo 'lorem ipsum 23   ' |esed '\s+$' ''
   # Result: "lorem ipsum 23"
   ```
3. **Capture groups**: Can be indexed (numbered) or named
   - Indexed capture groups:
     ```sh
     echo 'cargo +nightly watch' |esed '(\w+)\s+\+(\w+)\s+(\w+)' 'cmd: $1, channel: $2, subcmd: $3'
     # Result: "cmd: cargo, channel: nightly, subcmd: watch"
     ```
   - Named capture groups:
     ```sh
     echo "123.45" |esed '(?P<dollars>\d+)\.(?P<cents>\d+)' '$dollars dollars and $cents cents'
     # Result: 123 dollars and 45 cents
     ```
     If stumbling upon ambiguities, use `${var}` instead of `$var`, like:
     ```sh
     echo '123.45' |esed '(?P<dollars>\d+)\.(?P<cents>\d+)' '${dollars}_dollars and ${cents}_cents'
     # Result: "123_dollars and 45_cents"
     ```
4. **Search & replace in a file**
   ```sh
   esed -i 'window.fetch' 'fetch' http.js
   ```
   To preview changes (the `-i`/`--in-place` flag needs to be given to modify files):
   ```sh
   esed 'window.fetch' 'fetch' http.js
   ```
5. **Search & replace across project**
   ```sh
   esed 'from "react"' 'from "preact"' ../*/*.json
   ```

### Escaping special characters
To escape `$`, use `$$`:
```sh
echo "foo" |esed 'foo' '$$bar'
# Result: "$bar"
```

## Usage
```
esed v1.1.1 - Easy sed

Usage: esed [OPTIONS] <SEARCH> <REPLACEMENT> [FILE]...
Arguments:
  <SEARCH>       SEARCH: regexp or (if using '-s'/'--strings') string
  <REPLACEMENT>  REPLACEMENT (regex captured values like $1 can be used)
  [FILE]...      Optional path(s) to file(s)

Options:
  -i, --in-place           Modify file(s) in-place instead of output to stdout
  -s, --strings            Treat SEARCH and REPLACEMENT as literal strings [default: regex]
  -c, --case-insensitive   Use regex case-insensitive [default: case-sensitive]
  -w, --word               Match the SEARCH regex as a full word [default: match anywhere]
  -d, --dot-match-newline  Dots also match newlines [default: newlines not matched by dots]
  -n, --no-multiline       No multiline: '^' and '$' anchor only to the very beginning and end [default: multiline]
  -l, --limit <MAX>        Limit the number of replacements per file (0: unlimited) [default: 0]
  -h, --help               Print help
  -V, --version            Print version
```
