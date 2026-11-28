//! Target derivation. A target is a pure function of
//! `(SCHEME, nonce, p, index, kind)`: every draw comes from a SHA-256
//! counter stream keyed by those values and a role name, so curve, base point
//! and target point use independent streams. `kind = public` hashes `Q` to the
//! curve and clears the cofactor, so no party knows `log_G Q`
//! (SUCCESS_CRITERIA §1 "public target"); `kind = planted` (controls only)
//! sets `Q = [d]G` and returns `d` separately for the owner file.

use crate::curve::{cover_admits, embedding_degree, order_4_prime, Curve, Pt};
use crate::field::{Tower, E2, E6};
use crate::sha256::{hex, sha256, Stream};

pub const SCHEME: &str = "fresh-scaling/v1|family=jv-weak-cover-fp6";
/// Embedding degrees up to this bound are searched and refused.
pub const EMBEDDING_KMAX: u32 = 1000;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Public,
    Planted,
}
impl Kind {
    pub fn name(&self) -> &'static str {
        match self {
            Kind::Public => "public",
            Kind::Planted => "planted",
        }
    }
    pub fn parse(s: &str) -> Option<Kind> {
        match s {
            "public" => Some(Kind::Public),
            "planted" => Some(Kind::Planted),
            _ => None,
        }
    }
}

pub fn stream_key(nonce: &str, p: u64, index: u64, kind: Kind, role: &str) -> String {
    format!("{SCHEME}|nonce={nonce}|p={p}|index={index}|kind={}|role={role}", kind.name())
}

pub fn draw_e6(t: &Tower, s: &mut Stream) -> E6 {
    E6(core::array::from_fn(|_| E2([s.below(t.p), s.below(t.p)])))
}

pub fn draw_nonsquare(t: &Tower, s: &mut Stream) -> E6 {
    loop {
        let z = draw_e6(t, s);
        if z != E6::ZERO && !t.is_square(&z) {
            return z;
        }
    }
}

