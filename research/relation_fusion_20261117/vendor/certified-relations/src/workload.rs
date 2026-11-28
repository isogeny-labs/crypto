//! Public known-answer geometry fixtures. No witness is passed to preprocessing.
use super::*;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fixture {
    pub id: String,
    pub system: System,
    pub witness: Vec<u64>,
    pub public_scalars: Vec<u64>,
}
fn add(left: Option<(u64, u64)>, right: Option<(u64, u64)>, p: u64) -> Option<(u64, u64)> {
    let Some((x, y)) = left else {
        return right;
    };
    let Some((u, v)) = right else {
        return left;
    };
    if x == u && (y + v) % p == 0 {
        return None;
    }
    let slope = if x == u {
        mul((mul(3, mul(x, x, p), p) + 2) % p, inv(mul(2, y, p), p), p)
    } else {
        mul((v + p - y) % p, inv((u + p - x) % p, p), p)
    };
    let z = (mul(slope, slope, p) + 2 * p - x - u) % p;
    Some((z, (mul(slope, (x + p - z) % p, p) + p - y) % p))
}
fn multiple(mut k: u64, p: u64) -> Option<(u64, u64)> {
    let mut result = None;
    let mut point = Some((3, 6));
    while k > 0 {
        if k & 1 == 1 {
            result = add(result, point, p);
        }
        point = add(point, point, p);
        k >>= 1;
    }
    result
}
fn next(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e3779b97f4a7c15);
    let mut x = *state;
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}
pub fn generate(p: u64, n: usize, seed: u64) -> Result<Fixture, String> {
    if ![2147483647, 9007199254740881].contains(&p) || !(2..=6).contains(&n) {
        return Err("fixture grid: stated primes and 2..6 variables".into());
    }
    let mut edges = BTreeSet::new();
    for i in 0..n - 1 {
        edges.insert((i, i + 1));
    }
    if n > 2 {
        edges.insert((0, n - 1));
    }
    generate_impl(p, n, seed, edges, format!("p{p}-cycle{n}-seed{seed}"))
}
/// Explicit bounded graph fixtures. Solver inputs contain equations only;
/// public witness/scalars remain generation/checking data outside construction.
pub fn generate_graph(
    p: u64,
    n: usize,
    seed: u64,
    edges: &[(usize, usize)],
) -> Result<Fixture, String> {
    if ![2147483647, 9007199254740881].contains(&p) || !(2..=8).contains(&n) || edges.len() > 16 {
        return Err("graph fixture grid: stated primes, 2..8 variables, <=16 edges".into());
    }
    generate_checked_graph(p, n, seed, edges)
}
/// Explicit capacity experiment. Original fixture entry points keep their limits.
#[cfg(feature = "scale-screen")]
pub fn generate_scale_graph(
    p: u64,
    n: usize,
    seed: u64,
    edges: &[(usize, usize)],
) -> Result<Fixture, String> {
    if ![2147483647, 9007199254740881].contains(&p) || !(2..=16).contains(&n) || edges.len() > 16 {
        return Err("scale fixture grid: stated primes, 2..16 variables, <=16 edges".into());
    }
    generate_checked_graph(p, n, seed, edges)
}
fn generate_checked_graph(
    p: u64,
    n: usize,
    seed: u64,
    edges: &[(usize, usize)],
) -> Result<Fixture, String> {
    let mut normalized = BTreeSet::new();
    for &(a, b) in edges {
        if a >= n || b >= n || a == b || !normalized.insert((a.min(b), a.max(b))) {
            return Err("invalid/duplicate graph edge".into());
        }
    }
    let mut reached = vec![false; n];
    reached[0] = true;
    let mut queue = VecDeque::from([0]);
    while let Some(v) = queue.pop_front() {
        for &(a, b) in &normalized {
            let next = if a == v {
                Some(b)
            } else if b == v {
                Some(a)
            } else {
                None
            };
            if let Some(w) = next {
                if !reached[w] {
                    reached[w] = true;
                    queue.push_back(w);
                }
            }
        }
    }
    if reached.iter().any(|x| !*x) {
        return Err("graph fixture must be connected".into());
    }
    let label = normalized
        .iter()
        .map(|(a, b)| format!("{a}-{b}"))
        .collect::<Vec<_>>()
        .join("_");
    generate_impl(
        p,
        n,
        seed,
        normalized,
        format!("p{p}-graph{n}-{label}-seed{seed}"),
    )
}
fn generate_impl(
    p: u64,
    n: usize,
    seed: u64,
    edges: BTreeSet<(usize, usize)>,
    id: String,
) -> Result<Fixture, String> {
    let mut state = seed;
    let mut points = Vec::new();
    let mut scalars = Vec::new();
    for _ in 0..1000 {
        if points.len() == n {
            break;
        }
        let k = 2 + next(&mut state) % 100000;
        if let Some(point) = multiple(k, p) {
            if point.1 != 0 && points.iter().all(|q: &(u64, u64)| q.0 != point.0) {
                points.push(point);
                scalars.push(k);
            }
        }
    }
    if points.len() != n {
        return Err("point fixture generation cap".into());
    }
    let mut f = Vec::new();
    for (i, j) in edges {
        let u = add(Some(points[i]), Some(points[j]), p)
            .ok_or("plus point at infinity")?
            .0;
        let v = add(
            Some(points[i]),
            Some((points[j].0, minus(points[j].1, p))),
            p,
        )
        .ok_or("minus point at infinity")?
        .0;
        if u == v {
            return Err("degenerate summation pair".into());
        }
        f.push(encode(&cheap::f3(n, [i, j], 2, 3, u, p)));
        f.push(encode(&cheap::f3(n, [i, j], 2, 3, v, p)));
    }
    let system = System {
        prime: p,
        variables: (0..n).map(|i| format!("x{i}")).collect(),
        polynomials: f,
    };
    let result = Fixture {
        id,
        system,
        witness: points.iter().map(|r| r.0).collect(),
        public_scalars: scalars,
    };
    if !witness_check(&result.system, &result.witness) {
        return Err("invalid generated fixture".into());
    }
    Ok(result)
}
pub fn witness_check(system: &System, witness: &[u64]) -> bool {
    if witness.len() != system.variables.len() {
        return false;
    }
    let Ok(f) = system.decode() else {
        return false;
    };
    let p = system.prime;
    f.iter().all(|g| {
        g.iter().fold(0u64, |total, (m, c)| {
            let term = m
                .iter()
                .zip(witness)
                .fold(*c, |v, (e, x)| mul(v, pow(*x, *e as u64, p), p));
            (total + term) % p
        }) == 0
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "scale-screen")]
    #[test]
    fn scale_entry_keeps_old_admission_and_legacy_values() {
        let p = 2147483647;
        assert!(generate_graph(p, 9, 7, &[]).is_err());
        assert!(generate_scale_graph(p, 17, 7, &[]).is_err());
        assert!(generate_scale_graph(p, 16, 7, &[(0, 16)]).is_err());
        let edges = [(0, 1), (1, 2)];
        assert_eq!(
            serde_json::to_vec(&generate_graph(p, 3, 7, &edges).unwrap().system).unwrap(),
            serde_json::to_vec(&generate_scale_graph(p, 3, 7, &edges).unwrap().system).unwrap()
        );
    }
    #[test]
    fn graph_validation_rejects_bad_shapes_before_point_generation() {
        let p = 2147483647;
        for edges in [
            vec![],
            vec![(0, 0)],
            vec![(0, 3)],
            vec![(0, 1), (1, 0)],
            vec![(0, 1)],
        ] {
            assert!(generate_graph(p, 3, 7, &edges).is_err());
        }
        assert!(generate_graph(p, 9, 7, &[(0, 1)]).is_err());
        assert!(generate_graph(1u64 << 53, 3, 7, &[(0, 1), (1, 2)]).is_err());
        assert!(generate(p, 7, 7).is_err(), "legacy admission is unchanged");
    }
    #[test]
    fn explicit_graphs_preserve_witnesses_and_original_input_identities() {
        let p = 2147483647;
        for n in [6, 7, 8] {
            let order: Vec<usize> = match n {
                6 => vec![0, 1, 2, 5, 3, 4],
                7 => vec![0, 1, 2, 6, 3, 4, 5],
                _ => vec![0, 1, 2, 3, 7, 4, 5, 6],
            };
            let path: Vec<_> = order.windows(2).map(|w| (w[0], w[1])).collect();
            let fixture = generate_graph(p, n, 7, &path).unwrap();
            assert!(witness_check(&fixture.system, &fixture.witness));
            let f = fixture.system.decode().unwrap();
            let pairs = cheap::consequences(&f, n, p);
            assert_eq!(pairs.len(), n - 1);
            assert!(pairs
                .iter()
                .all(|r| verify(&r.cubic_difference, &f, p) && verify(&r.cubic_syzygy, &f, p)));
        }
    }
    #[test]
    fn graph_entry_matches_legacy_cycle_interface_on_both_fields() {
        for p in [2147483647, 9007199254740881] {
            for n in [2, 3, 4, 6] {
                let mut cycle: Vec<_> = (0..n - 1).map(|i| (i, i + 1)).collect();
                if n > 2 {
                    cycle.push((0, n - 1));
                }
                let old = generate(p, n, 7).unwrap();
                let explicit = generate_graph(p, n, 7, &cycle).unwrap();
                assert_eq!(
                    serde_json::to_vec(&old.system).unwrap(),
                    serde_json::to_vec(&explicit.system).unwrap()
                );
                assert_eq!(old.witness, explicit.witness);
                assert_eq!(old.public_scalars, explicit.public_scalars);
            }
        }
    }
    #[test]
    fn shared_point_inputs_have_valid_witnesses_and_certificates() {
        for p in [2147483647, 9007199254740881] {
            for n in [2, 3, 4, 6] {
                for seed in [7, 101] {
                    let fixture = generate(p, n, seed).unwrap();
                    assert!(witness_check(&fixture.system, &fixture.witness));
                    let f = fixture.system.decode().unwrap();
                    let (_, pairs) = cheap::augment(&f, n, p);
                    assert_eq!(pairs.len(), if n == 2 { 1 } else { n });
                    for pair in pairs {
                        assert!(verify(&pair.cubic_syzygy, &f, p));
                    }
                }
            }
        }
    }
}
