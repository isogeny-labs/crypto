//! `cell`: deterministic derivation of one panel cell (prime, prime-order curve,
//! Kummer base) and its JSON record `pkm.m8-panel.cell/v1`.
use crate::ec::{lift_small, Ec};
use crate::util::*;
use crypto_lib::cryptanalysis::ic_boundary::{GroupOps, PrimeCurve};
use crypto_lib::cryptanalysis::prime_fast::{point_order_hasse, FastCurve};
use crypto_lib::cryptanalysis::residual_walk::is_prime_u64;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};
use std::path::Path;

pub const RULE_TEXT: &str = "p runs over the primes >= 2^(T*R) (T*R >= 16) with 2^T | p-1, in increasing order; a prime is ELIGIBLE iff \
it is not a Fermat prime (p-1 not a power of two), a prime-order curve y^2=x^3+ax+b is found within 400 attempts, and a base with \
B_real >= ceil(B/2) exists under the g rule; the first eligible prime is used and every rejected prime is recorded. Curve search: StdRng \
(rand 0.8, ChaCha12) seeded by SHA-256('pkm.m8-panel/v1|curve|<slug>|<seed>|<p>'), a, b uniform in [1,p), skip singular, #E counted natively \
(exhaustive Legendre sum for p < 2^20, else Hasse-interval baby-step giant-step of a random point whose unique candidate is #E), first prime #E; \
r*Pv = O is checked for a random point. zeta = h^((p-1)>>T) for the least h >= 2 with zeta^(2^(T-1)) != 1. g rule: the least integer g in [2, 20000) \
outside mu_(2^T) (g^(2^T) != 1) with B_real >= ceil(2^T/2). V = { g*zeta^k }. Real base = { x in V : x^3+ax+b is a non-zero square }, positive \
sign = the point with y <= (p-1)/2. Generator P = point with the least x >= 1, y <= (p-1)/2. Exception: T=3, R=8 reuses the L1 decisive cell \
(p=16801793; a, b, g reconstructed by a native port of CPython random.Random(str) following gen.py, no eligibility rule applied; see l1_reuse).";

pub struct RealPt {
    pub v_index: usize,
    pub x: u64,
    pub y: u64,
}

pub struct Cell {
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub r: u64,
    pub t: usize,
    pub g: u64,
    pub zeta: u64,
    pub v: Vec<u64>,
    pub real: Vec<RealPt>,
    pub px: u64,
    pub py: u64,
    pub slug: String,
    pub sha256: String,
}

