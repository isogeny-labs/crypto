//! Independent standalone Rust qualifier for `AnchoredSolveBundle` (`cover.anchored-multi-pattern.dlp/v1`).
//!
//! Verifies from first principles without trusting any producer summary counter:
//! 1. Prime moduli `(p, l)`, `foreign_hook_calls == 0`, elliptic curve nonsingularity,
//!    subgroup order `[l]G == O` and `[l]Q == O`, cover `y^2 = H(x)` squarefreeness,
//!    factor-base point validity, and Jacobian order `[l]G_J == 0`, `[l]Q_J == 0`.
//! 2. Every referenced Phase-1 source certificate: polynomial norm `A^2 - C^2 H == norm`,
//!    distinct small-point ordinate bindings, irreducible `F_p`-quadratic Mumford remainders,
//!    and Cantor Jacobian group identity `sum s_i B_i + sum c_r D(r) == 0`.
//! 3. Every anchor witness used in descent: exact integer cancellation of all large classes
//!    except `+(scale) * D(class)` (`scale in {1, 2}`), plus Cantor Jacobian verification.
//! 4. Every `AnchoredDescentCert`: residual `[a]G_J + [b]Q_J == R`, polynomial Weil identity
//!    `C^2 H - A^2 == u_R * S`, squarefree sextic factorization matching `cert.pattern`,
//!    linear/quadratic zero bindings, exact integer expansion `expanded_row`, and Cantor
//!    Jacobian identity `[scale] R == sum d_k B_k`.
//! 5. Affine kernel projection consistency modulo `l`, unique proportionality constant `alpha`,
//!    and when `verified == true`, elliptic curve discrete log `[d]G == Q` on `E(F_{p^6})`.
//! 6. Exact `bitop/v1` phase repricing and sensitivity-band conservation.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use crypto_lib::cryptanalysis::jv_anchored_descent::{
    canonical_quadratic_class, class_to_divisor, verify_expanded_jacobian, SexticPattern,
};
use crypto_lib::cryptanalysis::jv_bitops::{self as work, Kind, OpTally};
use crypto_lib::cryptanalysis::jv_cover::{BaseEl, Div, EllE, Fld, Fq, Fq3, Hyp, PtE6, E2, E6};
use crypto_lib::cryptanalysis::jv_sieve::two_large_prime::extension_anchored_dlp::{
    AnchoredSolveBundle, ReferencedSource,
};
use crypto_lib::cryptanalysis::jv_sieve::{fdeg, FpRing};
use crypto_lib::cryptanalysis::residual_walk::{inv_mod, is_prime_u64};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct CheckItem {
    pub name: String,
    pub passed: bool,
    pub detail: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct BundleQualification {
    pub path: String,
    pub arm_label: String,
    pub p: u64,
    pub l: u64,
    pub seed: u64,
    pub bundle_verified_flag: bool,
    pub all_passed: bool,
    pub checks: Vec<CheckItem>,
}

fn decode_e6(raw: &[[u64; 2]; 3]) -> E6 {
    E6([E2(raw[0]), E2(raw[1]), E2(raw[2])])
}

fn decode_poly(raw: &[[u64; 2]]) -> Vec<E2> {
    let mut v: Vec<E2> = raw.iter().copied().map(E2).collect();
    while v.last() == Some(&E2::ZERO) {
        v.pop();
    }
    v
}

fn poly_add(f: &Fq, a: &[E2], b: &[E2]) -> Vec<E2> {
    let mut out: Vec<E2> = (0..a.len().max(b.len()))
        .map(|i| f.add(a.get(i).unwrap_or(&E2::ZERO), b.get(i).unwrap_or(&E2::ZERO)))
        .collect();
    while out.last() == Some(&E2::ZERO) {
        out.pop();
    }
    out
}

fn poly_sub(f: &Fq, a: &[E2], b: &[E2]) -> Vec<E2> {
    let mut out: Vec<E2> = (0..a.len().max(b.len()))
        .map(|i| f.sub(a.get(i).unwrap_or(&E2::ZERO), b.get(i).unwrap_or(&E2::ZERO)))
        .collect();
    while out.last() == Some(&E2::ZERO) {
        out.pop();
    }
    out
}

fn poly_mul(f: &Fq, a: &[E2], b: &[E2]) -> Vec<E2> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut out = vec![E2::ZERO; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            out[i + j] = f.add(&out[i + j], &f.mul(x, y));
        }
    }
    while out.last() == Some(&E2::ZERO) {
        out.pop();
    }
    out
}

