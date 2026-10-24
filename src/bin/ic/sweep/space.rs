//! The sweeper's search space: typed, conditional, and scale-relative.
//!
//! # Tracks
//!
//! Owner decision, 2026-10-24: an index-calculus pipeline counts only when
//! its relations come from solving summation-polynomial systems over an
//! algebraically defined factor base, without table collisions.  Every
//! space therefore carries a [`Track`]:
//!
//! - [`Track::Algebraic`]: the counted track.  Relations come from a
//!   `descent-algebraic` solve.
//! - [`Track::Table`]: a reference track (pair tables, meet in the middle,
//!   large primes on a pair table).  It is searched and reported, and is
//!   never ranked against the algebraic track as an improvement.
//!
//! The `rootfind-*` oracles are in neither: whether solving the univariate
//! Semaev polynomial per base point counts as "solving a summation
//! polynomial system" or as a root table is not decided by the criterion as
//! recorded, so no space offers them until the owner rules.
//!
//! # Scale-relative knobs
//!
//! A configuration tuned at one size must say something at the next.  Base
//! sizes are therefore `2^(θ · scale)` and the descent dimension is `δ · n`,
//! with θ and δ the searched quantities, so the search space is the same at
//! every rung and a good point at a small rung is a hypothesis at the next.
//! Absolute caps (`MAX_BASE`) bound memory by construction; they are not
//! tuning knobs.
//!
//! # Known-dead regions
//!
//! Nothing here offers a design that `research/ic_sweeper_20261024/README.md`
//! §2.3 lists as falsified (4/5/6-sum extraction on compact orbits,
//! cyclotomic bases, u-frame symmetrisation, Z/3 grading, single large
//! primes on a bounded domain at `m = 2`, …).

use std::collections::BTreeMap;

use serde_json::{json, Value};

use super::trial::TrialConfig;

/// One point of the space: parameter name to chosen value.  Inactive
/// parameters (whose condition does not hold) are absent.
pub type Assignment = BTreeMap<String, Value>;

/// Absolute ceiling on a factor base's size, in abscissae or columns.  A
/// memory guard: a pair table over a larger base is not a trial, it is an
/// out-of-memory kill.
pub const MAX_BASE: u64 = 4096;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Track {
    Algebraic,
    Table,
}

impl Track {
    pub fn as_str(self) -> &'static str {
        match self {
            Track::Algebraic => "algebraic",
            Track::Table => "table",
        }
    }
    /// Whether results on this track count towards an improvement claim.
    pub fn counted(self) -> bool {
        self == Track::Algebraic
    }
}

#[derive(Clone, Debug)]
pub enum Domain {
    Cat(Vec<String>),
    Int { lo: i64, hi: i64, log: bool },
    Float { lo: f64, hi: f64, log: bool },
}

