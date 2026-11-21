//! A shared store of factor-base logarithm tables, keyed by curve,
//! generator and materialised factor base.
//!
//! A run that finds its table here skips relation collection and linear
//! algebra and goes straight to the individual logarithm.  That makes the
//! run *amortized*, not cold: the precomputation was paid by an earlier
//! run.  Every report that uses the store says so ([`provenance`]), so an
//! amortized cost cannot pass for a cold one.
//!
//! **Key.**  SHA-256 over a line-oriented canonical text ([`KEY_SCHEME`]):
//! the curve's degree, subfield degree, `a` and `b` indices, field
//! modulus, subgroup order and cofactor; the generator's coordinates; and
//! [`factor_base_sha256`], the hash of the *sorted materialised point
//! list* — not the recipe, so two recipes that give the same points share
//! a table and a change in materialisation changes the key.
//!
//! **Layout.**  One file `<key>.json` per key in the store directory
//! (`--log-store <dir>`, or `$IC_LOG_STORE` where a command allows it),
//! holding a schema-2 [`LogTableDocument`].  A file is written to a
//! temporary name and hard-linked into place, so it appears whole or not
//! at all, and an existing file is never replaced.
//!
//! **Lookup** recomputes the key from the curve and base in hand, checks
//! the document's generator and factor-base hash against them, and
//! re-verifies every column as `[log]G == point` before the table is used.
//! Any mismatch is an error, never a silent fallback.
use super::experiment::{
    self, log_table_from_doc, log_table_to_doc, GeneratorPoint, LogTableDocument, Solver,
};
use super::params;
use crypto_lib::binary_ecc::BinaryPoint;
use crypto_lib::cryptanalysis::koblitz_factor_base_search::FactorBaseSpec;
use crypto_lib::cryptanalysis::koblitz_index_calculus::{
    FactorBaseLogTable, FrobeniusFactorBase, KoblitzCurve,
};
use crypto_lib::hash::sha256::sha256;
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Environment variable naming a default store directory.
pub const STORE_ENV: &str = "IC_LOG_STORE";
/// First line of the canonical key text; changing the key's encoding
/// changes this.
pub const KEY_SCHEME: &str = "ic.log_store.key/v1";
/// First line of the canonical factor-base text.
pub const FACTOR_BASE_SCHEME: &str = "ic.log_store.factor_base/v1";

/// The store directory: the flag, else a non-empty `$IC_LOG_STORE`.
pub fn dir_from(flag: Option<&Path>) -> Option<PathBuf> {
    flag.map(Path::to_path_buf).or_else(|| {
        std::env::var_os(STORE_ENV)
            .filter(|v| !v.is_empty())
            .map(PathBuf::from)
    })
}

/// SHA-256 of the materialised factor base: its affine points as
/// lowercase-hex `x,y` lines in ascending `(x, y)` order, after a header
/// and the point count.  The order the builder produced them in does not
/// matter; the points themselves do.
pub fn factor_base_sha256(fb: &FrobeniusFactorBase) -> String {
    let mut points: Vec<(BigUint, BigUint)> = Vec::with_capacity(fb.points.len());
    let mut infinity = 0usize;
    for p in &fb.points {
        match p {
            BinaryPoint::Affine { x, y } => points.push((x.to_biguint(), y.to_biguint())),
            BinaryPoint::Infinity => infinity += 1,
        }
    }
    points.sort_unstable();
    let mut text = format!(
        "{FACTOR_BASE_SCHEME}\npoints={}\ninfinity={infinity}\n",
        points.len()
    );
    for (x, y) in &points {
        text.push_str(&x.to_str_radix(16));
        text.push(',');
        text.push_str(&y.to_str_radix(16));
        text.push('\n');
    }
    hex::encode(sha256(text.as_bytes()))
}

/// The curve's generator, as the document records it.
pub fn generator(c: &KoblitzCurve) -> Result<GeneratorPoint, String> {
    match c.generator() {
        BinaryPoint::Affine { x, y } => Ok(GeneratorPoint {
            x: params::hex(&x.to_biguint()),
            y: params::hex(&y.to_biguint()),
        }),
        BinaryPoint::Infinity => Err("the curve's generator is the point at infinity".into()),
    }
}

