//! Source-bound local component timer for binary rootfinding pair-index layouts.
//! Frozen boundary: research/rootfind_pair_index_timing_20261116/PROTOCOL.md
//! and AMENDMENT-1.md. Curve arithmetic is outside timed intervals.

#[path = "../research/rootfind_inline_page_index_20261116/inline_page_index.rs"]
mod inline_page_index;
#[path = "../research/rootfind_paged_pair_index_20261116/paged_index.rs"]
mod paged_index;

use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::hint::black_box;
use std::time::Instant;

use crypto_lib::cryptanalysis::ic_boundary::{
    koblitz_factor_base, koblitz_instance, BinaryGroup, ColumnFold, FactorBase, MixHasher,
    PairTable,
};
use crypto_lib::cryptanalysis::koblitz_fast::{FastCurve, FastPoint};
use crypto_lib::cryptanalysis::koblitz_index_calculus::build_standard_subspace_factor_base;
use inline_page_index::InlinePagePairIndex;
use paged_index::PagedPairIndex;
use serde_json::json;
use sha2::{Digest, Sha256};

type ProductionHash = HashMap<u64, (u32, u32), BuildHasherDefault<MixHasher>>;

#[derive(Clone, Copy)]
struct Pair {
    key: u32,
    i: u32,
    j: u32,
}

struct Case {
    n: u32,
    abscissae: usize,
    pairs: Vec<Pair>,
    probes: Vec<u32>,
    expected_checksum: u64,
    pair_sha256: String,
    probe_sha256: String,
}

#[derive(Clone, Copy)]
enum Arm {
    Hash,
    HashAa,
    Flat,
    Paged,
    Inline,
}

impl Arm {
    fn label(self) -> &'static str {
        match self {
            Self::Hash => "hash",
            Self::HashAa => "hash-aa",
            Self::Flat => "flat-direct-u32",
            Self::Paged => "paged",
            Self::Inline => "inline",
        }
    }
}

enum Index {
    Hash(ProductionHash),
    Flat(Vec<u32>),
    Paged(PagedPairIndex),
    Inline(InlinePagePairIndex),
}

impl Index {
    fn new(arm: Arm, n: u32, abscissae: usize) -> Self {
        match arm {
            Arm::Hash | Arm::HashAa => Self::Hash(HashMap::with_capacity_and_hasher(
                abscissae * abscissae + 1,
                BuildHasherDefault::default(),
            )),
            Arm::Flat => Self::Flat(vec![0; (1usize << n) + 1]),
            Arm::Paged => Self::Paged(
                PagedPairIndex::from_base_abscissae(n, abscissae).expect("paged capacity"),
            ),
            Arm::Inline => Self::Inline(
                InlinePagePairIndex::from_base_abscissae(n, abscissae).expect("inline capacity"),
            ),
        }
    }

    fn insert(&mut self, pair: Pair) {
        match self {
            Self::Hash(map) => {
                map.entry(u64::from(pair.key)).or_insert((pair.i, pair.j));
            }
            Self::Flat(words) => {
                assert!(pair.i < 512 && pair.j < 512);
                let code = ((pair.i << 9) | pair.j) + 1;
                let slot = &mut words[pair.key as usize];
                if *slot == 0 {
                    *slot = code;
                }
            }
            Self::Paged(index) => {
                index
                    .insert(pair.key, pair.i, pair.j)
                    .expect("paged insert");
            }
            Self::Inline(index) => {
                index
                    .insert(pair.key, pair.i, pair.j)
                    .expect("inline insert");
            }
        }
    }

    fn get(&self, key: u32) -> Option<(u32, u32)> {
        if key == 0 {
            return None;
        }
        match self {
            Self::Hash(map) => map.get(&u64::from(key)).copied(),
            Self::Flat(words) => words.get(key as usize).and_then(|&code| {
                (code != 0).then(|| {
                    let packed = code - 1;
                    (packed >> 9, packed & 511)
                })
            }),
            Self::Paged(index) => index.get(key),
            Self::Inline(index) => index.get(key),
        }
    }
}

fn digest_words(words: impl IntoIterator<Item = u32>) -> String {
    let mut hasher = Sha256::new();
    for word in words {
        hasher.update(word.to_le_bytes());
    }
    format!("{:x}", hasher.finalize())
}

fn base_digest(fb: &FactorBase<FastPoint>) -> String {
    let mut hasher = Sha256::new();
    for (idx, point) in fb.points.iter().enumerate() {
        hasher.update(point.pack().to_le_bytes());
        hasher.update((fb.col_of[idx] as u64).to_le_bytes());
        hasher.update(fb.coef_of[idx].to_le_bytes());
    }
    format!("{:x}", hasher.finalize())
}

