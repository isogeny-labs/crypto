//! # The planner: an `ic.pipeline` configuration for a curve, from measured evidence
//!
//! Given a curve, pick every stage of the pipeline (factor base and its
//! size, decomposition oracle and its table builder, relation matrix and stop
//! rule, target source) the way the repository's measurements say to, and
//! say how far that configuration is from the matched Pollard rho.
//!
//! Every rule names the study it rests on ([`Evidence`]) and how far the curve
//! is from what was measured ([`Support`]): a rule applied outside the sizes it
//! was measured at is reported as extrapolated, never as known.  The planner
//! measures nothing and changes no counted unit; it only reads choices off the
//! evidence, so a plan is a hypothesis about the best configuration until a
//! session confirms it on that curve.
//!
//! The rules, by curve class:
//!
//! | class | factor base | oracle | relation matrix | targets |
//! |:--|:--|:--|:--|:--|
//! | prime (any `j`) | `prime-abscissa`, `round(#E^{1/3}/2)` abscissae | `mitm-prime-blocked:m=2` (`p < 2^62`), else `mitm:m=2,negation_folded=1` | `incremental-gauss` (target pinned); `sparse-cado` for a log database | `walk` |
//! | random binary | `binary-subspace`, dimension `round(log2 #E / 3)` | `mitm:m=2,negation_folded=1` | as above | `walk` |
//! | Koblitz, odd `n < 64` | `compact-orbit-scan`, `max(1, round(0.6 (r/n³)^{1/3}))` orbit columns | `mitm-frobenius-counted:m=2` | as above | `stride-renew` for `log2 r ≤ 21`, else `walk` |
//!
//! Anything else (extension fields, even-degree or wide Koblitz curves, genus
//! 2) has no measured `ic.pipeline` recipe: the plan names the rho reference
//! and no index-calculus configuration.

use std::collections::BTreeMap;

use serde::Serialize;

use crate::cryptanalysis::ic_boundary::{BinaryInstance, PrimeInstance};

/// The planner's rule set.  A spec that pins a plan names this version; a
/// change to any rule or calibration constant changes it.
pub const PLANNER_VERSION: &str = "ic-planner/1";

/// What kind of curve the plan is for: the classes the evidence separates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CurveClass {
    /// A prime-field curve with no automorphism beyond negation.
    Prime,
    /// A prime-field curve with `j = 0` (`a = 0`): an order-6 automorphism.
    PrimeJ0,
    /// A prime-field curve with `j = 1728` (`b = 0`): an order-4 automorphism.
    PrimeJ1728,
    /// An ordinary binary curve without Frobenius structure over `F_2`.
    BinaryRandom,
    /// `K_a / F_{2^n}`: Frobenius acts on the subgroup.
    Koblitz,
    /// Anything the evidence has no `ic.pipeline` recipe for.
    Other,
}

/// The facts about a curve the rules read.
#[derive(Clone, Debug, Serialize)]
pub struct CurveProfile {
    pub class: CurveClass,
    /// Field characteristic for a prime field.
    pub p: Option<u64>,
    /// Extension degree for a binary field.
    pub n: Option<u32>,
    pub group_order: f64,
    pub r: f64,
    pub cofactor: u64,
}

impl CurveProfile {
    pub fn log2_r(&self) -> f64 {
        self.r.log2()
    }

    /// A prime-field instance's profile; `j = 0` and `j = 1728` are told
    /// apart by `a = 0` and `b = 0`.
    pub fn of_prime(inst: &PrimeInstance) -> Self {
        let class = if inst.curve.a == 0 {
            CurveClass::PrimeJ0
        } else if inst.curve.b == 0 {
            CurveClass::PrimeJ1728
        } else {
            CurveClass::Prime
        };
        Self {
            class,
            p: Some(inst.curve.p),
            n: None,
            group_order: inst.group_order as f64,
            r: inst.r as f64,
            cofactor: inst.cofactor,
        }
    }

    /// A binary-field instance's profile: Koblitz when the instance carries
    /// its Frobenius structure.
    pub fn of_binary(inst: &BinaryInstance) -> Self {
        Self {
            class: if inst.koblitz.is_some() {
                CurveClass::Koblitz
            } else {
                CurveClass::BinaryRandom
            },
            p: None,
            n: Some(inst.n),
            group_order: inst.group_order as f64,
            r: inst.r as f64,
            cofactor: inst.cofactor,
        }
    }

