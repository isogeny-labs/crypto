#![forbid(unsafe_code)]
#[allow(dead_code)]
mod upstream {
    include!("upstream/arithmetic.rs");
}
#[allow(dead_code)]
mod independent_reference;
#[allow(dead_code)]
#[path = "upstream/original_chord.rs"]
mod original_chord;
#[path = "upstream/original_reference.rs"]
mod symmetric_reference;
mod transport;

use independent_reference as reference;
use original_chord::{add, inv, mul, sub, Diagnostics, P};
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufWriter, Write};
use transport::{Anchor, Frame, Label, LabelSet, Matrix};
use upstream::{Arithmetic, Curve, Point};

const MASTER: u64 = 0x5452414e53503331;
const REPS: [u64; 4] = [
    0x5245504c00000001,
    0x5245504c00000002,
    0x5245504c00000003,
    0x5245504c00000004,
];
const TAGS: [u64; 5] = [
    0x5441524745540001,
    0x464143544f520001,
    0x534c4f5045530001,
    0x504f534954560001,
    0x5052454649580001,
];
const FIXTURES: [Curve; 2] = [
    Curve {
        p: P,
        a: P - 3,
        b: 21,
        n: 2_147_487_637,
        g: (1_391_443_431, 177_979_044),
    },
    upstream::PRIME31,
];

fn mix(mut x: u64) -> u64 {
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        mix(self.0)
    }
    fn field(&mut self) -> u64 {
        self.next() % P
    }
    fn point(&mut self, e: Curve) -> Point {
        Arithmetic::default().scalar(e, 1 + self.next() % (e.n - 1), Some(e.g))
    }
}
fn seed(fixture: usize, rep: usize, stream: usize) -> u64 {
    let fixture_tag = 0x4355525600000001 + fixture as u64;
    let key = mix(mix(MASTER ^ fixture_tag) ^ REPS[rep]);
    mix(key ^ TAGS[stream])
}
fn prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    let mut d = 3;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}
fn isqrt(n: u64) -> u64 {
    if n < 2 {
        return n;
    }
    let (mut lo, mut hi) = (0, n.min(1u64 << 32));
    while lo < hi {
        let m = lo + (hi - lo).div_ceil(2);
        if m <= n / m {
            lo = m;
        } else {
            hi = m - 1;
        }
    }
    lo
}
fn anchors(e: Curve) -> [Anchor; 2] {
    let mut out = Vec::new();
    for s in 0..1024 {
        let rhs = Arithmetic::default().rhs(e, s);
        if rhs == 0 {
            continue;
        }
        if let Some(delta) = Arithmetic::default().sqrt(sub(0, rhs), P) {
            let delta = delta.min(P - delta);
            assert!(reference::trace_zero_anchor(e.a, e.b, s, delta));
            out.push(Anchor { s, delta });
            if out.len() == 2 {
                return [out[0], out[1]];
            }
        }
    }
    panic!("anchor search exceeded frozen deterministic cap");
}
fn ref_label(l: Label) -> reference::Fp2 {
    reference::cayley(l.coordinates().unwrap()).unwrap()
}
fn as_label(z: reference::Fp2) -> Label {
    let (x, y) = reference::canonical_parameter(z).unwrap();
    if y == 0 {
        assert_eq!(x, 1);
        Label::Identity
    } else {
        assert_eq!(y, 1);
        Label::Finite(x)
    }
}
fn ref_slope(frame: Frame, side: usize, label: Label) -> Option<u64> {
    let (w, v) = frame.residual();
    let a = frame.anchors()[side];
    reference::recover_slope(
        w,
        v,
        a.s,
        a.delta,
        ref_label(frame.factors()[side]),
        ref_label(label),
    )
    .unwrap()
}
fn ref_at(frame: Frame, side: usize, slope: Option<u64>) -> Label {
    let (w, v) = frame.residual();
    let a = frame.anchors()[side];
    as_label(
        reference::evaluate_label(w, v, a.s, a.delta, ref_label(frame.factors()[side]), slope)
            .unwrap(),
    )
}
fn slope_of(e: Curve, p: Point, q: Point) -> Option<u64> {
    let (Some((x, y)), Some((u, v))) = (p, q) else {
        return None;
    };
    if x == u && add(y, v) == 0 {
        return None;
    }
    Some(if x == u {
        mul(add(mul(3, mul(x, x)), e.a), inv(add(y, y)))
    } else {
        mul(sub(v, y), inv(sub(u, x)))
    })
}
fn distinct_pair(e: Curve, k: Point, rng: &mut Rng) -> (Point, Point) {
    for _ in 0..64 {
        let p = rng.point(e);
        let q = reference::point_add(e.a, e.b, k, reference::point_neg(p)).unwrap();
        if q.is_some() && p != q {
            return (p, q);
        }
    }
    panic!("positive pair generation exceeded cap");
}

