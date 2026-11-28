mod affine;

use affine::{power, Point};
mod sliced;
use rand::{rngs::StdRng, Rng, SeedableRng};
use sliced::{
    aps::{self, LabeledBase, LabeledPoint, Labels},
    fast::{FastCurve, Pt},
};
use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, Write};

const SEEDS: [u64; 4] = [20261115, 20261116, 20261117, 20261118];
const DRAW_CAP: usize = 100;
const BASELINE: [u64; 4] = [1, 2, 4, 8];
const COMPACT: [u64; 4] = [1, 1, 1, 2];

struct Fixture {
    curve: FastCurve,
    reference: affine::Curve,
    g: Pt,
    q: Pt,
    affine_g: Point,
    affine_q: Point,
}

impl Fixture {
    fn new(p: u64, a: u64, b: u64, r: u64, g: (u64, u64), known: u64) -> Self {
        let curve = FastCurve::new("APS disjoint label probe", p, a, b, r);
        let reference = affine::Curve { modulus: p, a, b };
        let affine_g = Some(g);
        let fast_g = curve.point(g.0, g.1);
        let q = curve.mul(Some(fast_g), known).expect("valid fixture Q");
        let affine_q = Some(curve.canonical(q));
        assert!(reference.contains(g));
        assert_eq!(reference.mul(affine_g, known), affine_q);
        assert_eq!(reference.mul(affine_g, r), None);
        aps::validate_instance(&curve, fast_g, q).expect("valid APS fixture");
        Self {
            curve,
            reference,
            g: fast_g,
            q,
            affine_g,
            affine_q,
        }
    }

    fn reference_label(&self, u: u64, v: u64) -> Point {
        self.reference.add(
            self.reference.mul(self.affine_g, u),
            self.reference.mul(self.affine_q, v),
        )
    }
}

struct Draw {
    slot: usize,
    u: u64,
    point: Point,
    flipped: bool,
    outcome: &'static str,
}

struct Sample {
    base: LabeledBase,
    draws: Vec<Draw>,
    complete: bool,
}

fn sample(fixture: &Fixture, size: usize, weights: [u64; 4], stream: &[u64]) -> Sample {
    let mut base = LabeledBase {
        parts: vec![Vec::new(); 4],
        points: BTreeMap::new(),
        draws: 0,
    };
    let mut draws = Vec::new();
    let n = fixture.curve.n;
    let shifts = weights.map(|weight| fixture.curve.mul(Some(fixture.q), weight));
    for (slot, weight) in weights.into_iter().enumerate() {
        while base.parts[slot].len() < size {
            let Some(&u) = stream.get(base.draws) else {
                return Sample {
                    base,
                    draws,
                    complete: false,
                };
            };
            base.draws += 1;
            let candidate = fixture
                .curve
                .add(fixture.curve.mul(Some(fixture.g), u), shifts[slot]);
            assert_eq!(
                candidate.map(|p| fixture.curve.canonical(p)),
                fixture.reference_label(u, weight)
            );
            let Some(point) = candidate else {
                draws.push(Draw {
                    slot,
                    u,
                    point: None,
                    flipped: false,
                    outcome: "infinity",
                });
                continue;
            };
            let (point, flipped) = fixture.curve.canonical_sign(point);
            let (x, y) = fixture.curve.canonical(point);
            let mut outcome = "duplicate_x";
            if let std::collections::btree_map::Entry::Vacant(entry) = base.points.entry(x) {
                let (u, v) = if flipped {
                    ((n - u) % n, n - weight)
                } else {
                    (u, weight)
                };
                assert_eq!(fixture.reference_label(u, v), Some((x, y)));
                entry.insert(LabeledPoint { x, y, u, v });
                base.parts[slot].push(x);
                outcome = "accepted";
            }
            draws.push(Draw {
                slot,
                u,
                point: Some((x, y)),
                flipped,
                outcome,
            });
        }
    }
    Sample {
        base,
        draws,
        complete: true,
    }
}

