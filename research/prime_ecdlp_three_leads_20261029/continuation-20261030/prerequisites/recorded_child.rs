//! Local Unix software prerequisite: declared launch descriptor plus observed
//! root exit, durably requested before publishing the old outcome label.
//!
//! This is NOT the scientific launcher. Gate callbacks and source/binary labels
//! are supplied data, not checkpoint/protocol/runtime or execution attestation.
//! Public program/argument bytes are captured from the same owned Command later
//! passed to spawn, but environment, I/O, pre-exec hooks, executable resolution,
//! image/source admission, descendants, power-loss and resume are unqualified.
//! No prices, work counts or uninterrupted clock are inferred. Trusted local
//! directory assumptions remain. Reader consistency is not proof of execution
//! or of receipt of a writer's sync acknowledgment.

#[cfg(not(unix))]
compile_error!("recorded_child currently requires Unix argument/status encoding");

#[path = "child_observation.rs"]
pub mod child_observation;

use child_observation::attempt_driver::durable_attempt::{
    self, Attempt, Binding, Observed, Outcome,
};
use child_observation::attempt_driver::Gate;
use child_observation::{ChildObservation, ObservedExit};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

const MAX_RECORD: usize = 16384;

fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn unhex(text: &str) -> io::Result<String> {
    if !text.len().is_multiple_of(2)
        || !text
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(invalid("noncanonical hex"));
    }
    let bytes: Vec<u8> = text
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|p| {
            u8::from_str_radix(std::str::from_utf8(p).expect("ASCII checked"), 16)
                .expect("hex checked")
        })
        .collect();
    String::from_utf8(bytes).map_err(|_| invalid("non-UTF8 error message"))
}

#[derive(Clone)]
pub struct Identity {
    id: String,
    request: String,
    source: String,
    claimed_binary: String,
    binding: Binding,
}
impl Identity {
    pub fn new(id: &str, request: &str, source: &str, claimed_binary: &str) -> io::Result<Self> {
        let binding = Binding::new(request, source)?;
        if id.is_empty()
            || id.len() > 64
            || !id
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
            || claimed_binary.len() != 64
            || !claimed_binary
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid id/binary label",
            ));
        }
        Ok(Self {
            id: id.into(),
            request: request.into(),
            source: source.into(),
            claimed_binary: claimed_binary.into(),
            binding,
        })
    }
}

#[derive(Clone)]
pub struct Descriptor {
    identity: Identity,
    body: String,
}
impl Descriptor {
    pub fn capture(identity: Identity, command: &Command) -> io::Result<Self> {
        let args: Vec<_> = command.get_args().collect();
        if args.len() > 32 {
            return Err(invalid("too many public arguments"));
        }
        let mut body = format!(
            "attempt_id={}\nrequest_sha256={}\nsource_revision={}\nclaimed_binary_sha256={}\nprogram_hex={}\nargc={}\n",
            identity.id, identity.request, identity.source, identity.claimed_binary,
            hex(command.get_program().as_bytes()), args.len());
        for (i, arg) in args.iter().enumerate() {
            body.push_str(&format!("arg{i}_hex={}\n", hex(arg.as_bytes())));
        }
        let cwd = command
            .get_current_dir()
            .map(|p| hex(p.as_os_str().as_bytes()))
            .unwrap_or_else(|| "inherited-unbound".into());
        body.push_str(&format!("cwd_hex_or_unbound={cwd}\nenvironment=unbound\nimage=unbound\nargv0=unbound\nio=unbound\npre_exec=unbound\nexecutable_resolution=unbound\nwork_counts=unavailable\ntiming=unavailable\n"));
        if body.len() + 1024 > MAX_RECORD {
            return Err(invalid("oversize descriptor"));
        }
        Ok(Self { identity, body })
    }
    fn directory(&self, root: &Path) -> PathBuf {
        root.join(&self.identity.id)
    }
    fn record(&self, schema: &str, tail: &str) -> String {
        format!("{schema}\n{}{tail}", self.body)
    }
}

