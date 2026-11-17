//! Small authenticated Rust transport for one immutable measurement bundle.
//! It never receives the Runpod API credential. IC_TASK_TOKEN is ephemeral.
use std::{
    env,
    fs::{self, File},
    io::{self, BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    path::Path,
    process::{self, Command, Stdio},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
const ROOT: &str = "/opt/ic-run";
#[derive(Default)]
struct State {
    uploaded: bool,
    started: bool,
    finished: bool,
    archive_ready: bool,
    code: Option<i32>,
}
fn authorized(actual: &str, expected: &str) -> bool {
    actual.len() == expected.len()
        && actual
            .bytes()
            .zip(expected.bytes())
            .fold(0u8, |d, (a, b)| d | (a ^ b))
            == 0
}
fn begin(s: &mut State) -> bool {
    if !s.uploaded || s.started {
        return false;
    }
    s.started = true;
    true
}
fn reply(s: &mut TcpStream, status: &str, body: &str) -> io::Result<()> {
    write!(s,"HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",body.len())
}
fn execute(state: Arc<Mutex<State>>) {
    let result = (|| -> io::Result<(i32, bool)> {
        let task = Path::new(ROOT).join("task");
        fs::create_dir(&task)?;
        let unpack = Command::new("tar")
            .args(["--no-same-owner", "-xzf"])
            .arg(Path::new(ROOT).join("bundle.tar.gz"))
            .arg("-C")
            .arg(&task)
            .status()?;
        if !unpack.success() {
            return Ok((90, false));
        }
        let script = if task.join("run-task.sh").is_file() {
            "run-task.sh"
        } else if task.join("run-portfolio-task.sh").is_file() {
            "run-portfolio-task.sh"
        } else {
            "run-rho-native-task.sh"
        };
        let status = Command::new("bash")
            .arg(script)
            .current_dir(&task)
            .env_remove("IC_TASK_TOKEN")
            .stdout(Stdio::from(File::create(
                Path::new(ROOT).join("task.stdout"),
            )?))
            .stderr(Stdio::from(File::create(
                Path::new(ROOT).join("task.stderr"),
            )?))
            .status()?;
        let archive = Command::new("tar")
            .args(["-czf"])
            .arg(Path::new(ROOT).join("artifacts.tar.gz"))
            .arg("-C")
            .arg(ROOT)
            .args(["task/run", "task.stdout", "task.stderr"])
            .status()?;
        Ok((status.code().unwrap_or(128), archive.success()))
    })();
    let mut s = state.lock().expect("state");
    s.finished = true;
    match result {
        Ok((code, ready)) => {
            s.code = Some(code);
            s.archive_ready = ready;
        }
        Err(_) => s.code = Some(91),
    }
}
fn serve(mut stream: TcpStream, token: &str, state: Arc<Mutex<State>>) -> io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(60)))?;
    stream.set_write_timeout(Some(Duration::from_secs(60)))?;
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut first = String::new();
    reader.read_line(&mut first)?;
    let fields: Vec<_> = first.split_whitespace().collect();
    if fields.len() != 3 {
        return reply(&mut stream, "400 Bad Request", "{}");
    }
    let (method, path) = (fields[0], fields[1]);
    let (mut auth, mut length, mut total, mut expect) = (String::new(), 0usize, first.len(), false);
    loop {
        let mut line = String::new();
        let n = reader.read_line(&mut line)?;
        total += n;
        if total > 32768 || n == 0 {
            return reply(&mut stream, "400 Bad Request", "{}");
        }
        if line == "\r\n" {
            break;
        }
        if let Some((k, v)) = line.split_once(':') {
            let v = v.trim();
            if k.eq_ignore_ascii_case("authorization") {
                auth = v.into();
            }
            if k.eq_ignore_ascii_case("content-length") {
                length = v.parse().map_err(|_| io::Error::other("length"))?;
            }
            if k.eq_ignore_ascii_case("expect") && v.eq_ignore_ascii_case("100-continue") {
                expect = true;
            }
        }
    }
    if !authorized(&auth, &format!("Bearer {token}")) {
        return reply(&mut stream, "403 Forbidden", "{}");
    }
    if method == "GET" && (path == "/health" || path == "/status") {
        let s = state.lock().expect("state");
        let code = s.code.map_or("null".into(), |v| v.to_string());
        return reply(&mut stream,"200 OK",&format!("{{\"ready\":true,\"uploaded\":{},\"started\":{},\"finished\":{},\"archive_ready\":{},\"exit_code\":{code}}}",s.uploaded,s.started,s.finished,s.archive_ready));
    }
    if method == "PUT" && path == "/bundle" {
        if length == 0 || length > 128 * 1024 * 1024 {
            return reply(&mut stream, "413 Content Too Large", "{}");
        }
        if state.lock().expect("state").started {
            return reply(&mut stream, "409 Conflict", "{}");
        }
        if expect {
            stream.write_all(b"HTTP/1.1 100 Continue\r\n\r\n")?;
        }
        let partial = Path::new(ROOT).join("bundle.partial");
        let mut out = File::create(&partial)?;
        let received = io::copy(&mut reader.take(length as u64), &mut out)?;
        if received != length as u64 {
            return reply(&mut stream, "400 Bad Request", "{}");
        }
        out.sync_all()?;
        fs::rename(partial, Path::new(ROOT).join("bundle.tar.gz"))?;
        state.lock().expect("state").uploaded = true;
        return reply(&mut stream, "200 OK", "{\"uploaded\":true}");
    }
    if method == "POST" && path == "/run" {
        if !begin(&mut state.lock().expect("state")) {
            return reply(&mut stream, "409 Conflict", "{}");
        }
        thread::spawn(move || execute(state));
        return reply(&mut stream, "202 Accepted", "{\"started\":true}");
    }
    if method == "GET" && path == "/artifacts" {
        if !state.lock().expect("state").archive_ready {
            return reply(&mut stream, "409 Conflict", "{}");
        }
        let mut f = File::open(Path::new(ROOT).join("artifacts.tar.gz"))?;
        let n = f.metadata()?.len();
        write!(stream,"HTTP/1.1 200 OK\r\nContent-Type: application/gzip\r\nContent-Length: {n}\r\nConnection: close\r\n\r\n")?;
        io::copy(&mut f, &mut stream)?;
        return Ok(());
    }
    reply(&mut stream, "404 Not Found", "{}")
}
fn main() -> io::Result<()> {
    fs::create_dir_all(ROOT)?;
    let token = env::var("IC_TASK_TOKEN").map_err(|_| io::Error::other("missing task token"))?;
    if token.len() < 48 {
        return Err(io::Error::other("short task token"));
    }
    let lifetime = env::var("IC_RUNNER_LIFETIME_SECONDS")
        .unwrap_or_else(|_| "2700".into())
        .parse::<u64>()
        .map_err(|_| io::Error::other("invalid runner lifetime"))?;
    if !(900..=7200).contains(&lifetime) {
        return Err(io::Error::other("runner lifetime outside bounded range"));
    }
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(lifetime));
        process::exit(0);
    });
    let state = Arc::new(Mutex::new(State::default()));
    let listener = TcpListener::bind("0.0.0.0:8000")?;
    eprintln!("Rust capture transport ready");
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            if serve(stream, &token, state.clone()).is_err() {
                eprintln!("capture request failed");
            }
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn authorization_and_single_execution_gate() {
        assert!(authorized("Bearer abc", "Bearer abc"));
        assert!(!authorized("Bearer abd", "Bearer abc"));
        assert!(!authorized("Bearer ab", "Bearer abc"));
        let mut s = State::default();
        assert!(!begin(&mut s));
        s.uploaded = true;
        assert!(begin(&mut s));
        assert!(!begin(&mut s));
    }
}
