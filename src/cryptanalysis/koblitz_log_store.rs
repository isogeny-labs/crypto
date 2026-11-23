//! A shared store of Koblitz factor-base logarithm tables, keyed by curve,
//! generator and materialised factor base.
//!
//! Moved here from the `ic` binary (`src/bin/ic/log_store.rs`, which
//! re-exports it) so that the `ecbench` index-calculus methods can use the
//! same store, key, file format and verification; the `ic` behaviour is
//! unchanged.
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
use crate::binary_ecc::{BinaryPoint, F2mElement};
use crate::cryptanalysis::koblitz_factor_base_search::FactorBaseSpec;
use crate::cryptanalysis::koblitz_index_calculus::{
    DecompositionStrategy, FactorBaseLogTable, FrobeniusFactorBase, KoblitzCurve,
};
use crate::hash::sha256::sha256;
use clap::ValueEnum;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

/// Environment variable naming a default store directory.
pub const STORE_ENV: &str = "IC_LOG_STORE";
/// First line of the canonical key text; changing the key's encoding
/// changes this.
pub const KEY_SCHEME: &str = "ic.log_store.key/v1";
/// First line of the canonical factor-base text.
pub const FACTOR_BASE_SCHEME: &str = "ic.log_store.factor_base/v1";

/// The decomposition solver a table's relations were found with.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Solver {
    Groebner,
    Sat,
    Enumerate,
    /// Meet-in-the-middle over a precomputed pair-sum table.
    PairTable,
    /// Trimoska WDSat on the same Semaev ANF as `sat`.
    Wdsat,
    /// ALMASTY-inspired quadratic Semaev FES (`m = 2`): Gray early-exit,
    /// Möbius all-roots, Monica past `n = 24`.
    MqFes,
}
impl Solver {
    pub fn name(self) -> &'static str {
        match self {
            Self::Groebner => "groebner",
            Self::Sat => "sat",
            Self::Enumerate => "enumerate",
            Self::PairTable => "pair-table",
            Self::Wdsat => "wdsat",
            Self::MqFes => "mq-fes",
        }
    }
    pub fn strategy(self) -> DecompositionStrategy {
        match self {
            Self::Groebner => DecompositionStrategy::Groebner,
            Self::Sat => DecompositionStrategy::Sat,
            Self::Enumerate => DecompositionStrategy::Enumerate,
            Self::PairTable => DecompositionStrategy::PairTable,
            Self::Wdsat => DecompositionStrategy::Wdsat,
            Self::MqFes => DecompositionStrategy::MqFes,
        }
    }
}

fn one_u32() -> u32 {
    1
}
fn one_u64() -> u64 {
    1
}
fn is_one_u32(v: &u32) -> bool {
    *v == 1
}
fn is_one_u64(v: &u64) -> bool {
    *v == 1
}

/// `0x`-prefixed lowercase hex, as the `ic` documents write coordinates.
pub fn hex(n: &BigUint) -> String {
    format!("0x{}", n.to_str_radix(16))
}

/// A decimal or `0x` hexadecimal integer of at most 1024 bits.
pub fn number(s: &str) -> Result<BigUint, String> {
    if s.is_empty() || s.len() > 320 {
        return Err("integer must contain 1..320 characters".into());
    }
    let (digits, radix) = s.strip_prefix("0x").map_or((s, 10), |v| (v, 16));
    if digits.is_empty()
        || !digits.bytes().all(|c| {
            if radix == 16 {
                c.is_ascii_hexdigit()
            } else {
                c.is_ascii_digit()
            }
        })
    {
        return Err("integers must be decimal or use a 0x hexadecimal prefix".into());
    }
    let n = BigUint::parse_bytes(digits.as_bytes(), radix).ok_or("invalid integer")?;
    if n.bits() > 1024 {
        return Err("integer exceeds the inspection limit of 1024 bits".into());
    }
    Ok(n)
}

