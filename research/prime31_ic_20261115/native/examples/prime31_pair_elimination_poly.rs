//! Fixed-pair algebraic elimination for the existing prime31 m=8 S4/S3 IC.
//!
//! Exactly eliminate the two depth-six Dickson-fibre variables x6,x7 from
//! S3(x6,x7,w)=0 over F_p, and retain algebraic preimages for later point-sign
//! verification.  This computes the squarefree degree-961 pair-abscissa
//! polynomial H(w). It is NOT a target-dependent relation oracle or ECDLP.
//!
//! Reuses the existing Rust prime31 Arithmetic, Base and curve certificate.
//! All pair roots are obtained from the S3 quadratic over F_p; EC additions
//! appear only in the independent semantic validator, never root discovery.
//! Standing: producer-development [pcc:unverified].
use prime31_algebraic_ic::{check_curve, Arithmetic, Base, PRIME31};
use serde_json::json;
use std::collections::BTreeMap;

const P: u64 = 2_147_483_647;
const A: u64 = 847_514_846;
const B: u64 = 2_087_444_724;
const DEPTH: usize = 6;
const BRANCHES: [usize; 7] = [16, 20, 62, 124, 240, 468, 31];

fn add(x: u64, y: u64) -> u64 {
    let z = x + y;
    if z >= P { z - P } else { z }
}
fn sub(x: u64, y: u64) -> u64 {
    if x >= y { x - y } else { x + P - y }
}
fn mul(x: u64, y: u64) -> u64 {
    ((u128::from(x) * u128::from(y)) % u128::from(P)) as u64
}
fn pow(mut x: u64, mut k: u64) -> u64 {
    let mut y = 1;
    while k != 0 {
        if k & 1 == 1 { y = mul(y, x); }
        x = mul(x, x);
        k >>= 1;
    }
    y
}
fn sqrt(z: u64) -> Option<u64> {
    let s = pow(z, (P + 1) / 4);
    (mul(s, s) == z % P).then_some(s)
}
fn dickson(x: u64) -> u64 {
    sub(mul(x, x), 2)
}
fn branch(mut x: u64, mut y: u64) -> Result<usize, String> {
    if x == y { return Ok(6); } // diagonal: doubled point, not the identity
    for j in 0..DEPTH {
        if add(x, y) == 0 { return Ok(j); }
        x = dickson(x);
        y = dickson(y);
    }
    Err("a distinct same-fibre pair has no first-collision branch".into())
}
// Explicit Semaev S3(x,y,w)=a*w^2+b*w+c; no elliptic-curve group
// operations or known discrete logarithms are used to find its roots.
// The x=y case is linear (finite x(2P)); x(P+(-P))=O is excluded.
fn s3_roots(x: u64, y: u64) -> Result<Vec<u64>, String> {
    let xy = mul(x, y);
    let a = mul(sub(x, y), sub(x, y));
    let b = sub(0, mul(2, add(mul(add(x, y), add(xy, A)), mul(2, B))));
    let c = sub(mul(sub(xy, A), sub(xy, A)), mul(4, mul(B, add(x, y))));
    if a == 0 {
        if b == 0 { return Err("degenerate S3 diagonal".into()); }
        return Ok(vec![mul(sub(0, c), pow(b, P - 2))]);
    }
    let disc = sub(mul(b, b), mul(4, mul(a, c)));
    let root = sqrt(disc).ok_or("S3 discriminant is nonsquare")?;
    let inv = pow(mul(2, a), P - 2);
    let mut out = vec![mul(sub(root, b), inv), mul(sub(sub(0, b), root), inv)];
    out.sort_unstable();
    out.dedup();
    if out.len() != 2 { return Err("S3 distinct-pair roots merged".into()); }
    Ok(out)
}
// Low-to-high monic polynomial coefficients over F_p.
// Each factor (W-w) is inserted by field operations only, in O(d^2) work.
fn from_roots(roots: &[u64]) -> Vec<u64> {
    let mut p = vec![1u64];
    for &w in roots {
        let mut next = vec![0u64; p.len() + 1];
        for (i, &c) in p.iter().enumerate() {
            next[i] = sub(next[i], mul(c, w));
            next[i + 1] = add(next[i + 1], c);
        }
        p = next;
    }
    p
}
fn poly_mul(a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut ans = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            ans[i + j] = add(ans[i + j], mul(x, y));
        }
    }
    ans
}
fn eval(p: &[u64], x: u64) -> u64 {
    p.iter().rev().fold(0, |acc, &c| add(mul(acc, x), c))
}
// Portable FNV-1a checksum of sorted 32-bit little-endian abscissae.
// SHA-256 of the same bytes is recorded in the independent Python replay.
fn fnv_le32(roots: &[u64]) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for &w in roots {
        for byte in (w as u32).to_le_bytes() {
            h = (h ^ u64::from(byte)).wrapping_mul(0x100_0000_01b3);
        }
    }
    h
}