/// A table's identity in the store.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoreKey {
    /// Lowercase hex SHA-256 of [`StoreKey::canonical`]; the file stem.
    pub key: String,
    pub factor_base_sha256: String,
    pub generator: GeneratorPoint,
    /// The exact text hashed, kept so a report can show what was bound.
    pub canonical: String,
}

/// The key of the table for `fb` on `c` with `c`'s generator.
pub fn key_for(c: &KoblitzCurve, fb: &FrobeniusFactorBase) -> Result<StoreKey, String> {
    key_from_parts(c, &generator(c)?, &factor_base_sha256(fb))
}

fn key_from_parts(
    c: &KoblitzCurve,
    g: &GeneratorPoint,
    fb_sha256: &str,
) -> Result<StoreKey, String> {
    let modulus: Vec<String> = c
        .curve
        .irreducible
        .low_terms
        .iter()
        .map(ToString::to_string)
        .collect();
    let canonical = format!(
        "{KEY_SCHEME}\ndegree={}\nsubfield={}\ncurve_a={}\ncurve_b={}\nmodulus_low_terms={}\nsubgroup_order={}\ncofactor={}\ngenerator_x={}\ngenerator_y={}\nfactor_base_sha256={fb_sha256}\n",
        c.n,
        c.k,
        c.a,
        c.b_index,
        modulus.join(","),
        c.subgroup_order,
        c.cofactor,
        g.x,
        g.y,
    );
    Ok(StoreKey {
        key: hex::encode(sha256(canonical.as_bytes())),
        factor_base_sha256: fb_sha256.to_string(),
        generator: g.clone(),
        canonical,
    })
}

/// Where the table for `key` lives in `dir`.
pub fn path_for(dir: &Path, key: &StoreKey) -> PathBuf {
    dir.join(format!("{}.json", key.key))
}

/// A schema-2 document: [`log_table_to_doc`] plus the generator and the
/// factor-base hash the key binds.
pub fn stored_doc(
    c: &KoblitzCurve,
    spec: &FactorBaseSpec,
    fb: &FrobeniusFactorBase,
    summands: u8,
    solver: Solver,
    table: &FactorBaseLogTable,
) -> Result<LogTableDocument, String> {
    let mut doc = log_table_to_doc(c, spec, summands, solver, table);
    doc.schema_version = 2;
    doc.generator = Some(generator(c)?);
    doc.factor_base_sha256 = Some(factor_base_sha256(fb));
    Ok(doc)
}

/// Check a document against the curve and base in hand and re-verify
/// every column.  Used for store hits and for any schema-2 file.
pub fn verify_doc(
    c: &KoblitzCurve,
    fb: &FrobeniusFactorBase,
    doc: &LogTableDocument,
) -> Result<FactorBaseLogTable, String> {
    if doc.schema_version >= 2 {
        let want = factor_base_sha256(fb);
        if doc.factor_base_sha256.as_deref() != Some(want.as_str()) {
            return Err(format!(
                "logarithm table was built over a different factor base (factor-base sha256 {} here, {} in the table)",
                want,
                doc.factor_base_sha256.as_deref().unwrap_or("absent")
            ));
        }
    }
    // Curve, generator and `[log]G == point` for every column.
    log_table_from_doc(c, doc)
}

/// The stored table for `fb` on `c`, re-verified; `Ok(None)` when the
/// store holds no file for the key.  A file that is there but does not
/// match or does not verify is an error.
pub fn lookup(
    dir: &Path,
    c: &KoblitzCurve,
    fb: &FrobeniusFactorBase,
) -> Result<Option<(FactorBaseLogTable, LogTableDocument, StoreKey, PathBuf)>, String> {
    let key = key_for(c, fb)?;
    let path = path_for(dir, &key);
    if std::fs::symlink_metadata(&path).is_err() {
        return Ok(None);
    }
    let doc = experiment::load_log_table(&path)?;
    if doc.schema_version < 2 {
        return Err(format!(
            "{}: a store entry must be a schema-2 table (generator and factor-base hash)",
            path.display()
        ));
    }
    if doc.generator.as_ref() != Some(&key.generator) {
        return Err(format!(
            "{}: stored table names a different generator",
            path.display()
        ));
    }
    let table = verify_doc(c, fb, &doc).map_err(|e| format!("{}: {e}", path.display()))?;
    // The document's own fields must give back the file's key.
    let again = key_from_parts(c, &key.generator, &key.factor_base_sha256)?;
    if again.key != key.key {
        return Err(format!("{}: stored table key mismatch", path.display()));
    }
    Ok(Some((table, doc, key, path)))
}

