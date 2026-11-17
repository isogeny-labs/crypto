//! Incremental case evidence. Complete records survive later failure/kill.
use crate::benchmark::Event;
use serde::Serialize;
use serde_json::json;
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

pub fn write_json_atomic(path: &Path, value: &impl Serialize) -> Result<(), String> {
    let temporary = path.with_extension("json.tmp");
    let bytes = serde_json::to_vec_pretty(value).map_err(|e| e.to_string())?;
    let mut file = File::create(&temporary).map_err(|e| e.to_string())?;
    file.write_all(&bytes).map_err(|e| e.to_string())?;
    file.sync_all().map_err(|e| e.to_string())?;
    fs::rename(&temporary, path).map_err(|e| e.to_string())?;
    #[cfg(unix)]
    File::open(path.parent().ok_or("No parent directory")?)
        .and_then(|dir| dir.sync_all())
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub struct Writer {
    out: PathBuf,
    journal: File,
    sequence: u64,
}

impl Writer {
    pub fn new(out: &Path) -> Result<Self, String> {
        fs::create_dir_all(out.join("cases")).map_err(|e| e.to_string())?;
        let journal = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(out.join("benchmark-cases.jsonl"))
            .map_err(|e| e.to_string())?;
        Ok(Self {
            out: out.into(),
            journal,
            sequence: 0,
        })
    }

    pub fn record(&mut self, event: Event) -> Result<(), String> {
        // Publish the independently replayable case before reporting completion.
        if let Event::Completed { case, record } = &event {
            let path = self
                .out
                .join("cases")
                .join(format!("{:08}.json", case.ordinal));
            if path.exists() {
                return Err("Refusing to replace a completed case".into());
            }
            write_json_atomic(
                &path,
                &json!({"schema":"twist-relations/benchmark/v1","records":[record]}),
            )?;
        }
        let envelope =
            json!({"schema":"twist-relations/case-event/v1","sequence":self.sequence,"data":event});
        let mut bytes = serde_json::to_vec(&envelope).map_err(|e| e.to_string())?;
        bytes.push(b'\n');
        self.journal.write_all(&bytes).map_err(|e| e.to_string())?;
        self.journal.sync_data().map_err(|e| e.to_string())?;
        write_json_atomic(&self.out.join("active-case.json"), &envelope)?;
        self.sequence += 1;
        Ok(())
    }
}
