//! Atomic snapshots of completed breadth-first expansion batches.

use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::walk::{Edge, Failure, Node, Walk};

const MAGIC: &str = "IWCKPT1 ";

#[derive(Serialize, Deserialize)]
struct Snapshot {
    fingerprint: String,
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    failures: Vec<Failure>,
    phi_ms: Vec<(u64, u128, usize)>,
    walk_ms: u128,
    edges_beyond_cap: usize,
    root_histogram: Vec<(u64, usize, usize)>,
}

/// Persist a complete expansion batch. The caller owns the run directory lock.
pub fn save(path: &Path, fingerprint: &str, walk: &Walk) -> Result<(), String> {
    let snapshot = Snapshot {
        fingerprint: fingerprint.into(),
        nodes: walk.nodes.clone(),
        edges: walk.edges.clone(),
        failures: walk.failures.clone(),
        phi_ms: walk.stats.phi_ms.clone(),
        walk_ms: walk.stats.walk_ms,
        edges_beyond_cap: walk.stats.edges_beyond_cap,
        root_histogram: walk
            .stats
            .root_histogram
            .iter()
            .map(|(&(ell, roots), &count)| (ell, roots, count))
            .collect(),
    };
    let payload = serde_json::to_vec(&snapshot)
        .map_err(|e| format!("serializing isogeny walk checkpoint: {e}"))?;
    let checksum = blake3::hash(&payload).to_hex();
    let temporary = path.with_extension("ckpt.tmp");
    if temporary.exists() || temporary.is_symlink() {
        fs::remove_file(&temporary)
            .map_err(|e| format!("removing stale {}: {e}", temporary.display()))?;
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|e| format!("creating {}: {e}", temporary.display()))?;
    file.write_all(format!("{MAGIC}{checksum}\n").as_bytes())
        .and_then(|_| file.write_all(&payload))
        .and_then(|_| file.sync_all())
        .map_err(|e| format!("writing {}: {e}", temporary.display()))?;
    fs::rename(&temporary, path).map_err(|e| format!("publishing {}: {e}", path.display()))
}

/// Load a snapshot only for the same root and immutable walk parameters.
pub fn load(path: &Path, fingerprint: &str, walk: &mut Walk) -> Result<(), String> {
    let bytes = fs::read(path).map_err(|e| format!("reading {}: {e}", path.display()))?;
    let newline = bytes
        .iter()
        .position(|&b| b == b'\n')
        .ok_or_else(|| format!("{}: missing walk checkpoint header", path.display()))?;
    let (header, body) = bytes.split_at(newline);
    let expected = std::str::from_utf8(header)
        .ok()
        .and_then(|s| s.strip_prefix(MAGIC))
        .ok_or_else(|| format!("{}: invalid walk checkpoint version", path.display()))?;
    if expected != blake3::hash(&body[1..]).to_hex().as_str() {
        return Err(format!(
            "{}: walk checkpoint checksum mismatch",
            path.display()
        ));
    }
    let state: Snapshot = serde_json::from_slice(&body[1..])
        .map_err(|e| format!("{}: invalid walk checkpoint: {e}", path.display()))?;
    if state.fingerprint != fingerprint {
        return Err(format!(
            "{}: walk checkpoint parameters differ",
            path.display()
        ));
    }
    let unique_j: HashSet<_> = state.nodes.iter().map(|n| n.j).collect();
    if state.nodes.is_empty()
        || unique_j.len() != state.nodes.len()
        || state.nodes[0].j != walk.nodes[0].j
        || state.nodes[0].model != walk.nodes[0].model
        || state
            .edges
            .iter()
            .any(|e| e.source >= state.nodes.len() || e.target >= state.nodes.len())
        || state.failures.iter().any(|f| f.source >= state.nodes.len())
        || state
            .nodes
            .iter()
            .any(|n| n.parent_edge.is_some_and(|i| i >= state.edges.len()))
    {
        return Err(format!("{}: invalid walk checkpoint graph", path.display()));
    }
    walk.nodes = state.nodes;
    walk.edges = state.edges;
    walk.failures = state.failures;
    walk.stats.phi_ms = state.phi_ms;
    walk.stats.walk_ms = state.walk_ms;
    walk.stats.edges_beyond_cap = state.edges_beyond_cap;
    walk.stats.root_histogram = state
        .root_histogram
        .into_iter()
        .map(|(ell, roots, count)| ((ell, roots), count))
        .collect();
    Ok(())
}
