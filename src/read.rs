use colour::unnamed::Colour;
use colour::*;
use std::env;
use std::fs::{metadata, File};
use std::io::{self, BufRead};
use std::path;
use std::path::Path;

#[derive(Debug)]
/// Inpspired by https://github.com/hashicorp/hcl/blob/main/pos.go#L10
struct Pos {
    /// Line is the source code line where this position points. Lines are
    /// counted starting at 1 and incremented for each newline character
    /// encountered.
    line: usize,
    /// Column is the source code column where this position points, in
    /// unicode characters, with counting starting at 1.
    ///
    /// Column counts characters as they appear visually, so for example a
    /// latin letter with a combining diacritic mark counts as one character.
    /// This is intended for rendering visual markers against source code in
    /// contexts where these diacritics would be rendered in a single character
    /// cell. Technically speaking, Column is counting grapheme clusters as
    /// used in unicode normalization.
    column: usize,
    /// Byte is the byte offset into the file where the indicated character
    /// begins. This is a zero-based offset to the first byte of the first
    /// UTF-8 codepoint sequence in the character, and thus gives a position
    /// that can be resolved _without_ awareness of Unicode characters.
    byte: usize,
}

/// This is WIP which simply reads a file, and outputs
/// some info in the style of the rust compiler.
pub fn read() -> () {
    let args: Vec<String> = env::args().collect();
    let default = &String::from("src/main.rs");
    let input = args.get(1).unwrap_or(default);

    let filename = path::Path::new(&input);

    green!("    Filename ");
    cyan!("{}\n\n", filename.display());

    if let md = metadata(filename).unwrap() {
        if md.is_dir() {
            red_ln!("    Error: {} is a directory", filename.display());
            return;
        }
    }

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for (line_num, line) in lines.enumerate() {
            let mut pos = Pos {
                line: 0,
                column: 0,
                byte: 0,
            };
            pos.line = line_num + 1;

            if let Ok(ip) = line {
                let ln_num_len = line_num.to_string().len();
                // println!("{}", ip);
                let first_char_index = ip
                    .chars()
                    .into_iter()
                    .position(|c| c != ' ')
                    .unwrap_or_default();

                pos.column = first_char_index;

                // yellow!("warning: ");
                cyan!("debug: ");
                print!("{}\n", "...This is not the rust compiler talking...");

                blue!("{:width$}--> ", "", width = ln_num_len);
                print!("{}:{}:{}\n", filename.display(), line_num, pos.column);

                blue!("{:width$} |\n", "", width = ln_num_len);
                blue!("{} | ", line_num);
                print!("{}\n", ip);

                blue!("{:width$} | ", "", width = ln_num_len);
                if pos.column >= 0 {
                    yellow!(
                        "{:width$}^ this is the start of the line!\n",
                        "",
                        width = first_char_index as usize
                    );
                } else {
                    red!("^ this line is empty!\n",);
                }
                blue!("{:width$} |\n", "", width = ln_num_len);
                blue!("{:width$} = ", "", width = ln_num_len);
                print!("note: I repeat, this is not the rust compiler talking...\n");
                println!("");

                // warning: unused variable: `root`
                //   --> src/main.rs:25:9
                //    |
                // 25 |     let root = current_dir.clone().join(arg);
                //    |         ^^^^ help: if this is intentional, prefix it with an underscore: `_root`
                //    = note: `#[warn(unused_variables)]` on by default
            };
        }
    };
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
