//! Streaming file concatenation for `catr`.

use std::{
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
};

/// Copies files to the output in order and flushes the output on success.
///
/// A path of `-` reads from `input`. An empty file list also reads from
/// `input`. Multiple `-` operands share the same stream position.
pub fn concatenate(
    files: &[PathBuf],
    input: &mut impl Read,
    output: &mut impl Write,
) -> io::Result<()> {
    if files.is_empty() {
        io::copy(input, output)?;
    }

    for path in files {
        if path.as_os_str() == "-" {
            io::copy(input, output)?;
        } else {
            io::copy(&mut File::open(path)?, output)?;
        }
    }

    output.flush()
}
