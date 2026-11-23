//! Exact prime31 y-projection fiber and signed-two-sum support gate.
//! All point additions here are explicitly generic **control enumeration**:
//! this module is NOT an algebraic relation oracle or an ECDLP solver.
//! cargo test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_y_lift_support_gate
//! cargo run  --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_y_lift_support_gate
use prime31_algebraic_ic::{check_curve, Arithmetic, Base, Curve, PRIME31};
use serde_json::json;
use std::collections::BTreeSet;

type Affine = (u64, u64);

fn mul(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
fn sub(a: u64, b: u64, p: u64) -> u64 {
    (a + p - b) % p
}
fn signed(points: &[Affine], e: Curve) -> Vec<Affine> {
    let mut out = BTreeSet::new();
    for &(x, y) in points {
        out.insert((x, y));
        out.insert((x, sub(0, y, e.p)));
    }
    out.into_iter().collect()
}
fn support(points: &[Affine], ar: &mut Arithmetic, e: Curve) -> (BTreeSet<Affine>, usize) {
    let mut rows = BTreeSet::new();
    let mut identities = 0;
    for (i, &a) in points.iter().enumerate() {
        for &b in &points[i..] {
            match ar.add(e, Some(a), Some(b)) {
                Some(p) => {
                    rows.insert(p);
                }
                None => identities += 1,
            }
        }
    }
    (rows, identities)
}
fn run() -> Result<serde_json::Value, String> {
    let e = PRIME31;
    let mut ar = Arithmetic::default();
    check_curve(e, &mut ar)?;
    let base = Base::build(e, 6, &mut ar)?;
    if base.points.len() != 31 {
        return Err("canonical prime31 B31 base changed".into());
    }
    let mut fiber = BTreeSet::new();
    let mut per_y = Vec::new();
    let mut unique = Vec::new();
    let mut one = 0;
    let mut three = 0;
    let inv2 = (e.p + 1) / 2;
    for &(x, y) in &base.points {
        // X^3+aX+b-y^2 = (X-x)(X^2+xX+x^2+a).
        // The remaining rational X-roots come from Delta=-3x^2-4a.
        let disc = sub(0, (3 * mul(x, x, e.p) + 4 * e.a) % e.p, e.p);
        let mut xs = BTreeSet::from([x]);
        if let Some(d) = ar.sqrt(disc, e.p) {
            xs.insert(mul(sub(d, x, e.p), inv2, e.p));
            xs.insert(mul(sub(sub(0, d, e.p), x, e.p), inv2, e.p));
        }
        if xs.len() == 1 {
            one += 1;
            unique.push((x, y));
        } else if xs.len() == 3 {
            three += 1;
        } else {
            return Err("exceptional repeated-root y fiber".into());
        }
        per_y.push(xs.iter().copied().map(|root| (root, y)).collect::<Vec<_>>());
        for root in xs {
            let q = Some((root, y));
            if !ar.on_curve(e, q) {
                return Err("invalid cubic lift".into());
            }
            fiber.insert(q.unwrap());
        }
    }
    if (one, three, fiber.len()) != (14, 17, 65) {
        return Err("prime31 y-fiber census changed".into());
    }
    // For distinct base y-values, independently count oriented y(sum) roots.
    // The 1/3/9 distribution is a negative-control cost measure, NOT an oracle.
    let mut pair_projection = [0usize; 10];
    for i in 0..per_y.len() {
        for j in i + 1..per_y.len() {
            let mut outputs = BTreeSet::new();
            for &a in &per_y[i] {
                for &b in &per_y[j] {
                    let Some((_, yr)) = ar.add(e, Some(a), Some(b)) else {
                        return Err("unexpected y-pair cancellation".into());
                    };
                    outputs.insert(yr);
                }
            }
            let expected = per_y[i].len() * per_y[j].len();
            if outputs.len() != expected {
                return Err("unexpected distinct-y projection collision".into());
            }
            pair_projection[expected] += 1;
        }
    }
    if (pair_projection[1], pair_projection[3], pair_projection[9]) != (91, 238, 136) {
        return Err("prime31 y-sum root multiplicities changed".into());
    }
    // A larger Dickson fibre contains enough injective y-fibres to
    // recover B31 without triple lifts; this changes the factor-base encoding.
    let base7 = Base::build(e, 7, &mut ar)?;
    let mut injective7 = Vec::new();
    for &(x, y) in &base7.points {
        let disc = sub(0, (3 * mul(x, x, e.p) + 4 * e.a) % e.p, e.p);
        if ar.sqrt(disc, e.p).is_none() {
            injective7.push((x, y));
        }
    }
    if base7.points.len() != 64 || injective7.len() != 39 {
        return Err("prime31 depth-seven injective y count changed".into());
    }
    let (injective7_signed_31, injective7_identities) =
        support(&signed(&injective7[..31], e), &mut ar, e);
    if injective7_signed_31.len() != 1922 || injective7_identities != 31 {
        return Err("depth-seven B31 unique-y support changed".into());
    }
    let canonical = base.points;
    let extended: Vec<_> = fiber.into_iter().collect();
    let (or_original, oi) = support(&canonical, &mut ar, e);
    let (or_extended, ei) = support(&extended, &mut ar, e);
    let (signed_original, si) = support(&signed(&canonical, e), &mut ar, e);
    let (signed_extended, se) = support(&signed(&extended, e), &mut ar, e);
    let (signed_unique, ui) = support(&signed(&unique, e), &mut ar, e);
    if (or_original.len(), or_extended.len(), signed_original.len(), signed_extended.len(), signed_unique.len())
        != (496, 2145, 1922, 8450, 392)
        || (oi, ei, si, se, ui) != (0, 0, 31, 65, 14)
        || !signed_original.is_subset(&signed_extended)
    {
        return Err("exact signed-support control mismatch".into());
    }
    Ok(json!({
        "standing": "[pcc:unverified]",
        "method": "generic group-support enumeration: NEGATIVE CONTROL, not algebraic IC",
        "curve_p": e.p,
        "curve_order_n": e.n,
        "depth": base.depth,
        "depth7_full_base": base7.points.len(),
        "depth7_injective_y_base": injective7.len(),
        "depth7_first31_injective_y_signed_pair_support": injective7_signed_31.len(),
        "canonical_base": canonical.len(),
        "base_y_unique_root": one,
        "base_y_three_roots": three,
        "y_only_rational_lifted_points": extended.len(),
        "distinct_y_pair_output_root_histogram": {"1":pair_projection[1],"3":pair_projection[3],"9":pair_projection[9]},
        "oriented_pair_support_canonical": or_original.len(),
        "oriented_pair_support_y_lifted": or_extended.len(),
        "signed_pair_support_canonical": signed_original.len(),
        "signed_pair_support_y_lifted": signed_extended.len(),
        "signed_pair_support_injective_y_subset": signed_unique.len(),
        "y_lift_spurious_signed_targets_against_original_factor_base": signed_extended.len()-signed_original.len(),
        "identity_pair_counts": [oi, ei, si, se, ui],
        "group_additions": ar.counts.group_additions
    }))
}
fn main() {
    match run() {
        Ok(v) => println!("{}", serde_json::to_string_pretty(&v).unwrap()),
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn support_fixture() {
        super::run().unwrap();
    }
}
