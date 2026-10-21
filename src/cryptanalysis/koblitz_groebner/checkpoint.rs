//! Application checkpoints for bounded Macaulay diagnostics. These contain
//! mathematical state, not process memory, and are portable across x86 CPUs.
//! Immutable generations use the managed worker's manifest-sha256-v1 contract.

use super::*;
use crate::cryptanalysis::sparse_macaulay::{low_column_start, SparseProgress};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Write};
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

static INTERRUPTED: AtomicBool = AtomicBool::new(false);
extern "C" fn request_checkpoint(_: libc::c_int) {
    INTERRUPTED.store(true, Ordering::Relaxed);
}

type Answer = (Option<u32>, Vec<SolvingProfile>);

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

#[derive(Default, Serialize, Deserialize)]
struct Catalog {
    version: u32,
    completed: BTreeMap<String, Answer>,
    active: Option<Active>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Active {
    input: String,
    degree: u32,
    profiles: Vec<SolvingProfile>,
    cols: Vec<u64>,
    n_rows: usize,
}

#[derive(Serialize, Deserialize)]
struct Entry {
    path: String,
    sha256: String,
    bytes: u64,
}

#[derive(Serialize, Deserialize)]
struct Manifest {
    schema: String,
    job_id: String,
    spec_sha256: String,
    generation: u64,
    committed: bool,
    files: Vec<Entry>,
}

struct HashWriter {
    file: File,
    hash: Sha256,
    bytes: u64,
}
impl Write for HashWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let n = self.file.write(bytes)?;
        self.hash.update(&bytes[..n]);
        self.bytes += n as u64;
        Ok(n)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}
