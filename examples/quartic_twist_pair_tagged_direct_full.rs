//! Full-stream direct-posting action-tagged pair index for the p=37 twist.
//! Frozen protocol: research/twist_index_calculus_20261020/QUARTIC_TWIST_LINE_PROTOCOL.md.
use std::alloc::{GlobalAlloc, Layout, System};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::env;
use std::fs;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use crypto_lib::cryptanalysis::gaudry_quartic::{mitm_signed, pair_table, Curve4, Pt4, E4};
use crypto_lib::cryptanalysis::residual_walk::{is_prime_u64, pow_mod};
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde_json::{json, Value};

struct MeteredAlloc;
static LIVE: AtomicU64 = AtomicU64::new(0);
static PEAK: AtomicU64 = AtomicU64::new(0);

fn add_live(size: usize) {
    let now = LIVE.fetch_add(size as u64, Ordering::Relaxed) + size as u64;
    PEAK.fetch_max(now, Ordering::Relaxed);
}

unsafe impl GlobalAlloc for MeteredAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc(layout) };
        if !ptr.is_null() {
            add_live(layout.size());
        }
        ptr
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc_zeroed(layout) };
        if !ptr.is_null() {
            add_live(layout.size());
        }
        ptr
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) };
        LIVE.fetch_sub(layout.size() as u64, Ordering::Relaxed);
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let result = unsafe { System.realloc(ptr, layout, new_size) };
        if !result.is_null() {
            if new_size >= layout.size() {
                add_live(new_size - layout.size());
            } else {
                LIVE.fetch_sub((layout.size() - new_size) as u64, Ordering::Relaxed);
            }
        }
        result
    }
}

#[global_allocator]
static ALLOCATOR: MeteredAlloc = MeteredAlloc;

