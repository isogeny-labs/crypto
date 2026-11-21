//! Cl(End(E)) for every ordinary census-fp curve with End(E) known, and the
//! order of the class of a prime above each l in {2, 3, 5, 7} that splits or
//! ramifies in End(E) and does not divide its conductor.
//!
//! # Mathematics
//!
//! End(E) is the imaginary quadratic order O of discriminant D = f^2 D_K
//! (`endomorphism_ring.discriminant`, `conductor_f`). Cl(O) is the form class
//! group of primitive positive definite binary quadratic forms of
//! discriminant D (Cox, *Primes of the form x^2 + ny^2*, Thm 7.7): its
//! elements are the h(D) primitive reduced forms (a, b, c), |b| <= a <= c,
//! b >= 0 when |b| = a or a = c, and the law is Dirichlet composition
//! followed by reduction (Cohen, *A course in computational algebraic number
//! theory*, Alg. 5.4.2 and 5.4.7). The invariant factors come from the
//! q-power maps: for each prime q | h, N_j = #{x : x^(q^j) = 1} =
//! q^(sum_i min(e_i, j)) fixes the exponents e_i of the q-primary part.
//!
//! For l prime to f with (D/l) = (D_K/l) != -1, a prime ideal above l
//! corresponds to the form (l, b, c), b^2 = D mod 4l, 0 <= b <= l; its
//! conjugate is (l, -b, c), the inverse class, so the order is one number.
//! It is the size of the crater of the l-volcano of a curve with End(E) = O:
//! Pic(O) acts freely and transitively on the F_p-isomorphism classes with
//! End(E) = O, and the crater through E is the orbit of <[l]> (Kohel 1996).
//!
//! # Checks
//!
//! Every value stored passes, before anything is written:
//!
//! - the input: D = f^2 D_K with D_K fundamental, one (f, D_K) per D;
//! - the group: every composition lands on one of the h reduced forms of
//!   discriminant D; x * x^-1 is the principal form for every x; each N_j is a
//!   power of q, and the q-parts multiply to h;
//! - the class number formula for orders (Cox, Thm 7.24):
//!   h(D) = h(D_K) f / [O_K^* : O^*] * prod_{q | f} (1 - (D_K/q)/q), with
//!   h(D_K) counted from the reduced forms of D_K, must equal #Cl(O) (an
//!   independent identity when f > 1);
//! - h(D_K) equals the stored `frobenius_genus1.class_number` where that is
//!   `proved`;
//! - genus theory (Cox, Prop. 3.11 and Thm 3.15): #Cl(O)[2] = 2^(mu - 1),
//!   mu fixed by the odd primes of D and D mod 32, must equal the 2-rank of
//!   the computed invariant factors;
//! - the l-class: (l, b, c) is primitive, its order divides the exponent, and
//!   it is at most 2 when l ramifies. Independently, a principal ideal of norm
//!   l^k not divisible by l is a primitive representation
//!   4 l^k = X^2 - D y^2 of l^k by the principal form, so such a
//!   representation exists iff ord | k (split) or ord = 1 (ramified, k = 1);
//!   checked for every k <= ord with l^k <= 10^10 (`norm_equation`, or
//!   `norm_equation_partial` when l^ord exceeds the bound);
//! - where ecdb holds the l-volcano (l = 2, 3, 5, `volcano_vertex`), every
//!   census curve with End(E) = O and l ∤ f sits at level 0, and its crater
//!   size equals the order (1 when l is inert);
//! - stored `order_class_group` / `order_ell_class` rows must agree exactly in
//!   every value column ([`compare_existing`]), and a stored
//!   `number_field.class_group` must agree exactly.
//!
//! A discriminant that fails any check is a reported problem and nothing is
//! written for it.
//!
//! # Writes
//!
//! `order_class_group` (one row per discriminant), `order_ell_class` (one row
//! per discriminant and l), both status `proved`, with the independent
//! identities that held in `cross_checks`; `number_field.class_group` where it
//! is NULL and f = 1 (only when the stored class number agrees). Insert or
//! fill only; a rerun inserts nothing.
//!
//! # Report
//!
//! `class_group_discriminants`, `class_group_curves`, `class_groups_refused`
//! (each also a problem), `class_groups_inserted`, `ell_classes_inserted`,
//! `ell_orders_norm_full`, `ell_orders_norm_partial`,
//! `ell_orders_volcano_agree` (pairs (D, l)), `volcano_curves_agree`,
//! `stored_h_dk_agree`, `existing_rows_agree`, `number_field_class_groups_filled`.

use super::{bulk, bulk1, Ctx, Report};
use anyhow::Result;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

/// The small primes l of the l-class orders.
pub const ELLS: [i64; 4] = [2, 3, 5, 7];
/// Largest |D| handled (the census has |D| <= 4p < 4096).
pub const MAX_ABS_D: i64 = 1 << 16;
/// Largest norm l^k searched by the norm-equation cross-check.
pub const NORM_LIMIT: i64 = 10_000_000_000;

const METHOD: &str = "Cl(O) by reduced forms and Dirichlet composition (Cohen 5.4.7), invariant factors from the \
                      q-power maps; checked against the class number formula for orders, genus theory, the norm \
                      equation and the stored l-volcano craters";

const DISC_SQL: &str = "
SELECT er.discriminant::int8, er.conductor_f::int8, fg.cm_discriminant::int8,
       CASE WHEN fg.class_number_status = 'proved' THEN fg.class_number::int8 END, count(*)::int8
FROM ec.curve c
JOIN ec.endomorphism_ring er ON er.curve_id = c.id
JOIN ec.abelian_variety av ON av.curve_id = c.id
JOIN ec.isogeny_class_fq ic ON ic.id = av.isogeny_class_id
JOIN ec.frobenius_genus1 fg ON fg.isogeny_class_id = ic.id
WHERE c.family = 'census-fp' AND c.genus = 1 AND ic.g = 1 AND NOT er.over_closure
  AND er.kind = 'imaginary_quadratic_order' AND er.conductor_f IS NOT NULL AND er.discriminant IS NOT NULL
  AND fg.cm_discriminant IS NOT NULL AND NOT coalesce(fg.is_supersingular, false)
  AND abs(er.discriminant) < $1::int8
GROUP BY 1, 2, 3, 4
ORDER BY 1";