/// Hash to an affine point with `y != 0`: uniform `x`, deterministic root,
/// sign from the stream.
pub fn draw_point(c: &Curve, s: &mut Stream, z: &E6) -> Pt {
    let t = c.t;
    loop {
        let x = draw_e6(t, s);
        let r = c.rhs(&x);
        if r == E6::ZERO {
            continue;
        }
        if let Some(y) = t.sqrt_with(&r, z) {
            let ny = t.neg(&y);
            let lo = if y < ny { y } else { ny };
            let y = if s.bit() { t.neg(&lo) } else { lo };
            return Pt { x, y, inf: false };
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Rejections {
    pub alpha_in_fq: u64,
    pub cover_refused: u64,
    pub singular: u64,
    pub j_in_proper_subfield: u64,
    pub order_not_4_prime: u64,
    pub small_embedding_degree: u64,
}

pub struct Target {
    pub p: u64,
    pub index: u64,
    pub kind: Kind,
    pub alpha: E6,
    pub g: Pt,
    pub q: Pt,
    pub l: u64,
    pub order: u128,
    pub planted_scalar: Option<u64>,
    pub attempts: u64,
    pub rej: Rejections,
    pub stream_words: u64,
}

pub fn generate(nonce: &str, p: u64, index: u64, kind: Kind) -> Target {
    let t = Tower::new(p);
    let mut cs = Stream::new(&stream_key(nonce, p, index, kind, "curve"));
    let z = draw_nonsquare(&t, &mut cs);
    let mut rej = Rejections::default();
    let mut attempts = 0u64;
    let (alpha, order, l) = loop {
        attempts += 1;
        let alpha = draw_e6(&t, &mut cs);
        if alpha.in_fq() {
            rej.alpha_in_fq += 1;
            continue;
        }
        if !cover_admits(&t, &alpha) {
            rej.cover_refused += 1;
            continue;
        }
        let c = Curve::new(&t, alpha);
        let Some(j) = c.j_invariant() else {
            rej.singular += 1;
            continue;
        };
        if t.in_subfield(&j, 2) || t.in_subfield(&j, 3) {
            rej.j_in_proper_subfield += 1;
            continue;
        }
        let p0 = draw_point(&c, &mut cs, &z);
        let checks = [draw_point(&c, &mut cs, &z), draw_point(&c, &mut cs, &z)];
        let Some((order, l)) = order_4_prime(&c, &p0, &checks) else {
            rej.order_not_4_prime += 1;
            continue;
        };
        if embedding_degree(p, l, EMBEDDING_KMAX).is_some() {
            rej.small_embedding_degree += 1;
            continue;
        }
        break (alpha, order, l);
    };
    let c = Curve::new(&t, alpha);
    let mut gs = Stream::new(&stream_key(nonce, p, index, kind, "generator"));
    let g = loop {
        let g = c.mul(&draw_point(&c, &mut gs, &z), 4);
        if !g.inf && c.mul(&g, l as u128).inf {
            break g;
        }
    };
    let mut planted_scalar = None;
    let mut ts = Stream::new(&stream_key(nonce, p, index, kind, "target"));
    let q = match kind {
        Kind::Public => loop {
            let q = c.mul(&draw_point(&c, &mut ts, &z), 4);
            if !q.inf && c.mul(&q, l as u128).inf && q != g && q != c.neg(&g) {
                break q;
            }
        },
        Kind::Planted => {
            let d = 2 + ts.below(l - 3);
            planted_scalar = Some(d);
            c.mul(&g, d as u128)
        }
    };
    Target {
        p,
        index,
        kind,
        alpha,
        g,
        q,
        l,
        order,
        planted_scalar,
        attempts,
        rej,
        stream_words: cs.words + gs.words + ts.words,
    }
}

fn e2j(e: &E2) -> String {
    format!("[{},{}]", e.0[0], e.0[1])
}
fn e6j(e: &E6) -> String {
    format!("[{},{},{}]", e2j(&e.0[0]), e2j(&e.0[1]), e2j(&e.0[2]))
}
fn ptj(p: &Pt) -> String {
    format!("{{\"inf\":{},\"x\":{},\"y\":{}}}", p.inf, e6j(&p.x), e6j(&p.y))
}

/// The exact public-input encoding of `nagao-e2e/public.json` (compact,
/// keys sorted): what a worker receives. No answer field.
pub fn public_json(t: &Target) -> String {
    format!(
        "{{\"alpha\":{},\"g\":{},\"l\":{},\"p\":{},\"q\":{}}}",
        e6j(&t.alpha),
        ptj(&t.g),
        t.l,
        t.p,
        ptj(&t.q)
    )
}

pub fn manifest_json(t: &Target, nonce: &str, public: &str) -> String {
    let q6 = (t.p as u128).pow(6);
    let trace = (q6 + 1) as i128 - t.order as i128;
    let b = (t.l as f64).log2();
    let r = &t.rej;
    format!(
        concat!(
            "{{\"schema\":\"fresh-scaling.target-manifest/v1\",\"pcc\":\"unverified\",",
            "\"scheme\":\"{}\",\"nonce\":\"{}\",\"p\":{},\"index\":{},\"kind\":\"{}\",",
            "\"public_sha256\":\"{}\",\"l\":{},\"order\":{},\"cofactor\":4,\"frobenius_trace\":{},",
            "\"b_log2_l\":{:.4},\"field_bits_p6\":{:.4},\"embedding_degree_gt\":{},",
            "\"j_in_proper_subfield\":false,\"anomalous\":false,",
            "\"curve_attempts\":{},\"rejections\":{{\"alpha_in_fq\":{},\"cover_refused\":{},\"singular\":{},",
            "\"j_in_proper_subfield\":{},\"order_not_4_prime\":{},\"small_embedding_degree\":{}}},",
            "\"stream_words\":{},\"transfer_admission\":\"not checked here: decided by the frozen pipeline's public-input validation\",",
            "\"answer_known_to_generator\":{}}}"
        ),
        SCHEME,
        nonce,
        t.p,
        t.index,
        t.kind.name(),
        hex(&sha256(public.as_bytes())),
        t.l,
        t.order,
        trace,
        b,
        6.0 * (t.p as f64).log2(),
        EMBEDDING_KMAX,
        t.attempts,
        r.alpha_in_fq,
        r.cover_refused,
        r.singular,
        r.j_in_proper_subfield,
        r.order_not_4_prime,
        r.small_embedding_degree,
        t.stream_words,
        t.planted_scalar.is_some()
    )
}
