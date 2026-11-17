//! EXP-R31-RR9: classical elliptic Riemann--Roch L(9O) determinant gate.
//!
//! This is a **relation predicate on GIVEN signed leaves**, not a point
//! extractor, not an unknown-scalar ECDLP solver, and not new complexity.
//! Nine inputs MUST be pairwise distinct finite points. Repetition without
//! elliptic Hermite jets yields false determinant-zero positives.
//!
//! Native pinned-source run:
//! cargo +1.100.0 test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_rr9_evaluation
//! cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_rr9_evaluation
use prime31_algebraic_ic::{check_curve, Arithmetic, Base, Point, PRIME31};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

const P: u64 = 2_147_483_647;
const NCASES: usize = 32;
const CAP: usize = 10_000;
const SEED: u64 = 0x5252_395f_4445_5447;

#[inline]
fn fp_add(a: u64, b: u64) -> u64 {
    let s = a + b;
    if s >= P { s - P } else { s }
}
#[inline]
fn fp_sub(a: u64, b: u64) -> u64 {
    if a >= b { a - b } else { a + P - b }
}
#[inline]
fn fp_mul(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % P as u128) as u64
}
fn fp_pow(mut a: u64, mut e: u64) -> u64 {
    let mut v = 1;
    while e > 0 {
        if e & 1 == 1 { v = fp_mul(v, a); }
        a = fp_mul(a, a);
        e >>= 1;
    }
    v
}
fn neg(q: Point) -> Point {
    q.map(|(x, y)| (x, if y == 0 { 0 } else { P - y }))
}
fn splitmix64(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// Basis for L(9 O), ordered by pole orders 0,2,3,4,5,6,7,8,9.
fn evaluation_row(q: Point) -> Result<[u64; 9], String> {
    let (x, y) = q.ok_or("infinity has no affine evaluation row")?;
    if x >= P || y >= P { return Err("noncanonical affine coordinates".into()); }
    let x2 = fp_mul(x, x);
    let x3 = fp_mul(x2, x);
    let x4 = fp_mul(x3, x);
    Ok([1, x, y, x2, fp_mul(x, y), x3, fp_mul(x2, y), x4, fp_mul(x3, y)])
}

/// Modular Gaussian determinant, no elliptic group arithmetic or secret scalars.
fn determinant(mut a: [[u64; 9]; 9]) -> u64 {
    let mut d = 1;
    for j in 0..9 {
        let pivot = (j..9).find(|&i| a[i][j] != 0);
        let Some(i) = pivot else { return 0; };
        if i != j {
            a.swap(j, i);
            d = fp_sub(0, d);
        }
        let pj = a[j][j];
        d = fp_mul(d, pj);
        let inv = fp_pow(pj, P - 2);
        for r in j + 1..9 {
            if a[r][j] == 0 { continue; }
            let scale = fp_mul(a[r][j], inv);
            for c in j + 1..9 {
                a[r][c] = fp_sub(a[r][c], fp_mul(scale, a[j][c]));
            }
            a[r][j] = 0;
        }
    }
    d
}

/// Multiply truncated power series over Fp; coefficients, not derivatives.
fn series_product(a: &[u64], b: &[u64]) -> Vec<u64> {
    assert_eq!(a.len(), b.len());
    let mut out = vec![0u64; a.len()];
    for i in 0..a.len() {
        for j in 0..(a.len()-i) {
            out[i+j] = fp_add(out[i+j], fp_mul(a[i], b[j]));
        }
    }
    out
}

/// The coefficient of t^order in each L(9O) basis monomial, where
/// t=x-x(P), y(t)^2=(x(P)+t)^3+a(x(P)+t)+b.
/// Valid for any affine P with y(P) != 0 and 0 <= order <= 8.
fn evaluation_higher_jet(q: Point, order: usize) -> Result<[u64; 9], String> {
    if order >= 9 { return Err("RR9 has only nine rows".into()); }
    if order == 0 { return evaluation_row(q); }
    let (x, y) = q.ok_or("infinity needs an O-local parameter")?;
    if x >= P || y >= P || y == 0 {
        return Err("noncanonical point or x is not uniformizer at y=0".into());
    }
    let len = order+1;
    let mut one = vec![0u64; len]; one[0] = 1;
    let mut xs = vec![0u64; len]; xs[0] = x; xs[1] = 1;
    let mut ys = vec![0u64; len]; ys[0] = y;
    let x2 = fp_mul(x, x);
    let inv_two_y = fp_pow(fp_mul(2, y), P-2);
    for k in 1..len {
        let c = match k {
            1 => fp_add(fp_mul(3, x2), PRIME31.a),
            2 => fp_mul(3, x),
            3 => 1,
            _ => 0,
        };
        let mut cross = 0u64;
        for j in 1..k {
            cross = fp_add(cross, fp_mul(ys[j], ys[k-j]));
        }
        ys[k] = fp_mul(fp_sub(c, cross), inv_two_y);
    }
    let xx2 = series_product(&xs, &xs);
    let xx3 = series_product(&xx2, &xs);
    let xx4 = series_product(&xx3, &xs);
    let xy = series_product(&xs, &ys);
    let x2y = series_product(&xx2, &ys);
    let x3y = series_product(&xx3, &ys);
    let cols = [one, xs, ys, xx2, xy, xx3, x2y, xx4, x3y];
    Ok(std::array::from_fn(|i| cols[i][order]))
}

/// Return a false-zero risk explicitly, never interpret a diagonal determinant.
fn relation_determinant(leaves: &[Point; 8], residual: Point) -> Result<u64, String> {
    let mut seen = HashSet::new();
    for q in leaves.iter().copied().chain(std::iter::once(neg(residual))) {
        if q.is_none() || !seen.insert(q) {
            return Err("non-distinct or infinite evaluation divisor: Hermite jets required".into());
        }
    }
    let mut a = [[0u64; 9]; 9];
    for (i, &q) in leaves.iter().enumerate() { a[i] = evaluation_row(q)?; }
    a[8] = evaluation_row(neg(residual))?;
    Ok(determinant(a))
}

/// Order-one jet along the elliptic curve in local parameter x-x(P).
/// Valid at affine P with y != 0: dy/dx = (3x^2+a)/(2y).
fn evaluation_tangent_jet(q: Point) -> Result<[u64; 9], String> {
    let (x, y) = q.ok_or("infinity has no finite local x parameter")?;
    if x >= P || y >= P || y == 0 {
        return Err("noncanonical point or local x not uniformizer at y=0".into());
    }
    let x2 = fp_mul(x, x);
    let x3 = fp_mul(x2, x);
    let dy = fp_mul(
        fp_add(fp_mul(3, x2), PRIME31.a),
        fp_pow(fp_mul(2, y), P - 2),
    );
    Ok([
        0, 1, dy,
        fp_mul(2, x),
        fp_add(y, fp_mul(x, dy)),
        fp_mul(3, x2),
        fp_add(fp_mul(fp_mul(2, x), y), fp_mul(x2, dy)),
        fp_mul(4, x3),
        fp_add(fp_mul(fp_mul(3, x2), y), fp_mul(x3, dy)),
    ])
}

/// Confluent L(9O) matrix for all rational affine multiplicities <=9.
/// On this prime-order, odd-order curve there are no rational y=0 points:
/// such a point would be nontrivial rational 2-torsion.
/// Infinity remains a separate unsupported branch, not a false positive.
fn relation_determinant_hermite(
    leaves: &[Point; 8], residual: Point,
) -> Result<u64, String> {
    let mut a = [[0u64; 9]; 9];
    let mut seen: HashMap<Point, usize> = HashMap::new();
    for (i, q) in leaves.iter().copied()
        .chain(std::iter::once(neg(residual))).enumerate()
    {
        if q.is_none() { return Err("effective divisor includes infinity".into()); }
        let entry = seen.entry(q).or_insert(0);
        *entry += 1;
        let order = *entry - 1;
        a[i] = if order == 1 {
            evaluation_tangent_jet(q)?
        } else {
            evaluation_higher_jet(q, order)?
        };
    }
    Ok(determinant(a))
}

/// Compute the unique (projective) F=U(x)+y V(x) in L(9O) vanishing
/// on the GIVEN length-eight affine divisor, including Hermite multiplicity.
/// This is coefficient interpolation, NOT target-conditioned factor extraction.
fn rr9_leaves_kernel(leaves: &[Point; 8]) -> Result<[u64; 9], String> {
    let mut rows = [[0u64; 9]; 8];
    let mut seen: HashMap<Point, usize> = HashMap::new();
    for (i, &q) in leaves.iter().enumerate() {
        if q.is_none() { return Err("kernel divisor includes infinity".into()); }
        let order = seen.entry(q).or_insert(0);
        rows[i] = evaluation_higher_jet(q, *order)?;
        *order += 1;
    }
    let mut pivots = [usize::MAX; 8];
    let mut rank = 0usize;
    for col in 0..9 {
        if rank == 8 { break; }
        let Some(pivot) = (rank..8).find(|&i| rows[i][col] != 0) else {
            continue;
        };
        rows.swap(rank, pivot);
        let inv = fp_pow(rows[rank][col], P - 2);
        for j in col..9 { rows[rank][j] = fp_mul(rows[rank][j], inv); }
        for i in 0..8 {
            if i == rank { continue; }
            let scale = rows[i][col];
            for j in col..9 {
                rows[i][j] = fp_sub(rows[i][j], fp_mul(scale, rows[rank][j]));
            }
        }
        pivots[rank] = col;
        rank += 1;
    }
    if rank != 8 { return Err(format!("Riemann-Roch length-eight rank is {rank}, not 8")); }
    let free = (0..9).find(|j| !pivots.contains(j))
        .ok_or("missing free coefficient")?;
    let mut out = [0u64; 9];
    out[free] = 1;
    for i in 0..8 {
        out[pivots[i]] = fp_sub(0, rows[i][free]);
    }
    let mut again: HashMap<Point, usize> = HashMap::new();
    for &q in leaves {
        let order = again.entry(q).or_insert(0);
        let row = evaluation_higher_jet(q, *order)?;
        let v = (0..9).fold(0, |acc, i| fp_add(acc, fp_mul(row[i], out[i])));
        if v != 0 { return Err("interpolated kernel fails its supplied leaf jet".into()); }
        *order += 1;
    }
    Ok(out)
}

/// Small original-field polynomial convolution, not group arithmetic.
fn rr9_poly_mul(a: &[u64], b: &[u64]) -> Vec<u64> {
    assert!(!a.is_empty() && !b.is_empty());
    let mut c = vec![0u64; a.len() + b.len() - 1];
    for (i, &ai) in a.iter().enumerate() {
        for (j, &bj) in b.iter().enumerate() {
            c[i+j] = fp_add(c[i+j], fp_mul(ai, bj));
        }
    }
    c
}
fn rr9_function_at(coeff: &[u64; 9], q: Point) -> Result<u64, String> {
    let row = evaluation_row(q)?;
    Ok((0..9).fold(0, |acc, i| fp_add(acc, fp_mul(row[i], coeff[i]))))
}

/// Norm under y -> -y: U(X)^2 - (X^3+aX+b)*V(X)^2.
fn rr9_norm(coeff: &[u64; 9]) -> [u64; 10] {
    let u = [coeff[0], coeff[1], coeff[3], coeff[5], coeff[7]];
    let v = [coeff[2], coeff[4], coeff[6], coeff[8]];
    let uu = rr9_poly_mul(&u, &u);
    let cc = [PRIME31.b, PRIME31.a, 0u64, 1u64];
    let cvv = rr9_poly_mul(&cc, &rr9_poly_mul(&v, &v));
    assert_eq!(uu.len(), 9);
    assert_eq!(cvv.len(), 10);
    std::array::from_fn(|i| fp_sub(*uu.get(i).unwrap_or(&0), cvv[i]))
}

/// Exact degree-nine coefficient identity, even for repeated leaves.
/// The signed F(-R) evaluation is separately checked: x-norm alone
/// deliberately forgets the orientation.
fn rr9_norm_certificate(
    leaves: &[Point; 8], residual: Point, wrong_residual: Point,
) -> Result<Value, String> {
    let coeff = rr9_leaves_kernel(leaves)?;
    let (rx, _) = residual.ok_or("norm certificate R=O needs infinity branch")?;
    let mut product = vec![1u64];
    for &q in leaves.iter().chain(std::iter::once(&residual)) {
        let (x, _) = q.ok_or("norm certificate has infinity among nine points")?;
        product = rr9_poly_mul(&product, &[fp_sub(0, x), 1]);
    }
    if product.len() != 10 { return Err("norm factor polynomial degree is not nine".into()); }
    let norm = rr9_norm(&coeff);
    let leading = fp_sub(0, fp_mul(coeff[8], coeff[8]));
    if leading == 0 { return Err("V_3=0; ninth zero is at infinity".into()); }
    if norm[9] != leading { return Err("norm polynomial leading coefficient mismatch".into()); }
    let factored: [u64; 10] = std::array::from_fn(|i| fp_mul(product[i], leading));
    if norm != factored {
        return Err("coefficient norm factorization fails supplied 9-point divisor".into());
    }
    if rr9_function_at(&coeff, neg(residual))? != 0 {
        return Err("norm x-root has wrong F(-R) orientation".into());
    }
    if rr9_function_at(&coeff, neg(wrong_residual))? == 0 {
        return Err("F unexpectedly vanishes at known wrong residual".into());
    }
    Ok(json!({
        "verified":true, "residual_x":rx,
        "kernel_coefficients_L9O":coeff,
        "norm_coefficients_ascending":norm,
        "factorization_leading_coefficient":leading,
        "signed_orientation_true":true, "signed_orientation_wrong":false
    }))
}

/// Ordinary polynomial remainder modulo a monic polynomial (ascending coefficients).
/// This performs no elliptic additions, square-root searches or support enumeration.
fn rr9_poly_rem_monic(mut numerator: Vec<u64>, divisor: &[u64]) -> Result<Vec<u64>, String> {
    if divisor.len() < 2 || divisor[divisor.len()-1] != 1 {
        return Err("polynomial divisor must be monic of positive degree".into());
    }
    if numerator.iter().chain(divisor.iter()).any(|&x| x >= P) {
        return Err("noncanonical polynomial coefficient".into());
    }
    while numerator.len() >= divisor.len() {
        let lead = *numerator.last().unwrap();
        let shift = numerator.len() - divisor.len();
        if lead != 0 {
            for j in 0..(divisor.len()-1) {
                numerator[shift+j] = fp_sub(numerator[shift+j], fp_mul(lead, divisor[j]));
            }
        }
        numerator.pop(); // top coefficient cancelled against monic divisor
    }
    numerator.resize(divisor.len()-1, 0);
    Ok(numerator)
}

/// H^exponent mod monic M; O(log exponent) polynomial multiplications.
/// This does NOT find M from a public residual, so it is not an ECDLP oracle.
fn rr9_poly_pow_mod(
    base: &[u64], mut exponent: u32, modulus: &[u64],
) -> Result<Vec<u64>, String> {
    let mut acc = vec![1u64];
    let mut power = rr9_poly_rem_monic(base.to_vec(), modulus)?;
    while exponent > 0 {
        if exponent & 1 == 1 {
            acc = rr9_poly_rem_monic(rr9_poly_mul(&acc, &power), modulus)?;
        }
        exponent >>= 1;
        if exponent > 0 {
            power = rr9_poly_rem_monic(rr9_poly_mul(&power, &power), modulus)?;
        }
    }
    acc.resize(modulus.len()-1, 0);
    Ok(acc)
}

/// Polynomial quotient/remainder over the *original* prime field.
//// All inputs are ascending order; zero polynomial has representation [0].
/// No elliptic point enumeration or discrete-log arithmetic.
fn rr9_poly_trim(mut v: Vec<u64>) -> Vec<u64> {
    while v.len() > 1 && v.last() == Some(&0) { v.pop(); }
    v
}
fn rr9_poly_div_rem(numerator: &[u64], denominator: &[u64])
    -> Result<(Vec<u64>, Vec<u64>), String>
{
    if numerator.is_empty() || denominator.is_empty()
        || numerator.iter().chain(denominator.iter()).any(|&z| z >= P)
    {
        return Err("empty or noncanonical polynomial".into());
    }
    let mut n = rr9_poly_trim(numerator.to_vec());
    let d = rr9_poly_trim(denominator.to_vec());
    if d == [0] { return Err("division by zero polynomial".into()); }
    let inv_lc = fp_pow(*d.last().unwrap(), P - 2);
    let mut q = vec![0u64; n.len().saturating_sub(d.len()) + 1];
    while n != [0] && n.len() >= d.len() {
        let offset = n.len() - d.len();
        let c = fp_mul(*n.last().unwrap(), inv_lc);
        q[offset] = c;
        for (j, &dj) in d.iter().enumerate() {
            n[offset+j] = fp_sub(n[offset+j], fp_mul(c, dj));
        }
        n = rr9_poly_trim(n);
    }
    Ok((rr9_poly_trim(q), n))
}

/// Make gcd monic to identify *distinct* factors, including multiplicities.
fn rr9_poly_gcd_monic(left: &[u64], right: &[u64]) -> Result<Vec<u64>, String> {
    if left.is_empty() || right.is_empty() {
        return Err("empty polynomial in gcd".into());
    }
    let mut a = rr9_poly_trim(left.to_vec());
    let mut b = rr9_poly_trim(right.to_vec());
    while b != [0] {
        let (_, rem) = rr9_poly_div_rem(&a, &b)?;
        a = b;
        b = rem;
    }
    if a == [0] { return Err("gcd(0,0) undefined".into()); }
    let inv_lc = fp_pow(*a.last().unwrap(), P-2);
    Ok(a.iter().map(|&z| fp_mul(z, inv_lc)).collect())
}

/// Characteristic p>8, monic degree-eight M and squarefree H:
/// M | H^8 iff rad(M)=M/gcd(M,M') divides H.
/// This membership oracle is exact for all eight-root multiplicity partitions.
/// It DOES NOT recover M or the seven signed-chart coefficients from public R.
fn rr9_radical_support(m: &[u64], h: &[u64]) -> Result<(bool, usize), String> {
    if m.len() != 9 || m[8] != 1 || h.len() < 2 || h.last() != Some(&1)
        || m.iter().chain(h.iter()).any(|&z| z >= P)
    {
        return Err("expected canonical monic degree-eight M and squarefree monic H".into());
    }
    let hd: Vec<u64> = (1..h.len())
        .map(|i| fp_mul(h[i], i as u64 % P)).collect();
    if rr9_poly_gcd_monic(h, &hd)? != [1] {
        return Err("factor-base x polynomial H is not squarefree".into());
    }
    let md: Vec<u64> = (1..m.len())
        .map(|i| fp_mul(m[i], i as u64)).collect();
    let gcd = rr9_poly_gcd_monic(m, &md)?;
    let (rad, remainder) = rr9_poly_div_rem(m, &gcd)?;
    if remainder != [0] || rad.last() != Some(&1) || rad.len() < 2 {
        return Err("invalid monic squarefree radical of degree-eight M".into());
    }
    let (_, h_rem) = rr9_poly_div_rem(h, &rad)?;
    Ok((h_rem == [0], rad.len()-1))
}

/// Public coefficient-only support test (NOT a coefficient extractor).
///
/// Input F=U+yV is already supplied, as are public R and H_B. The test
/// F(-R)=0, deg N_F=9, and [N_F / (-v3^2 (X-x(R)))] | H_B^8
/// is equivalent to F's other eight zeros being supported at +/-base points
/// (with multiplicities), under the stated ordinary prime31 curve assumptions.
/// No signed leaves, group arithmetic, baby/giant tables or secret logs enter.
fn rr9_public_norm_support(
    coeff: &[u64; 9], residual: Point, h: &[u64],
) -> Result<bool, String> {
    let (rx, ry) = residual.ok_or("R=O requires the infinity branch")?;
    if rx >= P || ry >= P || coeff.iter().any(|&x| x >= P) {
        return Err("noncanonical public input".into());
    }
    if h.len() < 2 || h.last() != Some(&1) {
        return Err("factor-base polynomial H must be monic".into());
    }
    let rhs = fp_add(fp_add(fp_mul(fp_mul(rx, rx), rx), fp_mul(PRIME31.a, rx)), PRIME31.b);
    if fp_mul(ry, ry) != rhs { return Err("residual off original curve".into()); }
    if rr9_function_at(coeff, neg(residual))? != 0 { return Ok(false); }
    let norm = rr9_norm(coeff);
    let lead = norm[9]; // -v3^2; zero means a lower pole-order branch
    if lead == 0 { return Ok(false); }
    // Synthetic division by (X-rx); a nonzero remainder is a false fibre.
    let mut m = [0u64; 9];
    m[8] = norm[9];
    for i in (0..8).rev() {
        m[i] = fp_add(norm[i+1], fp_mul(rx, m[i+1]));
    }
    if fp_add(norm[0], fp_mul(rx, m[0])) != 0 { return Ok(false); }
    let inverse_lead = fp_pow(lead, P-2);
    for c in &mut m { *c = fp_mul(*c, inverse_lead); }
    if m[8] != 1 { return Err("norm quotient failed monic normalization".into()); }
    Ok(rr9_poly_pow_mod(h, 8, &m)?.iter().all(|&x| x == 0))
}

/// Exact signed-target parameterization:
/// F=(X-r)A(X)+(Y+s)V(X), R=(r,s), deg A,deg V <=3.
/// On the V_3=1 projective chart it has exactly seven free coefficients.
fn rr9_from_signed_chart(
    a: &[u64; 4], v: &[u64; 4], residual: Point,
) -> Result<[u64; 9], String> {
    let (r, s) = residual.ok_or("signed chart R=O needs a separate branch")?;
    if r >= P || s >= P || a.iter().chain(v.iter()).any(|&x| x >= P) || v[3] != 1 {
        return Err("noncanonical point/chart or V_3 not normalized to one".into());
    }
    let mut u = [0u64; 5];
    for i in 0..4 {
        u[i] = fp_sub(u[i], fp_mul(r, a[i]));
        u[i+1] = fp_add(u[i+1], a[i]);
        u[i] = fp_add(u[i], fp_mul(s, v[i]));
    }
    Ok([u[0], u[1], v[0], u[2], v[1], u[3], v[2], u[4], v[3]])
}

/// Direct monic degree-eight residual x-factor on the signed R chart:
/// M = (X²+rX+r²+a)V² - (X-r)A² - 2s A V, where R=(r,s) and V3=1.
/// No full norm, synthetic division, factor-base candidates, or EC additions.
fn rr9_signed_chart_monic_factor(
    a: &[u64; 4], v: &[u64; 4], residual: Point,
) -> Result<[u64; 9], String> {
    let (r, s) = residual.ok_or("R=O needs an infinity chart")?;
    if r >= P || s >= P || a.iter().chain(v.iter()).any(|&x| x >= P) || v[3] != 1 {
        return Err("invalid chart coefficient or nonnormalized V3".into());
    }
    let rhs = fp_add(fp_add(fp_mul(fp_mul(r, r), r), fp_mul(PRIME31.a, r)), PRIME31.b);
    if fp_mul(s, s) != rhs { return Err("signed chart residual off curve".into()); }
    let d = [fp_add(fp_mul(r, r), PRIME31.a), r, 1u64];
    let d_vv = rr9_poly_mul(&d, &rr9_poly_mul(v, v));
    let xm_r_aa = rr9_poly_mul(&[fp_sub(0, r), 1u64], &rr9_poly_mul(a, a));
    let av = rr9_poly_mul(a, v);
    assert_eq!((d_vv.len(), xm_r_aa.len(), av.len()), (9, 8, 7));
    let two_s = fp_mul(2, s);
    let factor = std::array::from_fn(|i| {
        let mut z = d_vv[i];
        if i < xm_r_aa.len() { z = fp_sub(z, xm_r_aa[i]); }
        if i < av.len() { z = fp_sub(z, fp_mul(two_s, av[i])); }
        z
    });
    if factor[8] != 1 { return Err("chart factor not monic of degree eight".into()); }
    Ok(factor)
}

/// Recover unique normalized A,V from a *supplied* F with F(-R)=0 and V3 !=0.
/// It does not discover F or an unknown DLP. Invalid signed input returns None.
fn rr9_to_signed_chart(
    coeff: &[u64; 9], residual: Point,
) -> Result<Option<([u64; 4], [u64; 4])>, String> {
    let (r, s) = residual.ok_or("signed chart R=O needs a separate branch")?;
    if r >= P || s >= P || coeff.iter().any(|&x| x >= P) {
        return Err("noncanonical signed chart input".into());
    }
    if coeff[8] == 0 { return Ok(None); }
    let inv = fp_pow(coeff[8], P-2);
    let u = [coeff[0], coeff[1], coeff[3], coeff[5], coeff[7]]
        .map(|c| fp_mul(c, inv));
    let v = [coeff[2], coeff[4], coeff[6], coeff[8]]
        .map(|c| fp_mul(c, inv));
    let mut numerator = [0u64; 5];
    for i in 0..5 {
        numerator[i] = fp_sub(u[i], if i < 4 { fp_mul(s, v[i]) } else { 0 });
    }
    let mut a = [0u64; 4];
    a[3] = numerator[4];
    for i in (0..3).rev() {
        a[i] = fp_add(numerator[i+1], fp_mul(r, a[i+1]));
    }
    if fp_add(numerator[0], fp_mul(r, a[0])) != 0 { return Ok(None); }
    let reconstructed = rr9_from_signed_chart(&a, &v, residual)?;
    let normalized = coeff.map(|z| fp_mul(z, inv));
    if reconstructed != normalized {
        return Err("signed chart reconstruction lost coefficients".into());
    }
    if rr9_function_at(&reconstructed, neg(residual))? != 0 {
        return Err("signed chart failed F(-R)=0".into());
    }
    Ok(Some((a, v)))
}

/// A falsifier that preserves the full signed F(-R)=0 equation while changing
/// the norm support: add (X-x(R)) to U. Rejected unless a genuine collision.
fn rr9_preserve_target_mutation(coeff: &[u64; 9], residual: Point) -> Result<[u64; 9], String> {
    let (rx, _) = residual.ok_or("R=O unsupported")?;
    let mut out = *coeff;
    out[0] = fp_sub(out[0], rx);
    out[1] = fp_add(out[1], 1);
    if rr9_function_at(&out, neg(residual))? != 0 {
        return Err("mutation did not preserve signed residual condition".into());
    }
    Ok(out)
}

fn first_rational_points(ar: &mut Arithmetic) -> Vec<(u64, u64)> {
    let mut out = Vec::with_capacity(31);
    for x in 0..PRIME31.p {
        let rhs = ar.rhs(PRIME31, x);
        if let Some(y) = ar.sqrt(rhs, PRIME31.p) {
            if y == 0 { continue; }
            out.push((x, y.min(PRIME31.p - y)));
            if out.len() == 31 { break; }
        }
    }
    assert_eq!(out.len(), 31);
    out
}

fn group_sum(ar: &mut Arithmetic, pts: &[Point; 8]) -> Point {
    let mut acc: Point = None;
    for &q in pts { acc = ar.add(PRIME31, acc, q); }
    acc
}

/// Generate pairwise-distinct signed leaves without consulting any discrete-log
/// label. The public residual is constructed outside the determinant oracle.
fn panel(ar: &mut Arithmetic, base: &[(u64, u64)], label: &str) -> Value {
    assert_eq!(base.len(), 31);
    let mut seed = SEED ^ if label.starts_with("dickson") { 0x1234_5678 } else { 0 };
    let mut h = vec![1u64];
    for &(x, _) in base {
        h = rr9_poly_mul(&h, &[fp_sub(0, x), 1]);
    }
    assert_eq!(h.len(), 32);
    let mut attempts = 0usize;
    let mut pos_ns = 0u128;
    let mut neg_ns = 0u128;
    let mut records = Vec::new();
    let base_additions_before = ar.counts.group_additions;

    while records.len() < NCASES {
        attempts += 1;
        assert!(attempts <= CAP, "failed deterministic admissible trial cap");
        let mut choices: Vec<usize> = (0..31).collect();
        let mut leaves: [Point; 8] = [None; 8];
        for i in 0..8 {
            let j = i + (splitmix64(&mut seed) as usize % (31 - i));
            choices.swap(i, j);
            let q = Some(base[choices[i]]);
            leaves[i] = if splitmix64(&mut seed) & 1 == 1 { neg(q) } else { q };
        }
        let r = group_sum(ar, &leaves);
        let not_r = ar.add(PRIME31, r, Some(PRIME31.g));
        // Require two NONDEGENERATE full nine-point divisors. Rejections are
        // counted and happen before looking at determinant values.
        if r.is_none() || not_r.is_none() { continue; }
        let unique_pos = {
            let mut s = HashSet::new();
            leaves.iter().copied().chain(std::iter::once(neg(r))).all(|q| s.insert(q))
        };
        let unique_neg = {
            let mut s = HashSet::new();
            leaves.iter().copied().chain(std::iter::once(neg(not_r))).all(|q| s.insert(q))
        };
        if !unique_pos || !unique_neg { continue; }

        let t = Instant::now();
        let d_yes = relation_determinant(&leaves, r).expect("prechecked distinct positives");
        pos_ns += t.elapsed().as_nanos();
        let t = Instant::now();
        let d_no = relation_determinant(&leaves, not_r).expect("prechecked distinct negatives");
        neg_ns += t.elapsed().as_nanos();

        assert_eq!(d_yes, 0, "elliptic RR9 determinant misses a positive");
        assert_ne!(d_no, 0, "elliptic RR9 determinant false positive on distinct nine points");
        assert_ne!(r, not_r, "G is unexpectedly identity");
        let norm_certificate = rr9_norm_certificate(&leaves, r, not_r)
            .expect("RR9 original-curve norm certificate for 8 distinct leaves");
        let coeff: [u64; 9] = serde_json::from_value(
            norm_certificate["kernel_coefficients_L9O"].clone()
        ).expect("nine coefficient witness values");
        assert!(rr9_public_norm_support(&coeff, r, &h)
            .expect("public coefficient-only positive support test"));
        let (chart_a, chart_v) = rr9_to_signed_chart(&coeff, r)
            .expect("signed target chart").expect("true norm has V3!=0");
        let chart_factor = rr9_signed_chart_monic_factor(&chart_a, &chart_v, r)
            .expect("direct signed monic eight-factor");
        assert!(rr9_poly_pow_mod(&h, 8, &chart_factor)
            .expect("direct M divides public H^8").iter().all(|&z| z == 0));
        let (radical_supported, radical_degree) = rr9_radical_support(&chart_factor, &h)
            .expect("independent radical support on distinct-leaf prime31 M");
        assert!(radical_supported, "radical gate rejected valid public signed support");
        assert_eq!(radical_degree, 8, "distinct leaf x-coordinates must be squarefree");
        let chart_coeff = rr9_from_signed_chart(&chart_a, &chart_v, r).unwrap();
        let normalized_norm = rr9_norm(&chart_coeff);
        let mut quotient = [0u64; 9];
        quotient[8] = normalized_norm[9];
        for i in (0..8).rev() {
            quotient[i] = fp_add(normalized_norm[i+1], fp_mul(r.unwrap().0, quotient[i+1]));
        }
        assert_eq!(fp_add(normalized_norm[0], fp_mul(r.unwrap().0, quotient[0])), 0);
        let inv_lead = fp_pow(quotient[8], P-2);
        for z in &mut quotient { *z = fp_mul(*z, inv_lead); }
        assert_eq!(quotient, chart_factor, "direct chart factor differs from full x-norm quotient");
        let changed_coeff = rr9_preserve_target_mutation(&coeff, r)
            .expect("negative control preserves signed F(-R)=0");
        let (mut_a, mut_v) = rr9_to_signed_chart(&changed_coeff, r)
            .expect("altered F remains on target chart").expect("mutation V3!=0");
        let altered_factor = rr9_signed_chart_monic_factor(&mut_a, &mut_v, r)
            .expect("target-preserving mutation monic factor");
        assert_ne!(altered_factor, chart_factor, "altered F has unchanged norm factor");
        assert!(rr9_poly_pow_mod(&h, 8, &altered_factor)
            .unwrap().iter().any(|&z| z != 0));
        assert!(!rr9_radical_support(&altered_factor, &h)
            .expect("independent radical mutated coefficient negative").0,
            "radical oracle accepted a target-preserving unsupported M");
        assert!(!rr9_public_norm_support(&changed_coeff, r, &h)
            .expect("same-target altered F support test"),
            "altered F unexpectedly has eight factor-base x roots");
        records.push(json!({
            "case":records.len(), "eight_signed_leaves":leaves, "sum_residual":r,
            "negative_residual_plus_G":not_r, "positive_det":d_yes,
            "negative_det":d_no, "no_repeated_or_infinite_points":true,
            "original_curve_sum_witness":true,
            "norm_certificate":norm_certificate,
            "public_norm_support_true":true,
            "signed_target_chart_valid":true,
            "target_preserving_norm_mutation_rejected":true
        }));
    }

    // Intentionally violate pairwise distinctness to falsify the naive
    // determinant, retaining a wrong target (correct sum plus nonzero G).
    let first = &records[0];
    let original: Vec<Point> = serde_json::from_value(first["eight_signed_leaves"].clone())
        .expect("serialize/deserialize eight public points");
    let mut repeated = [None; 8];
    repeated.copy_from_slice(&original);
    repeated[1] = repeated[0];
    let repeated_sum = group_sum(ar, &repeated);
    let wrong_target = ar.add(PRIME31, repeated_sum, Some(PRIME31.g));
    assert_ne!(wrong_target, repeated_sum);
    let mut naive = [[0u64; 9]; 9];
    for i in 0..8 { naive[i] = evaluation_row(repeated[i]).unwrap(); }
    // If wrong target happens to be infinity, use another nonzero offset.
    let wrong_target = if wrong_target.is_none() {
        {
            let double_g = ar.add(PRIME31, Some(PRIME31.g), Some(PRIME31.g));
            ar.add(PRIME31, repeated_sum, double_g)
        }
    } else { wrong_target };
    naive[8] = evaluation_row(neg(wrong_target)).unwrap();
    let false_zero = determinant(naive);
    assert_eq!(false_zero, 0, "duplicate matrix rows must cause a false zero");
    assert!(relation_determinant(&repeated, wrong_target).is_err(),
        "safe squarefree oracle must reject repeated signed points");
    // Repair the diagonal failure via the TRUE elliptic tangent jet rather
    // than ignoring the duplicated point or admitting the false positive.
    let hermite_true = relation_determinant_hermite(&repeated, repeated_sum)
        .expect("double divisor positive with x-local uniformizer");
    let hermite_false = relation_determinant_hermite(&repeated, wrong_target)
        .expect("double divisor negative with x-local uniformizer");
    assert_eq!(hermite_true, 0, "confluent RR9 misses a true repeated-leaf sum");
    assert_ne!(hermite_false, 0, "confluent RR9 admits a false repeated-leaf sum");

    // Preregistered diagonal tests: third, fourth and eighth occurrence.
    // Construction with original EC arithmetic is *outside* the determinant.
    let mut higher_controls = Vec::new();
    for multiplicity in [3usize, 4, 8] {
        let mut repeated_leaves: [Point; 8] = [Some(base[1]); 8];
        for leaf in repeated_leaves.iter_mut().take(multiplicity) {
            *leaf = Some(base[0]);
        }
        let true_r = group_sum(ar, &repeated_leaves);
        let false_r = ar.add(PRIME31, true_r, Some(PRIME31.g));
        assert!(true_r.is_some() && false_r.is_some());
        assert_ne!(true_r, false_r);
        let true_det = relation_determinant_hermite(&repeated_leaves, true_r)
            .expect("positive high-multiplicity divisor");
        let false_det = relation_determinant_hermite(&repeated_leaves, false_r)
            .expect("negative high-multiplicity divisor");
        assert_eq!(true_det, 0, "higher jet rejects valid signed relation");
        assert_ne!(false_det, 0, "higher jet admits wrong public residual");
        let higher_norm = rr9_norm_certificate(&repeated_leaves, true_r, false_r)
            .expect("RR9 norm polynomial identity with repeated Hermite leaves");
        let coeff: [u64; 9] = serde_json::from_value(
            higher_norm["kernel_coefficients_L9O"].clone()
        ).expect("confluent RR9 coefficients");
        assert!(rr9_public_norm_support(&coeff, true_r, &h)
            .expect("public support with repeated signed leaves"));
        let (chart_a, chart_v) = rr9_to_signed_chart(&coeff, true_r)
            .expect("confluent signed target chart").expect("nonzero residual chart");
        let chart_factor = rr9_signed_chart_monic_factor(&chart_a, &chart_v, true_r)
            .expect("direct repeated-point chart factor");
        assert!(rr9_poly_pow_mod(&h, 8, &chart_factor)
            .unwrap().iter().all(|&z| z == 0));
        let (radical_supported, radical_degree) = rr9_radical_support(&chart_factor, &h)
            .expect("independent repeated-point radical membership");
        assert!(radical_supported, "radical oracle missed repeated valid leaf support");
        assert_eq!(radical_degree, if multiplicity == 8 { 1 } else { 2 });
        let norm = rr9_norm(&rr9_from_signed_chart(&chart_a, &chart_v, true_r).unwrap());
        let rx = true_r.unwrap().0;
        let mut quotient = [0u64; 9];
        quotient[8] = norm[9];
        for i in (0..8).rev() {
            quotient[i] = fp_add(norm[i+1], fp_mul(rx, quotient[i+1]));
        }
        assert_eq!(fp_add(norm[0], fp_mul(rx, quotient[0])), 0);
        let inv_lead = fp_pow(quotient[8], P-2);
        for z in &mut quotient { *z = fp_mul(*z, inv_lead); }
        assert_eq!(quotient, chart_factor);
        let changed_coeff = rr9_preserve_target_mutation(&coeff, true_r)
            .expect("target-preserving repeated mutation");
        let (mut_a, mut_v) = rr9_to_signed_chart(&changed_coeff, true_r)
            .expect("confluent changed chart").expect("changed V3!=0");
        let altered_factor = rr9_signed_chart_monic_factor(&mut_a, &mut_v, true_r)
            .expect("confluent changed monic factor");
        assert_ne!(altered_factor, chart_factor);
        assert!(rr9_poly_pow_mod(&h, 8, &altered_factor).unwrap().iter().any(|&z| z != 0));
        assert!(!rr9_radical_support(&altered_factor, &h)
            .expect("repeated altered coefficient radical negative").0);
        assert!(!rr9_public_norm_support(&changed_coeff, true_r, &h)
            .expect("altered repeated support check"));
        higher_controls.push(json!({
            "norm_certificate":higher_norm,
            "public_norm_support_true":true,
            "signed_target_chart_valid":true,
            "target_preserving_norm_mutation_rejected":true,
            "multiplicity":multiplicity,
            "leaves":repeated_leaves, "true_residual":true_r,
            "wrong_residual_plus_G":false_r,
            "true_det":true_det, "wrong_det":false_det,
            "curve_witness_outside_determinant":true
        }));
    }

    // Ninth-jet falsifier: nine copies of P cannot sum to infinity,
    // since P != O and the group has prime order not dividing nine.
    let all_one: [Point; 8] = [Some(base[0]); 8];
    let fake_residual = neg(Some(base[0]));
    let real_residual = group_sum(ar, &all_one);
    assert_ne!(real_residual, fake_residual);
    let ninth_jet_wrong_det = relation_determinant_hermite(&all_one, fake_residual)
        .expect("nine identical affine points require Taylor orders zero through eight");
    assert_ne!(ninth_jet_wrong_det, 0, "nine equal points falsely satisfy RR9");

    json!({
        "base":label, "factor_base":base, "seed":format!("{SEED:#x}"),
        "accepted_positive":NCASES, "accepted_negative":NCASES,
        "draws_including_rejections":attempts, "positive_det_total_ns_diagnostic":pos_ns.to_string(),
        "negative_det_total_ns_diagnostic":neg_ns.to_string(),
        "source_group_additions_outside_determinant":ar.counts.group_additions-base_additions_before,
        "naive_repeated_row_false_zero":false_zero==0,
        "safe_repeated_row_rejected":true,
        "confluent_hermite_repeated_true_det":hermite_true,
        "confluent_hermite_repeated_false_det":hermite_false,
        "confluent_hermite_double_control":true,
        "confluent_hermite_higher_controls":higher_controls,
        "ninth_jet_wrong_det":ninth_jet_wrong_det,
        "ninth_jet_original_curve_negative":true,
        "no_unknown_ecdlp_solved":true,
        "public_norm_membership_only_not_extraction":true,
        "cases":records
    })
}


/// Exact affine linear solve for a GIVEN B8 sign vector and public residual.
/// This deliberately enumerates signed supports: it is a correctness oracle,
/// not a nonenumerative relation extraction algorithm or an ECDLP solver.
fn rr9_b8_signed_linear_solve(
    base: &[(u64, u64); 8], signs: u8, residual: Point,
) -> Result<Option<([u64; 4], [u64; 4])>, String> {
    let (r, s) = residual.ok_or("B8 solver excludes infinity residual")?;
    let mut rows = [[0u64; 8]; 8];
    for (i, &(x, y)) in base.iter().enumerate() {
        let signed_y = if ((signs >> i) & 1) == 1 { fp_sub(0, y) } else { y };
        let powers = [1, x, fp_mul(x, x), fp_mul(fp_mul(x, x), x)];
        if x == r {
            // The ordinary split product is 0=0 at x=r. Its actual
            // quotient root condition is the pair of *linear* branches
            // M(r)=V(r)[(3r^2+a)V(r)-2sA(r)]=0.
            // The selected signed leaf fixes WHICH branch is correct:
            // R: F(R)=2s V(r)=0; -R: F'(-R)=0 (confluent jet).
            if s == 0 { return Err("rational two-torsion invalid for this prime-order curve".into()); }
            if signed_y == s {
                for j in 0..3 { rows[i][4+j] = powers[j]; }
                rows[i][7] = fp_sub(0, powers[3]);
            } else if signed_y == fp_sub(0, s) {
                let d = fp_add(fp_mul(3, fp_mul(r, r)), PRIME31.a);
                let two_s = fp_mul(2, s);
                for j in 0..4 { rows[i][j] = fp_mul(two_s, powers[j]); }
                for j in 0..3 { rows[i][4+j] = fp_sub(0, fp_mul(d, powers[j])); }
                rows[i][7] = fp_mul(d, powers[3]);
            } else {
                return Err("factor-base point off the residual x-fibre".into());
            }
            continue;
        }
        let t = fp_sub(x, r);
        let w = fp_add(s, signed_y);
        for j in 0..4 { rows[i][j] = fp_mul(t, powers[j]); }
        for j in 0..3 { rows[i][4+j] = fp_mul(w, powers[j]); }
        // Fixed V_3=1 contributes w*x^3 to the left-hand side.
        rows[i][7] = fp_sub(0, fp_mul(w, powers[3]));
    }
    let mut pivots = [usize::MAX; 7];
    let mut rank = 0usize;
    for col in 0..7 {
        let Some(piv) = (rank..8).find(|&i| rows[i][col] != 0) else {
            continue;
        };
        rows.swap(rank, piv);
        let inv = fp_pow(rows[rank][col], P-2);
        for j in col..8 { rows[rank][j] = fp_mul(rows[rank][j], inv); }
        for i in 0..8 {
            if i == rank { continue; }
            let factor = rows[i][col];
            for j in col..8 {
                rows[i][j] = fp_sub(rows[i][j], fp_mul(factor, rows[rank][j]));
            }
        }
        pivots[rank] = col;
        rank += 1;
    }
    if (rank..8).any(|i| rows[i][7] != 0) { return Ok(None); }
    if rank != 7 {
        return Err(format!("B8 signed chart has a consistent rank-{rank} positive-dimensional fibre"));
    }
    let mut result = [0u64; 7];
    for i in 0..7 { result[pivots[i]] = rows[i][7]; }
    Ok(Some((
        [result[0], result[1], result[2], result[3]],
        [result[4], result[5], result[6], 1],
    )))
}

/// For x != r, -(x-r)M(x)=L_+(x)*L_-(x).
/// For x = r, use the true quotient-root identity
/// M(r)=V(r)[(3r²+a)V(r)-2sA(r)], not the tautological 0=0.
fn rr9_b8_factor_identity_at(
    a: &[u64; 4], v: &[u64; 4], residual: Point, q: (u64, u64),
) -> Result<(u64, u64), String> {
    let (r, s) = residual.ok_or("B8 factor identity needs affine residual")?;
    let (x, y) = q;
    let x2 = fp_mul(x, x);
    let x3 = fp_mul(x2, x);
    let eval = |coeff: &[u64; 4]| {
        fp_add(fp_add(coeff[0], fp_mul(coeff[1], x)),
            fp_add(fp_mul(coeff[2], x2), fp_mul(coeff[3], x3)))
    };
    let av = eval(a);
    let vv = eval(v);
    let t = fp_sub(x, r);
    let l_plus = fp_add(fp_mul(t, av), fp_mul(fp_add(s, y), vv));
    let l_minus = fp_add(fp_mul(t, av), fp_mul(fp_sub(s, y), vv));
    let m = rr9_signed_chart_monic_factor(a, v, residual)?;
    let mval = m.iter().rev().fold(0u64, |z, &c| fp_add(fp_mul(z, x), c));
    if x == r {
        let d = fp_add(fp_mul(3, fp_mul(r, r)), PRIME31.a);
        return Ok((mval, fp_mul(vv, fp_sub(fp_mul(d, vv), fp_mul(fp_mul(2,s), av)))));
    }
    Ok((fp_sub(0, fp_mul(t, mval)), fp_mul(l_plus, l_minus)))
}

/// Exhaustive B8 signed-fibre equivalence test. All 256 masks enter only as
/// a TEST oracle; no target scalar or basis discrete logs enter the solver.
fn rr9_b8_linear_fibre_audit(ar: &mut Arithmetic) -> Value {
    let first = first_rational_points(ar);
    let base: [(u64, u64); 8] = first[..8].try_into().unwrap();
    let mut h = vec![1u64];
    for &(x, _) in &base {
        h = rr9_poly_mul(&h, &[fp_sub(0, x), 1]);
    }
    assert_eq!(h.len(), 9);
    let mut supports: HashMap<Point, Vec<u8>> = HashMap::new();
    let mut sums = [None; 256];
    for mask in 0u16..256 {
        let leaves: [Point; 8] = std::array::from_fn(|i| {
            if ((mask >> i) & 1) == 1 { neg(Some(base[i])) } else { Some(base[i]) }
        });
        let sum = group_sum(ar, &leaves);
        sums[mask as usize] = sum;
        supports.entry(sum).or_default().push(mask as u8);
    }
    let collisions = 256usize - supports.len();
    let mut checked_residuals = HashSet::new();
    let mut excluded_infinity = 0usize;
    let mut excluded_same_x = 0usize;
    let mut positive_systems = 0usize;
    let mut recovered_masks = 0usize;
    let mut exact_fibres = 0usize;
    let mut factor_checks = 0usize;
    for mask in 0usize..256 {
        let target = sums[mask];
        if target.is_none() {
            excluded_infinity += 1;
            continue;
        }
        let r = target.unwrap().0;
        if base.iter().any(|&(x, _)| x == r) {
            excluded_same_x += 1;
            continue;
        }
        if !checked_residuals.insert(target) { continue; }
        let expected = &supports[&target];
        let mut found = Vec::new();
        for candidate in 0u16..256 {
            positive_systems += 1;
            if let Some((a,v)) = rr9_b8_signed_linear_solve(&base, candidate as u8, target)
                .expect("all 8x7 linear fibres must be determinate") {
                let f = rr9_from_signed_chart(&a, &v, target).unwrap();
                let signed: [Point; 8] = std::array::from_fn(|i| {
                    if ((candidate >> i) & 1) == 1 { neg(Some(base[i])) } else { Some(base[i]) }
                });
                assert_eq!(rr9_function_at(&f, neg(target)).unwrap(), 0);
                for &q in &signed { assert_eq!(rr9_function_at(&f, q).unwrap(), 0); }
                let m = rr9_signed_chart_monic_factor(&a, &v, target).unwrap();
                assert_eq!(m.as_slice(), h.as_slice(), "linear solution does not have M=H_B");
                assert!(rr9_radical_support(&m, &h).unwrap().0);
                assert_eq!(group_sum(ar, &signed), target, "false-positive chart solution");
                found.push(candidate as u8);
                recovered_masks += 1;
            }
        }
        assert_eq!(found, *expected, "signed linear fibre differs from original-curve group sums");
        exact_fibres += 1;
        if factor_checks == 0 {
            // Independent algebraic polynomial-factor identity, *not* a solver.
            // Test all 256 arbitrary canonical A,V coefficients and eight base x.
            for j in 0u64..256 {
                let a = [j, j+1, j*j+17, j*3+7];
                let v = [j*11+3, j*j+2, j+9, 1];
                for &q in &base {
                    let (left,right) = rr9_b8_factor_identity_at(&a,&v,target,q).unwrap();
                    assert_eq!(left,right,"B8 split-quadric identity fails");
                    factor_checks += 1;
                }
            }
        }
    }
    let mut negative_systems = 0usize;
    let mut excluded_negative_offsets = 0usize;
    let mut verified_negative_targets = 0usize;
    for &initial_mask in &[0x00usize, 0x35, 0x96, 0xff] {
        let mut target = sums[initial_mask];
        let mut found_negative = false;
        for _attempt in 1..=64 {
            target = ar.add(PRIME31, target, Some(PRIME31.g));
            let suitable = target.map(|(x,_)| !base.iter().any(|&(bx,_)| bx==x))
                .unwrap_or(false);
            if suitable && !supports.contains_key(&target) {
                found_negative = true;
                break;
            }
            excluded_negative_offsets += 1;
        }
        assert!(found_negative, "64 deterministic G offsets failed to find B8 negative");
        for mask in 0u16..256 {
            negative_systems += 1;
            assert!(rr9_b8_signed_linear_solve(&base, mask as u8, target)
                .expect("negative linear chart must be determinate").is_none(),
                "negative public residual admits a false signed fibre");
        }
        verified_negative_targets += 1;
    }
    assert!(exact_fibres >= 1);
    assert!(positive_systems >= 256);
    assert_eq!(negative_systems, 4 * 256);
    assert_eq!(factor_checks, 256 * 8);
    json!({
        "factor_base":"first_eight_rational_x_prime31",
        "formal_sign_masks":256, "distinct_original_curve_sums":supports.len(),
        "sum_collisions":collisions, "admissible_unique_public_residuals":exact_fibres,
        "skipped_masks_with_infinity_R":excluded_infinity,
        "skipped_masks_with_base_x_equal_residual_x":excluded_same_x,
        "positive_affine_linear_systems":positive_systems,
        "recovered_signed_masks":recovered_masks,
        "exact_signed_fibres":exact_fibres,
        "independent_split_quadric_identity_checks":factor_checks,
        "negative_affine_linear_systems":negative_systems,
        "negative_public_residuals":verified_negative_targets,
        "negative_offsets_skipped":excluded_negative_offsets,
        "algorithm_enumerates_all_256_sign_masks":true,
        "complete_unknown_Q_discrete_log_solves":0,
        "nonenumerative_extraction_demonstrated":false,
        "pcc":"unverified"
    })
}

//// Exact exceptional-x B8 controls, with construction outside the coefficient solver.
/// Choose six independent affine leaves, derive a seventh by the original
/// curve group law so seven sum to O (R leaf) or 2R (-R leaf).
/// Reject repeated x, infinity and any target-x collision before testing.
fn rr9_b8_exceptional_fixture(
    ar: &mut Arithmetic, residual: Point, minus_residual_leaf: bool,
) -> [(u64,u64); 8] {
    let (r, _) = residual.expect("fixture requires affine R");
    let first = first_rational_points(ar);
    let required_seven_sum = if minus_residual_leaf {
        ar.add(PRIME31, residual, residual)
    } else { None };
    for shift in 0..=first.len()-6 {
        let mut leaves: [Point; 8] = [None; 8];
        for i in 0..6 { leaves[i] = Some(first[shift+i]); }
        if leaves[..6].iter().any(|q| q.unwrap().0 == r) { continue; }
        let mut six_sum: Point = None;
        for &q in &leaves[..6] { six_sum = ar.add(PRIME31, six_sum, q); }
        let seventh = ar.add(PRIME31, required_seven_sum, neg(six_sum));
        let Some((x7,_)) = seventh else { continue; };
        if x7 == r || leaves[..6].iter().any(|q| q.unwrap().0 == x7) { continue; }
        leaves[6] = seventh;
        leaves[7] = if minus_residual_leaf { neg(residual) } else { residual };
        let basis: [(u64,u64);8] = leaves.map(|q| q.expect("finite fixture"));
        assert_eq!(group_sum(ar, &leaves), residual);
        assert_eq!(basis.iter().map(|p| p.0).collect::<HashSet<_>>().len(), 8);
        return basis;
    }
    panic!("could not build a deterministic distinct-x exceptional fixture");
}

fn rr9_b8_exceptional_fibre_audit(ar: &mut Arithmetic) -> Value {
    let residual = Some(PRIME31.g);
    let mut panels = Vec::new();
    for &minus_residual_leaf in &[false, true] {
        let base = rr9_b8_exceptional_fixture(ar, residual, minus_residual_leaf);
        let mut h = vec![1u64];
        for &(x,_) in &base { h = rr9_poly_mul(&h, &[fp_sub(0,x),1]); }
        let mut expected = Vec::<u8>::new();
        let mut sums = Vec::new();
        for mask in 0u16..256 {
            let leaves: [Point;8] = std::array::from_fn(|i|
                if (mask >> i) & 1 == 1 { neg(Some(base[i])) } else { Some(base[i]) });
            let sum = group_sum(ar,&leaves);
            if sum == residual { expected.push(mask as u8); }
            sums.push(sum);
        }
        assert!(expected.contains(&0), "fixture does not construct its planted positive");
        let mut found = Vec::<u8>::new();
        let mut tangent_checks = 0usize;
        let mut exceptional_product_checks = 0usize;
        for mask in 0u16..256 {
            if let Some((a,v)) = rr9_b8_signed_linear_solve(&base,mask as u8,residual)
                .expect("exceptional B8 has determinate signed chart") {
                let f = rr9_from_signed_chart(&a,&v,residual).unwrap();
                let m = rr9_signed_chart_monic_factor(&a,&v,residual).unwrap();
                assert_eq!(m.as_slice(),h.as_slice(), "exceptional signed solution M!=H");
                assert_eq!(rr9_function_at(&f,neg(residual)).unwrap(),0);
                let leaves: [Point;8] = std::array::from_fn(|i|
                    if (mask >> i) & 1 == 1 { neg(Some(base[i])) } else { Some(base[i]) });
                for &q in &leaves { assert_eq!(rr9_function_at(&f,q).unwrap(),0); }
                if leaves[7] == neg(residual) {
                    let jet = evaluation_tangent_jet(neg(residual)).unwrap();
                    let deriv = (0..9).fold(0u64, |acc,i|
                        fp_add(acc, fp_mul(jet[i],f[i])));
                    assert_eq!(deriv,0, "confluent -R tangent missing");
                    tangent_checks += 1;
                }
                assert!(rr9_radical_support(&m,&h).unwrap().0);
                assert_eq!(group_sum(ar,&leaves),residual);
                found.push(mask as u8);
            }
        }
        assert_eq!(found,expected,"exceptional signed fibres differ from group sums");
        // This negative target must share x with a factor-base point yet have
        // no signed eight-leaf support. Verify absence by the independent EC oracle.
        let mut negative = None;
        for &q in &base[..7] {
            for candidate in [Some(q),neg(Some(q))] {
                if !sums.contains(&candidate) { negative = Some(candidate); break; }
            }
            if negative.is_some() { break; }
        }
        let neg_target = negative.expect("no absent exceptional-x signed residual");
        let mut negative_systems = 0usize;
        for mask in 0u16..256 {
            negative_systems += 1;
            assert!(rr9_b8_signed_linear_solve(&base,mask as u8,neg_target)
                .expect("negative exceptional-x fibre determinate").is_none(),
                "false exceptional-x B8 support");
        }
        // Test the actual exceptional quotient identity independently of
        // the signed solver on deterministic arbitrary A,V coefficients.
        for j in 0u64..128 {
            let a = [j, j+1, j*j+17, j*3+7];
            let v = [j*11+3, j*j+2, j+9, 1];
            let (l,r) = rr9_b8_factor_identity_at(&a,&v,residual,base[7]).unwrap();
            assert_eq!(l,r,"exceptional M(r) factor identity failed");
            exceptional_product_checks += 1;
        }
        panels.push(json!({
            "minus_residual_leaf":minus_residual_leaf,
            "eight_distinct_x":true,
            "residual_is_generator":true,
            "positive_original_curve_sign_masks":expected.len(),
            "recovered_signed_masks":found.len(),
            "exact_signed_fibre_set":true,
            "tangent_derivative_checks":tangent_checks,
            "same_x_outside_support_negative_targets":1,
            "negative_systems":negative_systems,
            "exceptional_polynomial_factor_checks":exceptional_product_checks
        }));
    }
    json!({"panels":panels,"exceptional_branch_count":2,
        "nonenumerative_extraction":false,"unknown_ecdlp_logs":0,
        "pcc":"unverified"})
}


/// Reconstruct the ninth zero of an RR9 function from seven *given* signed
/// affine points S_i and a public affine residual R.  This is NOT relation
/// extraction: the divisor identity proves T=R-sum(S_i), a generic group
/// subtraction, although this implementation obtains T algebraically.
///
/// Hermite jet comparison is essential if T shares an x-coordinate with a
/// supplied zero: F(T)=0 alone is insufficient to distinguish T from -T.
fn rr9_complete_seven_signed_leaves(
    ar: &mut Arithmetic, leaves: &[Point; 7], residual: Point,
) -> Result<Point, String> {
    residual.ok_or("seven-anchor completion excludes R=O")?;
    if leaves.iter().any(|q| q.is_none()) {
        return Err("seven-anchor completion excludes supplied infinity".into());
    }
    let divisor: [Point; 8] = std::array::from_fn(|i| {
        if i < 7 { leaves[i] } else { neg(residual) }
    });
    // Existing L(9O) kernel uses true higher-order jets for repeated points.
    let coeff = rr9_leaves_kernel(&divisor)?;
    let norm = rr9_norm(&coeff);
    if norm[9] == 0 {
        // Ninth zero is at O, where the affine norm factor is unavailable.
        return Ok(None);
    }
    let mut known_x = vec![1u64];
    for q in leaves.iter().copied().chain(std::iter::once(residual)) {
        let (x, _) = q.ok_or("infinite point in divisor x factors")?;
        known_x = rr9_poly_mul(&known_x, &[fp_sub(0, x), 1]);
    }
    let (quotient, remainder) = rr9_poly_div_rem(&norm, &known_x)?;
    if remainder != [0] || quotient.len() != 2 || quotient[1] != norm[9] {
        return Err("RR9 norm does not leave an exact linear ninth-zero factor".into());
    }
    let x = fp_mul(fp_sub(0, quotient[0]), fp_pow(quotient[1], P - 2));
    let rhs = ar.rhs(PRIME31, x);
    let y = ar.sqrt(rhs, P)
        .ok_or("ninth norm x-coordinate has no rational curve lift")?;
    if y == 0 { return Err("prime-order prime31 has no affine rational 2-torsion".into()); }
    let positive = Some((x, y));
    let negative = neg(positive);
    // Test for ONE ADDITIONAL local zero beyond the eight already imposed.
    // This also disambiguates the exceptional x(T)=x(R) tangent-jet branch.
    let has_extra = |q: Point| -> Result<bool, String> {
        let order = divisor.iter().filter(|&&given| given == q).count();
        let row = evaluation_higher_jet(q, order)?;
        let value = (0..9).fold(0u64, |acc, i| fp_add(acc, fp_mul(row[i], coeff[i])));
        Ok(value == 0)
    };
    match (has_extra(positive)?, has_extra(negative)?) {
        (true, false) => Ok(positive),
        (false, true) => Ok(negative),
        (false, false) => Err(format!("RR9 ninth norm root at x={x} has no signed extra zero")),
        (true, true) => Err(format!("RR9 ninth norm root at x={x} has ambiguous extra zeros")),
    }
}

/// Full exact B8 sign-mask and original-curve B31 audit of seven-leaf
/// interpolation. The original group sum is the INDEPENDENT test oracle;
/// only seven signed leaves and public R are passed to the algebraic routine.
fn rr9_seven_anchor_completion_audit(ar: &mut Arithmetic) -> Value {
    let first = first_rational_points(ar);
    let b8: [(u64,u64);8] = first[..8].try_into().unwrap();
    let mut b8_checked = 0usize;
    let mut b8_infinity = 0usize;
    let mut exceptional_r_x = 0usize;
    let mut computation_ns = 0u128;
    for mask in 0u16..256 {
        let signed: [Point; 8] = std::array::from_fn(|i| {
            if (mask >> i) & 1 == 1 { neg(Some(b8[i])) } else { Some(b8[i]) }
        });
        let residual = group_sum(ar, &signed);
        if residual.is_none() {
            b8_infinity += 1;
            continue;
        }
        let seven: [Point; 7] = std::array::from_fn(|i| signed[i]);
        let started = Instant::now();
        let completion = rr9_complete_seven_signed_leaves(ar, &seven, residual)
            .expect("B8 RR9 algebraic completion should have one exact signed ninth zero");
        computation_ns += started.elapsed().as_nanos();
        assert_eq!(completion, signed[7], "B8 algebraic ninth zero differs from original EC sum");
        if residual.unwrap().0 == signed[7].unwrap().0 { exceptional_r_x += 1; }
        b8_checked += 1;
    }
    assert!(b8_checked >= 250, "unexpected number of infinity B8 residuals");
    let residual = Some(PRIME31.g);
    let mut exceptional_fixtures = 0usize;
    for &minus in &[false, true] {
        let points = rr9_b8_exceptional_fixture(ar, residual, minus);
        let seven: [Point; 7] = std::array::from_fn(|i| Some(points[i]));
        let completed = rr9_complete_seven_signed_leaves(ar, &seven, residual)
            .expect("exceptional seven-anchor Hermite completion");
        assert_eq!(completed, Some(points[7]), "R/-R exceptional completion chooses wrong tangent sign");
        exceptional_fixtures += 1;
    }
    let mut negative_off_base = 0usize;
    let mut negative_trials = 0usize;
    for &mask in &[0u8, 0x35, 0x96, 0xff] {
        let signed: [Point;8] = std::array::from_fn(|i|
            if (mask >> i) & 1 == 1 { neg(Some(b8[i])) } else { Some(b8[i]) });
        let seven: [Point;7] = std::array::from_fn(|i| signed[i]);
        let mut target = group_sum(ar, &signed);
        let seven_sum = seven.iter().copied().fold(None, |acc,q| ar.add(PRIME31, acc, q));
        for _ in 0..64 {
            target = ar.add(PRIME31,target,Some(PRIME31.g));
            negative_trials += 1;
            if target.is_none() { continue; }
            let expected = ar.add(PRIME31, target, neg(seven_sum));
            let Some((ex,_)) = expected else { continue; };
            if b8.iter().any(|&(x,_)| x == ex) { continue; }
            let completed = rr9_complete_seven_signed_leaves(ar, &seven, target)
                .expect("negative off-base algebraic completion");
            assert_eq!(completed, expected, "nonfactor-base completion must match generic subtraction");
            assert!(!b8.iter().any(|&(x,_)| x == completed.unwrap().0),
                "off-base negative was accepted as a factor-base leaf");
            negative_off_base += 1;
            break;
        }
    }
    assert_eq!(negative_off_base, 4, "not all B8 off-base negative controls completed");
    let dickson = Base::build(PRIME31, 6, ar).unwrap();
    assert_eq!(dickson.points.len(),31);
    let mut b31_checked = 0usize;
    let mut draws = 0usize;
    for (index, base) in [&first, &dickson.points].into_iter().enumerate() {
        let mut seed = SEED ^ (index as u64) ^ 0x5345_5645_4e5f_414e;
        let mut panel_checked = 0usize;
        while panel_checked < 32 {
            draws += 1;
            assert!(draws < 2000, "B31 seven-anchor completion trial cap exceeded");
            let mut choices: Vec<usize> = (0..31).collect();
            let signed: [Point;8] = std::array::from_fn(|i| {
                let j = i + (splitmix64(&mut seed) as usize % (31-i));
                choices.swap(i,j);
                let q = Some(base[choices[i]]);
                if splitmix64(&mut seed)&1==1 {neg(q)} else {q}
            });
            let residual = group_sum(ar,&signed);
            if residual.is_none() { continue; }
            let seven: [Point;7] = std::array::from_fn(|i| signed[i]);
            let started = Instant::now();
            let completion = rr9_complete_seven_signed_leaves(ar,&seven,residual)
                .expect("B31 given seven signed leaves complete to eighth");
            computation_ns += started.elapsed().as_nanos();
            assert_eq!(completion,signed[7], "B31 target-conditioned ninth norm root mismatches EC sum");
            panel_checked += 1;
            b31_checked += 1;
        }
    }
    json!({
        "schema":"prime31.rr9.seven_anchor_completion.v1",
        "p":PRIME31.p,"n":PRIME31.n,
        "b8_sign_masks_total":256,
        "b8_positive_checked":b8_checked,"b8_infinity_residuals":b8_infinity,
        "b8_residual_x_equals_withheld_x":exceptional_r_x,
        "explicit_R_and_minus_R_jet_fixtures":exceptional_fixtures,
        "negative_off_base_completed":negative_off_base,
        "negative_target_offsets_tried":negative_trials,
        "b31_given_support_completions":b31_checked,
        "b31_raw_draws":draws,
        "interpolation_wall_ns_diagnostic_only":computation_ns.to_string(),
        "seven_signed_leaf_points_supplied":true,
        "generic_group_subtraction_equivalence_proved":true,
        "public_only_relation_extraction":false,
        "complete_unknown_target_ecdlp_recoveries":0,
        "pcc":"unverified"
    })
}

fn run() -> Value {
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar).expect("prime31 curve certificate");
    let seven_anchor_audit = rr9_seven_anchor_completion_audit(&mut ar);
    let b8_fibre_audit = rr9_b8_linear_fibre_audit(&mut ar);
    let b8_exceptional_audit = rr9_b8_exceptional_fibre_audit(&mut ar);
    let b1 = first_rational_points(&mut ar);
    let b2 = Base::build(PRIME31, 6, &mut ar).expect("real Dickson31 depth-six base");
    assert_eq!(b2.points.len(), 31);
    let first = panel(&mut ar, &b1, "first31_rational_x");
    let dickson = panel(&mut ar, &b2.points, "dickson_depth6_31_rational_points");
    json!({
        "schema":"prime31.rr9.evaluation.v1", "p":PRIME31.p, "n":PRIME31.n,
        "curve":PRIME31, "method":"L(9O) evaluation determinant on nine distinct affine points",
        "positive_count":64, "negative_count":64,
        "panels":[first,dickson], "b8_signed_fibres":b8_fibre_audit,
        "b8_exceptional_x_signed_fibres":b8_exceptional_audit,
        "seven_anchor_completion":seven_anchor_audit, "source_only_pcc":"unverified",
        "nonenumerative_factor_extraction":false,
        "complete_unknown_target_ecdlp_recoveries":0,
        "matched_optimized_rho":false
    })
}
fn main() { println!("{}", run()); }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rr9_pole_basis_and_mod_arithmetic() {
        assert_eq!(fp_add(P - 1, 1), 0);
        assert_eq!(fp_sub(0, 1), P - 1);
        assert_eq!(fp_mul(P - 1, P - 1), 1);
        assert_eq!(fp_pow(7, P - 2), 1_840_700_269); // checked against Fermat inverse
        let mut a = [[0u64; 9]; 9];
        for i in 0..9 { a[i][i] = 1; }
        assert_eq!(determinant(a), 1);
        a[2] = a[1];
        assert_eq!(determinant(a), 0);
    }
    #[test]
    fn rr9_taylor_first_two_coefficients_match_independent_rows() {
        let mut ar = Arithmetic::default();
        check_curve(PRIME31, &mut ar).unwrap();
        let points = first_rational_points(&mut ar);
        for &point in points.iter().take(16) {
            let q = Some(point);
            assert_eq!(evaluation_higher_jet(q, 0).unwrap(), evaluation_row(q).unwrap());
            assert_eq!(evaluation_higher_jet(q, 1).unwrap(),
                       evaluation_tangent_jet(q).unwrap());
        }
        assert!(evaluation_higher_jet(Some(points[0]), 9).is_err());
    }
    #[test]
    fn rr9_public_support_polynomial_repeated_root_control() {
        // H=X-7, M=(X-7)^8: M divides H^8, not H^7.
        let h = [fp_sub(0, 7), 1u64];
        let mut m = vec![1u64];
        for _ in 0..8 { m = rr9_poly_mul(&m, &h); }
        assert_eq!(m.len(), 9);
        assert_eq!(rr9_poly_pow_mod(&h, 8, &m).unwrap(), vec![0; 8]);
        assert_ne!(rr9_poly_pow_mod(&h, 7, &m).unwrap(), vec![0; 8]);
        assert!(rr9_poly_rem_monic(vec![1], &[1]).is_err());
        assert!(rr9_poly_rem_monic(vec![2], &[1, 2]).is_err());
    }
    #[test]
    fn rr9_radical_support_partitions_and_negative_controls() {
        // Eight x-roots, with repetitions, deliberately selected from the
        // original F_p. This tests the exact polynomial iff theorem independent
        // of the coefficient interpolation and original-curve group arithmetic.
        let roots = [7u64, 11, 19, 23, 29, 31, 37, 41];
        let mut h = vec![1u64];
        for &x in &roots {
            h = rr9_poly_mul(&h, &[fp_sub(0, x), 1]);
        }
        for (mults, expected_degree) in [
            (&[8usize][..], 1usize),
            (&[4usize, 4][..], 2usize),
            (&[5usize, 2, 1][..], 3usize),
            (&[1usize, 1, 1, 1, 1, 1, 1, 1][..], 8usize),
        ] {
            let mut m = vec![1u64];
            for (i, &count) in mults.iter().enumerate() {
                for _ in 0..count {
                    m = rr9_poly_mul(&m, &[fp_sub(0, roots[i]), 1]);
                }
            }
            assert_eq!(m.len(), 9);
            let (fast, degree) = rr9_radical_support(&m, &h).unwrap();
            assert!(fast);
            assert_eq!(degree, expected_degree);
            assert!(rr9_poly_pow_mod(&h, 8, &m).unwrap().iter().all(|&z| z == 0));
            if expected_degree == 8 {
                // The discriminant-nonzero stratum has no H^8 power gate:
                // membership reduces directly to eight remainder equations H mod M.
                assert!(rr9_poly_rem_monic(h.clone(), &m).unwrap().iter().all(|&z| z == 0));
            }
            let mut h_omit = vec![1u64];
            for &x in roots.iter().skip(1) {
                h_omit = rr9_poly_mul(&h_omit, &[fp_sub(0, x), 1]);
            }
            assert!(!rr9_radical_support(&m, &h_omit).unwrap().0);
            assert!(rr9_poly_pow_mod(&h_omit, 8, &m).unwrap().iter().any(|&z| z != 0));
        }
        // M contains eight roots but its degree/order and canonicality are
        // checked independently of the x-factor-base polynomial.
        assert!(rr9_radical_support(&[1, 1], &h).is_err());
        let mut malformed = [0u64; 9];
        malformed[0] = P;
        malformed[8] = 1;
        assert!(rr9_radical_support(&malformed, &h).is_err());
        let double_root_h = rr9_poly_mul(
            &[fp_sub(0, 7), 1], &[fp_sub(0, 7), 1]);
        assert!(rr9_radical_support(&malformed.map(|z| if z == P { 0 } else { z }), &double_root_h).is_err());
        assert!(rr9_poly_div_rem(&[1], &[0]).is_err());
    }
    #[test]
    fn rr9_public_support_rejects_infinity_and_invalid_coefficients() {
        let zero = [0u64; 9];
        let h = [1u64, 1];
        assert!(rr9_public_norm_support(&zero, None, &h).is_err());
        assert!(rr9_to_signed_chart(&zero, None).is_err());
        assert!(rr9_to_signed_chart(&zero, Some(PRIME31.g)).unwrap().is_none());
        let aa = [0u64; 4];
        let invalid_v = [0u64; 4];
        assert!(rr9_signed_chart_monic_factor(&aa, &invalid_v, Some(PRIME31.g)).is_err());
        assert!(rr9_signed_chart_monic_factor(&aa, &[0,0,0,1], None).is_err());
        let mut bad = zero;
        bad[0] = P;
        assert!(rr9_public_norm_support(&bad, Some(PRIME31.g), &h).is_err());
    }
    #[test]
    fn rr9_seven_anchor_generic_equivalence_on_actual_prime31() {
        let mut ar = Arithmetic::default();
        check_curve(PRIME31,&mut ar).unwrap();
        let report = rr9_seven_anchor_completion_audit(&mut ar);
        assert_eq!(report["b8_sign_masks_total"],256);
        assert!(report["b8_positive_checked"].as_u64().unwrap()>=250);
        assert_eq!(report["explicit_R_and_minus_R_jet_fixtures"],2);
        assert_eq!(report["negative_off_base_completed"],4);
        assert_eq!(report["b31_given_support_completions"],64);
        assert_eq!(report["public_only_relation_extraction"],false);
        assert_eq!(report["complete_unknown_target_ecdlp_recoveries"],0);
    }
    #[test]
    fn rr9_b8_exceptional_x_jet_fibre_equivalence() {
        let mut ar = Arithmetic::default();
        check_curve(PRIME31,&mut ar).unwrap();
        let report = rr9_b8_exceptional_fibre_audit(&mut ar);
        assert_eq!(report["exceptional_branch_count"],2);
        for panel in report["panels"].as_array().unwrap() {
            assert_eq!(panel["exact_signed_fibre_set"],true);
            assert_eq!(panel["negative_systems"],256);
            assert_eq!(panel["exceptional_polynomial_factor_checks"],128);
        }
        assert!(report["panels"][1]["tangent_derivative_checks"].as_u64().unwrap()>=1);
    }
    #[test]
    fn rr9_b8_full_signed_fibre_equivalence() {
        let mut ar = Arithmetic::default();
        check_curve(PRIME31, &mut ar).unwrap();
        let report = rr9_b8_linear_fibre_audit(&mut ar);
        assert_eq!(report["formal_sign_masks"], 256);
        assert_eq!(report["negative_affine_linear_systems"], 1024);
        assert_eq!(report["independent_split_quadric_identity_checks"], 2048);
        assert_eq!(report["algorithm_enumerates_all_256_sign_masks"], true);
        assert_eq!(report["nonenumerative_extraction_demonstrated"], false);
    }
    #[test]
    fn rr9_original_prime31_bases_and_duplicate_control() {
        let data = run();
        assert_eq!(data["positive_count"], 64);
        assert_eq!(data["negative_count"], 64);
        for panel in data["panels"].as_array().unwrap() {
            assert_eq!(panel["accepted_positive"], 32);
            assert_eq!(panel["accepted_negative"], 32);
            for case in panel["cases"].as_array().unwrap() {
                assert_eq!(case["norm_certificate"]["verified"], true);
                assert_eq!(case["public_norm_support_true"], true);
                assert_eq!(case["signed_target_chart_valid"], true);
                assert_eq!(case["target_preserving_norm_mutation_rejected"], true);
                assert_eq!(
                    case["norm_certificate"]["norm_coefficients_ascending"]
                        .as_array().unwrap().len(), 10);
            }
            assert_eq!(panel["naive_repeated_row_false_zero"], true);
            assert_eq!(panel["safe_repeated_row_rejected"], true);
            assert_eq!(panel["confluent_hermite_repeated_true_det"], 0);
            assert_ne!(panel["confluent_hermite_repeated_false_det"], 0);
            assert_eq!(panel["confluent_hermite_double_control"], true);
            let h = panel["confluent_hermite_higher_controls"].as_array().unwrap();
            assert_eq!(h.len(), 3);
            for (i, &m) in [3, 4, 8].iter().enumerate() {
                assert_eq!(h[i]["multiplicity"], m);
                assert_eq!(h[i]["true_det"], 0);
                assert_ne!(h[i]["wrong_det"], 0);
                assert_eq!(h[i]["norm_certificate"]["verified"], true);
                assert_eq!(h[i]["public_norm_support_true"], true);
                assert_eq!(h[i]["signed_target_chart_valid"], true);
                assert_eq!(h[i]["target_preserving_norm_mutation_rejected"], true);
            }
            assert_ne!(panel["ninth_jet_wrong_det"], 0);
            assert_eq!(panel["ninth_jet_original_curve_negative"], true);
        }
    }
}
