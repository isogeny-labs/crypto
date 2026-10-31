//! `ecbench sweep`: one `ecbench.spec/v1` for a method × size scaling
//! study.
//!
//! A scaling exponent is only as good as the sizes it is fitted over, so
//! the generator writes one curve per size and the same arms on every
//! curve.  Sizes are field bits for `prime` (a `prime_search` curve of
//! cofactor 1, `8 ≤ bits ≤ 32`) and extension degrees `n` for `koblitz`
//! (`K_a / F_{2^n}`, `3 ≤ n ≤ 62`; `r` is the prime subgroup order the
//! workload reports).  The result is an ordinary spec: `ecbench plan`
//! validates it, `ecbench run` measures it, `ecbench bound fit` fits the
//! exponent of one arm from the session, `ecbench report` plots it.
//!
//! Roles: `rho.negation` (or, when it is absent, the first method) is the
//! reference; other `rho.*` variants are baselines; BSGS, kangaroo and
//! index calculus are candidates; an optional A/A control repeats the
//! reference and prices the noise floor of the session.

use std::collections::BTreeMap;

use crate::cryptanalysis::ecbench::methods::{decl, MethodSpec};
use crate::cryptanalysis::ecbench::spec::{
    ArmSpec, Level, Measurement, Order, Role, Spec, WorkloadPlan, SPEC_SCHEMA,
};
use crate::cryptanalysis::ecbench::workload::{CurveSpec, TargetKind};

/// The curve family a sweep runs over.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Family {
    Prime,
    Koblitz,
}

impl Family {
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "prime" => Ok(Family::Prime),
            "koblitz" => Ok(Family::Koblitz),
            _ => Err(format!("--family is `prime` or `koblitz`, not `{s}`")),
        }
    }
    fn name(self) -> &'static str {
        match self {
            Family::Prime => "prime",
            Family::Koblitz => "koblitz",
        }
    }
}

/// Everything a sweep spec is made of.
#[derive(Clone, Debug)]
pub struct SweepOptions {
    pub family: Family,
    /// Field bits (prime) or extension degrees (koblitz), in order.
    pub sizes: Vec<u32>,
    /// Method ids; each takes its defaults.
    pub methods: Vec<String>,
    /// `ic.pipeline` parameters as `factor_base=…;oracle=…`, adding one
    /// `ic.pipeline` arm.
    pub ic: Option<String>,
    pub targets: u64,
    pub rounds: u32,
    pub timeout_seconds: u64,
    /// Repeat the reference as an A/A control.
    pub control: bool,
    /// `K_a` for a Koblitz sweep.
    pub koblitz_a: u8,
    /// The `prime_search` seed (one per size, derived from this and the
    /// size, so the curves differ).
    pub curve_seed: u64,
    pub target_seed: u64,
    pub isolation_required: Level,
    pub label: Option<String>,
}

impl Default for SweepOptions {
    fn default() -> Self {
        SweepOptions {
            family: Family::Prime,
            sizes: Vec::new(),
            methods: Vec::new(),
            ic: None,
            targets: 2,
            rounds: 3,
            timeout_seconds: 300,
            control: true,
            koblitz_a: 1,
            curve_seed: 59297,
            target_seed: 20261002,
            isolation_required: Level::L2,
            label: None,
        }
    }
}

/// `16,18,20` or `16..30:2` (inclusive range with a step) as a size list.
pub fn parse_sizes(text: &str) -> Result<Vec<u32>, String> {
    let mut out = Vec::new();
    for part in text.split(',').map(str::trim).filter(|p| !p.is_empty()) {
        if let Some((range, step)) = part.split_once(':') {
            let (lo, hi) = range
                .split_once("..")
                .ok_or_else(|| format!("size range `{part}` is lo..hi:step"))?;
            let (lo, hi, step): (u32, u32, u32) = (
                lo.parse().map_err(|_| format!("size `{lo}`"))?,
                hi.parse().map_err(|_| format!("size `{hi}`"))?,
                step.parse().map_err(|_| format!("step `{step}`"))?,
            );
            if step == 0 || lo > hi {
                return Err(format!("size range `{part}` is empty"));
            }
            let mut v = lo;
            while v <= hi {
                out.push(v);
                v += step;
            }
        } else {
            out.push(part.parse().map_err(|_| format!("size `{part}`"))?);
        }
    }
    if out.is_empty() {
        return Err("no sizes given".into());
    }
    let mut sorted = out.clone();
    sorted.sort_unstable();
    sorted.dedup();
    if sorted.len() != out.len() {
        return Err("a size is listed twice".into());
    }
    Ok(out)
}