pub(crate) fn live() -> u64 {
    LIVE.load(Ordering::Relaxed)
}
pub(crate) fn reset_peak() {
    PEAK.store(live(), Ordering::Relaxed);
}
pub(crate) fn peak() -> u64 {
    PEAK.load(Ordering::Relaxed)
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pair {
    i: u16,
    j: u16,
    s: i8,
}

#[derive(Clone, Copy)]
struct SignedIndex {
    index: u16,
    sign: i8,
}

#[derive(Clone, Copy)]
struct ActionParams {
    z: u64,
    h2: u64,
    h3_degree: usize,
    h3_coeff: u64,
    omega: u64,
}

#[derive(Clone, Copy)]
struct Posting {
    i: u16,
    j: u16,
    si: i8,
    sj: i8,
    y_neg: bool,
}

#[derive(Clone, Copy)]
struct Key {
    x: E4,
    reference_y: E4,
    start: u32,
    len: u32,
}

#[derive(Clone, Copy)]
struct Stage {
    x: E4,
    y: E4,
    rep: u16,
    tag: u8,
}

pub(crate) struct TaggedIndex {
    reps: Vec<Pair>,
    keys: Vec<Key>,
    key_lookup: HashMap<E4, u32>,
    postings: Vec<Posting>,
    orbit_hist: BTreeMap<usize, u64>,
}

impl TaggedIndex {
    #[allow(dead_code)] // Used by the combined producer when this example is imported as a module.
    pub(crate) fn sizes(&self) -> (usize, usize, usize) {
        (self.reps.len(), self.keys.len(), self.postings.len())
    }
}

fn u(v: &Value) -> u64 {
    v.as_u64().expect("u64")
}
fn e(v: &Value) -> E4 {
    let a = v.as_array().expect("F4 element");
    E4([u(&a[0]), u(&a[1]), u(&a[2]), u(&a[3])])
}
fn point(v: &Value) -> Pt4 {
    Pt4 {
        x: e(&v[0]),
        y: e(&v[1]),
        inf: false,
    }
}
fn coordinates(v: &Value) -> Pt4 {
    Pt4 {
        x: e(&v["generator_x"]),
        y: e(&v["generator_y"]),
        inf: false,
    }
}

fn params(c: &Curve4) -> ActionParams {
    let f = &c.f;
    let p = f.p;
    let t = E4([0, 1, 0, 0]);
    let z = f.pow(&t, (p - 1) as u128);
    assert!(z.in_fp());
    let h = f.pow(&t, (p as u128).pow(4) - 1 - (p - 1) as u128 / 2);
    let h2 = f.sq(&h);
    let h3 = f.mul(&h2, &h);
    assert!(h2.in_fp());
    let monomials: Vec<_> = h3.0.iter().enumerate().filter(|(_, x)| **x != 0).collect();
    assert_eq!(monomials.len(), 1);
    let (h3_degree, &h3_coeff) = monomials[0];
    let omega = (2..p)
        .map(|a| pow_mod(a, (p - 1) / 3, p))
        .find(|&a| a != 1)
        .expect("cubic root");
    ActionParams {
        z: z.0[0],
        h2: h2.0[0],
        h3_degree,
        h3_coeff,
        omega,
    }
}

fn psi(c: &Curve4, pt: Pt4, par: ActionParams) -> Pt4 {
    if pt.inf {
        return pt;
    }
    let f = &c.f;
    Pt4 {
        x: f.scale(&f.frob_p(&pt.x, par.z), par.h2),
        y: f.mul_monomial(&f.frob_p(&pt.y, par.z), par.h3_coeff, par.h3_degree),
        inf: false,
    }
}

fn transformed_pair(pair: Pair, map: &[SignedIndex]) -> Pair {
    let a = map[pair.i as usize];
    let b = map[pair.j as usize];
    assert_ne!(a.index, b.index);
    Pair {
        i: a.index.min(b.index),
        j: a.index.max(b.index),
        s: a.sign * pair.s * b.sign,
    }
}

pub(crate) fn build_tagged(c: &Curve4, base: &[Pt4]) -> TaggedIndex {
    assert!(base.len() <= u16::MAX as usize);
    let par = params(c);
    let by_x: HashMap<_, _> = base.iter().enumerate().map(|(i, pt)| (pt.x, i)).collect();
    assert_eq!(by_x.len(), base.len());
    let mut maps: Vec<Vec<SignedIndex>> = (0..12).map(|_| Vec::with_capacity(base.len())).collect();
    for &pt in base {
        let mut frob = pt;
        for j in 0..4 {
            let mut image = frob;
            for k in 0..3 {
                assert!(c.on_curve(&image));
                let &at = by_x.get(&image.x).expect("invariant base closure");
                let sign = if image.y == base[at].y {
                    1
                } else {
                    assert_eq!(image.y, c.f.neg(&base[at].y));
                    -1
                };
                maps[j * 3 + k].push(SignedIndex {
                    index: at.try_into().unwrap(),
                    sign,
                });
                image.x = c.f.scale(&image.x, par.omega);
            }
            frob = psi(c, frob, par);
        }
        assert_eq!(frob, c.neg(&pt));
    }
    let mut seen = HashSet::new();
    let mut reps = Vec::new();
    let mut stage = Vec::new();
    let mut orbit_hist = BTreeMap::new();
    for i in 0..base.len() {
        for j in (i + 1)..base.len() {
            for s in [-1, 1] {
                let entry = Pair {
                    i: i.try_into().unwrap(),
                    j: j.try_into().unwrap(),
                    s,
                };
                if seen.contains(&entry) {
                    continue;
                }
                let orbit: BTreeSet<_> = maps
                    .iter()
                    .map(|map| transformed_pair(entry, map))
                    .collect();
                assert!(orbit.contains(&entry));
                for &member in &orbit {
                    assert!(seen.insert(member));
                }
                *orbit_hist.entry(orbit.len()).or_insert(0) += 1;
                let rep = *orbit.iter().next().unwrap();
                assert!(reps.len() < u16::MAX as usize);
                let rep_id = reps.len() as u16;
                reps.push(rep);
                let second = if rep.s == 1 {
                    base[rep.j as usize]
                } else {
                    c.neg(&base[rep.j as usize])
                };
                let sum = c.add(&base[rep.i as usize], &second);
                assert!(!sum.inf);
                let mut image = sum;
                let mut used = HashSet::new();
                for a in 0..4 {
                    let mut cubic = image;
                    for b in 0..3 {
                        let tag = a * 3 + b;
                        if used.insert(transformed_pair(rep, &maps[tag])) {
                            stage.push(Stage {
                                x: cubic.x,
                                y: cubic.y,
                                rep: rep_id,
                                tag: tag as u8,
                            });
                        }
                        cubic.x = c.f.scale(&cubic.x, par.omega);
                    }
                    image = psi(c, image, par);
                }
                assert_eq!(used.len(), orbit.len());
            }
        }
    }
    assert_eq!(seen.len(), base.len() * (base.len() - 1));
    assert_eq!(stage.len(), seen.len());
    drop(seen);
    drop(by_x);
    stage.sort_unstable_by_key(|item| item.x);
    let mut keys = Vec::new();
    let mut postings = Vec::with_capacity(stage.len());
    let mut at = 0usize;
    while at < stage.len() {
        let x = stage[at].x;
        let y = stage[at].y;
        let start = postings.len();
        while at < stage.len() && stage[at].x == x {
            let item = stage[at];
            let y_neg = if item.y == y {
                false
            } else {
                assert_eq!(item.y, c.f.neg(&y));
                true
            };
            let rep = reps[item.rep as usize];
            let map = &maps[item.tag as usize];
            let ai = map[rep.i as usize];
            let bj = map[rep.j as usize];
            postings.push(Posting {
                i: ai.index,
                j: bj.index,
                si: ai.sign,
                sj: rep.s * bj.sign,
                y_neg,
            });
            at += 1;
        }
        keys.push(Key {
            x,
            reference_y: y,
            start: start.try_into().unwrap(),
            len: (postings.len() - start).try_into().unwrap(),
        });
    }
    drop(stage);
    keys.shrink_to_fit();
    postings.shrink_to_fit();
    reps.shrink_to_fit();
    let key_lookup: HashMap<E4, u32> = keys
        .iter()
        .enumerate()
        .map(|(index, key)| (key.x, index.try_into().unwrap()))
        .collect();
    assert_eq!(key_lookup.len(), keys.len());
    TaggedIndex {
        reps,
        keys,
        key_lookup,
        postings,
        orbit_hist,
    }
}

pub(crate) fn audit_postings(c: &Curve4, base: &[Pt4], table: &TaggedIndex) {
    let mut seen = HashSet::new();
    for key in &table.keys {
        for posting in &table.postings[key.start as usize..(key.start + key.len) as usize] {
            let pa = if posting.si == 1 {
                base[posting.i as usize]
            } else {
                c.neg(&base[posting.i as usize])
            };
            let pb = if posting.sj == 1 {
                base[posting.j as usize]
            } else {
                c.neg(&base[posting.j as usize])
            };
            let sum = c.add(&pa, &pb);
            assert_eq!(sum.x, key.x);
            assert_eq!(sum.y == c.f.neg(&key.reference_y), posting.y_neg);
            let (i, j) = (posting.i.min(posting.j), posting.i.max(posting.j));
            assert!(seen.insert(Pair {
                i,
                j,
                s: posting.si * posting.sj
            }));
        }
    }
    assert_eq!(seen.len(), base.len() * (base.len() - 1));
}

#[derive(Default)]
pub(crate) struct QueryCounters {
    pub(crate) lookups: u64,
    pub(crate) candidate_postings: u64,
}

pub(crate) fn tagged_decompositions(
    c: &Curve4,
    base: &[Pt4],
    table: &TaggedIndex,
    r: &Pt4,
    counters: &mut QueryCounters,
) -> Vec<Vec<(usize, i64)>> {
    let mut out = BTreeSet::new();
    for k in 0..base.len() {
        for sk in [1i64, -1] {
            let pk = if sk == 1 { base[k] } else { c.neg(&base[k]) };
            let tk = c.sub(r, &pk);
            for l in (k + 1)..base.len() {
                for sl in [1i64, -1] {
                    let pl = if sl == 1 { base[l] } else { c.neg(&base[l]) };
                    let needed = c.sub(&tk, &pl);
                    if needed.inf {
                        continue;
                    }
                    counters.lookups += 1;
                    let Some(&which) = table.key_lookup.get(&needed.x) else {
                        continue;
                    };
                    let key = &table.keys[which as usize];
                    let needed_neg = if needed.y == key.reference_y {
                        false
                    } else {
                        assert_eq!(needed.y, c.f.neg(&key.reference_y));
                        true
                    };
                    let bucket =
                        &table.postings[key.start as usize..(key.start + key.len) as usize];
                    counters.candidate_postings += bucket.len() as u64;
                    for posting in bucket {
                        let (i, j) = (posting.i as usize, posting.j as usize);
                        if i == k || i == l || j == k || j == l {
                            continue;
                        }
                        let overall = if needed_neg == posting.y_neg {
                            1i64
                        } else {
                            -1i64
                        };
                        let mut terms = vec![
                            (k, sk),
                            (l, sl),
                            (i, overall * posting.si as i64),
                            (j, overall * posting.sj as i64),
                        ];
                        terms.sort_unstable();
                        out.insert(terms);
                    }
                }
            }
        }
    }
    out.into_iter().collect()
}

struct Input {
    p: u64,
    b0: u64,
    n: u64,
    base: Vec<Pt4>,
    g: Pt4,
    q: Pt4,
    targets: Vec<(u64, u64)>,
    selected: Vec<Vec<(usize, i64)>>,
}

fn load(p: u64, p13_path: &str, mixed_path: &str) -> Input {
    let p13: Value = serde_json::from_str(&fs::read_to_string(p13_path).unwrap()).unwrap();
    if p == 13 {
        let base = p13["base_coordinates"]
            .as_array()
            .unwrap()
            .iter()
            .map(point)
            .collect();
        let selected = p13["selected_relations"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| {
                r["terms"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|t| (u(&t[0]) as usize, t[1].as_i64().unwrap()))
                    .collect()
            })
            .collect();
        return Input {
            p,
            b0: 4,
            n: 28_753,
            base,
            g: coordinates(&p13),
            q: Pt4 {
                x: e(&p13["target_x"]),
                y: e(&p13["target_y"]),
                inf: false,
            },
            targets: p13["query_trace"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| (u(&v[0]), u(&v[1])))
                .collect(),
            selected,
        };
    }
    assert_eq!(p, 37);
    let records: Vec<Value> = fs::read_to_string(mixed_path)
        .unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    let single = records
        .iter()
        .find(|r| u(&r["p"]) == 37 && u(&r["seed"]) == 0 && r["line_powers"] == json!([0]))
        .unwrap();
    let union = records
        .iter()
        .find(|r| u(&r["p"]) == 37 && u(&r["seed"]) == 0 && r["line_powers"] == json!([0, 2]))
        .unwrap();
    assert_eq!(single["generator_x"], union["generator_x"]);
    assert_eq!(single["generator_y"], union["generator_y"]);
    assert_eq!(single["target_x"], union["target_x"]);
    let c = Curve4::with_params(37, E4::ZERO, E4([0, 0, 0, 2]));
    let mut base: Vec<_> = union["base_coordinates"]
        .as_array()
        .unwrap()
        .iter()
        .map(point)
        .collect();
    assert_eq!(base.len(), 48);
    let mut rng = StdRng::seed_from_u64(0x5441_4747_0000_0037);
    for a in 1..37 {
        for b in 1..37 {
            if let Some(pt) = c.lift_x(&E4([a, 0, b, 0]), &mut rng) {
                base.push(pt);
            }
        }
    }
    assert_eq!(base.len(), 624);
    let targets: Vec<_> = single["query_trace"]
        .as_array()
        .unwrap()
        .iter()
        .take(3)
        .map(|v| (u(&v[0]), u(&v[1])))
        .collect();
    assert_eq!(targets.len(), 3);
    assert_eq!(
        targets,
        union["query_trace"]
            .as_array()
            .unwrap()
            .iter()
            .take(3)
            .map(|v| (u(&v[0]), u(&v[1])))
            .collect::<Vec<_>>()
    );
    let selected: Vec<Vec<(usize, i64)>> = Vec::new(); // L0/L02 indices are not a full 2D-base oracle certificate.
    Input {
        p,
        b0: 2,
        n: u(&single["n"]),
        base,
        g: coordinates(single),
        q: Pt4 {
            x: e(&single["target_x"]),
            y: e(&single["target_y"]),
            inf: false,
        },
        targets,
        selected,
    }
}

fn run(input: Input) -> Value {
    assert!(is_prime_u64(input.n) && input.n > 4 * input.p * input.p);
    let common = Curve4::with_params(input.p, E4::ZERO, E4([0, 0, 0, input.b0]));
    assert!(common.on_curve(&input.g) && common.on_curve(&input.q));
    assert!(common.mul(&input.g, input.n).inf);
    assert_eq!(common.mul(&input.g, 271), input.q);
    assert!(input.base.iter().all(|pt| common.on_curve(pt)));
    let residuals: Vec<_> = input
        .targets
        .iter()
        .map(|&(a, b)| common.add(&common.mul(&input.g, a), &common.mul(&input.q, b)))
        .collect();
    let full_curve = Curve4::with_params(input.p, E4::ZERO, E4([0, 0, 0, input.b0]));
    full_curve.reset_ops();
    full_curve.f.reset_muls();
    let full_before = live();
    reset_peak();
    let start = Instant::now();
    let full_table = pair_table(&full_curve, &input.base);
    let full_setup_ms = start.elapsed().as_secs_f64() * 1e3;
    let full_setup_m = full_curve.f.muls();
    let full_setup_g = full_curve.ops();
    let full_retained = live() - full_before;
    let full_peak = peak() - full_before;
    let tagged_curve = Curve4::with_params(input.p, E4::ZERO, E4([0, 0, 0, input.b0]));
    tagged_curve.reset_ops();
    tagged_curve.f.reset_muls();
    let tagged_before = live();
    reset_peak();
    let start = Instant::now();
    let tagged = build_tagged(&tagged_curve, &input.base);
    let tagged_setup_ms = start.elapsed().as_secs_f64() * 1e3;
    let tagged_setup_m = tagged_curve.f.muls();
    let tagged_setup_g = tagged_curve.ops();
    let tagged_retained = live() - tagged_before;
    let tagged_peak = peak() - tagged_before;
    let audit_curve = Curve4::with_params(input.p, E4::ZERO, E4([0, 0, 0, input.b0]));
    audit_postings(&audit_curve, &input.base, &tagged);
    let audit_m = audit_curve.f.muls();
    let audit_g = audit_curve.ops();
    full_curve.reset_ops();
    full_curve.f.reset_muls();
    tagged_curve.reset_ops();
    tagged_curve.f.reset_muls();
    let mut full_query_ms = 0.0;
    let mut tagged_query_ms = 0.0;
    let mut counters = QueryCounters::default();
    let mut rows = Vec::new();
    for (query_index, r) in residuals.iter().enumerate() {
        let start = Instant::now();
        let full = mitm_signed(&full_curve, &input.base, &full_table, r);
        full_query_ms += start.elapsed().as_secs_f64() * 1e3;
        let start = Instant::now();
        let from_tags =
            tagged_decompositions(&tagged_curve, &input.base, &tagged, r, &mut counters);
        tagged_query_ms += start.elapsed().as_secs_f64() * 1e3;
        assert_eq!(
            full, from_tags,
            "signed four-sum mismatch query {query_index}"
        );
        if input.p == 13 {
            assert!(full.contains(&input.selected[query_index]));
        }
        rows.push(
            json!({"query_index":query_index,"a":input.targets[query_index].0,
            "b":input.targets[query_index].1,"decompositions":full.len(),
            "sets_equal":true,"selected_present":input.p==13}),
        );
    }
    json!({"p":input.p,"b0":input.b0,"n":input.n,"base_points":input.base.len(),
        "queries":rows,"full_pair_entries":input.base.len()*(input.base.len()-1),
        "tagged_pair_representatives":tagged.reps.len(),
        "tagged_postings":tagged.postings.len(),"tagged_keys":tagged.keys.len(),
        "pair_orbit_hist":tagged.orbit_hist,
        "full_setup_fp_muls":full_setup_m,"full_setup_group_ops":full_setup_g,
        "full_query_fp_muls":full_curve.f.muls(),"full_query_group_ops":full_curve.ops(),
        "tagged_setup_fp_muls":tagged_setup_m,"tagged_setup_group_ops":tagged_setup_g,
        "tagged_query_fp_muls":tagged_curve.f.muls(),"tagged_query_group_ops":tagged_curve.ops(),
        "full_table_live_requested_bytes":full_retained,
        "full_table_peak_requested_bytes":full_peak,
        "tagged_table_live_requested_bytes":tagged_retained,
        "tagged_table_peak_requested_bytes":tagged_peak,
        "tagged_lookup_calls":counters.lookups,
        "tagged_key_comparisons":0,
        "tagged_key_hash_lookups":counters.lookups,
        "tagged_candidate_postings":counters.candidate_postings,
        "posting_audit_fp_muls_excluded":audit_m,
        "posting_audit_group_ops_excluded":audit_g,
        "full_setup_ms_l0":full_setup_ms,"full_query_ms_l0":full_query_ms,
        "tagged_setup_ms_l0":tagged_setup_ms,"tagged_query_ms_l0":tagged_query_ms,
        "status":"pass"})
}

fn main() {
    let args: Vec<_> = env::args().collect();
    assert_eq!(
        args.len(),
        6,
        "usage: quartic_twist_pair_tagged_direct_full --p 37 P13_RAW MIXED_RAW P37_2D_RAW"
    );
    assert_eq!(args[1], "--p");
    let p: u64 = args[2].parse().unwrap();
    assert_eq!(p, 37);
    let mut input = load(p, &args[3], &args[4]);
    let record: Value = serde_json::from_str(&fs::read_to_string(&args[5]).unwrap()).unwrap();
    assert_eq!(record["p"], 37);
    assert_eq!(record["physical_base"], input.base.len());
    assert_eq!(coordinates(&record), input.g);
    assert_eq!(e(&record["target_x"]), input.q.x);
    assert_eq!(e(&record["target_y"]), input.q.y);
    let recorded_base: Vec<_> = record["base_coordinates"]
        .as_array()
        .unwrap()
        .iter()
        .map(point)
        .collect();
    assert_eq!(recorded_base, input.base);
    input.targets = record["query_trace"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| (u(&v[0]), u(&v[1])))
        .collect();
    assert_eq!(input.targets.len(), 53);
    let mut output = run(input);
    output["trace_kind"] = json!("p37_full_2d_53_residuals");
    let recorded_counts: Vec<_> = record["query_trace"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| u(&v[2]))
        .collect();
    let measured_counts: Vec<_> = output["queries"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| u(&v["decompositions"]))
        .collect();
    assert_eq!(recorded_counts, measured_counts);
    println!("{}", output);
}