/// `param` must currently take one of `any_of`.  Only categorical parents.
#[derive(Clone, Debug)]
pub struct Cond {
    pub param: String,
    pub any_of: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Param {
    pub name: String,
    pub domain: Domain,
    /// All must hold (a conjunction) for the parameter to exist.
    pub when: Vec<Cond>,
    pub doc: String,
}

impl Param {
    pub fn cat(name: &str, values: &[&str], doc: &str) -> Param {
        Param {
            name: name.into(),
            domain: Domain::Cat(values.iter().map(|s| s.to_string()).collect()),
            when: vec![],
            doc: doc.into(),
        }
    }
    pub fn int(name: &str, lo: i64, hi: i64, log: bool, doc: &str) -> Param {
        Param {
            name: name.into(),
            domain: Domain::Int { lo, hi, log },
            when: vec![],
            doc: doc.into(),
        }
    }
    pub fn float(name: &str, lo: f64, hi: f64, log: bool, doc: &str) -> Param {
        Param {
            name: name.into(),
            domain: Domain::Float { lo, hi, log },
            when: vec![],
            doc: doc.into(),
        }
    }
    pub fn when(mut self, param: &str, any_of: &[&str]) -> Param {
        self.when.push(Cond {
            param: param.into(),
            any_of: any_of.iter().map(|s| s.to_string()).collect(),
        });
        self
    }
    /// Whether this parameter exists given the values chosen so far.
    pub fn active(&self, a: &Assignment) -> bool {
        self.when.iter().all(|c| {
            a.get(&c.param)
                .and_then(Value::as_str)
                .is_some_and(|v| c.any_of.iter().any(|x| x == v))
        })
    }
}

/// The instance a study runs on, apart from its size.
#[derive(Clone, Debug)]
pub struct InstanceKind {
    pub regime: String,
    pub koblitz_a: Option<u8>,
    pub family: Option<String>,
    pub max_cofactor: Option<u64>,
}

impl InstanceKind {
    pub fn describe(&self) -> String {
        match self.regime.as_str() {
            "koblitz" => format!(
                "koblitz a={}",
                self.koblitz_a.map_or("auto".into(), |a| a.to_string())
            ),
            "prime" => format!("prime {}", self.family.as_deref().unwrap_or("generic")),
            other => other.to_string(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    /// Built by `Space::custom` in tests: no mapping to a trial configuration.
    #[cfg_attr(not(test), allow(dead_code))]
    Custom,
    KoblitzAlgebraic,
    Char2Algebraic,
    KoblitzTable,
    PrimeTable,
}

#[derive(Clone, Debug)]
pub struct Space {
    pub name: &'static str,
    pub track: Track,
    pub doc: &'static str,
    pub params: Vec<Param>,
    kind: Kind,
    instance: InstanceKind,
}

pub const SPACE_NAMES: &[&str] = &[
    "koblitz-algebraic",
    "char2-algebraic",
    "koblitz-table",
    "prime-table",
];

/// The space called `name`, for an instance.
pub fn space_for(name: &str, instance: &InstanceKind) -> Result<Space, String> {
    let want = |regime: &str| {
        if instance.regime == regime {
            Ok(())
        } else {
            Err(format!(
                "space `{name}` is for the {regime} regime, not {}",
                instance.regime
            ))
        }
    };
    match name {
        "koblitz-algebraic" => {
            want("koblitz")?;
            Ok(algebraic_space(
                Kind::KoblitzAlgebraic,
                "koblitz-algebraic",
                true,
                instance,
            ))
        }
        "char2-algebraic" => {
            want("char2")?;
            Ok(algebraic_space(
                Kind::Char2Algebraic,
                "char2-algebraic",
                false,
                instance,
            ))
        }
        "koblitz-table" => {
            want("koblitz")?;
            Ok(koblitz_table_space(instance))
        }
        "prime-table" => {
            want("prime")?;
            Ok(prime_table_space(instance))
        }
        other => Err(format!(
            "unknown space `{other}`; the spaces are {}",
            SPACE_NAMES.join(", ")
        )),
    }
}

const MATRIX_SOLVERS: &[&str] = &["inherited-f4", "matrix-f4", "matrix-f5"];

fn algebraic_space(kind: Kind, name: &'static str, koblitz: bool, inst: &InstanceKind) -> Space {
    let bases: &[&str] = if koblitz {
        &["binary-subspace", "koblitz-closure"]
    } else {
        &["binary-subspace"]
    };
    let mut params = vec![
        Param::cat("m", &["2", "3"], "summands: 2 descends S3, 3 descends S4"),
        Param::cat(
            "base",
            bases,
            "factor base kind (a subspace base, as descent needs)",
        ),
        Param::float(
            "dim_frac",
            0.15,
            0.5,
            false,
            "subspace dimension as a fraction of the field degree n; the unknown count is m·dim",
        ),
    ];
    if koblitz {
        params.push(
            Param::cat(
                "slots",
                &["default", "all"],
                "descend over every Frobenius image of V (needs the closure base)",
            )
            .when("base", &["koblitz-closure"]),
        );
    }
    params.extend([
        Param::cat(
            "solver",
            &[
                "inherited-f4",
                "matrix-f4",
                "matrix-f5",
                "f4-f2",
                "crossbred-f2",
                "fes-f2",
                "sat-cdcl",
                "buchberger-f2",
            ],
            "the polynomial system solver",
        ),
        Param::int(
            "max_degree",
            3,
            6,
            false,
            "Macaulay degree cap before splitting",
        )
        .when("solver", MATRIX_SOLVERS),
        Param::int(
            "node_budget",
            256,
            65536,
            true,
            "algebraic reduction calls before a solve gives up (a count)",
        )
        .when("solver", MATRIX_SOLVERS),
        Param::int(
            "crossbred_D",
            2,
            5,
            false,
            "Macaulay degree of the Crossbred preprocessing",
        )
        .when("solver", &["crossbred-f2"]),
        Param::int(
            "crossbred_k",
            4,
            16,
            false,
            "variables Crossbred enumerates",
        )
        .when("solver", &["crossbred-f2"]),
        Param::int(
            "sat_conflicts",
            10_000,
            1_000_000,
            true,
            "CDCL conflict budget (a count, not a time)",
        )
        .when("solver", &["sat-cdcl"]),
        Param::cat("sat_xor", &["cnf", "native"], "parity encoding").when("solver", &["sat-cdcl"]),
        Param::cat(
            "linalg",
            &["incremental-gauss", "structured-gauss"],
            "relation matrix",
        ),
        Param::cat(
            "targets",
            &["walk", "random"],
            "how trial points are produced",
        ),
    ]);
    Space {
        name,
        track: Track::Algebraic,
        doc: "Relations from a descent-algebraic solve of S3 or S4 over a subspace base.",
        params,
        kind,
        instance: inst.clone(),
    }
}

fn koblitz_table_space(inst: &InstanceKind) -> Space {
    let params = vec![
        Param::cat("m", &["2", "3"], "summands of the Frobenius pair-table oracle"),
        Param::int("columns", 2, 64, true, "signed Frobenius orbits in the compact base (the family optimum is small and flat in n)"),
        Param::int("norm", 0, 128, false, "Z[τ] multiplier closure norm; 0 for the seeds alone"),
        Param::cat("large_primes", &["0", "1", "2"], "residuals outside the base a partial relation may carry").when("m", &["3"]),
        Param::float("lp_domain_factor", 2.0, 40.0, true, "large-prime domain as a multiple of |F|").when("large_primes", &["1", "2"]),
        Param::cat("linalg", &["incremental-gauss", "structured-gauss"], "relation matrix"),
        Param::cat("targets", &["walk", "random"], "how trial points are produced"),
    ];
    Space {
        name: "koblitz-table",
        track: Track::Table,
        doc: "Reference track: compact-orbit pair table (mitm-frobenius), with large primes and Z[τ] closure.",
        params,
        kind: Kind::KoblitzTable,
        instance: inst.clone(),
    }
}

fn prime_table_space(inst: &InstanceKind) -> Space {
    let family = inst.family.as_deref().unwrap_or("generic");
    let mut bases = vec!["prime-abscissa"];
    if matches!(family, "j0" | "j1728" | "d7" | "d8") {
        bases.extend(["glv-orbit", "cm-multiplier"]);
    }
    let params = vec![
        Param::cat("m", &["2", "3"], "summands of the pair-table oracle"),
        Param::cat("base", &bases, "factor base kind"),
        Param::float(
            "size_exp",
            0.08,
            0.45,
            false,
            "base size is 2^(size_exp · bits), capped at MAX_BASE",
        ),
        Param::int(
            "norm",
            0,
            64,
            false,
            "CM multiplier closure norm; 0 for the seeds alone",
        )
        .when("base", &["cm-multiplier"]),
        Param::cat("negation_folded", &["1"], "halve the table's additions"),
        Param::cat(
            "linalg",
            &["incremental-gauss", "structured-gauss"],
            "relation matrix",
        ),
        Param::cat(
            "targets",
            &["walk", "random"],
            "how trial points are produced",
        ),
    ];
    Space {
        name: "prime-table",
        track: Track::Table,
        doc: "Reference track: prime-field pair table (mitm) over abscissa, GLV-orbit or CM-multiplier bases.",
        params,
        kind: Kind::PrimeTable,
        instance: inst.clone(),
    }
}

// ── Assignment accessors ────────────────────────────────────────────

fn text<'a>(a: &'a Assignment, k: &str) -> Result<&'a str, String> {
    a.get(k)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("assignment has no `{k}`"))
}
fn int(a: &Assignment, k: &str) -> Result<i64, String> {
    a.get(k)
        .and_then(Value::as_i64)
        .ok_or_else(|| format!("assignment has no integer `{k}`"))
}
fn float(a: &Assignment, k: &str) -> Result<f64, String> {
    a.get(k)
        .and_then(Value::as_f64)
        .ok_or_else(|| format!("assignment has no number `{k}`"))
}

/// `2^(theta · scale)` rounded, clamped to `[lo, MAX_BASE]`.
fn scaled_size(theta: f64, scale: u32, lo: u64) -> u64 {
    ((2f64.powf(theta * scale as f64)).round() as u64).clamp(lo, MAX_BASE)
}

impl Space {
    /// A space with no pipeline behind it, for tests of the optimiser.
    #[cfg(test)]
    pub fn custom(name: &'static str, track: Track, params: Vec<Param>) -> Space {
        Space {
            name,
            track,
            doc: "custom",
            params,
            kind: Kind::Custom,
            instance: InstanceKind {
                regime: "custom".into(),
                koblitz_a: None,
                family: None,
                max_cofactor: None,
            },
        }
    }

