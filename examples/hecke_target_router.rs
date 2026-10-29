//! Public-target routing against externally certified known-log points.
//! Generic shift control, not a new Hecke relation generator or a speed claim.
use crypto_lib::cryptanalysis::ic_boundary::{
    CountedGroup, FieldOps, GroupOps, PrimeCurve, PrimePoint,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Deserialize)]
struct Label {
    x: u64,
    y: u64,
    log: u64,
}
#[derive(Deserialize)]
struct Input {
    p: u64,
    a: u64,
    b: u64,
    n: u64,
    px: u64,
    py: u64,
    qx: u64,
    qy: u64,
    #[serde(default)]
    q_infinity: bool,
    labels: Vec<Label>,
    shift_limit: u64,
}
#[derive(Debug, Serialize)]
struct Output {
    recovered: Option<u64>,
    probes: u64,
    label_checks: usize,
    setup: GroupOps,
    online: GroupOps,
    setup_field: FieldOps,
    online_field: FieldOps,
    status: &'static str,
}

fn recover(input: &Input) -> Result<Output, String> {
    use crypto_lib::cryptanalysis::residual_walk::is_prime_u64;
    if input.p >= 1 << 32
        || input.n >= 1 << 32
        || input.a >= input.p
        || input.b >= input.p
        || !is_prime_u64(input.p)
        || !is_prime_u64(input.n)
        || input.p <= 3
    {
        return Err("invalid bounded prime curve/subgroup parameters".into());
    }
    let p = input.p as u128;
    if (4 * (input.a as u128).pow(3) + 27 * (input.b as u128).pow(2)).is_multiple_of(p) {
        return Err("singular curve".into());
    }
    let curve = PrimeCurve {
        p: input.p,
        a: input.a,
        b: input.b,
    };
    let generator = PrimePoint::affine(input.px, input.py);
    let target = if input.q_infinity {
        PrimePoint::INFINITY
    } else {
        PrimePoint::affine(input.qx, input.qy)
    };
    let canonical = |q: PrimePoint| q.x < input.p && q.y < input.p && curve.is_on_curve(q);
    if !canonical(generator) || !canonical(target) {
        return Err("invalid public point".into());
    }
    let mut setup = GroupOps::default();
    if curve.mul(&mut setup, generator, input.n) != PrimePoint::INFINITY
        || curve.mul(&mut setup, target, input.n) != PrimePoint::INFINITY
    {
        return Err("public point fails the n-torsion check".into());
    }
    let mut table = BTreeMap::new();
    for row in &input.labels {
        let point = PrimePoint::affine(row.x, row.y);
        if row.log >= input.n
            || !canonical(point)
            || curve.mul(&mut setup, generator, row.log) != point
        {
            return Err("known-log certificate failed".into());
        }
        if table.insert((row.x, row.y), row.log).is_some() {
            return Err("duplicate labelled point".into());
        }
    }
    let mut online = GroupOps::default();
    let mut q = target;
    let mut answer = None;
    let mut probes = 0;
    for shift in 0..input.shift_limit.min(input.n) {
        probes += 1;
        let d = if q.infinity {
            Some(0)
        } else {
            table.get(&(q.x, q.y)).copied()
        };
        if let Some(d) = d {
            let candidate = (d + input.n - shift) % input.n;
            if curve.mul(&mut online, generator, candidate) != target {
                return Err("final scalar verification failed".into());
            }
            answer = Some(candidate);
            break;
        }
        if shift + 1 < input.shift_limit.min(input.n) {
            q = curve.add(&mut online, q, generator);
        }
    }
    Ok(Output {
        recovered: answer,
        probes,
        label_checks: input.labels.len(),
        setup_field: setup.field_ops,
        online_field: online.field_ops,
        setup,
        online,
        status: if answer.is_some() {
            "verified"
        } else {
            "bounded_miss"
        },
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args()
        .nth(1)
        .ok_or("usage: hecke_target_router INPUT.json")?;
    let gate = std::process::Command::new("bash")
        .args(["scripts/checkpoint.sh", "gate"])
        .status()?;
    if !gate.success() {
        return Err("checkpoint gate refused experiment".into());
    }
    let input: Input = serde_json::from_slice(&std::fs::read(path)?)?;
    println!("{}", serde_json::to_string(&recover(&input)?)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> Input {
        Input {
            p: 17,
            a: 2,
            b: 2,
            n: 19,
            px: 5,
            py: 1,
            qx: 6,
            qy: 3,
            q_infinity: false,
            labels: vec![Label { x: 5, y: 1, log: 1 }],
            shift_limit: 19,
        }
    }
    #[test]
    fn arbitrary_public_target_is_verified() {
        let out = recover(&fixture()).unwrap();
        assert_eq!(out.recovered, Some(2));
        assert!(out.online.adds > 0);
        assert!(out.online_field.muls > 0);
    }
    #[test]
    fn cap_preserves_miss_and_bad_labels_fail_closed() {
        let mut input = fixture();
        input.shift_limit = 1;
        assert_eq!(recover(&input).unwrap().status, "bounded_miss");
        input.labels[0].log = 2;
        assert!(recover(&input).is_err());
    }
    #[test]
    fn duplicate_and_wrong_subgroup_are_refused() {
        let mut input = fixture();
        input.labels.push(Label { x: 5, y: 1, log: 1 });
        assert!(recover(&input).is_err());
        let mut input = fixture();
        input.n = 17;
        assert!(recover(&input).is_err());
    }
    #[test]
    fn identity_target_is_verified_as_zero() {
        let mut input = fixture();
        input.q_infinity = true;
        assert_eq!(recover(&input).unwrap().recovered, Some(0));
    }
}
