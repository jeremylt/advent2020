use crate::prelude::*;

// -----------------------------------------------------------------------------
// Load file
// -----------------------------------------------------------------------------
pub(crate) fn load_file(path: &str) -> std::io::BufReader<File> {
    let input: File;
    match File::open(path) {
        Ok(file) => input = file,
        Err(_error) => panic!("Unable to open input file"),
    }
    let buffer = BufReader::new(input);
    buffer
}

// -----------------------------------------------------------------------------
