//! Fixed public E/F_211 non-fibre Jacobian coset-slice experiment.
//! The target scalar is never supplied to the relation collector.
use std::collections::{BTreeMap, HashMap};
use std::time::Instant;

use crypto_lib::cryptanalysis::chained_transfer::bielliptic::{Bielliptic, Quotient};
use crypto_lib::cryptanalysis::chained_transfer::ec::{Curve, Pt};
use crypto_lib::cryptanalysis::chained_transfer::field::{Fp, F2};
use crypto_lib::cryptanalysis::chained_transfer::measure::rho_on_ec;
use crypto_lib::cryptanalysis::chained_transfer::pushforward::{place_image, x_is_square_e1};

const P: u64 = 211;
const R: u64 = 109;
const C: [u64; 4] = [122, 208, 210, 149];
const TRIALS: usize = 128;
const STREAM_SEED: u64 = 0xC0FF_EE13;
const RHO_SEED: u64 = 12_648_439;

fn pt(x: u64, y: u64) -> Pt {
    Some((F2::base(x), F2::base(y)))
}
fn key(q: &Pt) -> (u64, u64) {
    let (x, y) = q.expect("point at infinity in factor base");
    assert!(x.in_base() && y.in_base(), "non-rational projected point");
    (x.a, y.a)
}
fn next(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}
fn mm(x: i64) -> i64 {
    x.rem_euclid(R as i64)
}
fn pow(mut b: i64, mut e: u64) -> i64 {
    let mut a = 1;
    while e > 0 {
        if e & 1 == 1 {
            a = mm(a * b);
        }
        b = mm(b * b);
        e >>= 1;
    }
    a
}
fn inverse(x: i64) -> i64 {
    assert_ne!(mm(x), 0);
    pow(mm(x), R - 2)
}
fn rank(mut rows: Vec<Vec<i64>>, cols: usize) -> usize {
    let mut piv = 0;
    for col in 0..cols {
        if let Some(j) = (piv..rows.len()).find(|&j| mm(rows[j][col]) != 0) {
            rows.swap(piv, j);
            let inv = inverse(rows[piv][col]);
            for k in col..cols {
                rows[piv][k] = mm(rows[piv][k] * inv);
            }
            for j in 0..rows.len() {
                if j != piv {
                    let m = mm(rows[j][col]);
                    for k in col..cols {
                        rows[j][k] = mm(rows[j][k] - m * rows[piv][k]);
                    }
                }
            }
            piv += 1;
            if piv == rows.len() {
                break;
            }
        }
    }
    piv
}

#[derive(Clone)]
struct Row {
    a: u64,
    b: u64,
    p1: (u64, u64),
    p2: (u64, u64),
}

