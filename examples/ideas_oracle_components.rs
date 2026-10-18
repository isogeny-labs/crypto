//! Native component certificates for items 4, 16 and 17 of the 2026-10-18 audit.
//! These are correctness certificates, not a four-presentation solver study.
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;

use crypto_lib::binary_ecc::curve::point_add;
use crypto_lib::binary_ecc::{BinaryCurve, BinaryPoint, F2mElement};
use crypto_lib::cryptanalysis::binary_semaev_s4::{AnfF2m, AnfPoly};
use crypto_lib::cryptanalysis::koblitz_index_calculus::{
    find_irreducible_sparse_wide, points_with_x, KoblitzCurve,
};
use crypto_lib::cryptanalysis::wide_gf2m::WideGf2;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use serde_json::{json, Value};

fn fpow(f: &WideGf2, mut a: u128, mut e: u128) -> u128 {
    let mut z = 1;
    while e > 0 {
        if e & 1 != 0 {
            z = f.mul(z, a);
        }
        a = f.sqr(a);
        e >>= 1;
    }
    z
}

fn hex(a: u128) -> String {
    format!("{a:x}")
}

fn rank(xs: &[u128], n: usize) -> usize {
    let mut pivots = vec![0u128; n];
    let mut out = 0;
    for &x in xs {
        let mut v = x;
        while v != 0 {
            let b = 127 - v.leading_zeros() as usize;
            if pivots[b] == 0 {
                pivots[b] = v;
                out += 1;
                break;
            }
            v ^= pivots[b];
        }
    }
    out
}

fn c2_member(f: &WideGf2, roots: &[u128], x: u128) -> bool {
    // C2 uses subsets of cardinality 2: repeated roots are excluded.
    roots
        .iter()
        .any(|&z| x ^ z != z && fpow(f, x ^ z, 167) == 1)
}

fn cyclotomic_n83() -> Value {
    let n = 83;
    let irr = find_irreducible_sparse_wide(n).expect("degree-83 irreducible");
    let f = WideGf2::new(&irr);
    let exponent = ((1u128 << n) - 1) / 167;
    let candidate = (2..4096)
        .find(|&a| fpow(&f, a, exponent) != 1)
        .expect("nontrivial 167th root in frozen candidate range");
    let gamma = fpow(&f, candidate, exponent);
    assert_ne!(gamma, 1);
    assert_eq!(fpow(&f, gamma, 167), 1);
    let roots: Vec<_> = (0..167).map(|i| fpow(&f, gamma, i)).collect();
    let root_set: BTreeSet<_> = roots.iter().copied().collect();
    assert_eq!(root_set.len(), 167);
    let mut onb = Vec::new();
    let mut k = 1;
    for _ in 0..83 {
        onb.push(roots[k] ^ roots[(167 - k) % 167]);
        k = (k * 2) % 167;
    }
    assert_eq!(k, 1);
    let onb_rank = rank(&onb, 83);
    assert_eq!(onb_rank, 83);
    for i in 0..83 {
        assert_eq!(f.sqr(onb[i]), onb[(i + 1) % 83]);
    }
    let mut generic_checks = 0;
    for &r in &roots {
        let generic = F2mElement::from_biguint(&BigUint::from(r), n);
        assert_eq!(generic.square(&irr).to_biguint().to_u128(), Some(f.sqr(r)));
        let g = F2mElement::from_biguint(&BigUint::from(gamma), n);
        assert_eq!(
            generic.mul(&g, &irr).to_biguint().to_u128(),
            Some(f.mul(r, gamma))
        );
        generic_checks += 2;
    }
    let mut c2 = BTreeMap::new();
    let mut hasher = blake3::Hasher::new();
    let mut witnesses = Vec::new();
    for i in 0..167 {
        for j in i + 1..167 {
            let x = roots[i] ^ roots[j];
            assert_eq!(fpow(&f, roots[i], 167), 1);
            assert_eq!(fpow(&f, roots[j], 167), 1);
            c2.entry(x).or_insert((i, j));
        }
    }
    for (&x, &(i, j)) in &c2 {
        hasher.update(&x.to_le_bytes());
        hasher.update(&(i as u32).to_le_bytes());
        hasher.update(&(j as u32).to_le_bytes());
        assert!(c2.contains_key(&f.sqr(x)));
        witnesses.push(json!({"x_hex":hex(x),"i":i,"j":j}));
    }
    for &r in &roots {
        assert!(root_set.contains(&f.sqr(r)));
    }
    let mut state = 20261018u128;
    let mut member_probes = 0;
    for _ in 0..4096 {
        state ^= state << 31;
        state ^= state >> 19;
        state ^= state << 23;
        let x = state & f.mask();
        let answer = c2_member(&f, &roots, x);
        assert_eq!(answer, c2.contains_key(&x));
        member_probes += usize::from(answer);
    }
    // Probe every member with a separate constructive lookup predicate.
    let mut witnessed = 0;
    for &x in c2.keys() {
        assert!(roots
            .iter()
            .any(|&z| z != x ^ z && root_set.contains(&(x ^ z))));
        witnessed += 1;
    }
    json!({"kind":"cyclotomic_component_certificate","n":n,"p":167,
        "field_modulus_hex":hex(f.modulus_bits()),"kernel":f.kernel_name(),
        "candidate":candidate,"gamma_hex":hex(gamma),"gamma_order":167,
        "generic_backend_checks":generic_checks,"onb_rank":onb_rank,
        "onb_hex":onb.iter().map(|&x|hex(x)).collect::<Vec<_>>(),
        "c1_cardinality":roots.len(),"c2_pair_count":167*166/2,
        "c2_cardinality":c2.len(),"c2_witnessed_members":witnessed,
        "frobenius_closed":true,"full_width_probes":4096,
        "probe_members":member_probes,"probe_mismatches":0,
        "c2_certificate_blake3":hasher.finalize().to_hex().to_string(),
        "c2_witnesses":witnesses,"passed":true,
        "scope":"multiword field/base construction and C2 membership only; no subgroup projection, descent solving degree, useful rank or ECDLP cost measured"})
}