/// What [`insert`] did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inserted {
    Written,
    /// A file for the key was already there and verified; it was kept.
    AlreadyPresent,
}

impl Inserted {
    pub fn name(self) -> &'static str {
        match self {
            Self::Written => "written",
            Self::AlreadyPresent => "already_present",
        }
    }
}

/// Put `doc` in the store under its key: written to a temporary file,
/// synced, and hard-linked into place, which fails rather than replace an
/// existing file.  An existing file for the key is kept if it verifies
/// and is an error otherwise.
pub fn insert(
    dir: &Path,
    c: &KoblitzCurve,
    fb: &FrobeniusFactorBase,
    doc: &LogTableDocument,
) -> Result<(Inserted, StoreKey, PathBuf), String> {
    let key = key_for(c, fb)?;
    if doc.schema_version < 2
        || doc.factor_base_sha256.as_deref() != Some(key.factor_base_sha256.as_str())
        || doc.generator.as_ref() != Some(&key.generator)
    {
        return Err("refusing to store a table that does not carry this key's generator and factor-base hash".into());
    }
    verify_doc(c, fb, doc)?;
    std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let path = path_for(dir, &key);
    let existing = |path: &Path| -> Result<(Inserted, StoreKey, PathBuf), String> {
        match lookup(dir, c, fb)? {
            Some(_) => Ok((Inserted::AlreadyPresent, key.clone(), path.to_path_buf())),
            None => Err(format!("{}: store entry vanished", path.display())),
        }
    };
    if std::fs::symlink_metadata(&path).is_ok() {
        return existing(&path);
    }
    let text = serde_json::to_string_pretty(doc).map_err(|e| e.to_string())?;
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let tmp = dir.join(format!(
        ".{}.json.tmp.{}.{nonce}",
        key.key,
        std::process::id()
    ));
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&tmp)
        .and_then(|mut f| {
            f.write_all(text.as_bytes())?;
            f.write_all(b"\n")?;
            f.sync_all()
        })
        .map_err(|e| format!("could not create {}: {e}", tmp.display()))?;
    let linked = std::fs::hard_link(&tmp, &path);
    let _ = std::fs::remove_file(&tmp);
    match linked {
        Ok(()) => {
            if let Ok(d) = std::fs::File::open(dir) {
                let _ = d.sync_all();
            }
            Ok((Inserted::Written, key, path))
        }
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => existing(&path),
        Err(e) => Err(format!("could not link {}: {e}", path.display())),
    }
}