const CRATER_SQL: &str = "
SELECT er.discriminant::int8, iv.ell::int8, iv.crater_size, vv.level, count(*)::int8
FROM ec.curve c
JOIN ec.endomorphism_ring er ON er.curve_id = c.id
JOIN ec.volcano_vertex vv ON vv.curve_id = c.id
JOIN ec.isogeny_volcano iv ON iv.id = vv.volcano_id
WHERE c.family = 'census-fp' AND NOT er.over_closure AND er.kind = 'imaginary_quadratic_order'
  AND er.conductor_f IS NOT NULL AND er.discriminant IS NOT NULL AND abs(er.discriminant) < $1::int8
  AND mod(er.conductor_f, iv.ell) <> 0
GROUP BY 1, 2, 3, 4";

/// Every stored value of a row: a rerun must agree with all of them (ON CONFLICT DO NOTHING would otherwise skip a
/// conflicting row silently).
const EXISTING_SQL: &str = "
SELECT g.discriminant::int8, g.fundamental_discriminant::int8, g.conductor_f::int8, g.class_number::int8,
       g.class_group::int8[], g.class_number_fundamental::int8, g.unit_index::int8, g.two_rank,
       (SELECT coalesce(jsonb_agg(jsonb_build_array(e.ell::int8, e.splitting, e.prime_form::int8[],
                                                    e.class_order::int8)), '[]'::jsonb)
          FROM ec.order_ell_class e WHERE e.discriminant = g.discriminant)
FROM ec.order_class_group g WHERE abs(g.discriminant) < $1::int8";

const NUMBER_FIELD_SQL: &str = "
SELECT n.fundamental_discriminant::int8, n.class_number::int8, n.class_group::int8[]
FROM ec.number_field n
WHERE n.degree = 2 AND n.fundamental_discriminant < 0 AND abs(n.fundamental_discriminant) < $1::int8";

const GROUP_SQL: &str = "
INSERT INTO ec.order_class_group (discriminant, fundamental_discriminant, conductor_f, class_number, class_group,
       class_number_fundamental, unit_index, two_rank, cross_checks, status, method, provenance_id)
SELECT r.d, r.d_k, r.f, r.h, r.cg::ec.pos[], r.h_k, r.u, r.two_rank, r.checks, 'proved', r.method, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(d numeric, d_k numeric, f numeric, h numeric, cg numeric[], h_k numeric,
     u numeric, two_rank int, checks text[], method text)
ON CONFLICT (discriminant) DO NOTHING";

const ELL_SQL: &str = "
INSERT INTO ec.order_ell_class (discriminant, ell, splitting, prime_form, class_order, cross_checks, status, method,
       provenance_id)
SELECT r.d, r.ell, r.splitting, r.form::ec.zint[], r.ord, r.checks, 'proved', r.method, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(d numeric, ell numeric, splitting text, form numeric[], ord numeric,
     checks text[], method text)
ON CONFLICT (discriminant, ell) DO NOTHING";

/// Fill a NULL class group of the quadratic field Q(sqrt D_K); never touches a stored one, and only where the stored
/// class number is NULL or agrees.
const NUMBER_FIELD_UPDATE_SQL: &str = "
UPDATE ec.number_field n SET class_group = r.cg::ec.pos[], class_number = coalesce(n.class_number, r.h)
FROM jsonb_to_recordset($1::jsonb) AS r(d numeric, h numeric, cg numeric[])
WHERE n.degree = 2 AND n.fundamental_discriminant = r.d AND n.class_group IS NULL
  AND (n.class_number IS NULL OR n.class_number = r.h)";

// ------------------------------------------------------------- integers

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

/// (g, x, y) with a x + b y = g = gcd(a, b) >= 0.
fn xgcd(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut r0, mut r1, mut x0, mut x1, mut y0, mut y1) = (a, b, 1i64, 0i64, 0i64, 1i64);
    while r1 != 0 {
        let q = r0.div_euclid(r1);
        (r0, r1) = (r1, r0 - q * r1);
        (x0, x1) = (x1, x0 - q * x1);
        (y0, y1) = (y1, y0 - q * y1);
    }
    if r0 < 0 {
        (-r0, -x0, -y0)
    } else {
        (r0, x0, y0)
    }
}

fn isqrt(n: i64) -> i64 {
    if n < 0 {
        return -1;
    }
    let mut r = (n as f64).sqrt() as i64;
    while r * r > n {
        r -= 1;
    }
    while (r + 1) * (r + 1) <= n {
        r += 1;
    }
    r
}

fn factor(mut n: u64) -> Vec<(u64, u32)> {
    let mut out = Vec::new();
    let mut d = 2;
    while d * d <= n {
        let mut e = 0;
        while n.is_multiple_of(d) {
            n /= d;
            e += 1;
        }
        if e > 0 {
            out.push((d, e));
        }
        d += 1;
    }
    if n > 1 {
        out.push((n, 1));
    }
    out
}

fn is_fundamental(d: i64) -> bool {
    if d >= 0 {
        return false;
    }
    let squarefree = |m: u64| factor(m).iter().all(|&(_, e)| e == 1);
    match d.rem_euclid(4) {
        1 => squarefree(d.unsigned_abs()),
        0 => matches!((d / 4).rem_euclid(4), 2 | 3) && squarefree((d / 4).unsigned_abs()),
        _ => false,
    }
}

/// The Kronecker symbol (D/q) for a discriminant D and a prime q.
pub fn kronecker(d: i64, q: u64) -> i64 {
    if q == 2 {
        return match d.rem_euclid(8) {
            1 | 7 => 1,
            3 | 5 => -1,
            _ => 0,
        };
    }
    let r = d.rem_euclid(q as i64) as u64;
    if r == 0 {
        return 0;
    }
    let (mut base, mut e, mut acc) = (r, (q - 1) / 2, 1u64);
    while e > 0 {
        if e & 1 == 1 {
            acc = acc * base % q;
        }
        base = base * base % q;
        e >>= 1;
    }
    if acc == 1 {
        1
    } else {
        -1
    }
}

// ------------------------------------------------------------- forms

/// A positive definite binary quadratic form a x^2 + b x y + c y^2.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Form {
    pub a: i64,
    pub b: i64,
    pub c: i64,
}

impl Form {
    pub fn disc(&self) -> i64 {
        self.b * self.b - 4 * self.a * self.c
    }