// Replay the stored witnesses through the independent generic multiword
// backend. This reads the prior output; it does not trust its passed flag.
fn verify_n83(input: &str) -> Value {
    let raw = fs::read(input).expect("n83 certificate input");
    let data: Value = serde_json::from_slice(&raw).expect("n83 certificate JSON");
    assert_eq!(data["n"], 83);
    assert_eq!(data["p"], 167);
    let irr = find_irreducible_sparse_wide(83).unwrap();
    let f = WideGf2::new(&irr);
    assert_eq!(data["field_modulus_hex"], hex(f.modulus_bits()));
    let gamma_bits =
        BigUint::parse_bytes(data["gamma_hex"].as_str().unwrap().as_bytes(), 16).unwrap();
    let gamma = F2mElement::from_biguint(&gamma_bits, 83);
    assert_ne!(gamma, F2mElement::one(83));
    let mut roots = vec![F2mElement::one(83)];
    for i in 1..167 {
        roots.push(roots[i - 1].mul(&gamma, &irr));
    }
    assert_eq!(roots[166].mul(&gamma, &irr), F2mElement::one(83));
    let mut members = BTreeSet::new();
    let mut hash = blake3::Hasher::new();
    for witness in data["c2_witnesses"].as_array().unwrap() {
        let i = witness["i"].as_u64().unwrap() as usize;
        let j = witness["j"].as_u64().unwrap() as usize;
        assert!(i < j && j < 167);
        let x = roots[i].add(&roots[j]).to_biguint().to_u128().unwrap();
        assert_eq!(witness["x_hex"], hex(x));
        assert!(members.insert(x));
        hash.update(&x.to_le_bytes());
        hash.update(&(i as u32).to_le_bytes());
        hash.update(&(j as u32).to_le_bytes());
    }
    assert_eq!(members.len(), 13861);
    assert_eq!(data["c2_cardinality"], members.len());
    assert_eq!(
        data["c2_certificate_blake3"],
        hash.finalize().to_hex().as_str()
    );
    for &x in &members {
        let gx = F2mElement::from_biguint(&BigUint::from(x), 83);
        let square = gx.square(&irr).to_biguint().to_u128().unwrap();
        assert!(members.contains(&square));
    }
    json!({"kind":"cyclotomic_component_replay","input":input,
        "input_blake3":blake3::hash(&raw).to_hex().to_string(),
        "generic_root_order_checks":167,"generic_witness_checks":members.len(),
        "generic_frobenius_checks":members.len(),"passed":true,
        "scope":"generic-backend replay of all stored C2 witnesses and Frobenius closure; not a descent solve"})
}

#[derive(Clone)]
enum MobiusPoint {
    Infinity,
    Affine {
        u: F2mElement,
        v: F2mElement,
        y: F2mElement,
    },
}

