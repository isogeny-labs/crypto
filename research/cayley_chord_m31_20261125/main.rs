//! Public/synthetic development correctness; no performance measurement.
mod extractor;
mod reference;
#[allow(dead_code)]
#[path = "upstream/arithmetic.rs"]
mod upstream;
use extractor::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{BufWriter, Write};
use upstream::{Arithmetic, Curve};

const MASTER: u64 = 0x4341594c45593231;
const REPLICATES: [u64; 4] = [
    0x5245504c00000001,
    0x5245504c00000002,
    0x5245504c00000003,
    0x5245504c00000004,
];
const PRIMARY: Curve = Curve {
    p: P,
    a: P - 3,
    b: 21,
    n: 2_147_487_637,
    g: (1_391_443_431, 177_979_044),
};

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d <= n / d {
        if n % d == 0 {
            return n == d;
        }
        d += 1;
    }
    true
}
fn validate_curve(e: Curve) {
    assert_eq!(e.p, P);
    assert!(is_prime(P) && is_prime(e.n));
    assert_ne!(
        add(mul(4, mul(mul(e.a, e.a), e.a)), mul(27, mul(e.b, e.b))),
        0
    );
    let mut ar = Arithmetic::default();
    assert!(ar.on_curve(e, Some(e.g)));
    assert_eq!(ar.scalar(e, e.n, Some(e.g)), None);
    assert_eq!(ar.scalar(e, e.n - 1, Some(e.g)), ar.neg(e, Some(e.g)));
    assert!(46_341u64 * 46_341 >= P);
    assert!(2 * e.n > P + 1 + 2 * 46_341);
}
fn mix64(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}
struct SplitMix64(u64);
impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        mix64(self.0)
    }
}

#[derive(Default)]
struct Totals {
    cases: usize,
    natural: usize,
    controls: usize,
    empty: usize,
    pairs: usize,
    withheld: usize,
    max_pairs: usize,
}

#[allow(clippy::too_many_arguments)]
fn check_case(
    e: Curve,
    fi: usize,
    ri: usize,
    ti: usize,
    kind: &str,
    r: Affine,
    selector: Selector,
    withheld: Option<Pair>,
    d: &mut Diagnostics,
    t: &mut Totals,
    out: &mut impl Write,
) {
    let actual = extract(e, Some(r), selector, d).expect("valid public input");
    let lambda = match selector {
        Selector::Identity => None,
        Selector::Lambda(l) => Some(l),
    };
    let expected = reference::extract(e.a, e.b, r.0, r.1, lambda);
    assert_eq!(actual,expected,"mismatch fixture={fi} replicate={ri} target={ti} kind={kind} R={r:?} selector={selector:?}");
    assert!(actual.len() <= 4);
    let mut ar = Arithmetic::default();
    for &(p1, p2) in &actual {
        assert!(ar.on_curve(e, Some(p1)) && ar.on_curve(e, Some(p2)));
        assert_eq!(ar.add(e, Some(p1), Some(p2)), Some(r));
        assert_eq!(z_mul(cayley(p1.0), cayley(p2.0)), selector_z(selector));
    }
    if let Some(pair) = withheld {
        assert!(actual.contains(&pair), "known-positive witness missing");
        t.withheld += 1;
    }
    t.cases += 1;
    if kind == "unplanted_lambda" {
        t.natural += 1;
    } else {
        t.controls += 1;
    }
    t.empty += usize::from(actual.is_empty());
    t.pairs += actual.len();
    t.max_pairs = t.max_pairs.max(actual.len());
    let sj = lambda
        .map(|x| x.to_string())
        .unwrap_or_else(|| "null".into());
    let pairs = actual
        .iter()
        .map(|((x1, y1), (x2, y2))| format!("[[{x1},{y1}],[{x2},{y2}]]"))
        .collect::<Vec<_>>()
        .join(",");
    writeln!(out,"{{\"fixture\":{fi},\"replicate\":{ri},\"target_index\":{ti},\"kind\":\"{kind}\",\"R\":[{},{}],\"lambda\":{sj},\"pairs\":[{pairs}]}}",r.0,r.1).unwrap();
}