fn main() {
    let all_start = Instant::now();
    let cover = Bielliptic::new(P, C).expect("frozen bielliptic curve rejected");
    let e = cover.e1;
    let g = pt(165, 19);
    let q = pt(178, 46);
    assert_eq!((e.a2, e.a4, e.a6, e.order()), (210, 186, 126, 218));
    assert!(e.on_curve(&g) && e.on_curve(&q));
    assert!(e.mul(&g, R as u128).is_none() && e.mul(&q, R as u128).is_none());
    let e2point = cover
        .e2
        .rational_points()
        .into_iter()
        .flatten()
        .map(Some)
        .find(|x| {
            if cover.e2.mul(x, 2).is_none() {
                return false;
            }
            let k = cover.pull(Quotient::E2, x);
            !k.is_identity() && cover.push(Quotient::E1, &k).is_none()
        })
        .expect("no structural complementary point");
    let k = cover.pull(Quotient::E2, &e2point);
    assert!(!k.is_identity());
    assert!(cover.push(Quotient::E1, &k).is_none());
    assert_eq!(cover.push(Quotient::E2, &k), cover.e2.mul(&e2point, 2));
    // Full natural image of rational degree-one places, projected to the r-part.
    // This dictionary is built before inspecting any candidate relation.
    let mut base = BTreeMap::<(u64, u64), usize>::new();
    for point in e.rational_points() {
        let Some((x, _)) = point else { continue };
        if !x_is_square_e1(&cover, x.a) {
            continue;
        }
        let hpoint = e.mul(&point, 2);
        if hpoint.is_none() {
            continue;
        }
        assert!(e.mul(&hpoint, R as u128).is_none());
        let pk = key(&hpoint);
        if !base.contains_key(&pk) {
            let i = base.len();
            base.insert(pk, i);
        }
    }
    let setup_ns = all_start.elapsed().as_nanos();
    println!("input p={P} r={R} c={C:?} G={:?} Q={:?} e2_point={:?} K_u={:?} K_v={:?} setup_ns={setup_ns}",
        key(&g),key(&q),key(&e2point),k.u_coeffs(),k.v_coeffs());

    let collection_start = Instant::now();
    let mut rng = STREAM_SEED;
    let mut rows = Vec::<Row>::new();
    let mut seen_r = HashMap::<(u64, u64), (u64, u64)>::new();
    let mut collisions = 0usize;
    let mut infinity_sources = 0usize;
    let mut full_fibres = 0usize;
    let mut deg1 = 0usize;
    let mut nonsplit = 0usize;
    let mut split_same = 0usize;
    let mut split_distinct = 0usize;
    let mut projected_distinct = 0usize;
    let mut target_rows = 0usize;
    let mut first_target = None;
    for trial in 0..TRIALS {
        let a = next(&mut rng) % R;
        let b = next(&mut rng) % R;
        let source = e.add(&e.mul(&g, a as u128), &e.mul(&q, b as u128));
        if source.is_none() {
            infinity_sources += 1;
            println!("candidate trial={trial} a={a} b={b} status=infinity");
            continue;
        }
        if let Some(prev) = seen_r.insert(key(&source), (a, b)) {
            collisions += 1;
            if collisions == 1 {
                println!("first_source_collision trial={trial} prev={prev:?} now=({a},{b})");
            }
        }
        let pulled = cover.pull(Quotient::E1, &source);
        let d = cover.jac.add(&pulled, &k);
        assert!(cover.jac.is_valid(&d));
        assert_ne!(d, pulled, "nonzero complement became full fibre");
        if d.degree() != 2 {
            deg1 += 1;
            println!(
                "candidate trial={trial} a={a} b={b} status=degree_{}",
                d.degree()
            );
            continue;
        }
        let u = d.u_coeffs();
        let v = d.v_coeffs();
        let roots: Vec<u64> = (0..P)
            .filter(|&x| (u[0] + u[1] * x + x * x) % P == 0)
            .collect();
        if roots.len() != 2 {
            nonsplit += 1;
            println!("candidate trial={trial} a={a} b={b} status=nonsplit_or_ramified roots={} u={u:?} v={v:?}",roots.len());
            continue;
        }
        let images: Vec<Pt> = roots
            .iter()
            .map(|&x| place_image(&cover, Quotient::E1, x, (v[0] + v[1] * x) % P))
            .collect();
        let (p1, p2) = (images[0], images[1]);
        let image_sum = e.add(&p1, &p2);
        let pushed = cover.push(Quotient::E1, &d);
        let two_r = e.add(&source, &source);
        assert_eq!(pushed, two_r, "complement did not project to zero");
        assert_eq!(image_sum, pushed, "split-place projection mismatch");
        assert!(cover.push(Quotient::E2, &d) == cover.e2.mul(&e2point, 2));
        if d == pulled {
            full_fibres += 1;
        }
        if p1 == p2 {
            split_same += 1;
            println!("candidate trial={trial} a={a} b={b} status=split_same");
            continue;
        }
        split_distinct += 1;
        if p1.is_none() || p2.is_none() {
            println!("candidate trial={trial} a={a} b={b} status=split_infinity_image");
            continue;
        }
        let (h1, h2) = (e.mul(&p1, 2), e.mul(&p2, 2));
        if h1.is_none() || h2.is_none() || h1 == h2 {
            println!("candidate trial={trial} a={a} b={b} status=projected_collapse");
            continue;
        }
        assert!(e.mul(&h1, R as u128).is_none() && e.mul(&h2, R as u128).is_none());
        assert_eq!(e.add(&h1, &h2), e.mul(&source, 4));
        projected_distinct += 1;
        println!("candidate trial={trial} a={a} b={b} status=accepted source={:?} p1={:?} p2={:?} h1={:?} h2={:?}",key(&source),key(&p1),key(&p2),key(&h1),key(&h2));
        if b != 0 {
            target_rows += 1;
            if first_target.is_none() {
                first_target = Some((trial, a, b, key(&p1), key(&p2), key(&h1), key(&h2)));
            }
        }
        rows.push(Row {
            a,
            b,
            p1: key(&h1),
            p2: key(&h2),
        });
    }
    let collection_ns = collection_start.elapsed().as_nanos();
    let nc = base.len();
    let rank_start = Instant::now();
    let mut coeff = Vec::new();
    for row in &rows {
        let mut v = vec![0i64; nc + 2];
        v[base[&row.p1]] += 1;
        v[base[&row.p2]] += 1;
        v[nc] = mm(-4 * (row.b as i64));
        v[nc + 1] = mm(4 * (row.a as i64));
        coeff.push(v);
    }
    let base_rank = rank(coeff.iter().map(|v| v[..nc].to_vec()).collect(), nc);
    let full_rank = rank(coeff.iter().map(|v| v[..nc + 1].to_vec()).collect(), nc + 1);
    let rank_ns = rank_start.elapsed().as_nanos();
    let slice_cold_ns = all_start.elapsed().as_nanos();
    assert!(full_rank <= base_rank + 1);
    println!("slice_summary trials={TRIALS} infinity_sources={infinity_sources} source_collisions={collisions} deg_lt_2={deg1} nonsplit_or_ramified={nonsplit} split_same={split_same} split_distinct={split_distinct} projected_distinct={projected_distinct} full_fibres={full_fibres} target_rows={target_rows} base_columns={nc} base_rank={base_rank} augmented_rank={full_rank} target_rank_gain={} setup_ns={setup_ns} collection_ns={collection_ns} rank_ns={rank_ns} slice_cold_ns={slice_cold_ns}",full_rank-base_rank);
    println!("first_target_witness={first_target:?}");
    // A separately initialized source group and the same fixed G,Q for rho.
    let rho_start = Instant::now();
    let direct = Curve::new(Fp::new(P).unwrap(), 210, 186, 126).unwrap();
    assert!(direct.on_curve(&g) && direct.on_curve(&q));
    let rho = rho_on_ec(&direct, g, q, R, RHO_SEED);
    let rho_ns = rho_start.elapsed().as_nanos();
    let rho_ops = rho.group_ops.adds + rho.group_ops.doubles;
    assert!(rho.recovered.is_some() && direct.mul(&g, rho.recovered.unwrap() as u128) == q);
    println!(
        "rho_summary recovered={:?} verified=true group_ops={rho_ops} cold_ns={rho_ns}",
        rho.recovered
    );
    println!("complete EC Jacobian complementary-coset slice control");
}