fn poly_rem(f: &Fq, a: &[E2], m: &[E2]) -> Option<Vec<E2>> {
    if m.is_empty() {
        return None;
    }
    if m.len() == 1 {
        return (m[0] != E2::ZERO).then_some(Vec::new());
    }
    let lead = *m.last()?;
    if lead == E2::ZERO {
        return None;
    }
    let lead_inv = f.inv(&lead);
    let mut r = a.to_vec();
    while r.last() == Some(&E2::ZERO) {
        r.pop();
    }
    while r.len() >= m.len() {
        let shift = r.len() - m.len();
        let coef = f.mul(r.last()?, &lead_inv);
        for (i, mc) in m.iter().enumerate() {
            r[shift + i] = f.sub(&r[shift + i], &f.mul(&coef, mc));
        }
        while r.last() == Some(&E2::ZERO) {
            r.pop();
        }
    }
    Some(r)
}

fn poly_gcd(f: &Fq, a: &[E2], b: &[E2]) -> Vec<E2> {
    let mut x = a.to_vec();
    let mut y = b.to_vec();
    while !y.is_empty() {
        let Some(r) = poly_rem(f, &x, &y) else {
            return Vec::new();
        };
        x = y;
        y = r;
    }
    if let Some(&lead) = x.last() {
        if lead != E2::ZERO {
            let li = f.inv(&lead);
            for c in &mut x {
                *c = f.mul(c, &li);
            }
        }
    }
    x
}

fn poly_eval_fp(f: &Fq, a: &[E2], x: u64) -> E2 {
    a.iter()
        .rev()
        .fold(E2::ZERO, |acc, c| f.add(&f.scale(&acc, x), c))
}

fn verify_referenced_source(
    f: &Fq,
    hx: &[E2],
    jac: &Hyp<Fq>,
    base: &[BaseEl],
    src: &ReferencedSource,
) -> Result<(), String> {
    let p = f.p;
    if src.norm.len() != 10 || src.norm[9] == 0 || src.norm.iter().any(|&v| v >= p) {
        return Err(format!("source {}: invalid degree-9 norm", src.source_id));
    }
    let a = decode_poly(&src.a);
    let c = decode_poly(&src.c);
    if a.len() > 5 || c.len() != 2 || c[1] == E2::ZERO {
        return Err(format!("source {}: invalid (A, C) degrees", src.source_id));
    }
    let norm_e2: Vec<E2> = src.norm.iter().map(|&v| E2([v, 0])).collect();
    let lhs = poly_sub(
        f,
        &poly_mul(f, &a, &a),
        &poly_mul(f, &poly_mul(f, &c, &c), hx),
    );
    if lhs != norm_e2 {
        return Err(format!("source {}: A^2 - C^2 H != norm", src.source_id));
    }

    let mut prod_fp = vec![1u64];
    let ring = FpRing::new(p);
    let mut prev_x = None;
    for &(b_idx, sign) in &src.terms {
        let b = base
            .get(b_idx)
            .ok_or_else(|| format!("source {}: base index out of range", src.source_id))?;
        if !matches!(sign, -1 | 1) {
            return Err(format!("source {}: non-unit small sign", src.source_id));
        }
        if prev_x.is_some_and(|px| b.x <= px) {
            return Err(format!(
                "source {}: unordered small abscissae",
                src.source_id
            ));
        }
        prev_x = Some(b.x);
        let den = poly_eval_fp(f, &c, b.x);
        let y = if sign == 1 { b.y } else { f.neg(&b.y) };
        if den == E2::ZERO || f.add(&poly_eval_fp(f, &a, b.x), &f.mul(&den, &y)) != E2::ZERO {
            return Err(format!("source {}: ordinate binding failed", src.source_id));
        }
        let lin = vec![if b.x == 0 { 0 } else { p - b.x }, 1];
        prod_fp = ring.mul(&prod_fp, &lin);
    }

    if src.terms.len() + 2 * src.remainders.len() != 9 || src.remainders.len() > 3 {
        return Err(format!("source {}: support degree != 9", src.source_id));
    }
    for &(key, r_coeff) in &src.remainders {
        if !matches!(r_coeff, -1 | 1) || key.iter().any(|&v| v >= p) {
            return Err(format!(
                "source {}: invalid remainder metadata",
                src.source_id
            ));
        }
        let u_fp = [key[0], key[1], 1u64];
        let disc = ((u_fp[1] as u128 * u_fp[1] as u128 + 4 * p as u128 * p as u128
            - 4 * u_fp[0] as u128)
            % p as u128) as u64;
        if disc == 0 || f.pow(&f.from_fp(disc), (p - 1) as u128 / 2) != f.from_fp(p - 1) {
            return Err(format!(
                "source {}: remainder U not irreducible",
                src.source_id
            ));
        }
        prod_fp = ring.mul(&prod_fp, &u_fp);

        let u_e2 = vec![E2([u_fp[0], 0]), E2([u_fp[1], 0]), E2::ONE];
        let v_stored = vec![E2([key[2], key[3]]), E2([key[4], key[5]])];
        let v_principal: Vec<E2> = if r_coeff == 1 {
            v_stored
        } else {
            v_stored.iter().map(|z| f.neg(z)).collect()
        };
        let check_ac = poly_add(f, &a, &poly_mul(f, &c, &v_principal));
        if !poly_rem(f, &check_ac, &u_e2).ok_or("rem")?.is_empty() {
            return Err(format!(
                "source {}: A + C*V != 0 (mod U) on remainder",
                src.source_id
            ));
        }
        let check_mum = poly_sub(f, &poly_mul(f, &v_principal, &v_principal), hx);
        if !poly_rem(f, &check_mum, &u_e2).ok_or("rem")?.is_empty() {
            return Err(format!("source {}: V^2 != H (mod U)", src.source_id));
        }
        let (canon_k, canon_s) = canonical_quadratic_class(f, &u_fp, &v_principal);
        if canon_k != key || canon_s != r_coeff {
            return Err(format!(
                "source {}: non-canonical remainder key",
                src.source_id
            ));
        }
    }

    // `prod_fp` is monic degree 9; `src.norm` is `lead * prod_fp`.
    let lead = src.norm[9];
    let scaled_prod = ring.scale(&prod_fp, lead);
    if scaled_prod != src.norm {
        return Err(format!(
            "source {}: norm != lead * prod(linear) * prod(quadratics)",
            src.source_id
        ));
    }

    // Cantor Jacobian group law check: `sum s_i B_i + sum c_r D(r) == 0`.
    let mut acc = jac.identity();
    for &(b_idx, sign) in &src.terms {
        let d = if sign == 1 {
            base[b_idx].d.clone()
        } else {
            jac.neg(&base[b_idx].d)
        };
        acc = jac.add(&acc, &d);
    }
    for &(key, r_coeff) in &src.remainders {
        let d0 = class_to_divisor(&key);
        if !jac.is_valid(&d0) {
            return Err(format!("source {}: invalid Mumford divisor", src.source_id));
        }
        let d = if r_coeff == 1 { d0 } else { jac.neg(&d0) };
        acc = jac.add(&acc, &d);
    }
    if !jac.is_identity(&acc) {
        return Err(format!("source {}: Cantor sum != 0", src.source_id));
    }
    Ok(())
}

