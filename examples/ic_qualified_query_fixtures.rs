//! Additional ordinary binary-curve queries. Native correctness controls only.
use crypto_lib::binary_ecc::{BinaryCurve, BinaryPoint, F2mElement};
use crypto_lib::cryptanalysis::{
    ecbench::claim::id_sha256,
    ic_boundary::{
        random_binary_instance, ArtinSchreier, BinaryGroup, BinaryInstance, CountedGroup, GroupOps,
    },
    koblitz_fast::{FastCurve, FastPoint},
    koblitz_index_calculus::find_irreducible_sparse,
    residual_walk::is_prime_u64,
    semaev_decomp::Gf2,
};
use num_bigint::BigUint;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, io::Write};

#[allow(dead_code)]
#[path = "ic_algebraic_followup/exact.rs"]
mod exact;
#[allow(dead_code)]
#[path = "../src/bin/ecbench/structured_s4_math.rs"]
mod math;

fn span(bs: &[u64]) -> Vec<u64> {
    math::span(bs)
}
fn element(x: u64, n: u32) -> F2mElement {
    math::element(x, n)
}
fn emit(v: &Value) {
    println!("{v}");
    std::io::stdout().flush().unwrap();
}
fn point(p: FastPoint) -> Value {
    if p.infinity {
        json!({"infinity":true})
    } else {
        json!({"x":p.x,"y":p.y})
    }
}
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
fn full_degree(gf: &Gf2, b: u64) -> bool {
    let mut c = b;
    for d in 1..=gf.n {
        c = gf.sqr(c);
        if c == b {
            return d == gf.n;
        }
    }
    unreachable!()
}
fn validate(inst: &BinaryInstance) {
    assert!(inst.a == 0 && inst.b > 1 && full_degree(&inst.gf, inst.b));
    assert!(is_prime_u64(inst.r) && inst.r > 2);
    assert_eq!(inst.r.checked_mul(inst.cofactor).unwrap(), inst.group_order);
    assert_eq!(gcd(inst.r, inst.cofactor), 1);
    assert!(!inst.generator.infinity && inst.fast.is_on_curve(inst.generator));
    assert!(
        BinaryGroup(&inst.fast)
            .mul(&mut GroupOps::default(), inst.generator, inst.r)
            .infinity
    );
}
fn record(inst: &BinaryInstance) -> Value {
    let mut r = json!({"schema":"ic-qualified-curve/v1","icv1":inst.name,
        "n":inst.n,"field_modulus":inst.gf.irr,"polynomial_low_terms":inst.irreducible.low_terms,
        "a":inst.a,"b":inst.b,"group_order":inst.group_order,"r":inst.r,"cofactor":inst.cofactor,
        "generator":point(inst.generator),"subgroup":"prime-order-subgroup-generated-by-G",
        "full_degree_b":true,"registry_status":"unregistered-research-model"});
    let hash = id_sha256(&r).unwrap();
    r["record_sha256"] = json!(hash);
    r["ec1_record_id"] = json!(format!("EC1N{}Cbinh{}", inst.n, &hash[..12]));
    r
}
fn construct(n: u32) {
    assert!([17, 18, 22, 25].contains(&n));
    for offset in 0..64u64 {
        let seed = 202_610_250_100 + u64::from(n) + offset;
        let Some(inst) = random_binary_instance(n, seed, 16) else {
            emit(
                &json!({"schema":"ic-qualified-construction/v1","n":n,"seed":seed,"status":"factory_returned_none"}),
            );
            continue;
        };
        let degree = full_degree(&inst.gf, inst.b);
        let qualified =
            inst.a == 0 && degree && is_prime_u64(inst.r) && gcd(inst.r, inst.cofactor) == 1;
        if qualified {
            validate(&inst);
        }
        let mut r = json!({"schema":"ic-qualified-construction/v1","n":n,"seed":seed,
            "status":if qualified {"selected"} else {"rejected"},"full_degree_b":degree,
            "model":inst.describe()});
        if qualified {
            r["qualified_curve"] = record(&inst);
        }
        emit(&r);
        if qualified {
            return;
        }
    }
    emit(
        &json!({"schema":"ic-qualified-construction/v1","n":n,"status":"no_qualifying_model_in_64_factory_seeds"}),
    );
}
fn read_model(path: &str) -> (BinaryInstance, Value) {
    let text = std::fs::read_to_string(path).unwrap();
    let records: Vec<Value> = text
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    let r = records
        .iter()
        .find(|r| r["status"] == "selected")
        .expect("no selected model")["qualified_curve"]
        .clone();
    let n = r["n"].as_u64().unwrap() as u32;
    let irr = find_irreducible_sparse(n).unwrap();
    let gf = Gf2::new(&irr);
    assert_eq!(r["field_modulus"], json!(gf.irr));
    assert_eq!(r["polynomial_low_terms"], json!(irr.low_terms));
    let a = r["a"].as_u64().unwrap();
    let b = r["b"].as_u64().unwrap();
    let generator = FastPoint::affine(
        r["generator"]["x"].as_u64().unwrap(),
        r["generator"]["y"].as_u64().unwrap(),
    );
    let group_order = r["group_order"].as_u64().unwrap();
    let rp = r["r"].as_u64().unwrap();
    let h = r["cofactor"].as_u64().unwrap();
    let c = BinaryCurve {
        m: n,
        irreducible: irr.clone(),
        a: element(a, n),
        b: element(b, n),
        generator: BinaryPoint::Affine {
            x: element(generator.x, n),
            y: element(generator.y, n),
        },
        order: BigUint::from(rp),
        cofactor: BigUint::from(h),
    };
    let inst = BinaryInstance {
        name: r["icv1"].as_str().unwrap().to_string(),
        n,
        irreducible: irr,
        fast: FastCurve::new(&c).unwrap(),
        artin_schreier: ArtinSchreier::new(&gf),
        gf,
        a,
        b,
        group_order,
        r: rp,
        cofactor: h,
        generator,
        koblitz: None,
    };
    validate(&inst);
    assert_eq!(record(&inst), r);
    (inst, r)
}
fn hash(domain: &[u8], words: &[u64], string: &str) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(domain);
    for w in words {
        h.update(w.to_le_bytes());
    }
    h.update((string.len() as u64).to_le_bytes());
    h.update(string.as_bytes());
    h.finalize().into()
}
fn target(
    inst: &BinaryInstance,
    identity: &str,
    m: usize,
    ell: usize,
    draw: u64,
) -> (FastPoint, u64) {
    let group = BinaryGroup(&inst.fast);
    for attempt in 0..4096 {
        let words = [
            202_610_250_001,
            m as u64,
            u64::from(inst.n),
            ell as u64,
            draw,
            attempt,
        ];
        let hx = hash(b"ic-qualified-target-x/v1\0", &words, identity);
        let hs = hash(b"ic-qualified-target-sign/v1\0", &words, identity);
        let x = u64::from_le_bytes(hx[..8].try_into().unwrap()) & ((1u64 << inst.n) - 1);
        let ps = inst.points_with_x(x);
        if ps.is_empty() {
            continue;
        }
        let p = ps[usize::from(hs[0] & 1) % ps.len()];
        let r = group.mul(&mut GroupOps::default(), p, inst.cofactor);
        if r.infinity {
            continue;
        }
        assert!(
            inst.fast.is_on_curve(r) && group.mul(&mut GroupOps::default(), r, inst.r).infinity
        );
        return (r, attempt);
    }
    panic!("hash-to-subgroup exceeded predeclared 4096 attempts")
}