    pub fn params_json(&self) -> Value {
        json!({
            "space": self.name,
            "track": self.track.as_str(),
            "counted": self.track.counted(),
            "instance": self.instance.describe(),
            "doc": self.doc,
            "parameters": self.params.iter().map(|p| json!({
                "name": p.name,
                "domain": match &p.domain {
                    Domain::Cat(v) => json!({"categorical": v}),
                    Domain::Int { lo, hi, log } => json!({"int": [lo, hi], "log": log}),
                    Domain::Float { lo, hi, log } => json!({"float": [lo, hi], "log": log}),
                },
                "when": p.when.iter().map(|c| json!({c.param.clone(): c.any_of.clone()})).collect::<Vec<_>>(),
                "doc": p.doc,
            })).collect::<Vec<_>>(),
        })
    }

    fn base_config(
        &self,
        scale: u32,
        factor_base: String,
        oracle: String,
        solver: Option<String>,
    ) -> TrialConfig {
        TrialConfig {
            regime: self.instance.regime.clone(),
            degree: scale,
            family: self.instance.family.clone(),
            koblitz_a: self.instance.koblitz_a,
            max_cofactor: self.instance.max_cofactor,
            factor_base,
            oracle,
            solver,
            targets: "walk".into(),
            linalg: "incremental-gauss".into(),
            max_trials: 2_000_000,
            seed: 0x1C_B0_0B_DA_7A,
            repeats: 1,
            rho_runs: 8,
        }
    }