pub fn qualify_bundle(path: &Path, bundle: &AnchoredSolveBundle) -> BundleQualification {
    work::enable(false);
    let mut checks = Vec::<CheckItem>::new();
    let p = bundle.p;
    let l = bundle.l;

    // 1. Domain, curve, cover, factor base, and zero foreign hooks.
    let domain_res: Result<(Fq3, E6, Vec<E2>, Vec<BaseEl>, Div<E2>, Div<E2>), String> = (|| {
        if bundle.schema != "cover.anchored-multi-pattern.dlp/v1" {
            return Err(format!("unexpected schema {}", bundle.schema));
        }
        if !is_prime_u64(p) || !is_prime_u64(l) || p < 5 || l < 3 {
            return Err("non-prime p or l".into());
        }
        if bundle.accounting.foreign_hook_calls != 0 {
            return Err(format!(
                "nonzero foreign_hook_calls = {}",
                bundle.accounting.foreign_hook_calls
            ));
        }
        let f6 = Fq3::new(p);
        let alpha = decode_e6(&bundle.instance_alpha);
        if alpha.in_fq() {
            return Err("alpha lies in F_{p^2}".into());
        }
        // Model: y^2 = x(x - alpha)(x - sigma(alpha)); nonsingular iff roots 0, alpha, sigma(alpha) are distinct.
        let sa = f6.sigma(&alpha);
        if alpha == E6::ZERO || sa == E6::ZERO || alpha == sa {
            return Err("singular elliptic curve".into());
        }
        let ec = EllE::new(&f6, &alpha);
        let g = PtE6 {
            x: decode_e6(&bundle.instance_g.0),
            y: decode_e6(&bundle.instance_g.1),
            inf: false,
        };
        let q = PtE6 {
            x: decode_e6(&bundle.instance_q.0),
            y: decode_e6(&bundle.instance_q.1),
            inf: false,
        };
        if !ec.on_curve(&g) || !ec.on_curve(&q) {
            return Err("G or Q not on E(F_{p^6})".into());
        }
        if !ec.mul(&g, l).inf || !ec.mul(&q, l).inf {
            return Err("[l]G != O or [l]Q != O".into());
        }

        let hx = decode_poly(&bundle.cover_hx);
        if hx.len() != 8 || hx[7] != E2::ONE {
            return Err("cover H(x) is not monic degree 7".into());
        }
        let hx_deriv: Vec<E2> = hx
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, c)| f6.f.scale(c, (i as u64) % p))
            .collect();
        if poly_gcd(&f6.f, &hx, &hx_deriv) != vec![E2::ONE] {
            return Err("cover H(x) is not squarefree".into());
        }
        // Build `Hyp` with owned leaked `Fq`? Wait: `f6` lives inside this closure; build `BaseEl` first
        // and verify `gj, qj` right here!
        let jac = Hyp {
            f: &f6.f,
            h: hx.clone(),
            g: 3,
        };
        let mut base = Vec::with_capacity(bundle.base_points.len());
        let mut prev_x = None;
        for &[x, y0, y1] in &bundle.base_points {
            if x >= p || y0 >= p || y1 >= p {
                return Err("base coordinate >= p".into());
            }
            if prev_x.is_some_and(|px| x <= px) {
                return Err("base abscissae not strictly ascending".into());
            }
            prev_x = Some(x);
            let y = E2([y0, y1]);
            if y == E2::ZERO || f6.f.sq(&y) != poly_eval_fp(&f6.f, &hx, x) {
                return Err("base point not on cover".into());
            }
            let d = Div {
                u: vec![f6.f.neg(&f6.f.from_fp(x)), E2::ONE],
                v: vec![y],
            };
            if !jac.is_valid(&d) {
                return Err("invalid base divisor".into());
            }
            base.push(BaseEl { x, y, d });
        }
        let gj = Div {
            u: decode_poly(&bundle.gj_u),
            v: decode_poly(&bundle.gj_v),
        };
        let qj = Div {
            u: decode_poly(&bundle.qj_u),
            v: decode_poly(&bundle.qj_v),
        };
        if !jac.is_valid(&gj)
            || !jac.is_valid(&qj)
            || jac.is_identity(&gj)
            || !jac.is_identity(&jac.mul(&gj, l as u128))
            || !jac.is_identity(&jac.mul(&qj, l as u128))
        {
            return Err("invalid transferred Jacobian points G_J or Q_J".into());
        }
        Ok((f6, alpha, hx, base, gj, qj))
    })();

    let (f6, alpha, hx, base, gj, qj) = match domain_res {
        Ok(t) => {
            checks.push(CheckItem {
                name: "domain_curve_cover_and_zero_foreign_hooks".into(),
                passed: true,
                detail: format!("p={p}, l={l}, |B|={}", t.3.len()),
            });
            t
        }
        Err(e) => {
            checks.push(CheckItem {
                name: "domain_curve_cover_and_zero_foreign_hooks".into(),
                passed: false,
                detail: e,
            });
            return BundleQualification {
                path: path.display().to_string(),
                arm_label: bundle.arm_label.clone(),
                p,
                l,
                seed: bundle.seed,
                bundle_verified_flag: bundle.verified,
                all_passed: false,
                checks,
            };
        }
    };
    let f_jac = Fq::new(p);
    let jac = Hyp {
        f: &f_jac,
        h: hx,
        g: 3,
    };

    // 2. Referenced Phase-1 sources: polynomial norm + Mumford + Cantor group law.
    let ref_res: Result<(), String> = (|| {
        for (&id, src) in &bundle.referenced_sources {
            if src.source_id != id {
                return Err(format!("key {id} != source_id {}", src.source_id));
            }
            verify_referenced_source(jac.f, &jac.h, &jac, &base, src)?;
        }
        Ok(())
    })();
    checks.push(match ref_res {
        Ok(()) => CheckItem {
            name: "referenced_sources_norm_and_cantor".into(),
            passed: true,
            detail: format!(
                "{} referenced Phase-1 sources verified",
                bundle.referenced_sources.len()
            ),
        },
        Err(e) => CheckItem {
            name: "referenced_sources_norm_and_cantor".into(),
            passed: false,
            detail: e,
        },
    });

    // 3. Anchor witness exact integer cofactor cancellation + Cantor Jacobian check.
    let anc_res: Result<usize, String> = (|| {
        let mut checked_anchors = 0usize;
        for cert in &bundle.descent_certs {
            for &(class_key, c_sign, a_scale, ref witness) in &cert.quadratic_terms {
                if !matches!(c_sign, -1 | 1) || !matches!(a_scale, 1 | 2) || witness.is_empty() {
                    return Err("invalid anchor metadata on descent cert".into());
                }
                let mut small = BTreeMap::<usize, i128>::new();
                let mut large = BTreeMap::<[u64; 6], i128>::new();
                for &(src_id, w) in witness {
                    let src = bundle
                        .referenced_sources
                        .get(&src_id)
                        .ok_or_else(|| format!("missing referenced source {src_id}"))?;
                    for &(b_idx, sgn) in &src.terms {
                        *small.entry(b_idx).or_default() += (w as i128) * (sgn as i128);
                    }
                    for &(rk, rc) in &src.remainders {
                        *large.entry(rk).or_default() += (w as i128) * (rc as i128);
                    }
                }
                large.retain(|_, v| *v != 0);
                if large.len() != 1 || large.get(&class_key) != Some(&(a_scale as i128)) {
                    return Err(format!(
                        "anchor witness failed exact large cancellation: got {large:?}"
                    ));
                }
                let d_class = class_to_divisor(&class_key);
                let neg_d = jac.neg(&d_class);
                // `small + a_scale * D(class) == 0 => a_scale * (-D(class)) == small`
                let small_vec: Vec<(usize, i64)> = small
                    .into_iter()
                    .filter(|&(_, v)| v != 0)
                    .map(|(k, v)| (k, v as i64))
                    .collect();
                if !verify_expanded_jacobian(&jac, &base, &neg_d, a_scale, &small_vec) {
                    return Err("anchor witness failed Cantor Jacobian replay".into());
                }
                checked_anchors += 1;
            }
        }
        Ok(checked_anchors)
    })();
    checks.push(match anc_res {
        Ok(n) => CheckItem {
            name: "anchor_witness_cofactor_cancellation".into(),
            passed: true,
            detail: format!("{n} anchor witnesses cancelled & Cantor-verified"),
        },
        Err(e) => CheckItem {
            name: "anchor_witness_cofactor_cancellation".into(),
            passed: false,
            detail: e,
        },
    });

    // 4. Descent certificates: residual scalar combo, Weil norm, squarefree factorization, Cantor check.
    let desc_res: Result<usize, String> = (|| {
        let ring = FpRing::new(p);
        for cert in &bundle.descent_certs {
            if !matches!(cert.scale, 1 | 2) {
                return Err("invalid cert scale".into());
            }
            let r_div = Div {
                u: decode_poly(&cert.residual_u),
                v: decode_poly(&cert.residual_v),
            };
            let r_want = jac.add(
                &jac.mul(&gj, cert.a_scalar as u128),
                &jac.mul(&qj, cert.b_scalar as u128),
            );
            if r_div != r_want {
                return Err(format!(
                    "residual {} != [a]G_J + [b]Q_J",
                    cert.residual_index
                ));
            }
            if cert.sextic_s[6] != 1 || cert.sextic_s.iter().any(|&v| v >= p) {
                return Err("sextic_s not monic over F_p".into());
            }
            // Squarefree check on S(x).
            let mut s_deriv = Vec::new();
            for (i, &c) in cert.sextic_s.iter().enumerate().skip(1) {
                s_deriv.push(((i as u64) * c) % p);
            }
            while s_deriv.last() == Some(&0) {
                s_deriv.pop();
            }
            if fdeg(&ring.gcd(&cert.sextic_s, &s_deriv)) != 0 {
                return Err("sextic_s is not squarefree".into());
            }

            let a_poly = decode_poly(&cert.a_poly);
            let c_poly = decode_poly(&cert.c_poly);
            if a_poly.len() > 5 || c_poly.len() != 2 || c_poly[1] != E2::ONE {
                return Err("invalid descent (A, C) shape".into());
            }
            let s_e2: Vec<E2> = cert.sextic_s.iter().map(|&v| E2([v, 0])).collect();
            let lhs = poly_sub(
                jac.f,
                &poly_mul(jac.f, &poly_mul(jac.f, &c_poly, &c_poly), &jac.h),
                &poly_mul(jac.f, &a_poly, &a_poly),
            );
            let rhs = poly_mul(jac.f, &r_div.u, &s_e2);
            if lhs != rhs {
                return Err("C^2 H - A^2 != u_R * S".into());
            }

            // Pattern & exact polynomial factorization over F_p[x].
            let expected_counts = match cert.pattern {
                SexticPattern::P1_6 => (6, 0),
                SexticPattern::P1_4_2 => (4, 1),
                SexticPattern::P1_2_2_2 => (2, 2),
                SexticPattern::P2_3 => (0, 3),
            };
            if (cert.linear_terms.len(), cert.quadratic_terms.len()) != expected_counts {
                return Err("term counts do not match declared SexticPattern".into());
            }

            let mut fac_fp = vec![1u64];
            for &(b_idx, sigma) in &cert.linear_terms {
                let b = base.get(b_idx).ok_or("descent base index")?;
                if !matches!(sigma, -1 | 1) {
                    return Err("invalid linear sign".into());
                }
                let lin = vec![if b.x == 0 { 0 } else { p - b.x }, 1];
                fac_fp = ring.mul(&fac_fp, &lin);
                // Note: if `C(b.x) != 0`, check `A(b.x) + sigma * C(b.x) * y_B == 0`.
                let den = poly_eval_fp(jac.f, &c_poly, b.x);
                if den != E2::ZERO {
                    let y = if sigma == 1 { b.y } else { jac.f.neg(&b.y) };
                    if jac
                        .f
                        .add(&poly_eval_fp(jac.f, &a_poly, b.x), &jac.f.mul(&den, &y))
                        != E2::ZERO
                    {
                        return Err("descent linear ordinate mismatch".into());
                    }
                }
            }
            for &(key, c_sign, _, _) in &cert.quadratic_terms {
                let u_fp = [key[0], key[1], 1u64];
                fac_fp = ring.mul(&fac_fp, &u_fp);
                let u_e2 = vec![E2([u_fp[0], 0]), E2([u_fp[1], 0]), E2::ONE];
                let v_stored = vec![E2([key[2], key[3]]), E2([key[4], key[5]])];
                let v_zero: Vec<E2> = if c_sign == 1 {
                    v_stored
                } else {
                    v_stored.iter().map(|z| jac.f.neg(z)).collect()
                };
                let check_ac = poly_add(jac.f, &a_poly, &poly_mul(jac.f, &c_poly, &v_zero));
                if !poly_rem(jac.f, &check_ac, &u_e2).ok_or("rem")?.is_empty() {
                    return Err("descent quadratic A + C*V != 0 (mod U)".into());
                }
            }
            if fac_fp != cert.sextic_s {
                return Err("sextic_s != product of linear and quadratic factors".into());
            }

            // Recompute integer `expanded_row` from `linear_terms` and `referenced_sources`.
            let s_i64 = cert.scale as i64;
            let mut row_acc = BTreeMap::<usize, i64>::new();
            for &(b_idx, sigma) in &cert.linear_terms {
                *row_acc.entry(b_idx).or_default() -= s_i64 * (sigma as i64);
            }
            for &(_, c_sign, a_scale, ref witness) in &cert.quadratic_terms {
                let mult = (s_i64 / (a_scale as i64)) * (c_sign as i64);
                for &(src_id, w) in witness {
                    let src = bundle
                        .referenced_sources
                        .get(&src_id)
                        .ok_or("missing referenced source")?;
                    for &(b_idx, st) in &src.terms {
                        *row_acc.entry(b_idx).or_default() += mult * (w as i64) * (st as i64);
                    }
                }
            }
            let want_row: Vec<(usize, i64)> =
                row_acc.into_iter().filter(|&(_, c)| c != 0).collect();
            if want_row != cert.expanded_row {
                return Err("expanded_row does not match exact witness combination".into());
            }
            if !verify_expanded_jacobian(&jac, &base, &r_div, cert.scale, &cert.expanded_row) {
                return Err("descent expanded_row failed Cantor [scale]R == sum d_k B_k".into());
            }
        }
        Ok(bundle.descent_certs.len())
    })();
    checks.push(match desc_res {
        Ok(n) => CheckItem {
            name: "descent_certificates_weil_and_jacobian".into(),
            passed: true,
            detail: format!("{n} descent certificates verified"),
        },
        Err(e) => CheckItem {
            name: "descent_certificates_weil_and_jacobian".into(),
            passed: false,
            detail: e,
        },
    });

    // 5. Affine projection & elliptic discrete logarithm `[d]G == Q`.
    let dlp_res: Result<String, String> = (|| {
        if bundle.descent_certs.len() != bundle.affine_rows.len() {
            return Err("descent_certs / affine_rows length mismatch".into());
        }
        if let Some(kernel) = &bundle.initial_kernel {
            if kernel.len() != base.len()
                || kernel.last() != Some(&1)
                || kernel.iter().any(|&v| v >= l)
            {
                return Err("invalid homogeneous kernel shape".into());
            }
            for (cert, row) in bundle.descent_certs.iter().zip(&bundle.affine_rows) {
                let mut dot = 0u128;
                for &(idx, coef) in &cert.expanded_row {
                    let v = (coef as i128).rem_euclid(l as i128) as u128;
                    dot = (dot + v * (kernel[idx] as u128)) % (l as u128);
                }
                let want_target = if cert.b_scalar == 0 {
                    0
                } else {
                    (((l - cert.b_scalar) as u128 * (cert.scale as u128)) % (l as u128)) as u64
                };
                let want_rhs =
                    (((cert.a_scalar as u128) * (cert.scale as u128)) % (l as u128)) as u64;
                if row.projected[0] != dot as u64
                    || row.projected[1] != want_target
                    || row.projected_rhs != want_rhs
                {
                    return Err("affine row projection mismatch".into());
                }
            }
        } else if bundle.verified {
            return Err("verified bundle missing initial_kernel".into());
        }

        if bundle.verified {
            let d = bundle
                .recovered_scalar
                .ok_or("verified==true but recovered_scalar is None")?;
            if d == 0 || d >= l {
                return Err("recovered_scalar out of (0, l)".into());
            }
            if bundle.affine_rows.len() < 2 {
                return Err("fewer than 2 affine rows for 2-unknown affine system".into());
            }
            // Recover unique proportionality constant `alpha` and check all projected rows.
            let mut alpha_opt: Option<u64> = None;
            for row in &bundle.affine_rows {
                let k0 = row.projected[0];
                let t1 = row.projected[1];
                let td = ((t1 as u128 * d as u128) % (l as u128)) as u64;
                let rem_rhs = (row.projected_rhs + l - td) % l;
                if k0 != 0 {
                    let al = ((rem_rhs as u128 * inv_mod(k0, l) as u128) % (l as u128)) as u64;
                    if let Some(prev) = alpha_opt {
                        if prev != al {
                            return Err("inconsistent affine proportionality alpha".into());
                        }
                    } else {
                        alpha_opt = Some(al);
                    }
                } else if rem_rhs != 0 {
                    return Err("inconsistent zero-k0 affine row".into());
                }
            }
            if alpha_opt.is_none() {
                return Err("no nonzero kernel coefficient in affine rows".into());
            }
            let g_pt = PtE6 {
                x: decode_e6(&bundle.instance_g.0),
                y: decode_e6(&bundle.instance_g.1),
                inf: false,
            };
            let q_pt = PtE6 {
                x: decode_e6(&bundle.instance_q.0),
                y: decode_e6(&bundle.instance_q.1),
                inf: false,
            };
            let ec = EllE::new(&f6, &alpha);
            if ec.mul(&g_pt, d) != q_pt {
                return Err("[d]G != Q on E(F_{p^6})".into());
            }
            Ok(format!("verified [d]G == Q with d = {d}"))
        } else {
            if bundle.recovered_scalar.is_some() {
                return Err("verified==false but recovered_scalar is Some".into());
            }
            Ok(format!(
                "unsolved/capped bundle cleanly recorded ({:?})",
                bundle.refusal
            ))
        }
    })();
    checks.push(match dlp_res {
        Ok(msg) => CheckItem {
            name: "affine_projection_and_elliptic_dlp".into(),
            passed: true,
            detail: msg,
        },
        Err(e) => CheckItem {
            name: "affine_projection_and_elliptic_dlp".into(),
            passed: false,
            detail: e,
        },
    });

    // 6. Exact `bitop/v1` repricing conservation.
    let price_res: Result<(), String> = (|| {
        let widths = work::widths(p, l);
        let mut sum_obs = 0u128;
        for ph in &bundle.accounting.phases {
            let mut t = OpTally::default();
            t.counts[Kind::FieldMul as usize] = ph.field_mul;
            t.counts[Kind::FieldAdd as usize] = ph.field_add;
            t.counts[Kind::FieldInv as usize] = ph.field_inv;
            t.counts[Kind::ModrMul as usize] = ph.modr_mul;
            t.counts[Kind::ModrAdd as usize] = ph.modr_add;
            t.counts[Kind::ModrInv as usize] = ph.modr_inv;
            t.counts[Kind::WordOp as usize] = ph.word_op;
            t.counts[Kind::HashProbe as usize] = ph.hash_probe;
            t.rng_words = ph.rng_words;
            let got = work::price(&t, &widths).map_err(|e| format!("pricing error: {e:?}"))?;
            if got != ph.bitops {
                return Err(format!(
                    "phase {} bitops mismatch: {got} != {}",
                    ph.phase, ph.bitops
                ));
            }
            sum_obs += got;
        }
        if sum_obs != bundle.accounting.observed_bitops {
            return Err("observed_bitops != sum(phase.bitops)".into());
        }
        let w0 = sum_obs + bundle.accounting.uniform_priced_bitops;
        let u = bundle.accounting.uniform_unpriced_ops as u128;
        for (name, band) in [
            ("schoolbook", &bundle.accounting.schoolbook_band),
            ("karatsuba", &bundle.accounting.karatsuba_band),
        ] {
            if band.w0_bitops != w0
                || band.w0_1_bitops != w0 + (u * band.gae_bitops) / 10
                || band.w1_bitops != w0 + u * band.gae_bitops
            {
                return Err(format!("{name} sensitivity band arithmetic mismatch"));
            }
        }
        Ok(())
    })();
    checks.push(match price_res {
        Ok(()) => CheckItem {
            name: "bitop_v1_repricing_conservation".into(),
            passed: true,
            detail: format!("observed_bitops = {}", bundle.accounting.observed_bitops),
        },
        Err(e) => CheckItem {
            name: "bitop_v1_repricing_conservation".into(),
            passed: false,
            detail: e,
        },
    });

    let all_passed = checks.iter().all(|c| c.passed);
    BundleQualification {
        path: path.display().to_string(),
        arm_label: bundle.arm_label.clone(),
        p,
        l,
        seed: bundle.seed,
        bundle_verified_flag: bundle.verified,
        all_passed,
        checks,
    }
}

