//! Useful rank per second on the frozen B677 M31 workload: the library's new
//! signed pair-table source (`pair_table_relations`) against the library's
//! default Semaev projection oracle (`PairProjection::build` +
//! `decompose_four`), both feeding one consumer (dedup, exact replay,
//! large-prime-then-factor-base elimination with Barrett reduction and
//! sparse pivots) until the target logarithm is recovered and checked by
//! d*G = Q. Queries R = aG + bQ follow the B677 seed of crypto #5097/#5184.
//!
//! Usage: pair_table_rank_rate pairtable <k/256 window> <salt> <threads>
//!        pair_table_rank_rate default

use crypto_lib::cryptanalysis::pair_table_relations::*;
use crypto_lib::cryptanalysis::pkm_smooth_tower::Curve;
use crypto_lib::cryptanalysis::semaev_projection::PairProjection;
use serde_json::json;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::time::Instant;

const N: u64 = 2_147_446_831;
const A: i64 = 847_514_846;
const G: (u64, u64) = (625_037_715, 2_100_627_251);
const Q: (u64, u64) = (976_979_610, 1_228_979_761);
const FB: usize = 509;
const LP: usize = 168;
const TARGET: usize = FB + LP;
const RHS: usize = TARGET + 1;
const WIDTH: usize = RHS + 1;