impl Cell {
    pub fn load(dir: &Path) -> Result<Cell, String> {
        let j = read_json(&dir.join("cell.json"))?;
        if !check_seal(&j) {
            return Err("cell.json seal mismatch".into());
        }
        let real = j["real_base"]
            .as_array()
            .ok_or("real_base")?
            .iter()
            .map(|e| {
                Ok(RealPt {
                    v_index: u64_of(e, "v_index")? as usize,
                    x: u64_of(e, "x")?,
                    y: u64_of(e, "y")?,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        Ok(Cell {
            p: u64_of(&j, "p")?,
            a: u64_of(&j, "a")?,
            b: u64_of(&j, "b")?,
            r: u64_of(&j, "r")?,
            t: u64_of(&j, "T")? as usize,
            g: u64_of(&j, "g")?,
            zeta: u64_of(&j, "zeta")?,
            v: u64_vec(&j["V"])?,
            real,
            px: u64_of(&j["generator"], "x")?,
            py: u64_of(&j["generator"], "y")?,
            slug: j["slug"].as_str().ok_or("slug")?.to_string(),
            sha256: j["sha256"].as_str().ok_or("sha256")?.to_string(),
        })
    }
    pub fn ec(&self) -> Ec {
        Ec::new(self.p, self.a, self.b)
    }
}

pub fn least_prime(t: usize, ratio: u32) -> Result<u64, String> {
    let bits = t as u32 * ratio;
    if !(16..=61).contains(&bits) || t == 0 || t > 12 {
        return Err(format!("T*R = {bits} bits out of range 16..=61 (every prime must be >= 2^16)"));
    }
    least_prime_from(t, 1u64 << bits)
}

/// Least prime `>= lo` with `2^t | p - 1`.
pub fn least_prime_from(t: usize, lo: u64) -> Result<u64, String> {
    let step = 1u64 << t;
    let mut k = (lo.max(2) - 1).div_ceil(step);
    loop {
        let p = k * step + 1;
        if p >= lo && is_prime_u64(p) {
            return Ok(p);
        }
        k += 1;
        if k > (1u64 << 62) / step {
            return Err("prime search overflow".into());
        }
    }
}

/// `#E(F_p)` when prime (`Some`), else `None`.
pub fn prime_order(p: u64, a: u64, b: u64, rng: &mut StdRng) -> Option<u64> {
    let n = if p < 1 << 20 {
        PrimeCurve { p, a, b }.point_count()
    } else {
        let fc = FastCurve::new("cell", p, a, b, 0);
        let pt = loop {
            let x = rng.gen_range(0..p);
            if let Some(pt) = fc.lift_x(x) {
                break pt;
            }
        };
        point_order_hasse(&fc, pt)?
    };
    is_prime_u64(n).then_some(n)
}

fn seeded(slug: &str, seed: u64, what: &str) -> StdRng {
    let h = sha256_hex(format!("pkm.m8-panel/v1|{what}|{slug}|{seed}").as_bytes());
    let mut s = [0u8; 32];
    s.copy_from_slice(&hex::decode(h).expect("hex"));
    StdRng::from_seed(s)
}
pub fn seeded_rng(slug: &str, seed: u64, what: &str) -> StdRng {
    seeded(slug, seed, what)
}

fn nonsingular(p: u64, a: u64, b: u64) -> bool {
    let d = addm(
        mulm(4, powm(a, 3, p), p),
        mulm(27, mulm(b, b, p), p),
        p,
    );
    d != 0
}

pub struct Derived {
    pub json: Value,
    pub cell: Cell,
}

const CURVE_ATTEMPTS: u64 = 400;
const G_BOUND: u64 = 20_000;
const PRIME_TRIES: usize = 64;
pub const L1_SEED: &str = "16801793-8-3-1-random";

pub fn is_fermat(p: u64) -> bool {
    (p - 1).is_power_of_two()
}

/// Real base of `V = g*mu` (abscissae on the curve, canonical positive sign).
fn base_of(p: u64, a: u64, b: u64, zeta: u64, t: usize, g: u64) -> (Vec<u64>, Vec<RealPt>) {
    let v: Vec<u64> = (0..1u64 << t).map(|k| mulm(g, powm(zeta, k, p), p)).collect();
    let real = v
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| lift_small(p, a, b, x).map(|(x, y)| RealPt { v_index: i, x, y }))
        .collect();
    (v, real)
}

fn root_of_unity(p: u64, t: usize) -> Option<u64> {
    (2..p)
        .map(|h| powm(h, (p - 1) >> t, p))
        .find(|&z| powm(z, 1u64 << (t - 1), p) != 1)
}

/// The L1 decisive cell's `a, b, g` by a native port of gen.py's draw order on
/// CPython's `random.Random(str)`; returns the generator positioned at gen.py's
/// target draw (so `x_R` is reconstructible too).
pub fn l1_reconstruct() -> Result<(u64, u64, u64, PyRandom), String> {
    let p = 16_801_793u64;
    let mut py = PyRandom::from_str_seed(L1_SEED);
    let mut rng = seeded("l1", 0, "l1-count");
    loop {
        let a = py.randrange(1, p);
        let b = py.randrange(1, p);
        if !nonsingular(p, a, b) {
            continue;
        }
        if prime_order(p, a, b, &mut rng).is_some() {
            let g = py.randrange(2, p);
            return Ok((a, b, g, py));
        }
    }
}

struct Pick {
    a: u64,
    b: u64,
    r: u64,
    g: u64,
    attempts: u64,
    v: Vec<u64>,
    real: Vec<RealPt>,
    zeta: u64,
}

/// Eligibility of one prime: not Fermat, a prime-order curve within the attempt
/// bound, and a `g` outside `mu_{2^t}` (g^(2^t) != 1) with `B_real >= B/2`.
fn eligible(p: u64, t: usize, slug: &str, seed: u64) -> Result<Pick, String> {
    if is_fermat(p) {
        return Err("Fermat prime (msolve 0.6.5 crash band)".into());
    }
    let mut rng = seeded(slug, seed, &format!("curve|{p}"));
    let mut found = None;
    for attempts in 1..=CURVE_ATTEMPTS {
        let a = rng.gen_range(1..p);
        let b = rng.gen_range(1..p);
        if !nonsingular(p, a, b) {
            continue;
        }
        if let Some(r) = prime_order(p, a, b, &mut rng) {
            found = Some((a, b, r, attempts));
            break;
        }
    }
    let (a, b, r, attempts) = found.ok_or_else(|| format!("no prime-order curve in {CURVE_ATTEMPTS} attempts"))?;
    let zeta = root_of_unity(p, t).ok_or("no root of unity")?;
    let need = (1usize << t).div_ceil(2);
    for g in 2..G_BOUND.min(p) {
        if powm(g, 1u64 << t, p) == 1 {
            continue;
        }
        let (v, real) = base_of(p, a, b, zeta, t, g);
        if real.len() >= need {
            return Ok(Pick { a, b, r, g, attempts, v, real, zeta });
        }
    }
    Err(format!("no g < {G_BOUND} outside mu_(2^t) with B_real >= {need}"))
}

pub fn derive(t: usize, ratio: u32, seed: u64, no_l1: bool) -> Result<Derived, String> {
    derive_from(t, ratio, seed, no_l1, None)
}

/// [`derive`] with an optional lower bound on the prime that replaces `2^(T*R)` (recorded as
/// `min_prime_override`; used to step over primes on which msolve crashed, per the protocol).
pub fn derive_from(t: usize, ratio: u32, seed: u64, no_l1: bool, min_prime: Option<u64>) -> Result<Derived, String> {
    let slug = format!("t{t}-r{ratio}-s{seed}");
    let l1 = t == 3 && ratio == 8 && !no_l1;
    let mut rejected: Vec<Value> = Vec::new();
    let (p, pick) = if l1 {
        let p = 16_801_793u64;
        let (a, b, g, _) = l1_reconstruct()?;
        let mut rng = seeded(&slug, seed, "curve");
        let r = prime_order(p, a, b, &mut rng).ok_or("L1 curve not prime order")?;
        let zeta = root_of_unity(p, t).ok_or("no root of unity")?;
        let (v, real) = base_of(p, a, b, zeta, t, g);
        (p, Pick { a, b, r, g, attempts: 1, v, real, zeta })
    } else {
        let mut p = match min_prime {
            Some(lo) => least_prime_from(t, lo)?,
            None => least_prime(t, ratio)?,
        };
        let mut tried = 0;
        loop {
            match eligible(p, t, &slug, seed) {
                Ok(pk) => break (p, pk),
                Err(why) => rejected.push(json!({"p": p, "reason": why})),
            }
            tried += 1;
            if tried >= PRIME_TRIES {
                return Err("no eligible prime found".into());
            }
            p = least_prime_from(t, p + 1)?;
        }
    };
    let Pick { a, b, r, g, attempts, v, real, zeta } = pick;
    let ec = Ec::new(p, a, b);
    let mut rng = seeded(&slug, seed, "check");
    let pv = loop {
        let x = rng.gen_range(0..p);
        if let Some((x, y)) = lift_small(p, a, b, x) {
            break ec.pt(x, y);
        }
    };
    let mut ops = GroupOps::default();
    if !is_prime_u64(r) || !ec.mul(&mut ops, pv, r).infinity {
        return Err("order check failed".into());
    }
    let bsz = 1usize << t;
    let (px, py_) = (1..p).find_map(|x| lift_small(p, a, b, x)).ok_or("no generator")?;
    if !ec.mul(&mut GroupOps::default(), ec.pt(px, py_), r).infinity {
        return Err("generator order check failed".into());
    }
    let l1_json = if l1 {
        json!({
            "requested": true,
            "status": "reconstructed_unverified",
            "reason": "the generated L1 JSON (a, b, g, zeta, x_R) is not in the committed files of research/prime_leads_chase_20261113/L1-pkm/ \
(only seeds and logs are); a, b, g (and x_R, see export --l1-replication) are re-derived by a native port of CPython random.Random(str) \
following gen.py's draw order with seed string '16801793-8-3-1-random'. MT19937 is tested against the reference vector; the CPython seeding \
path and the result against the original L1 system cannot be checked here (no Python, no stored fingerprint); the mode-gb replication gate compares row structure with the A3 log.",
            "seed_string": L1_SEED,
        })
    } else {
        json!({"requested": false})
    };
    let j = json!({
        "schema": "pkm.m8-panel.cell/v1",
        "slug": slug,
        "seed": seed,
        "master_seed": seed,
        "T": t,
        "R": ratio,
        "p": p,
        "a": a,
        "b": b,
        "r": r,
        "group_order": r,
        "g": g,
        "zeta": zeta,
        "B": bsz,
        "B_real": real.len(),
        "signed_points": 2 * real.len(),
        "V": v,
        "V_real": real.iter().map(|e| e.x).collect::<Vec<_>>(),
        "real_base": real.iter().map(|e| json!({"v_index": e.v_index, "x": e.x, "y": e.y})).collect::<Vec<_>>(),
        "generator": {"x": px, "y": py_},
        "curve_search_attempts": attempts,
        "rejected_primes": rejected,
        "min_prime_override": min_prime,
        "eligibility": {"curve_attempt_bound": CURVE_ATTEMPTS, "g_bound": G_BOUND, "b_real_min": bsz.div_ceil(2)},
        "point_count_method": if p < 1 << 20 { "legendre_sum" } else { "hasse_bsgs" },
        "derivation_rule": RULE_TEXT,
        "l1_reuse": l1_json,
        "tool": crate::tool_info(),
    });
    let j = seal(j);
    let cell = Cell {
        p,
        a,
        b,
        r,
        t,
        g,
        zeta,
        v,
        real,
        px,
        py: py_,
        slug,
        sha256: j["sha256"].as_str().unwrap_or_default().to_string(),
    };
    Ok(Derived { json: j, cell })
}

pub fn run(args: &[String]) -> Result<(), String> {
    let t = crate::arg_req(args, "--t")?.parse::<usize>().map_err(|e| e.to_string())?;
    let ratio = crate::arg_req(args, "--ratio")?.parse::<u32>().map_err(|e| e.to_string())?;
    let seed = crate::arg_req(args, "--seed")?.parse::<u64>().map_err(|e| e.to_string())?;
    let out = crate::arg_req(args, "--out")?;
    let no_l1 = crate::flag(args, "--no-l1-reuse");
    let min_prime = crate::arg_opt(args, "--min-prime").map(|s| s.parse::<u64>()).transpose().map_err(|e| e.to_string())?;
    let d = derive_from(t, ratio, seed, no_l1, min_prime)?;
    write_json(&Path::new(&out).join("cell.json"), &d.json)?;
    println!(
        "cell {} p={} r={} B={} B_real={} sha256={}",
        d.cell.slug,
        d.cell.p,
        d.cell.r,
        d.cell.v.len(),
        d.cell.real.len(),
        d.cell.sha256
    );
    Ok(())
}