/// `factor_base=prime-abscissa:size=32;oracle=mitm:negation_folded=1` as
/// parameters.  A value may itself hold `;` (`koblitz-orbit:divisor=0;1`):
/// a segment starts a new parameter only when its key is one `ic.pipeline`
/// declares, otherwise it continues the previous value.
pub fn parse_ic_params(text: &str, method: &str) -> Result<BTreeMap<String, String>, String> {
    let d = decl(method).ok_or_else(|| format!("unknown method `{method}`"))?;
    let known = |k: &str| d.params.iter().any(|p| p.name == k);
    let mut out: BTreeMap<String, String> = BTreeMap::new();
    let mut last: Option<String> = None;
    for seg in text.split(';') {
        let key = seg.split_once('=').map(|(k, _)| k.trim());
        match key {
            Some(k) if known(k) => {
                let v = seg.split_once('=').map(|(_, v)| v).unwrap_or("");
                out.insert(k.to_string(), v.trim().to_string());
                last = Some(k.to_string());
            }
            _ => match &last {
                Some(k) => {
                    let v = out.get_mut(k).expect("last key is present");
                    v.push(';');
                    v.push_str(seg);
                }
                None => {
                    return Err(format!(
                        "`{seg}` is not a parameter of {method} (it takes: {})",
                        d.params
                            .iter()
                            .map(|p| p.name)
                            .collect::<Vec<_>>()
                            .join(", ")
                    ))
                }
            },
        }
    }
    Ok(out)
}

fn arm_name(id: &str) -> String {
    id.replace('.', "-")
}

fn role_of(id: &str, reference: &str) -> Role {
    if id == reference {
        Role::Reference
    } else if id.starts_with("rho.") {
        Role::Baseline
    } else {
        Role::Candidate
    }
}

/// The spec a sweep describes.
pub fn generate(o: &SweepOptions) -> Result<Spec, String> {
    if o.sizes.is_empty() {
        return Err("no sizes".into());
    }
    if o.methods.is_empty() && o.ic.is_none() {
        return Err("no methods".into());
    }
    if o.targets == 0 || o.rounds == 0 {
        return Err("--targets and --rounds are at least 1".into());
    }
    let mut curves = Vec::new();
    for &s in &o.sizes {
        curves.push(match o.family {
            Family::Prime => {
                if !(8..=32).contains(&s) {
                    return Err(format!("prime_search takes 8 ≤ bits ≤ 32, not {s}"));
                }
                CurveSpec::PrimeSearch {
                    bits: s,
                    seed: o.curve_seed,
                }
            }
            Family::Koblitz => {
                if o.koblitz_a > 1 || !(3..=62).contains(&s) {
                    return Err(format!(
                        "koblitz takes a ∈ {{0,1}} and 3 ≤ n ≤ 62 here, not a={} n={s}",
                        o.koblitz_a
                    ));
                }
                CurveSpec::Koblitz {
                    a: o.koblitz_a,
                    n: s,
                }
            }
        });
    }
    let mut ids: Vec<(String, BTreeMap<String, String>)> = Vec::new();
    for m in &o.methods {
        if decl(m).is_none() {
            return Err(format!("unknown method `{m}` (see `ecbench methods`)"));
        }
        if ids.iter().any(|(i, _)| i == m) {
            return Err(format!("method `{m}` is listed twice"));
        }
        ids.push((m.clone(), BTreeMap::new()));
    }
    if let Some(ic) = &o.ic {
        ids.push(("ic.pipeline".into(), parse_ic_params(ic, "ic.pipeline")?));
    }
    let reference = if ids.iter().any(|(i, _)| i == "rho.negation") {
        "rho.negation".to_string()
    } else {
        ids[0].0.clone()
    };
    let mut arms: Vec<ArmSpec> = ids
        .iter()
        .map(|(id, params)| ArmSpec {
            name: arm_name(id),
            role: role_of(id, &reference),
            method: MethodSpec {
                id: id.clone(),
                params: params.clone(),
            },
        })
        .collect();
    if o.control {
        arms.push(ArmSpec {
            name: format!("{}-aa", arm_name(&reference)),
            role: Role::Control,
            method: MethodSpec {
                id: reference.clone(),
                params: BTreeMap::new(),
            },
        });
    }
    let unit = match o.family {
        Family::Prime => "bits of p",
        Family::Koblitz => "degrees n of K_a/F_2^n",
    };
    let sizes_text = o
        .sizes
        .iter()
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join(",");
    let spec = Spec {
        schema: SPEC_SCHEMA.into(),
        label: o.label.clone().unwrap_or_else(|| {
            format!(
                "sweep: {} method(s) on {} curves, {} {}",
                arms.len(),
                o.family.name(),
                unit,
                sizes_text
            )
        }),
        description: format!(
            "Method × size scaling study ({} curves, {unit} {sizes_text}; {} independent single-target workload(s) per curve, {} interleaved round(s)). One curve per size so `ecbench bound fit` and `ecbench report` can fit cost against r. Counted group operations; a measurement, not a performance claim.",
            o.family.name(),
            o.targets,
            o.rounds
        ),
        workloads: WorkloadPlan {
            curves,
            targets_per_curve: o.targets,
            target_seed: o.target_seed,
            target_kind: TargetKind::default(),
        },
        arms,
        measurement: Measurement {
            rounds: o.rounds,
            warmup: 1,
            order: Order::Alternate,
            seed: 1,
            isolation_required: o.isolation_required,
            timeout_seconds: o.timeout_seconds,
        },
    };
    // Round trip through the loader so a spec this module writes is one
    // `ecbench plan` reads.
    let text = serde_json::to_string(&spec).map_err(|e| e.to_string())?;
    Spec::from_json(&text)?;
    Ok(spec)
}