fn check_production_sampler(f: &Fixture, size: usize, seed: u64, cap: usize, trial: &Sample) {
    let baseline = aps::sample_base(
        &f.curve,
        f.g,
        f.q,
        &[size; 4],
        Labels::Guaranteed,
        seed,
        cap,
    );
    if trial.complete {
        let baseline = baseline.expect("coupled production baseline should complete");
        assert_eq!(baseline.draws, trial.base.draws);
        assert_eq!(baseline.parts, trial.base.parts);
        for (x, point) in baseline.points {
            let other = &trial.base.points[&x];
            assert_eq!(
                (point.x, point.y, point.u, point.v),
                (other.x, other.y, other.u, other.v)
            );
        }
    } else {
        assert_eq!(trial.base.draws, cap);
        assert_eq!(
            baseline.unwrap_err(),
            "factor-base sampling budget exhausted"
        );
    }
}

fn signed<const N: usize>(coefficients: [u64; N], r: u64, mask: usize) -> u64 {
    coefficients
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, weight)| {
            let term = if mask & (1 << i) == 0 {
                weight
            } else {
                r - weight
            };
            (acc + term) % r
        })
}

// From the bridge's centered-magnitude rule. An odd number of minimum
// valuations makes the scaled signed sum odd, while the sum of magnitudes
// strictly below r rules out nonzero multiples of r.
fn valuation_certain(weights: [u64; 4], r: u64) -> bool {
    let centered = weights.map(|w| w.min(r - w));
    let minimum = centered
        .iter()
        .filter(|&&w| w != 0)
        .map(|w| w.trailing_zeros())
        .min();
    centered.iter().sum::<u64>() < r
        && minimum.is_some_and(|v| {
            centered
                .iter()
                .filter(|&&w| w != 0 && w.trailing_zeros() == v)
                .count()
                % 2
                == 1
        })
}

fn prove_layout(base: &LabeledBase, weights: [u64; 4], r: u64) -> usize {
    let bins: Vec<_> = base.parts.iter().flatten().copied().collect();
    assert_eq!(
        bins.iter().copied().collect::<BTreeSet<_>>().len(),
        bins.len()
    );
    assert!(valuation_certain(weights, r));
    assert!((0..16).all(|mask| signed(weights, r, mask) != 0));
    for (part, weight) in base.parts.iter().zip(weights) {
        assert!(part.iter().all(|x| {
            let v = base.points[x].v;
            v == weight || v == r - weight
        }));
    }
    bins.len()
}

#[derive(Default)]
struct DomainAudit {
    roots: BTreeSet<Vec<u64>>,
    tuples: usize,
    informative: usize,
    zero_denominator: usize,
}

fn reference_recover(f: &Fixture, a: u64, b: u64) -> u64 {
    let r = f.curve.n;
    assert_ne!(b, 0);
    let inverse = power(b, r - 2, r);
    assert_eq!(b * inverse % r, 1);
    let recovered = (r - a) * inverse % r;
    assert_eq!(f.reference.mul(f.affine_g, recovered), f.affine_q);
    recovered
}

fn audit_domain(
    f: &Fixture,
    base: &LabeledBase,
    mut witness: impl FnMut(&[u64], usize, u64, u64, u64) -> io::Result<()>,
) -> io::Result<DomainAudit> {
    let mut audit = DomainAudit::default();
    let n = f.curve.n;
    for &x in &base.parts[0] {
        for &y in &base.parts[1] {
            for &z in &base.parts[2] {
                for &w in &base.parts[3] {
                    let tuple = [x, y, z, w];
                    let points = tuple.map(|coordinate| &base.points[&coordinate]);
                    let labels_u = points.map(|point| point.u);
                    let labels_v = points.map(|point| point.v);
                    assert!(valuation_certain(labels_v, n));
                    audit.tuples += 1;
                    for mask in (0..16).step_by(2) {
                        let denominator = signed(labels_v, n, mask);
                        assert_ne!(denominator, 0);
                        let mut group_sum = None;
                        for (i, point) in points.iter().enumerate() {
                            let mut value = (point.x, point.y);
                            if mask & (1 << i) != 0 {
                                value = f.reference.negate(value);
                            }
                            group_sum = f.reference.add(group_sum, Some(value));
                        }
                        if group_sum.is_some() {
                            continue;
                        }
                        let numerator = signed(labels_u, n, mask);
                        assert_eq!(f.reference_label(numerator, denominator), None);
                        let mut root = tuple.to_vec();
                        root.sort_unstable();
                        audit.roots.insert(root);
                        if denominator == 0 {
                            audit.zero_denominator += 1;
                        } else {
                            audit.informative += 1;
                            let logarithm = reference_recover(f, numerator, denominator);
                            witness(&tuple, mask, numerator, denominator, logarithm)?;
                        }
                    }
                }
            }
        }
    }
    Ok(audit)
}

