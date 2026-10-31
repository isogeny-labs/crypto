//! Input exporter and exact group reference for a bounded prime-field pilot.
//! No degree claim or new FB1/ICV1 identity is assigned by this development exporter.
use super::*;
use std::collections::BTreeSet;
type Point = Option<(u64, u64)>;
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Spec {
    p: u64,
    a: u64,
    b: u64,
    exponent: u64,
    shift: u64,
    coset_a: u64,
    coset_b: u64,
    max_degree: u32,
    seed: u64,
}
fn rhs(s: &Spec, x: u64) -> u64 {
    add(
        add(mul(mul(x, x, s.p), x, s.p), mul(s.a, x, s.p), s.p),
        s.b,
        s.p,
    )
}
fn curve_points(s: &Spec) -> Vec<(u64, u64)> {
    (0..s.p)
        .flat_map(|x| {
            (0..s.p)
                .filter(move |&y| mul(y, y, s.p) == rhs(s, x))
                .map(move |y| (x, y))
        })
        .collect()
}
fn point_add(s: &Spec, p: Point, q: Point) -> Point {
    let (Some((x, y)), Some((u, v))) = (p, q) else {
        return p.or(q);
    };
    if x == u && add(y, v, s.p) == 0 {
        return None;
    }
    let (numerator, denominator) = if x == u {
        (add(mul(3, mul(x, x, s.p), s.p), s.a, s.p), mul(2, y, s.p))
    } else {
        (add(v, neg(y, s.p), s.p), add(u, neg(x, s.p), s.p))
    };
    let slope = mul(numerator, pow(denominator, s.p - 2, s.p), s.p);
    let nx = add(
        add(mul(slope, slope, s.p), neg(x, s.p), s.p),
        neg(u, s.p),
        s.p,
    );
    Some((
        nx,
        add(mul(slope, add(x, neg(nx, s.p), s.p), s.p), neg(y, s.p), s.p),
    ))
}
fn base(s: &Spec) -> Vec<u64> {
    (0..s.p)
        .filter(|&x| {
            pow(x, s.exponent, s.p) == s.coset_a
                && pow(add(x, s.shift, s.p), s.exponent, s.p) == s.coset_b
                && (0..s.p).any(|y| mul(y, y, s.p) == rhs(s, x))
        })
        .collect()
}
fn membership(values: &[u64], p: u64, slot: usize) -> Vec<Term> {
    let mut coefficients = vec![1];
    for &root in values {
        let mut next = vec![0; coefficients.len() + 1];
        for (i, &c) in coefficients.iter().enumerate() {
            next[i] = add(next[i], mul(neg(root, p), c, p), p);
            next[i + 1] = add(next[i + 1], c, p);
        }
        coefficients = next;
    }
    coefficients
        .into_iter()
        .enumerate()
        .filter(|(_, c)| *c != 0)
        .map(|(i, c)| Term {
            x: if slot == 0 { i as u32 } else { 0 },
            y: if slot == 1 { i as u32 } else { 0 },
            c,
        })
        .collect()
}
fn s3(s: &Spec, r: u64) -> Vec<Term> {
    let mut f = Poly::new();
    let p = s.p;
    accumulate(&mut f, (2, 2), 1, p);
    accumulate(&mut f, (1, 1), neg(mul(2, s.a, p), p), p);
    accumulate(&mut f, (0, 0), mul(s.a, s.a, p), p);
    for m in [(2, 1), (1, 2)] {
        accumulate(&mut f, m, neg(mul(2, r, p), p), p);
    }
    for m in [(1, 0), (0, 1)] {
        accumulate(
            &mut f,
            m,
            neg(add(mul(mul(2, r, p), s.a, p), mul(4, s.b, p), p), p),
            p,
        );
    }
    accumulate(&mut f, (0, 0), neg(mul(mul(4, r, p), s.b, p), p), p);
    for m in [(2, 0), (0, 2)] {
        accumulate(&mut f, m, mul(r, r, p), p);
    }
    accumulate(&mut f, (1, 1), neg(mul(2, mul(r, r, p), p), p), p);
    f.into_iter().map(|((x, y), c)| Term { x, y, c }).collect()
}
#[derive(Serialize)]
struct Target {
    target: (u64, u64),
    system_path: String,
    system_sha256: String,
    witness: Option<[usize; 2]>,
}
#[derive(Serialize)]
struct Arm {
    name: String,
    x_values: Vec<u64>,
    points: Vec<(u64, u64)>,
    targets: Vec<Target>,
}
#[derive(Serialize)]
struct Manifest {
    schema: &'static str,
    spec: Spec,
    group_order: usize,
    prime_order_checked: bool,
    identity_status: &'static str,
    arms: Vec<Arm>,
}
pub(super) fn prepare(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let spec: Spec = serde_json::from_slice(&fs::read(input)?)?;
    if !(5..=257).contains(&spec.p)
        || !is_prime(spec.p)
        || spec.a >= spec.p
        || spec.b >= spec.p
        || spec.shift >= spec.p
        || spec.coset_a == 0
        || spec.coset_b == 0
        || spec.coset_a >= spec.p
        || spec.coset_b >= spec.p
        || spec.exponent == 0
        || !(spec.p - 1).is_multiple_of(spec.exponent)
        || spec.max_degree > 24
    {
        return Err("invalid bounded prime-field pilot spec".into());
    }
    if add(
        mul(4, pow(spec.a, 3, spec.p), spec.p),
        mul(27, pow(spec.b, 2, spec.p), spec.p),
        spec.p,
    ) == 0
    {
        return Err("singular curve rejected".into());
    }
    let points = curve_points(&spec);
    if !is_prime(points.len() as u64 + 1) {
        return Err("pilot requires a prime-order full curve group".into());
    }
    let candidate = base(&spec);
    if !(2..=12).contains(&candidate.len()) {
        return Err("candidate size outside 2..12 pilot range".into());
    }
    let all_x: Vec<u64> = points
        .iter()
        .map(|p| p.0)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let mut hashed = all_x.clone();
    hashed.sort_by_key(|x| {
        let mut h = Sha256::new();
        h.update(b"prime-fiber-control/v1");
        h.update(spec.seed.to_be_bytes());
        h.update(spec.p.to_be_bytes());
        h.update(x.to_be_bytes());
        (h.finalize().to_vec(), *x)
    });
    hashed.truncate(candidate.len());
    hashed.sort_unstable();
    let low = all_x
        .iter()
        .copied()
        .take(candidate.len())
        .collect::<Vec<_>>();
    let arms = [
        ("shifted-fiber-intersection", candidate),
        ("low-x-control", low),
        ("hash-control", hashed),
    ];
    fs::create_dir(output)?;
    let mut manifest = Manifest {
        schema: "prime-factor-base-pilot-input/v1",
        spec: spec.clone(),
        group_order: points.len() + 1,
        prime_order_checked: true,
        identity_status:
            "development exporter; register exact curve and canonical FB1 before promotion",
        arms: vec![],
    };
    for (name, x_values) in arms {
        let base_points = points
            .iter()
            .copied()
            .filter(|p| x_values.contains(&p.0))
            .collect::<Vec<_>>();
        let mut sums = BTreeMap::new();
        for (i, &p) in base_points.iter().enumerate() {
            for (j, &q) in base_points.iter().enumerate() {
                if let Some(r) = point_add(&spec, Some(p), Some(q)) {
                    sums.entry(r).or_insert([i, j]);
                }
            }
        }
        fs::create_dir(format!("{output}/{name}"))?;
        let mut arm = Arm {
            name: name.into(),
            x_values: x_values.clone(),
            points: base_points,
            targets: vec![],
        };
        // One representative of each +/- target pair: S3 has the identical x target.
        for &x in &all_x {
            let &target = points
                .iter()
                .find(|p| p.0 == x)
                .ok_or("target lift missing")?;
            let system = System {
                p: spec.p,
                max_degree: spec.max_degree,
                polynomials: vec![
                    membership(&x_values, spec.p, 0),
                    membership(&x_values, spec.p, 1),
                    s3(&spec, x),
                ],
            };
            validate(&system)?;
            let data = serde_json::to_vec_pretty(&system)?;
            let system_path = format!("{name}/target-{x}.json");
            fs::write(format!("{output}/{system_path}"), &data)?;
            arm.targets.push(Target {
                target,
                system_path,
                system_sha256: format!("{:x}", Sha256::digest(&data)),
                witness: sums.get(&target).copied(),
            });
        }
        manifest.arms.push(arm);
    }
    fs::write(
        format!("{output}/manifest.json"),
        serde_json::to_vec_pretty(&manifest)?,
    )?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn toy() -> Spec {
        Spec {
            p: 17,
            a: 2,
            b: 2,
            exponent: 8,
            shift: 2,
            coset_a: 1,
            coset_b: 1,
            max_degree: 8,
            seed: 315,
        }
    }
    fn eval(poly: &[Term], x: u64, y: u64, p: u64) -> u64 {
        poly.iter().fold(0, |s, t| {
            add(
                s,
                mul(t.c, mul(pow(x, t.x as u64, p), pow(y, t.y as u64, p), p), p),
                p,
            )
        })
    }
    #[test]
    fn known_group_law_and_order() {
        let s = toy();
        assert_eq!(curve_points(&s).len() + 1, 19);
        assert_eq!(point_add(&s, Some((5, 1)), Some((5, 1))), Some((6, 3)));
        assert_eq!(point_add(&s, Some((5, 1)), Some((5, 16))), None);
    }
    #[test]
    fn candidate_lift_filter_and_membership() {
        let s = toy();
        let v = base(&s);
        assert_eq!(v, vec![13, 16]);
        let f = membership(&v, s.p, 0);
        for x in 0..s.p {
            assert_eq!(eval(&f, x, 0, s.p) == 0, v.contains(&x));
        }
    }
    #[test]
    fn exported_s3_matches_one_exact_point_sum() {
        let s = toy();
        let p = (13, 7);
        let q = (16, 4);
        assert_eq!(mul(p.1, p.1, s.p), rhs(&s, p.0));
        assert_eq!(mul(q.1, q.1, s.p), rhs(&s, q.0));
        let r = point_add(&s, Some(p), Some(q)).unwrap();
        assert_eq!(eval(&s3(&s, r.0), p.0, q.0, s.p), 0);
    }
}
