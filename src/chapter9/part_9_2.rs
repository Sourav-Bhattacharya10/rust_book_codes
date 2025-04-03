use std::fs;
use std::io;

pub fn recoverable_errors() -> Result<String, io::Error> {
    fs::read_to_string("src/chapter9/hello.txt")
}