fn audit_mixed(base: &LabeledBase, r: u64) -> (usize, usize, usize) {
    let weights: Vec<_> = base.points.values().map(|point| point.v).collect();
    let mut result = (0, 0, 0);
    for a in 0..weights.len() {
        for b in a + 1..weights.len() {
            for c in b + 1..weights.len() {
                for d in c + 1..weights.len() {
                    let tuple = [weights[a], weights[b], weights[c], weights[d]];
                    let certain = valuation_certain(tuple, r);
                    let safe = (0..16).all(|mask| signed(tuple, r, mask) != 0);
                    assert!(!certain || safe);
                    result.0 += 1;
                    result.1 += usize::from(certain);
                    result.2 += usize::from(safe);
                }
            }
        }
    }
    result
}

fn split_roots(f: &Fixture, base: &LabeledBase) -> Vec<Vec<u64>> {
    let p = f.curve.f.p;
    let equations = aps::split_equations(&f.curve);
    let mut roots = BTreeSet::new();
    for &x in &base.parts[0] {
        for &y in &base.parts[1] {
            for &z in &base.parts[2] {
                for &w in &base.parts[3] {
                    let has_auxiliary = (0..p).any(|t| {
                        let values = [x, y, z, w, t];
                        equations.iter().all(|eq| {
                            eq.iter().fold(0, |sum, (exponents, coefficient)| {
                                let value = exponents
                                    .iter()
                                    .zip(values)
                                    .fold(*coefficient, |acc, (e, v)| {
                                        acc * power(v, u64::from(*e), p) % p
                                    });
                                (sum + value) % p
                            }) == 0
                        })
                    });
                    if has_auxiliary {
                        let mut tuple = vec![x, y, z, w];
                        tuple.sort_unstable();
                        roots.insert(tuple);
                    }
                }
            }
        }
    }
    roots.into_iter().collect()
}
fn check_recovery(f: &Fixture, base: &LabeledBase, roots: &[Vec<u64>]) -> Option<aps::Recovery> {
    let recovery = aps::recover(&f.curve, f.g, f.q, base, roots).expect("valid recovery");
    assert_eq!(recovery.is_some(), !roots.is_empty());
    if let Some(witness) = &recovery {
        assert_eq!(
            witness.logarithm,
            reference_recover(f, witness.a, witness.b)
        );
        assert!(roots.contains(&witness.coordinates));
        let mask = witness
            .signs
            .iter()
            .enumerate()
            .fold(0, |mask, (i, &sign)| mask | ((sign < 0) as usize) << i);
        let entries: Vec<_> = witness
            .coordinates
            .iter()
            .map(|x| &base.points[x])
            .collect();
        let points: [&LabeledPoint; 4] =
            entries.try_into().unwrap_or_else(|_| panic!("arity four"));
        assert_eq!(
            signed(points.map(|point| point.u), f.curve.n, mask),
            witness.a
        );
        assert_eq!(
            signed(points.map(|point| point.v), f.curve.n, mask),
            witness.b
        );
        let sum = points.iter().enumerate().fold(None, |sum, (i, point)| {
            let value = (point.x, point.y);
            let value = if mask & (1 << i) == 0 {
                value
            } else {
                f.reference.negate(value)
            };
            f.reference.add(sum, Some(value))
        });
        assert_eq!(sum, None);
    }
    recovery
}