    /// A curve the planner has no recipe for, kept so the plan can still name
    /// the rho reference.
    pub fn other(group_order: f64, r: f64, cofactor: u64) -> Self {
        Self {
            class: CurveClass::Other,
            p: None,
            n: None,
            group_order,
            r,
            cofactor,
        }
    }
}

/// What the logarithms are for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Goal {
    /// One target per run, the leaderboard's primary comparison: stop as soon
    /// as the target's column is pinned.
    OneTarget,
    /// Every factor-base logarithm, for a database many targets reuse
    /// (`ic logs`): the matrix must reach full rank.
    LogDatabase,
}

impl std::str::FromStr for Goal {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "one-target" => Ok(Goal::OneTarget),
            "log-database" => Ok(Goal::LogDatabase),
            other => Err(format!("goal `{other}`: use one-target or log-database")),
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct PlanOptions {
    pub goal: Goal,
    /// Memory the pair table may take; the base is shrunk to fit.
    pub max_table_bytes: f64,
}

impl Default for PlanOptions {
    fn default() -> Self {
        Self {
            goal: Goal::OneTarget,
            max_table_bytes: 8.0 * (1u64 << 30) as f64,
        }
    }
}

/// A file in this repository and the measured claim a rule takes from it.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct Evidence {
    pub path: &'static str,
    pub claim: &'static str,
}

/// How close the curve is to what a rule was measured on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Support {
    /// Inside the sizes the rule's evidence covers.
    Measured,
    /// Between measured sizes, on a fitted rule.
    Interpolated,
    /// Outside every measured size: the rule's form is assumed to hold.
    Extrapolated,
    /// Nothing measured: a default, not a finding.
    Unmeasured,
}

/// One stage's choice and why.
#[derive(Clone, Debug, Serialize)]
pub struct Choice {
    /// `factor_base`, `oracle`, `linalg` or `targets`: the ecbench
    /// `ic.pipeline` parameter it sets.
    pub stage: &'static str,
    pub value: String,
    pub rule: &'static str,
    pub reason: String,
    pub support: Support,
    pub evidence: Vec<Evidence>,
}

/// Where the plan stands against the matched rho.
#[derive(Clone, Debug, Serialize)]
pub struct RhoOutlook {
    /// The ecbench method of the matched reference.
    pub reference: &'static str,
    /// Expected `S_IC / S_rho` for the planned configuration.
    pub ic_over_rho: Option<f64>,
    pub support: Support,
    pub basis: String,
    pub evidence: Vec<Evidence>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Plan {
    pub planner: &'static str,
    pub profile: CurveProfile,
    pub goal: Goal,
    /// Empty when the class has no recipe.
    pub choices: Vec<Choice>,
    pub rho: RhoOutlook,
    /// One line: what to run.
    pub verdict: String,
    pub warnings: Vec<String>,
}

impl Plan {
    /// The value chosen for an `ic.pipeline` parameter.
    pub fn get(&self, stage: &str) -> Option<&str> {
        self.choices
            .iter()
            .find(|c| c.stage == stage)
            .map(|c| c.value.as_str())
    }

    /// The `ic.pipeline` parameters, or `None` when there is no recipe.
    pub fn pipeline_params(&self) -> Option<BTreeMap<String, String>> {
        if self.choices.is_empty() {
            return None;
        }
        Some(
            self.choices
                .iter()
                .map(|c| (c.stage.to_string(), c.value.clone()))
                .collect(),
        )
    }