/// The retained separately authored symmetric-Semaev implementation computes
/// a complete superset for the implied x-Cayley product. Direct Fp^2 chord
/// evaluation, not the candidate slope decoder, selects the exact fiber.
fn reference_pairs(frame: Frame, slope: Option<u64>, label: Label) -> Vec<(Point, Point)> {
    let Some(m) = slope else {
        return vec![
            (None, Some(frame.residual())),
            (Some(frame.residual()), None),
        ];
    };
    let (w, v) = frame.residual();
    let e = frame.curve();
    let m2 = reference::fp_mul(m, m);
    let s = reference::fp_sub(m2, w);
    let q = reference::fp_add(
        reference::fp_add(
            reference::fp_add(reference::fp_mul(w, m2), reference::fp_mul(w, w)),
            e.a,
        ),
        reference::fp_mul(reference::fp_add(v, v), m),
    );
    let lambda = if s == 0 {
        None
    } else {
        Some(reference::fp_mul(
            reference::fp_sub(1, q),
            reference::fp_inverse(s).unwrap(),
        ))
    };
    let anchor = frame.anchors()[0];
    let eta = ref_label(frame.factors()[0]);
    let mut pairs = Vec::new();
    for (p, q) in symmetric_reference::extract(e.a, e.b, w, v, lambda) {
        let chord =
            reference::chord_ratio(e.a, e.b, Some(p), Some(q), anchor.s, anchor.delta).unwrap();
        if reference::extension_mul(eta, chord) == ref_label(label) {
            pairs.push((Some(p), Some(q)));
        }
    }
    pairs.sort_unstable();
    pairs.dedup();
    pairs
}
fn label_number(l: Label) -> u64 {
    match l {
        Label::Identity => P,
        Label::Finite(x) => x,
    }
}
fn point_json(p: Point) -> String {
    match p {
        None => "null".into(),
        Some((x, y)) => format!("[{x},{y}]"),
    }
}
fn pairs_json(pairs: &[(Point, Point)]) -> String {
    format!(
        "[{}]",
        pairs
            .iter()
            .map(|(p, q)| format!("[{},{}]", point_json(*p), point_json(*q)))
            .collect::<Vec<_>>()
            .join(",")
    )
}
fn labels_json(labels: &[Label]) -> String {
    format!(
        "[{}]",
        labels
            .iter()
            .map(|x| label_number(*x).to_string())
            .collect::<Vec<_>>()
            .join(",")
    )
}
fn fail_guard<T>(result: Result<T, String>, count: &mut usize) {
    assert!(result.is_err());
    *count += 1;
}