fn write_file(
    dir: &Path,
    name: &str,
    write: impl FnOnce(&mut BufWriter<HashWriter>) -> io::Result<()>,
) -> io::Result<Entry> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    // Owner-only where the platform has Unix permissions.
    #[cfg(unix)]
    options.mode(0o600);
    let file = options.open(dir.join(name))?;
    let hash_writer = HashWriter {
        file,
        hash: Sha256::new(),
        bytes: 0,
    };
    // Buffer above the hash writer: update SHA-256 in large blocks rather
    // than once per four-byte matrix index.
    let mut writer = BufWriter::with_capacity(1024 * 1024, hash_writer);
    write(&mut writer)?;
    writer.flush()?;
    let writer = writer.into_inner().map_err(|e| e.into_error())?;
    writer.file.sync_all()?;
    Ok(Entry {
        path: name.into(),
        sha256: format!("{:x}", writer.hash.finalize()),
        bytes: writer.bytes,
    })
}
fn put(w: &mut impl Write, n: usize) -> io::Result<()> {
    w.write_all(&(n as u64).to_le_bytes())
}
fn number(r: &mut impl Read) -> io::Result<usize> {
    let mut b = [0; 8];
    r.read_exact(&mut b)?;
    usize::try_from(u64::from_le_bytes(b)).map_err(|_| invalid("checkpoint size overflow"))
}
fn write_matrix(w: &mut impl Write, s: &SparseProgress) -> io::Result<()> {
    w.write_all(b"SPARSE01")?;
    for n in [
        s.n_cols,
        s.low_start,
        s.next_column,
        s.high_rank,
        s.vanished,
        s.max_weight,
        s.rows.len(),
    ] {
        put(w, n)?;
    }
    for row in &s.rows {
        put(w, row.len())?;
        for n in row {
            w.write_all(&n.to_le_bytes())?;
        }
    }
    for pivot in &s.is_pivot {
        w.write_all(&[u8::from(*pivot)])?;
    }
    put(w, s.buckets.len())?;
    for bucket in &s.buckets {
        put(w, bucket.len())?;
        for n in bucket {
            put(w, *n)?;
        }
    }
    Ok(())
}
fn read_matrix(path: &Path, active: &Active) -> io::Result<SparseProgress> {
    let file = File::open(path)?;
    let bytes = file.metadata()?.len() as usize;
    let mut r = BufReader::with_capacity(1024 * 1024, file);
    let mut magic = [0; 8];
    r.read_exact(&mut magic)?;
    if &magic != b"SPARSE01" {
        return Err(invalid("unsupported matrix checkpoint"));
    }
    let n_cols = number(&mut r)?;
    let low_start = number(&mut r)?;
    let next_column = number(&mut r)?;
    let high_rank = number(&mut r)?;
    let vanished = number(&mut r)?;
    let max_weight = number(&mut r)?;
    let n_rows = number(&mut r)?;
    if n_cols != active.cols.len()
        || n_rows != active.n_rows
        || low_start != low_column_start(&active.cols)
        || next_column > low_start
        || n_rows > bytes / 9
        || high_rank > n_rows
        || vanished > n_rows
    {
        return Err(invalid("matrix checkpoint dimensions disagree"));
    }
    let mut rows = Vec::with_capacity(n_rows);
    let mut indices_left = bytes / 4;
    for _ in 0..n_rows {
        let count = number(&mut r)?;
        if count > n_cols || count > indices_left {
            return Err(invalid("invalid row size"));
        }
        indices_left -= count;
        let mut row = Vec::with_capacity(count);
        for _ in 0..count {
            let mut b = [0; 4];
            r.read_exact(&mut b)?;
            row.push(u32::from_le_bytes(b));
        }
        if row.iter().any(|n| *n as usize >= n_cols) || row.windows(2).any(|p| p[0] >= p[1]) {
            return Err(invalid("noncanonical checkpoint row"));
        }
        rows.push(row);
    }
    let mut flags = vec![0; n_rows];
    r.read_exact(&mut flags)?;
    if flags.iter().any(|f| *f > 1) || flags.iter().filter(|f| **f == 1).count() != high_rank {
        return Err(invalid("invalid checkpoint pivots"));
    }
    let is_pivot: Vec<bool> = flags.into_iter().map(|f| f == 1).collect();
    if number(&mut r)? != n_cols.max(1) {
        return Err(invalid("invalid bucket count"));
    }
    let mut buckets = Vec::with_capacity(n_cols.max(1));
    let mut seen = vec![false; n_rows];
    for c in 0..n_cols.max(1) {
        let count = number(&mut r)?;
        if count > n_rows || (c < next_column && count != 0) {
            return Err(invalid("invalid checkpoint bucket"));
        }
        let mut bucket = Vec::with_capacity(count);
        for _ in 0..count {
            let i = number(&mut r)?;
            if i >= n_rows
                || seen[i]
                || is_pivot[i]
                || rows[i].first().map(|v| *v as usize) != Some(c)
            {
                return Err(invalid("checkpoint bucket membership disagrees"));
            }
            seen[i] = true;
            bucket.push(i);
        }
        buckets.push(bucket);
    }
    if rows
        .iter()
        .enumerate()
        .any(|(i, row)| !row.is_empty() && !is_pivot[i] && !seen[i])
        || rows.iter().filter(|row| row.is_empty()).count() != vanished
        || r.read(&mut [0; 1])? != 0
    {
        return Err(invalid("incomplete checkpoint state"));
    }
    Ok(SparseProgress {
        rows,
        buckets,
        is_pivot,
        next_column,
        n_cols,
        low_start,
        high_rank,
        vanished,
        max_weight,
    })
}

/// A single-writer checkpoint store. The supervisor owns the outer job lock.
pub struct Store {
    root: PathBuf,
    job: String,
    spec: String,
    generation: u64,
    catalog: Catalog,
    resumed: Option<SparseProgress>,
    interval: Duration,
    last_save: Instant,
    /// True when the last solve reused a completed target's profiles.
    pub replayed: bool,
}