    /// The principal form of discriminant d.
    pub fn principal(d: i64) -> Form {
        let b = d.rem_euclid(2);
        Form { a: 1, b, c: (b * b - d) / 4 }
    }

    pub fn inverse(&self) -> Form {
        reduce(Form { a: self.a, b: -self.b, c: self.c })
    }
}

/// The reduced form equivalent to a positive definite form (Cohen, Alg. 5.4.2).
pub fn reduce(f: Form) -> Form {
    let (mut a, mut b, mut c) = (f.a, f.b, f.c);
    loop {
        if b > a || b <= -a {
            // f(x + k y, y): b -> b + 2 a k, c -> a k^2 + b k + c, with -a < b + 2 a k <= a
            let k = (a - b).div_euclid(2 * a);
            c += k * (a * k + b);
            b += 2 * a * k;
        }
        if a > c {
            (a, b, c) = (c, -b, a);
            continue;
        }
        if a == c && b < 0 {
            b = -b;
        }
        return Form { a, b, c };
    }
}

/// Dirichlet composition of two primitive forms of the same discriminant, reduced (Cohen, Alg. 5.4.7).
pub fn compose(f1: Form, f2: Form) -> Form {
    let (f1, f2) = if f1.a > f2.a { (f2, f1) } else { (f1, f2) };
    let d = f1.disc();
    let s = (f1.b + f2.b) / 2;
    let n = f2.b - s;
    let (g, y1) = if f2.a % f1.a == 0 {
        (f1.a, 0)
    } else {
        let (g, u, _) = xgcd(f2.a, f1.a);
        (g, u)
    };
    let (d1, x2, y2) = if s % g == 0 {
        (g, 0, -1)
    } else {
        let (d1, x2, y2) = xgcd(s, g);
        (d1, x2, -y2)
    };
    let v1 = f1.a / d1;
    let v2 = f2.a / d1;
    let r = ((y1 as i128 * y2 as i128 * n as i128 - x2 as i128 * f2.c as i128).rem_euclid(v1 as i128)) as i64;
    let b3 = f2.b + 2 * v2 * r;
    let a3 = v1 * v2;
    let c3 = (b3 * b3 - d) / (4 * a3);
    reduce(Form { a: a3, b: b3, c: c3 })
}

/// The primitive reduced forms of discriminant d < 0, sorted.
pub fn reduced_forms(d: i64) -> Vec<Form> {
    let mut out = Vec::new();
    let mut a = 1i64;
    while 3 * a * a <= -d {
        for b in -a + 1..=a {
            let num = b * b - d;
            if num % (4 * a) != 0 {
                continue;
            }
            let c = num / (4 * a);
            if c < a || (b < 0 && a == c) {
                continue;
            }
            if gcd(gcd(a, b), c) == 1 {
                out.push(Form { a, b, c });
            }
        }
        a += 1;
    }
    out.sort();
    out
}

// ------------------------------------------------------------- the group

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClassGroup {
    pub d: i64,
    pub forms: Vec<Form>,
    /// Invariant factors n_1 | n_2 | ..., ascending; empty when h = 1.
    pub invariants: Vec<u64>,
}

impl ClassGroup {
    pub fn h(&self) -> u64 {
        self.forms.len() as u64
    }

    pub fn two_rank(&self) -> u32 {
        self.invariants.iter().filter(|n| *n % 2 == 0).count() as u32
    }

    pub fn exponent(&self) -> u64 {
        self.invariants.last().copied().unwrap_or(1)
    }

    fn index(&self, f: Form) -> Result<usize, String> {
        self.forms.binary_search(&f).map_err(|_| {
            format!("composition gave {f:?} (disc {}), not a reduced form of discriminant {}", f.disc(), self.d)
        })
    }

    fn mul(&self, x: Form, y: Form) -> Result<Form, String> {
        let z = compose(x, y);
        self.index(z)?;
        Ok(z)
    }

    /// Order of a form in Cl(D) by repeated composition, bounded by h.
    pub fn order_of(&self, x: Form) -> Result<u64, String> {
        let x = reduce(x);
        self.index(x)?;
        let id = Form::principal(self.d);
        let mut y = x;
        for k in 1..=self.h() {
            if y == id {
                return Ok(k);
            }
            y = self.mul(y, x)?;
        }
        Err(format!("{x:?} has no order <= h = {} in Cl({})", self.h(), self.d))
    }
}

/// Cl(D) for a negative discriminant D: its reduced forms and invariant factors, with the group checks of the module
/// documentation.
pub fn class_group(d: i64) -> Result<ClassGroup, String> {
    if d >= 0 || !matches!(d.rem_euclid(4), 0 | 1) || d.abs() >= MAX_ABS_D {
        return Err(format!("{d} is not a negative discriminant with |D| < {MAX_ABS_D}"));
    }
    let forms = reduced_forms(d);
    let mut g = ClassGroup { d, forms, invariants: Vec::new() };
    let h = g.h();
    let id = Form::principal(d);
    if g.index(id).is_err() {
        return Err(format!("the principal form {id:?} is not reduced"));
    }
    for &x in &g.forms {
        if x.disc() != d {
            return Err(format!("reduced form {x:?} has discriminant {}", x.disc()));
        }
        if g.mul(x, x.inverse())? != id {
            return Err(format!("{x:?} * {:?} is not principal", x.inverse()));
        }
    }
    // q-primary parts: exps[q] = the exponents e_i, descending
    let mut exps: Vec<(u64, Vec<u32>)> = Vec::new();
    for (q, vq) in factor(h) {
        // pow[i] = x_i^q
        let mut pow = Vec::with_capacity(g.forms.len());
        for &x in &g.forms {
            let mut y = x;
            for _ in 1..q {
                y = g.mul(y, x)?;
            }
            pow.push(g.index(y)?);
        }
        let id_i = g.index(id)?;
        let full = q.pow(vq);
        let mut cur: Vec<usize> = (0..g.forms.len()).collect();
        let mut s_prev = 0u32;
        let mut r = Vec::new(); // r[j-1] = #{i : e_i >= j}
        for j in 1..=vq {
            for c in cur.iter_mut() {
                *c = pow[*c];
            }
            let n_j = cur.iter().filter(|&&c| c == id_i).count() as u64;
            let s_j = (0..=vq)
                .find(|&s| q.pow(s) == n_j)
                .ok_or_else(|| format!("#{{x : x^({q}^{j}) = 1}} = {n_j} is not a power of {q} (Cl({d}), h = {h})"))?;
            if s_j < s_prev || (j > 1 && s_j - s_prev > r[j as usize - 2]) {
                return Err(format!("q-power counts of Cl({d}) are not those of an abelian group at q = {q}"));
            }
            r.push(s_j - s_prev);
            s_prev = s_j;
            if n_j == full {
                break;
            }
        }
        if q.pow(s_prev) != full {
            return Err(format!("the {q}-part of Cl({d}) has order {} != {full}", q.pow(s_prev)));
        }
        let mut e = Vec::new();
        for (j, &rj) in r.iter().enumerate() {
            let next = r.get(j + 1).copied().unwrap_or(0);
            for _ in 0..rj - next {
                e.push(j as u32 + 1);
            }
        }
        e.sort_unstable_by(|a, b| b.cmp(a));
        exps.push((q, e));
    }
    let rank = exps.iter().map(|(_, e)| e.len()).max().unwrap_or(0);
    let mut inv = vec![1u64; rank];
    for (q, e) in &exps {
        for (i, &ei) in e.iter().enumerate() {
            inv[rank - 1 - i] *= q.pow(ei);
        }
    }
    if inv.iter().product::<u64>() != h || inv.windows(2).any(|w| w[1] % w[0] != 0) {
        return Err(format!("invariant factors {inv:?} of Cl({d}) do not multiply to h = {h} in a chain"));
    }
    g.invariants = inv;
    Ok(g)
}

