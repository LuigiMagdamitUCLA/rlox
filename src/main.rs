use std::process;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let mut args: Vec<String> = Vec::new();
    args.push(String::from(""));

    error(2, String::from("message"));
    if args.len() > 1 {
        println!("Usage: rlox [script]");
        process::exit(1);
    } else if args.len() == 1 {
        run_file(String::from("src/file.txt"));
    } else {
        //
    }
}
fn run_file(path: String) {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    // read the file into a vector
    reader.read_to_end(&mut buffer).ok();

    let s = String::from_utf8(buffer).expect("invalid");
    // read bytes back into string
    run(s);
    
}

fn run(source: String) {
    println!("{}", source);
}

// error reporting
fn error(line: u32, message: String) {
    report(line, String::from(""), message);
}

fn report(line: u32, location: String, message: String) {
    eprintln!("[line {}] Error: {} {}", line, location, message);
}