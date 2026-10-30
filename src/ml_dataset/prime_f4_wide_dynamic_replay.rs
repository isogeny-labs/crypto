//! Independent, bounded dynamic-profile artifact replay. Dormant until the parent
//! verifier binds Expected to original run/public/domain objects and passes only
//! manifest-verified artifacts. No producer polynomial/group/extraction helpers.
//! Custody, supplied-basis consistency and mathematics do NOT prove native derivation.
#![allow(dead_code)] // Dormant child: central registration/admission is a separate parent change.
use sha2::{Digest, Sha256};
use std::cmp::Ordering;
use std::time::Instant;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Error {
    Shape,
    Identity,
    Arithmetic,
    Claim,
    Work,
    Storage,
    Deadline,
    Allocation,
}
type R<T> = Result<T, Error>;
#[derive(Clone, Copy)]
pub(super) struct ReplayLimits {
    pub work: u64,
    pub allocation_bytes: u64,
    pub deadline: Instant,
}
struct Budget {
    limits: ReplayLimits,
    work: u64,
    bytes: u64,
}
impl Budget {
    fn spend(&mut self, n: u64) -> R<()> {
        if Instant::now() >= self.limits.deadline {
            return Err(Error::Deadline);
        }
        self.work = self.work.checked_add(n).ok_or(Error::Work)?;
        if self.work > self.limits.work {
            return Err(Error::Work);
        }
        Ok(())
    }
    fn reserve<T>(&mut self, v: &mut Vec<T>, n: usize) -> R<()> {
        if n <= v.capacity() {
            return Ok(());
        }
        self.spend(n as u64)?;
        let bytes = n
            .checked_mul(std::mem::size_of::<T>())
            .and_then(|n| n.checked_add(128))
            .ok_or(Error::Storage)? as u64;
        self.bytes = self.bytes.checked_add(bytes).ok_or(Error::Storage)?;
        if self.bytes > self.limits.allocation_bytes {
            return Err(Error::Storage);
        }
        v.try_reserve_exact(n - v.len())
            .map_err(|_| Error::Allocation)
    }
    fn push<T>(&mut self, v: &mut Vec<T>, x: T, cap: usize) -> R<()> {
        if v.len() >= cap {
            return Err(Error::Shape);
        }
        if v.len() == v.capacity() {
            self.reserve(v, (v.len() + 16).min(cap))?
        }
        v.push(x);
        Ok(())
    }
}
// A bounded borrowed JSON tree. No unbounded serde allocation, escaped identifiers,
// floats, duplicate keys, signed integers or noncanonical numeric representations.
#[derive(Debug)]
enum J<'a> {
    Null,
    Bool(bool),
    Num(u64),
    Str(&'a str),
    Arr(Vec<J<'a>>),
    Obj(Vec<(&'a str, J<'a>)>),
}
struct Parser<'a, 'b> {
    bytes: &'a [u8],
    at: usize,
    b: &'b mut Budget,
}
impl<'a> Parser<'a, '_> {
    fn ws(&mut self) -> R<()> {
        while self.at < self.bytes.len() && self.bytes[self.at].is_ascii_whitespace() {
            if !matches!(self.bytes[self.at], b' ' | b'\n' | b'\r' | b'\t') {
                return Err(Error::Shape);
            }
            self.b.spend(1)?;
            self.at += 1
        }
        Ok(())
    }
    fn token(&mut self, x: &[u8]) -> R<()> {
        self.b.spend(x.len() as u64)?;
        if self.bytes.get(self.at..self.at + x.len()) != Some(x) {
            return Err(Error::Shape);
        }
        self.at += x.len();
        Ok(())
    }
    fn string(&mut self) -> R<&'a str> {
        self.token(b"\"")?;
        let start = self.at;
        while let Some(&c) = self.bytes.get(self.at) {
            self.b.spend(1)?;
            self.at += 1;
            if c == b'"' {
                return std::str::from_utf8(&self.bytes[start..self.at - 1])
                    .map_err(|_| Error::Shape);
            }
            if !(32..=126).contains(&c) || c == b'\\' {
                return Err(Error::Shape);
            }
        }
        Err(Error::Shape)
    }
    fn value(&mut self, depth: u8) -> R<J<'a>> {
        if depth > 16 {
            return Err(Error::Shape);
        }
        self.ws()?;
        match self.bytes.get(self.at).copied().ok_or(Error::Shape)? {
            b'n' => {
                self.token(b"null")?;
                Ok(J::Null)
            }
            b't' => {
                self.token(b"true")?;
                Ok(J::Bool(true))
            }
            b'f' => {
                self.token(b"false")?;
                Ok(J::Bool(false))
            }
            b'"' => Ok(J::Str(self.string()?)),
            b'0'..=b'9' => {
                let start = self.at;
                let mut x = 0u64;
                while let Some(c @ b'0'..=b'9') = self.bytes.get(self.at) {
                    self.b.spend(1)?;
                    x = x
                        .checked_mul(10)
                        .and_then(|x| x.checked_add((c - b'0') as u64))
                        .ok_or(Error::Shape)?;
                    self.at += 1
                }
                if self.at - start > 1 && self.bytes[start] == b'0' {
                    return Err(Error::Shape);
                }
                Ok(J::Num(x))
            }
            b'[' => {
                self.token(b"[")?;
                self.ws()?;
                let mut a = Vec::new();
                if self.bytes.get(self.at) == Some(&b']') {
                    self.token(b"]")?;
                    return Ok(J::Arr(a));
                }
                loop {
                    let x = self.value(depth + 1)?;
                    self.b.push(&mut a, x, 8192)?;
                    self.ws()?;
                    if self.bytes.get(self.at) == Some(&b']') {
                        self.token(b"]")?;
                        break;
                    }
                    self.token(b",")?
                }
                Ok(J::Arr(a))
            }
            b'{' => {
                self.token(b"{")?;
                self.ws()?;
                let mut a: Vec<(&str, J<'a>)> = Vec::new();
                if self.bytes.get(self.at) == Some(&b'}') {
                    self.token(b"}")?;
                    return Ok(J::Obj(a));
                }
                loop {
                    self.ws()?;
                    let key = self.string()?;
                    self.b.spend(a.len() as u64 + 1)?;
                    if a.iter().any(|(k, _)| *k == key) {
                        return Err(Error::Shape);
                    }
                    self.ws()?;
                    self.token(b":")?;
                    let x = self.value(depth + 1)?;
                    self.b.push(&mut a, (key, x), 64)?;
                    self.ws()?;
                    if self.bytes.get(self.at) == Some(&b'}') {
                        self.token(b"}")?;
                        break;
                    }
                    self.token(b",")?
                }
                Ok(J::Obj(a))
            }
            _ => Err(Error::Shape),
        }
    }
}
fn parse<'a>(s: &'a [u8], b: &mut Budget) -> R<J<'a>> {
    let mut p = Parser { bytes: s, at: 0, b };
    let x = p.value(0)?;
    p.ws()?;
    if p.at != s.len() {
        return Err(Error::Shape);
    }
    Ok(x)
}
impl<'a> J<'a> {
    fn obj(&self, keys: &[&str]) -> R<()> {
        let Self::Obj(o) = self else {
            return Err(Error::Shape);
        };
        if o.len() != keys.len() || o.iter().any(|(k, _)| !keys.contains(k)) {
            return Err(Error::Shape);
        }
        Ok(())
    }
    fn get(&self, k: &str) -> R<&J<'a>> {
        let Self::Obj(o) = self else {
            return Err(Error::Shape);
        };
        o.iter()
            .find(|(key, _)| *key == k)
            .map(|(_, v)| v)
            .ok_or(Error::Shape)
    }
    fn arr(&self, max: usize) -> R<&[J<'a>]> {
        match self {
            Self::Arr(x) if x.len() <= max => Ok(x),
            _ => Err(Error::Shape),
        }
    }
    fn n(&self) -> R<u64> {
        match self {
            Self::Num(x) => Ok(*x),
            _ => Err(Error::Shape),
        }
    }
    fn s(&self) -> R<&str> {
        match self {
            Self::Str(x) => Ok(x),
            _ => Err(Error::Shape),
        }
    }
    fn yes(&self) -> R<bool> {
        match self {
            Self::Bool(x) => Ok(*x),
            _ => Err(Error::Shape),
        }
    }
    fn dec(&self) -> R<u128> {
        decimal(self.s()?)
    }
    fn d64(&self) -> R<u64> {
        u64::try_from(self.dec()?).map_err(|_| Error::Shape)
    }
    fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }
    fn eqs(&self, x: &str) -> R<()> {
        if self.s()? == x {
            Ok(())
        } else {
            Err(Error::Identity)
        }
    }
    fn eqn(&self, x: u64) -> R<()> {
        if self.n()? == x {
            Ok(())
        } else {
            Err(Error::Identity)
        }
    }
    fn no(&self) -> R<()> {
        if !self.yes()? {
            Ok(())
        } else {
            Err(Error::Claim)
        }
    }
}
fn decimal(s: &str) -> R<u128> {
    if s.is_empty() || s.len() > 39 || (s.len() > 1 && s.starts_with('0')) {
        return Err(Error::Shape);
    }
    s.bytes().try_fold(0u128, |n, c| {
        if !c.is_ascii_digit() {
            return Err(Error::Shape);
        }
        n.checked_mul(10)
            .and_then(|n| n.checked_add((c - b'0') as u128))
            .ok_or(Error::Shape)
    })
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Point {
    pub x: u64,
    pub y: u64,
}
type P = Option<Point>;
fn point(j: &J<'_>, legacy: bool) -> R<P> {
    if legacy {
        let k = j.get("kind")?.s()?;
        if k == "infinity" {
            j.obj(&["kind"])?;
            return Ok(None);
        }
        j.obj(&["kind", "x", "y"])?;
        if k != "affine" {
            return Err(Error::Shape);
        }
        return Ok(Some(Point {
            x: j.get("x")?.n()?,
            y: j.get("y")?.n()?,
        }));
    }
    if j.is_null() {
        return Ok(None);
    }
    j.obj(&["x", "y"])?;
    Ok(Some(Point {
        x: j.get("x")?.d64()?,
        y: j.get("y")?.d64()?,
    }))
}
fn mul(x: u64, y: u64, p: u64) -> u64 {
    ((x as u128 * y as u128) % p as u128) as u64
}
fn add(x: u64, y: u64, p: u64) -> u64 {
    ((x as u128 + y as u128) % p as u128) as u64
}
fn sub(x: u64, y: u64, p: u64) -> u64 {
    ((x as u128 + p as u128 - y as u128) % p as u128) as u64
}
fn pow(mut x: u64, mut e: u64, p: u64, b: &mut Budget) -> R<u64> {
    let mut r = 1;
    while e != 0 {
        b.spend(64)?;
        if e & 1 != 0 {
            r = mul(r, x, p)
        }
        e >>= 1;
        if e != 0 {
            x = mul(x, x, p)
        }
    }
    Ok(r)
}
fn prime(p: u64, b: &mut Budget) -> R<bool> {
    if !(5..=(1u64 << 61) - 1).contains(&p) || p.is_multiple_of(2) {
        return Ok(false);
    }
    let s = (p - 1).trailing_zeros();
    let d = (p - 1) >> s;
    for a in [2, 325, 9375, 28178, 450775, 9780504, 1795265022] {
        let a = a % p;
        if a == 0 {
            continue;
        }
        let mut x = pow(a, d, p, b)?;
        if x == 1 || x == p - 1 {
            continue;
        }
        let mut ok = false;
        for _ in 1..s {
            b.spend(64)?;
            x = mul(x, x, p);
            if x == p - 1 {
                ok = true;
                break;
            }
        }
        if !ok {
            return Ok(false);
        }
    }
    Ok(true)
}
fn on_curve(q: Point, e: &Expected<'_>) -> bool {
    q.x < e.p
        && q.y < e.p
        && mul(q.y, q.y, e.p)
            == add(
                add(mul(mul(q.x, q.x, e.p), q.x, e.p), mul(e.a, q.x, e.p), e.p),
                e.b,
                e.p,
            )
}
fn sum(x: P, y: P, e: &Expected<'_>, b: &mut Budget) -> R<P> {
    b.spend(256)?;
    let (Some(x), Some(y)) = (x, y) else {
        return Ok(x.or(y));
    };
    let p = e.p;
    let (num, den) = if x.x == y.x {
        if add(x.y, y.y, p) == 0 {
            return Ok(None);
        }
        if x != y {
            return Err(Error::Arithmetic);
        }
        (add(mul(3, mul(x.x, x.x, p), p), e.a, p), add(x.y, x.y, p))
    } else {
        (sub(y.y, x.y, p), sub(y.x, x.x, p))
    };
    let inv = pow(den, p - 2, p, b)?;
    if mul(den, inv, p) != 1 {
        return Err(Error::Arithmetic);
    }
    let slope = mul(num, inv, p);
    let u = sub(sub(mul(slope, slope, p), x.x, p), y.x, p);
    Ok(Some(Point {
        x: u,
        y: sub(mul(slope, sub(x.x, u, p), p), x.y, p),
    }))
}
/// Parent constructs this from verified ORIGINAL public/domain/config/run inputs,
/// not from the solver artifact being checked. It must preserve domain order.
pub(super) struct Expected<'a> {
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub m: usize,
    pub target: Point,
    pub domain: &'a [Point],
    pub codes: &'a [u64],
    pub work_limit: u64,
    pub trace_bytes: u64,
    pub memory_bytes: u64,
}
/// Metadata must already be bound to the original attempt's authoritative manifest.
/// This child nevertheless checks registry, bytes and digest itself before parsing.
pub(super) struct Artifact<'a> {
    pub path: &'a str,
    pub role: &'a str,
    pub encoding: &'a str,
    pub bytes: &'a [u8],
    pub sha256: [u8; 32],
}
pub(super) struct CapturedWitness<'a> {
    pub bytes: &'a [u8],
    pub sha256: [u8; 32],
    pub original_sequence: u64,
}
pub(super) struct Bundle<'a> {
    pub slots: [Option<Artifact<'a>>; 8],
    pub committed_witness: Option<CapturedWitness<'a>>,
}
#[derive(Debug)]
pub(super) struct Verified {
    pub original_input_verified: bool,
    pub reported_basis_linear_transitions_verified: bool,
    pub group_rows_verified: u64,
    pub first_witness_verified: bool,
    pub witness_capture_committed: bool,
    pub independent_group_domain_complete: bool,
    pub solver_completed_verified: bool,
    pub native_backend_invoked_reported: bool,
    pub native_statistics_complete_reported: bool,
    pub negative_verified: bool,
    pub independent_derivation_verified: bool,
    pub operation_trajectory_verified: bool,
}
const PATHS: [&str; 8] = [
    "prime-f4-input.json",
    "prime-f4-wide-dynamic-setup.json",
    "prime-f4-wide-dynamic-native.json",
    "prime-f4-wide-dynamic-events.jsonl",
    "prime-f4-wide-dynamic-linear.json",
    "prime-f4-wide-dynamic-group.json",
    "pdp-witness.json",
    "prime-f4-wide-dynamic-result.json",
];
const ROLES: [&str; 8] = [
    "complete-prime-field-polynomial-system",
    "restricted-wide-prime-dynamic-setup/v1",
    "restricted-wide-prime-dynamic-native/v1",
    "restricted-wide-prime-dynamic-events/v1",
    "restricted-wide-prime-dynamic-linear/v1",
    "restricted-wide-prime-dynamic-group/v1",
    "independently-verified-pdp",
    "restricted-wide-prime-dynamic-result/v1",
];
fn hash(s: &[u8], b: &mut Budget) -> R<[u8; 32]> {
    let mut h = Sha256::new();
    for c in s.chunks(64) {
        b.spend(65536)?;
        h.update(c)
    }
    b.spend(131072)?;
    Ok(h.finalize().into())
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Term {
    e: [u16; 28],
    c: u64,
}
type Poly = Vec<Term>;
fn order(a: &Term, c: &Term, v: usize) -> Ordering {
    a.e[..v]
        .iter()
        .map(|x| *x as u32)
        .sum::<u32>()
        .cmp(&c.e[..v].iter().map(|x| *x as u32).sum::<u32>())
        .then_with(|| {
            (0..v)
                .rev()
                .find_map(|i| (a.e[i] != c.e[i]).then(|| c.e[i].cmp(&a.e[i])))
                .unwrap_or(Ordering::Equal)
        })
}
fn poly(j: &J<'_>, v: usize, p: u64, cap: usize, b: &mut Budget) -> R<Poly> {
    let mut out = Vec::new();
    for t in j.arr(cap)? {
        t.obj(&["exponents", "coefficient"])?;
        let mut e = [0; 28];
        let a = t.get("exponents")?.arr(28)?;
        if a.len() != v {
            return Err(Error::Shape);
        }
        for (i, x) in a.iter().enumerate() {
            e[i] = u16::try_from(x.n()?).map_err(|_| Error::Shape)?
        }
        let c = t.get("coefficient")?.d64()?;
        if c == 0 || c >= p {
            return Err(Error::Arithmetic);
        }
        let t = Term { e, c };
        if out
            .last()
            .is_some_and(|x| order(x, &t, v) != Ordering::Greater)
        {
            return Err(Error::Shape);
        }
        b.push(&mut out, t, cap)?
    }
    Ok(out)
}
fn insert(out: &mut Poly, t: Term, p: u64, b: &mut Budget) -> R<()> {
    if t.c == 0 {
        return Ok(());
    }
    for old in out.iter_mut() {
        b.spend(64)?;
        if old.e == t.e {
            old.c = add(old.c, t.c, p);
            return Ok(());
        }
    }
    b.push(out, t, 8192)
}
fn canon(out: &mut Poly, v: usize, b: &mut Budget) -> R<()> {
    let n = out.len() as u64;
    b.spend(n.saturating_mul(n).saturating_mul(32))?;
    out.retain(|x| x.c != 0);
    out.sort_unstable_by(|a, c| order(c, a, v));
    Ok(())
}
fn eval(poly: &[Term], x: &[u64], p: u64, b: &mut Budget) -> R<u64> {
    let mut r = 0;
    for t in poly {
        let mut c = t.c;
        for (i, &power) in t.e[..x.len()].iter().enumerate() {
            b.spend(16)?;
            if power != 0 {
                c = mul(c, pow(x[i], power as u64, p, b)?, p)
            }
        }
        r = add(r, c, p)
    }
    Ok(r)
}
fn product(a: &[Term], c: &[Term], v: usize, p: u64, b: &mut Budget) -> R<Poly> {
    let mut out = Vec::new();
    for x in a {
        for y in c {
            b.spend(128)?;
            let mut e = [0; 28];
            for i in 0..v {
                e[i] = x.e[i].checked_add(y.e[i]).ok_or(Error::Arithmetic)?
            }
            insert(
                &mut out,
                Term {
                    e,
                    c: mul(x.c, y.c, p),
                },
                p,
                b,
            )?
        }
    }
    canon(&mut out, v, b)?;
    Ok(out)
}
fn atom(variable: Option<usize>, constant: u64, b: &mut Budget) -> R<Poly> {
    let mut e = [0; 28];
    let c = if let Some(i) = variable {
        e[i] = 1;
        1
    } else {
        constant
    };
    let mut out = Vec::new();
    b.push(&mut out, Term { e, c }, 1)?;
    Ok(out)
}
// Independent algebraic identity: (xy+xz+yz-a)^2-4xyz(x+y+z)-4b(x+y+z).
// This deliberately does not copy the producer's thirteen-term transcription.
fn original(e: &Expected<'_>, b: &mut Budget) -> R<Vec<Poly>> {
    let m = e.m;
    let v = 2 * m - 2;
    let mut all = Vec::new();
    for i in 0..m - 1 {
        let axes = if i == 0 {
            [Some(0), Some(1), Some(m)]
        } else if i == m - 2 {
            [Some(2 * m - 3), Some(m - 1), None]
        } else {
            [Some(m + i - 1), Some(i + 1), Some(m + i)]
        };
        let x = atom(axes[0], e.target.x, b)?;
        let y = atom(axes[1], e.target.x, b)?;
        let z = atom(axes[2], e.target.x, b)?;
        let mut pair = Vec::new();
        for (u, w) in [(&x, &y), (&x, &z), (&y, &z)] {
            for t in product(u, w, v, e.p, b)? {
                insert(&mut pair, t, e.p, b)?
            }
        }
        insert(
            &mut pair,
            Term {
                e: [0; 28],
                c: sub(0, e.a, e.p),
            },
            e.p,
            b,
        )?;
        let mut out = product(&pair, &pair, v, e.p, b)?;
        let xy = product(&x, &y, v, e.p, b)?;
        let xyz = product(&xy, &z, v, e.p, b)?;
        for q in [&x, &y, &z] {
            for mut t in product(&xyz, q, v, e.p, b)? {
                t.c = sub(0, mul(4, t.c, e.p), e.p);
                insert(&mut out, t, e.p, b)?
            }
            for mut t in q.iter().copied() {
                t.c = sub(0, mul(mul(4, e.b, e.p), t.c, e.p), e.p);
                insert(&mut out, t, e.p, b)?
            }
        }
        canon(&mut out, v, b)?;
        b.push(&mut all, out, 29)?
    }
    for leaf in 0..m {
        let mut out = atom(None, 1, b)?;
        for &code in e.codes {
            let mut factor = atom(Some(leaf), 0, b)?;
            insert(
                &mut factor,
                Term {
                    e: [0; 28],
                    c: sub(0, code, e.p),
                },
                e.p,
                b,
            )?;
            out = product(&out, &factor, v, e.p, b)?
        }
        canon(&mut out, v, b)?;
        b.push(&mut all, out, 29)?
    }
    Ok(all)
}
fn integers(j: &J<'_>, out: &mut [u64]) -> R<()> {
    let a = j.arr(out.len())?;
    if a.len() != out.len() {
        return Err(Error::Shape);
    }
    for (x, y) in out.iter_mut().zip(a) {
        *x = y.n()?
    }
    Ok(())
}
fn input(j: &J<'_>, e: &Expected<'_>, original: &[Poly], b: &mut Budget) -> R<()> {
    j.obj(&[
        "schema",
        "strategy",
        "m",
        "n_vars",
        "modulus",
        "curve",
        "target",
        "target_x",
        "ring",
        "variables",
        "variable_roles",
        "monomial_order",
        "domains",
        "factor_base_x",
        "signed_factor_base",
        "polynomials",
        "chain_equation_count",
        "membership_equation_count",
        "membership_variables",
        "native_normalization_applied",
        "coefficient_arithmetic",
        "native_modulus_limit_exclusive",
        "affine_chain_exceptional_infinity_branches_covered",
        "negative_scope",
    ])?;
    let v = 2 * e.m - 2;
    j.get("schema")?.eqs("ml-dataset.prime-f4-chain-input/v1")?;
    j.get("strategy")?
        .eqs("prime-affine-s3-chain-f4-visitor/v1")?;
    j.get("m")?.eqn(e.m as u64)?;
    j.get("n_vars")?.eqn(v as u64)?;
    if j.get("modulus")?.d64()? != e.p
        || point(j.get("target")?, false)? != Some(e.target)
        || j.get("target_x")?.d64()? != e.target.x
    {
        return Err(Error::Identity);
    }
    let c = j.get("curve")?;
    c.obj(&["a", "b"])?;
    if c.get("a")?.d64()? != e.a || c.get("b")?.d64()? != e.b {
        return Err(Error::Identity);
    }
    j.get("ring")?.eqs("Fp[x0,...,x(m-1),u0,...,u(m-3)]")?;
    j.get("monomial_order")?.eqs("degrevlex")?;
    j.get("chain_equation_count")?.eqn((e.m - 1) as u64)?;
    j.get("membership_equation_count")?.eqn(e.m as u64)?;
    j.get("native_normalization_applied")?.no()?;
    j.get("affine_chain_exceptional_infinity_branches_covered")?
        .no()?;
    j.get("native_modulus_limit_exclusive")?.eqs("4294967296")?;
    j.get("coefficient_arithmetic")?.eqs(
        "u128 modular intermediates; merged nonzero terms, leading-first, no monic rescaling",
    )?;
    j.get("negative_scope")?
        .eqs("encoding-relative only; no exhaustive PDP nonexistence claim")?;
    let names = j.get("variables")?.arr(28)?;
    let roles = j.get("variable_roles")?.arr(28)?;
    let domains = j.get("domains")?.arr(28)?;
    if names.len() != v || roles.len() != v || domains.len() != v {
        return Err(Error::Shape);
    }
    for i in 0..v {
        let name = names[i].s()?;
        let prefix = if i < e.m { b'x' } else { b'u' };
        let index = if i < e.m { i } else { i - e.m };
        if name.as_bytes().first() != Some(&prefix)
            || decimal(name.get(1..).ok_or(Error::Shape)?)? != index as u128
        {
            return Err(Error::Identity);
        }
        roles[i].obj(&["index", "name", "role", "domain"])?;
        roles[i].get("index")?.eqn(i as u64)?;
        roles[i].get("name")?.eqs(name)?;
        roles[i].get("role")?.eqs(if i < e.m {
            "factor_base_leaf"
        } else {
            "unrestricted_affine_auxiliary"
        })?;
        roles[i]
            .get("domain")?
            .eqs(if i < e.m { "factor_base_x" } else { "all_Fp" })?;
        if i < e.m {
            let d = domains[i].arr(32)?;
            if d.len() != e.codes.len() {
                return Err(Error::Identity);
            }
            for (x, &y) in d.iter().zip(e.codes) {
                x.eqn(y)?
            }
        } else if !domains[i].is_null() {
            return Err(Error::Shape);
        }
    }
    let codes = j.get("factor_base_x")?.arr(32)?;
    if codes.len() != e.codes.len() {
        return Err(Error::Identity);
    }
    for (x, &y) in codes.iter().zip(e.codes) {
        if x.d64()? != y {
            return Err(Error::Identity);
        }
    }
    let domain = j.get("signed_factor_base")?.arr(64)?;
    if domain.len() != e.domain.len() {
        return Err(Error::Identity);
    }
    for (x, &y) in domain.iter().zip(e.domain) {
        if point(x, false)? != Some(y) {
            return Err(Error::Identity);
        }
    }
    let vars = j.get("membership_variables")?.arr(15)?;
    if vars.len() != e.m {
        return Err(Error::Shape);
    }
    for (i, x) in vars.iter().enumerate() {
        x.eqn(i as u64)?
    }
    let polys = j.get("polynomials")?.arr(29)?;
    if polys.len() != original.len() {
        return Err(Error::Shape);
    }
    for (x, want) in polys.iter().zip(original) {
        if poly(x, v, e.p, 33, b)? != *want {
            return Err(Error::Arithmetic);
        }
    }
    Ok(())
}
struct Native {
    invoked: bool,
    statistics_complete: bool,
    basis: Vec<Poly>,
    retained: bool,
    unit: bool,
    work: u64,
    bytes: u64,
    events: u64,
    supplied_work: u64,
    stop_index: usize,
}
const NATIVE_STOPS: [&str; 16] = [
    "complete",
    "unit_derived",
    "invalid_input",
    "work_limit",
    "storage_limit",
    "row_limit",
    "column_limit",
    "basis_limit",
    "pair_limit",
    "pair_degree_limit",
    "step_limit",
    "event_limit",
    "deadline",
    "observer",
    "allocation",
    "exponent_overflow",
];
fn native(j: &J<'_>, e: &Expected<'_>, b: &mut Budget) -> R<Native> {
    j.obj(&["metadata", "basis"])?;
    let x = j.get("metadata")?;
    x.obj(&[
        "schema",
        "m",
        "variables",
        "stop",
        "supplied",
        "consumed",
        "limits",
        "acknowledged_events",
        "native_backend_invoked",
        "native_statistics_complete",
        "matrix_rows_reduced",
        "pivot_updates",
        "pairs_pending",
        "pairs_selected_unfinished",
        "maximum_rows_admitted",
        "maximum_columns_admitted",
        "maximum_pending_pairs",
        "completed_steps",
        "maximum_input_degree",
        "maximum_processed_row_degree",
        "basis_retained",
        "basis_retention_stop",
        "native_basis_derivation_verified",
        "independent_derivation_verified",
        "linear_extraction_complete",
        "group_domain_complete",
        "pdp_witness_verified",
        "exhaustive_no_decomposition",
        "solver_completed",
        "exact_instruction_count_verified",
        "full_execution_trace_verified",
        "fall_degree_verified",
    ])?;
    x.get("schema")?
        .eqs("crypto.ml-dataset.wide-prime-dynamic-native/v1")?;
    x.get("m")?.eqn(e.m as u64)?;
    x.get("variables")?.eqn((2 * e.m - 2) as u64)?;
    for k in [
        "native_basis_derivation_verified",
        "independent_derivation_verified",
        "linear_extraction_complete",
        "group_domain_complete",
        "pdp_witness_verified",
        "exhaustive_no_decomposition",
        "solver_completed",
        "exact_instruction_count_verified",
        "full_execution_trace_verified",
        "fall_degree_verified",
    ] {
        x.get(k)?.no()?
    }
    let stop = x.get("stop")?.s()?;
    if ![
        "complete",
        "unit_derived",
        "invalid_input",
        "work_limit",
        "storage_limit",
        "row_limit",
        "column_limit",
        "basis_limit",
        "pair_limit",
        "pair_degree_limit",
        "step_limit",
        "event_limit",
        "deadline",
        "observer",
        "allocation",
        "exponent_overflow",
    ]
    .contains(&stop)
    {
        return Err(Error::Shape);
    }
    let supplied = x.get("supplied")?;
    let consumed = x.get("consumed")?;
    for a in [supplied, consumed] {
        a.obj(&["work", "bytes"])?
    }
    let w = supplied.get("work")?.n()?;
    let by = supplied.get("bytes")?.n()?;
    let used = consumed.get("work")?.n()?;
    let usedb = consumed.get("bytes")?.n()?;
    if w > e
        .work_limit
        .checked_mul(1024)
        .ok_or(Error::Work)?
        .min(16_777_216)
        || by > (e.memory_bytes / 4).min(64 * 1024 * 1024)
        || used > w
        || usedb > by
    {
        return Err(Error::Claim);
    }
    let l = x.get("limits")?;
    l.obj(&[
        "work",
        "bytes",
        "rows",
        "columns",
        "basis",
        "pairs",
        "steps",
        "events",
        "max_pair_degree",
    ])?;
    l.get("work")?.eqn(w)?;
    l.get("bytes")?.eqn(by)?;
    for (k, cap) in [
        ("rows", 8192),
        ("columns", 8192),
        ("basis", 256),
        ("pairs", 32768),
        ("steps", 64),
        ("events", 4096),
        ("max_pair_degree", 12),
    ] {
        if l.get(k)?.n()? > cap {
            return Err(Error::Claim);
        }
    }
    for (k, limit) in [
        ("maximum_rows_admitted", "rows"),
        ("maximum_columns_admitted", "columns"),
        ("maximum_pending_pairs", "pairs"),
        ("completed_steps", "steps"),
        ("acknowledged_events", "events"),
    ] {
        if x.get(k)?.n()? > l.get(limit)?.n()? {
            return Err(Error::Claim);
        }
    }
    let retained = x.get("basis_retained")?.yes()?;
    let invoked = x.get("native_backend_invoked")?.yes()?;
    let statistics_complete = x.get("native_statistics_complete")?.yes()?;
    for key in [
        "matrix_rows_reduced",
        "pivot_updates",
        "pairs_pending",
        "pairs_selected_unfinished",
        "maximum_input_degree",
        "maximum_processed_row_degree",
    ] {
        x.get(key)?.n()?;
    }
    if statistics_complete && (!invoked || !["complete", "unit_derived"].contains(&stop)) {
        return Err(Error::Claim);
    }
    if !x.get("basis_retention_stop")?.is_null()
        && ![
            "InvalidInput",
            "WorkLimit",
            "StorageLimit",
            "Deadline",
            "Allocation",
            "ArithmeticOverflow",
            "TicketExhausted",
            "OutputLimit",
            "Serialization",
            "Observer",
            "Setup",
        ]
        .contains(&x.get("basis_retention_stop")?.s()?)
    {
        return Err(Error::Shape);
    }
    let mut basis = Vec::new();
    if retained {
        for p in j.get("basis")?.arr(256)? {
            let p = poly(p, 2 * e.m - 2, e.p, 8192, b)?;
            b.push(&mut basis, p, 256)?
        }
    } else if !j.get("basis")?.is_null() {
        return Err(Error::Shape);
    }
    Ok(Native {
        invoked,
        statistics_complete,
        basis,
        retained,
        unit: stop == "unit_derived",
        work: used,
        bytes: usedb,
        events: x.get("acknowledged_events")?.n()?,
        supplied_work: w,
        stop_index: NATIVE_STOPS
            .iter()
            .position(|s| *s == stop)
            .ok_or(Error::Shape)?,
    })
}
fn remainder(
    p: &[Term],
    assigned: &[Option<u64>; 28],
    v: usize,
    modulus: u64,
    b: &mut Budget,
) -> R<Poly> {
    let mut out = Vec::new();
    for &t in p {
        let mut t = t;
        for (i, value) in assigned[..v].iter().enumerate() {
            b.spend(16)?;
            if let Some(x) = value {
                t.c = mul(t.c, pow(*x, t.e[i] as u64, modulus, b)?, modulus);
                t.e[i] = 0
            }
        }
        insert(&mut out, t, modulus, b)?
    }
    canon(&mut out, v, b)?;
    Ok(out)
}
fn pivot(p: &[Term], v: usize) -> R<Option<(usize, u64, u64)>> {
    let mut axis = None;
    let mut a = 0;
    let mut c = 0;
    for t in p {
        let mut at = None;
        for i in 0..v {
            if t.e[i] != 0 {
                if t.e[i] != 1 || at.is_some() {
                    return Ok(None);
                }
                at = Some(i)
            }
        }
        if let Some(i) = at {
            if axis.is_some_and(|old| old != i) {
                return Ok(None);
            }
            axis = Some(i);
            a = t.c
        } else {
            c = t.c
        }
    }
    if let Some(i) = axis {
        Ok(Some((i, a, c)))
    } else if c != 0 {
        Err(Error::Arithmetic)
    } else {
        Ok(None)
    }
}
struct LinearEvidence {
    assignments: [Option<u64>; 28],
    eligible: bool,
}
fn linear(
    j: &J<'_>,
    e: &Expected<'_>,
    native: &Native,
    orig: &[Poly],
    b: &mut Budget,
) -> R<LinearEvidence> {
    j.obj(&[
        "schema",
        "report",
        "assignments",
        "source_ids",
        "transitions",
        "unresolved",
        "residues",
    ])?;
    j.get("schema")?
        .eqs("crypto.ml-dataset.wide-prime-dynamic-linear/v1")?;
    let r = j.get("report")?;
    r.obj(&[
        "stop",
        "assignments",
        "completed_passes",
        "original_equations_checked",
        "candidate_verified_original",
        "algebra_reserved_work",
        "checking_reserved_work",
        "independent_derivation_verified",
        "pdp_witness_verified",
    ])?;
    r.get("independent_derivation_verified")?.no()?;
    r.get("pdp_witness_verified")?.no()?;
    if ![
        "reported_basis_candidate",
        "no_retained_basis",
        "native_unit",
        "contradiction",
        "outside_domain",
        "nonlinear_or_underdetermined",
        "missing_pivot",
        "term_limit",
        "invalid_basis",
        "invalid_workspace",
        "work_limit",
        "deadline",
        "original_equation_mismatch",
        "invalid_original",
    ]
    .contains(&r.get("stop")?.s()?)
    {
        return Err(Error::Shape);
    }
    let v = 2 * e.m - 2;
    let transitions = j.get("transitions")?.arr(v)?;
    if r.get("completed_passes")?.n()? > (v + 1) as u64 {
        return Err(Error::Claim);
    }
    for key in ["algebra_reserved_work", "checking_reserved_work"] {
        if r.get(key)?.n()?
            > e.work_limit
                .checked_mul(1024)
                .ok_or(Error::Work)?
                .min(16_777_216)
        {
            return Err(Error::Claim);
        }
    }
    r.get("assignments")?.eqn(transitions.len() as u64)?;
    let mut assigned = [None; 28];
    let mut ids = [0usize; 28];
    let mut last = [0u64; 2];
    if !transitions.is_empty() && (!native.retained || native.unit) {
        return Err(Error::Claim);
    }
    for t in transitions {
        t.obj(&[
            "variable",
            "basis_id",
            "a",
            "b",
            "value",
            "algebra_reserved_work",
            "checking_reserved_work",
        ])?;
        let mut best = None;
        for (i, p) in native.basis.iter().enumerate() {
            let p = remainder(p, &assigned, v, e.p, b)?;
            if let Some((axis, a, c)) = pivot(&p, v)? {
                if best.is_none_or(|(old, id, _, _)| (axis, i) < (old, id)) {
                    best = Some((axis, i, a, c))
                }
            }
        }
        let (axis, id, a, c) = best.ok_or(Error::Arithmetic)?;
        t.get("variable")?.eqn(axis as u64)?;
        t.get("basis_id")?.eqn(id as u64)?;
        t.get("a")?.eqn(a)?;
        t.get("b")?.eqn(c)?;
        let value = t.get("value")?.n()?;
        if assigned[axis].is_some()
            || value >= e.p
            || a == 0
            || add(mul(a, value, e.p), c, e.p) != 0
            || (axis < e.m && !e.codes.contains(&value))
        {
            return Err(Error::Arithmetic);
        }
        assigned[axis] = Some(value);
        ids[axis] = id;
        for (i, k) in ["algebra_reserved_work", "checking_reserved_work"]
            .iter()
            .enumerate()
        {
            let n = t.get(k)?.n()?;
            if n < last[i] {
                return Err(Error::Claim);
            }
            last[i] = n;
        }
    }
    let vals = j.get("assignments")?.arr(v)?;
    let sources = j.get("source_ids")?.arr(v)?;
    let unresolved = j.get("unresolved")?.arr(v)?;
    if vals.len() != v || sources.len() != v || unresolved.len() != v {
        return Err(Error::Shape);
    }
    for i in 0..v {
        if let Some(x) = assigned[i] {
            vals[i].eqn(x)?;
            sources[i].eqn(ids[i] as u64)?;
            if !unresolved[i].is_null() {
                return Err(Error::Shape);
            }
        } else {
            if !vals[i].is_null() {
                return Err(Error::Shape);
            }
            sources[i].eqn(0)?;
            unresolved[i].eqn(i as u64)?
        }
    }
    let residues = j.get("residues")?.arr(v + 1)?;
    r.get("original_equations_checked")?
        .eqn(residues.len() as u64)?;
    if !residues.is_empty() {
        let mut x = [0; 28];
        for i in 0..v {
            x[i] = assigned[i].ok_or(Error::Claim)?
        }
        for (p, residue) in orig.iter().zip(residues) {
            residue.eqn(eval(p, &x[..v], e.p, b)?)?
        }
    }
    if r.get("candidate_verified_original")?.yes()? {
        r.get("stop")?.eqs("reported_basis_candidate")?;
        if transitions.len() != v
            || residues.len() != v + 1
            || residues.iter().any(|x| x.n() != Ok(0))
        {
            return Err(Error::Claim);
        }
    }
    Ok(LinearEvidence {
        assignments: assigned,
        eligible: r.get("candidate_verified_original")?.yes()?,
    })
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Origin {
    Candidate,
    Lift,
    Independent,
}
fn origin(j: &J<'_>) -> R<Origin> {
    match j.s()? {
        "reported_basis_linear_candidate" => Ok(Origin::Candidate),
        "reported_basis_leaf_lift" => Ok(Origin::Lift),
        "independent_group_prefix" => Ok(Origin::Independent),
        _ => Err(Error::Shape),
    }
}
#[derive(Clone, Copy)]
struct RowFacts {
    origin: Origin,
    cursor: u128,
    indices: [usize; 15],
    prefix: [P; 14],
    completed: usize,
    complete: bool,
    matched: bool,
    coordinates: [u64; 28],
    residues: [u64; 29],
    has_coordinates: bool,
    has_residues: bool,
    exceptions: [usize; 13],
    exception_count: usize,
    coordinate_match: bool,
}
fn tuple_indices(j: &J<'_>, e: &Expected<'_>) -> R<[usize; 15]> {
    let mut out = [0; 15];
    let a = j.arr(15)?;
    if a.len() != e.m {
        return Err(Error::Shape);
    }
    for (i, x) in a.iter().enumerate() {
        out[i] = usize::try_from(x.n()?).map_err(|_| Error::Shape)?;
        if out[i] >= e.domain.len() {
            return Err(Error::Identity);
        }
    }
    Ok(out)
}
fn expected_tuple(
    mut cursor: u128,
    independent: bool,
    e: &Expected<'_>,
    assigned: &[Option<u64>; 28],
) -> R<[usize; 15]> {
    let mut out = [0; 15];
    for i in (0..e.m).rev() {
        let mut choices = [0usize; 64];
        let mut n = 0;
        for (j, q) in e.domain.iter().enumerate() {
            if independent || assigned[i] == Some(q.x) {
                choices[n] = j;
                n += 1
            }
        }
        if n == 0 {
            return Err(Error::Identity);
        }
        out[i] = choices[(cursor % n as u128) as usize];
        cursor /= n as u128
    }
    if cursor != 0 {
        return Err(Error::Identity);
    }
    Ok(out)
}
fn tuple_math(
    indices: [usize; 15],
    completed: usize,
    e: &Expected<'_>,
    orig: &[Poly],
    b: &mut Budget,
) -> R<RowFacts> {
    if completed > e.m - 1 {
        return Err(Error::Shape);
    }
    let mut f = RowFacts {
        origin: Origin::Independent,
        cursor: 0,
        indices,
        prefix: [None; 14],
        completed,
        complete: completed == e.m - 1,
        matched: false,
        coordinates: [0; 28],
        residues: [0; 29],
        has_coordinates: false,
        has_residues: false,
        exceptions: [0; 13],
        exception_count: 0,
        coordinate_match: false,
    };
    for (i, &index) in indices[..e.m].iter().enumerate() {
        let q = *e.domain.get(index).ok_or(Error::Identity)?;
        f.coordinates[i] = q.x
    }
    let mut acc = Some(e.domain[indices[0]]);
    for i in 1..=completed {
        acc = sum(acc, Some(e.domain[indices[i]]), e, b)?;
        f.prefix[i - 1] = acc;
        if i < e.m - 1 {
            if let Some(q) = acc {
                f.coordinates[e.m + i - 1] = q.x
            } else {
                f.exceptions[f.exception_count] = i - 1;
                f.exception_count += 1
            }
        }
    }
    f.matched = f.complete && acc == Some(e.target);
    if f.complete && f.exception_count == 0 {
        f.has_coordinates = true;
        for (i, p) in orig.iter().enumerate() {
            f.residues[i] = eval(p, &f.coordinates[..2 * e.m - 2], e.p, b)?
        }
        f.has_residues = true;
    }
    Ok(f)
}
fn provenance(
    f: &mut RowFacts,
    e: &Expected<'_>,
    assigned: &[Option<u64>; 28],
    native: &Native,
    claimed: Option<bool>,
) -> R<()> {
    let actual = f.has_coordinates
        && (f.origin == Origin::Independent
            || assigned[..2 * e.m - 2]
                .iter()
                .zip(&f.coordinates)
                .all(|(a, x)| *a == Some(*x)));
    if f.has_coordinates {
        let zero = f.residues[..2 * e.m - 1].iter().all(|x| *x == 0);
        if native.unit && zero {
            return Err(Error::Claim);
        }
        if f.matched && !zero {
            return Err(Error::Arithmetic);
        }
    }
    if f.origin == Origin::Lift && (!f.matched || actual) {
        return Err(Error::Claim);
    }
    if let Some(value) = claimed {
        if (value && !actual) || (f.has_residues && value != actual) {
            return Err(Error::Claim);
        }
        f.coordinate_match = value;
    } else {
        f.coordinate_match = actual;
    }
    Ok(())
}
fn row(
    j: &J<'_>,
    e: &Expected<'_>,
    orig: &[Poly],
    assigned: &[Option<u64>; 28],
    native: &Native,
    b: &mut Budget,
) -> R<RowFacts> {
    j.obj(&[
        "schema",
        "origin",
        "cursor",
        "indices",
        "prefixes",
        "completed_additions",
        "exceptional_auxiliary_positions",
        "coordinates",
        "residues",
        "total",
        "computed_match",
        "candidate_coordinates_match",
        "complete",
        "tuples_reserved",
        "additions_reserved",
        "additions_performed",
        "checking_reserved_work",
    ])?;
    j.get("schema")?
        .eqs("crypto.ml-dataset.wide-prime-dynamic-group-row/v1")?;
    let o = origin(j.get("origin")?)?;
    let cursor = j.get("cursor")?.dec()?;
    let indices = tuple_indices(j.get("indices")?, e)?;
    if indices != expected_tuple(cursor, o == Origin::Independent, e, assigned)? {
        return Err(Error::Identity);
    }
    let n = usize::try_from(j.get("completed_additions")?.n()?).map_err(|_| Error::Shape)?;
    let mut f = tuple_math(indices, n, e, orig, b)?;
    f.origin = o;
    f.cursor = cursor;
    let prefixes = j.get("prefixes")?.arr(14)?;
    if prefixes.len() != n {
        return Err(Error::Shape);
    }
    for (x, &want) in prefixes.iter().zip(&f.prefix) {
        if point(x, true)? != want {
            return Err(Error::Arithmetic);
        }
    }
    let ex = j.get("exceptional_auxiliary_positions")?.arr(13)?;
    if ex.len() != f.exception_count {
        return Err(Error::Arithmetic);
    }
    for (x, &want) in ex.iter().zip(&f.exceptions) {
        x.eqn(want as u64)?
    }
    if j.get("complete")?.yes()? != f.complete || j.get("computed_match")?.yes()? != f.matched {
        return Err(Error::Claim);
    }
    let total = j.get("total")?;
    if f.complete {
        if point(total, true)? != f.prefix[e.m - 2] {
            return Err(Error::Arithmetic);
        }
    } else if !total.is_null() {
        return Err(Error::Shape);
    }
    if f.has_coordinates {
        let mut coords = [0; 28];
        integers(j.get("coordinates")?, &mut coords[..2 * e.m - 2])?;
        if coords != f.coordinates {
            return Err(Error::Arithmetic);
        }
    } else if !j.get("coordinates")?.is_null() {
        return Err(Error::Shape);
    }
    let residues = j.get("residues")?;
    f.has_residues = !residues.is_null();
    if f.has_residues {
        if !f.has_coordinates {
            return Err(Error::Shape);
        }
        let mut r = [0; 29];
        integers(residues, &mut r[..2 * e.m - 1])?;
        if r != f.residues {
            return Err(Error::Arithmetic);
        }
        if native.unit && r[..2 * e.m - 1].iter().all(|x| *x == 0) {
            return Err(Error::Claim);
        }
        if f.matched && r[..2 * e.m - 1].iter().any(|x| *x != 0) {
            return Err(Error::Arithmetic);
        }
    }
    provenance(
        &mut f,
        e,
        assigned,
        native,
        Some(j.get("candidate_coordinates_match")?.yes()?),
    )?;
    if j.get("additions_performed")?.n()? > j.get("additions_reserved")?.n()?
        || j.get("additions_reserved")?.n()? > e.work_limit
        || j.get("tuples_reserved")?.n()? > e.work_limit.min(1024)
    {
        return Err(Error::Claim);
    }
    Ok(f)
}
struct Rows {
    count: u64,
    native: u128,
    independent: u128,
    first: Option<RowFacts>,
    last: Option<RowFacts>,
    last_offset: u64,
    last_bytes: u64,
    independent_started: bool,
    native_total: Option<u128>,
    entered: u64,
    observable_additions: u64,
    last_counts: [u64; 4],
}
fn lines<'a>(
    bytes: &'a [u8],
    cap: usize,
    mut visit: impl FnMut(&'a [u8], usize) -> R<()>,
) -> R<()> {
    if bytes.is_empty() {
        return Ok(());
    }
    if bytes.last() != Some(&b'\n') {
        return Err(Error::Shape);
    }
    let mut offset = 0;
    for (count, line) in bytes.split_inclusive(|x| *x == b'\n').enumerate() {
        if count >= cap || line.len() == 1 {
            return Err(Error::Shape);
        }
        visit(line, offset)?;
        offset += line.len();
    }
    Ok(())
}
fn groups(
    bytes: &[u8],
    e: &Expected<'_>,
    orig: &[Poly],
    assigned: &[Option<u64>; 28],
    native: &Native,
    eligible: bool,
    b: &mut Budget,
) -> R<Rows> {
    let native_total = if eligible {
        let mut total = 1u128;
        for value in &assigned[..e.m] {
            let count = e.domain.iter().filter(|q| Some(q.x) == *value).count();
            if count == 0 {
                return Err(Error::Claim);
            }
            total = total.checked_mul(count as u128).ok_or(Error::Shape)?;
        }
        Some(total)
    } else {
        None
    };
    let mut state = Rows {
        count: 0,
        native: 0,
        independent: 0,
        first: None,
        last: None,
        last_offset: 0,
        last_bytes: 0,
        independent_started: false,
        native_total,
        entered: 0,
        observable_additions: 0,
        last_counts: [0; 4],
    };
    let mut counts = [0u64; 4];
    lines(bytes, e.work_limit.min(1024) as usize, |line, offset| {
        if state.last.is_some_and(|x| !x.complete) {
            return Err(Error::Claim);
        }
        let j = parse(line, b)?;
        let f = row(&j, e, orig, assigned, native, b)?;
        let independent = f.origin == Origin::Independent;
        if independent {
            if !state.independent_started && state.native != state.native_total.unwrap_or(0) {
                return Err(Error::Claim);
            }
            state.independent_started = true;
            if f.cursor != state.independent {
                return Err(Error::Identity);
            }
        } else if !eligible || state.independent_started || f.cursor != state.native {
            return Err(Error::Identity);
        }
        for (i, k) in [
            "tuples_reserved",
            "additions_reserved",
            "additions_performed",
            "checking_reserved_work",
        ]
        .iter()
        .enumerate()
        {
            let n = j.get(k)?.n()?;
            if n < counts[i] {
                return Err(Error::Claim);
            }
            counts[i] = n
        }
        if counts[0] != state.count + 1 {
            return Err(Error::Claim);
        }
        state.entered += 1;
        state.observable_additions += f.completed as u64;
        if counts[1] < state.entered * (e.m as u64 - 1) || counts[2] < state.observable_additions {
            return Err(Error::Claim);
        }
        state.last_counts = counts;
        if f.matched && state.first.is_none() {
            state.first = Some(f)
        }
        if f.complete {
            if independent {
                state.independent += 1
            } else {
                state.native += 1
            }
        }
        state.last = Some(f);
        state.last_offset = offset as u64;
        state.last_bytes = line.len() as u64;
        state.count += 1;
        Ok(())
    })?;
    Ok(state)
}
fn witness(
    j: &J<'_>,
    e: &Expected<'_>,
    orig: &[Poly],
    assigned: &[Option<u64>; 28],
    first: &RowFacts,
    b: &mut Budget,
) -> R<()> {
    j.obj(&[
        "schema",
        "summands",
        "target",
        "sum",
        "replay_group_add_calls",
        "certificate_origin",
        "search_phase",
        "source_cursor",
        "tuple_indices",
        "candidate_coordinates_match",
        "exceptional_auxiliary_positions",
        "independent_derivation_verified",
    ])?;
    j.get("schema")?
        .eqs("restricted-wide-prime-dynamic-witness/v1")?;
    j.get("independent_derivation_verified")?.no()?;
    j.get("replay_group_add_calls")?.eqn(e.m as u64)?;
    let indices = tuple_indices(j.get("tuple_indices")?, e)?;
    let o = origin(j.get("certificate_origin")?)?;
    if indices != first.indices
        || o != first.origin
        || j.get("source_cursor")?.dec()? != first.cursor
    {
        return Err(Error::Identity);
    }
    j.get("search_phase")?.eqs(if o == Origin::Independent {
        "independent_prefix"
    } else {
        "native_lifts"
    })?;
    let ps = j.get("summands")?.arr(15)?;
    if ps.len() != e.m {
        return Err(Error::Shape);
    }
    let mut acc = None;
    for (i, q) in ps.iter().enumerate() {
        let q = point(q, false)?;
        if q != Some(e.domain[indices[i]]) {
            return Err(Error::Identity);
        }
        acc = sum(acc, q, e, b)?;
    }
    if acc != Some(e.target)
        || point(j.get("sum")?, false)? != acc
        || point(j.get("target")?, false)? != Some(e.target)
    {
        return Err(Error::Arithmetic);
    }
    let f = tuple_math(indices, e.m - 1, e, orig, b)?;
    let ex = j.get("exceptional_auxiliary_positions")?.arr(13)?;
    if ex.len() != f.exception_count {
        return Err(Error::Arithmetic);
    }
    for (x, &y) in ex.iter().zip(&f.exceptions) {
        x.eqn(y as u64)?
    }
    if f.has_coordinates && f.residues[..2 * e.m - 1].iter().any(|x| *x != 0) {
        return Err(Error::Arithmetic);
    }
    let cm = j.get("candidate_coordinates_match")?;
    let matches = f.has_coordinates
        && assigned[..2 * e.m - 2]
            .iter()
            .zip(&f.coordinates)
            .all(|(x, y)| *x == Some(*y));
    match o {
        Origin::Independent => {
            if !cm.is_null() {
                return Err(Error::Shape);
            }
        }
        Origin::Candidate => {
            if !cm.yes()? || !matches {
                return Err(Error::Claim);
            }
        }
        Origin::Lift => {
            if cm.yes()? || matches {
                return Err(Error::Claim);
            }
        }
    }
    Ok(())
}
struct Events {
    counts: [u64; 4],
    global_end: u64,
    gaps: u64,
}
fn events(bytes: &[u8], native: &Native, b: &mut Budget) -> R<Events> {
    let mut out = Events {
        counts: [0; 4],
        global_end: 0,
        gaps: 0,
    };
    let mut prior = [[0u64; 2]; 4];
    let mut terminal = false;
    lines(bytes, 4096, |line, _| {
        let j = parse(line, b)?;
        j.obj(&[
            "schema",
            "source",
            "global_sequence",
            "local_sequence",
            "stage",
            "reserved_work",
            "reserved_bytes",
            "persistence",
        ])?;
        j.get("schema")?
            .eqs("crypto.ml-dataset.wide-prime-dynamic-event/v1")?;
        let k = match j.get("source")?.s()? {
            "setup" => 0,
            "native" => 1,
            "linear" => 2,
            "group" => 3,
            _ => return Err(Error::Shape),
        };
        let stage = j.get("stage")?.s()?;
        let valid = match k {
            0 => ["admitted", "complete"].contains(&stage),
            1 => ["begin", "initial", "basis", "batch", "terminal"].contains(&stage),
            2 => ["linear", "transition", "complete", "partial"].contains(&stage),
            _ => ["tuple", "partial_tuple"].contains(&stage),
        };
        if !valid {
            return Err(Error::Shape);
        }
        if k == 1 && terminal {
            return Err(Error::Claim);
        }
        terminal |= k == 1 && stage == "terminal";
        j.get("local_sequence")?.eqn(out.counts[k])?;
        if k == 0 {
            if out.counts[0] >= 2 {
                return Err(Error::Claim);
            }
            j.get("stage")?.eqs(if out.counts[0] == 0 {
                "admitted"
            } else {
                "complete"
            })?;
            j.get("reserved_work")?.eqn(131072)?;
            j.get("reserved_bytes")?.eqn(4096)?;
        }
        let g = j.get("global_sequence")?.n()?;
        if g < out.global_end || g - out.global_end > 1 {
            return Err(Error::Claim);
        }
        out.gaps += g - out.global_end;
        out.global_end = g + 1;
        if out.global_end > 4096 || out.gaps > 1 {
            return Err(Error::Claim);
        }
        for (i, key) in ["reserved_work", "reserved_bytes"].iter().enumerate() {
            let x = j.get(key)?.n()?;
            if x < prior[k][i] {
                return Err(Error::Claim);
            }
            prior[k][i] = x;
        }
        if !["local_only", "bound_capture"].contains(&j.get("persistence")?.s()?) {
            return Err(Error::Shape);
        }
        out.counts[k] += 1;
        Ok(())
    })?;
    if out.counts[1] != native.events
        || prior[1][0] > native.work
        || prior[1][1] > native.bytes
        || (terminal && (prior[1][0] != native.work || prior[1][1] != native.bytes))
    {
        return Err(Error::Claim);
    }
    Ok(out)
}
fn hex(j: &J<'_>) -> R<[u8; 32]> {
    let s = j.s()?.as_bytes();
    if s.len() != 64 {
        return Err(Error::Shape);
    }
    let mut out = [0; 32];
    for (i, c) in s.iter().enumerate() {
        let d = match c {
            b'0'..=b'9' => c - b'0',
            b'a'..=b'f' => c - b'a' + 10,
            _ => return Err(Error::Shape),
        };
        out[i / 2] = out[i / 2] * 16 + d;
    }
    Ok(out)
}
fn snapshot(
    j: &J<'_>,
    e: &Expected<'_>,
    orig: &[Poly],
    assigned: &[Option<u64>; 28],
    native: &Native,
    rows: &mut Rows,
    b: &mut Budget,
) -> R<()> {
    if j.is_null() {
        return Ok(());
    }
    j.obj(&[
        "phase",
        "origin",
        "cursor",
        "tuple_indices",
        "progress",
        "tuple_entered",
        "completed_prefixes",
        "prefix_points",
        "exceptional_auxiliary_positions",
        "coordinate_count",
        "coordinates",
        "residue_count",
        "residues",
        "computed_match",
        "row",
        "metadata_acknowledged",
    ])?;
    let o = origin(j.get("origin")?)?;
    j.get("phase")?.eqs(if o == Origin::Independent {
        "independent_prefix"
    } else {
        "native_lifts"
    })?;
    let cursor = j.get("cursor")?.dec()?;
    let indices = tuple_indices(j.get("tuple_indices")?, e)?;
    if indices != expected_tuple(cursor, o == Origin::Independent, e, assigned)? {
        return Err(Error::Identity);
    }
    let n = usize::try_from(j.get("completed_prefixes")?.n()?).map_err(|_| Error::Shape)?;
    let mut f = tuple_math(indices, n, e, orig, b)?;
    f.origin = o;
    f.cursor = cursor;
    provenance(&mut f, e, assigned, native, None)?;
    let ps = j.get("prefix_points")?.arr(14)?;
    if ps.len() != n {
        return Err(Error::Shape);
    }
    for (x, &y) in ps.iter().zip(&f.prefix) {
        if point(x, false)? != y {
            return Err(Error::Arithmetic);
        }
    }
    let ex = j.get("exceptional_auxiliary_positions")?.arr(13)?;
    if ex.len() != f.exception_count {
        return Err(Error::Arithmetic);
    }
    for (x, &y) in ex.iter().zip(&f.exceptions) {
        x.eqn(y as u64)?
    }
    let co = j.get("coordinates")?.arr(28)?;
    j.get("coordinate_count")?.eqn(co.len() as u64)?;
    let re = j.get("residues")?.arr(29)?;
    j.get("residue_count")?.eqn(re.len() as u64)?;
    if co.len() > 2 * e.m - 2 || re.len() > 2 * e.m - 1 {
        return Err(Error::Shape);
    }
    for (i, x) in co.iter().enumerate() {
        if i >= e.m && (i - e.m >= n || f.exceptions[..f.exception_count].contains(&(i - e.m))) {
            return Err(Error::Shape);
        }
        x.eqn(f.coordinates[i])?
    }
    if !re.is_empty() && !f.has_coordinates {
        return Err(Error::Shape);
    }
    for (x, &y) in re.iter().zip(&f.residues) {
        x.eqn(y)?
    }
    if j.get("computed_match")?.yes()? != f.matched {
        return Err(Error::Claim);
    }
    let entered = j.get("tuple_entered")?.yes()?;
    let progress = j.get("progress")?.s()?;
    if !["before_tuple", "partial_tuple", "complete_tuple"].contains(&progress)
        || (!entered && (progress != "before_tuple" || n != 0 || !re.is_empty()))
        || (progress == "complete_tuple" && !f.complete)
        || (progress == "partial_tuple" && !entered)
    {
        return Err(Error::Claim);
    }
    let reference = j.get("row")?;
    if !reference.is_null() {
        reference.obj(&["row_index", "offset", "bytes"])?;
        let old = rows.last.ok_or(Error::Identity)?;
        reference
            .get("row_index")?
            .eqn(rows.count.checked_sub(1).ok_or(Error::Identity)?)?;
        reference.get("offset")?.eqn(rows.last_offset)?;
        reference.get("bytes")?.eqn(rows.last_bytes)?;
        if old.cursor != f.cursor
            || old.indices != f.indices
            || old.origin != f.origin
            || old.completed != f.completed
            || old.matched != f.matched
        {
            return Err(Error::Identity);
        }
    } else {
        j.get("metadata_acknowledged")?.no()?;
        if o == Origin::Independent && rows.native != rows.native_total.unwrap_or(0) {
            return Err(Error::Claim);
        }
        if o != Origin::Independent && (rows.independent_started || rows.native_total.is_none()) {
            return Err(Error::Claim);
        }
        let expected = if o == Origin::Independent {
            rows.independent
        } else {
            rows.native
        };
        if cursor != expected {
            return Err(Error::Identity);
        }
        if entered {
            if rows.last.is_some_and(|old| !old.complete) {
                return Err(Error::Claim);
            }
            if f.matched && rows.first.is_none() {
                rows.first = Some(f)
            }
            rows.entered += 1;
            rows.observable_additions += f.completed as u64;
            if f.complete {
                if o == Origin::Independent {
                    rows.independent += 1
                } else {
                    rows.native += 1
                }
            }
            rows.last = Some(f);
        }
    }
    Ok(())
}
fn receipt(j: &J<'_>, capture: &CapturedWitness<'_>, artifact: &Artifact<'_>) -> R<bool> {
    j.obj(&[
        "commit",
        "payload_sha256",
        "payload_bytes",
        "sequence",
        "provenance",
    ])?;
    if hex(j.get("payload_sha256")?)? != capture.sha256
        || capture.sha256 != artifact.sha256
        || capture.bytes != artifact.bytes
    {
        return Err(Error::Identity);
    }
    j.get("payload_bytes")?.eqn(artifact.bytes.len() as u64)?;
    match j.get("provenance")?.s()? {
        "dedicated" => j.get("sequence")?.eqn(capture.original_sequence)?,
        "legacy_adopted" => {
            if !j.get("sequence")?.is_null() {
                return Err(Error::Shape);
            }
        }
        _ => return Err(Error::Shape),
    }
    match j.get("commit")?.s()? {
        "new_commit" => {
            j.get("provenance")?.eqs("dedicated")?;
            Ok(true)
        }
        "already_committed" => Ok(false),
        _ => Err(Error::Shape),
    }
}
fn first_record(
    j: &J<'_>,
    e: &Expected<'_>,
    rows: &Rows,
    bundle: &Bundle<'_>,
) -> R<(bool, bool, bool)> {
    if j.is_null() {
        if rows.first.is_some() || bundle.slots[6].is_some() {
            return Err(Error::Claim);
        }
        return Ok((false, false, false));
    }
    j.obj(&[
        "lifecycle",
        "identity",
        "facts",
        "replay",
        "publication",
        "witness_owner",
        "witness_bytes",
        "witness_sha256",
    ])?;
    if ![
        "computed_pending",
        "replay_admitted",
        "locally_verified",
        "published",
        "transferred",
        "failed",
    ]
    .contains(&j.get("lifecycle")?.s()?)
    {
        return Err(Error::Shape);
    }
    let f = rows.first.ok_or(Error::Claim)?;
    let i = j.get("identity")?;
    i.obj(&[
        "origin",
        "search_phase",
        "source_cursor",
        "tuple_indices",
        "exceptional_auxiliary_positions",
        "candidate_coordinates_match",
    ])?;
    if origin(i.get("origin")?)? != f.origin
        || i.get("source_cursor")?.dec()? != f.cursor
        || tuple_indices(i.get("tuple_indices")?, e)? != f.indices
    {
        return Err(Error::Identity);
    }
    i.get("search_phase")?
        .eqs(if f.origin == Origin::Independent {
            "independent_prefix"
        } else {
            "native_lifts"
        })?;
    let ex = i.get("exceptional_auxiliary_positions")?.arr(13)?;
    if ex.len() != f.exception_count {
        return Err(Error::Arithmetic);
    }
    for (x, &y) in ex.iter().zip(&f.exceptions) {
        x.eqn(y as u64)?
    }
    let relation = i.get("candidate_coordinates_match")?;
    if f.origin == Origin::Independent {
        if !relation.is_null() {
            return Err(Error::Shape);
        }
    } else if relation.yes()? != f.coordinate_match {
        return Err(Error::Claim);
    }
    let facts = j.get("facts")?;
    facts.obj(&[
        "computed",
        "local_replay_complete",
        "publication_attempted",
        "durable_acknowledged",
        "final_artifact_registered",
    ])?;
    if !facts.get("computed")?.yes()? {
        return Err(Error::Claim);
    }
    facts.get("final_artifact_registered")?.no()?;
    let local = facts.get("local_replay_complete")?.yes()?;
    let attempted = facts.get("publication_attempted")?.yes()?;
    let durable = facts.get("durable_acknowledged")?.yes()?;
    if (durable && !attempted) || (attempted && !local) {
        return Err(Error::Claim);
    }
    let replay = j.get("replay")?;
    replay.obj(&[
        "additions_reserved",
        "additions_performed",
        "equation_checks_completed",
        "coordinate_checks_completed",
        "failed_stage",
    ])?;
    let reserve = replay.get("additions_reserved")?.n()?;
    let performed = replay.get("additions_performed")?.n()?;
    if reserve > e.m as u64
        || performed > reserve
        || replay.get("equation_checks_completed")?.n()? > (2 * e.m - 1) as u64
        || replay.get("coordinate_checks_completed")?.n()? > (2 * e.m - 2) as u64
    {
        return Err(Error::Claim);
    }
    if local
        && (performed != e.m as u64
            || (!f.has_coordinates && replay.get("equation_checks_completed")?.n()? != 0)
            || (f.has_coordinates
                && replay.get("equation_checks_completed")?.n()? != (2 * e.m - 1) as u64))
    {
        return Err(Error::Claim);
    }
    if !replay.get("failed_stage")?.is_null()
        && ![
            "indices",
            "on_curve",
            "group",
            "prefix_compare",
            "original_equations",
            "provenance",
            "serialization",
            "publication",
            "final_staging",
        ]
        .contains(&replay.get("failed_stage")?.s()?)
    {
        return Err(Error::Shape);
    }
    let publication = j.get("publication")?;
    publication.obj(&["state", "receipt"])?;
    let state = publication.get("state")?.s()?;
    if !["not_attempted", "refused", "indeterminate", "acknowledged"].contains(&state)
        || durable != (state == "acknowledged")
    {
        return Err(Error::Claim);
    }
    let mut new = false;
    if durable {
        let a = bundle.slots[6].as_ref().ok_or(Error::Identity)?;
        let c = bundle.committed_witness.as_ref().ok_or(Error::Identity)?;
        new = receipt(publication.get("receipt")?, c, a)?;
        if hex(j.get("witness_sha256")?)? != a.sha256 {
            return Err(Error::Identity);
        }
        j.get("witness_bytes")?.eqn(a.bytes.len() as u64)?;
        j.get("witness_owner")?.eqs("final_artifact")?;
    } else {
        if !publication.get("receipt")?.is_null()
            || !j.get("witness_sha256")?.is_null()
            || bundle.slots[6].is_some()
        {
            return Err(Error::Claim);
        }
        j.get("witness_owner")?.eqs("slot6")?;
        if !j.get("witness_bytes")?.is_null() {
            j.get("witness_bytes")?.n()?;
        }
    }
    Ok((local, durable, new))
}
fn terminal(
    j: &J<'_>,
    e: &Expected<'_>,
    bundle: &Bundle<'_>,
    rows: &Rows,
    ev: &Events,
    native: &Native,
    local: bool,
    durable: bool,
    new_payload: bool,
) -> R<bool> {
    j.obj(&[
        "schema",
        "strategy",
        "stage",
        "disposition",
        "cause",
        "native_stop",
        "linear_stop",
        "group_stop",
        "search_phase",
        "accounting",
        "source_slots",
        "last_state",
        "first_certificate",
        "claims",
    ])?;
    j.get("schema")?
        .eqs("restricted-wide-prime-dynamic-result/v1")?;
    j.get("strategy")?.eqs("prime_f4_wide_dynamic")?;
    j.get("native_stop")?.eqs(NATIVE_STOPS[native.stop_index])?;
    if !j.get("linear_stop")?.is_null()
        && ![
            "reported_basis_candidate",
            "no_retained_basis",
            "native_unit",
            "contradiction",
            "outside_domain",
            "nonlinear_or_underdetermined",
            "missing_pivot",
            "term_limit",
            "invalid_basis",
            "invalid_workspace",
            "work_limit",
            "deadline",
            "original_equation_mismatch",
            "invalid_original",
        ]
        .contains(&j.get("linear_stop")?.s()?)
    {
        return Err(Error::Shape);
    }
    if !j.get("group_stop")?.is_null()
        && ![
            "independent_traversal_finished",
            "pending_certificate",
            "already_started",
            "invalid_domain",
            "invalid_candidate",
            "native_contradiction",
            "original_equation_mismatch",
            "tuple_limit",
            "addition_limit",
            "check_work_limit",
            "group_bytes",
            "event_limit",
            "persistence_failure",
            "deadline",
            "invalid_state",
        ]
        .contains(&j.get("group_stop")?.s()?)
    {
        return Err(Error::Shape);
    }
    if ![
        "not_started",
        "native_lifts",
        "independent_prefix",
        "finished",
    ]
    .contains(&j.get("search_phase")?.s()?)
    {
        return Err(Error::Shape);
    }
    if ![
        "preparation",
        "native",
        "linear",
        "group",
        "certificate_replay",
        "publication",
        "final_staging",
        "terminal",
    ]
    .contains(&j.get("stage")?.s()?)
    {
        return Err(Error::Shape);
    }
    let disposition = j.get("disposition")?.s()?;
    if !["complete", "censored", "failed"].contains(&disposition) {
        return Err(Error::Shape);
    }
    if ![
        "none",
        "invalid_input",
        "invalid_state",
        "native_stop",
        "linear_stop",
        "group_stop",
        "work_limit",
        "storage_limit",
        "artifact_bytes",
        "event_limit",
        "addition_limit",
        "tuple_limit",
        "deadline",
        "allocation_failure",
        "replay_mismatch",
        "native_contradiction",
        "persistence_failure",
        "indeterminate_io",
        "serialization_failure",
    ]
    .contains(&j.get("cause")?.s()?)
    {
        return Err(Error::Shape);
    }
    let slots = j.get("source_slots")?.arr(7)?;
    if slots.len() != 7 {
        return Err(Error::Shape);
    }
    for (i, s) in slots.iter().enumerate() {
        s.obj(&["index", "state", "bytes", "digest_binding", "sha256"])?;
        s.get("index")?.eqn(i as u64)?;
        let state = s.get("state")?.s()?;
        if ![
            "not_produced",
            "partial_unpromoted",
            "local_complete",
            "transferred",
        ]
        .contains(&state)
        {
            return Err(Error::Shape);
        }
        let binding = s.get("digest_binding")?.s()?;
        if i != 6 || !durable {
            s.get("digest_binding")?.eqs("outer_manifest")?;
            if !s.get("sha256")?.is_null() {
                return Err(Error::Shape);
            }
        } else {
            if binding != "capture_receipt" {
                return Err(Error::Claim);
            }
            let artifact = bundle.slots[i].as_ref().ok_or(Error::Identity)?;
            if hex(s.get("sha256")?)? != artifact.sha256 {
                return Err(Error::Identity);
            }
        }
        if let Some(a) = &bundle.slots[i] {
            if !["local_complete", "transferred"].contains(&state) {
                return Err(Error::Claim);
            }
            s.get("bytes")?.eqn(a.bytes.len() as u64)?;
            if state == "transferred" && (i != 6 || !durable) {
                return Err(Error::Claim);
            }
        } else if state == "local_complete" && i == 6 && !durable {
            s.get("bytes")?.n()?;
        } else if !["not_produced", "partial_unpromoted"].contains(&state)
            || !s.get("bytes")?.is_null()
        {
            return Err(Error::Claim);
        }
    }
    let a = j.get("accounting")?;
    a.obj(&[
        "counter_snapshot_stage",
        "pools",
        "requested_bytes",
        "group",
        "traversal",
        "events",
    ])?;
    a.get("counter_snapshot_stage")?
        .eqs("before_terminal_retention")?;
    let w = e
        .work_limit
        .checked_mul(1024)
        .ok_or(Error::Work)?
        .min(16_777_216);
    let pools = a.get("pools")?.arr(6)?;
    if pools.len() != 6 {
        return Err(Error::Shape);
    }
    let mut reserves = [0; 6];
    let mut limits = [0; 6];
    for (i, p) in pools.iter().enumerate() {
        p.obj(&[
            "name",
            "parent",
            "work_limit",
            "reserved_work",
            "performed_work",
        ])?;
        p.get("name")?.eqs(
            [
                "algebra", "checking", "native", "observer", "capture", "terminal",
            ][i],
        )?;
        limits[i] = p.get("work_limit")?.n()?;
        reserves[i] = p.get("reserved_work")?.n()?;
        p.get("parent")?.eqs(match i {
            0 | 1 => "root",
            4 => "checking",
            _ => "algebra",
        })?;
        let performed = p.get("performed_work")?;
        if i == 2 {
            if !performed.is_null() {
                return Err(Error::Claim);
            }
        } else if performed.n()? > reserves[i] {
            return Err(Error::Claim);
        }
        if reserves[i] > limits[i] {
            return Err(Error::Claim);
        }
    }
    if limits[0] != w * 3 / 4
        || limits[1] != w - w * 3 / 4
        || limits[5] > 262144
        || limits[2] > limits[0]
        || limits[3] > limits[0]
        || limits[4] > limits[1]
        || reserves[2]
            .saturating_add(reserves[3])
            .saturating_add(reserves[5])
            > reserves[0]
        || reserves[4] > reserves[1]
    {
        return Err(Error::Claim);
    }
    if limits[2] != native.supplied_work || reserves[2] != native.work {
        return Err(Error::Claim);
    }
    let requested = a.get("requested_bytes")?;
    requested.obj(&["limit", "reserved"])?;
    requested
        .get("limit")?
        .eqn((e.memory_bytes / 4).min(64 * 1024 * 1024))?;
    if requested.get("reserved")?.n()? > requested.get("limit")?.n()? {
        return Err(Error::Claim);
    }
    let g = a.get("group")?;
    g.obj(&[
        "addition_limit",
        "additions_reserved",
        "additions_performed",
        "tuple_limit",
        "tuples_reserved",
    ])?;
    g.get("addition_limit")?.eqn(e.work_limit)?;
    g.get("tuple_limit")?.eqn(e.work_limit.min(1024))?;
    if g.get("additions_performed")?.n()? > g.get("additions_reserved")?.n()?
        || g.get("additions_reserved")?.n()? > e.work_limit
        || g.get("tuples_reserved")?.n()? > e.work_limit.min(1024)
        || g.get("tuples_reserved")?.n()? < rows.entered
    {
        return Err(Error::Claim);
    }
    let first = j.get("first_certificate")?;
    let (replay_reserved, replay_performed) = if first.is_null() {
        (0, 0)
    } else {
        let progress = first.get("replay")?;
        (
            progress.get("additions_reserved")?.n()?,
            progress.get("additions_performed")?.n()?,
        )
    };
    if g.get("additions_reserved")?.n()? < rows.entered * (e.m as u64 - 1) + replay_reserved
        || g.get("additions_performed")?.n()? < rows.observable_additions + replay_performed
        || g.get("additions_reserved")?.n()? < rows.last_counts[1]
        || g.get("additions_performed")?.n()? < rows.last_counts[2]
    {
        return Err(Error::Claim);
    }
    let traversal = a.get("traversal")?;
    traversal.obj(&[
        "native_lifts_visited",
        "independent_visited",
        "independent_total",
    ])?;
    let total = (e.domain.len() as u128)
        .checked_pow(e.m as u32)
        .ok_or(Error::Shape)?;
    let not_started = j.get("search_phase")?.s()? == "not_started";
    if not_started
        && (!j.get("group_stop")?.is_null()
            || !j.get("last_state")?.is_null()
            || !j.get("first_certificate")?.is_null()
            || rows.count != 0
            || rows.entered != 0
            || rows.native != 0
            || rows.independent != 0
            || bundle.slots[5].is_some()
            || bundle.slots[6].is_some()
            || g.get("tuples_reserved")?.n()? != 0
            || g.get("additions_reserved")?.n()? != 0
            || g.get("additions_performed")?.n()? != 0)
    {
        return Err(Error::Claim);
    }
    let reported_total = if not_started { 0 } else { total };
    if traversal.get("native_lifts_visited")?.dec()? != rows.native
        || traversal.get("independent_visited")?.dec()? != rows.independent
        || traversal.get("independent_total")?.dec()? != reported_total
    {
        return Err(Error::Claim);
    }
    let es = a.get("events")?;
    es.obj(&[
        "limit",
        "global_acknowledged",
        "setup",
        "native",
        "linear",
        "group",
        "new_payload",
        "attempted",
        "pending",
    ])?;
    es.get("limit")?.eqn(4096)?;
    for (i, k) in ["setup", "native", "linear", "group"].iter().enumerate() {
        es.get(k)?.eqn(ev.counts[i])?
    }
    es.get("new_payload")?.eqn(u64::from(new_payload))?;
    let acknowledged = ev.counts.iter().sum::<u64>() + u64::from(new_payload);
    es.get("global_acknowledged")?.eqn(acknowledged)?;
    if acknowledged > 4096
        || ev.global_end > acknowledged
        || ev.gaps > u64::from(new_payload)
        || es.get("pending")?.n()? > 1
        || es.get("attempted")?.n()? < ev.counts.iter().sum::<u64>()
    {
        return Err(Error::Claim);
    }
    let claims = j.get("claims")?;
    claims.obj(&[
        "solver_completed",
        "group_domain_complete",
        "negative_verified",
        "pdp_witness_verified",
        "certificate_durably_persisted",
        "independent_derivation_verified",
        "operation_trajectory_verified",
    ])?;
    claims.get("independent_derivation_verified")?.no()?;
    claims.get("operation_trajectory_verified")?.no()?;
    let complete = rows.independent == total;
    let positive = claims.get("pdp_witness_verified")?.yes()?;
    if claims.get("certificate_durably_persisted")?.yes()? != durable
        || positive != local
        || (claims.get("group_domain_complete")?.yes()? && !complete)
        || (claims.get("negative_verified")?.yes()? && (!complete || rows.first.is_some()))
        || (claims.get("solver_completed")?.yes()?
            && (!complete
                || !claims.get("group_domain_complete")?.yes()?
                || disposition != "complete"
                || j.get("search_phase")?.s()? != "finished"
                || j.get("group_stop")?.s()? != "independent_traversal_finished"
                || j.get("cause")?.s()? != "none"
                || (positive && !durable)
                || (!positive && !claims.get("negative_verified")?.yes()?)))
    {
        return Err(Error::Claim);
    }
    if disposition == "complete" && !claims.get("solver_completed")?.yes()? {
        return Err(Error::Claim);
    }
    Ok(complete)
}
/// Outer custody adapter. `e`, `binding`, and `capture_limits` must be derived
/// from the verified original run; this function does not select a profile or
/// authenticate an arbitrary caller's Expected. No output files are written.
/// Raw file buffers belong to outer custody, separately from parser accounting.
pub(super) fn verify_files(
    dir: &std::path::Path,
    e: &Expected<'_>,
    artifacts: &[crate::ml_dataset::types::Artifact],
    binding: &crate::ml_dataset::capture::AttemptBinding,
    capture_limits: &crate::ml_dataset::capture::CaptureLimits,
    limits: ReplayLimits,
) -> Result<Verified, String> {
    use crate::ml_dataset::{capture, storage};
    let dir = storage::root_dir(dir, false)?;
    let caps = slot_caps(e.trace_bytes).map_err(|x| format!("dynamic slot limits: {x:?}"))?;
    let mut descriptors = [None; 8];
    let mut raw: [Vec<u8>; 8] = std::array::from_fn(|_| Vec::new());
    let mut hashes = [[0u8; 32]; 8];
    let mut total = 0u64;
    for i in 0..8 {
        let matching: Vec<_> = artifacts
            .iter()
            .filter(|a| a.path == PATHS[i] || a.role == ROLES[i])
            .collect();
        if matching.is_empty() {
            continue;
        }
        if matching.len() != 1 {
            return Err(format!("duplicate dynamic slot identity: {}", PATHS[i]));
        }
        let a = matching[0];
        let encoding = if i == 3 || i == 5 { "jsonl" } else { "json" };
        if a.path != PATHS[i] || a.role != ROLES[i] || a.encoding != encoding || a.bytes > caps[i] {
            return Err(format!(
                "dynamic slot identity or size mismatch: {}",
                PATHS[i]
            ));
        }
        total = total
            .checked_add(a.bytes)
            .ok_or("dynamic file size overflow")?;
        if total > e.memory_bytes || total > e.trace_bytes {
            return Err("dynamic file buffers exceed original allowance".into());
        }
        // Containment also rejects symlinks; bounded reader rechecks regularity
        // and catches growth before the child hashes these exact returned bytes.
        storage::contained_path(&dir, &a.path, false)?;
        raw[i] = super::prime_f4_wide::read_bounded(
            &dir,
            &a.path,
            usize::try_from(caps[i]).map_err(|_| "dynamic cap exceeds usize")?,
        )?;
        if raw[i].len() as u64 != a.bytes {
            return Err("dynamic file length differs from manifest".into());
        }
        hashes[i] = hex::decode(&a.sha256)
            .map_err(|_| "invalid dynamic digest")?
            .try_into()
            .map_err(|_| "invalid dynamic digest length")?;
        descriptors[i] = Some(a);
    }
    let prefix = capture::verify_live_committed_prefix(&dir, binding, capture_limits)?;
    let mut committed = None;
    if let Some(prefix) = &prefix {
        let matching: Vec<_> = prefix
            .payloads
            .iter()
            .filter(|(_, a)| a.path == "payloads/pdp-witness.json" || a.role == ROLES[6])
            .collect();
        if matching.len() > 1 {
            return Err("duplicate committed dynamic witness identity".into());
        }
        if let Some((sequence, a)) = matching.first().copied() {
            let canonical =
                descriptors[6].ok_or("committed dynamic witness lacks final artifact")?;
            if a.path != "payloads/pdp-witness.json"
                || a.role != ROLES[6]
                || a.encoding != "json"
                || a.sha256 != canonical.sha256
                || a.bytes != canonical.bytes
            {
                return Err("dynamic witness differs from committed publication".into());
            }
            committed = Some(CapturedWitness {
                bytes: &raw[6],
                sha256: hashes[6],
                original_sequence: *sequence,
            });
        }
    }
    if descriptors[6].is_some() && committed.is_none() {
        return Err("dynamic witness lacks bound committed publication".into());
    }
    let slots = std::array::from_fn(|i| {
        descriptors[i].map(|a| Artifact {
            path: &a.path,
            role: &a.role,
            encoding: &a.encoding,
            bytes: &raw[i],
            sha256: hashes[i],
        })
    });
    verify(
        e,
        &Bundle {
            slots,
            committed_witness: committed,
        },
        limits,
    )
    .map_err(|x| format!("dynamic mathematical replay: {x:?}"))
}

fn slot_caps(trace_bytes: u64) -> R<[u64; 8]> {
    let encoded = (trace_bytes / 32).min(4 * 1024 * 1024);
    if encoded <= 4 * 65536 {
        return Err(Error::Storage);
    }
    let a = encoded - 65536 - 4096;
    Ok([a / 3, 4096, a / 4, a / 12, a / 12, a / 6, a / 12, 65536])
}

/// Strict replay of a bound artifact bundle. Missing terminal evidence remains a
/// checked prefix and cannot produce durable-negative or completion assurance.
/// Parent must OR-trigger this profile from original config and reserved roles;
/// it must never use absence of this function's invocation as NotApplicable.
pub(super) fn verify(e: &Expected<'_>, bundle: &Bundle<'_>, limits: ReplayLimits) -> R<Verified> {
    let mut b = Budget {
        limits,
        work: 0,
        bytes: 0,
    };
    // Fixed inline parser/arithmetic/row snapshots, distinct from cumulative
    // requested heap storage; not a measured process RSS claim.
    b.bytes = 32 * 1024;
    if b.bytes > b.limits.allocation_bytes {
        return Err(Error::Storage);
    }
    b.spend(32 * 1024)?;
    if !(4..=15).contains(&e.m)
        || e.a >= e.p
        || e.b >= e.p
        || e.domain.is_empty()
        || e.domain.len() > 64
        || e.codes.is_empty()
        || e.codes.len() > 32
        || e.work_limit == 0
        || !prime(e.p, &mut b)?
    {
        return Err(Error::Identity);
    }
    if !on_curve(e.target, e)
        || add(
            mul(4, mul(mul(e.a, e.a, e.p), e.a, e.p), e.p),
            mul(27, mul(e.b, e.b, e.p), e.p),
            e.p,
        ) == 0
    {
        return Err(Error::Identity);
    }
    for (i, &q) in e.domain.iter().enumerate() {
        b.spend(256)?;
        if !on_curve(q, e) || !e.codes.contains(&q.x) || e.domain[..i].contains(&q) {
            return Err(Error::Identity);
        }
    }
    for (i, &x) in e.codes.iter().enumerate() {
        if x >= e.p || i > 0 && e.codes[i - 1] >= x || !e.domain.iter().any(|q| q.x == x) {
            return Err(Error::Identity);
        }
    }
    let caps = slot_caps(e.trace_bytes)?;
    for (i, artifact) in bundle.slots.iter().enumerate() {
        if let Some(f) = artifact {
            if f.path != PATHS[i]
                || f.role != ROLES[i]
                || f.encoding != if i == 3 || i == 5 { "jsonl" } else { "json" }
                || f.bytes.len() as u64 > caps[i]
            {
                return Err(Error::Identity);
            }
            if hash(f.bytes, &mut b)? != f.sha256 {
                return Err(Error::Identity);
            }
        }
    }
    let input_artifact = bundle.slots[0].as_ref().ok_or(Error::Identity)?;
    let orig = original(e, &mut b)?;
    input(&parse(input_artifact.bytes, &mut b)?, e, &orig, &mut b)?;
    let setup = parse(
        bundle.slots[1].as_ref().ok_or(Error::Identity)?.bytes,
        &mut b,
    )?;
    setup.obj(&[
        "schema",
        "modulus",
        "prime_verified",
        "reserved_work",
        "reserved_bytes",
        "acknowledged_events",
    ])?;
    setup
        .get("schema")?
        .eqs("crypto.ml-dataset.wide-prime-dynamic-setup/v1")?;
    if setup.get("modulus")?.d64()? != e.p || !setup.get("prime_verified")?.yes()? {
        return Err(Error::Claim);
    }
    setup.get("reserved_work")?.eqn(131072)?;
    setup.get("reserved_bytes")?.eqn(4096)?;
    setup.get("acknowledged_events")?.eqn(2)?;
    let n = native(
        &parse(
            bundle.slots[2].as_ref().ok_or(Error::Identity)?.bytes,
            &mut b,
        )?,
        e,
        &mut b,
    )?;
    let ev = events(
        bundle.slots[3].as_ref().ok_or(Error::Identity)?.bytes,
        &n,
        &mut b,
    )?;
    if ev.counts[0] != 2 {
        return Err(Error::Claim);
    }
    let (linear_evidence, linear_verified) = if let Some(artifact) = &bundle.slots[4] {
        (
            linear(&parse(artifact.bytes, &mut b)?, e, &n, &orig, &mut b)?,
            true,
        )
    } else {
        (
            LinearEvidence {
                assignments: [None; 28],
                eligible: false,
            },
            false,
        )
    };
    let assigned = linear_evidence.assignments;
    let mut rows = groups(
        bundle.slots[5].as_ref().map_or(&[][..], |a| a.bytes),
        e,
        &orig,
        &assigned,
        &n,
        linear_evidence.eligible,
        &mut b,
    )?;
    let mut local = false;
    let mut durable = false;
    let mut complete = false;
    let mut solver_completed = false;
    let mut negative_verified = false;
    if let Some(artifact) = &bundle.slots[7] {
        let result = parse(artifact.bytes, &mut b)?;
        snapshot(
            result.get("last_state")?,
            e,
            &orig,
            &assigned,
            &n,
            &mut rows,
            &mut b,
        )?;
        let (l, d, new) = first_record(result.get("first_certificate")?, e, &rows, bundle)?;
        local = l;
        durable = d;
        complete = terminal(&result, e, bundle, &rows, &ev, &n, l, d, new)?;
        solver_completed = result.get("claims")?.get("solver_completed")?.yes()?;
        negative_verified = result.get("claims")?.get("negative_verified")?.yes()?;
        if local {
            let first = rows.first.as_ref().ok_or(Error::Claim)?;
            if !first.matched
                || (first.has_coordinates
                    && (n.unit || first.residues[..2 * e.m - 1].iter().any(|x| *x != 0)))
            {
                return Err(Error::Claim);
            }
        }
        if durable {
            let artifact = bundle.slots[6].as_ref().ok_or(Error::Identity)?;
            let first = rows.first.as_ref().ok_or(Error::Claim)?;
            if n.unit
                && first.has_coordinates
                && first.residues[..2 * e.m - 1].iter().all(|x| *x == 0)
            {
                return Err(Error::Claim);
            }
            witness(
                &parse(artifact.bytes, &mut b)?,
                e,
                &orig,
                &assigned,
                first,
                &mut b,
            )?;
        }
    } else if bundle.slots[6].is_some() || bundle.committed_witness.is_some() {
        return Err(Error::Claim);
    }
    Ok(Verified {
        original_input_verified: true,
        reported_basis_linear_transitions_verified: linear_verified,
        group_rows_verified: rows.count,
        first_witness_verified: local,
        witness_capture_committed: durable,
        independent_group_domain_complete: complete,
        solver_completed_verified: solver_completed,
        native_backend_invoked_reported: n.invoked,
        native_statistics_complete_reported: n.statistics_complete,
        negative_verified,
        independent_derivation_verified: false,
        operation_trajectory_verified: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};
    use std::time::Duration;
    const DOMAIN: [Point; 2] = [Point { x: 6, y: 18 }, Point { x: 6, y: 83 }];
    fn expected() -> Expected<'static> {
        Expected {
            p: 101,
            a: 0,
            b: 7,
            m: 4,
            target: Point { x: 84, y: 12 },
            domain: &DOMAIN,
            codes: &[6],
            work_limit: 16384,
            trace_bytes: 128 * 1024 * 1024,
            memory_bytes: 256 * 1024 * 1024,
        }
    }
    fn limits() -> ReplayLimits {
        ReplayLimits {
            work: 10_000_000_000,
            allocation_bytes: 128 * 1024 * 1024,
            deadline: Instant::now() + Duration::from_secs(20),
        }
    }
    fn budget() -> Budget {
        Budget {
            limits: limits(),
            work: 0,
            bytes: 0,
        }
    }
    fn small_native() -> Native {
        Native {
            invoked: false,
            statistics_complete: false,
            basis: Vec::new(),
            retained: true,
            unit: false,
            work: 0,
            bytes: 0,
            events: 0,
            supplied_work: 10000,
            stop_index: 3,
        }
    }
    fn bytes(v: &Value) -> Vec<u8> {
        serde_json::to_vec(v).unwrap()
    }
    fn parsed<'a>(data: &'a [u8], b: &mut Budget) -> J<'a> {
        parse(data, b).unwrap()
    }
    fn input_fixture(e: &Expected<'_>) -> Value {
        let v = 2 * e.m - 2;
        let polys = original(e, &mut budget()).unwrap();
        json!({
 "schema":"ml-dataset.prime-f4-chain-input/v1","strategy":"prime-affine-s3-chain-f4-visitor/v1","m":e.m,"n_vars":v,"modulus":e.p.to_string(),"curve":{"a":e.a.to_string(),"b":e.b.to_string()},"target":{"x":e.target.x.to_string(),"y":e.target.y.to_string()},"target_x":e.target.x.to_string(),"ring":"Fp[x0,...,x(m-1),u0,...,u(m-3)]",
 "variables":(0..v).map(|i|if i<e.m{format!("x{i}")}else{format!("u{}",i-e.m)}).collect::<Vec<_>>(),
 "variable_roles":(0..v).map(|i|json!({"index":i,"name":if i<e.m{format!("x{i}")}else{format!("u{}",i-e.m)},"role":if i<e.m{"factor_base_leaf"}else{"unrestricted_affine_auxiliary"},"domain":if i<e.m{"factor_base_x"}else{"all_Fp"}})).collect::<Vec<_>>(),"monomial_order":"degrevlex","domains":(0..v).map(|i|if i<e.m{json!(e.codes)}else{Value::Null}).collect::<Vec<_>>(),"factor_base_x":e.codes.iter().map(|x|x.to_string()).collect::<Vec<_>>(),"signed_factor_base":e.domain.iter().map(|q|json!({"x":q.x.to_string(),"y":q.y.to_string()})).collect::<Vec<_>>(),"polynomials":polys.iter().map(|p|p.iter().map(|t|json!({"exponents":t.e[..v],"coefficient":t.c.to_string()})).collect::<Vec<_>>()).collect::<Vec<_>>(),"chain_equation_count":e.m-1,"membership_equation_count":e.m,"membership_variables":(0..e.m).collect::<Vec<_>>(),"native_normalization_applied":false,"coefficient_arithmetic":"u128 modular intermediates; merged nonzero terms, leading-first, no monic rescaling","native_modulus_limit_exclusive":"4294967296","affine_chain_exceptional_infinity_branches_covered":false,"negative_scope":"encoding-relative only; no exhaustive PDP nonexistence claim"})
    }
    #[test]
    fn complete_native_metadata_requires_reported_invocation() {
        let e = expected();
        let mut value = native_fixture();
        value["metadata"]["stop"] = json!("complete");
        value["metadata"]["native_statistics_complete"] = json!(true);
        let raw = bytes(&value);
        assert!(native(&parsed(&raw, &mut budget()), &e, &mut budget()).is_err());
        value["metadata"]["native_backend_invoked"] = json!(true);
        let raw = bytes(&value);
        let result = native(&parsed(&raw, &mut budget()), &e, &mut budget()).unwrap();
        assert!(result.invoked && result.statistics_complete);
    }
    fn native_fixture() -> Value {
        json!({"metadata":{"schema":"crypto.ml-dataset.wide-prime-dynamic-native/v1","m":4,"variables":6,"stop":"work_limit","supplied":{"work":10000,"bytes":10000},"consumed":{"work":0,"bytes":0},"limits":{"work":10000,"bytes":10000,"rows":8192,"columns":8192,"basis":256,"pairs":32768,"steps":64,"events":4096,"max_pair_degree":12},"acknowledged_events":0,"native_backend_invoked":false,"native_statistics_complete":false,"matrix_rows_reduced":0,"pivot_updates":0,"pairs_pending":0,"pairs_selected_unfinished":0,"maximum_rows_admitted":0,"maximum_columns_admitted":0,"maximum_pending_pairs":0,"completed_steps":0,"maximum_input_degree":4,"maximum_processed_row_degree":0,"basis_retained":true,"basis_retention_stop":null,"native_basis_derivation_verified":false,"independent_derivation_verified":false,"linear_extraction_complete":false,"group_domain_complete":false,"pdp_witness_verified":false,"exhaustive_no_decomposition":false,"solver_completed":false,"exact_instruction_count_verified":false,"full_execution_trace_verified":false,"fall_degree_verified":false},"basis":[]})
    }
    fn bundle(raw: &[Vec<u8>; 8]) -> Bundle<'_> {
        Bundle {
            slots: std::array::from_fn(|i| {
                if i < 4 || !raw[i].is_empty() {
                    Some(Artifact {
                        path: PATHS[i],
                        role: ROLES[i],
                        encoding: if i == 3 || i == 5 { "jsonl" } else { "json" },
                        bytes: &raw[i],
                        sha256: Sha256::digest(&raw[i]).into(),
                    })
                } else {
                    None
                }
            }),
            committed_witness: None,
        }
    }
    fn prefix_fixture() -> [Vec<u8>; 8] {
        let e = expected();
        let mut raw = std::array::from_fn(|_| Vec::new());
        raw[0] = bytes(&input_fixture(&e));
        raw[1] = bytes(
            &json!({"schema":"crypto.ml-dataset.wide-prime-dynamic-setup/v1","modulus":"101","prime_verified":true,"reserved_work":131072,"reserved_bytes":4096,"acknowledged_events":2}),
        );
        raw[2] = bytes(&native_fixture());
        for (sequence, stage) in [(0, "admitted"), (1, "complete")] {
            raw[3].extend(bytes(&json!({"schema":"crypto.ml-dataset.wide-prime-dynamic-event/v1","source":"setup","global_sequence":sequence,"local_sequence":sequence,"stage":stage,"reserved_work":131072,"reserved_bytes":4096,"persistence":"local_only"})));
            raw[3].push(b'\n');
        }
        raw
    }
    fn positive_row() -> Value {
        json!({"schema":"crypto.ml-dataset.wide-prime-dynamic-group-row/v1","origin":"independent_group_prefix","cursor":"0","indices":[0,0,0,0],"prefixes":[{"kind":"affine","x":98,"y":9},{"kind":"affine","x":99,"y":91},{"kind":"affine","x":84,"y":12}],"completed_additions":3,"exceptional_auxiliary_positions":[],"coordinates":[6,6,6,6,98,99],"residues":[0,0,0,0,0,0,0],"total":{"kind":"affine","x":84,"y":12},"computed_match":true,"candidate_coordinates_match":true,"complete":true,"tuples_reserved":1,"additions_reserved":3,"additions_performed":3,"checking_reserved_work":20000})
    }
    #[test]
    fn dynamic_replay_strict_json_duplicate_unknown_numeric_and_depth_controls() {
        for bad in [
            br#"{"a":1,"a":2}"#.as_slice(),
            b"[01]",
            b"[-1]",
            b"[1.0]",
            b"[1e2]",
            b"[true,]",
            br#"{"\u0061":1}"#,
            b"[18446744073709551616]",
        ] {
            assert!(parse(bad, &mut budget()).is_err(), "{bad:?}")
        }
        let mut b = budget();
        let j = parsed(br#"{"a":1,"extra":2}"#, &mut b);
        assert_eq!(j.obj(&["a"]), Err(Error::Shape));
        let deep = format!("{}0{}", "[".repeat(18), "]".repeat(18));
        assert!(parse(deep.as_bytes(), &mut budget()).is_err());
    }
    #[test]
    fn dynamic_replay_decimal_u128_exact_max_and_noncanonical_controls() {
        for x in [0, 1, u64::MAX as u128 + 1, 1u128 << 90, u128::MAX] {
            assert_eq!(decimal(&x.to_string()), Ok(x))
        }
        for x in [
            "",
            "00",
            "01",
            "+1",
            "-1",
            " 1",
            "1 ",
            "1.0",
            "340282366920938463463374607431768211456",
        ] {
            assert_eq!(decimal(x), Err(Error::Shape))
        }
    }
    #[test]
    fn dynamic_replay_parser_exact_work_allocation_and_deadline() {
        let data = br#"{"x":[1,2,3]}"#;
        let mut b = budget();
        parse(data, &mut b).unwrap();
        let used = b.work;
        let allocated = b.bytes;
        let mut exact = budget();
        exact.limits.work = used;
        exact.limits.allocation_bytes = allocated;
        parse(data, &mut exact).unwrap();
        let mut short = budget();
        short.limits.work = used - 1;
        assert!(matches!(parse(data, &mut short), Err(Error::Work)));
        let mut short = budget();
        short.limits.allocation_bytes = allocated - 1;
        assert!(matches!(parse(data, &mut short), Err(Error::Storage)));
        let mut late = budget();
        late.limits.deadline = Instant::now();
        assert!(matches!(parse(data, &mut late), Err(Error::Deadline)));
    }
    #[test]
    fn dynamic_replay_independent_group_known_prefix_and_infinity() {
        let e = expected();
        let mut b = budget();
        let p = Some(DOMAIN[0]);
        assert_eq!(sum(p, p, &e, &mut b).unwrap(), Some(Point { x: 98, y: 9 }));
        let three = sum(Some(Point { x: 98, y: 9 }), p, &e, &mut b).unwrap();
        assert_eq!(three, Some(Point { x: 99, y: 91 }));
        assert_eq!(sum(three, p, &e, &mut b).unwrap(), Some(e.target));
        assert_eq!(sum(p, Some(DOMAIN[1]), &e, &mut b).unwrap(), None);
        assert_eq!(sum(None, p, &e, &mut b).unwrap(), p);
        assert!(prime((1u64 << 61) - 1, &mut b).unwrap());
        assert!(!prime(341, &mut b).unwrap());
    }
    #[test]
    fn dynamic_replay_original_polynomial_differential_all_arities() {
        for m in 4..=15 {
            for (p, a, c, target) in [(101, 0, 7, 84), ((1u64 << 61) - 1, 9, 31, 123)] {
                let mut e = expected();
                e.m = m;
                e.p = p;
                e.a = a;
                e.b = c;
                e.target.x = target;
                let got = original(&e, &mut budget()).unwrap();
                let legacy =
                    super::super::prime_chain_polynomials(m, p, a, c, target, &[6]).unwrap();
                assert_eq!(got.len(), legacy.len());
                for (x, y) in got.iter().zip(&legacy) {
                    assert_eq!(x.len(), y.len());
                    for (t, (powers, coefficient)) in x.iter().zip(y) {
                        assert_eq!(t.c, *coefficient);
                        assert_eq!(
                            t.e[..2 * m - 2]
                                .iter()
                                .map(|x| *x as u32)
                                .collect::<Vec<_>>(),
                            *powers
                        );
                    }
                }
            }
        }
    }
    #[test]
    fn dynamic_replay_rehashed_original_input_and_reserved_role_tampering() {
        let e = expected();
        let raw = prefix_fixture();
        let ok = verify(&e, &bundle(&raw), limits()).unwrap();
        assert!(ok.original_input_verified);
        assert!(!ok.independent_derivation_verified);
        for mutation in 0..4 {
            let mut data: Value = serde_json::from_slice(&raw[0]).unwrap();
            match mutation {
                0 => data["target"]["x"] = json!("85"),
                1 => data["polynomials"][0][0]["coefficient"] = json!("2"),
                2 => data["domains"][4] = json!([6]),
                _ => data["unknown"] = json!(true),
            }
            let mut changed = raw.clone();
            changed[0] = bytes(&data);
            assert!(verify(&e, &bundle(&changed), limits()).is_err())
        }
        let mut moved = bundle(&raw);
        moved.slots[0].as_mut().unwrap().role = "unrelated";
        assert!(matches!(verify(&e, &moved, limits()), Err(Error::Identity)));
        let mut wrong = bundle(&raw);
        wrong.slots[3].as_mut().unwrap().encoding = "json";
        assert!(verify(&e, &wrong, limits()).is_err());
    }
    #[test]
    fn dynamic_replay_group_row_math_cursor_residue_and_point_schema_tampering() {
        let e = expected();
        let orig = original(&e, &mut budget()).unwrap();
        let good = positive_row();
        let data = bytes(&good);
        let mut b = budget();
        assert!(
            row(
                &parsed(&data, &mut b),
                &e,
                &orig,
                &[None; 28],
                &small_native(),
                &mut b
            )
            .unwrap()
            .matched
        );
        for i in 0..5 {
            let mut bad = good.clone();
            match i {
                0 => bad["prefixes"][1]["y"] = json!(90),
                1 => bad["cursor"] = json!("1"),
                2 => bad["residues"][0] = json!(1),
                3 => bad["prefixes"][0]["x"] = json!("98"),
                _ => bad["completed_additions"] = json!(2),
            }
            let data = bytes(&bad);
            let mut b = budget();
            assert!(row(
                &parsed(&data, &mut b),
                &e,
                &orig,
                &[None; 28],
                &small_native(),
                &mut b
            )
            .is_err());
        }
    }
    #[test]
    fn dynamic_replay_exceptional_prefix_group_witness_does_not_claim_affine_root() {
        let mut e = expected();
        e.target = Point { x: 98, y: 9 };
        let orig = original(&e, &mut budget()).unwrap();
        let data = bytes(
            &json!({"schema":"crypto.ml-dataset.wide-prime-dynamic-group-row/v1","origin":"independent_group_prefix","cursor":"4","indices":[0,1,0,0],"prefixes":[{"kind":"infinity"},{"kind":"affine","x":6,"y":18},{"kind":"affine","x":98,"y":9}],"completed_additions":3,"exceptional_auxiliary_positions":[0],"coordinates":null,"residues":null,"total":{"kind":"affine","x":98,"y":9},"computed_match":true,"candidate_coordinates_match":false,"complete":true,"tuples_reserved":5,"additions_reserved":15,"additions_performed":15,"checking_reserved_work":100000}),
        );
        let mut n = small_native();
        n.unit = true;
        let mut b = budget();
        let f = row(&parsed(&data, &mut b), &e, &orig, &[None; 28], &n, &mut b).unwrap();
        assert!(f.matched);
        assert!(!f.has_coordinates);
        assert_eq!(f.exception_count, 1);
    }
    #[test]
    fn dynamic_replay_jsonl_duplicate_cursor_and_unterminated_tail_reject() {
        let e = expected();
        let orig = original(&e, &mut budget()).unwrap();
        let mut raw = bytes(&positive_row());
        assert!(groups(
            &raw,
            &e,
            &orig,
            &[None; 28],
            &small_native(),
            false,
            &mut budget()
        )
        .is_err());
        raw.push(b'\n');
        assert_eq!(
            groups(
                &raw,
                &e,
                &orig,
                &[None; 28],
                &small_native(),
                false,
                &mut budget()
            )
            .unwrap()
            .count,
            1
        );
        raw.extend(raw.clone());
        assert!(groups(
            &raw,
            &e,
            &orig,
            &[None; 28],
            &small_native(),
            false,
            &mut budget()
        )
        .is_err());
    }
    #[test]
    fn dynamic_replay_receipt_samebytes_original_sequence_and_legacy_resolution() {
        let raw = b"fixed bounded witness";
        let digest = Sha256::digest(raw).into();
        let a = Artifact {
            path: PATHS[6],
            role: ROLES[6],
            encoding: "json",
            bytes: raw,
            sha256: digest,
        };
        let c = CapturedWitness {
            bytes: raw,
            sha256: digest,
            original_sequence: 7,
        };
        let hexadecimal = digest
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>();
        let mut value = json!({"commit":"new_commit","payload_sha256":hexadecimal,"payload_bytes":raw.len(),"sequence":7,"provenance":"dedicated"});
        let data = bytes(&value);
        assert_eq!(receipt(&parsed(&data, &mut budget()), &c, &a), Ok(true));
        value["sequence"] = json!(8);
        assert!(receipt(&parsed(&bytes(&value), &mut budget()), &c, &a).is_err());
        value["commit"] = json!("already_committed");
        value["provenance"] = json!("legacy_adopted");
        value["sequence"] = Value::Null;
        assert_eq!(
            receipt(&parsed(&bytes(&value), &mut budget()), &c, &a),
            Ok(false)
        );
        let altered = CapturedWitness {
            bytes: b"different",
            sha256: digest,
            original_sequence: 7,
        };
        assert!(receipt(&parsed(&bytes(&value), &mut budget()), &altered, &a).is_err());
    }
    #[test]
    fn dynamic_replay_linear_nonmonic_lowest_pivot_and_forged_basis_boundary() {
        let e = expected();
        let orig = original(&e, &mut budget()).unwrap();
        let values = [6, 6, 6, 6, 98, 99];
        let mut n = small_native();
        for (i, &x) in values.iter().enumerate() {
            let mut exponent = [0; 28];
            exponent[i] = 1;
            n.basis.push(vec![
                Term { e: exponent, c: 3 },
                Term {
                    e: [0; 28],
                    c: sub(0, mul(3, x, 101), 101),
                },
            ]);
        }
        let good = json!({"schema":"crypto.ml-dataset.wide-prime-dynamic-linear/v1","report":{"stop":"reported_basis_candidate","assignments":6,"completed_passes":7,"original_equations_checked":7,"candidate_verified_original":true,"algebra_reserved_work":600,"checking_reserved_work":60,"independent_derivation_verified":false,"pdp_witness_verified":false},"assignments":values,"source_ids":[0,1,2,3,4,5],"transitions":values.iter().enumerate().map(|(i,&x)|json!({"variable":i,"basis_id":i,"a":3,"b":sub(0,mul(3,x,101),101),"value":x,"algebra_reserved_work":100*(i+1),"checking_reserved_work":10*(i+1)})).collect::<Vec<_>>(),"unresolved":[null,null,null,null,null,null],"residues":[0,0,0,0,0,0,0]});
        let data = bytes(&good);
        let mut b = budget();
        let assignment = linear(&parsed(&data, &mut b), &e, &n, &orig, &mut b).unwrap();
        assert_eq!(&assignment.assignments[..6], &values.map(Some));
        // A consistent forged reported basis is deliberately accepted ONLY as supplied-
        // basis consistency. The public result always fixes derivation assurance false.
        for i in 0..4 {
            let mut bad = good.clone();
            match i {
                0 => bad["transitions"][0]["basis_id"] = json!(1),
                1 => bad["transitions"][0]["a"] = json!(4),
                2 => bad["assignments"][5] = json!(98),
                _ => bad["report"]["independent_derivation_verified"] = json!(true),
            }
            let data = bytes(&bad);
            let mut b = budget();
            assert!(linear(&parsed(&data, &mut b), &e, &n, &orig, &mut b).is_err());
        }
    }
    fn terminal_fixture(raw: &[Vec<u8>; 8]) -> Value {
        let w = 16_777_216u64;
        json!({"schema":"restricted-wide-prime-dynamic-result/v1","strategy":"prime_f4_wide_dynamic","stage":"native","disposition":"censored","cause":"native_stop","native_stop":"work_limit","linear_stop":null,"group_stop":null,"search_phase":"not_started","accounting":{"counter_snapshot_stage":"before_terminal_retention","pools":[{"name":"algebra","parent":"root","work_limit":w*3/4,"reserved_work":262144,"performed_work":0},{"name":"checking","parent":"root","work_limit":w/4,"reserved_work":0,"performed_work":0},{"name":"native","parent":"algebra","work_limit":10000,"reserved_work":0,"performed_work":null},{"name":"observer","parent":"algebra","work_limit":0,"reserved_work":0,"performed_work":0},{"name":"capture","parent":"checking","work_limit":0,"reserved_work":0,"performed_work":0},{"name":"terminal","parent":"algebra","work_limit":262144,"reserved_work":262144,"performed_work":0}],"requested_bytes":{"limit":67108864,"reserved":10000},"group":{"addition_limit":16384,"additions_reserved":0,"additions_performed":0,"tuple_limit":1024,"tuples_reserved":0},"traversal":{"native_lifts_visited":"0","independent_visited":"0","independent_total":"0"},"events":{"limit":4096,"global_acknowledged":2,"setup":2,"native":0,"linear":0,"group":0,"new_payload":0,"attempted":2,"pending":0}},"source_slots":(0..7).map(|i|json!({"index":i,"state":if i<4{"local_complete"}else{"not_produced"},"bytes":if i<4{json!(raw[i].len())}else{Value::Null},"digest_binding":"outer_manifest","sha256":null})).collect::<Vec<_>>(),"last_state":null,"first_certificate":null,"claims":{"solver_completed":false,"group_domain_complete":false,"negative_verified":false,"pdp_witness_verified":false,"certificate_durably_persisted":false,"independent_derivation_verified":false,"operation_trajectory_verified":false}})
    }
    #[test]
    fn dynamic_replay_rehashed_terminal_outer_binding_unavailable_performed_and_false_completeness()
    {
        let e = expected();
        let mut raw = prefix_fixture();
        let good = terminal_fixture(&raw);
        raw[7] = bytes(&good);
        let ok = verify(&e, &bundle(&raw), limits()).unwrap();
        assert!(!ok.independent_group_domain_complete);
        for i in 0..6 {
            let mut bad = good.clone();
            match i {
                0 => bad["source_slots"][0]["bytes"] = json!(1),
                1 => bad["source_slots"][0]["sha256"] = json!("00".repeat(32)),
                2 => bad["accounting"]["pools"][2]["performed_work"] = json!(0),
                3 => bad["claims"]["negative_verified"] = json!(true),
                4 => bad["accounting"]["events"]["new_payload"] = json!(1),
                _ => bad["claims"]["operation_trajectory_verified"] = json!(true),
            }
            raw[7] = bytes(&bad);
            assert!(verify(&e, &bundle(&raw), limits()).is_err());
        }
    }
    #[test]
    fn dynamic_replay_terminal_snapshot_absent_row_and_disagreement() {
        let e = expected();
        let orig = original(&e, &mut budget()).unwrap();
        let row_bytes = bytes(&positive_row());
        let mut data = row_bytes.clone();
        data.push(b'\n');
        let mut rows = groups(
            &data,
            &e,
            &orig,
            &[None; 28],
            &small_native(),
            false,
            &mut budget(),
        )
        .unwrap();
        let good = json!({"phase":"independent_prefix","origin":"independent_group_prefix","cursor":"0","tuple_indices":[0,0,0,0],"progress":"complete_tuple","tuple_entered":true,"completed_prefixes":3,"prefix_points":[{"x":"98","y":"9"},{"x":"99","y":"91"},{"x":"84","y":"12"}],"exceptional_auxiliary_positions":[],"coordinate_count":6,"coordinates":[6,6,6,6,98,99],"residue_count":7,"residues":[0,0,0,0,0,0,0],"computed_match":true,"row":{"row_index":0,"offset":0,"bytes":data.len()},"metadata_acknowledged":true});
        let mut b = budget();
        snapshot(
            &parsed(&bytes(&good), &mut b),
            &e,
            &orig,
            &[None; 28],
            &small_native(),
            &mut rows,
            &mut b,
        )
        .unwrap();
        assert_eq!(rows.independent, 1);
        let mut bad = good.clone();
        bad["prefix_points"][1]["x"] = json!("98");
        let mut b = budget();
        assert!(snapshot(
            &parsed(&bytes(&bad), &mut b),
            &e,
            &orig,
            &[None; 28],
            &small_native(),
            &mut rows,
            &mut b
        )
        .is_err());
        let mut absent = good;
        absent["row"] = Value::Null;
        absent["metadata_acknowledged"] = json!(false);
        let mut empty = groups(
            b"",
            &e,
            &orig,
            &[None; 28],
            &small_native(),
            false,
            &mut budget(),
        )
        .unwrap();
        let mut b = budget();
        snapshot(
            &parsed(&bytes(&absent), &mut b),
            &e,
            &orig,
            &[None; 28],
            &small_native(),
            &mut empty,
            &mut b,
        )
        .unwrap();
        assert_eq!(empty.count, 0);
        assert_eq!(empty.independent, 1);
        assert!(empty.first.is_some());
    }
    #[test]
    fn dynamic_replay_v2_eligible_native_stage_cannot_be_omitted_but_partial_assignments_can() {
        let e = expected();
        let orig = original(&e, &mut budget()).unwrap();
        let mut assigned = [None; 28];
        for (i, x) in [6, 6, 6, 6, 98, 99].into_iter().enumerate() {
            assigned[i] = Some(x)
        }
        let mut line = bytes(&positive_row());
        line.push(b'\n');
        assert!(groups(
            &line,
            &e,
            &orig,
            &assigned,
            &small_native(),
            true,
            &mut budget()
        )
        .is_err());
        assert!(groups(
            &line,
            &e,
            &orig,
            &assigned,
            &small_native(),
            false,
            &mut budget()
        )
        .is_ok());
        let mut native = positive_row();
        native["origin"] = json!("reported_basis_linear_candidate");
        let mut line = bytes(&native);
        line.push(b'\n');
        assert!(groups(
            &line,
            &e,
            &orig,
            &assigned,
            &small_native(),
            false,
            &mut budget()
        )
        .is_err());
        assert_eq!(
            groups(
                &line,
                &e,
                &orig,
                &assigned,
                &small_native(),
                true,
                &mut budget()
            )
            .unwrap()
            .native,
            1
        );
    }
    #[test]
    fn dynamic_replay_v2_missing_row_provenance_is_recomputed() {
        let e = expected();
        let orig = original(&e, &mut budget()).unwrap();
        let mut assigned = [None; 28];
        for (i, x) in [6, 6, 6, 6, 98, 99].into_iter().enumerate() {
            assigned[i] = Some(x)
        }
        let good = json!({"phase":"native_lifts","origin":"reported_basis_linear_candidate","cursor":"0","tuple_indices":[0,0,0,0],"progress":"complete_tuple","tuple_entered":true,"completed_prefixes":3,"prefix_points":[{"x":"98","y":"9"},{"x":"99","y":"91"},{"x":"84","y":"12"}],"exceptional_auxiliary_positions":[],"coordinate_count":6,"coordinates":[6,6,6,6,98,99],"residue_count":0,"residues":[],"computed_match":true,"row":null,"metadata_acknowledged":false});
        let mut rows = groups(
            b"",
            &e,
            &orig,
            &assigned,
            &small_native(),
            true,
            &mut budget(),
        )
        .unwrap();
        let mut b = budget();
        snapshot(
            &parsed(&bytes(&good), &mut b),
            &e,
            &orig,
            &assigned,
            &small_native(),
            &mut rows,
            &mut b,
        )
        .unwrap();
        assert!(rows.first.unwrap().coordinate_match);
        let mut bad = good;
        bad["origin"] = json!("reported_basis_leaf_lift");
        let mut rows = groups(
            b"",
            &e,
            &orig,
            &assigned,
            &small_native(),
            true,
            &mut budget(),
        )
        .unwrap();
        let mut b = budget();
        assert!(snapshot(
            &parsed(&bytes(&bad), &mut b),
            &e,
            &orig,
            &assigned,
            &small_native(),
            &mut rows,
            &mut b
        )
        .is_err());
    }
    #[test]
    fn dynamic_replay_v2_unit_contradiction_survives_stripped_residues_and_addition_minima() {
        let e = expected();
        let orig = original(&e, &mut budget()).unwrap();
        let mut bad = positive_row();
        bad["residues"] = Value::Null;
        let mut unit = small_native();
        unit.unit = true;
        let mut b = budget();
        assert!(row(
            &parsed(&bytes(&bad), &mut b),
            &e,
            &orig,
            &[None; 28],
            &unit,
            &mut b
        )
        .is_err());
        let mut bad = positive_row();
        bad["additions_reserved"] = json!(0);
        bad["additions_performed"] = json!(0);
        let mut data = bytes(&bad);
        data.push(b'\n');
        assert!(groups(
            &data,
            &e,
            &orig,
            &[None; 28],
            &small_native(),
            false,
            &mut budget()
        )
        .is_err());
    }
    #[test]
    fn dynamic_replay_v2_setup_receipt_types_and_exact_admission() {
        let e = expected();
        let raw = prefix_fixture();
        verify(&e, &bundle(&raw), limits()).unwrap();
        for (key, value) in [
            ("reserved_work", json!("131072")),
            ("reserved_work", json!(131071)),
            ("reserved_bytes", json!(4095)),
            ("acknowledged_events", json!(1)),
        ] {
            let mut changed = raw.clone();
            let mut setup: Value = serde_json::from_slice(&raw[1]).unwrap();
            setup[key] = value;
            changed[1] = bytes(&setup);
            assert!(verify(&e, &bundle(&changed), limits()).is_err());
        }
        let mut changed = raw;
        let event = json!({"schema":"crypto.ml-dataset.wide-prime-dynamic-event/v1","source":"setup","global_sequence":0,"local_sequence":0,"stage":"complete","reserved_work":131072,"reserved_bytes":4096,"persistence":"local_only"});
        changed[3] = bytes(&event);
        changed[3].push(b'\n');
        assert!(verify(&e, &bundle(&changed), limits()).is_err());
    }
}