fn malformed(e: Curve, an: [Anchor; 2]) -> usize {
    let k = Some(e.g);
    let fs = [Label::Identity; 2];
    let mut count = 0;
    fail_guard(
        Frame::new(Curve { a: 1, b: 0, ..e }, Some((0, 0)), an, fs),
        &mut count,
    );
    for bad in [
        Curve { p: P - 2, ..e },
        Curve { a: P, ..e },
        Curve { b: P, ..e },
        Curve { a: 0, b: 0, ..e },
    ] {
        fail_guard(Frame::new(bad, k, an, fs), &mut count);
    }
    for bad in [None, Some((P, e.g.1)), Some((e.g.0, P)), Some((0, 0))] {
        fail_guard(Frame::new(e, bad, an, fs), &mut count);
    }
    for bad in [
        Anchor { s: P, ..an[0] },
        Anchor { delta: 0, ..an[0] },
        Anchor { delta: P, ..an[0] },
        Anchor {
            s: e.g.0,
            delta: an[0].delta,
        },
    ] {
        fail_guard(Frame::new(e, k, [bad, an[1]], fs), &mut count);
    }
    fail_guard(
        Frame::new(e, k, an, [Label::Finite(P), Label::Identity]),
        &mut count,
    );
    fail_guard(Label::from_coordinates(0, 0), &mut count);
    fail_guard(Label::from_coordinates(P, 1), &mut count);
    fail_guard(Label::from_coordinates(1, P), &mut count);
    fail_guard(Matrix::new([[1, 0], [0, 0]]), &mut count);
    fail_guard(Matrix::new([[P, 0], [0, 1]]), &mut count);
    fail_guard(LabelSet::new(&[Label::Finite(P)]), &mut count);
    let f = Frame::new(e, k, an, fs).unwrap();
    fail_guard(f.label_from_slope(2, Some(0)), &mut count);
    fail_guard(f.label_from_slope(0, Some(P)), &mut count);
    fail_guard(f.slope_from_label(0, Label::Finite(P)), &mut count);
    fail_guard(Frame::from_prefix(e, k, Some((P, 0)), None, an), &mut count);
    fail_guard(Frame::from_prefix(e, k, k, None, an), &mut count);
    fail_guard(
        transport::chord_ratio(e, Some((P, 0)), k, an[0]),
        &mut count,
    );
    count
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(
        args.len(),
        2,
        "usage: residual-transport-check OUTPUT_DIRECTORY"
    );
    let out = std::path::Path::new(&args[1]);
    std::fs::create_dir_all(out).unwrap();
    let mut cases = BufWriter::new(File::create(out.join("cases.jsonl")).unwrap());
    let mut intersections = BufWriter::new(File::create(out.join("intersections.jsonl")).unwrap());
    let mut prefixes = BufWriter::new(File::create(out.join("prefixes.jsonl")).unwrap());
    let mut seeds = BufWriter::new(File::create(out.join("seeds.jsonl")).unwrap());
    let mut unique_seeds = BTreeSet::new();
    for f in 0..2 {
        for r in 0..4 {
            for s in 0..5 {
                let child = seed(f, r, s);
                assert!(unique_seeds.insert(child));
                writeln!(seeds,"{{\"fixture\":{f},\"replicate\":{r},\"stream\":{s},\"seed\":\"0x{child:016x}\"}}").unwrap();
            }
        }
    }
    seeds.flush().unwrap();
    assert_eq!(unique_seeds.len(), 40);
    let (
        mut targets,
        mut frames,
        mut rows,
        mut empty,
        mut pair_occurrences,
        mut incompat,
        mut sqrt_calls,
    ) = (0, 0, 0, 0, 0, 0, 0);
    let (mut random_empty, mut controlled_empty, mut random_pairs, mut controlled_pairs) =
        (0, 0, 0, 0);
    let (
        mut set_queries,
        mut set_visits,
        mut reference_set_pairs,
        mut prefix_checks,
        mut positive_checks,
    ) = (0, 0, 0, 0, 0);
    let (mut invalid, mut sanity) = (0, 0);
    for (fi, e) in FIXTURES.iter().copied().enumerate() {
        assert!(prime(P) && prime(e.n) && reference::nonsingular(e.a, e.b));
        let mut ar = Arithmetic::default();
        assert!(ar.on_curve(e, Some(e.g)));
        assert_eq!(ar.scalar(e, e.n, Some(e.g)), None);
        assert_eq!(ar.scalar(e, e.n - 1, Some(e.g)), ar.neg(e, Some(e.g)));
        assert!(
            e.n >= P + 1 - isqrt(4 * P)
                && e.n <= P + 1 + isqrt(4 * P)
                && 2 * e.n > P + 1 + isqrt(4 * P)
        );
        let an = anchors(e);
        invalid += malformed(e, an);
        let eta = Label::Finite(17);
        let same = Frame::new(e, Some(e.g), [an[0], an[0]], [eta, eta]).unwrap();
        let conjugate = Anchor {
            s: an[0].s,
            delta: sub(0, an[0].delta),
        };
        let conj = Frame::new(
            e,
            Some(e.g),
            [an[0], conjugate],
            [eta, eta.inverse().unwrap()],
        )
        .unwrap();
        for label in [
            Label::Identity,
            Label::Finite(0),
            Label::Finite(1),
            Label::Finite(P - 1),
        ] {
            assert_eq!(same.transport().apply(label).unwrap(), label);
            assert_eq!(
                conj.transport().apply(label).unwrap(),
                label.inverse().unwrap()
            );
            sanity += 2;
        }
        for rep in 0..4 {
            let mut target_rng = Rng(seed(fi, rep, 0));
            let mut factor_rng = Rng(seed(fi, rep, 1));
            let mut slope_rng = Rng(seed(fi, rep, 2));
            let mut positive_rng = Rng(seed(fi, rep, 3));
            let mut prefix_rng = Rng(seed(fi, rep, 4));
            for ti in 0..16 {
                let k = target_rng.point(e);
                let (w, v) = k.unwrap();
                targets += 1;
                let positive = distinct_pair(e, k, &mut positive_rng);
                let half = Arithmetic::default().scalar(e, e.n.div_ceil(2), k);
                assert_eq!(reference::point_add(e.a, e.b, half, half).unwrap(), k);
                for variant in 0..4 {
                    let (support, factors) = match variant {
                        0 => (an, [Label::Identity; 2]),
                        1 => (an, [Label::Finite(0); 2]),
                        2 => (
                            [an[1], an[0]],
                            [
                                Label::Finite(factor_rng.field()),
                                Label::Finite(factor_rng.field()),
                            ],
                        ),
                        3 => (
                            [an[0], an[0]],
                            [Label::Finite(factor_rng.field()), Label::Identity],
                        ),
                        _ => unreachable!(),
                    };
                    let frame = Frame::new(e, k, support, factors).unwrap();
                    frames += 1;
                    assert_ne!(frame.transport().determinant(), 0);
                    assert_eq!(frame.transport().apply(factors[0]).unwrap(), factors[1]);
                    let mut slopes: Vec<_> = (0..16).map(|_| Some(slope_rng.field())).collect();
                    slopes.extend([
                        None,
                        Some(0),
                        Some(P - 1),
                        Some(mul(add(mul(3, mul(w, w)), e.a), inv(sub(0, add(v, v))))),
                        ref_slope(frame, 0, Label::Identity),
                        ref_slope(frame, 1, Label::Identity),
                        slope_of(e, positive.0, positive.1),
                        slope_of(e, half, half),
                    ]);
                    assert_eq!(slopes.len(), 24);
                    let mut label_panel = Vec::new();
                    for (kind, slope) in slopes.iter().copied().enumerate() {
                        let left = frame.label_from_slope(0, slope).unwrap();
                        let right = frame.label_from_slope(1, slope).unwrap();
                        assert_eq!(left, ref_at(frame, 0, slope));
                        assert_eq!(right, ref_at(frame, 1, slope));
                        assert_eq!(frame.slope_from_label(0, left).unwrap(), slope);
                        assert_eq!(frame.slope_from_label(1, right).unwrap(), slope);
                        assert_eq!(ref_slope(frame, 0, left), slope);
                        assert_eq!(ref_slope(frame, 1, right), slope);
                        assert_eq!(frame.transport().apply(left).unwrap(), right);
                        assert_eq!(frame.transport().inverse().apply(right).unwrap(), left);
                        let mut d = Diagnostics::default();
                        let got = frame
                            .extract_compatible(left, right, &mut d)
                            .unwrap()
                            .unwrap();
                        let want = reference_pairs(frame, slope, left);
                        assert_eq!(got,want,"complete fiber mismatch fixture={fi} rep={rep} target={ti} variant={variant} kind={kind}");
                        assert_eq!(d.sqrt_calls, usize::from(slope.is_some()));
                        sqrt_calls += d.sqrt_calls;
                        for (p, q) in &got {
                            assert!(
                                reference::on_curve(e.a, e.b, *p)
                                    && reference::on_curve(e.a, e.b, *q)
                            );
                            assert_eq!(reference::point_add(e.a, e.b, *p, *q).unwrap(), k);
                            for j in 0..2 {
                                let a = support[j];
                                let ratio =
                                    reference::chord_ratio(e.a, e.b, *p, *q, a.s, a.delta).unwrap();
                                let actual = reference::extension_mul(ref_label(factors[j]), ratio);
                                assert_eq!(actual, ref_label(if j == 0 { left } else { right }));
                            }
                        }
                        if kind == 22 {
                            assert!(got.contains(&positive));
                            positive_checks += 1;
                        }
                        if kind == 23 {
                            assert!(got.contains(&(half, half)));
                            positive_checks += 1;
                        }
                        let wrong = match right {
                            Label::Identity => Label::Finite(0),
                            Label::Finite(x) => Label::Finite((x + 1) % P),
                        };
                        let mut rejected = Diagnostics::default();
                        assert!(frame
                            .extract_compatible(left, wrong, &mut rejected)
                            .unwrap()
                            .is_none());
                        assert_eq!(rejected.sqrt_calls, 0);
                        incompat += 1;
                        rows += 1;
                        empty += usize::from(got.is_empty());
                        pair_occurrences += got.len();
                        if kind < 16 {
                            random_empty += usize::from(got.is_empty());
                            random_pairs += got.len();
                        } else {
                            controlled_empty += usize::from(got.is_empty());
                            controlled_pairs += got.len();
                        }
                        let sj = slope.map_or("null".to_string(), |x| x.to_string());
                        writeln!(cases,"{{\"fixture\":{fi},\"replicate\":{rep},\"target\":{ti},\"frame\":{variant},\"kind\":{kind},\"K\":[{w},{v}],\"anchors\":[[{},{}],[{},{}]],\"factors\":{},\"slope\":{sj},\"labels\":[{},{}],\"pairs\":{}}}",support[0].s,support[0].delta,support[1].s,support[1].delta,labels_json(&factors),label_number(left),label_number(right),pairs_json(&got)).unwrap();
                        label_panel.push((left, right));
                    }
                    for which in 0..3 {
                        let (mut left, mut right): (Vec<_>, Vec<_>) = match which {
                            0 => (
                                label_panel[..8].iter().map(|p| p.0).collect(),
                                label_panel[4..16].iter().map(|p| p.1).collect(),
                            ),
                            1 => (
                                label_panel[..16].iter().map(|p| p.0).collect(),
                                label_panel[8..16].iter().map(|p| p.1).collect(),
                            ),
                            2 => (vec![label_panel[16].0; 3], Vec::new()),
                            _ => unreachable!(),
                        };
                        if which == 0 {
                            left.push(left[0]);
                            right.push(right[0]);
                        }
                        let li = LabelSet::new(&left).unwrap();
                        let ri = LabelSet::new(&right).unwrap();
                        let (got, stats) = transport::intersect(frame, &li, &ri).unwrap();
                        let mut want = BTreeSet::new();
                        for l in &left {
                            for r in &right {
                                reference_set_pairs += 1;
                                if ref_slope(frame, 0, *l) == ref_slope(frame, 1, *r) {
                                    want.insert((*l, *r));
                                }
                            }
                        }
                        assert_eq!(got, want.into_iter().collect::<Vec<_>>());
                        assert_eq!(stats.visited, li.len().min(ri.len()));
                        assert_eq!(stats.membership_tests, stats.visited);
                        set_queries += 1;
                        set_visits += stats.visited;
                        let matches = format!(
                            "[{}]",
                            got.iter()
                                .map(|(l, r)| format!(
                                    "[{},{}]",
                                    label_number(*l),
                                    label_number(*r)
                                ))
                                .collect::<Vec<_>>()
                                .join(",")
                        );
                        writeln!(intersections,"{{\"fixture\":{fi},\"replicate\":{rep},\"target\":{ti},\"frame\":{variant},\"kind\":{which},\"left\":{},\"right\":{},\"matches\":{matches},\"visited\":{},\"membership_tests\":{}}}",labels_json(&left),labels_json(&right),stats.visited,stats.membership_tests).unwrap();
                    }
                }
                for prefix_kind in 0..2 {
                    let p1 = prefix_rng.point(e);
                    let p2 = if prefix_kind == 0 {
                        prefix_rng.point(e)
                    } else {
                        reference::point_neg(p1)
                    };
                    let a = reference::point_add(e.a, e.b, p1, p2).unwrap();
                    let r = reference::point_add(e.a, e.b, a, k).unwrap();
                    let frame = Frame::from_prefix(e, r, p1, p2, an).unwrap();
                    assert_eq!(Some(frame.residual()), k);
                    let (rk, etas) = reference::prefix_factors(
                        e.a,
                        e.b,
                        r,
                        p1,
                        p2,
                        [(an[0].s, an[0].delta), (an[1].s, an[1].delta)],
                    )
                    .unwrap();
                    assert_eq!(rk, k);
                    for (j, eta) in etas.iter().enumerate() {
                        assert_eq!(ref_label(frame.factors()[j]), *eta);
                    }
                    let points = [p1, p2, positive.0, positive.1];
                    let mut labels = [Label::Identity; 2];
                    for j in 0..2 {
                        let anch = (an[j].s, an[j].delta);
                        let full = reference::four_point_label(e.a, e.b, points, anch).unwrap();
                        labels[j] = as_label(full);
                        assert_eq!(
                            labels[j],
                            frame
                                .label_from_slope(j, slope_of(e, positive.0, positive.1))
                                .unwrap()
                        );
                        for perm in [
                            [points[2], points[0], points[3], points[1]],
                            [points[3], points[2], points[1], points[0]],
                        ] {
                            assert_eq!(
                                reference::four_point_label(e.a, e.b, perm, anch).unwrap(),
                                full
                            );
                        }
                    }
                    let mut d = Diagnostics::default();
                    let got = frame
                        .extract_compatible(labels[0], labels[1], &mut d)
                        .unwrap()
                        .unwrap();
                    assert!(got.contains(&positive));
                    for (p, q) in &got {
                        let k2 = reference::point_add(e.a, e.b, *p, *q).unwrap();
                        assert_eq!(reference::point_add(e.a, e.b, a, k2).unwrap(), r);
                    }
                    prefix_checks += 1;
                    writeln!(prefixes,"{{\"fixture\":{fi},\"replicate\":{rep},\"target\":{ti},\"kind\":{prefix_kind},\"constructed_positive\":true,\"R\":{},\"prefix\":[{},{}],\"K\":[{w},{v}],\"factors\":{},\"labels\":{},\"pairs\":{}}}",point_json(r),point_json(p1),point_json(p2),labels_json(&frame.factors()),labels_json(&labels),pairs_json(&got)).unwrap();
                }
            }
        }
    }
    assert_eq!(targets, 128);
    assert_eq!(frames, 512);
    assert_eq!(rows, 12288);
    assert_eq!(set_queries, 1536);
    assert_eq!(prefix_checks, 256);
    assert_eq!(positive_checks, 1024);
    cases.flush().unwrap();
    intersections.flush().unwrap();
    prefixes.flush().unwrap();
    println!("{{\"study\":\"cayley-residual-transport-m31-v1\",\"field\":{P},\"fixtures\":2,\"targets\":{targets},\"frames\":{frames},\"paired_transport_extraction_cases\":{rows},\"random_slope_cases\":8192,\"controlled_slope_cases\":4096,\"matched_empty_fibers_all_cases\":{empty},\"matched_empty_fibers_random\":{random_empty},\"matched_empty_fibers_controlled\":{controlled_empty},\"ordered_pair_occurrences_all_cases\":{pair_occurrences},\"ordered_pair_occurrences_random\":{random_pairs},\"ordered_pair_occurrences_controlled\":{controlled_pairs},\"coordinate_sqrt_calls\":{sqrt_calls},\"incompatible_labels_rejected_before_sqrt\":{incompat},\"positive_controls_recovered\":{positive_checks},\"set_queries\":{set_queries},\"set_entries_visited\":{set_visits},\"reference_cartesian_pair_checks\":{reference_set_pairs},\"four_point_prefix_cases_constructed\":{prefix_checks},\"invalid_inputs_rejected\":{invalid},\"matrix_sanity_cases\":{sanity},\"distinct_child_seeds\":{},\"scope\":\"local development correctness only\"}}",unique_seeds.len());
}
