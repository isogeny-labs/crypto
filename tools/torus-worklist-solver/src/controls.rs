//! Predeclared fixed correctness controls; no timings or large-base search.
use crate::{solve, Answer, Limits, Report, Schedule, Stop};
use num_bigint::BigUint;
use serde_json::{json, Value};
use torus_coset_circuit::{
    index::{canonical_point, IndexTable},
    Spec,
};
use torus_indexed_circuit::Augmented;
use torus_s4::{Curve, Point, Poly, System};
use torus_s4_tree::{Fp2, TorusParameters};
type Result<T> = std::result::Result<T, String>;
pub const FIXTURE: &[u8] = include_bytes!(
    "../../../research/curve25519_torus_native_20261115/target-circuit-v2/native-v2/controls.json"
);
fn system(p: &BigUint, n: usize, equations: Vec<Poly>) -> System {
    System {
        p: p.clone(),
        variables: (0..n).map(|i| format!("x{i}")).collect(),
        equations,
    }
}
fn root(report: &Report, system: &System) -> Result<Option<Vec<BigUint>>> {
    if let Answer::Root(strings) = &report.answer {
        let values = strings
            .iter()
            .map(|s| s.parse::<BigUint>().map_err(|e| e.to_string()))
            .collect::<Result<Vec<_>>>()?;
        system.check(&values)?;
        if values.iter().any(|v| v >= &system.p) {
            return Err("noncanonical root".into());
        }
        let at = system
            .equations
            .iter()
            .flat_map(|p| p.terms.keys())
            .find_map(|m| m.first().copied())
            .ok_or("missing constrained variable")?;
        let mut bad = values.clone();
        bad[at] = (&bad[at] + 1u32) % &system.p;
        if system.check(&bad).is_ok() || system.check(&values[..values.len() - 1]).is_ok() {
            return Err("root mutation/truncation accepted".into());
        }
        Ok(Some(values))
    } else {
        Ok(None)
    }
}
fn pair(system: &System, roles: &[usize], limits: &Limits) -> Result<[Report; 2]> {
    let reports = [
        solve(system, roles, Schedule::Sweep, limits.clone()),
        solve(system, roles, Schedule::Worklist, limits.clone()),
    ];
    for r in &reports {
        root(r, system)?;
        if matches!(r.answer, Answer::Stopped(Stop::Invalid(_))) {
            return Err(format!("unexpected invalid: {:?}", r.answer));
        }
    }
    let capped = reports
        .iter()
        .any(|r| matches!(r.answer, Answer::Stopped(Stop::Cap(_))));
    if !capped
        && (reports[0].answer != reports[1].answer || reports[0].decisions != reports[1].decisions)
    {
        return Err("completed schedules disagree on answer or full decisions".into());
    }
    Ok(reports)
}
fn roles(a: &Augmented, indexed: bool) -> Vec<usize> {
    a.roles
        .iter()
        .flat_map(|r| {
            r.selector_bits
                .iter()
                .copied()
                .chain(if indexed {
                    r.index_bits.clone()
                } else {
                    vec![]
                })
                .chain(r.branch_bit)
                .chain(r.raw_v)
                // Original torus coordinates require extraction when no index circuit exists.
                .chain(if indexed {
                    vec![]
                } else {
                    r.parameter.to_vec()
                })
        })
        .collect()
}
fn target_control(s: &Spec, target: Point, limits: &Limits) -> Result<Value> {
    let (c, a) = Augmented::target(s, 2, target.clone(), true)?;
    let original = pair(&c.system, &roles(&a, false), limits)?;
    let augmented = pair(&a.system, &roles(&a, true), limits)?;
    let table = IndexTable::new(s)?;
    let mut replays = vec![];
    for (indexed, reports) in [(false, &original), (true, &augmented)] {
        for r in reports {
            let values = root(r, if indexed { &a.system } else { &c.system })?;
            let row = if let Some(values) = values {
                let original_values = if indexed {
                    a.project_and_check(&values)?
                } else {
                    values
                };
                Some(
                    c.decode_row(&original_values, &table)?
                        .iter()
                        .map(|r| json!({"j":r.j,"t":r.t,"coefficient":r.coefficient}))
                        .collect::<Vec<_>>(),
                )
            } else {
                None
            };
            replays.push(json!({"indexed":indexed,"schedule":r.schedule,"row":row}));
        }
    }
    Ok(
        json!({"target":target.map(|(u,v)|vec![u.to_string(),v.to_string()]),
        "original_stats":c.stats(),"augmented_variables":a.system.variables.len(),"augmented_equations":a.system.equations.len(),
        "original_branch_roles":roles(&a,false),"indexed_branch_roles":roles(&a,true),
        "original":original,"indexed":augmented,"independent_row_replays":replays}),
    )
}
pub struct Artifacts {
    pub report: Value,
    pub actual_original: Value,
    pub actual_augmented: Value,
}
pub fn run() -> Result<Artifacts> {
    let p = BigUint::from(101u32);
    let c = |n| Poly::small(&p, n);
    let x = |i| Poly::var(&p, i);
    let limits = Limits::default();
    let mut equations = (1..64).rev().map(|i| &x(i) - &x(i - 1)).collect::<Vec<_>>();
    equations.push(&x(0) - &c(7));
    let chain = system(&p, 64, equations);
    let chain_reports = pair(&chain, &[], &limits)?;
    if chain_reports
        .iter()
        .any(|r| !matches!(r.answer, Answer::Root(_)))
        || chain_reports[1].counts.equation_revisions >= chain_reports[0].counts.equation_revisions
    {
        return Err("reverse chain root or diagnostic revision prediction failed".into());
    }
    let boolean = |v| &(&x(v) * &x(v)) - &x(v);
    let backtrack = system(
        &p,
        3,
        vec![
            boolean(0),
            boolean(1),
            &x(2) - &(&x(0) * &x(1)),
            &x(2) - &c(1),
        ],
    );
    let back_reports = pair(&backtrack, &[0, 1], &limits)?;
    if back_reports.iter().any(|r| {
        r.counts.undos == 0 || r.counts.conflicts == 0 || !matches!(r.answer, Answer::Root(_))
    }) {
        return Err("backtrack control did not explore/undo a failed branch".into());
    }
    let cases = [
        (
            "triangular",
            system(&p, 2, vec![&x(0) - &c(3), &x(1) - &(&x(0) * &x(0))]),
        ),
        (
            "inconsistent",
            system(&p, 1, vec![&x(0) - &c(1), &x(0) - &c(2)]),
        ),
        ("nonsquare", system(&p, 1, vec![&(&x(0) * &x(0)) - &c(2)])),
        ("one_of_1100", system(&p, 1100, vec![&x(1099) - &c(7)])),
    ];
    let mut polynomial = vec![];
    for (name, s) in &cases {
        let rs = pair(s, &(0..s.variables.len()).collect::<Vec<_>>(), &limits)?;
        if rs.iter().any(|r| matches!(r.answer, Answer::Stopped(_))) {
            return Err(format!("fixed polynomial control stopped: {name}"));
        }
        polynomial.push(json!({"name":name,"reports":rs}));
    }
    let mut cubic = system(&p, 1, vec![c(0)]);
    cubic.equations[0].terms.insert(vec![0, 0, 0], 1u32.into());
    cubic.equations[0].terms.insert(vec![], 100u32.into());
    let cubic_reports = pair(
        &cubic,
        &[0],
        &Limits {
            degree: 3,
            ..limits.clone()
        },
    )?;
    if cubic_reports
        .iter()
        .any(|r| !matches!(r.answer, Answer::Stopped(Stop::Unsupported(_))))
    {
        return Err("cubic not unsupported".into());
    }
    let no_roles = pair(&backtrack, &[], &limits)?;
    if no_roles
        .iter()
        .any(|r| !matches!(r.answer, Answer::Stopped(Stop::Unsupported(_))))
    {
        return Err("derived root independently branched".into());
    }
    let forced = [
        Limits {
            events: 0,
            ..limits.clone()
        },
        Limits {
            modeled_reservations: 0,
            ..limits.clone()
        },
        Limits {
            nodes: 0,
            ..limits.clone()
        },
        Limits {
            depth: 0,
            ..limits.clone()
        },
        Limits {
            variables: 0,
            ..limits.clone()
        },
        Limits {
            equations: 0,
            ..limits.clone()
        },
        Limits {
            degree: 0,
            ..limits.clone()
        },
        Limits {
            terms: 0,
            ..limits.clone()
        },
    ];
    let mut caps = vec![];
    for l in forced {
        let rs = pair(&backtrack, &[0, 1], &l)?;
        if rs
            .iter()
            .any(|r| !matches!(r.answer, Answer::Stopped(Stop::Cap(_))))
        {
            return Err("forced cap not refused".into());
        }
        caps.push(json!({"limits":l,"reports":rs}));
    }
    let mut malformed = vec![];
    for kind in 0..5 {
        let mut s = system(&p, 2, vec![&x(0) - &c(7)]);
        let role = match kind {
            0 => {
                s.p = 100u32.into();
                vec![]
            }
            1 => {
                s.equations[0].terms.insert(vec![2], 1u32.into());
                vec![]
            }
            2 => {
                s.equations[0].terms.insert(vec![0], p.clone());
                vec![]
            }
            3 => vec![0, 0],
            _ => vec![2],
        };
        let rs = [
            solve(&s, &role, Schedule::Sweep, limits.clone()),
            solve(&s, &role, Schedule::Worklist, limits.clone()),
        ];
        if rs
            .iter()
            .any(|r| !matches!(r.answer, Answer::Stopped(Stop::Invalid(_))))
        {
            return Err("malformed accepted".into());
        }
        malformed.push(json!({"case":kind,"reports":rs}));
    }
    let old = TorusParameters::actual_four()?;
    let spec = Spec {
        p: old.f.p.clone(),
        h: old.h.pow(&213156431u32.into(), &old.f),
        order_factor: 60824497,
        q: 1,
        k: 2,
        curve_a: Some(486662u32.into()),
    };
    let curve = Curve {
        f: old.f.clone(),
        a: 486662u32.into(),
    };
    let target = curve.add(
        &canonical_point(&spec, 1, 0)?,
        &canonical_point(&spec, 2, 0)?,
    )?;
    let actual = target_control(&spec, target.clone(), &limits)?;
    for arm in ["original", "indexed"] {
        if actual[arm]
            .as_array()
            .ok_or("actual reports missing")?
            .iter()
            .any(|r| {
                r["answer"].get("NoRootObservation").is_some() || r["answer"] == "NoRootObservation"
            })
        {
            return Err("known-consistent actual fixture reported contradiction".into());
        }
    }
    let (actual_circuit, actual_indexed) = Augmented::target(&spec, 2, target, true)?;
    let toy = Spec {
        p: 101u32.into(),
        h: Fp2 {
            a: 17u32.into(),
            b: 12u32.into(),
        },
        order_factor: 17,
        q: 3,
        k: 8,
        curve_a: Some(95u32.into()),
    };
    let frozen: Value = serde_json::from_slice(FIXTURE).map_err(|e| e.to_string())?;
    let targets = frozen["toy"]["image_group_points"]
        .as_array()
        .ok_or("toy targets")?;
    if targets.len() != 13 {
        return Err("public target population changed".into());
    }
    let mut toy_reports = vec![];
    for (i, target) in targets.iter().enumerate() {
        eprintln!("fixed toy public target {i}/13");
        let q = if target.is_null() {
            None
        } else {
            Some((
                target[0]
                    .as_str()
                    .ok_or("target u")?
                    .parse()
                    .map_err(|e| format!("{e}"))?,
                target[1]
                    .as_str()
                    .ok_or("target v")?
                    .parse()
                    .map_err(|e| format!("{e}"))?,
            ))
        };
        toy_reports.push(target_control(&toy, q, &limits)?);
    }
    Ok(Artifacts {
        actual_original: actual_circuit.system.canonical_json(),
        actual_augmented: actual_indexed.system.canonical_json(),
        report: json!({"schema":"torus.worklist.control/v1","standing":"unsigned/unverified","limits":limits,
        "accounting":"partial diagnostic events and cumulative modeled reservations; no timing/cold-cost/live-memory claim",
        "chain":chain_reports,"backtrack":back_reports,"polynomial":polynomial,"cubic":cubic_reports,
        "no_branch_roles":no_roles,"forced_caps":caps,"malformed":malformed,"actual_q1_k2":actual,"toy":toy_reports}),
    })
}
#[cfg(test)]
mod tests {
    #[test]
    fn fixed_correctness_controls() {
        super::run().unwrap();
    }
}