fn collect_json_bundles(target: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    if target.is_file() {
        out.push(target.to_path_buf());
        return Ok(());
    }
    let mut entries: Vec<_> = fs::read_dir(target)
        .map_err(|e| format!("read_dir {}: {e}", target.display()))?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            collect_json_bundles(&p, out)?;
        } else if p
            .file_name()
            .and_then(|s| s.to_str())
            .is_some_and(|n| n.starts_with("bundle_") && n.ends_with(".json"))
        {
            out.push(p);
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!(
            "usage: {} <bundle_or_dir> [--out <qualification.json>]",
            args[0]
        );
        std::process::exit(2);
    }
    let target = PathBuf::from(&args[1]);
    let out_path = if args.len() >= 4 && args[2] == "--out" {
        Some(PathBuf::from(&args[3]))
    } else {
        None
    };

    let mut files = Vec::new();
    if let Err(e) = collect_json_bundles(&target, &mut files) {
        eprintln!("error collecting bundles: {e}");
        std::process::exit(1);
    }
    if files.is_empty() {
        eprintln!("no bundle_*.json files found under {}", target.display());
        std::process::exit(1);
    }

    let mut reports = Vec::with_capacity(files.len());
    let mut verified_bundles = 0usize;
    let mut failed_bundles = 0usize;
    for file in &files {
        let raw = fs::read_to_string(file).unwrap_or_else(|e| {
            eprintln!("failed to read {}: {e}", file.display());
            std::process::exit(1);
        });
        let bundle: AnchoredSolveBundle = serde_json::from_str(&raw).unwrap_or_else(|e| {
            eprintln!("failed to parse {}: {e}", file.display());
            std::process::exit(1);
        });
        let rep = qualify_bundle(file, &bundle);
        if rep.bundle_verified_flag {
            verified_bundles += 1;
        }
        if !rep.all_passed {
            failed_bundles += 1;
        }
        reports.push(rep);
    }

    let summary = serde_json::json!({
        "schema": "cover.anchored-multi-pattern.qualification/v1",
        "bundles_checked": reports.len(),
        "verified_solves": verified_bundles,
        "failed_bundles": failed_bundles,
        "all_passed": failed_bundles == 0,
        "results": reports,
    });
    let encoded = serde_json::to_string_pretty(&summary).unwrap();
    if let Some(out) = out_path {
        fs::write(&out, &encoded).expect("write qualification output");
    } else {
        println!("{encoded}");
    }
    if failed_bundles > 0 {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_lib::cryptanalysis::jv_anchored_descent::DescentMode;
    use crypto_lib::cryptanalysis::jv_cover::generate_spec;
    use crypto_lib::cryptanalysis::jv_sieve::two_large_prime::extension_anchored_dlp::{
        run_anchored_ic_dlp_on, AnchoredPipelineOptions, CollectorArm,
    };

    #[test]
    fn qualifier_accepts_genuine_solve_and_rejects_sabotaged_bundles() {
        let truth_spec = generate_spec(307, 101);
        let truth_d = truth_spec.d;
        let mut public_spec = truth_spec;
        public_spec.d = 0;

        let opts = AnchoredPipelineOptions {
            collector: CollectorArm::DeferredPairsFast,
            descent: DescentMode::TracedAnchored,
            ..Default::default()
        };
        let bundle = run_anchored_ic_dlp_on(&public_spec, 101, opts).expect("solve p=307");
        assert_eq!(bundle.recovered_scalar, Some(truth_d));

        let dummy = Path::new("mem_bundle.json");
        let ok = qualify_bundle(dummy, &bundle);
        assert!(ok.all_passed, "genuine bundle failed: {:#?}", ok.checks);

        // Sabotage 1: foreign hook call > 0.
        let mut bad_foreign = bundle.clone();
        bad_foreign.accounting.foreign_hook_calls = 1;
        assert!(!qualify_bundle(dummy, &bad_foreign).all_passed);

        // Sabotage 2: wrong discrete logarithm scalar.
        let mut bad_d = bundle.clone();
        bad_d.recovered_scalar = Some((truth_d % (bundle.l - 1)) + 1 ^ 3);
        if bad_d.recovered_scalar == Some(truth_d) {
            bad_d.recovered_scalar = Some(truth_d - 1);
        }
        assert!(!qualify_bundle(dummy, &bad_d).all_passed);

        // Sabotage 3: corrupt sextic polynomial coefficient.
        let mut bad_poly = bundle.clone();
        bad_poly.descent_certs[0].sextic_s[0] =
            (bad_poly.descent_certs[0].sextic_s[0] + 1) % bundle.p;
        assert!(!qualify_bundle(dummy, &bad_poly).all_passed);

        // Sabotage 4: corrupt bitop/v1 phase subtotal.
        let mut bad_price = bundle.clone();
        bad_price.accounting.phases[0].bitops += 1;
        assert!(!qualify_bundle(dummy, &bad_price).all_passed);
    }
}