    /// The trial configuration an assignment means at `scale` (the field
    /// degree for binary regimes, the subgroup bits for prime).
    pub fn build(&self, a: &Assignment, scale: u32) -> Result<TrialConfig, String> {
        match self.kind {
            Kind::Custom => Err("a custom space has no pipeline".into()),
            Kind::KoblitzAlgebraic | Kind::Char2Algebraic => {
                let m = text(a, "m")?;
                let n = scale as f64;
                let dimension = ((float(a, "dim_frac")? * n).round() as i64).clamp(2, 30);
                let base = text(a, "base")?;
                let factor_base = format!("{base}:dimension={dimension}");
                let mut oracle = format!("descent-algebraic:m={m}");
                if a.get("slots").and_then(Value::as_str) == Some("all") {
                    oracle.push_str(",slots=all");
                }
                let solver_name = text(a, "solver")?;
                let mut solver = solver_name.to_string();
                let mut kv: Vec<String> = Vec::new();
                match solver_name {
                    "inherited-f4" | "matrix-f4" | "matrix-f5" => {
                        kv.push(format!("max_degree={}", int(a, "max_degree")?));
                        kv.push(format!("node_budget={}", int(a, "node_budget")?));
                    }
                    "crossbred-f2" => {
                        kv.push(format!("D={}", int(a, "crossbred_D")?));
                        kv.push(format!("k={}", int(a, "crossbred_k")?));
                    }
                    "sat-cdcl" => {
                        kv.push(format!("sat_conflict_budget={}", int(a, "sat_conflicts")?));
                        kv.push(format!("sat_xor_encoding={}", text(a, "sat_xor")?));
                    }
                    _ => {}
                }
                if !kv.is_empty() {
                    solver = format!("{solver_name}:{}", kv.join(","));
                }
                let mut cfg = self.base_config(scale, factor_base, oracle, Some(solver));
                cfg.linalg = text(a, "linalg")?.into();
                cfg.targets = text(a, "targets")?.into();
                Ok(cfg)
            }
            Kind::KoblitzTable => {
                let m = text(a, "m")?;
                let columns = int(a, "columns")?.clamp(2, MAX_BASE as i64);
                let mut fb = format!("compact-orbit-scan:columns={columns},raw_x_cap=1073741824");
                let norm = int(a, "norm")?;
                if norm > 0 {
                    fb.push_str(&format!(",norm={norm}"));
                }
                let mut oracle = format!("mitm-frobenius:m={m}");
                if let Some(lp) = a.get("large_primes").and_then(Value::as_str) {
                    oracle.push_str(&format!(",large_primes={lp}"));
                    if lp != "0" {
                        oracle.push_str(&format!(
                            ",lp_domain_factor={:.3}",
                            float(a, "lp_domain_factor")?
                        ));
                    }
                }
                let mut cfg = self.base_config(scale, fb, oracle, None);
                cfg.linalg = text(a, "linalg")?.into();
                cfg.targets = text(a, "targets")?.into();
                Ok(cfg)
            }
            Kind::PrimeTable => {
                let m = text(a, "m")?;
                let size = scaled_size(float(a, "size_exp")?, scale, 4);
                let base = text(a, "base")?;
                let mut fb = format!("{base}:size={size}");
                if base == "cm-multiplier" {
                    let norm = int(a, "norm")?;
                    if norm > 0 {
                        fb.push_str(&format!(",norm={norm}"));
                    }
                }
                let oracle = format!("mitm:m={m},negation_folded={}", text(a, "negation_folded")?);
                let mut cfg = self.base_config(scale, fb, oracle, None);
                cfg.linalg = text(a, "linalg")?.into();
                cfg.targets = text(a, "targets")?.into();
                Ok(cfg)
            }
        }
    }