fn encode(p: &BinaryPoint, n: u32) -> MobiusPoint {
    match p {
        BinaryPoint::Infinity => MobiusPoint::Infinity,
        BinaryPoint::Affine { x, y } => MobiusPoint::Affine {
            u: F2mElement::one(n),
            v: x.add(&F2mElement::one(n)),
            y: y.clone(),
        },
    }
}

fn decode(p: &MobiusPoint, c: &BinaryCurve) -> BinaryPoint {
    match p {
        MobiusPoint::Infinity => BinaryPoint::Infinity,
        MobiusPoint::Affine { u, v, y } => BinaryPoint::Affine {
            x: u.add(v).mul(
                &u.flt_inverse(&c.irreducible).expect("valid inverse chart"),
                &c.irreducible,
            ),
            y: y.clone(),
        },
    }
}

// Check the unprojected polynomial graph of addition independently of the
// output coordinate formulas. Branch tests keep denominators nonzero.
fn addition_witness(
    c: &BinaryCurve,
    p: &BinaryPoint,
    q: &BinaryPoint,
    r: &BinaryPoint,
) -> &'static str {
    let irr = &c.irreducible;
    match (p, q) {
        (BinaryPoint::Infinity, _) => {
            assert_eq!(q, r);
            "identity-left"
        }
        (_, BinaryPoint::Infinity) => {
            assert_eq!(p, r);
            "identity-right"
        }
        (BinaryPoint::Affine { x: x1, y: y1 }, BinaryPoint::Affine { x: x2, y: y2 }) => {
            if x1 == x2 && y1.add(y2) == *x1 {
                assert_eq!(*r, BinaryPoint::Infinity);
                return "inverse-or-two-torsion";
            }
            let BinaryPoint::Affine { x: x3, y: y3 } = r else {
                panic!("finite generic/double result")
            };
            assert!(c.is_on_curve(r));
            if x1 == x2 {
                assert_eq!(y1, y2);
                assert!(!x1.is_zero());
                let lambda = x1
                    .square(irr)
                    .add(y1)
                    .mul(&x1.flt_inverse(irr).unwrap(), irr);
                assert!(lambda.mul(x1, irr).add(&x1.square(irr)).add(y1).is_zero());
                assert!(x3.add(&lambda.square(irr)).add(&lambda).add(&c.a).is_zero());
                assert!(y3
                    .add(&x1.square(irr))
                    .add(&lambda.add(&F2mElement::one(c.m)).mul(x3, irr))
                    .is_zero());
                "doubling"
            } else {
                let d = x1.add(x2);
                let lambda = y1.add(y2).mul(&d.flt_inverse(irr).unwrap(), irr);
                assert!(lambda.mul(&d, irr).add(y1).add(y2).is_zero());
                assert!(x3
                    .add(&lambda.square(irr))
                    .add(&lambda)
                    .add(x1)
                    .add(x2)
                    .add(&c.a)
                    .is_zero());
                assert!(y3
                    .add(&lambda.mul(&x1.add(x3), irr))
                    .add(x3)
                    .add(y1)
                    .is_zero());
                "generic"
            }
        }
    }
}

fn key(p: &BinaryPoint) -> String {
    match p {
        BinaryPoint::Infinity => "O".into(),
        BinaryPoint::Affine { x, y } => format!(
            "{}:{}",
            x.to_biguint().to_str_radix(16),
            y.to_biguint().to_str_radix(16)
        ),
    }
}

