//! The trial store: one append-only JSONL file per study.
//!
//! Line 1 is the header (everything that has to match for a resumed study to
//! mean the same thing).  Then one `baseline` line per rung the frozen
//! baseline was scored on, and one `trial` line per finished trial, written
//! and flushed as it finishes.  Nothing is ever rewritten: a crash loses at
//! most the trial in flight, and a torn final line (a write the crash cut
//! short) is dropped on the next open rather than poisoning the study.
//!
//! Resume is `open` on the same path with the same settings: the history
//! comes back, ids continue, and the sampler sees every earlier trial.

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::space::Assignment;
use super::trial::{Band, Class, TrialRecord};

pub const SCHEMA: &str = "ic.sweep-store/v1";

/// What must be equal for two runs to be the same study.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Header {
    pub schema: String,
    pub space: String,
    pub track: String,
    /// Whether results on this track count towards an improvement claim.
    pub counted: bool,
    pub instance: String,
    pub rungs: Vec<u32>,
    pub target_log2_r: f64,
    pub min_fit: usize,
    pub repeats: usize,
    pub seed: u64,
    pub max_trials: u64,
    pub rho_runs: usize,
    /// Provenance, not identity: not compared on resume.
    #[serde(default)]
    pub git_commit: Option<String>,
    #[serde(default)]
    pub binary_blake3: Option<String>,
}

