use crate::{arithmetic::Point, Result};
use serde::{Deserialize, Serialize};
use std::{
    ffi::CString,
    io::{BufRead, BufReader, Write},
    path::Path,
    process::{Child, Command, Stdio},
};
#[inline(never)]
fn request(n: u64, arg: u64) -> u64 {
    let a = [n, arg, 0, 0, 0, 0];
    let mut r = 0u64;
    unsafe {
        std::arch::asm!("rol rdi, 3","rol rdi, 13","rol rdi, 61","rol rdi, 51","xchg rbx, rbx",in("rax")a.as_ptr(),inout("rdx")r,out("rdi")_,options(nostack));
    }
    r
}
pub fn running() -> bool {
    request(0x1001, 0) > 0
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Mark {
    pub phase: String,
    pub gp_calls_before: usize,
    pub gp_calls_after: usize,
}
pub struct Roi {
    pub marks: Vec<Mark>,
}
impl Roi {
    pub fn new() -> Self {
        Self { marks: vec![] }
    }
    pub fn begin(&self) {
        request(0x43540004, 0);
        request(0x43540001, 0);
    }
    pub fn end(&mut self, name: &str, begin: usize, end: usize) {
        request(0x43540005, 0);
        let s = CString::new(name).unwrap();
        request(0x43540003, s.as_ptr() as u64);
        self.marks.push(Mark {
            phase: name.into(),
            gp_calls_before: begin,
            gp_calls_after: end,
        });
    }
}
pub fn point(p: Point) -> String {
    p.map_or("[0]".into(), |x| format!("[{},{}]", x[0], x[1]))
}
pub fn points(ps: &[Point]) -> String {
    format!(
        "[{}]",
        ps.iter().map(|p| point(*p)).collect::<Vec<_>>().join(",")
    )
}
pub struct Pari {
    pub child: Child,
    input: std::process::ChildStdin,
    output: BufReader<std::process::ChildStdout>,
    pub calls: usize,
    pub transcript: Vec<String>,
}
impl Pari {
    pub fn new(gp: &Path, so: &Path, stderr: &Path) -> Result<Self> {
        let mut c = Command::new(gp)
            .args(["-f", "-q", "-s", "64000000"])
            .env(
                "LD_LIBRARY_PATH",
                "/workspace/shared/twist-tools-20261023/cas/root/usr/lib/x86_64-linux-gnu",
            )
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(std::fs::File::create(stderr)?)
            .spawn()?;
        let input = c.stdin.take().unwrap();
        let output = BufReader::new(c.stdout.take().unwrap());
        let mut p = Self {
            child: c,
            input,
            output,
            calls: 0,
            transcript: vec![],
        };
        writeln!(p.input,"default(prompt,\"\");install(\"twist_roi_start\",\"v\",\"tstart\",{});install(\"twist_roi_stop\",\"v\",\"tstop\",{});",serde_json::to_string(&so.display().to_string())?,serde_json::to_string(&so.display().to_string())?)?;
        writeln!(
            p.input,
            "{}\nprint(\"READY\");",
            include_str!("../../../execution-v15/support/cold-ops.gp")
        )?;
        p.input.flush()?;
        let mut s = String::new();
        p.output.read_line(&mut s)?;
        if s.trim() != "READY" {
            return Err(format!("GP setup rejected: {s}").into());
        }
        Ok(p)
    }
    pub fn call(&mut self, expr: &str) -> Result<serde_json::Value> {
        writeln!(self.input, "coldcall({});", serde_json::to_string(expr)?)?;
        self.input.flush()?;
        let mut response = String::new();
        loop {
            let mut s = String::new();
            if self.output.read_line(&mut s)? == 0 {
                return Err("GP ended before END".into());
            }
            if s.trim() == "END" {
                break;
            }
            response.push_str(&s);
        }
        self.calls += 1;
        self.transcript.push(format!(
            "REQUEST {}\n{}\nRESPONSE\n{}",
            self.calls, expr, response
        ));
        if response.contains("ERROR|") {
            return Err(format!("GP rejected: {response}").into());
        }
        Ok(serde_json::from_str(response.trim())?)
    }
    pub fn close(mut self) -> Result<Vec<String>> {
        writeln!(self.input, "quit;")?;
        self.input.flush()?;
        drop(self.input);
        let status = self.child.wait()?;
        if !status.success() {
            return Err(format!("GP process failed: {status}").into());
        }
        Ok(self.transcript)
    }
}