fn count_identity(e: Curve) -> (usize, usize, usize) {
    let mut ar = Arithmetic::default();
    let positives = (1..=32)
        .map(|k| ar.scalar(e, k, Some(e.g)).unwrap())
        .collect::<Vec<_>>();
    let mut base = positives.clone();
    base.extend(positives.iter().map(|&p| ar.neg(e, Some(p)).unwrap()));
    assert_eq!(base.iter().copied().collect::<BTreeSet<_>>().len(), 64);
    assert_eq!(
        positives.iter().map(|p| p.0).collect::<BTreeSet<_>>().len(),
        32
    );
    let mut ordered_x: BTreeMap<Selector, usize> = BTreeMap::new();
    for &a in &positives {
        for &b in &positives {
            *ordered_x.entry(selector_for(a, b)).or_default() += 1;
        }
    }
    let mut unordered_signed: BTreeMap<Selector, usize> = BTreeMap::new();
    let mut ordered_nonzero = 0;
    for (i, &a) in base.iter().enumerate() {
        for (j, &b) in base.iter().enumerate() {
            if ar.add(e, Some(a), Some(b)).is_some() {
                ordered_nonzero += 1;
                if i <= j {
                    *unordered_signed.entry(selector_for(a, b)).or_default() += 1;
                }
            }
        }
    }
    assert_eq!(ordered_nonzero, 4032);
    assert_eq!(ordered_x.len(), unordered_signed.len());
    for (&label, &mc) in &ordered_x {
        assert_eq!(unordered_signed[&label], 2 * mc);
        assert!(mc <= positives.len());
    }
    assert_eq!(ordered_x.values().sum::<usize>(), 1024);
    let count = unordered_signed.values().sum::<usize>();
    assert_eq!(count, 2048);
    (
        ordered_x.len(),
        count,
        *unordered_signed.values().max().unwrap(),
    )
}