impl Store {
    /// Read the managed supervisor environment. With no checkpoint directory,
    /// the example retains its original execution path.
    pub fn from_env() -> io::Result<Option<Self>> {
        let Some(root) = std::env::var_os("SPOT_CHECKPOINT_DIR") else {
            return Ok(None);
        };
        let root = fs::canonicalize(root)?;
        let spec: serde_json::Value = serde_json::from_reader(File::open(
            root.parent()
                .ok_or_else(|| invalid("missing job state"))?
                .join("spec.json"),
        )?)?;
        let job = spec["spec"]["id"]
            .as_str()
            .ok_or_else(|| invalid("missing job id"))?;
        if spec["spec"]["checkpoint_contract"].as_str() != Some("manifest-sha256-v1") {
            return Err(invalid(
                "factor-base native checkpoints require manifest-sha256-v1",
            ));
        }
        let digest = spec["sha256"]
            .as_str()
            .ok_or_else(|| invalid("missing job digest"))?;
        let interval = std::env::var("SPOT_CHECKPOINT_SECONDS")
            .unwrap_or_else(|_| "300".into())
            .parse::<u64>()
            .map_err(|_| invalid("invalid checkpoint interval"))?;
        if interval == 0 {
            return Err(invalid("checkpoint interval must be positive"));
        }
        let selected = std::env::var_os("SPOT_CHECKPOINT_MANIFEST").map(PathBuf::from);
        // The handler performs no I/O or allocation. The elimination loop saves
        // at its next complete-column boundary. A hard kill uses the last commit.
        unsafe {
            libc::signal(
                libc::SIGTERM,
                request_checkpoint as *const () as libc::sighandler_t,
            );
        }
        Ok(Some(Self::open(
            root,
            job.into(),
            digest.into(),
            selected.as_deref(),
            Duration::from_secs(interval),
        )?))
    }

    fn open(
        root: PathBuf,
        job: String,
        spec: String,
        selected: Option<&Path>,
        interval: Duration,
    ) -> io::Result<Self> {
        if sparse_dense_finish() {
            return Err(invalid("native matrix checkpoints require KIC_SPARSE_DENSE_FINISH unset; dense finish is not checkpointed"));
        }
        let mut store = Self {
            root,
            job,
            spec,
            generation: 0,
            catalog: Catalog {
                version: 1,
                ..Default::default()
            },
            resumed: None,
            interval,
            last_save: Instant::now(),
            replayed: false,
        };
        // Partial/uncommitted generations consume an id but cannot be restored.
        for entry in fs::read_dir(&store.root)? {
            let name = entry?.file_name();
            if let Some(n) = name
                .to_str()
                .and_then(|s| s.strip_prefix("gen-"))
                .and_then(|s| s.parse::<u64>().ok())
            {
                store.generation = store.generation.max(n);
            }
        }
        if let Some(path) = selected {
            let path = fs::canonicalize(path)?;
            if !path.starts_with(&store.root) {
                return Err(invalid("selected checkpoint is outside the job"));
            }
            let manifest: Manifest = serde_json::from_reader(File::open(&path)?)?;
            if manifest.schema != "spot.checkpoint/v1"
                || !manifest.committed
                || manifest.job_id != store.job
                || manifest.spec_sha256 != store.spec
            {
                return Err(invalid("checkpoint identity disagrees"));
            }
            let dir = path.parent().unwrap();
            let names: Vec<_> = manifest.files.iter().map(|e| e.path.as_str()).collect();
            if names != ["catalog.json"] && names != ["catalog.json", "matrix.bin"] {
                return Err(invalid("unexpected checkpoint files"));
            }
            for entry in &manifest.files {
                let p = dir.join(&entry.path);
                if fs::symlink_metadata(&p)?.file_type().is_symlink() {
                    return Err(invalid("checkpoint symlink"));
                }
                let mut f = File::open(p)?;
                let mut hash = Sha256::new();
                let mut buf = [0; 65536];
                let mut total = 0;
                loop {
                    let n = f.read(&mut buf)?;
                    if n == 0 {
                        break;
                    }
                    hash.update(&buf[..n]);
                    total += n as u64;
                }
                if total != entry.bytes || format!("{:x}", hash.finalize()) != entry.sha256 {
                    return Err(invalid("checkpoint checksum disagrees"));
                }
            }
            store.catalog =
                serde_json::from_reader(BufReader::new(File::open(dir.join("catalog.json"))?))?;
            if store.catalog.version != 1 {
                return Err(invalid("unsupported checkpoint catalog"));
            }
            if let Some(active) = &store.catalog.active {
                if names.len() != 2 {
                    return Err(invalid("missing matrix state"));
                }
                let state = read_matrix(&dir.join("matrix.bin"), active)?;
                eprintln!(
                    "Restoring native checkpoint: degree={}, next_column={}",
                    active.degree, state.next_column
                );
                store.resumed = Some(state);
            } else if names.len() != 1 {
                return Err(invalid("unexpected matrix state"));
            }
        } else if std::env::var("SPOT_RESUMED").as_deref() == Ok("1") {
            return Err(invalid("resume requested without a validated manifest"));
        }
        Ok(store)
    }