/// Synced pending bytes are published by a nonoverwriting hard link. An error
/// may occur after visible publication; no retry overwrites either file.
fn store(directory: &Path, name: &str, bytes: &str) -> io::Result<()> {
    if bytes.len() > MAX_RECORD {
        return Err(invalid("oversize record"));
    }
    let pending = directory.join(format!("{name}.pending"));
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&pending)?;
    f.write_all(bytes.as_bytes())?;
    f.sync_all()?;
    fs::hard_link(&pending, directory.join(name))?;
    File::open(directory)?.sync_all()
}
fn read(path: &Path) -> io::Result<String> {
    if !fs::symlink_metadata(path)?.is_file() {
        return Err(invalid("not regular record"));
    }
    let mut bytes = Vec::new();
    File::open(path)?
        .take((MAX_RECORD + 1) as u64)
        .read_to_end(&mut bytes)?;
    if bytes.len() > MAX_RECORD {
        return Err(invalid("oversize record"));
    }
    String::from_utf8(bytes).map_err(|_| invalid("non-UTF8 record"))
}
fn optional(path: &Path) -> io::Result<Option<String>> {
    match read(path) {
        Ok(s) => Ok(Some(s)),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExitData {
    pub pid: u32,
    pub code: Option<i32>,
    pub signal: Option<i32>,
    pub success: bool,
    pub stop_requested: bool,
}
impl ExitData {
    fn observed(pid: u32, exit: &ObservedExit) -> Self {
        Self {
            pid,
            code: exit.code(),
            signal: exit.signal(),
            success: exit.success(),
            stop_requested: exit.deadline_stop_requested(),
        }
    }
    fn outcome(&self) -> Outcome {
        if self.stop_requested {
            Outcome::Capped
        } else if self.success {
            Outcome::Completed
        } else {
            Outcome::Failed
        }
    }
    fn tail(&self) -> String {
        let number = |n: Option<i32>| {
            n.map(|x| x.to_string())
                .unwrap_or_else(|| "unavailable".into())
        };
        format!(
            "pid={}\nexit_code={}\nsignal={}\nsuccess={}\nstop_requested={}\n",
            self.pid,
            number(self.code),
            number(self.signal),
            self.success,
            self.stop_requested
        )
    }
    fn parse(tail: &str) -> io::Result<Self> {
        let mut lines = tail.lines();
        let mut value = |key: &str| -> io::Result<String> {
            lines
                .next()
                .and_then(|s| s.strip_prefix(key))
                .map(str::to_owned)
                .ok_or_else(|| invalid("missing field"))
        };
        let pid = value("pid=")?
            .parse::<u32>()
            .map_err(|_| invalid("bad pid"))?;
        let number = |s: String| {
            if s == "unavailable" {
                Ok(None)
            } else {
                s.parse::<i32>()
                    .map(Some)
                    .map_err(|_| invalid("bad exit number"))
            }
        };
        let code = number(value("exit_code=")?)?;
        let signal = number(value("signal=")?)?;
        let boolean = |s: String| match s.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(invalid("bad bool")),
        };
        let success = boolean(value("success=")?)?;
        let stop_requested = boolean(value("stop_requested=")?)?;
        let data = Self {
            pid,
            code,
            signal,
            success,
            stop_requested,
        };
        if pid == 0
            || code.is_some_and(|v| v < 0)
            || signal.is_some_and(|v| v <= 0)
            || (code.is_some() && signal.is_some())
            || success != (code == Some(0))
            || data.tail() != tail
        {
            return Err(invalid("inconsistent/noncanonical exit"));
        }
        Ok(data)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BeforeKind {
    Refused,
    GateError,
    SpawnError,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BeforeData {
    pub kind: BeforeKind,
    pub message: String,
}
impl BeforeData {
    fn tail(&self) -> String {
        let kind = match self.kind {
            BeforeKind::Refused => "refused",
            BeforeKind::GateError => "gate-error",
            BeforeKind::SpawnError => "spawn-error",
        };
        format!(
            "kind={kind}\nmessage_hex={}\n",
            hex(self.message.as_bytes())
        )
    }
    fn outcome(&self) -> Outcome {
        if self.kind == BeforeKind::Refused {
            Outcome::Refused
        } else {
            Outcome::Failed
        }
    }
    fn parse(tail: &str) -> io::Result<Self> {
        let mut lines = tail.lines();
        let kind = match lines.next() {
            Some("kind=refused") => BeforeKind::Refused,
            Some("kind=gate-error") => BeforeKind::GateError,
            Some("kind=spawn-error") => BeforeKind::SpawnError,
            _ => return Err(invalid("bad pre-child kind")),
        };
        let message = unhex(
            lines
                .next()
                .and_then(|s| s.strip_prefix("message_hex="))
                .ok_or_else(|| invalid("missing message"))?,
        )?;
        let data = Self { kind, message };
        if data.tail() != tail || (data.kind == BeforeKind::Refused && !data.message.is_empty()) {
            return Err(invalid("noncanonical pre-child record"));
        }
        Ok(data)
    }
}

pub struct Prepared {
    directory: PathBuf,
    descriptor: Descriptor,
    attempt: Attempt,
    command: Command,
}
#[derive(Debug)]
pub struct StartError {
    pub primary: io::Error,
    pub recording: Option<io::Error>,
}
pub enum Started {
    Refused,
    Running(Box<Running>),
}
impl Prepared {
    /// Capture only PUBLIC command metadata. Do not put secrets in arguments.
    /// Gate and spawn use the same owned Command after durable launch metadata.
    pub fn begin(root: &Path, identity: Identity, command: Command) -> io::Result<Self> {
        let descriptor = Descriptor::capture(identity, &command)?;
        let directory = descriptor.directory(root);
        let attempt = Attempt::begin(&directory, descriptor.identity.binding.clone())?;
        store(
            &directory,
            "launch",
            &descriptor.record("PKM_LAUNCH_V1", ""),
        )?;
        Ok(Self {
            directory,
            descriptor,
            attempt,
            command,
        })
    }
    pub fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }
    fn before(&self, data: &BeforeData) -> io::Result<()> {
        store(
            &self.directory,
            "before",
            &self.descriptor.record("PKM_BEFORE_CHILD_V1", &data.tail()),
        )?;
        self.attempt.finish(data.outcome())
    }
    fn fail(&self, kind: BeforeKind, primary: io::Error) -> StartError {
        let data = BeforeData {
            kind,
            message: primary.to_string(),
        };
        StartError {
            recording: self.before(&data).err(),
            primary,
        }
    }
    pub fn start(mut self, gate: impl FnOnce() -> io::Result<Gate>) -> Result<Started, StartError> {
        match gate() {
            Err(e) => return Err(self.fail(BeforeKind::GateError, e)),
            Ok(Gate::Refuse) => {
                self.before(&BeforeData {
                    kind: BeforeKind::Refused,
                    message: String::new(),
                })
                .map_err(|primary| StartError {
                    primary,
                    recording: None,
                })?;
                return Ok(Started::Refused);
            }
            Ok(Gate::Permit) => (),
        }
        let child = match self.command.spawn() {
            Ok(child) => child,
            Err(e) => return Err(self.fail(BeforeKind::SpawnError, e)),
        };
        Ok(Started::Running(Box::new(Running {
            directory: self.directory,
            descriptor: self.descriptor,
            attempt: self.attempt,
            child: ChildObservation::new(child),
            done: None,
        })))
    }
}

/// Retain this handle on errors. Drop does not stop/reap the child. No post-spawn
/// I/O is required to return the handle; a crash before reap stays unfinished.
pub struct Running {
    directory: PathBuf,
    descriptor: Descriptor,
    attempt: Attempt,
    child: ChildObservation,
    done: Option<ExitData>,
}
impl Running {
    fn record(&mut self, data: ExitData) -> io::Result<ExitData> {
        store(
            &self.directory,
            "reap",
            &self.descriptor.record("PKM_REAP_V1", &data.tail()),
        )?;
        self.child.publish_outcome(&self.attempt)?;
        self.done = Some(data.clone());
        Ok(data)
    }
    pub fn poll_and_record(&mut self) -> io::Result<Option<ExitData>> {
        if let Some(data) = &self.done {
            return Ok(Some(data.clone()));
        }
        let pid = self.child.child_id();
        match self.child.poll()? {
            None => Ok(None),
            Some(exit) => {
                let data = ExitData::observed(pid, exit);
                self.record(data).map(Some)
            }
        }
    }
    pub fn stop_and_record(&mut self, deadline: Instant) -> io::Result<ExitData> {
        if let Some(data) = &self.done {
            return Ok(data.clone());
        }
        self.child.request_deadline_stop(deadline)?;
        let pid = self.child.child_id();
        let data = ExitData::observed(pid, self.child.reap_after_stop()?);
        self.record(data)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct State {
    pub recorded_outcome: Option<Outcome>,
    pub reap: Option<ExitData>,
    pub before_child_handle: Option<BeforeData>,
    pub work_counts: Option<u64>,
    pub uninterrupted_wall_ns: Option<u64>,
}
/// Strict local consistency, NOT authenticated execution or durability-ack proof.
/// A published reap without an outcome stays unfinished; pending-only is ignored.
pub fn inspect(root: &Path, descriptor: &Descriptor) -> io::Result<State> {
    let dir = descriptor.directory(root);
    let old = durable_attempt::inspect(&dir, &descriptor.identity.binding)?;
    let launch = optional(&dir.join("launch"))?;
    if launch.as_deref() != Some(descriptor.record("PKM_LAUNCH_V1", "").as_str()) {
        if launch.is_none() && old == Observed::Unfinished {
            return Ok(State {
                recorded_outcome: None,
                reap: None,
                before_child_handle: None,
                work_counts: None,
                uninterrupted_wall_ns: None,
            });
        }
        return Err(invalid("launch descriptor mismatch/missing"));
    }
    let parse = |name: &str, schema: &str| -> io::Result<Option<String>> {
        optional(&dir.join(name))?
            .map(|s| {
                s.strip_prefix(&descriptor.record(schema, ""))
                    .map(str::to_owned)
                    .ok_or_else(|| invalid("event binding mismatch"))
            })
            .transpose()
    };
    let reap = parse("reap", "PKM_REAP_V1")?
        .map(|s| ExitData::parse(&s))
        .transpose()?;
    let before = parse("before", "PKM_BEFORE_CHILD_V1")?
        .map(|s| BeforeData::parse(&s))
        .transpose()?;
    if reap.is_some() && before.is_some() {
        return Err(invalid("conflicting event classes"));
    }
    let recorded_outcome = match old {
        Observed::Unfinished => None,
        Observed::Terminal(outcome) => {
            let expected = reap
                .as_ref()
                .map(ExitData::outcome)
                .or_else(|| before.as_ref().map(BeforeData::outcome));
            if expected != Some(outcome) {
                return Err(invalid("outcome lacks matching event"));
            }
            Some(outcome)
        }
    };
    Ok(State {
        recorded_outcome,
        reap,
        before_child_handle: before,
        work_counts: None,
        uninterrupted_wall_ns: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Stdio;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::Duration;

    struct Scratch(PathBuf);
    impl Scratch {
        fn new() -> Self {
            static NEXT: AtomicU64 = AtomicU64::new(0);
            for _ in 0..1000 {
                let p = std::env::temp_dir().join(format!(
                    "pkm-recorded-{}-{}",
                    std::process::id(),
                    NEXT.fetch_add(1, Ordering::Relaxed)
                ));
                match fs::create_dir(&p) {
                    Ok(()) => return Self(p),
                    Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
                    Err(e) => panic!("scratch: {e}"),
                }
            }
            panic!("scratch namespace exhausted")
        }
    }
    impl Drop for Scratch {
        fn drop(&mut self) {
            if !std::thread::panicking() {
                let _ = fs::remove_dir_all(&self.0);
            }
        }
    }
    struct Cleanup(Running);
    impl Drop for Cleanup {
        fn drop(&mut self) {
            // Fixture cleanup only, not a production Drop guarantee or journal.
            if self.0.done.is_none() {
                let _ = self.0.child.request_deadline_stop(Instant::now());
                let _ = self.0.child.reap_after_stop();
            }
        }
    }
    fn identity(id: &str) -> Identity {
        Identity::new(id, &"a".repeat(64), &"b".repeat(40), &"c".repeat(64)).unwrap()
    }
    fn command(mode: &str) -> Command {
        let fixture =
            std::env::var_os("PKM_CHILD_FIXTURE").expect("recipe must supply native fixture");
        let mut c = Command::new(fixture);
        c.arg(mode)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        c
    }
    fn running(prepared: Prepared) -> Cleanup {
        match prepared.start(|| Ok(Gate::Permit)).unwrap() {
            Started::Running(child) => Cleanup(*child),
            Started::Refused => panic!("unexpected refusal"),
        }
    }
    fn finish(child: &mut Running) -> ExitData {
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            if let Some(data) = child.poll_and_record().unwrap() {
                return data;
            }
            if Instant::now() >= deadline {
                child.stop_and_record(deadline).unwrap();
                panic!("fixture exceeded software test budget");
            }
            std::thread::sleep(Duration::from_millis(1));
        }
    }
    fn wait_for_record_error(child: &mut Running) -> io::Error {
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            match child.poll_and_record() {
                Err(e) => return e,
                Ok(Some(_)) => panic!("expected record failure"),
                Ok(None) => (),
            }
            assert!(Instant::now() < deadline, "fixture error budget exceeded");
            std::thread::sleep(Duration::from_millis(1));
        }
    }

    #[test]
    fn durable_launch_exists_before_refusal_and_no_child_handle_is_created() {
        let root = Scratch::new();
        let prepared = Prepared::begin(
            &root.0,
            identity("one"),
            Command::new("/nonexistent/pkm-never-spawn"),
        )
        .unwrap();
        let descriptor = prepared.descriptor().clone();
        let result = prepared
            .start(|| {
                assert!(root.0.join("one/launch").exists());
                assert_eq!(inspect(&root.0, &descriptor)?.recorded_outcome, None);
                Ok(Gate::Refuse)
            })
            .unwrap();
        assert!(matches!(result, Started::Refused));
        let state = inspect(&root.0, &descriptor).unwrap();
        assert_eq!(state.recorded_outcome, Some(Outcome::Refused));
        assert_eq!(state.before_child_handle.unwrap().kind, BeforeKind::Refused);
        assert_eq!(state.reap, None);
    }

    #[test]
    fn actual_success_persists_exit_and_declared_launch_binding_without_a_clock() {
        let root = Scratch::new();
        let prepared = Prepared::begin(&root.0, identity("one"), command("ok")).unwrap();
        let descriptor = prepared.descriptor().clone();
        let mut child = running(prepared);
        let data = finish(&mut child.0);
        assert_eq!(data.code, Some(0));
        assert_eq!(data.signal, None);
        assert!(!data.stop_requested);
        let before = fs::read(root.0.join("one/reap")).unwrap();
        assert_eq!(child.0.poll_and_record().unwrap(), Some(data.clone()));
        assert_eq!(fs::read(root.0.join("one/reap")).unwrap(), before);
        let state = inspect(&root.0, &descriptor).unwrap();
        assert_eq!(state.recorded_outcome, Some(Outcome::Completed));
        assert_eq!(state.reap, Some(data));
        assert_eq!(state.work_counts, None);
        assert_eq!(state.uninterrupted_wall_ns, None);
        let wrong = Descriptor::capture(identity("one"), &command("fail")).unwrap();
        assert!(inspect(&root.0, &wrong).is_err());
        let wrong = Descriptor::capture(
            Identity::new("one", &"a".repeat(64), &"b".repeat(40), &"d".repeat(64)).unwrap(),
            &command("ok"),
        )
        .unwrap();
        assert!(inspect(&root.0, &wrong).is_err());
    }

    #[test]
    fn actual_nonzero_and_signal_exit_keep_distinct_statuses() {
        let root = Scratch::new();
        let prepared = Prepared::begin(&root.0, identity("failed"), command("fail")).unwrap();
        let descriptor = prepared.descriptor().clone();
        let mut child = running(prepared);
        assert_eq!(finish(&mut child.0).code, Some(7));
        assert_eq!(
            inspect(&root.0, &descriptor).unwrap().recorded_outcome,
            Some(Outcome::Failed)
        );
        let prepared = Prepared::begin(&root.0, identity("capped"), command("hold")).unwrap();
        let descriptor = prepared.descriptor().clone();
        let mut child = running(prepared);
        assert_eq!(child.0.poll_and_record().unwrap(), None);
        assert_eq!(
            inspect(&root.0, &descriptor).unwrap().recorded_outcome,
            None
        );
        let data = child.0.stop_and_record(Instant::now()).unwrap();
        assert!(data.stop_requested);
        assert_eq!(data.code, None);
        assert!(data.signal.is_some());
        assert_eq!(
            inspect(&root.0, &descriptor).unwrap().recorded_outcome,
            Some(Outcome::Capped)
        );
    }

    #[test]
    fn published_reap_without_outcome_stays_unfinished_after_record_failure() {
        let root = Scratch::new();
        let prepared = Prepared::begin(&root.0, identity("one"), command("ok")).unwrap();
        let descriptor = prepared.descriptor().clone();
        let mut child = running(prepared);
        fs::write(
            root.0.join("one/outcome.pending"),
            b"retained prior pending",
        )
        .unwrap();
        assert_eq!(
            wait_for_record_error(&mut child.0).kind(),
            io::ErrorKind::AlreadyExists
        );
        let state = inspect(&root.0, &descriptor).unwrap();
        assert_eq!(state.recorded_outcome, None);
        assert_eq!(state.reap.unwrap().code, Some(0));
        assert_eq!(state.work_counts, None);
        assert_eq!(
            fs::read(root.0.join("one/outcome.pending")).unwrap(),
            b"retained prior pending"
        );
    }

    #[test]
    fn collision_and_transplanted_or_inconsistent_reap_are_not_accepted() {
        let root = Scratch::new();
        let prepared = Prepared::begin(&root.0, identity("old"), command("ok")).unwrap();
        let descriptor = prepared.descriptor().clone();
        let mut child = running(prepared);
        finish(&mut child.0);
        let raw = read(&root.0.join("old/reap")).unwrap();
        let prepared = Prepared::begin(&root.0, identity("new"), command("ok")).unwrap();
        let new = prepared.descriptor().clone();
        fs::write(root.0.join("new/reap"), raw.as_bytes()).unwrap();
        assert!(inspect(&root.0, &new).is_err());
        let mut child = running(prepared);
        assert_eq!(
            wait_for_record_error(&mut child.0).kind(),
            io::ErrorKind::AlreadyExists
        );
        assert_eq!(read(&root.0.join("new/reap")).unwrap(), raw);
        fs::write(
            root.0.join("old/reap"),
            raw.replace("success=true", "success=false"),
        )
        .unwrap();
        assert!(inspect(&root.0, &descriptor).is_err());
    }

    #[test]
    fn gate_and_spawn_errors_are_preserved_before_obtaining_a_child_handle() {
        let root = Scratch::new();
        let prepared = Prepared::begin(
            &root.0,
            identity("gate"),
            Command::new("/nonexistent/pkm-never-spawn"),
        )
        .unwrap();
        let descriptor = prepared.descriptor().clone();
        let error = match prepared.start(|| Err(io::Error::other("gate unavailable"))) {
            Err(e) => e,
            Ok(_) => panic!("expected gate error"),
        };
        assert_eq!(error.primary.to_string(), "gate unavailable");
        assert!(error.recording.is_none());
        let state = inspect(&root.0, &descriptor).unwrap();
        assert_eq!(state.recorded_outcome, Some(Outcome::Failed));
        assert_eq!(
            state.before_child_handle.unwrap().kind,
            BeforeKind::GateError
        );
        let prepared = Prepared::begin(
            &root.0,
            identity("spawn"),
            Command::new("/nonexistent/pkm-never-spawn"),
        )
        .unwrap();
        let descriptor = prepared.descriptor().clone();
        assert!(prepared.start(|| Ok(Gate::Permit)).is_err());
        let state = inspect(&root.0, &descriptor).unwrap();
        assert_eq!(
            state.before_child_handle.unwrap().kind,
            BeforeKind::SpawnError
        );
        assert_eq!(state.reap, None);
    }

    #[test]
    fn primary_and_recording_errors_survive_without_publishing_a_terminal() {
        let root = Scratch::new();
        let prepared = Prepared::begin(&root.0, identity("one"), command("ok")).unwrap();
        let descriptor = prepared.descriptor().clone();
        fs::write(root.0.join("one/before.pending"), b"prior").unwrap();
        let error = match prepared.start(|| Err(io::Error::other("gate unavailable"))) {
            Err(e) => e,
            Ok(_) => panic!("expected error"),
        };
        assert_eq!(error.primary.to_string(), "gate unavailable");
        assert_eq!(
            error.recording.unwrap().kind(),
            io::ErrorKind::AlreadyExists
        );
        let state = inspect(&root.0, &descriptor).unwrap();
        assert_eq!(state.recorded_outcome, None);
        assert_eq!(state.before_child_handle, None);
    }

    #[test]
    fn malformed_oversize_symlink_missing_and_pending_launch_fail_closed() {
        for mode in [
            "torn",
            "oversize",
            "symlink",
            "pending",
            "missing-event",
            "odd-error-hex",
            "non-utf8-error",
        ] {
            let root = Scratch::new();
            let prepared = Prepared::begin(&root.0, identity("one"), command("ok")).unwrap();
            let descriptor = prepared.descriptor().clone();
            match mode {
                "torn" => fs::write(root.0.join("one/launch"), b"torn").unwrap(),
                "oversize" => {
                    fs::write(root.0.join("one/launch"), vec![b'x'; MAX_RECORD + 1]).unwrap()
                }
                "symlink" => {
                    fs::remove_file(root.0.join("one/launch")).unwrap();
                    std::os::unix::fs::symlink(
                        root.0.join("one/launch.pending"),
                        root.0.join("one/launch"),
                    )
                    .unwrap();
                }
                "pending" => {
                    fs::remove_file(root.0.join("one/launch")).unwrap();
                    assert_eq!(
                        inspect(&root.0, &descriptor).unwrap().recorded_outcome,
                        None
                    );
                    continue;
                }
                "missing-event" => prepared.attempt.finish(Outcome::Completed).unwrap(),
                "odd-error-hex" | "non-utf8-error" => {
                    let bad = if mode == "odd-error-hex" { "f" } else { "ff" };
                    fs::write(
                        root.0.join("one/before"),
                        descriptor.record(
                            "PKM_BEFORE_CHILD_V1",
                            &format!("kind=gate-error\nmessage_hex={bad}\n"),
                        ),
                    )
                    .unwrap();
                }
                _ => unreachable!(),
            }
            assert!(inspect(&root.0, &descriptor).is_err());
        }
    }

    #[test]
    fn invalid_identity_oversize_command_or_duplicate_id_cannot_overwrite() {
        let root = Scratch::new();
        assert!(
            Identity::new("../bad", &"a".repeat(64), &"b".repeat(40), &"c".repeat(64)).is_err()
        );
        assert!(Identity::new("good", &"a".repeat(64), &"b".repeat(40), "invalid").is_err());
        let mut c = command("ok");
        c.arg("x".repeat(MAX_RECORD));
        assert!(Prepared::begin(&root.0, identity("oversize"), c).is_err());
        assert!(!root.0.join("oversize").exists());
        let _ = Prepared::begin(&root.0, identity("one"), command("ok")).unwrap();
        let old = fs::read(root.0.join("one/launch")).unwrap();
        assert!(Prepared::begin(&root.0, identity("one"), command("fail")).is_err());
        assert_eq!(fs::read(root.0.join("one/launch")).unwrap(), old);
    }
}