#[derive(Clone)]
struct Sum<E> {
    p: E,
    count: u64,
    witness: Vec<E>,
}
fn sums<G: CountedGroup>(
    g: &G,
    base: &[G::Elt],
    k: usize,
    ops: &mut GroupOps,
) -> BTreeMap<u64, Sum<G::Elt>> {
    let o = g.identity();
    let mut states = BTreeMap::from([(
        g.key(&o),
        Sum {
            p: o,
            count: 1,
            witness: vec![],
        },
    )]);
    for _ in 0..k {
        let mut next = BTreeMap::<u64, Sum<G::Elt>>::new();
        for a in states.values() {
            for &b in base {
                let p = g.add(ops, a.p, b);
                let key = g.key(&p);
                if let Some(row) = next.get_mut(&key) {
                    assert_eq!(row.p, p);
                    row.count = row.count.checked_add(a.count).unwrap();
                } else {
                    let mut witness = a.witness.clone();
                    witness.push(b);
                    next.insert(
                        key,
                        Sum {
                            p,
                            count: a.count,
                            witness,
                        },
                    );
                }
            }
        }
        states = next;
    }
    states
}
fn point_count<G: CountedGroup>(
    g: &G,
    base: &[G::Elt],
    r: G::Elt,
    m: usize,
) -> (u64, Option<Vec<G::Elt>>, GroupOps) {
    let mut ops = GroupOps::default();
    let left = sums(g, base, m / 2, &mut ops);
    let right = sums(g, base, m - m / 2, &mut ops);
    let mut total = 0u64;
    let mut witness = None;
    for a in right.values() {
        let need = g.add(&mut ops, r, g.neg(a.p));
        if let Some(b) = left.get(&g.key(&need)) {
            assert_eq!(b.p, need);
            total = total
                .checked_add(a.count.checked_mul(b.count).unwrap())
                .unwrap();
            if witness.is_none() {
                let mut w = b.witness.clone();
                w.extend(&a.witness);
                witness = Some(w);
            }
        }
    }
    if let Some(w) = &witness {
        let sum = w.iter().fold(g.identity(), |s, &p| g.add(&mut ops, s, p));
        assert_eq!(sum, r);
    }
    (total, witness, ops)
}
fn fixtures(path: &str, m: usize, ell: usize, family: &str) {
    let (inst, curve) = read_model(path);
    assert!([(3, 22, 8), (3, 25, 9), (4, 17, 6), (4, 18, 6)].contains(&(m, inst.n, ell)));
    assert!(["prefix", "geometric", "geomtraceu", "random"].contains(&family));
    let identity = curve["record_sha256"].as_str().unwrap();
    let group = BinaryGroup(&inst.fast);
    let mut seen = BTreeMap::<(Vec<u64>, u64), u64>::new();
    let mut neg = 0;
    let mut sat = 0;
    for draw in 0..128u64 {
        let h = hash(
            b"ic-qualified-basis/v1\0",
            &[
                202_610_250_003,
                m as u64,
                u64::from(inst.n),
                ell as u64,
                draw,
            ],
            family,
        );
        let seed = u64::from_le_bytes(h[..8].try_into().unwrap());
        let bs = math::make_basis(&inst.gf, ell, family, seed);
        assert_eq!(math::basis(bs.iter().copied()).len(), ell);
        let mut base = BTreeMap::new();
        for x in span(&bs) {
            for p in inst.points_with_x(x) {
                if group.mul(&mut GroupOps::default(), p, inst.r).infinity {
                    assert!(inst.fast.is_on_curve(p));
                    base.insert(p.pack(), p);
                }
            }
        }
        let base: Vec<_> = base.into_values().collect();
        let (r, attempt) = target(&inst, identity, m, ell, draw);
        let solutions = exact::count_for_b(&inst.gf, &bs, r.x, m, inst.b);
        let (actual, witness, ops) = point_count(&group, &base, r, m);
        let direct = base.contains(&r);
        let key = (bs.clone(), r.x);
        let duplicate = seen.get(&key).copied();
        seen.entry(key).or_insert(draw);
        let selected = if duplicate.is_none()
            && base.len() >= 2
            && !direct
            && solutions == 0
            && actual == 0
            && neg < 4
        {
            neg += 1;
            Some("negative")
        } else if duplicate.is_none() && solutions > 0 && actual > 0 && sat < 2 {
            sat += 1;
            Some("positive")
        } else {
            None
        };
        let base_json: Vec<_> = base.iter().copied().map(point).collect();
        let base_sha = id_sha256(&json!(base_json)).unwrap();
        let witness = witness.map(|w| w.into_iter().map(point).collect::<Vec<_>>());
        emit(
            &json!({"schema":"ic-qualified-query/v1","m":m,"n":inst.n,"ell":ell,"family":family,"draw":draw,
            "seed":seed,"field_modulus":inst.gf.irr,"n_vars":m*ell+(m-2)*inst.n as usize,
            "curve_id":curve["ec1_record_id"],"curve_record_sha256":identity,"icv1":inst.name,"curve_a":inst.a,"curve_b":inst.b,
            "v_basis":bs,"x_r":r.x,"target":point(r),"target_law":"hash-lift-cofactor-clear-reject-O/v1",
            "target_hash_attempt":attempt,"solutions":solutions,"point_decompositions":actual,
            "point_witness":witness,"point_witness_verified":actual>0,"base_points":base.len(),"base_points_sha256":base_sha,
            "direct_base_hit":direct,"duplicate_of_draw":duplicate,"followup_selection":selected,
            "point_oracle_ops":ops,"oracle":"nonzero-b-quadratic-chain-mitm-v2","outcome":{"kind":"unmeasured"}}),
        );
    }
    eprintln!("selection negative={neg}/4 positive={sat}/2; all 128 draws retained");
}
fn main() {
    let a: Vec<_> = std::env::args().skip(1).collect();
    match a.first().map(String::as_str) {
        Some("model") if a.len() == 2 => construct(a[1].parse().unwrap()),
        Some("fixtures") if a.len() == 5 => {
            fixtures(&a[1], a[2].parse().unwrap(), a[3].parse().unwrap(), &a[4])
        }
        _ => panic!("model N | fixtures MODEL_JSONL M ELL FAMILY"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    struct Cyclic;
    impl CountedGroup for Cyclic {
        type Elt = u64;
        fn identity(&self) -> u64 {
            0
        }
        fn is_identity(&self, p: &u64) -> bool {
            *p == 0
        }
        fn add(&self, _: &mut GroupOps, a: u64, b: u64) -> u64 {
            (a + b) % 11
        }
        fn double(&self, _: &mut GroupOps, a: u64) -> u64 {
            2 * a % 11
        }
        fn neg(&self, a: u64) -> u64 {
            (11 - a) % 11
        }
        fn key(&self, a: &u64) -> u64 {
            *a
        }
    }
    #[test]
    fn point_mitm_counts_order_repeats_and_identity_intermediates() {
        let base = [1, 3, 8, 10];
        for m in [3, 4] {
            for r in 0..11 {
                let expected = (0..4usize.pow(m as u32))
                    .filter(|&i| {
                        let mut code = i;
                        let mut sum = 0;
                        for _ in 0..m {
                            sum += base[code % 4];
                            code /= 4;
                        }
                        sum % 11 == r
                    })
                    .count() as u64;
                assert_eq!(point_count(&Cyclic, &base, r, m).0, expected);
            }
        }
        assert_eq!(point_count(&Cyclic, &[], 1, 4).0, 0);
    }
    #[test]
    fn target_is_paired_valid_and_nonidentity_on_a_small_real_subgroup() {
        let inst = (0..64)
            .filter_map(|i| random_binary_instance(7, 7788 + i, 16))
            .find(|i| i.a == 0 && full_degree(&i.gf, i.b))
            .unwrap();
        validate(&inst);
        let r = record(&inst);
        let id = r["record_sha256"].as_str().unwrap();
        let mut xs = BTreeSet::new();
        for draw in 0..64 {
            let p = target(&inst, id, 3, 3, draw);
            assert_eq!(p, target(&inst, id, 3, 3, draw));
            xs.insert(p.0.pack());
        }
        assert!(xs.len() > 8);
    }
}