fn main() {
    let output = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "evidence/cases.jsonl".into());
    let mut out = BufWriter::new(File::create(output).unwrap());
    let mut totals = Totals::default();
    let mut diag = Diagnostics::default();
    let mut seeds = Vec::new();
    let mut distinct_seeds = BTreeSet::new();
    let mut count_receipts = Vec::new();
    for (fi, e) in [PRIMARY, upstream::PRIME31].into_iter().enumerate() {
        validate_curve(e);
        let mut ar = Arithmetic::default();
        for (ri, &rep) in REPLICATES.iter().enumerate() {
            let tag = 0x4355525600000001u64 + fi as u64;
            let keyed = mix64(mix64(MASTER ^ tag) ^ rep);
            let st = mix64(keyed ^ 0x5441524745540001);
            let sl = mix64(keyed ^ 0x4c414d4244410001);
            let sw = mix64(keyed ^ 0x5749544e45535301);
            for seed in [st, sl, sw] {
                assert!(distinct_seeds.insert(seed), "child seed reused");
            }
            seeds.push(format!("{{\"fixture\":{fi},\"replicate\":{ri},\"target_seed\":\"0x{st:016x}\",\"lambda_seed\":\"0x{sl:016x}\",\"witness_seed\":\"0x{sw:016x}\"}}"));
            let (mut tr, mut lr, mut wr) = (SplitMix64(st), SplitMix64(sl), SplitMix64(sw));
            for ti in 0..16 {
                let k = 1 + tr.next() % (e.n - 1);
                let r = ar.scalar(e, k, Some(e.g)).unwrap();
                for _ in 0..16 {
                    check_case(
                        e,
                        fi,
                        ri,
                        ti,
                        "unplanted_lambda",
                        r,
                        Selector::Lambda(lr.next() % P),
                        None,
                        &mut diag,
                        &mut totals,
                        &mut out,
                    );
                }
                for (name, sel) in [
                    ("identity_selector", Selector::Identity),
                    ("linear_slope", Selector::Lambda(sub(0, r.0))),
                    ("zero_lambda", Selector::Lambda(0)),
                ] {
                    check_case(
                        e,
                        fi,
                        ri,
                        ti,
                        name,
                        r,
                        sel,
                        None,
                        &mut diag,
                        &mut totals,
                        &mut out,
                    );
                }
                let (p1, p2) = loop {
                    let j = 1 + wr.next() % (e.n - 1);
                    let first = ar.scalar(e, j, Some(e.g)).unwrap();
                    if let Some(second) = ar.add(e, Some(r), ar.neg(e, Some(first))) {
                        if first != second {
                            break (first, second);
                        }
                    }
                };
                check_case(
                    e,
                    fi,
                    ri,
                    ti,
                    "known_pair",
                    r,
                    selector_for(p1, p2),
                    Some((p1, p2)),
                    &mut diag,
                    &mut totals,
                    &mut out,
                );
                let half = ar.scalar(e, (e.n + 1) / 2, Some(r)).unwrap();
                check_case(
                    e,
                    fi,
                    ri,
                    ti,
                    "doubling",
                    r,
                    selector_for(half, half),
                    Some((half, half)),
                    &mut diag,
                    &mut totals,
                    &mut out,
                );
                let nr = ar.neg(e, Some(r)).unwrap();
                let rr = ar.add(e, Some(r), Some(r)).unwrap();
                check_case(
                    e,
                    fi,
                    ri,
                    ti,
                    "tangent_at_minus_R",
                    r,
                    selector_for(nr, rr),
                    Some((nr, rr)),
                    &mut diag,
                    &mut totals,
                    &mut out,
                );
            }
        }
        let mut found = false;
        for k in 1..=64 {
            let r = ar.scalar(e, k, Some(e.g)).unwrap();
            let ls = roots(
                r.0,
                sub(1, e.a),
                add(r.0, e.b),
                &mut Diagnostics::default(),
                false,
            );
            if let Some(&l) = ls.first() {
                check_case(
                    e,
                    fi,
                    4,
                    k as usize,
                    "zero_first_discriminant",
                    r,
                    Selector::Lambda(l),
                    None,
                    &mut diag,
                    &mut totals,
                    &mut out,
                );
                found = true;
                break;
            }
        }
        assert!(found, "fixed repeated-root control not found");
        assert!(extract(e, None, Selector::Identity, &mut Diagnostics::default()).is_err());
        assert!(extract(
            e,
            Some(e.g),
            Selector::Lambda(P),
            &mut Diagnostics::default()
        )
        .is_err());
        assert!(extract(
            e,
            Some((P, e.g.1)),
            Selector::Identity,
            &mut Diagnostics::default()
        )
        .is_err());
        assert!(extract(
            e,
            Some((e.g.0, 0)),
            Selector::Identity,
            &mut Diagnostics::default()
        )
        .is_err());
        let (labels, count, max) = count_identity(e);
        count_receipts.push(format!("{{\"fixture\":{fi},\"signed_base_size\":64,\"x_base_size\":32,\"labels\":{labels},\"unordered_nonidentity_pairs\":{count},\"max_label_pairs\":{max},\"ordered_nonidentity_pairs\":4032,\"identity_2mc_passed\":true}}"));
    }
    out.flush().unwrap();
    assert_eq!(distinct_seeds.len(), 24);
    assert_eq!(totals.cases, 2818);
    assert_eq!(totals.natural, 2048);
    assert_eq!(totals.withheld, 384);
    assert!(
        diag.first_linear >= 128
            && diag.zero_slope_discriminant >= 2
            && diag.zero_coordinate_discriminant >= 128
    );
    println!("{{\"study\":\"cayley-chord-m31-correctness-v1\",\"scope\":\"development correctness; no timing or natural factor-base yield\",\"curves\":2,\"main_targets\":128,\"paired_cases\":{},\"unplanted_selector_cases\":{},\"control_cases\":{},\"empty_cases\":{},\"returned_ordered_pairs\":{},\"known_positive_pairs_recovered\":{},\"max_pairs\":{},\"reference_matches\":{},\"first_quadratic_cases\":{},\"first_linear_cases\":{},\"identity_selector_cases\":{},\"zero_first_discriminants\":{},\"zero_coordinate_discriminants\":{},\"candidate_sqrt_calls\":{},\"invalid_input_rejections\":8,\"double_count_receipts\":[{}],\"seed_records\":[{}]}}",totals.cases,totals.natural,totals.controls,totals.empty,totals.pairs,totals.withheld,totals.max_pairs,totals.cases,diag.first_quadratic,diag.first_linear,diag.identity_selector,diag.zero_slope_discriminant,diag.zero_coordinate_discriminant,diag.sqrt_calls,count_receipts.join(","),seeds.join(","));
}