/// The class number formula for orders: h(f^2 D_K) from h(D_K).
pub fn class_number_formula(d_k: i64, f: u64, h_k: u64) -> Result<u64, String> {
    let unit_index = unit_index(d_k, f);
    let (mut num, mut den) = (h_k as u128 * f as u128, unit_index as u128);
    for (q, _) in factor(f) {
        num *= (q as i128 - kronecker(d_k, q) as i128) as u128;
        den *= q as u128;
    }
    if num % den != 0 {
        return Err(format!("class number formula gives a non-integer {num}/{den} for D_K = {d_k}, f = {f}"));
    }
    Ok((num / den) as u64)
}

/// [O_K^* : O^*] for the order of conductor f in Q(sqrt D_K).
pub fn unit_index(d_k: i64, f: u64) -> u64 {
    match (d_k, f > 1) {
        (-3, true) => 3,
        (-4, true) => 2,
        _ => 1,
    }
}

/// The 2-rank of Cl(D) by genus theory: mu - 1 (Cox, Prop. 3.11, Thm 3.15).
pub fn genus_two_rank(d: i64) -> u32 {
    let odd = |m: u64| factor(m).iter().filter(|(q, _)| *q != 2).count() as u32;
    let mu = if d.rem_euclid(4) == 1 {
        odd(d.unsigned_abs())
    } else {
        let n = (-d / 4) as u64;
        let r = odd(n);
        match (n % 4, n % 8) {
            (3, _) => r,
            (1, _) | (2, _) => r + 1,
            (_, 4) => r + 1,
            _ => r + 2,
        }
    };
    mu - 1
}

// ------------------------------------------------------------- the l-class

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EllClass {
    pub ell: i64,
    pub splitting: &'static str,
    pub form: Form,
    pub order: u64,
    /// The norm-equation check covered every k <= order (otherwise only k with l^k <= NORM_LIMIT).
    pub norm_full: bool,
}

/// Is there a primitive representation n = x^2 + b0 x y + c0 y^2 (principal form of discriminant d) with l not
/// dividing both x and y?
pub fn primitive_rep(d: i64, n: i64, l: i64) -> bool {
    let b0 = d.rem_euclid(2);
    let mut y = 0i64;
    loop {
        let rhs = 4 * n as i128 + d as i128 * (y as i128) * (y as i128);
        if rhs < 0 {
            return false;
        }
        let rhs = rhs as i64;
        let x_big = isqrt(rhs);
        if x_big * x_big == rhs {
            for sx in [x_big, -x_big] {
                if (sx - b0 * y) % 2 == 0 {
                    let x = (sx - b0 * y) / 2;
                    if x % l != 0 || y % l != 0 {
                        return true;
                    }
                }
            }
        }
        y += 1;
    }
}

/// The class of a prime above l in Cl(O), O of discriminant d = f^2 d_K; None when l | f or l is inert.
pub fn ell_class(g: &ClassGroup, f: u64, d_k: i64, l: i64) -> Result<Option<EllClass>, String> {
    let d = g.d;
    if f.is_multiple_of(l as u64) {
        return Ok(None);
    }
    let chi = kronecker(d_k, l as u64);
    if chi != kronecker(d, l as u64) {
        return Err(format!("(D/{l}) != (D_K/{l}) although {l} does not divide f"));
    }
    let splitting = match chi {
        1 => "split",
        0 => "ramified",
        _ => return Ok(None),
    };
    let b = (0..=l).find(|b| (b * b - d) % (4 * l) == 0).ok_or_else(|| format!("no b with b^2 = {d} mod {}", 4 * l))?;
    let form = Form { a: l, b, c: (b * b - d) / (4 * l) };
    if gcd(gcd(form.a, form.b), form.c) != 1 {
        return Err(format!("the form {form:?} above {l} is not primitive"));
    }
    let order = g.order_of(form)?;
    if !g.exponent().is_multiple_of(order) {
        return Err(format!("the order {order} of {form:?} does not divide the exponent {}", g.exponent()));
    }
    if splitting == "ramified" && order > 2 {
        return Err(format!("ramified prime above {l} has order {order} > 2"));
    }
    // independent: primitive representations of l^k by the principal form
    let mut norm_full = false;
    if splitting == "ramified" {
        if primitive_rep(d, l, l) != (order == 1) {
            return Err(format!("norm equation: {l} represented by the principal form iff order 1, but order {order}"));
        }
        norm_full = true;
    } else {
        let mut n = 1i64;
        for k in 1..=order {
            n = match n.checked_mul(l) {
                Some(x) if x <= NORM_LIMIT => x,
                _ => break,
            };
            if primitive_rep(d, n, l) != (k == order) {
                return Err(format!(
                    "norm equation: {l}^{k} {} primitively represented, but the order of [{l}] is {order}",
                    if k == order { "is not" } else { "is" }
                ));
            }
            norm_full = k == order;
        }
    }
    Ok(Some(EllClass { ell: l, splitting, form, order, norm_full }))
}