impl Header {
    /// The fields that identify a study, as `(name, this, other)` for each
    /// that differs.
    pub fn mismatches(&self, other: &Header) -> Vec<String> {
        let mut out = Vec::new();
        macro_rules! check {
            ($($f:ident),*) => {$(
                if self.$f != other.$f {
                    out.push(format!("{}: store has {:?}, this run has {:?}", stringify!($f), self.$f, other.$f));
                }
            )*};
        }
        check!(
            schema, space, track, counted, instance, rungs, min_fit, repeats, seed, max_trials,
            rho_runs
        );
        if (self.target_log2_r - other.target_log2_r).abs() > 1e-9 {
            out.push(format!(
                "target_log2_r: store has {}, this run has {}",
                self.target_log2_r, other.target_log2_r
            ));
        }
        out
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BaselineLine {
    pub scale: u32,
    pub record: TrialRecord,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RungLine {
    pub scale: u32,
    pub class: Class,
    pub detail: String,
    pub log2_r: Option<f64>,
    pub band: Option<Band>,
    /// Worst `S_trial / S_baseline` over the band.
    pub ratio: Option<f64>,
    pub log2_ratio: Option<f64>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TrialState {
    Complete,
    Pruned,
    Failed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrialLine {
    pub id: usize,
    pub assignment: Assignment,
    pub state: TrialState,
    pub rungs: Vec<RungLine>,
    /// `log2` of the projected worst-case ratio; `None` unless complete.
    pub objective: Option<f64>,
    /// Whether `objective` is a fit read at the target size.
    pub projected: bool,
    pub slope: Option<f64>,
    pub note: String,
}

/// One line of the store.
///
/// Written and read through `serde_json::Value`, not an internally tagged
/// enum: the workspace enables serde_json's `arbitrary_precision`, under
/// which serde's tag buffering turns every number into a map and no `f64`
/// field parses.
#[derive(Clone, Debug)]
pub enum Line {
    Header(Header),
    Baseline(BaselineLine),
    Trial(TrialLine),
}

impl Line {
    fn to_value(&self) -> Result<serde_json::Value, String> {
        let (kind, mut v) = match self {
            Line::Header(h) => ("header", serde_json::to_value(h)),
            Line::Baseline(b) => ("baseline", serde_json::to_value(b)),
            Line::Trial(t) => ("trial", serde_json::to_value(t)),
        };
        let v = v.as_mut().map_err(|e| e.to_string())?;
        v["kind"] = kind.into();
        Ok(std::mem::take(v))
    }

    fn parse(bytes: &[u8]) -> Result<Line, String> {
        let v: serde_json::Value = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
        let kind = v
            .get("kind")
            .and_then(|k| k.as_str())
            .unwrap_or("")
            .to_string();
        let bad = |e: serde_json::Error| e.to_string();
        match kind.as_str() {
            "header" => serde_json::from_value(v).map(Line::Header).map_err(bad),
            "baseline" => serde_json::from_value(v).map(Line::Baseline).map_err(bad),
            "trial" => serde_json::from_value(v).map(Line::Trial).map_err(bad),
            other => Err(format!("unknown store line kind `{other}`")),
        }
    }
}

pub struct Store {
    path: PathBuf,
    file: File,
    pub baselines: Vec<BaselineLine>,
    pub trials: Vec<TrialLine>,
}

impl Store {
    /// Create the store, or resume it.  A store written for a different
    /// study is an error, never silently extended.
    pub fn open(path: &Path, expected: &Header) -> Result<Store, String> {
        if !path.exists() {
            let mut file = OpenOptions::new()
                .create_new(true)
                .append(true)
                .open(path)
                .map_err(|e| format!("cannot create {}: {e}", path.display()))?;
            write_line(&mut file, &Line::Header(expected.clone()))?;
            return Ok(Store {
                path: path.into(),
                file,
                baselines: vec![],
                trials: vec![],
            });
        }
        let (lines, torn_at) = read_lines(path)?;
        let mut it = lines.into_iter();
        let header = match it.next() {
            Some(Line::Header(h)) => h,
            _ => {
                return Err(format!(
                    "{} does not start with a study header",
                    path.display()
                ))
            }
        };
        let bad = header.mismatches(expected);
        if !bad.is_empty() {
            return Err(format!(
                "{} belongs to a different study:\n  {}",
                path.display(),
                bad.join("\n  ")
            ));
        }
        let (mut baselines, mut trials) = (vec![], vec![]);
        for line in it {
            match line {
                Line::Baseline(b) => baselines.push(b),
                Line::Trial(t) => trials.push(t),
                Line::Header(_) => {
                    return Err("a second study header in the middle of the store".into())
                }
            }
        }
        let file = OpenOptions::new()
            .append(true)
            .open(path)
            .map_err(|e| format!("cannot append to {}: {e}", path.display()))?;
        if torn_at.is_none()
            && std::fs::read(path)
                .map_err(|e| e.to_string())?
                .last()
                .is_some_and(|b| *b != b'\n')
        {
            let mut f = &file;
            f.write_all(b"\n").map_err(|e| e.to_string())?;
        }
        if let Some(valid_len) = torn_at {
            // Cut the fragment off, so the next line starts clean and the
            // store never carries a bad line in its middle.
            file.set_len(valid_len).map_err(|e| e.to_string())?;
        }
        Ok(Store {
            path: path.into(),
            file,
            baselines,
            trials,
        })
    }

    /// Read a store without opening it for writing (and so without dropping
    /// a torn last line): the header, the baselines and the trials.
    pub fn read(path: &Path) -> Result<(Header, Vec<BaselineLine>, Vec<TrialLine>), String> {
        let (lines, _) = read_lines(path)?;
        let mut it = lines.into_iter();
        let header = match it.next() {
            Some(Line::Header(h)) => h,
            _ => {
                return Err(format!(
                    "{} does not start with a study header",
                    path.display()
                ))
            }
        };
        let (mut baselines, mut trials) = (vec![], vec![]);
        for line in it {
            match line {
                Line::Baseline(b) => baselines.push(b),
                Line::Trial(t) => trials.push(t),
                Line::Header(_) => {
                    return Err("a second study header in the middle of the store".into())
                }
            }
        }
        Ok((header, baselines, trials))
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn append_baseline(&mut self, line: BaselineLine) -> Result<(), String> {
        write_line(&mut self.file, &Line::Baseline(line.clone()))?;
        self.baselines.push(line);
        Ok(())
    }

    pub fn append_trial(&mut self, line: TrialLine) -> Result<(), String> {
        write_line(&mut self.file, &Line::Trial(line.clone()))?;
        self.trials.push(line);
        Ok(())
    }

    pub fn next_id(&self) -> usize {
        self.trials.iter().map(|t| t.id + 1).max().unwrap_or(0)
    }
}

fn write_line(file: &mut File, line: &Line) -> Result<(), String> {
    let mut text = serde_json::to_string(&line.to_value()?).map_err(|e| e.to_string())?;
    text.push('\n');
    file.write_all(text.as_bytes()).map_err(|e| e.to_string())?;
    file.sync_data().map_err(|e| e.to_string())
}

/// Every parsable line, and, if the last line was torn (cut off by a
/// crash), the byte length of the file up to the start of that fragment.  A
/// bad line anywhere else is corruption and an error.
fn read_lines(path: &Path) -> Result<(Vec<Line>, Option<u64>), String> {
    let bytes = std::fs::read(path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    let mut out = Vec::new();
    let mut torn = None;
    let mut offset = 0usize;
    let pieces: Vec<&[u8]> = bytes.split(|b| *b == b'\n').collect();
    // `split` yields a final empty piece after a trailing newline.
    let last = pieces.iter().rposition(|p| !p.is_empty());
    for (i, piece) in pieces.iter().enumerate() {
        let start = offset;
        offset += piece.len() + 1;
        if piece.is_empty() {
            continue;
        }
        match Line::parse(piece) {
            Ok(line) => out.push(line),
            Err(_) if Some(i) == last => torn = Some(start as u64),
            Err(e) => {
                return Err(format!(
                    "{}:{}: corrupt store line: {e}",
                    path.display(),
                    i + 1
                ));
            }
        }
    }
    Ok((out, torn))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn header() -> Header {
        Header {
            schema: SCHEMA.into(),
            space: "koblitz-algebraic".into(),
            track: "algebraic".into(),
            counted: true,
            instance: "koblitz a=0".into(),
            rungs: vec![13, 17, 19, 23],
            target_log2_r: 81.0,
            min_fit: 4,
            repeats: 2,
            seed: 7,
            max_trials: 1000,
            rho_runs: 4,
            git_commit: Some("abc".into()),
            binary_blake3: None,
        }
    }

    fn trial(id: usize) -> TrialLine {
        let mut a = Assignment::new();
        a.insert("solver".into(), json!("inherited-f4"));
        a.insert("dim_frac".into(), json!(0.31));
        TrialLine {
            id,
            assignment: a,
            state: TrialState::Complete,
            rungs: vec![],
            objective: Some(-0.25),
            projected: true,
            slope: Some(0.01),
            note: String::new(),
        }
    }

    fn tmp(name: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("ic-sweep-store-{}-{name}", std::process::id()));
        let _ = std::fs::remove_file(&dir);
        dir
    }

    #[test]
    fn a_store_round_trips_and_resumes() {
        let path = tmp("roundtrip");
        {
            let mut s = Store::open(&path, &header()).unwrap();
            s.append_trial(trial(0)).unwrap();
            s.append_trial(trial(1)).unwrap();
            assert_eq!(s.next_id(), 2);
        }
        let s = Store::open(&path, &header()).unwrap();
        assert_eq!(s.trials.len(), 2);
        assert_eq!(s.trials[1].assignment["dim_frac"], json!(0.31));
        assert_eq!(s.next_id(), 2, "ids continue after a resume");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn a_store_for_a_different_study_is_refused() {
        let path = tmp("mismatch");
        Store::open(&path, &header()).unwrap();
        let mut other = header();
        other.rungs = vec![13, 17];
        other.seed = 8;
        let err = Store::open(&path, &other).err().unwrap();
        assert!(err.contains("rungs") && err.contains("seed"), "{err}");
        // Provenance is not identity: a different commit may resume.
        let mut rebuilt = header();
        rebuilt.git_commit = Some("def".into());
        assert!(Store::open(&path, &rebuilt).is_ok());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn a_torn_last_line_is_dropped_and_the_study_continues() {
        let path = tmp("torn");
        {
            let mut s = Store::open(&path, &header()).unwrap();
            s.append_trial(trial(0)).unwrap();
        }
        // A crash cut the next write short.
        let mut f = OpenOptions::new().append(true).open(&path).unwrap();
        f.write_all(b"{\"kind\":\"trial\",\"id\":1,\"assign")
            .unwrap();
        drop(f);
        let mut s = Store::open(&path, &header()).unwrap();
        assert_eq!(s.trials.len(), 1, "the torn trial is not counted");
        s.append_trial(trial(1)).unwrap();
        drop(s);
        let s = Store::open(&path, &header()).unwrap();
        assert_eq!(s.trials.len(), 2, "and the line written after it parses");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn corruption_in_the_middle_is_an_error_not_a_skip() {
        let path = tmp("corrupt");
        {
            let mut s = Store::open(&path, &header()).unwrap();
            s.append_trial(trial(0)).unwrap();
            s.append_trial(trial(1)).unwrap();
        }
        let text = std::fs::read_to_string(&path).unwrap();
        let mut lines: Vec<&str> = text.lines().collect();
        lines[1] = "{not json";
        std::fs::write(&path, lines.join("\n") + "\n").unwrap();
        let err = Store::open(&path, &header()).err().unwrap();
        assert!(err.contains("corrupt store line"), "{err}");
        let _ = std::fs::remove_file(&path);
    }
}