fn presentation_cert() -> Value {
    let kc = KoblitzCurve::new(0, 7).expect("public K0/GF(2^7)");
    let c = &kc.curve;
    let mut rows = Vec::new();
    for (m, ell) in [(3u32, 4u32), (4, 3)] {
        let xs: Vec<_> = (0u32..1 << ell)
            .map(|v| F2mElement::from_biguint(&BigUint::from(v), 7))
            .collect();
        let points: Vec<_> = xs.iter().flat_map(|x| points_with_x(c, x)).collect();
        let mut with_o = points.clone();
        with_o.push(BinaryPoint::Infinity);
        let mut branches = BTreeMap::<&str, usize>::new();
        let mut poles = 0;
        let mut certificate = blake3::Hasher::new();
        for p in &with_o {
            assert_eq!(decode(&encode(p, c.m), c), *p);
            if matches!(encode(p,c.m),MobiusPoint::Affine { ref v,.. } if v.is_zero()) {
                poles += 1;
            }
            for q in &with_o {
                let r = point_add(c, p, q);
                *branches.entry(addition_witness(c, p, q, &r)).or_default() += 1;
                let transported =
                    point_add(c, &decode(&encode(p, c.m), c), &decode(&encode(q, c.m), c));
                assert_eq!(r, decode(&encode(&transported, c.m), c));
                certificate.update(key(&r).as_bytes());
                certificate.update(b"\n");
            }
        }
        let tuples = points.len().pow(m);
        let mut sums = BTreeSet::new();
        let mut infinity_sums = 0;
        for mut index in 0..tuples {
            let mut sum = BinaryPoint::Infinity;
            let mut transport = MobiusPoint::Infinity;
            for _ in 0..m {
                let p = &points[index % points.len()];
                index /= points.len();
                sum = point_add(c, &sum, p);
                let previous = decode(&transport, c);
                let result = point_add(c, &previous, &decode(&encode(p, c.m), c));
                addition_witness(c, &previous, p, &result);
                transport = encode(&result, c.m);
            }
            assert_eq!(sum, decode(&transport, c));
            infinity_sums += usize::from(sum == BinaryPoint::Infinity);
            let k = key(&sum);
            certificate.update(k.as_bytes());
            certificate.update(b"\n");
            sums.insert(k);
        }
        rows.push(json!({"m":m,"ell":ell,"base_x_words":(0..1u32<<ell).collect::<Vec<_>>(),
            "base_points":points.iter().map(key).collect::<Vec<_>>(),"lifted_points":points.len(),
            "ordered_pair_checks":with_o.len()*with_o.len(),"branch_counts":branches,
            "mobius_pole_points":poles,"ordered_tuple_checks":tuples,"infinity_sums":infinity_sums,
            "distinct_sums":sums.len(),"certificate_blake3":certificate.finalize().to_hex().to_string(),
            "solving_degree":null,"macaulay_columns":null,"passed":true}));
    }
    json!({"kind":"presentation_component_certificate","a":0,"b":1,"n":7,
        "curve_identity":"K_0: y^2+xy=x^3+1 over GF(2^7)",
        "modulus_low_terms":c.irreducible.low_terms,"group_order":kc.group_order.to_string(),
        "target_law":"complete ordered tuple sum census; not planted decompositions",
        "mobius_map":"[X:Z] -> [Z:X+Z]; Y retained; projective pole V=0 included",
        "panels":rows,"passed":true,
        "scope":"coordinate transport and branch-complete addition certificates only; four descended presentations and degree comparison remain unimplemented"})
}

fn summarize(path: &str) -> Value {
    let mut panels = Vec::new();
    for name in ["equivariant-step", "equivariant-oracle"] {
        let raw = fs::read_to_string(format!("{path}/{name}.stdout.jsonl")).unwrap();
        let rows: Vec<Value> = raw
            .lines()
            .filter(|s| !s.trim().is_empty())
            .map(|s| serde_json::from_str(s).expect("native JSONL record"))
            .collect();
        let gates: Vec<_> = rows.iter().map(|r| r["gate"].clone()).collect();
        let gate_true = gates.iter().filter(|v| **v == Value::Bool(true)).count();
        let by_n: BTreeMap<u64, usize> = rows.iter().fold(BTreeMap::new(), |mut out, r| {
            *out.entry(r["n"].as_u64().unwrap()).or_default() += 1;
            out
        });
        panels.push(json!({"name":name,"rows":rows.len(),"gate_true":gate_true,
            "requested_rows":if name=="equivariant-step" {24} else {48},
            "complete":rows.len()==if name=="equivariant-step" {24} else {48},
            "gate_false":gates.iter().filter(|v| **v == Value::Bool(false)).count(),"by_n":by_n,
            "contained_true":rows.iter().filter(|r|r["contained"]==Value::Bool(true)).count(),
            "contained_false":rows.iter().filter(|r|r["contained"]==Value::Bool(false)).count(),
            "source":format!("{name}.stdout.jsonl")}));
    }
    json!({"panels":panels,"isolation":"L0 shared host","performance_claim":"none; incidental CPU/wall fields retained only"})
}

fn symbolic_product(a: &AnfF2m, b: &AnfF2m, c: &BinaryCurve) -> AnfF2m {
    let mut out = a.mul(b);
    out.reduce(c.m, &c.irreducible);
    out
}

fn symbolic_square(a: &AnfF2m, c: &BinaryCurve) -> AnfF2m {
    let mut out = a.square();
    out.reduce(c.m, &c.irreducible);
    out
}