fn scalar_mul(curve: &FastCurve, point: FastPoint, mut scalar: u64) -> FastPoint {
    let mut result = FastPoint::INFINITY;
    let mut addend = point;
    while scalar != 0 {
        if scalar & 1 != 0 {
            result = curve.add(result, addend);
        }
        addend = curve.add(addend, addend);
        scalar >>= 1;
    }
    result
}

fn checksum(index: &Index, probes: &[u32]) -> u64 {
    let mut acc = 0x4d59_5df4_d0f3_3173u64;
    for &key in probes {
        let witness = black_box(index.get(black_box(key)));
        let code = witness.map_or(0u64, |(i, j)| (u64::from(i + 1) << 32) | u64::from(j));
        acc = acc.rotate_left(7) ^ code.wrapping_add(u64::from(key));
    }
    black_box(acc)
}

fn build_case(
    n: u32,
    dimension: u32,
    abscissae: usize,
    points: usize,
    attempts_expected: usize,
    keys_expected: usize,
    collisions_expected: usize,
    curve_id: &str,
    base_sha256: &str,
) -> Case {
    let inst = koblitz_instance(0, n).expect("registered Koblitz E0 cell");
    assert_eq!(inst.curve_id().icv1, curve_id);
    let parent = build_standard_subspace_factor_base(
        inst.koblitz.as_ref().expect("Koblitz view"),
        dimension,
    )
    .expect("standard subspace");
    let fb = koblitz_factor_base(
        &inst,
        &parent,
        ColumnFold::SignedFrobeniusOrbit,
        format!("standard subspace d={dimension}"),
    )
    .expect("signed orbit base");
    assert_eq!(fb.abscissa_list.len(), abscissae);
    assert_eq!(fb.points.len(), points);
    assert_eq!(base_digest(&fb), base_sha256);
    assert!(points < 512);
    let group = BinaryGroup(&inst.fast);
    let (production, production_collisions, _) =
        PairTable::build_negation_folded_binary_batch_torsion_with_collisions(&group, &fb)
            .expect("production folded torsion table");
    let groups: Vec<&Vec<usize>> = fb
        .abscissa_list
        .iter()
        .map(|x| fb.x_index.get(x).expect("listed abscissa"))
        .collect();
    let mut first = HashMap::<u32, (u32, u32, FastPoint)>::new();
    let mut pairs = Vec::new();
    let mut collisions = Vec::new();
    let mut add_pair = |i: usize, j: usize, sum: FastPoint| {
        if sum.infinity {
            return;
        }
        let key = u32::try_from(sum.x + 1).expect("binary key fits");
        if let Some(&(fi, fj, first_sum)) = first.get(&key) {
            assert_eq!(sum.x, first_sum.x);
            assert!(sum == first_sum || sum == inst.fast.neg(first_sum));
            let row = if sum == first_sum {
                [i, j, fb.neg_index[fi as usize], fb.neg_index[fj as usize]]
            } else {
                [i, j, fi as usize, fj as usize]
            };
            let group_sum = row.iter().fold(FastPoint::INFINITY, |acc, &idx| {
                inst.fast.add(acc, fb.points[idx])
            });
            assert!(group_sum.infinity, "duplicate pair is a group equation");
            collisions.push([i, j, fi as usize, fj as usize]);
        } else {
            first.insert(key, (i as u32, j as u32, sum));
        }
        pairs.push(Pair {
            key,
            i: i as u32,
            j: j as u32,
        });
    };
    for (ai, group_i) in groups.iter().enumerate() {
        let i = group_i[0];
        if ai == 0 {
            for group_j in groups.iter().skip(1) {
                let j = group_j[0];
                let sum = inst.fast.add(fb.points[i], fb.points[j]);
                assert!(!sum.infinity);
                add_pair(i, j, sum);
            }
            continue;
        }
        for group_j in groups.iter().skip(ai) {
            for &j in group_j.iter() {
                add_pair(i, j, inst.fast.add(fb.points[i], fb.points[j]));
            }
        }
    }
    assert_eq!(pairs.len(), attempts_expected);
    assert_eq!(first.len(), keys_expected);
    assert_eq!(collisions.len(), collisions_expected);
    assert_eq!(collisions, production_collisions);
    assert_eq!(production.entries as usize, keys_expected);
    assert_eq!(production.insert_attempts as usize, attempts_expected);

    let q = scalar_mul(&inst.fast, inst.generator, 641);
    let mut probes = Vec::with_capacity(64 * points);
    for t in 0..64u64 {
        let a = if n == 13 {
            (1000 + 3 * t) % inst.r
        } else {
            105_000 + 3 * t
        };
        let residual = inst.fast.add(q, scalar_mul(&inst.fast, inst.generator, a));
        assert!(!residual.infinity);
        for &point in &fb.points {
            let difference = inst.fast.add(residual, inst.fast.neg(point));
            probes.push(if difference.infinity {
                0
            } else {
                u32::try_from(difference.x + 1).expect("binary key fits")
            });
        }
    }
    assert_eq!(probes.len(), 64 * points);
    let pair_sha256 = digest_words(pairs.iter().flat_map(|p| [p.key, p.i, p.j]));
    let probe_sha256 = digest_words(probes.iter().copied());
    let reference = Index::Hash(first.iter().fold(
        HashMap::with_capacity_and_hasher(abscissae * abscissae + 1, BuildHasherDefault::default()),
        |mut map, (&key, &(i, j, _))| {
            map.insert(u64::from(key), (i, j));
            map
        },
    ));
    let expected_checksum = checksum(&reference, &probes);
    for arm in [Arm::Hash, Arm::HashAa, Arm::Flat, Arm::Paged, Arm::Inline] {
        let mut index = Index::new(arm, n, abscissae);
        for &pair in &pairs {
            index.insert(pair);
        }
        assert_eq!(checksum(&index, &probes), expected_checksum);
        for (&key, &(i, j, _)) in &first {
            assert_eq!(index.get(key), Some((i, j)));
        }
    }
    println!(
        "PAIR_INDEX_CASE_JSON={}",
        json!({
            "schema":"rootfind-pair-index-case/v1", "n":n, "curve_id":curve_id,
            "base_sha256":base_sha256, "abscissae":abscissae, "points":points,
            "pair_attempts":pairs.len(), "first_keys":first.len(),
            "checked_collisions":collisions.len(), "probes":probes.len(),
            "pair_sha256":pair_sha256, "probe_sha256":probe_sha256,
            "expected_checksum":expected_checksum
        })
    );
    Case {
        n,
        abscissae,
        pairs,
        probes,
        expected_checksum,
        pair_sha256,
        probe_sha256,
    }
}

