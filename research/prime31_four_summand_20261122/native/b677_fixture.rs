//! The exact public Run66 hybrid base: Dickson B509 plus eight Y7 cosets.
//!
//! This descriptor contains public construction data only. It carries neither
//! target scalars nor a decomposition witness. All setup and certificate field
//! products below are included in the returned original arithmetic counters.

use prime31_algebraic_ic::{check_curve, Arithmetic, Base, Counts, PRIME31};
use serde::Serialize;
use std::collections::BTreeSet;

const EXTRA_CSV: &str = include_str!("../../prime31_f4_admission_20261121/RUN66_Y7_EXTRA168.csv");

/// SHA256 of the unchanged archived CSV bytes, recorded before this campaign.
pub const Y7_CSV_SHA256: &str = "fc484615394ee423190747d05001cea526580fb0df0279fbcf789ab6cafc0c46";

// Public (y0^7, y0) constants. Independent native scalar checks establish the
// final c=1852957275, matching the earlier xlift source; the later cubic-factor
// spelling 1852955275 is false. Historical files and first-build inputs remain
// unchanged; see scaling-b677/SOURCE_CORRECTION.md. Derive every c below.
const COSETS: [(u64, u64); 8] = [
    (1_970_273_222, 391_601_180),
    (1_011_118_577, 1_045_445_284),
    (784_868_900, 1_088_714_879),
    (1_902_095_001, 839_007_678),
    (131_394_216, 1_893_426_316),
    (49_444_078, 1_215_727_561),
    (53_346_144, 1_554_347_565),
    (1_852_957_275, 136_769_428),
];

#[derive(Clone, Debug, Serialize)]
pub struct FixtureCertificate {
    pub dickson_depth: usize,
    pub dickson_x_roots: usize,
    pub dickson_curve_points: usize,
    pub dickson_off_curve_x_roots: usize,
    pub extra_points: usize,
    pub hybrid_points: usize,
    pub selector_degree: usize,
    pub y7_roots_per_coset: [usize; 8],
    pub y7_coefficient_factorizations: usize,
    pub horizontal_triples_replayed: usize,
    pub y7_csv_sha256: &'static str,
}

#[derive(Clone, Debug)]
pub struct Hybrid677 {
    points: Vec<(u64, u64)>,
    terminal: u64,
    coset_squares: [u64; 8],
    counts: Counts,
    certificate: FixtureCertificate,
}

// The original Arithmetic field helpers are private. These small certificate
// helpers use the same u128 modular product and explicitly charge its existing
// counter; curve operations and Dickson acquisition use the original backend.
fn product(ar: &mut Arithmetic, a: u64, b: u64) -> u64 {
    ar.counts.fp_multiplications += 1;
    ((a as u128 * b as u128) % PRIME31.p as u128) as u64
}

fn power(ar: &mut Arithmetic, mut a: u64, mut exponent: u64) -> u64 {
    let mut result = 1;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = product(ar, result, a);
        }
        a = product(ar, a, a);
        exponent >>= 1;
    }
    result
}

fn polynomial_product(ar: &mut Arithmetic, a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut result = vec![0; a.len() + b.len() - 1];
    for (i, &left) in a.iter().enumerate() {
        for (j, &right) in b.iter().enumerate() {
            result[i + j] = (result[i + j] + product(ar, left, right)) % PRIME31.p;
        }
    }
    result
}

fn dickson_value(ar: &mut Arithmetic, mut x: u64) -> u64 {
    for _ in 0..10 {
        x = (product(ar, x, x) + PRIME31.p - 2) % PRIME31.p;
    }
    x
}