fn curve_equation(x: &AnfF2m, y: &AnfF2m, c: &BinaryCurve) -> AnfF2m {
    symbolic_square(y, c)
        .xor(&symbolic_product(x, y, c))
        .xor(&symbolic_product(&symbolic_square(x, c), x, c))
        .xor(&AnfF2m::from_const(&c.b, c.m))
        .xor(&symbolic_product(
            &AnfF2m::from_const(&c.a, c.m),
            &symbolic_square(x, c),
            c,
        ))
}

fn addition_anf(c: &BinaryCurve, double: bool) -> Vec<AnfPoly> {
    let n = c.m;
    let x1 = AnfF2m::from_vars(0, n as usize);
    let y1 = AnfF2m::from_vars(n, n as usize);
    let (x2, y2, x3, y3, lambda, w) = if double {
        (
            x1.clone(),
            y1.clone(),
            AnfF2m::from_vars(2 * n, n as usize),
            AnfF2m::from_vars(3 * n, n as usize),
            AnfF2m::from_vars(4 * n, n as usize),
            AnfF2m::from_vars(5 * n, n as usize),
        )
    } else {
        (
            AnfF2m::from_vars(2 * n, n as usize),
            AnfF2m::from_vars(3 * n, n as usize),
            AnfF2m::from_vars(4 * n, n as usize),
            AnfF2m::from_vars(5 * n, n as usize),
            AnfF2m::from_vars(6 * n, n as usize),
            AnfF2m::from_vars(7 * n, n as usize),
        )
    };
    let one = AnfF2m::from_const(&F2mElement::one(n), n);
    let a = AnfF2m::from_const(&c.a, n);
    let mut equations = vec![curve_equation(&x1, &y1, c), curve_equation(&x3, &y3, c)];
    if !double {
        equations.push(curve_equation(&x2, &y2, c));
    }
    if double {
        equations.push(
            symbolic_product(&lambda, &x1, c)
                .xor(&symbolic_square(&x1, c))
                .xor(&y1),
        );
        equations.push(x3.xor(&symbolic_square(&lambda, c)).xor(&lambda).xor(&a));
        equations.push(y3.xor(&symbolic_square(&x1, c)).xor(&symbolic_product(
            &lambda.xor(&one),
            &x3,
            c,
        )));
        equations.push(symbolic_product(&x1, &w, c).xor(&one));
    } else {
        let denominator = x1.xor(&x2);
        equations.push(symbolic_product(&lambda, &denominator, c).xor(&y1).xor(&y2));
        equations.push(
            x3.xor(&symbolic_square(&lambda, c))
                .xor(&lambda)
                .xor(&x1)
                .xor(&x2)
                .xor(&a),
        );
        equations.push(
            y3.xor(&symbolic_product(&lambda, &x1.xor(&x3), c))
                .xor(&x3)
                .xor(&y1),
        );
        equations.push(symbolic_product(&denominator, &w, c).xor(&one));
    }
    equations.into_iter().flat_map(|p| p.coeffs).collect()
}

fn assignment(fields: &[&F2mElement], n: u32) -> Vec<bool> {
    fields
        .iter()
        .flat_map(|x| (0..n).map(|b| x.to_biguint().bit(b as u64)))
        .collect()
}

