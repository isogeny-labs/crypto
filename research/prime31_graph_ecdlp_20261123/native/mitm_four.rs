//! Generic signed four-sum source for a same-curve graph consumer control.
//! Pair enumeration is charged and cannot support an algebraic speed claim.

use prime31_algebraic_ic::{Arithmetic, Point, PRIME31};
use prime31_four_summand::b677_fixture::Hybrid677;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

const Q: Point = Some((976_979_610, 1_228_979_761));
const FB: usize = 509;
const LP: usize = 168;
const TARGET: usize = FB + LP;
const RHS: usize = TARGET + 1;
const WIDTH: usize = RHS + 1;

#[derive(Clone, Copy)]
struct Pair {
    sum: Point,
    i: u16,
    j: u16,
    si: i8,
    sj: i8,
}

fn mod_mul(x: u64, y: u64) -> u64 {
    ((x as u128 * y as u128) % PRIME31.n as u128) as u64
}

fn mod_sub(x: u64, y: u64) -> u64 {
    (x + PRIME31.n - y) % PRIME31.n
}

fn mod_inverse(mut x: u64) -> u64 {
    assert_ne!(x, 0);
    let mut result = 1;
    let mut exponent = PRIME31.n - 2;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = mod_mul(result, x);
        }
        x = mod_mul(x, x);
        exponent >>= 1;
    }
    result
}

struct GraphRank {
    lp_pivots: Vec<Option<Vec<u64>>>,
    fb_pivots: Vec<Option<Vec<u64>>>,
    lp_rank: usize,
    fb_rank: usize,
    projected_rows: usize,
}

impl GraphRank {
    fn new() -> Self {
        Self {
            lp_pivots: vec![None; LP],
            fb_pivots: vec![None; FB + 1],
            lp_rank: 0,
            fb_rank: 0,
            projected_rows: 0,
        }
    }

    fn eliminate(row: &mut [u64], pivot: &[u64], coefficient: u64) {
        for col in 0..WIDTH {
            if pivot[col] != 0 {
                row[col] = mod_sub(row[col], mod_mul(coefficient, pivot[col]));
            }
        }
    }

    fn normalize(row: &mut [u64], coefficient: u64) {
        let inverse = mod_inverse(coefficient);
        for value in row {
            if *value != 0 {
                *value = mod_mul(*value, inverse);
            }
        }
    }

    fn add(&mut self, mut row: Vec<u64>) {
        assert_eq!(row.len(), WIDTH);
        for at in 0..LP {
            let col = FB + at;
            let coefficient = row[col];
            if coefficient == 0 {
                continue;
            }
            if let Some(pivot) = &self.lp_pivots[at] {
                Self::eliminate(&mut row, pivot, coefficient);
            } else {
                Self::normalize(&mut row, coefficient);
                self.lp_pivots[at] = Some(row);
                self.lp_rank += 1;
                return;
            }
        }
        self.projected_rows += 1;
        for col in 0..=TARGET {
            if (FB..TARGET).contains(&col) {
                assert_eq!(row[col], 0);
                continue;
            }
            let coefficient = row[col];
            if coefficient == 0 {
                continue;
            }
            let at = if col == TARGET { FB } else { col };
            if let Some(pivot) = &self.fb_pivots[at] {
                Self::eliminate(&mut row, pivot, coefficient);
            } else {
                Self::normalize(&mut row, coefficient);
                self.fb_pivots[at] = Some(row);
                self.fb_rank += 1;
                return;
            }
        }
        assert_eq!(row[RHS], 0, "replayed relations cannot contradict");
    }

    fn target_scalar(&self) -> Option<u64> {
        self.fb_pivots[FB].as_ref().map(|row| row[RHS])
    }
}