    /// An ecbench spec arm running this plan, or `None` without a recipe.
    pub fn ecbench_arm(&self, name: &str, role: &str) -> Option<serde_json::Value> {
        let params = self.pipeline_params()?;
        Some(serde_json::json!({
            "name": name,
            "role": role,
            "method": { "id": "ic.pipeline", "params": params },
        }))
    }
}

// ── Evidence ───────────────────────────────────────────────────────

const E_LADDER: Evidence = Evidence {
    path: "docs/ic/LEADERBOARD.md",
    claim: "Table B: the folded table with walked targets and the base at the family optimum, m = 2, is the best recipe on 11 of the 13 prime and random-binary rows (9.7–24.4 bits), 5.47–13.5× rho on prime fields and 10.6–38.2× on binary",
};
const E_LADDER_JSON: Evidence = Evidence {
    path: "docs/ic/leaderboard.json",
    claim: "board[].recipes: on every Table B row walked targets beat random draws of the same base and table (e.g. 8.93× against 76.7× rho at 18 bits), and m = 2 beats m = 3",
};
const E_HOLDOUT20: Evidence = Evidence {
    path: "research/ecbench_rootfinding_batchtable_20261013/followup/reports/holdout/REPORT.md",
    claim: "20-bit holdout: 48 abscissae 6,980 GAE against 7,613 at 40 and 7,970 at 64 (B48/B64 0.876 [0.819, 0.937]); 6.67× rho",
};
const E_SWEEP20: Evidence = Evidence {
    path: "research/ecbench_rootfinding_batchtable_20261013/followup/reports/sweep/REPORT.md",
    claim: "20-bit B64: target-pinned stopping 0.431 [0.399, 0.470] of checked full rank; direct subtraction 42.4× the pair table",
};
const E_BLOCKED32: Evidence = Evidence {
    path: "research/ic_oracle_setup_blocked_20261020/README.md",
    claim: "32-bit, N = 1,024–16,384: mitm-prime-blocked builds the same table 0.47–0.24× the batched and 0.25–0.13× the scalar builder's set-up wall at half the memory, identical counts; smallest gap 33.0× rho at 1,024 abscissae, target pinned at about 0.5 relations per column",
};
const E_LINALG32: Evidence = Evidence {
    path: "research/ic_linalg_pipeline_20261019/README.md",
    claim: "32-bit: sparse-cado is forced to full rank (about 4.75 relations per column); at full rank its linear-algebra wall is 0.72–0.25× dense's with S unchanged",
};
const E_LA_ENGINES: Evidence = Evidence {
    path: "research/la_engines_20261014/README.md",
    claim: "linear algebra alone, 16k–262k columns: the CADO-style filter is 2.48–10.40× faster than the legacy filter",
};
const E_ROOTFIND: Evidence = Evidence {
    path: "research/ecbench_rootfind_engine_20261012/README.md",
    claim: "prime panel 18–24 bits: rootfind-subtract equals subtract in counted operations (about 2.5× faster wall) and costs 310–2,775 S against the pair table's 10–44",
};
const E_GLV: Evidence = Evidence {
    path: "research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md",
    claim: "§5–§6: on j = 0 / j = 1728 curves the glv-orbit fold divides columns by 3 / 2, but with the pair table S moves only 1.00–1.13× (the table is 85–99% of S)",
};
const E_FROB: Evidence = Evidence {
    path: "research/ecbench_frobenius_orbit_vs_rho_20261014/README.md",
    claim: "Koblitz n13–n41: compact-orbit-scan + mitm-frobenius-counted:m=2 is 1.57–18.2× rho-frob; orbit ÷ subspace-negfold 0.065–0.511; stride-renew ÷ walk 0.40–0.75 for log2 r ≤ 21 and about 1 above",
};
const E_FROB_PROTOCOL: Evidence = Evidence {
    path: "research/ecbench_frobenius_orbit_vs_rho_20261014/PROTOCOL.md",
    claim: "chosen orbit columns per curve (lowest S of the counts tried): 1, 1, 2, 3, 1, 2, 8, 90 at n13–n41; model optimum c ∝ (r/n³)^{1/3}",
};

// ── Rules ──────────────────────────────────────────────────────────

/// Measured log2 r span of the prime base-size rule: the Table B ladder
/// (9.7–23.4 bits), the 20-bit holdout and the 32-bit sessions.
const PRIME_SIZE_SPAN: (f64, f64) = (9.68, 32.0);
/// Same, random binary (Table B).
const BINARY_SIZE_SPAN: (f64, f64) = (13.99, 24.41);
/// Same, Koblitz (the Frobenius-orbit panel).
const KOBLITZ_SPAN: (f64, f64) = (10.97, 39.0);

/// Calibration of the orbit-column rule `c = κ (r/n³)^{1/3}` on the
/// Frobenius-orbit panel's eight chosen counts: κ = 0.6 reproduces six of
/// them exactly and the other two (n37, n41) within 1.33×.
const KOBLITZ_COLUMN_KAPPA: f64 = 0.6;

/// Bytes per pair-table entry, worst case: the packed one-word layout at its
/// load (`mitm-prime-blocked`, p < 2^32) plus the gathered sums, and a
/// hash-map entry otherwise.
fn table_bytes_per_entry(p: Option<u64>) -> f64 {
    match p {
        Some(p) if p <= u64::from(u32::MAX) => 24.0,
        _ => 40.0,
    }
}

fn span_support(log2_r: f64, span: (f64, f64)) -> Support {
    if log2_r >= span.0 - 0.5 && log2_r <= span.1 + 0.5 {
        Support::Measured
    } else {
        Support::Extrapolated
    }
}

/// `S_IC / S_rho` read off measured `(log2 r, ratio)` points: piecewise
/// log-linear between them, and the end segment's slope beyond.
fn interpolate_ratio(points: &[(f64, f64)], log2_r: f64) -> (f64, Support) {
    let mut pts: Vec<(f64, f64)> = points.to_vec();
    pts.sort_by(|a, b| a.0.total_cmp(&b.0));
    let ln = |(x, y): (f64, f64)| (x, y.ln());
    if pts.len() == 1 {
        return (pts[0].1, Support::Extrapolated);
    }
    let (lo, hi) = (pts[0].0, pts[pts.len() - 1].0);
    let seg = if log2_r <= lo {
        (ln(pts[0]), ln(pts[1]))
    } else if log2_r >= hi {
        (ln(pts[pts.len() - 2]), ln(pts[pts.len() - 1]))
    } else {
        let i = pts.iter().position(|p| p.0 >= log2_r).unwrap().max(1);
        (ln(pts[i - 1]), ln(pts[i]))
    };
    let ((x0, y0), (x1, y1)) = seg;
    let y = y0 + (y1 - y0) * (log2_r - x0) / (x1 - x0);
    let exact = pts.iter().any(|p| (p.0 - log2_r).abs() < 0.25);
    let support = if exact {
        Support::Measured
    } else if log2_r > lo && log2_r < hi {
        Support::Interpolated
    } else {
        Support::Extrapolated
    };
    (y.exp(), support)
}

/// Measured `S_IC / S_rho` (ecbench GAE, matched `rho.negation`) for the
/// prime recipe: the 20-bit holdout and the 32-bit blocked-table session.
const PRIME_RHO_POINTS: [(f64, f64); 2] = [(19.10, 6.6671), (32.0, 32.955)];
/// Table B ledger points (ledger GAE), random binary.
const BINARY_RHO_POINTS: [(f64, f64); 5] = [
    (13.99, 13.069),
    (15.00, 10.621),
    (20.00, 17.986),
    (21.00, 23.025),
    (24.41, 38.211),
];
/// The Frobenius-orbit panel, against `rho.signed_frobenius` (fully priced).
const KOBLITZ_RHO_POINTS: [(f64, f64); 8] = [
    (10.97, 2.956),
    (15.37, 1.566),
    (16.00, 1.719),
    (17.00, 2.805),
    (20.46, 3.664),
    (21.00, 3.847),
    (27.78, 5.048),
    (39.00, 18.16),
];

/// The plan for a curve.
pub fn plan(profile: &CurveProfile, opts: &PlanOptions) -> Plan {
    let mut warnings = Vec::new();
    let mut choices = match profile.class {
        CurveClass::Prime | CurveClass::PrimeJ0 | CurveClass::PrimeJ1728 => {
            prime_choices(profile, opts, &mut warnings)
        }
        CurveClass::BinaryRandom => binary_choices(profile, opts, &mut warnings),
        CurveClass::Koblitz => match profile.n {
            Some(n) if n % 2 == 1 && n < 64 => koblitz_choices(profile, n, &mut warnings),
            _ => {
                warnings.push(
                    "Koblitz curve with even or wide n: the Frobenius-orbit base needs odd n < 64; \
                     no measured ic.pipeline recipe"
                        .into(),
                );
                Vec::new()
            }
        },
        CurveClass::Other => {
            warnings.push("no measured ic.pipeline recipe for this curve class".into());
            Vec::new()
        }
    };
    if !choices.is_empty() {
        choices.push(linalg_choice(opts.goal));
    }
    let rho = rho_outlook(profile, !choices.is_empty(), opts.goal);
    let verdict = match (choices.is_empty(), rho.ic_over_rho) {
        (true, _) => format!("run `{}`: no index-calculus recipe applies", rho.reference),
        (false, Some(x)) if x > 1.0 => format!(
            "`{}` is cheaper: the planned index calculus is expected at about {x:.1}× its S ({:?}); \
             run the plan only to measure or improve index calculus",
            rho.reference, rho.support
        ),
        (false, Some(x)) => format!(
            "the planned index calculus is expected below `{}` (about {x:.2}×, {:?}); confirm it in a paired session",
            rho.reference, rho.support
        ),
        (false, None) => format!("no measured comparison with `{}`", rho.reference),
    };
    Plan {
        planner: PLANNER_VERSION,
        profile: profile.clone(),
        goal: opts.goal,
        choices,
        rho,
        verdict,
        warnings,
    }
}

fn prime_choices(
    profile: &CurveProfile,
    opts: &PlanOptions,
    warnings: &mut Vec<String>,
) -> Vec<Choice> {
    let log2_r = profile.log2_r();
    // The family optimum: about #E^{1/3} signed points, so half that in
    // abscissae, capped so the table fits the memory budget.
    let ideal = (profile.group_order.cbrt() / 2.0).round().max(4.0);
    let per_entry = table_bytes_per_entry(profile.p);
    let cap = (opts.max_table_bytes / per_entry).sqrt().floor().max(4.0);
    let abscissae = ideal.min(cap) as u64;
    let mut size_support = span_support(log2_r, PRIME_SIZE_SPAN);
    let mut size_reason = format!(
        "about #E^(1/3)/2 = {ideal:.0} abscissae, the family optimum that balances the |F|²/4-addition table against the relation trials"
    );
    if abscissae < ideal as u64 {
        size_reason += &format!(
            "; capped at {abscissae} so the table fits {:.1} GiB (above the optimum's cost)",
            opts.max_table_bytes / (1u64 << 30) as f64
        );
        size_support = Support::Extrapolated;
        warnings.push(format!(
            "pair table capped by memory: {abscissae} abscissae instead of {ideal:.0}"
        ));
    }
    if profile.class != CurveClass::Prime {
        size_reason += "; the CM fold is not used: with the pair table it moves S by at most 1.13×";
    }
    let mut fb_evidence = vec![E_LADDER, E_HOLDOUT20, E_BLOCKED32];
    if profile.class != CurveClass::Prime {
        fb_evidence.push(E_GLV);
    }
    let fast = profile.p.is_some_and(|p| p < (1u64 << 62));
    let oracle = if fast {
        Choice {
            stage: "oracle",
            value: "mitm-prime-blocked:m=2".into(),
            rule: "prime.oracle.blocked-table",
            reason: "two-summand pair table (m = 2 beats m = 3 and root finding by 4–60× in S); the cache-blocked \
                     builder makes the identical table 2–8× faster than the others"
                .into(),
            support: Support::Measured,
            evidence: vec![E_LADDER_JSON, E_ROOTFIND, E_SWEEP20, E_BLOCKED32],
        }
    } else {
        Choice {
            stage: "oracle",
            value: "mitm:m=2,negation_folded=1".into(),
            rule: "prime.oracle.folded-table",
            reason: "two-summand negation-folded pair table; the batched builders need p < 2^62"
                .into(),
            support: Support::Measured,
            evidence: vec![E_LADDER_JSON, E_ROOTFIND, E_SWEEP20],
        }
    };
    vec![
        Choice {
            stage: "factor_base",
            value: format!("prime-abscissa:size={abscissae}"),
            rule: "prime.factor-base.family-optimum",
            reason: size_reason,
            support: size_support,
            evidence: fb_evidence,
        },
        oracle,
        Choice {
            stage: "targets",
            value: "walk".into(),
            rule: "targets.walk",
            reason: "walked targets beat fresh random draws on every Table B row of the same base"
                .into(),
            support: Support::Measured,
            evidence: vec![E_LADDER_JSON],
        },
    ]
}

fn binary_choices(
    profile: &CurveProfile,
    opts: &PlanOptions,
    warnings: &mut Vec<String>,
) -> Vec<Choice> {
    let log2_r = profile.log2_r();
    let ideal = (profile.group_order.log2() / 3.0).round().max(2.0);
    // A dimension-l subspace holds about 2^l abscissae, 2^{l+1} points.
    let cap = ((opts.max_table_bytes / 40.0).sqrt().log2() - 1.0)
        .floor()
        .max(2.0);
    let dimension = ideal.min(cap) as u32;
    let mut support = span_support(log2_r, BINARY_SIZE_SPAN);
    if (dimension as f64) < ideal {
        support = Support::Extrapolated;
        warnings.push(format!(
            "pair table capped by memory: dimension {dimension} instead of {ideal:.0}"
        ));
    }
    let _ = opts;
    vec![
        Choice {
            stage: "factor_base",
            value: format!("binary-subspace:dimension={dimension}"),
            rule: "binary.factor-base.family-optimum",
            reason: format!(
                "subspace of dimension round(log2 #E / 3) = {ideal:.0}: about #E^(1/3) abscissae, the family optimum"
            ),
            support,
            evidence: vec![E_LADDER],
        },
        Choice {
            stage: "oracle",
            value: "mitm:m=2,negation_folded=1".into(),
            rule: "binary.oracle.folded-table",
            reason: "two-summand negation-folded pair table, the ladder's best on every random-binary row".into(),
            support: Support::Measured,
            evidence: vec![E_LADDER, E_LADDER_JSON],
        },
        Choice {
            stage: "targets",
            value: "walk".into(),
            rule: "targets.walk",
            reason: "walked targets beat fresh random draws on every Table B row of the same base".into(),
            support: Support::Measured,
            evidence: vec![E_LADDER_JSON],
        },
    ]
}

fn koblitz_choices(profile: &CurveProfile, n: u32, warnings: &mut Vec<String>) -> Vec<Choice> {
    let log2_r = profile.log2_r();
    let columns = (KOBLITZ_COLUMN_KAPPA * (profile.r / f64::from(n).powi(3)).cbrt())
        .round()
        .max(1.0) as u64;
    let support = span_support(log2_r, KOBLITZ_SPAN);
    if support == Support::Extrapolated {
        warnings.push("Koblitz orbit-column rule is calibrated on log2 r 11–39 only".into());
    }
    let renew = log2_r <= 21.5;
    vec![
        Choice {
            stage: "factor_base",
            value: format!("compact-orbit-scan:columns={columns},raw_x_cap=100000000"),
            rule: "koblitz.factor-base.frobenius-orbit",
            reason: format!(
                "one column per signed Frobenius orbit; {columns} columns from c = {KOBLITZ_COLUMN_KAPPA}·(r/n³)^(1/3), \
                 calibrated on the eight chosen counts of the orbit panel; the orbit base beats the subspace base by 2–15×"
            ),
            support,
            evidence: vec![E_FROB_PROTOCOL, E_FROB],
        },
        Choice {
            stage: "oracle",
            value: "mitm-frobenius-counted:m=2".into(),
            rule: "koblitz.oracle.frobenius-table",
            reason: "pair table stored modulo Frobenius, with its set-up counted natively".into(),
            support,
            evidence: vec![E_FROB],
        },
        Choice {
            stage: "targets",
            value: if renew { "stride-renew" } else { "walk" }.into(),
            rule: "koblitz.targets.stride-renew",
            reason: if renew {
                "stride-renew costs 0.40–0.75× the walk at log2 r ≤ 21".into()
            } else {
                "above log2 r 21 stride-renew and the walk measure the same (ratio about 1)".into()
            },
            support,
            evidence: vec![E_FROB],
        },
    ]
}

fn linalg_choice(goal: Goal) -> Choice {
    match goal {
        Goal::OneTarget => Choice {
            stage: "linalg",
            value: "incremental-gauss".into(),
            rule: "linalg.target-pinned",
            reason: "stop when the target's column is pinned: about 0.5 relations per column, against 2–5 for full rank; \
                     sparse-cado would force full rank and multiply the relation trials"
                .into(),
            support: Support::Measured,
            evidence: vec![E_SWEEP20, E_BLOCKED32, E_LINALG32],
        },
        Goal::LogDatabase => Choice {
            stage: "linalg",
            value: "sparse-cado".into(),
            rule: "linalg.full-rank-sparse",
            reason: "every base logarithm needs full rank; there the CADO-style filter and block Wiedemann run 1.4–10× \
                     faster than the dense or legacy-filter elimination, with the same relations"
                .into(),
            support: Support::Measured,
            evidence: vec![E_LINALG32, E_LA_ENGINES],
        },
    }
}

fn rho_outlook(profile: &CurveProfile, has_recipe: bool, goal: Goal) -> RhoOutlook {
    let log2_r = profile.log2_r();
    let (reference, points, basis, evidence): (&'static str, &[(f64, f64)], &str, Vec<Evidence>) =
        match profile.class {
            CurveClass::Prime | CurveClass::PrimeJ0 | CurveClass::PrimeJ1728 => (
                "rho.negation",
                &PRIME_RHO_POINTS,
                "ecbench GAE, matched rho.negation, at 19.1 bits (48 abscissae) and 32 bits (1,024 abscissae)",
                vec![E_HOLDOUT20, E_BLOCKED32],
            ),
            CurveClass::BinaryRandom => (
                "rho.negation",
                &BINARY_RHO_POINTS,
                "Table B ledger GAE (ic boundary), not ecbench GAE",
                vec![E_LADDER],
            ),
            CurveClass::Koblitz => (
                "rho.signed_frobenius",
                &KOBLITZ_RHO_POINTS,
                "fully priced ecbench GAE against rho.signed_frobenius",
                vec![E_FROB],
            ),
            CurveClass::Other => ("rho.negation", &[], "", Vec::new()),
        };
    if !has_recipe || points.is_empty() {
        return RhoOutlook {
            reference,
            ic_over_rho: None,
            support: Support::Unmeasured,
            basis: "no index-calculus recipe to compare".into(),
            evidence,
        };
    }
    if goal == Goal::LogDatabase {
        return RhoOutlook {
            reference,
            ic_over_rho: None,
            support: Support::Unmeasured,
            basis: "the measured gaps are one-target; a log database is paid once and amortised over its \
                    targets, which no session here has priced against rho"
                .into(),
            evidence,
        };
    }
    let (ratio, support) = interpolate_ratio(points, log2_r);
    RhoOutlook {
        reference,
        ic_over_rho: Some(ratio),
        support,
        basis: format!("{basis}; piecewise log-linear in log2 r between measured points"),
        evidence,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prime(log2_r: f64, p: u64) -> CurveProfile {
        let r = 2f64.powf(log2_r);
        CurveProfile {
            class: CurveClass::Prime,
            p: Some(p),
            n: None,
            group_order: r,
            r,
            cofactor: 1,
        }
    }

    #[test]
    fn every_cited_file_exists() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        for e in [
            E_LADDER,
            E_LADDER_JSON,
            E_HOLDOUT20,
            E_SWEEP20,
            E_BLOCKED32,
            E_LINALG32,
            E_LA_ENGINES,
            E_ROOTFIND,
            E_GLV,
            E_FROB,
            E_FROB_PROTOCOL,
        ] {
            assert!(
                root.join(e.path).is_file(),
                "missing evidence file {}",
                e.path
            );
        }
    }

    #[test]
    fn prime_32_bit_matches_the_measured_best() {
        let pl = plan(&prime(32.0, 4_294_967_291), &PlanOptions::default());
        // The measured best of 1,024..16,384 was 1,024, and the unrecorded
        // pilot put the optimum between 768 and 1,024.
        let fb = pl.get("factor_base").unwrap();
        let n: u64 = fb
            .trim_start_matches("prime-abscissa:size=")
            .parse()
            .unwrap();
        assert!((768..=1024).contains(&n), "{fb}");
        assert_eq!(pl.get("oracle"), Some("mitm-prime-blocked:m=2"));
        assert_eq!(pl.get("linalg"), Some("incremental-gauss"));
        assert_eq!(pl.get("targets"), Some("walk"));
        let x = pl.rho.ic_over_rho.unwrap();
        assert!((x - 32.955).abs() < 0.01 && pl.rho.support == Support::Measured);
    }

    #[test]
    fn prime_20_bit_base_sits_inside_the_measured_bracket() {
        // r = 562427: the holdout's best of 40, 48, 64 abscissae was 48, and
        // 40 was not separable from it (B48/B40 interval contains 1).
        let mut pr = prime(562_427f64.log2(), 562_441);
        pr.group_order = 562_427.0;
        let pl = plan(&pr, &PlanOptions::default());
        let n: u64 = pl
            .get("factor_base")
            .unwrap()
            .trim_start_matches("prime-abscissa:size=")
            .parse()
            .unwrap();
        assert!((40..=48).contains(&n), "{n}");
    }

    #[test]
    fn a_log_database_takes_the_sparse_full_rank_matrix() {
        let opts = PlanOptions {
            goal: Goal::LogDatabase,
            ..PlanOptions::default()
        };
        let pl = plan(&prime(28.0, 268_435_399), &opts);
        assert_eq!(pl.get("linalg"), Some("sparse-cado"));
        assert!(
            pl.rho.ic_over_rho.is_none(),
            "one-target gaps do not price a log database"
        );
    }

    #[test]
    fn wide_primes_fall_back_to_the_generic_table() {
        let pl = plan(&prime(40.0, (1u64 << 62) + 135), &PlanOptions::default());
        assert_eq!(pl.get("oracle"), Some("mitm:m=2,negation_folded=1"));
        assert_eq!(pl.choices[0].support, Support::Extrapolated);
        assert_eq!(pl.rho.support, Support::Extrapolated);
    }

    #[test]
    fn memory_budget_caps_the_base() {
        let opts = PlanOptions {
            max_table_bytes: 24.0 * 500.0 * 500.0,
            ..PlanOptions::default()
        };
        let pl = plan(&prime(32.0, 4_294_967_291), &opts);
        assert_eq!(pl.get("factor_base"), Some("prime-abscissa:size=500"));
        assert!(!pl.warnings.is_empty());
    }

    #[test]
    fn koblitz_columns_reproduce_the_panel_choices() {
        // (n, r, chosen columns) from the Frobenius-orbit protocol.
        let mut exact = 0;
        for (n, r, chosen) in [
            (13u32, 2_003f64, 1u64),
            (17, 65_587.0, 1),
            (19, 130_873.0, 2),
            (23, 2_095_853.0, 3),
            (29, 42_457.0, 1),
            (31, 1_439_393.0, 2),
            (37, 230_603_167.0, 8),
            (41, 549_756_390_943.0, 90),
        ] {
            let pr = CurveProfile {
                class: CurveClass::Koblitz,
                p: None,
                n: Some(n),
                group_order: r * 4.0,
                r,
                cofactor: 4,
            };
            let pl = plan(&pr, &PlanOptions::default());
            let fb = pl.get("factor_base").unwrap();
            let c: f64 = fb
                .trim_start_matches("compact-orbit-scan:columns=")
                .split(',')
                .next()
                .unwrap()
                .parse()
                .unwrap();
            let q = c / chosen as f64;
            assert!(
                (0.75..=1.34).contains(&q),
                "n{n}: planned {c}, chosen {chosen}"
            );
            exact += usize::from(c as u64 == chosen);
        }
        assert_eq!(exact, 6);
    }

    #[test]
    fn classes_without_a_recipe_name_rho_only() {
        let pl = plan(&CurveProfile::other(1e12, 1e12, 1), &PlanOptions::default());
        assert!(pl.choices.is_empty() && pl.pipeline_params().is_none());
        assert!(pl.verdict.contains("rho.negation"));
    }

    #[test]
    fn interpolation_is_exact_at_points_and_monotone_between() {
        let (a, s) = interpolate_ratio(&PRIME_RHO_POINTS, 19.10);
        assert!((a - 6.6671).abs() < 1e-9 && s == Support::Measured);
        let (m, s) = interpolate_ratio(&PRIME_RHO_POINTS, 26.0);
        assert!(m > 6.6671 && m < 32.955 && s == Support::Interpolated);
    }
}
