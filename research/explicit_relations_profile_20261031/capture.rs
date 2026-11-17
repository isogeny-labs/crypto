// Transparent native line-protocol proxy. Capture exactly the first 8 actual
// oracle queries, preserving all responses and setup. The driver's following
// request intentionally sees EOF; this is a bounded prefix, not a recovery.
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
};
fn main() {
    let path = env::var("ER_CAPTURE").unwrap();
    let mut transcript = File::create(&path).unwrap();
    let mut answers = File::create(format!("{path}.answers")).unwrap();
    let mut child = Command::new(env::var("ER_SERVICE").unwrap())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();
    let mut input = child.stdin.take().unwrap();
    let mut output = BufReader::new(child.stdout.take().unwrap());
    let mut count = 0;
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        writeln!(transcript, "{line}").unwrap();
        transcript.flush().unwrap();
        writeln!(input, "{line}").unwrap();
        input.flush().unwrap();
        let mut response = String::new();
        assert!(output.read_line(&mut response).unwrap() > 0);
        assert!(!response.starts_with("ERROR"), "service rejected capture");
        write!(answers, "{response}").unwrap();
        print!("{response}");
        std::io::stdout().flush().unwrap();
        if line.starts_with("find ") {
            count += 1;
            if count == 8 {
                writeln!(transcript, "quit").unwrap();
                writeln!(input, "quit").unwrap();
                input.flush().unwrap();
                assert!(child.wait().unwrap().success());
                std::fs::write(format!("{path}.complete"), "8\n").unwrap();
                return;
            }
        }
    }
    let _ = child.kill();
    let _ = child.wait();
    panic!("driver stopped before eight queries");
}
