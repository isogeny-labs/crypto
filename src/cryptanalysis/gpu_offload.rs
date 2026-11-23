//! Process-wide GPU offload mode for the `ic` CLI.
//!
//! `ic --gpu auto|off|<worker>` sets the mode once at startup. Off (the default) changes nothing:
//! no worker is looked up and no report field is added. A stage that offloads reads
//! [`mode`] and [`report`]; every offloaded result is re-verified on the CPU by that stage and any
//! worker failure falls back to the CPU path (see `fes_gpu` and `koblitz_sparse_la`).
//!
//! Stage status is stated, not implied: [`report`] lists each stage with `offloaded` only where the
//! code path exists. Nothing is offloaded by this module itself.

use std::path::PathBuf;
use std::sync::OnceLock;

use serde_json::{json, Value};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GpuMode {
    Off,
    Auto,
    /// An explicit worker executable, used for every stage that speaks its contract.
    Worker(PathBuf),
}

impl GpuMode {
    pub fn parse(text: &str) -> Result<GpuMode, String> {
        match text {
            "off" => Ok(GpuMode::Off),
            "auto" => Ok(GpuMode::Auto),
            "" => Err("--gpu needs auto, off or a worker path".into()),
            path => {
                let path = PathBuf::from(path);
                if path.is_file() {
                    Ok(GpuMode::Worker(path))
                } else {
                    Err(format!("--gpu worker not found: {}", path.display()))
                }
            }
        }
    }
}

static MODE: OnceLock<GpuMode> = OnceLock::new();

/// Set the mode once; later calls are ignored so a library caller cannot flip it mid-run.
pub fn set_mode(mode: GpuMode) {
    let _ = MODE.set(mode);
}

pub fn mode() -> &'static GpuMode {
    MODE.get_or_init(|| GpuMode::Off)
}

pub fn enabled() -> bool {
    !matches!(mode(), GpuMode::Off)
}

/// The SpMV worker for `--spmv worker`: `IC_SPMV_WORKER` if it names a file.
pub fn discover_spmv_worker() -> Option<PathBuf> {
    let path = PathBuf::from(std::env::var_os("IC_SPMV_WORKER")?);
    path.is_file().then_some(path)
}

/// What the current mode can reach, for the JSON report. `None` when the mode is off.
pub fn report() -> Option<Value> {
    let (fes, spmv) = match mode() {
        GpuMode::Off => return None,
        GpuMode::Auto => (
            crate::cryptanalysis::fes_gpu::discover_worker(),
            discover_spmv_worker(),
        ),
        GpuMode::Worker(path) => (Some(path.clone()), Some(path.clone())),
    };
    let show = |worker: &Option<PathBuf>| match worker {
        Some(path) => json!(path.display().to_string()),
        None => Value::Null,
    };
    Some(json!({
        "mode": match mode() { GpuMode::Auto => "auto", _ => "worker" },
        "workers": {"fes": show(&fes), "spmv": show(&spmv)},
        // A stage is `offloaded` only where its code path exists in this build.
        "stages": {
            "fes_leaf": "not offloaded: a per-target worker launch is expected to lose to the in-process search (not yet measured against it); batching not built",
            "linear_algebra_spmv": "not offloaded: the SpMV worker relaunches per multiply; persistent worker not built",
            "decomposition_oracles": "not offloaded: gpu/semaev and gpu/rootfind not wired",
        },
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_accepts_keywords_and_rejects_missing_paths() {
        assert_eq!(GpuMode::parse("off"), Ok(GpuMode::Off));
        assert_eq!(GpuMode::parse("auto"), Ok(GpuMode::Auto));
        assert!(GpuMode::parse("/nonexistent/worker").is_err());
        assert!(GpuMode::parse("").is_err());
    }

    #[test]
    fn default_mode_is_off_and_reports_nothing() {
        // MODE may have been set by another test in this process; only the default is checked.
        if MODE.get().is_none() {
            assert!(!enabled());
            assert!(report().is_none());
        }
    }
}
