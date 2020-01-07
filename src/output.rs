use std::fs::File;
use std::io::{BufWriter, stdout, Write};

pub fn to_stdout(content: String) {
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    writeln!(out, "{}", &content).unwrap();
}

pub fn to_file(content: String, path: &str) {
    let mut writer = BufWriter::new(File::create(path).unwrap());
    writer.write_all(content.as_bytes()).unwrap();
}
