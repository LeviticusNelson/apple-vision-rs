use std::env;

use swift_rs::{swift, SRString};

swift!(fn perform_ocr_swift(path: &SRString) -> Option<SRString>);

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];

    println!("{}", extract_text(file.as_str()))
}

fn extract_text(path: &str) -> String {
    let value: SRString = path.into();
    let result = unsafe { perform_ocr_swift(&value) };
    String::from(result.unwrap().as_str())
}