    fn save(&mut self, state: Option<&SparseProgress>) -> io::Result<()> {
        self.generation = self
            .generation
            .checked_add(1)
            .ok_or_else(|| invalid("generation overflow"))?;
        let dir = self.root.join(format!("gen-{:020}", self.generation));
        fs::create_dir(&dir)?;
        let mut files = vec![write_file(&dir, "catalog.json", |w| {
            serde_json::to_writer(w, &self.catalog)?;
            Ok(())
        })?];
        if let Some(state) = state {
            files.push(write_file(&dir, "matrix.bin", |w| write_matrix(w, state))?);
        }
        let manifest = Manifest {
            schema: "spot.checkpoint/v1".into(),
            job_id: self.job.clone(),
            spec_sha256: self.spec.clone(),
            generation: self.generation,
            committed: true,
            files,
        };
        write_file(&dir, "manifest.tmp", |w| {
            serde_json::to_writer(w, &manifest)?;
            Ok(())
        })?;
        File::open(&dir)?.sync_all()?;
        fs::rename(
            dir.join("manifest.tmp"),
            dir.join("checkpoint-manifest.json"),
        )?;
        File::open(&dir)?.sync_all()?;
        File::open(&self.root)?.sync_all()?;
        self.last_save = Instant::now();
        #[cfg(test)]
        if self.generation == 2
            && std::env::var("SPOT_NATIVE_PAUSE_AFTER_SECOND_COMMIT").as_deref() == Ok("1")
        {
            std::thread::sleep(Duration::from_secs(60));
        }
        // Keep two previous committed generations for corruption fallback.
        // Uncommitted directories are deliberately retained as failure evidence.
        let mut committed = Vec::new();
        for entry in fs::read_dir(&self.root)? {
            let p = entry?.path();
            if p.file_name()
                .and_then(|s| s.to_str())
                .is_some_and(|s| s.starts_with("gen-"))
                && p.join("checkpoint-manifest.json").is_file()
            {
                committed.push(p);
            }
        }
        committed.sort();
        let excess = committed.len().saturating_sub(3);
        for p in committed.into_iter().take(excess) {
            fs::remove_dir_all(p)?;
        }
        File::open(&self.root)?.sync_all()?;
        Ok(())
    }