fn unprojected_anf() -> Value {
    let kc = KoblitzCurve::new(0, 7).unwrap();
    let c = &kc.curve;
    let irr = &c.irreducible;
    let all_points: Vec<_> = (0u32..128)
        .flat_map(|x| points_with_x(c, &F2mElement::from_biguint(&BigUint::from(x), 7)))
        .collect();
    assert_eq!(all_points.len() + 1, kc.group_order.to_usize().unwrap());
    let curve_bits = curve_equation(&AnfF2m::from_vars(0, 7), &AnfF2m::from_vars(7, 7), c);
    let mut affine_curve_equation_checks = 0;
    for x in 0u32..128 {
        for y in 0u32..128 {
            let x = F2mElement::from_biguint(&BigUint::from(x), 7);
            let y = F2mElement::from_biguint(&BigUint::from(y), 7);
            let encoded_zero = curve_bits
                .coeffs
                .iter()
                .all(|p| !p.eval(&assignment(&[&x, &y], 7)));
            assert_eq!(encoded_zero, c.is_on_curve(&BinaryPoint::Affine { x, y }));
            affine_curve_equation_checks += 1;
        }
    }
    let generic = addition_anf(c, false);
    let doubling = addition_anf(c, true);
    let serialize = |eqs: &[AnfPoly]| {
        eqs.iter()
            .map(|p| p.monomials().cloned().collect::<Vec<_>>())
            .collect::<Vec<_>>()
    };
    let mut panels = Vec::new();
    for (m, ell) in [(3u32, 4u32), (4, 3)] {
        let base: Vec<_> = (0u32..1 << ell)
            .flat_map(|x| points_with_x(c, &F2mElement::from_biguint(&BigUint::from(x), 7)))
            .collect();
        let mut counts = BTreeMap::<&str, usize>::new();
        let mut hash = blake3::Hasher::new();
        for p in &base {
            for q in &base {
                let (BinaryPoint::Affine { x: x1, y: y1 }, BinaryPoint::Affine { x: x2, y: y2 }) =
                    (p, q)
                else {
                    unreachable!()
                };
                if x1 == x2 && y1.add(y2) == *x1 {
                    *counts
                        .entry("inverse_or_torsion_separate_stratum")
                        .or_default() += 1;
                    continue;
                }
                let double = x1 == x2;
                let (lambda, w) = if double {
                    let w = x1.flt_inverse(irr).unwrap();
                    (x1.square(irr).add(y1).mul(&w, irr), w)
                } else {
                    let w = x1.add(x2).flt_inverse(irr).unwrap();
                    (y1.add(y2).mul(&w, irr), w)
                };
                let eqs = if double { &doubling } else { &generic };
                let expected = point_add(c, p, q);
                let mut roots = 0;
                for r in &all_points {
                    let BinaryPoint::Affine { x: x3, y: y3 } = r else {
                        unreachable!()
                    };
                    let fields = if double {
                        vec![x1, y1, x3, y3, &lambda, &w]
                    } else {
                        vec![x1, y1, x2, y2, x3, y3, &lambda, &w]
                    };
                    let bits = assignment(&fields, 7);
                    let zero = eqs.iter().all(|p| !p.eval(&bits));
                    assert_eq!(
                        zero,
                        *r == expected,
                        "Boolean graph must have exactly the group-law output"
                    );
                    roots += usize::from(zero);
                    hash.update(&[u8::from(zero)]);
                    *counts.entry("pair_output_assignments").or_default() += 1;
                }
                assert_eq!(roots, 1);
                *counts
                    .entry(if double {
                        "doubling_pairs"
                    } else {
                        "generic_pairs"
                    })
                    .or_default() += 1;
            }
        }
        panels.push(
            json!({"m":m,"ell":ell,"base_points":base.len(),"counts":counts,
            "certificate_blake3":hash.finalize().to_hex().to_string(),"passed":true}),
        );
    }
    let stats = |name: &str, eqs: &[AnfPoly], vars| {
        json!({"branch":name,"boolean_variables":vars,"equations":eqs.len(),
        "input_max_degree":eqs.iter().map(AnfPoly::degree).max().unwrap_or(0),"monomials_total":eqs.iter().map(AnfPoly::len).sum::<usize>(),
        "anf_equations":serialize(eqs)})
    };
    json!({"kind":"unprojected_boolean_graph_certificate","n":7,"curve_slug":"icv1-f2m7-t13-616700dd", "all_finite_curve_points":all_points.len(),
        "affine_curve_equation_checks":affine_curve_equation_checks,
        "charts":[stats("generic",&generic,56),stats("doubling",&doubling,42)],"panels":panels,"passed":true,
        "scope":"ANF input-degree and exhaustive finite-curve graph equivalence only; no F4 solving degree or four-presentation comparison"})
}

