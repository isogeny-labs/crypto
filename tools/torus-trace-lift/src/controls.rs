use crate::{Result, TraceMap};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use torus_coset_circuit::{Circuit, Spec, WitnessInput};
use torus_s4::Field;
use torus_s4_tree::{Fp2, TorusParameters};

pub struct Artifacts {
    pub report: Value,
    pub trace: Vec<u8>,
    pub identities: Vec<torus_s4_tree::identity::IdentityCertificate>,
}
fn check_core(f: &Field, q: u64, z: &Fp2, omega: &Fp2) -> bool {
    z.norm(f) == BigUint::one() && z.pow(&q.into(), f) == *omega
}
fn exhaustive(
    p: u64,
    h: Fp2,
    q: u64,
    labels: u64,
    replay: bool,
    trace: &mut Vec<u8>,
) -> Result<Value> {
    let f = Field::new(p.into())?;
    let mut roots = vec![];
    let mut comparisons = 0usize;
    let start = trace.len();
    let circuit = if replay {
        Some(Circuit::compile(&Spec {
            p: p.into(),
            h: h.clone(),
            order_factor: 17,
            q,
            k: labels,
            curve_a: None,
        })?)
    } else {
        None
    };
    for j in 1..=labels {
        let omega = h.pow(&(q * j).into(), &f);
        let map = TraceMap::new(&f, q, omega.clone())?;
        for x in 0..p {
            let (t, u) = map.evaluate(&x.into())?;
            if t != f.chebyshev(q as u128, &x.into()) {
                return Err("T recurrence mismatch".into());
            }
            let pell = f.sub(
                &f.square(&t),
                &f.mul(&f.sub(&f.square(&x.into()), &1u32.into()), &f.square(&u)),
            );
            if pell != BigUint::one() {
                return Err("Pell evaluation mismatch".into());
            }
            let lifted = map.lift(&x.into())?;
            if let Some(z) = &lifted {
                if !check_core(&f, q, z, &omega) {
                    return Err("lift fails original core".into());
                }
                if let Some(circuit) = &circuit {
                    let input = WitnessInput {
                        z: z.clone(),
                        selector: j - 1,
                        slack: labels - j,
                        branch: 0u32.into(),
                        raw_v: 0u32.into(),
                    };
                    circuit.system.check(&circuit.witness(&input)?)?;
                }
                roots.push(
                    json!({"j":j,"x":x,"b":z.b.to_string(),"T":t.to_string(),"U":u.to_string()}),
                );
            }
            for b in 0..p {
                let z = Fp2 {
                    a: x.into(),
                    b: b.into(),
                };
                let original = check_core(&f, q, &z, &omega);
                let projected = lifted.as_ref() == Some(&z);
                if original != projected {
                    return Err("original/projected finite mismatch".into());
                }
                trace.push(u8::from(original) | u8::from(projected) << 1);
                comparisons += 1;
            }
        }
    }
    Ok(
        json!({"p":p,"h":[h.a.to_string(),h.b.to_string()],"q":q,"labels":labels,"comparisons":comparisons,
        "trace_offset":start,"trace_length":trace.len()-start,"trace_order":"j ascending, x ascending, b ascending; bits0 original/1 lifted", "original_circuit_replay":replay,"roots":roots}),
    )
}
pub fn run() -> Result<Artifacts> {
    let h = Fp2 {
        a: 17u32.into(),
        b: 12u32.into(),
    };
    let mut trace = vec![];
    let mut finite = vec![
        exhaustive(101, h.clone(), 3, 8, true, &mut trace)?,
        exhaustive(
            59,
            Fp2 {
                a: 54u32.into(),
                b: 37u32.into(),
            },
            3,
            2,
            false,
            &mut trace,
        )?,
    ];
    for q in [1, 5, 17] {
        finite.push(exhaustive(101, h.clone(), q, 2, false, &mut trace)?)
    }
    if finite[0]["roots"].as_array().map(Vec::len) != Some(24) {
        return Err("p101 original member count changed".into());
    }
    let params = TorusParameters::actual_four()?;
    let f = &params.f;
    let q = 213156431u64;
    let mut actual = vec![];
    for j in [1u64, 10316, 20633] {
        for t in [0u64, 1, q - 1] {
            for conjugate in [false, true] {
                let mut z = params.h.pow(&(j + 60824497 * t).into(), f);
                if conjugate {
                    z.b = f.sub(&0u32.into(), &z.b)
                }
                let mut omega = params.h.pow(&(q * j).into(), f);
                if conjugate {
                    omega.b = f.sub(&0u32.into(), &omega.b);
                }
                let map = TraceMap::new(f, q, omega.clone())?;
                let lifted = map.lift(&z.a)?.ok_or("actual input not a scalar root")?;
                if lifted != z || !check_core(f, q, &lifted, &omega) {
                    return Err("actual original lift mismatch".into());
                }
                let mut wrong = lifted.clone();
                wrong.b = f.add(&wrong.b, &1u32.into());
                if check_core(f, q, &wrong, &omega) {
                    return Err("imaginary mutation accepted".into());
                }
                actual.push(json!({"j":j,"t":t,"conjugate":conjugate,"z":[z.a.to_string(),z.b.to_string()],"omega":[omega.a.to_string(),omega.b.to_string()]}));
            }
        }
    }
    let f = Field::new(101u32.into())?;
    let omega = h.pow(&3u32.into(), &f);
    let bad = [
        TraceMap::new(&f, 0, omega.clone()).is_err(),
        TraceMap::new(&f, 101, omega.clone()).is_err(),
        TraceMap::new(&f, 3, Fp2::one()).is_err(),
        TraceMap::new(
            &f,
            3,
            Fp2 {
                a: 0u32.into(),
                b: 0u32.into(),
            },
        )
        .is_err(),
        TraceMap::new(
            &f,
            3,
            Fp2 {
                a: 101u32.into(),
                b: 1u32.into(),
            },
        )
        .is_err(),
        TraceMap::new(&f, 3, omega)?.lift(&101u32.into()).is_err(),
    ];
    if !bad.into_iter().all(|x| x) {
        return Err("malformed/chart refusal failed".into());
    }
    let t = f.chebyshev(3, &50u32.into());
    let u = f.sub(&f.scale(&f.square(&50u32.into()), 4), &1u32.into());
    if t != BigUint::one() || !u.is_zero() {
        return Err("fixed ramified counterexample changed".into());
    }
    let ids = crate::identities::certificates()?;
    Ok(Artifacts {
        report: json!({"schema":"torus.trace-lift.control/v1","standing":"producer unsigned/unverified",
        "purpose":"fixed exact-coordinate correspondence, not relation root discovery or performance",
        "finite":finite,"total_pair_comparisons":trace.len(),"actual_signed_lifts":actual,
        "refusals":["q0","q divisible by characteristic","d0","nonunit omega","noncanonical omega","noncanonical x"],
        "ramified_control":{"p":101,"q":3,"x":50,"T":t.to_string(),"U":u.to_string(),"omega":[1,0],"status":"explicitly excluded"},
        "identity_ids":ids.iter().map(|c|c.id.clone()).collect::<Vec<_>>(),
        "cost_scope":"no timing/normalized field work/RAM measurement; no solver or exponent claim"}),
        trace,
        identities: ids,
    })
}