/// Where a run's factor-base logarithm table came from, for its report.
///
/// `source` is `"computed"` (collection and linear algebra ran in this
/// run: cold), `"store"` (read from the store: amortized), or `"file"`
/// (read from an explicit `--logs` file: amortized).
pub fn provenance(source: &str, key: Option<&StoreKey>, path: Option<&Path>) -> Value {
    let paid = source == "computed";
    json!({
        "log_table_source": source,
        "key": key.map(|k| k.key.clone()),
        "key_scheme": key.map(|_| KEY_SCHEME),
        "factor_base_sha256": key.map(|k| k.factor_base_sha256.clone()),
        "generator": key.map(|k| json!({"x": k.generator.x, "y": k.generator.y})),
        "path": path.map(|p| p.display().to_string()),
        "precomputation_paid_in_this_run": paid,
        "cost_basis": if paid { "cold" } else { "amortized" },
        "note": if paid {
            "relation collection and linear algebra ran in this run; its cost is a cold cost"
        } else {
            "relation collection and linear algebra were NOT paid in this run (an earlier run's table, re-verified); this run's cost is amortized and must be reported after, never as, the cold cost"
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::experiment::{curve, materialize};

    fn tmpdir(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!(
            "ic-log-store-{tag}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    fn spec(index: usize) -> FactorBaseSpec {
        FactorBaseSpec::Factor { index }
    }

    #[test]
    fn the_key_is_deterministic_and_binds_curve_and_base() {
        let c = curve(9, 0, 1, 1).unwrap();
        let fb = materialize(&c, &spec(0)).unwrap();
        let k1 = key_for(&c, &fb).unwrap();
        let k2 = key_for(
            &curve(9, 0, 1, 1).unwrap(),
            &materialize(&c, &spec(0)).unwrap(),
        )
        .unwrap();
        assert_eq!(k1, k2);
        assert_eq!(k1.key.len(), 64);
        // The point order does not matter; the points do.
        let mut shuffled = fb.clone();
        shuffled.points.reverse();
        assert_eq!(factor_base_sha256(&shuffled), k1.factor_base_sha256);
        let mut smaller = fb.clone();
        smaller.points.pop();
        assert_ne!(factor_base_sha256(&smaller), k1.factor_base_sha256);
        // Another curve, another key.
        let c1 = curve(9, 1, 1, 1).unwrap();
        let fb1 = materialize(&c1, &spec(0)).unwrap();
        assert_ne!(key_for(&c1, &fb1).unwrap().key, k1.key);
    }

    #[test]
    fn a_different_generator_or_base_gives_a_different_key() {
        let c = curve(9, 0, 1, 1).unwrap();
        let fb = materialize(&c, &spec(0)).unwrap();
        let k = key_for(&c, &fb).unwrap();
        // Generator: [2]G in place of G.
        let g2 = match c.mul(c.generator(), &BigUint::from(2u32)) {
            BinaryPoint::Affine { x, y } => GeneratorPoint {
                x: params::hex(&x.to_biguint()),
                y: params::hex(&y.to_biguint()),
            },
            BinaryPoint::Infinity => unreachable!(),
        };
        let kg = key_from_parts(&c, &g2, &k.factor_base_sha256).unwrap();
        assert_ne!(kg.key, k.key);
        // Base: one point fewer.
        let mut fb2 = fb.clone();
        fb2.points.pop();
        assert_ne!(key_for(&c, &fb2).unwrap().key, k.key);
    }

    /// Precompute once into the store, then: a lookup is a verified hit,
    /// a second insert keeps the first file, and a corrupted log is
    /// refused rather than used.
    #[test]
    fn a_store_round_trip_reverifies_and_rejects_corruption() {
        let c = curve(9, 0, 1, 1).unwrap();
        let sp = spec(0);
        let fb = materialize(&c, &sp).unwrap();
        let opts = experiment::ic_options(Solver::PairTable, 2, 200_000, 7);
        let (table, report) =
            crypto_lib::cryptanalysis::koblitz_index_calculus::solve_factor_base_logs(
                &c, &fb, &opts,
            )
            .expect("columns");
        assert!(report.verified);
        let doc = stored_doc(&c, &sp, &fb, 2, Solver::PairTable, &table).unwrap();
        let dir = tmpdir("roundtrip");
        assert!(lookup(&dir, &c, &fb).unwrap().is_none());
        let (how, key, path) = insert(&dir, &c, &fb, &doc).unwrap();
        assert_eq!(how, Inserted::Written);
        assert_eq!(path, path_for(&dir, &key));
        let (hit, _, hit_key, _) = lookup(&dir, &c, &fb).unwrap().expect("hit");
        assert_eq!(hit_key, key);
        assert_eq!(hit.columns, table.columns);
        let (again, _, _) = insert(&dir, &c, &fb, &doc).unwrap();
        assert_eq!(again, Inserted::AlreadyPresent);
        // No temporary files left behind.
        assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 1);

        // Corrupt one logarithm in place.
        let mut bad = doc.clone();
        let log = params::number(&bad.columns[0].log).unwrap() + 1u32;
        bad.columns[0].log = (log % &c.subgroup_order).to_string();
        std::fs::remove_file(&path).unwrap();
        std::fs::write(&path, serde_json::to_string(&bad).unwrap()).unwrap();
        let err = lookup(&dir, &c, &fb).unwrap_err();
        assert!(err.contains("re-verification"), "{err}");
        // And an insert never replaces the bad file with a good one silently.
        assert!(insert(&dir, &c, &fb, &doc).is_err());
        assert_eq!(
            std::fs::read_to_string(&path).unwrap(),
            serde_json::to_string(&bad).unwrap()
        );

        // A table from another base under this key is refused.
        let mut other = doc.clone();
        other.factor_base_sha256 = Some("00".repeat(32));
        std::fs::remove_file(&path).unwrap();
        std::fs::write(&path, serde_json::to_string(&other).unwrap()).unwrap();
        assert!(lookup(&dir, &c, &fb)
            .unwrap_err()
            .contains("different factor base"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// `ic logs --log-store` computes once and reuses after; `ic solve
    /// --log-store` finds the table by key and says it did not pay for it.
    #[test]
    fn logs_and_solve_share_the_store_and_report_the_source() {
        use clap::Parser;
        #[derive(Parser)]
        struct Logs {
            #[command(flatten)]
            a: experiment::LogsArgs,
        }
        #[derive(Parser)]
        struct Solve {
            #[command(flatten)]
            a: experiment::SolveArgs,
        }
        let dir = tmpdir("cli");
        let d = dir.to_str().unwrap();
        // Neither --database nor --log-store: refused by the parser.
        assert!(Logs::try_parse_from(["logs", "--degree", "9"]).is_err());
        let args = || {
            Logs::try_parse_from(["logs", "--degree", "9", "--log-store", d])
                .unwrap()
                .a
        };
        let first = experiment::logs(args(), true).unwrap();
        assert_eq!(first["status"], "complete");
        assert_eq!(first["log_table"]["log_table_source"], "computed");
        assert_eq!(first["log_table"]["precomputation_paid_in_this_run"], true);
        assert_eq!(first["log_table"]["store_write"], "written");
        assert!(first["counts"]["trials"].as_u64().unwrap() > 0);
        let second = experiment::logs(args(), true).unwrap();
        assert_eq!(second["log_table"]["log_table_source"], "store");
        assert_eq!(
            second["log_table"]["precomputation_paid_in_this_run"],
            false
        );
        assert_eq!(second["log_table"]["key"], first["log_table"]["key"]);
        assert_eq!(second["counts"]["trials"], 0);
        assert_eq!(second["counts"]["relations"], 0);
        let solve = Solve::try_parse_from(["solve", "--degree", "9", "--log-store", d])
            .unwrap()
            .a;
        let out = experiment::solve(solve, true).unwrap();
        assert_eq!(out["status"], "complete");
        assert_eq!(out["result"]["verified"], true);
        assert_eq!(out["log_table"]["log_table_source"], "store");
        assert_eq!(out["log_table"]["key"], first["log_table"]["key"]);
        assert_eq!(out["log_table"]["cost_basis"], "amortized");
        // Another base has no entry: an error, not a fallback.
        let other = Solve::try_parse_from([
            "solve",
            "--degree",
            "9",
            "--log-store",
            d,
            "--factor-index",
            "1",
        ])
        .unwrap()
        .a;
        if let Err(e) = experiment::solve(other, true) {
            assert!(
                e.contains("no stored logarithm table") || e.contains("factor"),
                "{e}"
            );
        } else {
            panic!("a missing store entry must not solve");
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_schema_one_document_still_loads() {
        let c = curve(9, 0, 1, 1).unwrap();
        let sp = spec(0);
        let fb = materialize(&c, &sp).unwrap();
        let opts = experiment::ic_options(Solver::PairTable, 2, 200_000, 7);
        let (table, _) = crypto_lib::cryptanalysis::koblitz_index_calculus::solve_factor_base_logs(
            &c, &fb, &opts,
        )
        .expect("columns");
        let v1 = log_table_to_doc(&c, &sp, 2, Solver::PairTable, &table);
        assert_eq!(v1.schema_version, 1);
        let text = serde_json::to_string(&v1).unwrap();
        assert!(!text.contains("generator") && !text.contains("factor_base_sha256"));
        let dir = tmpdir("v1");
        let path = dir.join("v1.json");
        std::fs::write(&path, &text).unwrap();
        let loaded = experiment::load_log_table(&path).unwrap();
        assert_eq!(loaded.schema_version, 1);
        assert_eq!(verify_doc(&c, &fb, &loaded).unwrap().columns, table.columns);
        // A v1 file is not accepted as a store entry.
        let key = key_for(&c, &fb).unwrap();
        std::fs::rename(&path, path_for(&dir, &key)).unwrap();
        assert!(lookup(&dir, &c, &fb).unwrap_err().contains("schema-2"));
        // A v1 document carrying v2 fields is refused at load.
        let mut mixed = stored_doc(&c, &sp, &fb, 2, Solver::PairTable, &table).unwrap();
        mixed.schema_version = 1;
        let p2 = dir.join("mixed.json");
        std::fs::write(&p2, serde_json::to_string(&mixed).unwrap()).unwrap();
        assert!(experiment::load_log_table(&p2).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