#[test]
fn fixed_koblitz_pair_index_build_probe_timing() {
    let cells = [
        (
            13,
            5,
            17,
            33,
            272,
            261,
            11,
            "ICV1:f2m-13-752c6463:181:8012:0x1:-7:unk:r:515ee5696294",
            "d9f69121c63a04c3f744c5be0e4ea488a030db0315794a4e05c7a9d3cd341807",
        ),
        (
            19,
            7,
            70,
            139,
            4_830,
            4_800,
            30,
            "ICV1:f2m-19-87f45376:797:523492:0x1:-7:unk:r:b6cf2467887c",
            "013a0d4223494500cd06b48bf41b51dda2dd5add0009d1b89cdb0b7bd49c0a5a",
        ),
        (
            23,
            8,
            138,
            275,
            18_906,
            18_842,
            64,
            "ICV1:f2m-23-c07a6b8c:5197:8383412:0x1:-7:unk:r:69e76b739aa7",
            "686cba2fafe973e49139c2722df47bf728340e25a4df5faad8b51dfa91df3877",
        ),
    ];
    let arms = [Arm::Hash, Arm::HashAa, Arm::Flat, Arm::Paged, Arm::Inline];
    for (curve_index, (n, dimension, a, points, attempts, keys, collisions, curve_id, digest)) in
        cells.into_iter().enumerate()
    {
        let case = build_case(
            n, dimension, a, points, attempts, keys, collisions, curve_id, digest,
        );
        for repetition in 0..108usize {
            for offset in 0..arms.len() {
                let arm = arms[(repetition + curve_index + offset) % arms.len()];
                let started = Instant::now();
                let mut index = Index::new(arm, case.n, case.abscissae);
                for &pair in &case.pairs {
                    index.insert(pair);
                }
                let build_ns = started.elapsed().as_nanos() as u64;
                let probe_started = Instant::now();
                let result = checksum(&index, &case.probes);
                let probe_ns = probe_started.elapsed().as_nanos() as u64;
                assert_eq!(result, case.expected_checksum);
                if repetition >= 7 {
                    println!(
                        "PAIR_INDEX_TIMING_JSON={}",
                        json!({
                            "schema":"rootfind-pair-index-timing/v1", "n":case.n,
                            "repetition":repetition-7, "arm":arm.label(),
                            "build_ns":build_ns, "probe_ns":probe_ns,
                            "combined_ns":build_ns+probe_ns,
                            "checksum":result,
                            "pair_sha256":case.pair_sha256,
                            "probe_sha256":case.probe_sha256
                        })
                    );
                }
            }
        }
    }
}