// ------------------------------------------------------------- one order

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrderData {
    pub d: i64,
    pub d_k: i64,
    pub f: u64,
    pub group: ClassGroup,
    pub h_k: u64,
    pub unit_index: u64,
    pub checks: Vec<&'static str>,
    pub ells: Vec<EllClass>,
}

/// Cl(O) and the l-classes of the order of discriminant d = f^2 d_K, with every check of the module documentation
/// except the volcano craters ([`check_craters`]). `stored_h_k` is a proved h(d_K) ecdb already holds.
pub fn order_data(d: i64, f: u64, d_k: i64, stored_h_k: Option<u64>) -> Result<OrderData, String> {
    if !is_fundamental(d_k) {
        return Err(format!("D_K = {d_k} is not a negative fundamental discriminant"));
    }
    if f == 0 || (f as i128) * (f as i128) * d_k as i128 != d as i128 {
        return Err(format!("D = {d} != f^2 D_K = {f}^2 * {d_k}"));
    }
    let group = class_group(d)?;
    let h_k = reduced_forms(d_k).len() as u64;
    let mut checks = vec!["inverse_closure"];
    if let Some(s) = stored_h_k {
        if s != h_k {
            return Err(format!("stored h({d_k}) = {s}, ecdb counts {h_k} reduced forms"));
        }
        checks.push("stored_h_dK");
    }
    let want = class_number_formula(d_k, f, h_k)?;
    if want != group.h() {
        return Err(format!("class number formula gives h({d}) = {want}, the group has {} elements", group.h()));
    }
    if f > 1 {
        checks.push("class_number_formula");
    }
    let genus = genus_two_rank(d);
    if genus != group.two_rank() {
        return Err(format!(
            "genus theory gives 2-rank {genus} for D = {d}, the invariant factors {:?} have {}",
            group.invariants,
            group.two_rank()
        ));
    }
    checks.push("genus_theory_2rank");
    let mut ells = Vec::new();
    for l in ELLS {
        if let Some(e) = ell_class(&group, f, d_k, l)? {
            ells.push(e);
        }
    }
    Ok(OrderData { d, d_k, f, group, h_k, unit_index: unit_index(d_k, f), checks, ells })
}

/// (l, crater size, level, curves) of the stored l-volcano vertices of census curves with one End(E).
pub type Crater = (i64, Option<i32>, i32, i64);

/// The stored l-volcano craters of curves with End(E) = O and l ∤ f: (l, crater size, level, curves). Every such
/// curve must be on the crater, and the crater size must be the order of the l-class (1 when l is inert). Returns the
/// primes l whose order the craters confirmed.
pub fn check_craters(od: &OrderData, craters: &[Crater]) -> Result<BTreeSet<i64>, String> {
    let mut agree = BTreeSet::new();
    for &(l, size, level, n) in craters {
        if od.f.is_multiple_of(l as u64) {
            continue;
        }
        if level != 0 {
            return Err(format!(
                "{n} curve(s) with End(E) of discriminant {} at level {level} of the {l}-volcano",
                od.d
            ));
        }
        let Some(size) = size else { continue };
        let want = od.ells.iter().find(|e| e.ell == l).map_or(1, |e| e.order);
        if size as u64 != want {
            return Err(format!(
                "the {l}-volcano crater of {n} curve(s) with D = {} has {size} vertices, the order of \
                                the {l}-class is {want}",
                od.d
            ));
        }
        if od.ells.iter().any(|e| e.ell == l) {
            agree.insert(l);
        }
    }
    Ok(agree)
}

// ------------------------------------------------------------- import

/// (f, D_K, stored proved h(D_K), curves) of one discriminant.
type DiscInput = (i64, i64, Option<i64>, i64);
/// The stored values of an `order_class_group` row and its `order_ell_class` rows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Existing {
    pub d_k: i64,
    pub f: i64,
    pub h: i64,
    pub class_group: Vec<i64>,
    pub h_k: i64,
    pub unit_index: i64,
    pub two_rank: i32,
    /// (l, splitting, prime form, class order)
    pub ells: Vec<(i64, String, Vec<i64>, i64)>,
}

/// Does a stored row agree exactly with what ecdb computes? Every stored column is compared; an l-class row ecdb
/// would not write (l | f or l inert) is a disagreement too.
pub fn compare_existing(od: &OrderData, ex: &Existing) -> Result<(), String> {
    let inv: Vec<i64> = od.group.invariants.iter().map(|&n| n as i64).collect();
    let want = (od.d_k, od.f as i64, od.group.h() as i64, &inv, od.h_k as i64, od.unit_index as i64);
    let have = (ex.d_k, ex.f, ex.h, &ex.class_group, ex.h_k, ex.unit_index);
    if want != have || ex.two_rank as u32 != od.group.two_rank() {
        return Err(format!(
            "stored (d_K, f, h, Cl, h(d_K), unit index, 2-rank) = ({}, {}, {}, {:?}, {}, {}, {}) disagrees with ecdb's \
             ({}, {}, {}, {inv:?}, {}, {}, {})",
            ex.d_k,
            ex.f,
            ex.h,
            ex.class_group,
            ex.h_k,
            ex.unit_index,
            ex.two_rank,
            od.d_k,
            od.f,
            od.group.h(),
            od.h_k,
            od.unit_index,
            od.group.two_rank()
        ));
    }
    for (l, splitting, form, order) in &ex.ells {
        let ok = od.ells.iter().any(|e| {
            e.ell == *l
                && e.splitting == splitting
                && *form == [e.form.a, e.form.b, e.form.c]
                && e.order as i64 == *order
        });
        if !ok {
            return Err(format!(
                "stored {l}-class row ({splitting}, form {form:?}, order {order}) disagrees with ecdb's {:?}",
                od.ells.iter().find(|e| e.ell == *l)
            ));
        }
    }
    Ok(())
}