fn cyclotomic_n83_anf(input: &str) -> Value {
    let raw = fs::read(input).unwrap();
    let data: Value = serde_json::from_slice(&raw).unwrap();
    let n = 83u32;
    let irr = find_irreducible_sparse_wide(n).unwrap();
    let f = WideGf2::new(&irr);
    assert_eq!(data["field_modulus_hex"], hex(f.modulus_bits()));
    let gamma = u128::from_str_radix(data["gamma_hex"].as_str().unwrap(), 16).unwrap();
    assert_ne!(gamma, 1);
    assert_eq!(fpow(&f, gamma, 167), 1);
    let c = BinaryCurve {
        m: n,
        irreducible: irr,
        a: F2mElement::zero(n),
        b: F2mElement::one(n),
        generator: BinaryPoint::Infinity,
        order: BigUint::from(0u32),
        cofactor: BigUint::from(1u32),
    };
    let fields: Vec<_> = (0..18)
        .map(|i| AnfF2m::from_vars(i * n, n as usize))
        .collect();
    let mut equations = Vec::new();
    let one = AnfF2m::from_const(&F2mElement::one(n), n);
    for base in [0usize, 8] {
        for (from, to, power) in [(0usize, 1usize, 1), (1, 2, 1), (2, 3, 3), (3, 4, 2)] {
            let mut image = fields[base + from].clone();
            for _ in 0..power {
                image = symbolic_square(&image, &c);
            }
            equations.push(image.xor(&fields[base + to]));
        }
        for (a, b, out) in [(0usize, 1usize, 5usize), (5, 2, 6), (6, 3, 7)] {
            equations.push(
                symbolic_product(&fields[base + a], &fields[base + b], &c).xor(&fields[base + out]),
            );
        }
        equations.push(symbolic_product(&fields[base + 7], &fields[base + 4], &c).xor(&one));
    }
    equations.push(fields[16].xor(&fields[0]).xor(&fields[8]));
    equations.push(symbolic_product(&fields[16], &fields[17], &c).xor(&one));
    let eqs: Vec<_> = equations.into_iter().flat_map(|p| p.coeffs).collect();
    assert_eq!(eqs.len(), 1494);
    assert_eq!(eqs.iter().map(AnfPoly::degree).max(), Some(2));
    let root_fields = |z| {
        let z2 = f.sqr(z);
        let z4 = f.sqr(z2);
        let z32 = f.sqr_k(z4, 3);
        let z128 = f.sqr_k(z32, 2);
        let t1 = f.mul(z, z2);
        let t2 = f.mul(t1, z4);
        let t3 = f.mul(t2, z32);
        vec![z, z2, z4, z32, z128, t1, t2, t3]
    };
    let bits = |values: &[u128]| {
        values
            .iter()
            .flat_map(|v| (0..83).map(move |b| (v >> b) & 1 != 0))
            .collect::<Vec<_>>()
    };
    let mut checks = Vec::new();
    for witness in data["c2_witnesses"].as_array().unwrap().iter().take(32) {
        let z0 = fpow(&f, gamma, witness["i"].as_u64().unwrap() as u128);
        let z1 = fpow(&f, gamma, witness["j"].as_u64().unwrap() as u128);
        let x = z0 ^ z1;
        assert_ne!(x, 0);
        assert_eq!(witness["x_hex"], hex(x));
        let mut values = root_fields(z0);
        values.extend(root_fields(z1));
        values.extend([x, f.inv(x)]);
        let value_bits = bits(&values);
        let accept = eqs.iter().all(|p| !p.eval(&value_bits));
        assert!(accept);
        values[16] ^= 1;
        let corrupted_bits = bits(&values);
        let rejects_corruption = eqs.iter().any(|p| p.eval(&corrupted_bits));
        assert!(rejects_corruption);
        let mut diagonal = root_fields(z0);
        diagonal.extend(root_fields(z0));
        diagonal.extend([0, 0]);
        let diagonal_bits = bits(&diagonal);
        let rejects_diagonal = eqs.iter().any(|p| p.eval(&diagonal_bits));
        assert!(rejects_diagonal);
        checks.push(json!({"x_hex":hex(x),"accepted":accept,"corrupted_x_rejected":rejects_corruption,"repeated_root_rejected":rejects_diagonal}));
    }
    assert_eq!(checks.len(), 32);
    let mut hash = blake3::Hasher::new();
    for p in &eqs {
        hash.update(&(p.len() as u32).to_le_bytes());
        for m in p.monomials() {
            hash.update(&(m.len() as u32).to_le_bytes());
            for v in m {
                hash.update(&v.to_le_bytes());
            }
        }
    }
    let anf: Vec<_> = eqs
        .iter()
        .map(|p| p.monomials().cloned().collect::<Vec<_>>())
        .collect();
    json!({"kind":"cyclotomic_n83_quadratic_membership_encoding","input":input,
        "input_blake3":blake3::hash(&raw).to_hex().to_string(),"field_modulus_hex":hex(f.modulus_bits()),
        "n":83,"p":167,"predicate":"x=z0+z1; z0^167=z1^167=1; x*w=1",
        "boolean_variables":1494,"boolean_equations":eqs.len(),"input_max_degree":2,
        "monomials_total":eqs.iter().map(AnfPoly::len).sum::<usize>(),"equation_hash_blake3":hash.finalize().to_hex().to_string(),
        "positive_checks":32,"corruption_rejections":32,"diagonal_rejections":32,"checks":checks,
        "wire_layout":"root0 fields0..7,root1 fields8..15,x16,invx17; root fields=z,z^2,z^4,z^32,z^128,z^3,z^7,z^39",
        "anf_equations":anf,"passed":true,"solving_degree":null,
        "scope":"multiword quadratic C2 membership input and correctness controls; no F4 solve, subgroup projection, useful relation rank or security claim"})
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mode = args
        .get(1)
        .expect("cyclotomic-n83|presentation-cert|summarize");
    let output = args.get(2).expect("output JSON file");
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)
        .expect("output must be fresh; prior evidence is never overwritten");
    let result = match mode.as_str() {
        "cyclotomic-n83" => cyclotomic_n83(),
        "verify-n83" => verify_n83(args.get(3).expect("n83 certificate input")),
        "tamper-n83" => {
            let mut data: Value = serde_json::from_slice(
                &fs::read(args.get(3).expect("n83 certificate input")).unwrap(),
            )
            .unwrap();
            data["c2_witnesses"][0]["x_hex"] = json!("0");
            data["test_fixture"] = json!("intentionally invalid witness; must be rejected");
            data
        }
        "unprojected-anf" => unprojected_anf(),
        "cyclotomic-n83-anf" => cyclotomic_n83_anf(args.get(3).expect("n83 certificate input")),
        "presentation-cert" => presentation_cert(),
        "summarize" => summarize(args.get(3).expect("raw run directory")),
        _ => panic!("unknown mode"),
    };
    file.write_all(&serde_json::to_vec_pretty(&result).unwrap())
        .unwrap();
    println!("{}", json!({"mode":mode,"output":output,"written":true}));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn high_word_field_and_generic_backend_agree() {
        let irr = find_irreducible_sparse_wide(83).unwrap();
        let f = WideGf2::new(&irr);
        for a in [1u128 << 80, (1 << 82) | (1 << 65) | 17] {
            for b in [2u128, (1 << 75) | 3] {
                let ga = F2mElement::from_biguint(&BigUint::from(a), 83);
                let gb = F2mElement::from_biguint(&BigUint::from(b), 83);
                assert_eq!(ga.mul(&gb, &irr).to_biguint().to_u128(), Some(f.mul(a, b)));
            }
        }
    }
    #[test]
    fn unprojected_charts_are_quadratic_and_reject_a_wrong_result() {
        let kc = KoblitzCurve::new(0, 7).unwrap();
        let c = &kc.curve;
        let equations = addition_anf(c, true);
        assert_eq!(equations.len(), 42);
        assert_eq!(equations.iter().map(AnfPoly::degree).max(), Some(2));
        let p = &points_with_x(c, &F2mElement::one(7))[0];
        let BinaryPoint::Affine { x, y } = p else {
            unreachable!()
        };
        let result = point_add(c, p, p);
        let BinaryPoint::Affine { x: xr, y: yr } = result else {
            unreachable!()
        };
        let w = x.flt_inverse(&c.irreducible).unwrap();
        let lambda = x.square(&c.irreducible).add(y).mul(&w, &c.irreducible);
        assert!(equations
            .iter()
            .all(|p| !p.eval(&assignment(&[x, y, &xr, &yr, &lambda, &w], 7))));
        let wrong_x = xr.add(&F2mElement::one(7));
        assert!(equations
            .iter()
            .any(|p| p.eval(&assignment(&[x, y, &wrong_x, &yr, &lambda, &w], 7))));
        let generic = addition_anf(c, false);
        assert_eq!(generic.len(), 49);
        assert_eq!(generic.iter().map(AnfPoly::degree).max(), Some(2));
    }
    #[test]
    fn mobius_pole_identity_inverse_and_doubling_are_retained() {
        let kc = KoblitzCurve::new(0, 7).unwrap();
        let c = &kc.curve;
        let ps = points_with_x(c, &F2mElement::one(7));
        assert!(!ps.is_empty());
        for p in ps {
            assert!(matches!(encode(&p,7),MobiusPoint::Affine {ref v,..} if v.is_zero()));
            assert_eq!(decode(&encode(&p, 7), c), p);
            let neg = crypto_lib::binary_ecc::curve::point_neg(&p);
            assert_eq!(
                addition_witness(c, &p, &neg, &point_add(c, &p, &neg)),
                "inverse-or-two-torsion"
            );
            assert_eq!(
                addition_witness(c, &p, &p, &point_add(c, &p, &p)),
                "doubling"
            );
            assert_eq!(
                addition_witness(c, &BinaryPoint::Infinity, &p, &p),
                "identity-left"
            );
        }
    }
}