fn main() -> Result<(), String> {
    if std::env::args().len() != 1 {
        return Err("usage: prime31_pair_elimination_poly".into());
    }
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar)?;
    if PRIME31.p != P || PRIME31.a != A || PRIME31.b != B
        || PRIME31.n != 2_147_446_831 {
        return Err("prime31 curve certificate changed".into());
    }
    let fb = Base::build(PRIME31, DEPTH, &mut ar)?;
    if fb.terminal != 933_677_148 || fb.points.len() != 31 {
        return Err("depth-six Dickson factor base drift".into());
    }

    let mut preimages: BTreeMap<u64, (usize, usize, usize)> = BTreeMap::new();
    let mut branch_roots: [Vec<u64>; 7] = std::array::from_fn(|_| Vec::new());
    let mut pair_count = 0usize;
    let mut semantic_checks = 0usize;
    for (i, &(x, ay)) in fb.points.iter().enumerate() {
        for (j, &(y, by)) in fb.points.iter().enumerate().skip(i) {
            pair_count += 1;
            let arm = branch(x, y)?;
            let roots = s3_roots(x, y)?;
            let expected_roots = if i == j { 1 } else { 2 };
            if roots.len() != expected_roots {
                return Err("S3 degree/diagonal root count changed".into());
            }
            for (k, &w) in roots.iter().enumerate() {
                if preimages.insert(w, (i, j, k)).is_some() {
                    return Err(format!("pair-abscissa preimage collision at {w}"));
                }
                branch_roots[arm].push(w);
            }
            // Only AFTER purely polynomial root discovery: check group semantics.
            let p_i = Some((x, ay));
            let p_j = Some((y, by));
            let plus = ar.add(PRIME31, p_i, p_j)
                .ok_or("unexpected group identity for positive pair")?;
            let mut ec_roots = vec![plus.0];
            if i != j {
                let minus = ar.add(PRIME31, p_i, Some((y, sub(0, by))))
                    .ok_or("unexpected group identity for distinct negative pair")?;
                ec_roots.push(minus.0);
            }
            ec_roots.sort_unstable();
            if roots != ec_roots {
                return Err(format!("S3 algebraic roots fail signed EC check at ({i},{j})"));
            }
            semantic_checks += roots.len();
        }
    }
    if pair_count != 496 || semantic_checks != 961 || preimages.len() != 961 {
        return Err("S3 finite pair projection cardinality changed".into());
    }
    let mut polys = Vec::new();
    let mut branch_data = Vec::new();
    for (j, roots) in branch_roots.iter_mut().enumerate() {
        roots.sort_unstable();
        if roots.len() != BRANCHES[j] {
            return Err(format!("Dickson first-collision branch {j} changed"));
        }
        let poly = from_roots(roots);
        if poly.len() != roots.len() + 1 || poly.last() != Some(&1)
            || roots.iter().any(|&w| eval(&poly, w) != 0) {
            return Err(format!("branch {j} polynomial certificate invalid"));
        }
        branch_data.push(json!({
            "branch": if j == 6 { "diagonal".to_string() } else { j.to_string() },
            "degree": roots.len(),
            "first_root": roots[0],
            "last_root": roots[roots.len() - 1]
        }));
        polys.push(poly);
    }
    let all_roots: Vec<u64> = preimages.keys().copied().collect();
    let h = from_roots(&all_roots);
    let mut product = vec![1u64];
    for part in &polys { product = poly_mul(&product, part); }
    if product != h || h.len() != 962 || h.last() != Some(&1)
        || h[0..6] != [1_384_764_775, 1_730_047_243, 2_061_394_966,
                        644_849_095, 1_634_675_601, 1_780_857_477]
        || fnv_le32(&all_roots) != 0x440e_f47d_5343_1251
        || all_roots.iter().any(|&w| eval(&h, w) != 0) {
        return Err("squarefree global pair polynomial failed exact factor/corpus check".into());
    }
    let outsiders = (0..256u64)
        .filter(|w| !preimages.contains_key(w))
        .filter(|&w| eval(&h, w) != 0)
        .count();
    if outsiders != 256 { return Err("negative polynomial controls failed".into()); }
    println!("{}", json!({
        "standing":"[pcc:unverified]",
        "kind":"algebraic fixed-pair existential elimination; NOT full ECDLP",
        "reused":"existing Rust Base::build, Arithmetic and PRIME31",
        "p":P,"n":PRIME31.n,
        "liftable_base":fb.points.len(),"unordered_pairs_including_diagonal":pair_count,
        "pair_x_provenance_collision_free":true,
        "algebraic_finite_s3_roots":semantic_checks,
        "global_polynomial_degree":h.len()-1,
        "first_six_coefficients_low_to_high":&h[..6],
        "sorted_roots_fnv1a_le32":format!("{:016x}",fnv_le32(&all_roots)),
        "branch_polynomials_multiply_to_global":true,
        "witness_sign_validation_group_additions":"post-construction check only",
        "finite_nonmember_controls":outsiders,
        "branches":branch_data,
        "caution":"m8 target-dependent S4 extraction and cold ECDLP cost still unmeasured"
    }));
    Ok(())
}