impl Hybrid677 {
    pub fn build() -> Result<Self, String> {
        let e = PRIME31;
        if (e.p, e.a, e.b, e.n, e.g)
            != (
                2_147_483_647,
                847_514_846,
                2_087_444_724,
                2_147_446_831,
                (625_037_715, 2_100_627_251),
            )
        {
            return Err("the original Prime31 curve has changed".into());
        }
        let mut ar = Arithmetic::default();
        check_curve(e, &mut ar)?;
        let dickson = Base::build(e, 10, &mut ar)?;
        if dickson.values.len() != 1024 || dickson.points.len() != 509 {
            return Err("the original depth10 Dickson B509 fixture has changed".into());
        }
        if dickson.terminal == 2 || dickson.terminal == e.p - 2 {
            return Err("exceptional Dickson terminal".into());
        }
        if !dickson.values.windows(2).all(|pair| pair[0] < pair[1]) {
            return Err("Dickson roots are not sorted and distinct".into());
        }
        let mut certified_curve_points = Vec::new();
        for &x in &dickson.values {
            if x >= e.p || dickson_value(&mut ar, x) != dickson.terminal {
                return Err("a Dickson root fails the public degree1024 selector".into());
            }
            let rhs = ar.rhs(e, x);
            if let Some(y) = ar.sqrt(rhs, e.p) {
                if y == 0 {
                    return Err("zero-y Dickson point on the prime-order fixture".into());
                }
                certified_curve_points.push((x, y.min(e.p - y)));
            }
        }
        if certified_curve_points != dickson.points {
            return Err("Dickson curve filtering disagrees with original Base::build".into());
        }

        let mut extras = Vec::new();
        for (line_number, line) in EXTRA_CSV.lines().enumerate() {
            if line.is_empty() {
                continue;
            }
            let (xs, ys) = line
                .split_once(',')
                .ok_or_else(|| format!("Y7 CSV line {} has no comma", line_number + 1))?;
            let x = xs.parse::<u64>().map_err(|err| err.to_string())?;
            let y = ys.parse::<u64>().map_err(|err| err.to_string())?;
            if y == 0 || !ar.on_curve(e, Some((x, y))) {
                return Err(format!("invalid Y7 point at CSV line {}", line_number + 1));
            }
            if dickson_value(&mut ar, x) == dickson.terminal {
                return Err("Y7 roots overlap the full Dickson x fiber".into());
            }
            extras.push((x, y.min(e.p - y)));
        }
        if extras.len() != 168 || !extras.windows(2).all(|pair| pair[0].0 < pair[1].0) {
            return Err("the frozen Y7 CSV must contain 168 sorted distinct x roots".into());
        }

        let mut cosets = [0; 8];
        let mut squares = [0; 8];
        let mut inverses = [0; 8];
        let mut fibers = [[0; 7]; 8];
        let zeta = power(&mut ar, 3, (e.p - 1) / 7);
        if zeta != 1_752_599_774 || zeta == 1 || power(&mut ar, zeta, 7) != 1 {
            return Err("the public seventh root of unity changed".into());
        }
        for (i, &(expected_c, y0)) in COSETS.iter().enumerate() {
            cosets[i] = power(&mut ar, y0, 7);
            if cosets[i] != expected_c || cosets[i] == 0 {
                return Err(format!(
                    "public Y7 coset {i}: y0^7 differs from its frozen constant"
                ));
            }
            squares[i] = product(&mut ar, cosets[i], cosets[i]);
            ar.counts.fp_inversions += 1;
            inverses[i] = power(&mut ar, cosets[i], e.p - 2);
            let mut y = y0;
            for slot in &mut fibers[i] {
                *slot = y;
                y = product(&mut ar, y, zeta);
            }
            if y != y0 {
                return Err("Y7 fiber cycle did not close".into());
            }
        }
        if squares.iter().copied().collect::<BTreeSet<_>>().len() != 8 {
            return Err("public Y7 cosets overlap up to sign".into());
        }

        let mut roots: [Vec<u64>; 8] = std::array::from_fn(|_| Vec::new());
        let mut triples: [[Vec<(u64, u64)>; 7]; 8] =
            std::array::from_fn(|_| std::array::from_fn(|_| Vec::new()));
        for &(x, canonical_y) in &extras {
            let rhs = ar.rhs(e, x);
            let seventh = power(&mut ar, rhs, 7);
            let matches: Vec<_> = squares
                .iter()
                .enumerate()
                .filter_map(|(i, &square)| (seventh == square).then_some(i))
                .collect();
            if matches.len() != 1 {
                return Err("an archived Y7 x root has no unique public coset".into());
            }
            let coset = matches[0];
            let fourth = power(&mut ar, rhs, 4);
            let signed_y = product(&mut ar, fourth, inverses[coset]);
            if product(&mut ar, signed_y, signed_y) != rhs
                || power(&mut ar, signed_y, 7) != cosets[coset]
                || signed_y.min(e.p - signed_y) != canonical_y
            {
                return Err("Y7 algebraic signed lift differs from the archived point".into());
            }
            let fiber = fibers[coset]
                .iter()
                .position(|&y| y == signed_y)
                .ok_or("Y7 signed lift lies outside its seven public fibers")?;
            roots[coset].push(x);
            triples[coset][fiber].push((x, signed_y));
        }

        let mut factorizations = 0;
        let mut horizontal_triples = 0;
        for i in 0..8 {
            if roots[i].len() != 21 {
                return Err(format!("Y7 coset {i} does not have 21 frozen roots"));
            }
            let mut expected = vec![1];
            for _ in 0..7 {
                expected = polynomial_product(&mut ar, &expected, &[e.b, e.a, 0, 1]);
            }
            expected[0] = (expected[0] + e.p - squares[i]) % e.p;
            let mut factored = vec![1];
            for &x in &roots[i] {
                factored = polynomial_product(&mut ar, &factored, &[(e.p - x) % e.p, 1]);
            }
            if expected != factored {
                return Err(format!(
                    "Y7 coset {i} fails the complete coefficient factorization"
                ));
            }
            factorizations += 1;
            for triple in &triples[i] {
                if triple.len() != 3 {
                    return Err("a registered Y7 cubic does not split into three roots".into());
                }
                let mut sum = None;
                for &point in triple {
                    sum = ar.add(e, sum, Some(point));
                }
                if sum.is_some() || triple.iter().map(|point| point.0).sum::<u64>() % e.p != 0 {
                    return Err("a Y7 horizontal triple fails its public group identity".into());
                }
                horizontal_triples += 1;
            }
        }

        let mut points = dickson.points;
        points.extend(extras);
        points.sort_unstable();
        if points.len() != 677 || !points.windows(2).all(|pair| pair[0].0 < pair[1].0) {
            return Err("the canonical hybrid base is not exactly 677 distinct x points".into());
        }
        if !points
            .iter()
            .all(|&(x, y)| y != 0 && y <= e.p - y && ar.on_curve(e, Some((x, y))))
        {
            return Err("canonical hybrid point validation failed".into());
        }
        Ok(Self {
            points,
            terminal: dickson.terminal,
            coset_squares: squares,
            counts: ar.counts,
            certificate: FixtureCertificate {
                dickson_depth: 10,
                dickson_x_roots: 1024,
                dickson_curve_points: 509,
                dickson_off_curve_x_roots: 515,
                extra_points: 168,
                hybrid_points: 677,
                selector_degree: 1192,
                y7_roots_per_coset: roots.each_ref().map(Vec::len),
                y7_coefficient_factorizations: factorizations,
                horizontal_triples_replayed: horizontal_triples,
                y7_csv_sha256: Y7_CSV_SHA256,
            },
        })
    }

    pub fn points(&self) -> &[(u64, u64)] {
        &self.points
    }

    pub fn terminal(&self) -> u64 {
        self.terminal
    }

    pub fn coset_squares(&self) -> &[u64; 8] {
        &self.coset_squares
    }

    pub fn counts(&self) -> Counts {
        self.counts
    }

    pub fn certificate(&self) -> &FixtureCertificate {
        &self.certificate
    }
}