fn next_word(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn dickson1024(mut x: u64) -> u64 {
    for _ in 0..10 {
        x = ((x as u128 * x as u128) % PRIME31.p as u128) as u64;
        x = (x + PRIME31.p - 2) % PRIME31.p;
    }
    x
}

fn main() {
    let cold = Instant::now();
    let fixture = Hybrid677::build().expect("frozen public B677 support");
    let fixture_ms = cold.elapsed().as_millis();
    let points = fixture.points();
    let base: Vec<_> = points
        .iter()
        .map(|(x, _)| dickson1024(*x) == fixture.terminal())
        .collect();
    assert_eq!(base.iter().filter(|&&flag| flag).count(), 509);
    assert_eq!(points.len(), 677);
    let mut ar = Arithmetic::default();
    assert!(ar.on_curve(PRIME31, Q));
    let signed: Vec<[Point; 2]> = points
        .iter()
        .map(|&point| [Some(point), ar.neg(PRIME31, Some(point))])
        .collect();
    let table_start = Instant::now();
    let mut pairs = Vec::with_capacity(4 * points.len() * (points.len() - 1) / 2);
    let mut index: HashMap<Point, Vec<usize>> = HashMap::with_capacity(pairs.capacity());
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            for si in 0..2 {
                for sj in 0..2 {
                    let sum = ar.add(PRIME31, signed[i][si], signed[j][sj]);
                    let at = pairs.len();
                    pairs.push(Pair {
                        sum,
                        i: i as u16,
                        j: j as u16,
                        si: if si == 0 { 1 } else { -1 },
                        sj: if sj == 0 { 1 } else { -1 },
                    });
                    index.entry(sum).or_default().push(at);
                }
            }
        }
    }
    let table_ms = table_start.elapsed().as_millis();
    let mut columns = vec![usize::MAX; points.len()];
    let mut fb_next = 0;
    let mut lp_next = FB;
    for (i, &is_base) in base.iter().enumerate() {
        if is_base {
            columns[i] = fb_next;
            fb_next += 1;
        } else {
            columns[i] = lp_next;
            lp_next += 1;
        }
    }
    assert_eq!((fb_next, lp_next), (FB, TARGET));
    let max_queries = std::env::args()
        .nth(1)
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(40);
    assert!((1..=128).contains(&max_queries));
    let mut seed = 0x7033_315f_6d34_0101u64;
    let mut rank = GraphRank::new();
    let mut query_records = Vec::new();
    let mut all_rows = Vec::new();
    let mut total_probe_additions = 0u64;
    let mut all_classes = [0usize; 5];
    let mut recovered = None;
    for label in 0..max_queries {
        let a = next_word(&mut seed) % PRIME31.n;
        let b = next_word(&mut seed) % PRIME31.n;
        let ag = ar.scalar(PRIME31, a, Some(PRIME31.g));
        let bq = ar.scalar(PRIME31, b, Q);
        let r = ar.add(PRIME31, ag, bq);
        assert!(r.is_some());
        let query_start = Instant::now();
        let mut unique = HashSet::<Vec<(u16, i8)>>::new();
        for left in &pairs {
            let neg_left = ar.neg(PRIME31, left.sum);
            let complement = ar.add(PRIME31, r, neg_left);
            total_probe_additions += 1;
            let Some(matches) = index.get(&complement) else {
                continue;
            };
            for &right_id in matches {
                let right = pairs[right_id];
                if left.i == right.i || left.i == right.j || left.j == right.i || left.j == right.j
                {
                    continue;
                }
                let mut terms = vec![
                    (left.i, left.si),
                    (left.j, left.sj),
                    (right.i, right.si),
                    (right.j, right.sj),
                ];
                terms.sort_unstable_by_key(|term| term.0);
                unique.insert(terms);
            }
        }
        let source_ms = query_start.elapsed().as_millis();
        let mut relations: Vec<_> = unique.into_iter().collect();
        relations.sort();
        let graph_start = Instant::now();
        let mut classes = [0usize; 5];
        for terms in &relations {
            let mut sum = None;
            let mut base_terms = 0;
            let mut row = vec![0u64; WIDTH];
            for &(i, sign) in terms {
                let i = i as usize;
                base_terms += usize::from(base[i]);
                let point = if sign == 1 {
                    signed[i][0]
                } else {
                    signed[i][1]
                };
                sum = ar.add(PRIME31, sum, point);
                row[columns[i]] = if sign == 1 { 1 } else { PRIME31.n - 1 };
            }
            assert_eq!(sum, r, "every emitted equation must replay");
            classes[base_terms] += 1;
            all_rows.push(json!({"label": label, "terms": terms}));
            if base_terms >= 2 {
                row[TARGET] = mod_sub(0, b);
                row[RHS] = a;
                rank.add(row);
            }
        }
        let graph_ms = graph_start.elapsed().as_millis();
        for k in 0..5 {
            all_classes[k] += classes[k];
        }
        query_records.push(json!({
            "label": label,
            "a": a,
            "b": b,
            "query": r,
            "classes_by_base_terms": classes,
            "source_ms": source_ms,
            "graph_and_replay_ms": graph_ms,
            "lp_rank": rank.lp_rank,
            "projected_rows": rank.projected_rows,
            "factor_base_target_rank": rank.fb_rank,
        }));
        eprintln!(
            "query {label}: relations={} lp_rank={} projected={} fb_rank={} source_ms={source_ms}",
            relations.len(),
            rank.lp_rank,
            rank.projected_rows,
            rank.fb_rank
        );
        if let Some(d) = rank.target_scalar() {
            assert_eq!(ar.scalar(PRIME31, d, Some(PRIME31.g)), Q);
            recovered = Some(d);
            break;
        }
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "scope": "single-target generic four-sum graph-control; pair enumeration fully charged",
            "curve": PRIME31,
            "public_target": Q,
            "seed": "0x7033315f6d340101",
            "fixture_certificate": fixture.certificate(),
            "base_size": FB,
            "lp_pool_size": LP,
            "pair_entries": pairs.len(),
            "distinct_pair_sums": index.len(),
            "probe_additions": total_probe_additions,
            "classes_by_base_terms": all_classes,
            "queries": query_records,
            "relation_rows": all_rows,
            "lp_rank": rank.lp_rank,
            "projected_rows": rank.projected_rows,
            "factor_base_target_rank": rank.fb_rank,
            "recovered_scalar": recovered,
            "recovered_scalar_replay": recovered.is_some(),
            "fixture_ms": fixture_ms,
            "fixture_work": fixture.counts(),
            "table_ms": table_ms,
            "cold_wall_ms": cold.elapsed().as_millis(),
            "charged_group_work": ar.counts,
            "total_group_additions": fixture.counts().group_additions + ar.counts.group_additions,
        }))
        .unwrap()
    );
}