/// The spec as pretty JSON (struct field order: schema, label,
/// description, workloads, arms, measurement).
pub fn to_json(spec: &Spec) -> Result<String, String> {
    let mut s = serde_json::to_string_pretty(spec).map_err(|e| e.to_string())?;
    s.push('\n');
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn opts() -> SweepOptions {
        SweepOptions {
            sizes: vec![16, 18, 20],
            methods: vec![
                "rho.plain".into(),
                "rho.negation".into(),
                "bsgs.negation".into(),
                "kangaroo.vow".into(),
            ],
            ic: Some("factor_base=prime-abscissa:size=32;oracle=mitm:negation_folded=1".into()),
            ..SweepOptions::default()
        }
    }

    #[test]
    fn sizes_parse_lists_and_ranges() {
        assert_eq!(parse_sizes("16,18,20").unwrap(), vec![16, 18, 20]);
        assert_eq!(parse_sizes("16..30:2").unwrap().len(), 8);
        assert!(parse_sizes("16,16").is_err());
        assert!(parse_sizes("").is_err());
        assert!(parse_sizes("30..16:2").is_err());
    }

    #[test]
    fn ic_params_keep_semicolons_inside_a_value() {
        let p = parse_ic_params(
            "factor_base=koblitz-orbit:divisor=0;1;oracle=mitm-frobenius:m=2",
            "ic.pipeline",
        )
        .unwrap();
        assert_eq!(p["factor_base"], "koblitz-orbit:divisor=0;1");
        assert_eq!(p["oracle"], "mitm-frobenius:m=2");
        assert!(parse_ic_params("nonsense", "ic.pipeline").is_err());
    }

    #[test]
    fn prime_sweep_has_one_curve_per_size_and_sensible_roles() {
        let s = generate(&opts()).unwrap();
        assert_eq!(s.workloads.curves.len(), 3);
        let role = |n: &str| s.arms.iter().find(|a| a.name == n).unwrap().role;
        assert_eq!(role("rho-negation"), Role::Reference);
        assert_eq!(role("rho-plain"), Role::Baseline);
        assert_eq!(role("bsgs-negation"), Role::Candidate);
        assert_eq!(role("ic-pipeline"), Role::Candidate);
        assert_eq!(role("rho-negation-aa"), Role::Control);
        let ic = s.arms.iter().find(|a| a.name == "ic-pipeline").unwrap();
        assert_eq!(ic.method.params["oracle"], "mitm:negation_folded=1");
        // The JSON is a spec the loader reads back unchanged.
        let again = Spec::from_json(&to_json(&s).unwrap()).unwrap();
        assert_eq!(again, s);
    }

    #[test]
    fn koblitz_sweep_uses_degrees_and_refuses_bad_input() {
        let o = SweepOptions {
            family: Family::Koblitz,
            sizes: vec![13, 17],
            methods: vec!["rho.signed_frobenius".into(), "rho.negation".into()],
            ..SweepOptions::default()
        };
        let s = generate(&o).unwrap();
        assert_eq!(s.workloads.curves[0], CurveSpec::Koblitz { a: 1, n: 13 });
        let reference = s.arms.iter().find(|a| a.role == Role::Reference).unwrap();
        assert_eq!(reference.method.id, "rho.negation");
        let bad = SweepOptions {
            methods: vec!["no.such".into()],
            ..o.clone()
        };
        assert!(generate(&bad).is_err());
        let big = SweepOptions {
            family: Family::Prime,
            sizes: vec![40],
            ..opts()
        };
        assert!(generate(&big).is_err());
    }
}