/// Barrett reduction modulo the group order N < 2^32.
const BARRETT: u64 = (u64::MAX / N) as u64;
#[inline(always)]
fn mm(x: u64, y: u64) -> u64 {
    let z = x * y;
    let q = ((z as u128 * BARRETT as u128) >> 64) as u64;
    let mut r = z - q * N;
    while r >= N {
        r -= N;
    }
    r
}
fn ms(x: u64, y: u64) -> u64 {
    (x + N - y) % N
}
fn minv(mut x: u64) -> u64 {
    let (mut r, mut e) = (1, N - 2);
    while e != 0 {
        if e & 1 != 0 {
            r = mm(r, x);
        }
        x = mm(x, x);
        e >>= 1;
    }
    r
}
type Pivot = (Vec<u64>, Vec<u16>);
struct GraphRank {
    lp: Vec<Option<Pivot>>,
    fb: Vec<Option<Pivot>>,
    lp_rank: usize,
    fb_rank: usize,
}
impl GraphRank {
    fn new() -> Self {
        Self {
            lp: vec![None; LP],
            fb: vec![None; FB + 1],
            lp_rank: 0,
            fb_rank: 0,
        }
    }
    fn elim(row: &mut [u64], piv: &Pivot, c: u64) {
        for &k in &piv.1 {
            let k = k as usize;
            row[k] = ms(row[k], mm(c, piv.0[k]));
        }
    }
    fn pivot(row: Vec<u64>) -> Pivot {
        let nz = (0..WIDTH)
            .filter(|&k| row[k] != 0)
            .map(|k| k as u16)
            .collect();
        (row, nz)
    }
    fn norm(row: &mut [u64], c: u64) {
        let i = minv(c);
        for v in row.iter_mut() {
            if *v != 0 {
                *v = mm(*v, i);
            }
        }
    }
    fn add(&mut self, mut row: Vec<u64>) -> bool {
        for at in 0..LP {
            let c = row[FB + at];
            if c == 0 {
                continue;
            }
            if let Some(p) = &self.lp[at] {
                Self::elim(&mut row, p, c);
            } else {
                Self::norm(&mut row, c);
                self.lp[at] = Some(Self::pivot(row));
                self.lp_rank += 1;
                return true;
            }
        }
        for col in 0..=TARGET {
            if (FB..TARGET).contains(&col) {
                continue;
            }
            let c = row[col];
            if c == 0 {
                continue;
            }
            let at = if col == TARGET { FB } else { col };
            if let Some(p) = &self.fb[at] {
                Self::elim(&mut row, p, c);
            } else {
                Self::norm(&mut row, c);
                self.fb[at] = Some(Self::pivot(row));
                self.fb_rank += 1;
                return true;
            }
        }
        assert_eq!(row[RHS], 0);
        false
    }
}
fn next_word(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}
fn mix(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn main() {
    let mut args = std::env::args().skip(1);
    let arm = args.next().unwrap_or_else(|| "pairtable".into());
    assert!(
        arm == "pairtable" || arm == "default",
        "arm must be pairtable or default"
    );
    let k_frac: usize = args.next().and_then(|v| v.parse().ok()).unwrap_or(8);
    let salt: u64 = args.next().and_then(|v| v.parse().ok()).unwrap_or(1);
    let threads: usize = args.next().and_then(|v| v.parse().ok()).unwrap_or(1);
    // Admission rule: "fb2" (default) keeps only relations with >= 2 factor-base terms,
    // the crypto #5097 consumer rule; ADMIT=all also admits higher large-prime arity.
    let admit_all = std::env::var("ADMIT").as_deref() == Ok("all");
    let f = Mersenne31;
    let p = f.p();
    let cold = Instant::now();
    let csv = include_str!("../research/semaev_fast_path_rank_20261125/b677_points.csv");
    let mut pts = Vec::new();
    let mut is_fb = Vec::new();
    for line in csv.lines() {
        let v: Vec<&str> = line.split(',').collect();
        pts.push((v[0].parse::<u64>().unwrap(), v[1].parse::<u64>().unwrap()));
        is_fb.push(v[2] == "fb");
    }
    let mut column = vec![0usize; pts.len()];
    let (mut nf, mut nl) = (0, FB);
    for i in 0..pts.len() {
        if is_fb[i] {
            column[i] = nf;
            nf += 1;
        } else {
            column[i] = nl;
            nl += 1;
        }
    }
    let t_build = Instant::now();
    let proj_curve = Curve {
        p,
        a: A as u64,
        b: 2_087_444_724,
    };
    let (table, proj) = if arm == "pairtable" {
        (Some(PairTable::build_threads(f, &pts, threads)), None)
    } else {
        let xs: Vec<u64> = pts.iter().map(|q| q.0).collect();
        (
            None,
            Some(PairProjection::build(proj_curve, &xs).expect("projection build")),
        )
    };
    let build_ms = t_build.elapsed().as_secs_f64() * 1e3;
    let x_index: std::collections::HashMap<u64, u32> = pts
        .iter()
        .enumerate()
        .map(|(i, q)| (q.0, i as u32))
        .collect();
    let npairs = table.as_ref().map_or(1, |t| t.pairs());
    let threads = if arm == "default" { 1 } else { threads };
    let window = npairs * k_frac / 256;
    let mut seed = 0x7033_315f_6d34_0101u64;
    let mut rank = GraphRank::new();
    let mut seen: HashSet<SignedFourSum> = HashSet::new();
    let mut st = PairTableStats::default();
    let (mut queries, mut useful, mut probe_ms, mut elim_ms) = (0u64, 0u64, 0.0, 0.0);
    let mut recovered = None;
    let mut done = false;
    let mut replay_ms = 0.0;
    let mut digest = std::collections::hash_map::DefaultHasher::new();
    while queries < 4096 && !done {
        // Draw a batch of `threads` queries in sequence order.
        let mut qs = Vec::with_capacity(threads);
        for _ in 0..threads {
            let a = next_word(&mut seed) % N;
            let b = next_word(&mut seed) % N;
            let ag = affine_mul(&f, A, a, Some(G));
            let bq = affine_mul(&f, A, b, Some(Q));
            let r = affine_add(&f, A, ag, bq).unwrap();
            queries += 1;
            let start = (mix(salt.wrapping_mul(0x1000_0000_01b3) ^ queries) as usize) % npairs;
            qs.push((a, b, r, start));
        }
        let t0 = Instant::now();
        let found: Vec<(Vec<SignedFourSum>, PairTableStats)> = if let Some(proj) = &proj {
            // Library default oracle: one x-only witness, lifted to every exact sign pattern.
            let (_, _, r, _) = qs[0];
            let mut out = Vec::new();
            if let Some(w) = proj.decompose_four(r.0).0 {
                let idx: Vec<u32> = w.x.iter().map(|x| x_index[x]).collect();
                for mask in 0..16u32 {
                    let terms: Vec<(u32, i8)> = (0..4)
                        .map(|k| (idx[k], if mask >> k & 1 == 1 { -1 } else { 1 }))
                        .collect();
                    let mut s = None;
                    for &(i, sg) in &terms {
                        let pt = Some(pts[i as usize]);
                        s = affine_add(&f, A, s, if sg > 0 { pt } else { affine_neg(p, pt) });
                    }
                    let distinct = idx.iter().collect::<std::collections::HashSet<_>>().len() == 4;
                    if s == Some(r) && distinct {
                        let mut t = [terms[0], terms[1], terms[2], terms[3]];
                        t.sort_unstable_by_key(|x| x.0);
                        out.push(SignedFourSum { terms: t });
                    }
                }
            }
            vec![(out, PairTableStats::default())]
        } else if threads == 1 {
            let table = table.as_ref().unwrap();
            let (_, _, r, start) = qs[0];
            let mut s1 = PairTableStats::default();
            let mut v = Vec::new();
            table.probe_window(r, start, window, &mut s1, |rel| v.push(rel));
            vec![(v, s1)]
        } else {
            let table = table.as_ref().unwrap();
            std::thread::scope(|sc| {
                let hs: Vec<_> = qs
                    .iter()
                    .map(|&(_, _, r, start)| {
                        sc.spawn(move || {
                            let mut s1 = PairTableStats::default();
                            let mut v = Vec::new();
                            table.probe_window(r, start, window, &mut s1, |rel| v.push(rel));
                            (v, s1)
                        })
                    })
                    .collect();
                hs.into_iter().map(|h| h.join().unwrap()).collect()
            })
        };
        probe_ms += t0.elapsed().as_secs_f64() * 1e3;
        let t1 = Instant::now();
        for ((a, b, r, _), (rels, s1)) in qs.into_iter().zip(found) {
            st.probes += s1.probes;
            st.hits += s1.hits;
            if done {
                continue; // charged above, not consumed
            }
            for rel in rels {
                if !seen.insert(rel) {
                    continue;
                }
                (a, b, rel).hash(&mut digest);
                let tr = Instant::now();
                let mut s = None;
                for &(i, sg) in &rel.terms {
                    let pt = Some(pts[i as usize]);
                    s = affine_add(&f, A, s, if sg > 0 { pt } else { affine_neg(p, pt) });
                }
                assert_eq!(s, Some(r), "relation must replay");
                replay_ms += tr.elapsed().as_secs_f64() * 1e3;
                if !admit_all && rel.terms.iter().filter(|t| is_fb[t.0 as usize]).count() < 2 {
                    continue;
                }
                let mut row = vec![0u64; WIDTH];
                for &(i, sg) in &rel.terms {
                    row[column[i as usize]] = if sg > 0 { 1 } else { N - 1 };
                }
                row[TARGET] = ms(0, b);
                row[RHS] = a;
                if rank.add(row) {
                    useful += 1;
                }
            }
            if let Some(row) = &rank.fb[FB] {
                let d = row.0[RHS];
                assert_eq!(affine_mul(&f, A, d, Some(G)), Some(Q));
                recovered = Some(d);
                done = true;
            }
        }
        elim_ms += t1.elapsed().as_secs_f64() * 1e3;
    }
    let cold_ms = cold.elapsed().as_secs_f64() * 1e3;
    let total = rank.lp_rank + rank.fb_rank;
    println!("{}", serde_json::to_string(&json!({
        "stream_digest": format!("{:016x}", digest.finish()),
        "arm": arm, "admit": if admit_all { "all" } else { "fb2" }, "k": k_frac, "salt": salt, "threads": threads, "queries": queries, "relations": seen.len(), "useful": useful,
        "rank": total, "recovered": recovered, "build_ms": build_ms, "probe_ms": probe_ms, "elim_ms": elim_ms,
        "cold_ms": cold_ms, "replay_ms": replay_ms, "rank_per_s": total as f64 / (cold_ms / 1e3), "probes": st.probes, "hits": st.hits,
        "ns_per_probe": probe_ms * 1e6 / (st.probes.max(1)) as f64,
    })).unwrap());
}