pub async fn import(ctx: &Ctx<'_>) -> Result<Report> {
    let mut report = Report {
        input: "End(E) of the ordinary census-fp curves (stored ec.endomorphism_ring)".into(),
        ..Default::default()
    };
    let client = ctx.client;
    let src = ctx
        .source(
            "ecdb/src/import/class_group.rs",
            &crate::math::sha256_hex(include_str!("class_group.rs")),
            "Class groups of End(E) and orders of the l-classes, computed by ecdb",
        )
        .await?;
    let prov = ctx.provenance(src, METHOD, None).await?;

    // discriminant -> [(f, d_K, stored h(d_K), curves)]
    let mut by_d: BTreeMap<i64, Vec<DiscInput>> = BTreeMap::new();
    for r in client.query(DISC_SQL, &[&MAX_ABS_D]).await? {
        by_d.entry(r.get(0)).or_default().push((r.get(1), r.get(2), r.get(3), r.get(4)));
    }
    let mut craters: BTreeMap<i64, Vec<Crater>> = BTreeMap::new();
    for r in client.query(CRATER_SQL, &[&MAX_ABS_D]).await? {
        craters.entry(r.get(0)).or_default().push((r.get(1), r.get(2), r.get(3), r.get(4)));
    }
    let mut existing: BTreeMap<i64, Existing> = BTreeMap::new();
    for r in client.query(EXISTING_SQL, &[&MAX_ABS_D]).await? {
        let rows: Value = r.get(8);
        let mut ells = Vec::new();
        for x in rows.as_array().into_iter().flatten() {
            let form: Option<Vec<i64>> = x[2].as_array().map(|v| v.iter().filter_map(Value::as_i64).collect());
            match (x[0].as_i64(), x[1].as_str(), form, x[3].as_i64()) {
                (Some(l), Some(sp), Some(form), Some(o)) => ells.push((l, sp.to_string(), form, o)),
                // an unreadable stored row can never agree: keep a sentinel that compare_existing refuses
                _ => ells.push((-1, x.to_string(), Vec::new(), -1)),
            }
        }
        existing.insert(
            r.get(0),
            Existing {
                d_k: r.get(1),
                f: r.get(2),
                h: r.get(3),
                class_group: r.get(4),
                h_k: r.get(5),
                unit_index: r.get(6),
                two_rank: r.get(7),
                ells,
            },
        );
    }
    let mut fields: BTreeMap<i64, (Option<i64>, Option<Vec<i64>>)> = BTreeMap::new();
    for r in client.query(NUMBER_FIELD_SQL, &[&MAX_ABS_D]).await? {
        fields.insert(r.get(0), (r.get(1), r.get(2)));
    }

    let (mut groups, mut ells, mut nf) = (Vec::new(), Vec::new(), Vec::new());
    for (&d, inputs) in &by_d {
        report.count("class_group_discriminants", 1);
        let curves: i64 = inputs.iter().map(|x| x.3).sum();
        let distinct: BTreeSet<(i64, i64)> = inputs.iter().map(|x| (x.0, x.1)).collect();
        let stored: BTreeSet<i64> = inputs.iter().filter_map(|x| x.2).collect();
        let refuse = |report: &mut Report, why: String| {
            report.count("class_groups_refused", 1);
            report.problem(format!("D = {d} ({curves} curves): {why}"));
        };
        if distinct.len() != 1 || stored.len() > 1 {
            refuse(&mut report, format!("inconsistent (f, D_K) {distinct:?} or stored h(D_K) {stored:?}"));
            continue;
        }
        let (f, d_k) = *distinct.iter().next().unwrap();
        let od = match order_data(d, f as u64, d_k, stored.iter().next().map(|&h| h as u64)) {
            Ok(od) => od,
            Err(why) => {
                refuse(&mut report, why);
                continue;
            }
        };
        let agree = match check_craters(&od, craters.get(&d).map_or(&[][..], |v| &v[..])) {
            Ok(a) => a,
            Err(why) => {
                refuse(&mut report, why);
                continue;
            }
        };
        let inv: Vec<i64> = od.group.invariants.iter().map(|&n| n as i64).collect();
        if let Some(ex) = existing.get(&d) {
            if let Err(why) = compare_existing(&od, ex) {
                refuse(&mut report, why);
                continue;
            }
            report.count("existing_rows_agree", 1);
        }
        if od.f == 1 {
            if let Some((h, cg)) = fields.get(&d) {
                let bad_h = h.is_some_and(|h| h as u64 != od.group.h());
                let bad_cg = cg.as_ref().is_some_and(|cg| *cg != inv);
                if bad_h || bad_cg {
                    refuse(
                        &mut report,
                        format!(
                            "number_field Q(sqrt {d}) stores h = {h:?}, Cl = {cg:?}, ecdb \
                                                 computes h = {}, Cl = {inv:?}",
                            od.group.h()
                        ),
                    );
                    continue;
                }
                if cg.is_none() {
                    nf.push(json!({ "d": d, "h": od.group.h(), "cg": inv }));
                }
            }
        }
        report.count("class_group_curves", curves as u64);
        if od.checks.contains(&"stored_h_dK") {
            report.count("stored_h_dk_agree", 1);
        }
        report.count("ell_orders_volcano_agree", agree.len() as u64);
        for &(l, size, _, n) in craters.get(&d).into_iter().flatten() {
            if size.is_some() && agree.contains(&l) {
                report.count("volcano_curves_agree", n as u64);
            }
        }
        let checks: Vec<&str> = od.checks.clone();
        groups.push(json!({ "d": d, "d_k": d_k, "f": f, "h": od.group.h(), "cg": inv, "h_k": od.h_k,
            "u": od.unit_index, "two_rank": od.group.two_rank(), "checks": checks, "method": METHOD }));
        for e in &od.ells {
            let mut c = vec![if e.norm_full { "norm_equation" } else { "norm_equation_partial" }];
            if e.norm_full {
                report.count("ell_orders_norm_full", 1);
            } else {
                report.count("ell_orders_norm_partial", 1);
            }
            if agree.contains(&e.ell) {
                c.push("volcano_crater");
            }
            ells.push(json!({ "d": d, "ell": e.ell, "splitting": e.splitting,
                "form": [e.form.a, e.form.b, e.form.c], "ord": e.order, "checks": c, "method": METHOD }));
        }
    }
    report.count("class_groups_inserted", bulk(client, GROUP_SQL, &groups, prov).await?);
    report.count("ell_classes_inserted", bulk(client, ELL_SQL, &ells, prov).await?);
    report.count("number_field_class_groups_filled", bulk1(client, NUMBER_FIELD_UPDATE_SQL, &nf).await?);
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::super::end_ring::{class_number, end_ring, iso_key};
    use super::*;

    fn cg(d: i64) -> Vec<u64> {
        class_group(d).unwrap().invariants
    }

    #[test]
    fn class_groups_match_the_literature() {
        // Cox, §2 and §3 (Table of forms); Buell, Binary quadratic forms, Table 2: -3299 and -4027 are the first
        // discriminants with 3-rank 2.
        assert_eq!(cg(-3), Vec::<u64>::new());
        assert_eq!(cg(-4), Vec::<u64>::new());
        assert_eq!(cg(-23), vec![3]);
        assert_eq!(cg(-56), vec![4]);
        assert_eq!(cg(-84), vec![2, 2]);
        assert_eq!(cg(-420), vec![2, 2, 2]);
        assert_eq!(cg(-3299), vec![3, 9]);
        assert_eq!(cg(-4027), vec![3, 3]);
        // non-maximal orders: h(-36) = 2 (f = 3, D_K = -4), h(-108) = 3 (f = 6, D_K = -3)
        assert_eq!(cg(-36), vec![2]);
        assert_eq!(cg(-108), vec![3]);
    }

    #[test]
    fn class_numbers_agree_with_end_ring_for_every_census_discriminant() {
        // the class numbers end_ring.rs already uses (and pins) for the Deuring counts
        for d in (-4100i64..0).filter(|d| matches!(d.rem_euclid(4), 0 | 1)) {
            let g = class_group(d).unwrap_or_else(|e| panic!("{d}: {e}"));
            assert_eq!(g.h(), class_number(d), "{d}");
            assert_eq!(g.two_rank(), genus_two_rank(d), "{d}");
        }
    }

    #[test]
    fn class_number_formula_holds_for_every_census_order() {
        for d_k in (-4100i64..0).filter(|&d| is_fundamental(d)) {
            for f in 1u64.. {
                let d = (f * f) as i64 * d_k;
                if d <= -4100 {
                    break;
                }
                let od = order_data(d, f, d_k, None).unwrap_or_else(|e| panic!("{d}: {e}"));
                assert_eq!(od.group.h(), class_number(d));
            }
        }
    }

    #[test]
    fn composition_is_a_group_law_by_brute_force() {
        for d in [-23, -56, -84, -95, -260, -399, -420, -1155, -3299, -36, -108, -16 * 7] {
            let g = class_group(d).unwrap();
            let id = Form::principal(d);
            for &x in &g.forms {
                assert_eq!(compose(x, id), x);
                for &y in &g.forms {
                    let xy = compose(x, y);
                    assert_eq!(xy, compose(y, x), "{d}: {x:?} {y:?}");
                    for &z in g.forms.iter().take(8) {
                        assert_eq!(compose(xy, z), compose(x, compose(y, z)));
                    }
                }
            }
            // the element-order statistics equal those of the claimed abelian group
            let mut have: BTreeMap<u64, u64> = BTreeMap::new();
            for &x in &g.forms {
                *have.entry(g.order_of(x).unwrap()).or_default() += 1;
            }
            let mut want: BTreeMap<u64, u64> = BTreeMap::new();
            let mut elems = vec![vec![]];
            for &n in &g.invariants {
                elems = elems
                    .into_iter()
                    .flat_map(|e: Vec<u64>| (0..n).map(move |i| [e.clone(), vec![i]].concat()))
                    .collect();
            }
            for e in elems {
                let o = e
                    .iter()
                    .zip(&g.invariants)
                    .map(|(&i, &n)| n / gcd(i as i64, n as i64) as u64)
                    .fold(1, |a, b| a / gcd(a as i64, b as i64) as u64 * b);
                *want.entry(o).or_default() += 1;
            }
            assert_eq!(have, want, "{d}");
        }
    }

    #[test]
    fn ell_class_orders_match_known_values() {
        // Cl(-23) = Z/3: (2, 1, 3) and (3, 1, 2) ~ (2, -1, 3) have order 3; (-23/5) = (-23/7) = -1, inert
        let od = order_data(-23, 1, -23, Some(3)).unwrap();
        let o: BTreeMap<i64, u64> = od.ells.iter().map(|e| (e.ell, e.order)).collect();
        assert_eq!(o, BTreeMap::from([(2, 3), (3, 3)]));
        // Cl(-56) = Z/4 = <(3, 2, 5)>; (2, 0, 7) and (7, 0, 2) ~ (2, 0, 7) are the element of order 2
        let od = order_data(-56, 1, -56, None).unwrap();
        let o: BTreeMap<i64, u64> = od.ells.iter().map(|e| (e.ell, e.order)).collect();
        assert_eq!(o, BTreeMap::from([(2, 2), (3, 4), (5, 4), (7, 2)]));
        assert!(od.ells.iter().all(|e| e.norm_full));
        // l | f is not applicable: D = -36 = 3^2 * -4 has no 3-class row
        let od = order_data(-36, 3, -4, None).unwrap();
        assert!(od.ells.iter().all(|e| e.ell != 3));
    }

    #[test]
    fn corrupted_inputs_are_refused() {
        assert!(order_data(-56, 2, -56, None).is_err()); // D != f^2 D_K
        assert!(order_data(-144, 6, -3, None).is_err()); // D_K inconsistent with D
        assert!(order_data(-23, 1, -23, Some(5)).is_err()); // wrong stored h(D_K)
        assert!(order_data(-48, 2, -12, None).is_err()); // D_K not fundamental
        let od = order_data(-56, 1, -56, None).unwrap();
        // a crater of the wrong size, and a curve with l ∤ f off the crater
        assert!(check_craters(&od, &[(3, Some(2), 0, 4)]).is_err());
        assert!(check_craters(&od, &[(3, Some(4), 1, 4)]).is_err());
        assert_eq!(check_craters(&od, &[(3, Some(4), 0, 4), (5, Some(4), 0, 4)]).unwrap(), BTreeSet::from([3, 5]));
    }

    fn stored(od: &OrderData) -> Existing {
        Existing {
            d_k: od.d_k,
            f: od.f as i64,
            h: od.group.h() as i64,
            class_group: od.group.invariants.iter().map(|&n| n as i64).collect(),
            h_k: od.h_k as i64,
            unit_index: od.unit_index as i64,
            two_rank: od.group.two_rank() as i32,
            ells: od
                .ells
                .iter()
                .map(|e| (e.ell, e.splitting.to_string(), vec![e.form.a, e.form.b, e.form.c], e.order as i64))
                .collect(),
        }
    }

    #[test]
    fn existing_rows_must_agree_in_every_column() {
        // D = -144: f = 6, D_K = -4, h = 4 (Cl = Z/4), unit index 2
        let od = order_data(-144, 6, -4, None).unwrap();
        assert_eq!((od.group.h(), od.unit_index), (4, 2));
        let ok = stored(&od);
        assert!(compare_existing(&od, &ok).is_ok());
        // a row that agrees on h and Cl but conflicts in any other column is refused, never silently kept
        let mut bad = ok.clone();
        bad.h_k = 2;
        assert!(compare_existing(&od, &bad).is_err());
        let mut bad = ok.clone();
        bad.unit_index = 1;
        assert!(compare_existing(&od, &bad).is_err());
        let mut bad = ok.clone();
        bad.two_rank = 0;
        assert!(compare_existing(&od, &bad).is_err());
        let mut bad = ok.clone();
        bad.class_group = vec![2, 2];
        assert!(compare_existing(&od, &bad).is_err());
        let od = order_data(-56, 1, -56, None).unwrap();
        let ok = stored(&od);
        assert!(compare_existing(&od, &ok).is_ok());
        let mut bad = ok.clone();
        bad.ells[0].2[1] += 4; // a different prime form with the right order
        assert!(compare_existing(&od, &bad).is_err());
        let mut bad = ok.clone();
        bad.ells[0].1 = "split".into(); // l = 2 ramifies in Z[sqrt -14]
        assert!(compare_existing(&od, &bad).is_err());
        let mut bad = ok.clone();
        bad.ells[1].3 = 2;
        assert!(compare_existing(&od, &bad).is_err());
        // an l-class row for an inert l (here l = 5 for D = -23) is refused
        let od = order_data(-23, 1, -23, None).unwrap();
        let mut bad = stored(&od);
        bad.ells.push((5, "split".into(), vec![5, 1, 2], 3));
        assert!(compare_existing(&od, &bad).is_err());
    }

    // ---------------------------------------------- brute force on tiny fields

    fn powmod(mut b: u64, mut e: u64, p: u64) -> u64 {
        let mut acc = 1;
        b %= p;
        while e > 0 {
            if e & 1 == 1 {
                acc = acc * b % p;
            }
            b = b * b % p;
            e >>= 1;
        }
        acc
    }

    fn trace(p: u64, a: u64, b: u64) -> i64 {
        -(0..p)
            .map(|x| {
                let r = (x * x % p * x + a * x + b) % p;
                if r == 0 {
                    0
                } else if powmod(r, (p - 1) / 2, p) == 1 {
                    1
                } else {
                    -1
                }
            })
            .sum::<i64>()
    }

    fn split_disc(t: i64, p: u64) -> (u64, i64) {
        let delta = t * t - 4 * p as i64;
        let mut v = 1u64;
        for (l, e) in factor(delta.unsigned_abs()) {
            v *= l.pow(e / 2);
        }
        let mut dk = delta / (v * v) as i64;
        if dk.rem_euclid(4) != 1 {
            dk *= 4;
            v /= 2;
        }
        (v, dk)
    }

    /// Vélu codomains of the rational l-isogenies, l = 2, 3, from y^2 = x^3 + A x + B.
    fn velu(p: u64, a: u64, b: u64, l: u64) -> Vec<(u64, u64)> {
        let mut out = Vec::new();
        for x0 in 0..p {
            let y2 = (x0 * x0 % p * x0 + a * x0 + b) % p;
            let gx = (3 * x0 * x0 + a) % p;
            let (v, w) = match l {
                2 if y2 == 0 => (gx, x0 * gx % p),
                3 if (3 * powmod(x0, 4, p) + 6 * a % p * x0 % p * x0 + 12 * b % p * x0 + p * p - a * a % p)
                    .is_multiple_of(p) =>
                {
                    let v = 2 * gx % p;
                    (v, (4 * y2 + x0 * v) % p)
                }
                _ => continue,
            };
            out.push(((a + 5 * p - 5 * v % p) % p, (b + 7 * p - 7 * w % p) % p));
        }
        out
    }

    #[test]
    fn crater_orbits_match_ell_class_orders_on_tiny_fields() {
        let mut checked = 0;
        for p in (5u64..110).filter(|&p| factor(p).len() == 1 && factor(p)[0].1 == 1) {
            // one model per F_p-isomorphism class, with its conductor
            let mut reps: BTreeMap<(u64, u64), (i64, u64, i64)> = BTreeMap::new();
            for a in 0..p {
                for b in 0..p {
                    if (4 * powmod(a, 3, p) + 27 * b * b).is_multiple_of(p) {
                        continue;
                    }
                    let key = iso_key(p, a, b);
                    if reps.contains_key(&key) {
                        continue;
                    }
                    let t = trace(p, a, b);
                    if t.rem_euclid(p as i64) == 0 {
                        continue;
                    }
                    let (v, d_k) = split_disc(t, p);
                    let e = end_ring(p, key.0, key.1, t, v, d_k).unwrap();
                    reps.insert(key, (t, e.f, d_k));
                }
            }
            for l in [2u64, 3] {
                if l == p {
                    continue;
                }
                let nbrs = |k: (u64, u64)| -> Vec<(u64, u64)> {
                    velu(p, k.0, k.1, l).into_iter().map(|(a2, b2)| iso_key(p, a2, b2)).collect()
                };
                for (&key, &(t, f, d_k)) in &reps {
                    let d = (f * f) as i64 * d_k;
                    let od = order_data(d, f, d_k, None).unwrap();
                    // the horizontal component through key: neighbours with the same End(E)
                    let mut seen = BTreeSet::from([key]);
                    let mut stack = vec![key];
                    while let Some(k) = stack.pop() {
                        for n in nbrs(k) {
                            let (tn, fnb, _) = reps[&n];
                            assert_eq!(tn, t);
                            if fnb == f && seen.insert(n) {
                                stack.push(n);
                            }
                        }
                    }
                    let want = match od.ells.iter().find(|e| e.ell == l as i64) {
                        Some(e) => e.order as usize,
                        None if f.is_multiple_of(l) => continue,
                        None => 1, // inert: no horizontal l-isogeny
                    };
                    assert_eq!(seen.len(), want, "p = {p}, l = {l}, curve {key:?}, D = {d}");
                    checked += 1;
                }
            }
        }
        assert!(checked > 1000, "{checked}");
    }
}