    /// Resume or compute one deterministic target's degree scan. Completed
    /// targets and lower degrees are replayed from the catalog; the active
    /// degree resumes at an exact pivot-column boundary.
    pub fn solve(
        &mut self,
        polys: &[F2BoolPoly],
        n_vars: usize,
        d_min: u32,
        d_max: u32,
    ) -> io::Result<Answer> {
        if INTERRUPTED.load(Ordering::Relaxed) {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "worker draining; restore last checkpoint",
            ));
        }
        let identity = serde_json::to_vec(&(1u32, polys, n_vars, d_min, d_max, sparse_f5()))?;
        let input = blake3::hash(&identity).to_hex().to_string();
        self.replayed = false;
        if let Some(answer) = self.catalog.completed.get(&input) {
            self.replayed = true;
            return Ok(answer.clone());
        }
        if self
            .catalog
            .active
            .as_ref()
            .is_some_and(|a| a.input != input)
        {
            return Err(invalid("replayed target differs from the active checkpoint; preserve arguments, binary and RNG implementation"));
        }
        let mut profiles = self
            .catalog
            .active
            .as_ref()
            .map(|a| a.profiles.clone())
            .unwrap_or_default();
        let start = self
            .catalog
            .active
            .as_ref()
            .map(|a| a.degree)
            .unwrap_or(system_degree(polys).max(1).max(d_min));
        let mut solved = None;
        for degree in start..=d_max {
            let (cols, n_rows, mut state) = if let Some(active) = self.catalog.active.take() {
                (
                    active.cols,
                    active.n_rows,
                    self.resumed
                        .take()
                        .ok_or_else(|| invalid("missing active matrix"))?,
                )
            } else {
                let built = if sparse_f5() {
                    build_macaulay_sparse_f5(polys, n_vars, degree)
                } else {
                    build_macaulay_sparse(polys, n_vars, degree)
                };
                let Some((cols, rows)) = built else {
                    break;
                };
                let n_rows = rows.len();
                let state = SparseProgress::new(rows, cols.len(), low_column_start(&cols));
                (cols, n_rows, state)
            };
            self.catalog.active = Some(Active {
                input: input.clone(),
                degree,
                profiles: profiles.clone(),
                cols: cols.clone(),
                n_rows,
            });
            self.save(Some(&state))?;
            let elim = state.finish(|state| {
                let stopping = INTERRUPTED.load(Ordering::Relaxed);
                if stopping || self.last_save.elapsed() >= self.interval {
                    self.save(Some(state))?;
                }
                if stopping {
                    return Err(io::Error::new(
                        io::ErrorKind::Interrupted,
                        "checkpoint committed for worker drain",
                    ));
                }
                Ok::<_, io::Error>(())
            })?;
            let profile =
                finish_sparse_profile(polys, degree, &cols, n_rows, low_column_start(&cols), elim);
            if profile.resolves() {
                solved = Some(degree);
            }
            profiles.push(profile);
            self.catalog.active = None;
            if solved.is_some() {
                break;
            }
            // Lower degrees must be durable before building the next matrix.
            // The next snapshot contains them with the next degree's state.
        }
        let answer = (solved, profiles);
        self.catalog.completed.insert(input, answer.clone());
        self.catalog.active = None;
        self.save(None)?;
        Ok(answer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::{Command, Stdio};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn fixture() -> Vec<F2BoolPoly> {
        use crate::cryptanalysis::pq_groebner_f2::F2BoolMono;
        [vec![3, 4], vec![6, 1], vec![5, 0]]
            .into_iter()
            .map(|m| F2BoolPoly::from_monos(m.into_iter().map(F2BoolMono::from_mask).collect(), 3))
            .collect()
    }
    fn dir() -> PathBuf {
        let parent = std::env::var_os("SPOT_NATIVE_TEST_PARENT")
            .map(PathBuf::from)
            .unwrap_or_else(std::env::temp_dir);
        let p = parent.join(format!(
            ".solver-checkpoint-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&p).unwrap();
        p
    }
    fn selected(root: &Path) -> PathBuf {
        let mut paths: Vec<_> = fs::read_dir(root)
            .unwrap()
            .map(|e| e.unwrap().path().join("checkpoint-manifest.json"))
            .filter(|p| p.is_file())
            .collect();
        paths.sort();
        paths.pop().unwrap()
    }

    #[test]
    fn exact_pivot_state_survives_binary_roundtrip_at_every_column() {
        let root = dir();
        let polys = fixture();
        let (cols, rows) = build_macaulay_sparse(&polys, 3, 3).unwrap();
        let low_start = low_column_start(&cols);
        let expected = crate::cryptanalysis::sparse_macaulay::eliminate_high_columns(
            rows.clone(),
            cols.len(),
            low_start,
        );
        for boundary in 1..=low_start {
            let mut state = SparseProgress::new(rows.clone(), cols.len(), low_start);
            let result = state.finish(|s| {
                if s.next_column == boundary {
                    Err(())
                } else {
                    Ok(())
                }
            });
            assert!(result.is_err());
            let active = Active {
                input: "fixture".into(),
                degree: 3,
                profiles: vec![],
                cols: cols.clone(),
                n_rows: rows.len(),
            };
            let entry = write_file(&root, "matrix.bin", |w| write_matrix(w, &state)).unwrap();
            assert!(entry.bytes > 0);
            let mut restored = read_matrix(&root.join("matrix.bin"), &active).unwrap();
            let actual = restored.finish(|_| Ok::<_, ()>(())).unwrap();
            assert_eq!(actual.high_rank, expected.high_rank);
            assert_eq!(actual.linear_rows, expected.linear_rows);
            assert_eq!(actual.vanished, expected.vanished);
            assert_eq!(actual.max_weight, expected.max_weight);
            fs::remove_file(root.join("matrix.bin")).unwrap();
        }
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn solver_checkpoint_child() {
        let Some(root) = std::env::var_os("SPOT_NATIVE_CHILD_ROOT").map(PathBuf::from) else {
            return;
        };
        let path = std::env::var_os("SPOT_NATIVE_CHILD_MANIFEST").map(PathBuf::from);
        let mut store = Store::open(
            root.clone(),
            "probe".into(),
            "a".repeat(64),
            path.as_deref(),
            Duration::ZERO,
        )
        .unwrap();
        let answer = store.solve(&fixture(), 3, 1, 3).unwrap();
        assert_eq!(answer, solving_degree_from(&fixture(), 3, 1, 3));
        fs::write(
            root.join("answer.json"),
            serde_json::to_vec(&answer).unwrap(),
        )
        .unwrap();
    }

    #[test]
    fn real_solver_recovers_after_sigkill_and_rejects_corruption() {
        let root = dir();
        let mut child = Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "cryptanalysis::koblitz_groebner::checkpoint::tests::solver_checkpoint_child",
                "--nocapture",
            ])
            .env("SPOT_NATIVE_CHILD_ROOT", &root)
            .env("SPOT_NATIVE_PAUSE_AFTER_SECOND_COMMIT", "1")
            .env_remove("SPOT_NATIVE_CHILD_MANIFEST")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let manifest = root.join("gen-00000000000000000002/checkpoint-manifest.json");
        let deadline = Instant::now() + Duration::from_secs(30);
        while !manifest.is_file() {
            assert!(
                Instant::now() < deadline,
                "child did not produce an inner-elimination checkpoint"
            );
            assert!(
                child.try_wait().unwrap().is_none(),
                "child exited before checkpoint"
            );
            std::thread::sleep(Duration::from_millis(10));
        }
        let catalog: Catalog = serde_json::from_reader(
            File::open(manifest.parent().unwrap().join("catalog.json")).unwrap(),
        )
        .unwrap();
        let state = read_matrix(
            &manifest.parent().unwrap().join("matrix.bin"),
            catalog.active.as_ref().unwrap(),
        )
        .unwrap();
        assert_eq!(
            state.next_column, 1,
            "test must kill after a pivot-column safe point"
        );
        child.kill().unwrap();
        child.wait().unwrap();
        let status = Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "cryptanalysis::koblitz_groebner::checkpoint::tests::solver_checkpoint_child",
                "--nocapture",
            ])
            .env("SPOT_NATIVE_CHILD_ROOT", &root)
            .env("SPOT_NATIVE_CHILD_MANIFEST", &manifest)
            .env_remove("SPOT_NATIVE_PAUSE_AFTER_SECOND_COMMIT")
            .status()
            .unwrap();
        assert!(status.success());
        let answer: Answer =
            serde_json::from_reader(File::open(root.join("answer.json")).unwrap()).unwrap();
        assert_eq!(answer, solving_degree_from(&fixture(), 3, 1, 3));
        let latest = selected(&root);
        let mut resumed = Store::open(
            root.clone(),
            "probe".into(),
            "a".repeat(64),
            Some(&latest),
            Duration::ZERO,
        )
        .unwrap();
        assert_eq!(resumed.solve(&fixture(), 3, 1, 3).unwrap(), answer);
        assert!(resumed.replayed);
        assert!(Store::open(
            root.clone(),
            "other-job".into(),
            "a".repeat(64),
            Some(&latest),
            Duration::ZERO
        )
        .is_err());
        fs::write(latest.parent().unwrap().join("catalog.json"), b"corrupt").unwrap();
        assert!(Store::open(
            root.clone(),
            "probe".into(),
            "a".repeat(64),
            Some(&latest),
            Duration::ZERO
        )
        .is_err());
        fs::remove_dir_all(root).unwrap();
    }
}