    /// The incumbent a space's trials are scored against, at `scale`.  Named
    /// and frozen before a search starts (AGENTS.md §1).  Provisional: it is
    /// the repository's own best-known arm, not published index calculus,
    /// which has no reference arm in the repository yet.
    pub fn baseline(&self, scale: u32) -> TrialConfig {
        match self.kind {
            Kind::Custom => unreachable!("a custom space has no baseline"),
            // Inherited F4 on the Weil descent of S3 over span{1..z^{d-1}},
            // d = ⌊(n+1)/3⌋.  The ladder report's algebraic baseline is the
            // S4 (m = 3) descent in ecbench's `pdp3-koblitz`; through this
            // framework's `descent-algebraic` the m = 3 arm finishes only at
            // n = 13 (n = 15 passes 600 s), which leaves a study one rung, so
            // the baseline is the m = 2 arm, which scores on n = 13..23 in
            // 1 s, 1 s, 1 s and 27 s.  Provisional, and not published index
            // calculus.
            Kind::KoblitzAlgebraic | Kind::Char2Algebraic => self.base_config(
                scale,
                format!("binary-subspace:dimension={}", ((scale + 1) / 3).max(2)),
                "descent-algebraic:m=2".into(),
                Some("inherited-f4".into()),
            ),
            // The leaderboard's Koblitz recipe: a small compact base, m = 3.
            Kind::KoblitzTable => self.base_config(
                scale,
                "compact-orbit-scan:columns=8,raw_x_cap=1073741824".into(),
                "mitm-frobenius:m=3".into(),
                None,
            ),
            // The leaderboard's prime recipe: a folded table, m = 2, base at
            // the family optimum ≈ 2^(bits/3).
            Kind::PrimeTable => self.base_config(
                scale,
                format!("prime-abscissa:size={}", scaled_size(1.0 / 3.0, scale, 4)),
                "mitm:m=2,negation_folded=1".into(),
                None,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn koblitz0() -> InstanceKind {
        InstanceKind {
            regime: "koblitz".into(),
            koblitz_a: Some(0),
            family: None,
            max_cofactor: None,
        }
    }

    fn sample(a: &mut Assignment, k: &str, v: Value) {
        a.insert(k.into(), v);
    }

    #[test]
    fn every_space_builds_for_its_instance_and_refuses_the_wrong_one() {
        assert!(space_for("koblitz-algebraic", &koblitz0()).is_ok());
        assert!(space_for("koblitz-table", &koblitz0()).is_ok());
        assert!(space_for("prime-table", &koblitz0()).is_err());
        assert!(space_for("no-such-space", &koblitz0())
            .unwrap_err()
            .contains("koblitz-algebraic"));
    }

    #[test]
    fn parents_come_before_the_parameters_that_depend_on_them() {
        let prime = InstanceKind {
            regime: "prime".into(),
            koblitz_a: None,
            family: Some("j0".into()),
            max_cofactor: None,
        };
        let char2 = InstanceKind {
            regime: "char2".into(),
            koblitz_a: None,
            family: None,
            max_cofactor: Some(8),
        };
        for space in [
            space_for("koblitz-algebraic", &koblitz0()).unwrap(),
            space_for("char2-algebraic", &char2).unwrap(),
            space_for("koblitz-table", &koblitz0()).unwrap(),
            space_for("prime-table", &prime).unwrap(),
        ] {
            let mut seen = Vec::new();
            for p in &space.params {
                for c in &p.when {
                    assert!(
                        seen.contains(&c.param),
                        "{}: `{}` is conditioned on `{}`, which comes later or does not exist",
                        space.name,
                        p.name,
                        c.param
                    );
                }
                seen.push(p.name.clone());
            }
        }
    }

    #[test]
    fn conditions_activate_only_on_the_named_values() {
        let space = space_for("koblitz-algebraic", &koblitz0()).unwrap();
        let crossbred_d = space
            .params
            .iter()
            .find(|p| p.name == "crossbred_D")
            .unwrap();
        let mut a = Assignment::new();
        sample(&mut a, "solver", json!("crossbred-f2"));
        assert!(crossbred_d.active(&a));
        sample(&mut a, "solver", json!("sat-cdcl"));
        assert!(!crossbred_d.active(&a));
        assert!(
            !crossbred_d.active(&Assignment::new()),
            "a missing parent means inactive"
        );
    }

    #[test]
    fn an_assignment_becomes_the_configuration_it_names() {
        let space = space_for("koblitz-algebraic", &koblitz0()).unwrap();
        let mut a = Assignment::new();
        for (k, v) in [
            ("m", json!("2")),
            ("base", json!("koblitz-closure")),
            ("dim_frac", json!(0.3)),
            ("slots", json!("all")),
            ("solver", json!("crossbred-f2")),
            ("crossbred_D", json!(3)),
            ("crossbred_k", json!(8)),
            ("linalg", json!("structured-gauss")),
            ("targets", json!("walk")),
        ] {
            sample(&mut a, k, v);
        }
        let cfg = space.build(&a, 23).unwrap();
        assert_eq!(cfg.factor_base, "koblitz-closure:dimension=7");
        assert_eq!(cfg.oracle, "descent-algebraic:m=2,slots=all");
        assert_eq!(cfg.solver.as_deref(), Some("crossbred-f2:D=3,k=8"));
        assert_eq!(cfg.linalg, "structured-gauss");
        assert_eq!(
            (cfg.regime.as_str(), cfg.degree, cfg.koblitz_a),
            ("koblitz", 23, Some(0))
        );
    }

    #[test]
    fn size_knobs_are_scale_relative_and_capped() {
        // The same θ means a larger base on a larger instance…
        assert!(scaled_size(0.3, 40, 4) > scaled_size(0.3, 20, 4));
        // …and no θ can ask for more than the memory guard allows.
        assert_eq!(scaled_size(0.45, 200, 4), MAX_BASE);
        assert_eq!(scaled_size(0.0, 20, 4), 4);
    }

    #[test]
    fn every_space_has_a_baseline_that_parses_as_a_configuration() {
        let prime = InstanceKind {
            regime: "prime".into(),
            koblitz_a: None,
            family: Some("generic".into()),
            max_cofactor: None,
        };
        for space in [
            space_for("koblitz-algebraic", &koblitz0()).unwrap(),
            space_for("koblitz-table", &koblitz0()).unwrap(),
            space_for("prime-table", &prime).unwrap(),
        ] {
            let cfg = space.baseline(19);
            assert!(cfg.bench_args().is_ok(), "{}: {:?}", space.name, cfg);
        }
    }

    #[test]
    fn only_the_algebraic_track_is_counted() {
        assert!(space_for("koblitz-algebraic", &koblitz0())
            .unwrap()
            .track
            .counted());
        assert!(!space_for("koblitz-table", &koblitz0())
            .unwrap()
            .track
            .counted());
    }

    #[test]
    fn generic_prime_curves_are_not_offered_automorphism_bases() {
        let generic = InstanceKind {
            regime: "prime".into(),
            koblitz_a: None,
            family: Some("generic".into()),
            max_cofactor: None,
        };
        let j0 = InstanceKind {
            regime: "prime".into(),
            koblitz_a: None,
            family: Some("j0".into()),
            max_cofactor: None,
        };
        let bases = |i: &InstanceKind| match &space_for("prime-table", i)
            .unwrap()
            .params
            .iter()
            .find(|p| p.name == "base")
            .unwrap()
            .domain
        {
            Domain::Cat(v) => v.clone(),
            _ => unreachable!(),
        };
        assert_eq!(bases(&generic), vec!["prime-abscissa"]);
        assert!(bases(&j0).contains(&"glv-orbit".to_string()));
    }
}