/// A serialised factor-base logarithm database, bound to its curve and
/// factor base so it cannot be replayed on another.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LogTableDocument {
    pub schema_version: u32,
    pub degree: u32,
    pub curve_a: u8,
    #[serde(default = "one_u32", skip_serializing_if = "is_one_u32")]
    pub subfield: u32,
    #[serde(default = "one_u64", skip_serializing_if = "is_one_u64")]
    pub curve_b: u64,
    pub spec: FactorBaseSpec,
    pub subgroup_order: String,
    pub cofactor: String,
    pub summands: u8,
    pub solver: Solver,
    /// `(x, y, log_G point)` per relation column; coordinates hex, log decimal.
    pub columns: Vec<LogColumn>,
    /// Schema 2: the generator the logarithms are taken to (hex coordinates).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generator: Option<GeneratorPoint>,
    /// Schema 2: SHA-256 of the sorted materialised factor base
    /// ([`factor_base_sha256`]).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub factor_base_sha256: Option<String>,
}
/// A generator's affine coordinates, hex.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GeneratorPoint {
    pub x: String,
    pub y: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LogColumn {
    pub x: String,
    pub y: String,
    pub log: String,
}
pub fn load_log_table(path: &Path) -> Result<LogTableDocument, String> {
    let mut data = Vec::new();
    File::open(path)
        .map_err(|e| format!("{}: {e}", path.display()))?
        .take(16_777_217)
        .read_to_end(&mut data)
        .map_err(|e| e.to_string())?;
    if data.len() > 16_777_216 {
        return Err("logarithm database exceeds 16 MiB".into());
    }
    let doc: LogTableDocument = serde_json::from_slice(&data)
        .map_err(|e| format!("invalid logarithm database JSON: {e}"))?;
    match doc.schema_version {
        1 if doc.generator.is_none() && doc.factor_base_sha256.is_none() => {}
        2 if doc.generator.is_some() && doc.factor_base_sha256.is_some() => {}
        1 | 2 => {
            return Err(
                "logarithm database fields do not match its schema version (schema 2 alone, and always, carries generator and factor_base_sha256)"
                    .into(),
            )
        }
        _ => return Err("unsupported logarithm database schema version".into()),
    }
    Ok(doc)
}
/// Serialise a solved logarithm table, bound to its curve and factor base.
pub fn log_table_to_doc(
    c: &KoblitzCurve,
    spec: &FactorBaseSpec,
    summands: u8,
    solver: Solver,
    table: &FactorBaseLogTable,
) -> LogTableDocument {
    let columns: Vec<LogColumn> = table
        .columns
        .iter()
        .map(|(point, log)| match point {
            BinaryPoint::Affine { x, y } => LogColumn {
                x: hex(&x.to_biguint()),
                y: hex(&y.to_biguint()),
                log: log.to_string(),
            },
            BinaryPoint::Infinity => unreachable!("projected columns are affine"),
        })
        .collect();
    LogTableDocument {
        schema_version: 1,
        degree: c.n,
        curve_a: c.a,
        subfield: c.k,
        curve_b: c.b_index,
        spec: spec.clone(),
        subgroup_order: c.subgroup_order.to_string(),
        cofactor: c.cofactor.to_string(),
        summands,
        solver,
        columns,
        generator: None,
        factor_base_sha256: None,
    }
}
/// Rebuild a logarithm table from its document and re-verify every column
/// against the reconstructed curve; a tampered or mismatched table is an error.
pub fn log_table_from_doc(
    c: &KoblitzCurve,
    doc: &LogTableDocument,
) -> Result<FactorBaseLogTable, String> {
    if doc.degree != c.n
        || doc.curve_a != c.a
        || doc.subfield != c.k
        || doc.curve_b != c.b_index
        || doc.subgroup_order != c.subgroup_order.to_string()
    {
        return Err("logarithm database does not belong to this curve".into());
    }
    if let Some(g) = &doc.generator {
        let here = match c.generator() {
            BinaryPoint::Affine { x, y } => GeneratorPoint {
                x: hex(&x.to_biguint()),
                y: hex(&y.to_biguint()),
            },
            BinaryPoint::Infinity => return Err("the curve's generator is infinity".into()),
        };
        if *g != here {
            return Err("logarithm database is taken to a different generator".into());
        }
    }
    let mut columns = Vec::with_capacity(doc.columns.len());
    for col in &doc.columns {
        let x = F2mElement::from_biguint(&number(&col.x)?, c.n);
        let y = F2mElement::from_biguint(&number(&col.y)?, c.n);
        let log = number(&col.log)?;
        columns.push((BinaryPoint::Affine { x, y }, log));
    }
    let table = FactorBaseLogTable { columns };
    if !table.verify(c) {
        return Err("logarithm database failed re-verification: a column log does not satisfy [x]G == point".into());
    }
    Ok(table)
}

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
            x: hex(&x.to_biguint()),
            y: hex(&y.to_biguint()),
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

/// The key from its parts: the curve, a generator and a factor-base hash.
pub fn key_from_parts(
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

/// A verified store hit: the table, its document, its key and its file.
pub type Hit = (FactorBaseLogTable, LogTableDocument, StoreKey, PathBuf);

/// The stored table for `fb` on `c`, re-verified; `Ok(None)` when the
/// store holds no file for the key.  A file that is there but does not
/// match or does not verify is an error.
pub fn lookup(
    dir: &Path,
    c: &KoblitzCurve,
    fb: &FrobeniusFactorBase,
) -> Result<Option<Hit>, String> {
    let key = key_for(c, fb)?;
    let path = path_for(dir, &key);
    if std::fs::symlink_metadata(&path).is_err() {
        return Ok(None);
    }
    let doc = load_log_table(&path)?;
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

/// Whether a report's `detail` says its factor-base log table came from a
/// store or a file rather than being computed in the run: the run is then
/// amortized, and its cost must never be read as a cold `S`.
pub fn detail_is_amortized(detail: &Value) -> bool {
    detail
        .pointer("/log_table/precomputation_paid_in_this_run")
        .and_then(Value::as_bool)
        == Some(false)
        || matches!(
            detail
                .pointer("/log_table/log_table_source")
                .and_then(Value::as_str),
            Some("store" | "file")
        )
}