fn run_arm(
    out: &mut impl Write,
    f: &Fixture,
    seed: u64,
    size: usize,
    weights: [u64; 4],
    stream: &[u64],
    stage: &str,
) -> io::Result<()> {
    let law = if weights == BASELINE {
        "baseline"
    } else {
        "compact"
    };
    let id = format!(
        "{}/{seed}/{size}/{law}/{stage}/{}",
        f.curve.f.p,
        stream.len()
    );
    let trial = sample(f, size, weights, stream);
    if weights == BASELINE {
        check_production_sampler(f, size, seed, stream.len(), &trial);
    }
    for (index, draw) in trial.draws.iter().enumerate() {
        assert_eq!(draw.u, stream[index]);
        writeln!(
            out,
            "draw\t{id}\t{index}\t{}\t{}\t{}\t{:?}\t{}\t{}",
            draw.slot, draw.u, weights[draw.slot], draw.point, draw.flipped, draw.outcome
        )?;
    }
    if stage == "cap" {
        writeln!(
            out,
            "cap\t{id}\t{}\t{}\t{}",
            trial.complete,
            trial.base.draws,
            trial.base.points.len()
        )?;
        return Ok(());
    }
    assert!(trial.complete);
    let base = trial.base;
    let chosen = prove_layout(&base, weights, f.curve.n);
    aps::validate_base(&f.curve, f.g, f.q, &base).expect("valid sampled labels");
    let mut corrupted = base.clone();
    let point = corrupted.points.values_mut().next().expect("nonempty base");
    point.u = (point.u + 1) % f.curve.n;
    assert!(aps::validate_base(&f.curve, f.g, f.q, &corrupted).is_err());

    let audit = audit_domain(f, &base, |tuple, sign, a, b, logarithm| {
        writeln!(
            out,
            "witness\t{id}\t{tuple:?}\t{sign}\t{a}\t{b}\t{logarithm}"
        )
    })?;
    assert_eq!(audit.tuples, size.pow(4));
    assert_eq!(audit.zero_denominator, 0);
    let mixed = audit_mixed(&base, f.curve.n);
    let roots: Vec<_> = audit.roots.into_iter().collect();
    let exact_recovery = check_recovery(f, &base, &roots);

    let split = split_roots(f, &base);
    assert_eq!(split, roots);
    let recovered = check_recovery(f, &base, &split);

    writeln!(
        out,
        "arm\t{id}\t{chosen}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{:?}\t{:?}",
        base.draws,
        audit.tuples,
        roots.len(),
        audit.informative,
        audit.zero_denominator,
        mixed.0,
        mixed.1,
        mixed.2,
        split.len(),
        4,
        exact_recovery.map(|r| r.logarithm),
        recovered.map(|r| r.logarithm),
    )
}

fn main() -> io::Result<()> {
    assert!(!valuation_certain([1, 2, 2, 2], 7));
    assert!((0..16).any(|mask| signed([1, 2, 2, 2], 7, mask) == 0));
    assert!(valuation_certain(COMPACT, 7));
    assert!(!valuation_certain(BASELINE, 7));
    assert!(!valuation_certain([1; 4], 107));
    assert!((0..16).any(|mask| signed([1; 4], 107, mask) == 0));

    let fixtures = [
        Fixture::new(101, 59, 6, 107, (78, 4), 37),
        Fixture::new(211, 166, 158, 227, (155, 121), 73),
    ];
    let mut out = io::BufWriter::new(io::stdout().lock());
    writeln!(out, "# id=field/seed/slot_size/law/stage/draw_cap")?;
    writeln!(out, "# draw=id,index,slot,u,v,canonical_xy,flipped,outcome")?;
    writeln!(
        out,
        "# witness=id,slot_ordered_x,sign_mask,numerator,denominator,logarithm"
    )?;
    writeln!(out, "# cap=id,complete,draws,points")?;
    writeln!(out, "# arm=id,points,draws,tuples,exact_roots,informative,zero_denominator,mixed_tuples,mixed_valuation_certain,mixed_exact_safe,split_roots,shift_calls,exact_recovered,split_recovered")?;

    for fixture in &fixtures {
        for size in [2, 3] {
            for seed in SEEDS {
                let mut rng = StdRng::seed_from_u64(seed);
                let stream: Vec<_> = (0..DRAW_CAP)
                    .map(|_| rng.gen_range(0..fixture.curve.n))
                    .collect();
                for weights in [BASELINE, COMPACT] {
                    run_arm(&mut out, fixture, seed, size, weights, &stream, "main")?;
                }
                if fixture.curve.f.p == 101 && size == 2 && seed == SEEDS[0] {
                    for cap in [0, 7, 8] {
                        for weights in [BASELINE, COMPACT] {
                            run_arm(
                                &mut out,
                                fixture,
                                seed,
                                size,
                                weights,
                                &stream[..cap],
                                "cap",
                            )?;
                        }
                    }
                }
            }
        }
    }
    out.flush()
}
