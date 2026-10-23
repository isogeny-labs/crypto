//! The isobench problems: what is computed, how instances are drawn, which methods compete, in
//! which unit their cost is charged, and how an answer is checked by a route other than the
//! method that produced it.
//!
//! Every instance is a deterministic function of (problem, parameters, size, index); every
//! method is a deterministic function of the instance and a seed shared by all arms of one
//! execution. Answers are canonical strings (decimal integers separated by `,` and `;`).
use super::counted::{self, CalKey, Calibration, Counted, Tally, UNIT};
use crate::api::{PrimeCurve, PrimeFieldInt};
use crate::bigint::Big;
use crate::curve::*;
use crate::field::{is_prime, Field, Rng, Zp, Zp2};
use crate::find::modpoly::Phi;
use crate::fpm::FpM;
use crate::int::Int;
use crate::kernel::chain::{ell_power_isogeny, Strategy};
use crate::path::graph::{verify_path, Oracle, Path, PhiCache, PhiOracle};
use crate::poly;
use std::cell::Cell;
use std::collections::BTreeMap;

pub const UNIT_ORACLE: &str = "isobench.oracle_calls";
pub const UNIT_NORM: &str = "isobench.norm_bits";

/// One problem of the registry.
/// The scaling model a bound is fitted under.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Model {
    /// cost = C size^alpha
    Power,
    /// cost = C 2^(alpha size): for size = log2 p, cost = C p^alpha
    Exp2,
}

impl Model {
    pub fn name(self) -> &'static str {
        match self {
            Model::Power => "power",
            Model::Exp2 => "exp2",
        }
    }
    /// the cost formula, for documents and pages
    pub fn formula(self) -> &'static str {
        match self {
            Model::Power => "C * size^alpha",
            Model::Exp2 => "C * 2^(alpha * size)",
        }
    }
    pub fn parse(s: &str) -> Option<Model> {
        match s {
            "power" => Some(Model::Power),
            "exp2" => Some(Model::Exp2),
            _ => None,
        }
    }
}

pub struct ProblemDecl {
    pub id: &'static str,
    pub summary: &'static str,
    /// what the size parameter is
    pub size: &'static str,
    pub unit: &'static str,
    /// the exponent alpha that the best known method is expected to show, under `model`
    pub declared_alpha: f64,
    /// how cost scales with size: C size^alpha, or C 2^(alpha size) (C p^alpha for size log2 p)
    pub model: Model,
    /// method ids, the first is the reference
    pub methods: &'static [&'static str],
    /// (name, default)
    pub params: &'static [(&'static str, &'static str)],
}

pub fn registry() -> &'static [ProblemDecl] {
    &[
        ProblemDecl {
            id: "kernel.codomain",
            summary: "codomain of the l-isogeny with kernel <P>, P a rational point of order l on y^2 = x^3 + x over p = 4lk - 1",
            size: "l (odd prime)",
            unit: UNIT,
            declared_alpha: 0.5,
            model: Model::Power,
            methods: &["xonly", "velu", "kohel", "sqrtvelu"],
            params: &[("bits", "127")],
        },
        ProblemDecl {
            id: "kernel.chain",
            summary: "codomain of the l^n-isogeny with kernel <P> as n l-isogenies, P of order l^n on y^2 = x^3 + x over p = 4 l^n k - 1",
            size: "n (chain length)",
            unit: UNIT,
            declared_alpha: 1.0,
            model: Model::Power,
            methods: &["naive", "balanced", "optimal"],
            params: &[("bits", "255"), ("ell", "2")],
        },
        ProblemDecl {
            id: "find.kernel",
            summary: "kernel polynomial of the normalised l-isogeny E -> E~ given E and Elkies' E~ (BMSS problem)",
            size: "l (odd prime >= 5)",
            unit: UNIT,
            declared_alpha: 1.0,
            model: Model::Power,
            methods: &[
                "fast_elkies_prime",
                "linear_algebra",
                "stark1972",
                "atkin1992",
                "atkin_modcomp",
                "elkies1992",
                "elkies1998",
                "fast_elkies",
            ],
            params: &[("bits", "127")],
        },
        ProblemDecl {
            id: "modpoly.phi",
            summary: "the classical modular polynomial Phi_l mod p, p = 4lk - 1",
            size: "l (prime)",
            unit: UNIT,
            declared_alpha: 2.0,
            model: Model::Power,
            methods: &["hecke", "linear_algebra"],
            params: &[("bits", "127")],
        },
        ProblemDecl {
            id: "count.sea",
            summary: "#E(F_p) for a random curve over a random prime of the given size",
            size: "log2 p",
            unit: UNIT,
            declared_alpha: 4.0,
            model: Model::Power,
            methods: &["sea", "sea_no_cycles"],
            params: &[],
        },
        ProblemDecl {
            id: "path.ordinary",
            summary: "a path of l-isogenies between two ordinary j-invariants in one isogeny class (the second is the end of a random walk from the first long enough to mix: 4 log2 p + 8 * 2^(log2 p / 4) steps)",
            size: "log2 p",
            unit: UNIT_ORACLE,
            declared_alpha: 0.25,
            model: Model::Exp2,
            methods: &["galbraith", "ghs", "galbraith_stolbunov"],
            params: &[("ells", "2,3,5,7"), ("budget", "2000000")],
        },
        ProblemDecl {
            id: "path.supersingular",
            summary: "a path of 2- and 3-isogenies between two random supersingular j-invariants over F_p^2",
            size: "log2 p",
            unit: UNIT_ORACLE,
            declared_alpha: 0.25,
            model: Model::Exp2,
            methods: &["delfs_galbraith", "bfs", "ghs"],
            params: &[("budget", "4000000")],
        },
        ProblemDecl {
            id: "csidh.action",
            summary: "the CSIDH class-group action [prod l_i^{e_i}] E_0 with exponents uniform in [-B, B]",
            size: "B (exponent bound)",
            unit: UNIT,
            declared_alpha: 1.0,
            model: Model::Power,
            methods: &["tree", "batched", "one_at_a_time"],
            params: &[("preset", "csidh512")],
        },
        ProblemDecl {
            id: "klpt",
            summary: "an equivalent left O_0-ideal of 2-power norm for a random ideal of prime norm (cost = log2 of the output norm)",
            size: "log2 p",
            unit: UNIT_NORM,
            declared_alpha: 1.0,
            model: Model::Power,
            methods: &["klpt"],
            params: &[],
        },
        ProblemDecl {
            id: "radical.chain",
            summary: "codomain j-path of a non-backtracking chain of n radical l-isogenies (Castryck-Decru-Vercauteren) on the X_1(l) family, one l-th root per step",
            size: "n (chain length)",
            unit: UNIT,
            declared_alpha: 1.0,
            model: Model::Power,
            methods: &["radical"],
            params: &[("ell", "7"), ("bits", "61")],
        },
    ]
}

pub fn problem(id: &str) -> Result<&'static ProblemDecl, String> {
    registry()
        .iter()
        .find(|p| p.id == id)
        .ok_or_else(|| format!("unknown problem {id}"))
}

pub type Params = BTreeMap<String, String>;

/// Fill defaults and reject unknown parameters.
pub fn resolve_params(p: &ProblemDecl, given: &Params) -> Result<Params, String> {
    let mut out = Params::new();
    for k in given.keys() {
        if !p.params.iter().any(|(n, _)| n == k) {
            return Err(format!("{}: unknown parameter {k}", p.id));
        }
    }
    for (n, d) in p.params {
        out.insert(
            n.to_string(),
            given.get(*n).cloned().unwrap_or_else(|| d.to_string()),
        );
    }
    Ok(out)
}

fn param_u64(p: &Params, k: &str) -> Result<u64, String> {
    p[k].parse()
        .map_err(|_| format!("parameter {k}: not an integer"))
}

/// The result of one method run.
#[derive(Clone, Debug)]
pub struct Outcome {
    /// canonical answer, or None when the method gave up (budget exhausted)
    pub answer: Option<String>,
    /// cost in the problem's unit (fme for counted runs of algebraic problems)
    pub cost: f64,
    pub tally: Option<Tally>,
    /// named work counters (oracle calls, walk steps, attempts, ...)
    pub work: BTreeMap<String, u64>,
    /// wall time of the method call alone (instance generation excluded), ms; 0 for counted
    /// runs
    pub ms: f64,
}

// ------------------------------------------------------------------ field dispatch

/// Work generic over the field type, run by [`with_field`] (field types with a calibration key).
pub trait Task {
    type Out;
    fn run<F: PrimeFieldInt + CalKey>(self, f: &F) -> Self::Out;
}

/// Run `task` over GF(p) with the field implementation `api::with_field` would choose.
pub fn with_field<T: Task>(p: &Int, task: T) -> T::Out {
    let bits = p.bits();
    let d = p.to_string();
    match bits {
        0..=62 => task.run(&Zp::new(p.to_i128().unwrap() as u64)),
        63..=64 => task.run(&FpM::<1>::from_dec(&d)),
        65..=128 => task.run(&FpM::<2>::from_dec(&d)),
        129..=192 => task.run(&FpM::<3>::from_dec(&d)),
        193..=256 => task.run(&FpM::<4>::from_dec(&d)),
        257..=320 => task.run(&FpM::<5>::from_dec(&d)),
        321..=384 => task.run(&FpM::<6>::from_dec(&d)),
        385..=448 => task.run(&FpM::<7>::from_dec(&d)),
        449..=512 => task.run(&FpM::<8>::from_dec(&d)),
        _ => panic!("isobench: fields above 512 bits are not supported"),
    }
}

/// Run `body` on the counted wrapper (IFMA off, fresh tally) or on the plain field (timed, ms).
macro_rules! measure {
    ($f:expr, $counted:expr, $cal:expr, |$g:ident| $body:expr) => {{
        if $counted {
            crate::ntt::SIMD.with(|s| s.set(false));
            counted::reset();
            let $g = &Counted($f.clone());
            let ans = $body;
            let tally = counted::take();
            crate::ntt::SIMD.with(|s| s.set(true));
            let fme = $cal.price(&$f.cal_key(), &tally, $f.log2_q())?;
            (ans, fme, Some(tally), 0.0)
        } else {
            let $g = $f;
            let t0 = std::time::Instant::now();
            let ans = $body;
            (ans, 0.0, None, t0.elapsed().as_secs_f64() * 1e3)
        }
    }};
}

fn ints<F: PrimeFieldInt>(f: &F, v: &[F::E]) -> String {
    v.iter()
        .map(|&x| f.int(x).to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn dec(s: &str) -> Int {
    let (neg, d) = match s.strip_prefix('-') {
        Some(d) => (true, d),
        None => (false, s),
    };
    let v = Int::from_big(&Big::from_dec(d));
    if neg {
        -v
    } else {
        v
    }
}

fn parse_ints(s: &str) -> Vec<Int> {
    if s.is_empty() {
        return vec![];
    }
    s.split(',').map(dec).collect()
}

/// The instance's own random stream: (problem, size, index, salt) -> seed.
fn inst_rng(problem: &str, size: u64, index: u64, salt: &str) -> Rng {
    let h = crate::sha256::sha256(format!("isobench/{problem}/{size}/{index}/{salt}").as_bytes());
    Rng::new(u64::from_le_bytes(h[..8].try_into().unwrap()))
}

// ------------------------------------------------------------------ kernel instances

/// p = 4 l^n k - 1 with exactly `bits` bits, k the index-th admissible value from the bottom,
/// and P of exact order l^n on y^2 = x^3 + x (supersingular, E(F_p) cyclic of order p + 1).
fn kernel_instance(bits: u64, ell: u64, n: u32, index: u64) -> Result<(Int, Int, Int), String> {
    let ln = Int::from(ell).pow(n);
    let four_ln = &Int::from(4u64) * &ln;
    let mut k = Int::from(2u64).pow(bits as u32 - 1).div_floor(&four_ln);
    let mut found = 0;
    let p = loop {
        k = &k + &Int::one();
        let p = &(&four_ln * &k) - &Int::one();
        if p.bits() as u64 > bits {
            return Err(format!("no {bits}-bit prime p = 4 * {ell}^{n} * k - 1"));
        }
        if p.bits() as u64 == bits && p.is_probable_prime() {
            if found == index {
                break p;
            }
            found += 1;
        }
    };
    struct T<'a>(&'a Int, &'a Int, u64);
    impl Task for T<'_> {
        type Out = (Int, Int);
        fn run<F: PrimeFieldInt + CalKey>(self, f: &F) -> (Int, Int) {
            let e = Curve::new(f.one(), f.zero());
            let cof = (self.0 + &Int::one()).div_floor(self.1);
            let low = self.1.div_floor(&Int::from(self.2));
            let mut x0 = 2u64;
            loop {
                let x = f.from_u64(x0);
                if let Some(y) = f.sqrt(rhs(f, &e, x)) {
                    let q = pmul_big(f, &e, &Pt::Aff(x, y), cof.mag());
                    if pmul_big(f, &e, &q, low.mag()) != Pt::Inf {
                        if let Pt::Aff(qx, qy) = q {
                            return (f.int(qx), f.int(qy));
                        }
                    }
                }
                x0 += 1;
            }
        }
    }
    let (x, y) = with_field(&p, T(&p, &ln, ell));
    Ok((p, x, y))
}

fn odd_prime(l: u64, what: &str) -> Result<(), String> {
    if l < 3 || !is_prime(l) {
        return Err(format!("{what}: l = {l} must be an odd prime"));
    }
    Ok(())
}

fn codomain<F: Field>(
    f: &F,
    x: F::E,
    y: F::E,
    ell: u64,
    method: &str,
) -> Result<Curve<F::E>, String> {
    use crate::kernel::{kohel::kohel, sqrt_velu::sqrt_velu_fast, velu::velu_cyclic, xonly};
    let e = Curve::new(f.one(), f.zero());
    Ok(match method {
        "xonly" => {
            xonly::velu_xonly_fast(f, &e, x, ell)
                .ok_or("x-only Velu failed")?
                .cod
        }
        "velu" => velu_cyclic(f, &e, &Pt::Aff(x, y), ell).cod,
        "sqrtvelu" => sqrt_velu_fast(f, &e, x, ell).ok_or("sqrt-Velu failed")?.cod,
        "kohel" => {
            let n = ((ell - 1) / 2) as usize;
            let xs = xonly::multiples_x_fast(f, &e, x, n).ok_or("x-multiples failed")?;
            kohel(f, &e, &poly::from_roots(f, &xs), ell).cod
        }
        m => return Err(format!("kernel.codomain: unknown method {m}")),
    })
}

fn chain_codomain<F: Field>(
    f: &F,
    x: F::E,
    y: F::E,
    ell: u64,
    n: usize,
    method: &str,
) -> Result<Curve<F::E>, String> {
    let e = Curve::new(f.one(), f.zero());
    let strat = match method {
        "naive" => Strategy::Naive,
        "balanced" => Strategy::Balanced,
        "optimal" => {
            let l = ell as f64;
            Strategy::Optimal {
                mul: 12.0 * l.log2().ceil(),
                eval: 4.0 * l,
                iso: 6.0 * l,
            }
        }
        m => return Err(format!("kernel.chain: unknown method {m}")),
    };
    Ok(ell_power_isogeny(f, &e, &Pt::Aff(x, y), ell, n, &[], strat)
        .0
        .cod)
}

// ------------------------------------------------------------------ fixed primes

/// The prime of a tier for problems over one field: 2^61 - 1, 2^127 - 1, 2^255 - 19, the
/// CSIDH-512 prime.
fn tier_prime(bits: u64) -> Result<Int, String> {
    let s = match bits {
        61 => "2305843009213693951",
        127 => "170141183460469231731687303715884105727",
        255 => "57896044618658097711785492504343953926634992332820282019728792003956564819949",
        511 => "5326738796327623094747867617954605554069371494832722337612446642054009560026576537626892113026381253624626941643949444792662881241621373288942880288065659",
        b => return Err(format!("bits = {b}: tiers are 61, 127, 255, 511")),
    };
    Ok(dec(s))
}

/// A random prime of exactly `bits` bits from the instance stream (p = 3 mod 4 if asked).
fn random_prime(bits: u64, rng: &mut Rng, three_mod_four: bool) -> Int {
    loop {
        let limbs: Vec<u64> = (0..bits.div_ceil(64)).map(|_| rng.next()).collect();
        let mut v =
            Int::from_big(&Big::from_limbs(&limbs)).modulo(&Int::from(2u64).pow(bits as u32 - 1));
        v = &v + &Int::from(2u64).pow(bits as u32 - 1);
        if three_mod_four {
            v = &(&v - &v.modulo(&Int::from(4u64))) + &Int::from(3u64);
        }
        if v.bits() as u64 == bits && v.is_probable_prime() {
            return v;
        }
    }
}

// ------------------------------------------------------------------ run

/// Run `method` on instance (size, index) of `problem`: counted (fme under `cal`) or plain.
pub fn run(
    problem: &str,
    method: &str,
    params: &Params,
    size: u64,
    index: u64,
    seed: u64,
    counted: bool,
    cal: &Calibration,
) -> Result<Outcome, String> {
    let decl = self::problem(problem)?;
    if !decl.methods.contains(&method) {
        return Err(format!("{problem}: unknown method {method}"));
    }
    match problem {
        "kernel.codomain" => {
            odd_prime(size, problem)?;
            let (p, x, y) = kernel_instance(param_u64(params, "bits")?, size, 1, index)?;
            struct T<'a>(&'a Int, &'a Int, u64, &'a str, bool, &'a Calibration);
            impl Task for T<'_> {
                type Out = Result<Outcome, String>;
                fn run<F: PrimeFieldInt + CalKey>(self, f: &F) -> Self::Out {
                    let (x, y) = (f.elem(self.0), f.elem(self.1));
                    let (cod, fme, tally, ms) =
                        measure!(f, self.4, self.5, |g| codomain(g, x, y, self.2, self.3)?);
                    Ok(Outcome {
                        answer: Some(format!("{},{}", f.int(cod.a), f.int(cod.b))),
                        cost: fme,
                        tally,
                        work: BTreeMap::new(),
                        ms,
                    })
                }
            }
            with_field(&p, T(&x, &y, size, method, counted, cal))
        }
        "kernel.chain" => {
            let ell = param_u64(params, "ell")?;
            if !is_prime(ell) {
                return Err("kernel.chain: ell must be prime".into());
            }
            let (p, x, y) = kernel_instance(param_u64(params, "bits")?, ell, size as u32, index)?;
            struct T<'a>(&'a Int, &'a Int, u64, usize, &'a str, bool, &'a Calibration);
            impl Task for T<'_> {
                type Out = Result<Outcome, String>;
                fn run<F: PrimeFieldInt + CalKey>(self, f: &F) -> Self::Out {
                    let (x, y) = (f.elem(self.0), f.elem(self.1));
                    let (cod, fme, tally, ms) = measure!(f, self.5, self.6, |g| chain_codomain(
                        g, x, y, self.2, self.3, self.4
                    )?);
                    Ok(Outcome {
                        answer: Some(format!("{},{}", f.int(cod.a), f.int(cod.b))),
                        cost: fme,
                        tally,
                        work: BTreeMap::new(),
                        ms,
                    })
                }
            }
            with_field(&p, T(&x, &y, ell, size as usize, method, counted, cal))
        }
        "find.kernel" => {
            if size < 5 || !is_prime(size) {
                return Err("find.kernel: l must be a prime >= 5".into());
            }
            let p = tier_prime(param_u64(params, "bits")?)?;
            struct T<'a>(u64, u64, &'a str, bool, &'a Calibration, u64);
            impl Task for T<'_> {
                type Out = Result<Outcome, String>;
                fn run<F: PrimeFieldInt + CalKey>(self, f: &F) -> Self::Out {
                    let (e, et, sigma) = bmss_instance(f, self.0, self.1)?;
                    let m = crate::find::bmss::Method::ALL
                        .into_iter()
                        .find(|m| m.name() == self.2)
                        .unwrap();
                    let s = if m.needs_sigma() { Some(sigma) } else { None };
                    let (iso, fme, tally, ms) = measure!(f, self.3, self.4, |g| {
                        crate::find::bmss::isogeny(g, m, &e, &et, self.0 as usize, s)
                            .map(|iso| iso.ker)
                    });
                    let _ = self.5;
                    Ok(Outcome {
                        answer: iso.map(|ker| ints(f, &ker)),
                        cost: fme,
                        tally,
                        work: BTreeMap::new(),
                        ms,
                    })
                }
            }
            with_field(&p, T(size, index, method, counted, cal, seed))
        }
        "modpoly.phi" => {
            if !is_prime(size) {
                return Err("modpoly.phi: l must be prime".into());
            }
            let (p, _, _) = kernel_instance(param_u64(params, "bits")?, size.max(3), 1, index)?;
            struct T<'a>(u64, &'a str, bool, &'a Calibration);
            impl Task for T<'_> {
                type Out = Result<Outcome, String>;
                fn run<F: PrimeFieldInt + CalKey>(self, f: &F) -> Self::Out {
                    let ell = self.0 as usize;
                    let (phi, fme, tally, ms) = measure!(f, self.2, self.3, |g| match self.1 {
                        "hecke" => Phi::compute_hecke(g, ell).c,
                        _ => Phi::compute_linear_algebra(g, ell).c,
                    });
                    Ok(Outcome {
                        answer: Some(phi_digest(f, &phi)),
                        cost: fme,
                        tally,
                        work: BTreeMap::new(),
                        ms,
                    })
                }
            }
            with_field(&p, T(size, method, counted, cal))
        }
        "count.sea" => {
            let c = sea_instance(size, index)?;
            struct T<'a>(&'a PrimeCurve, &'a str, bool, &'a Calibration, u64);
            impl Task for T<'_> {
                type Out = Result<Outcome, String>;
                fn run<F: PrimeFieldInt + CalKey>(self, f: &F) -> Self::Out {
                    let e = Curve::new(f.elem(&self.0.a), f.elem(&self.0.b));
                    let cyc = if self.1 == "sea" {
                        crate::find::sea::CYCLE_DEGREE
                    } else {
                        1
                    };
                    let mut rng = Rng::new(self.4);
                    let (n, fme, tally, ms) = measure!(f, self.2, self.3, |g| {
                        let mut phis = std::collections::HashMap::new();
                        let ge = Curve::new(e.a, e.b);
                        crate::find::sea::sea_opts(g, &ge, 1000, &mut phis, &mut rng, cyc)
                            .map(|r| r.0)
                    });
                    Ok(Outcome {
                        answer: n.map(|n| n.to_dec()),
                        cost: fme,
                        tally,
                        work: BTreeMap::new(),
                        ms,
                    })
                }
            }
            with_field(&c.p.clone(), T(&c, method, counted, cal, seed))
        }
        "path.ordinary" => path_ordinary(params, size, index, method, seed),
        "path.supersingular" => path_supersingular(params, size, index, method, seed),
        "csidh.action" => csidh_action(params, size, index, method, seed, counted, cal),
        "klpt" => klpt_run(size, index, seed),
        "radical.chain" => radical_chain(params, size, index, counted, cal),
        _ => Err(format!("unknown problem {problem}")),
    }
}

// ------------------------------------------------------------------ BMSS instances

/// A curve over the tier prime with a normalised l-isogeny to Elkies' E~: the index-th curve
/// y^2 = x^3 + a x + b (a, b from the instance stream) whose Phi_l(j, Y) has a usable root.
fn bmss_instance<F: PrimeFieldInt>(
    f: &F,
    ell: u64,
    index: u64,
) -> Result<(Curve<F::E>, Curve<F::E>, F::E), String> {
    use crate::find::bmss::sigma_from_phi;
    use crate::find::elkies::elkies_codomain;
    let phi = Phi::compute(f, ell as usize);
    let mut rng = inst_rng("find.kernel", ell, index, "curve");
    for _ in 0..10_000 {
        let e = Curve::new(f.random(&mut rng), f.random(&mut rng));
        if f.is_zero(e.a) || f.is_zero(e.b) {
            continue;
        }
        for jt in phi.neighbors(f, jinv(f, &e), &mut rng) {
            if let Some(et) = elkies_codomain(f, &phi, &e, jt) {
                if let Some(s) = sigma_from_phi(f, &phi, &e, &et) {
                    return Ok((e, et, s));
                }
            }
        }
    }
    Err("find.kernel: no instance found".into())
}

/// Phi_l at three points fixed by l: (r1, r2), (r2, r1), (r1, r1).
fn phi_digest<F: PrimeFieldInt>(f: &F, c: &[Vec<F::E>]) -> String {
    let ev = |x: F::E, y: F::E| {
        let ys: Vec<F::E> = c.iter().map(|row| poly::eval(f, row, y)).collect();
        poly::eval(f, &ys, x)
    };
    let (r1, r2) = (f.from_u64(123_456_789), f.from_u64(987_654_321));
    [ev(r1, r2), ev(r2, r1), ev(r1, r1)]
        .iter()
        .map(|&v| f.int(v).to_string())
        .collect::<Vec<_>>()
        .join(",")
}

// ------------------------------------------------------------------ SEA instances

fn sea_instance(bits: u64, index: u64) -> Result<PrimeCurve, String> {
    if !(41..=512).contains(&bits) {
        return Err(
            "count.sea: log2 p must be in 41..512 (smaller fields are counted by BSGS)".into(),
        );
    }
    let mut rng = inst_rng("count.sea", bits, index, "prime");
    let p = random_prime(bits, &mut rng, false);
    loop {
        let limbs: Vec<u64> = (0..bits.div_ceil(64)).map(|_| rng.next()).collect();
        let a = Int::from_big(&Big::from_limbs(&limbs)).modulo(&p);
        let limbs: Vec<u64> = (0..bits.div_ceil(64)).map(|_| rng.next()).collect();
        let b = Int::from_big(&Big::from_limbs(&limbs)).modulo(&p);
        if a.is_zero() || b.is_zero() {
            continue;
        }
        if let Ok(c) = PrimeCurve::new(p.clone(), a, b) {
            return Ok(c);
        }
    }
}

// ------------------------------------------------------------------ path problems

/// A neighbour oracle that counts its calls (one call = one root-finding per degree asked).
struct CountingOracle<'a, E, O: Oracle<E>> {
    inner: &'a O,
    degrees: u64,
    calls: Cell<u64>,
    _e: std::marker::PhantomData<E>,
}
impl<E, O: Oracle<E>> Oracle<E> for CountingOracle<'_, E, O> {
    fn neighbors(&self, j: E, rng: &mut Rng) -> Vec<(usize, E)> {
        self.calls.set(self.calls.get() + self.degrees);
        self.inner.neighbors(j, rng)
    }
}

fn parse_ells(s: &str) -> Result<Vec<usize>, String> {
    let v: Vec<usize> = s
        .split(',')
        .map(|t| t.trim().parse::<usize>())
        .collect::<Result<_, _>>()
        .map_err(|_| "ells: expected L1,L2,...")?;
    if v.is_empty() || v.iter().any(|&l| !is_prime(l as u64)) {
        return Err("ells must be primes".into());
    }
    Ok(v)
}

/// Two j-invariants in one ordinary isogeny class over a `bits`-bit prime: a random curve and
/// the end of a random walk from it over the problem's degrees. The class group has order about
/// sqrt(p) and the walk uses |ells| generators of an abelian Cayley graph, which mixes in about
/// h^(2/|ells|) steps, so the walk takes 4 bits + 8 * 2^(bits/4) steps (2^(bits/4) ~ h^(1/2) for
/// four degrees): its end is far from the start rather than a few steps away.
fn ordinary_instance(bits: u64, index: u64, ells: &[usize]) -> Result<(u64, u64, u64), String> {
    if !(12..=40).contains(&bits) {
        return Err("path.ordinary: log2 p must be in 12..40".into());
    }
    let mut rng = inst_rng("path.ordinary", bits, index, "pair");
    let p = random_prime(bits, &mut rng, false).to_i128().unwrap() as u64;
    let fp = Zp::new(p);
    let cache = PhiCache::new(&fp, ells);
    loop {
        let (a, b) = (fp.random(&mut rng), fp.random(&mut rng));
        let e = Curve::new(a, b);
        let j1 = jinv(&fp, &e);
        if a == 0 || b == 0 || fp.is_zero(f_disc(&fp, &e)) {
            continue;
        }
        let steps = 4 * bits as usize + 8 * (1usize << (bits / 4));
        let w = crate::testdata::random_walk(&fp, &cache, ells, j1, steps, &mut rng);
        let j2 = *w.js.last().unwrap();
        if j2 != j1 && w.js.len() > 1 {
            return Ok((p, j1, j2));
        }
    }
}

fn f_disc(f: &Zp, e: &Curve<u64>) -> u64 {
    let a3 = f.mul(f.sq(e.a), e.a);
    f.add(f.mul(f.from_u64(4), a3), f.mul(f.from_u64(27), f.sq(e.b)))
}

fn path_string<E: Copy>(path: &Path<E>, show: impl Fn(E) -> String) -> String {
    let js: Vec<String> = path.js.iter().map(|&j| show(j)).collect();
    let ls: Vec<String> = path.ells.iter().map(|l| l.to_string()).collect();
    format!("{};{}", js.join(","), ls.join(","))
}

fn path_ordinary(
    params: &Params,
    bits: u64,
    index: u64,
    method: &str,
    seed: u64,
) -> Result<Outcome, String> {
    use crate::path::{galbraith, ghs};
    let ells = parse_ells(&params["ells"])?;
    let budget = param_u64(params, "budget")? as usize;
    let (p, j1, j2) = ordinary_instance(bits, index, &ells)?;
    let fp = Zp::new(p);
    let cache = PhiCache::new(&fp, &ells);
    let mut rng = Rng::new(seed);
    let base = PhiOracle {
        f: &fp,
        cache: &cache,
        ells: &ells,
    };
    let oracle = CountingOracle {
        inner: &base,
        degrees: ells.len() as u64,
        calls: Cell::new(0),
        _e: std::marker::PhantomData,
    };
    let t0 = std::time::Instant::now();
    let path = match method {
        "galbraith" => galbraith::galbraith_with(&oracle, j1, j2, budget, &mut rng).0,
        "ghs" => ghs::ghs_with(&oracle, j1, j2, budget, &mut rng).0,
        _ => {
            let w: Vec<u32> = ells
                .iter()
                .map(|&l| (1_000_000 / (l * l)).max(1) as u32)
                .collect();
            let (path, st) =
                ghs::galbraith_stolbunov(&fp, &cache, &ells, &w, j1, j2, budget, &mut rng);
            // one root-finding of the chosen degree per step
            oracle.calls.set(st.steps as u64);
            path
        }
    };
    let calls = oracle.calls.get();
    Ok(Outcome {
        answer: path.map(|p| path_string(&p, |j| j.to_string())),
        cost: calls as f64,
        tally: None,
        work: BTreeMap::from([("oracle_calls".to_string(), calls)]),
        ms: t0.elapsed().as_secs_f64() * 1e3,
    })
}

/// Two random supersingular j-invariants over F_p^2 (p = 7 mod 8, `bits` bits): ends of random
/// 2-walks of 3 * bits steps from j = 1728.
fn supersingular_instance(bits: u64, index: u64) -> Result<(u64, (u64, u64), (u64, u64)), String> {
    if !(10..=40).contains(&bits) {
        return Err("path.supersingular: log2 p must be in 10..40".into());
    }
    let mut rng = inst_rng("path.supersingular", bits, index, "pair");
    let p = loop {
        let q = random_prime(bits, &mut rng, true).to_i128().unwrap() as u64;
        if q % 8 == 7 {
            break q;
        }
    };
    let f2 = Zp2::new(p);
    let cache = PhiCache::new(&Zp::new(p), &[2, 3]);
    let c2 = cache.lift(|x| (x, 0u64));
    let start = (1728u64, 0u64);
    let w1 = crate::testdata::random_walk(&f2, &c2, &[2], start, 3 * bits as usize, &mut rng);
    let w2 = crate::testdata::random_walk(&f2, &c2, &[2], start, 3 * bits as usize, &mut rng);
    Ok((p, *w1.js.last().unwrap(), *w2.js.last().unwrap()))
}

fn path_supersingular(
    params: &Params,
    bits: u64,
    index: u64,
    method: &str,
    seed: u64,
) -> Result<Outcome, String> {
    use crate::path::{delfs_galbraith, galbraith, ghs};
    let budget = param_u64(params, "budget")? as usize;
    let (p, j1, j2) = supersingular_instance(bits, index)?;
    let fp = Zp::new(p);
    let f2 = Zp2::new(p);
    let cache = PhiCache::new(&fp, &[2, 3]);
    let c2 = cache.lift(|x| (x, 0u64));
    let mut rng = Rng::new(seed);
    let ells2 = [2usize];
    let base = PhiOracle {
        f: &f2,
        cache: &c2,
        ells: &ells2,
    };
    let oracle = CountingOracle {
        inner: &base,
        degrees: 1,
        calls: Cell::new(0),
        _e: std::marker::PhantomData,
    };
    let t0 = std::time::Instant::now();
    let path = match method {
        "delfs_galbraith" => {
            let (path, st) = delfs_galbraith::delfs_galbraith(
                &f2, &fp, &cache, &c2, j1, j2, budget, budget, &mut rng,
            );
            // one 2-root-finding per walk step; the F_p BFS expands each node over l in {2, 3}
            oracle.calls.set((st.walk_steps + 2 * st.bfs_nodes) as u64);
            path
        }
        "bfs" => galbraith::galbraith_with(&oracle, j1, j2, budget, &mut rng).0,
        _ => ghs::ghs_with(&oracle, j1, j2, budget, &mut rng).0,
    };
    let calls = oracle.calls.get();
    Ok(Outcome {
        answer: path.map(|p| path_string(&p, |j: (u64, u64)| format!("{}+{}i", j.0, j.1))),
        cost: calls as f64,
        tally: None,
        work: BTreeMap::from([("oracle_calls".to_string(), calls)]),
        ms: t0.elapsed().as_secs_f64() * 1e3,
    })
}

// ------------------------------------------------------------------ CSIDH

fn csidh_exps(n: usize, bound: u64, index: u64) -> Vec<i32> {
    let mut rng = inst_rng("csidh.action", bound, index, "exps");
    (0..n)
        .map(|_| rng.below(2 * bound + 1) as i32 - bound as i32)
        .collect()
}

fn csidh_on<F: Field>(
    cs: &crate::path::csidh::Csidh<F>,
    exps: &[i32],
    method: &str,
    seed: u64,
) -> F::E {
    let mut rng = Rng::new(seed);
    let a = cs.fp.zero();
    match method {
        "tree" => cs.action_fast(a, exps, &mut rng),
        "batched" => cs.action_batched(a, exps, &mut rng),
        _ => cs.action(a, exps, &mut rng),
    }
}

fn csidh_action(
    params: &Params,
    bound: u64,
    index: u64,
    method: &str,
    seed: u64,
    counted: bool,
    cal: &Calibration,
) -> Result<Outcome, String> {
    use crate::path::csidh::Csidh;
    if bound == 0 || bound > 20 {
        return Err("csidh.action: B must be in 1..20".into());
    }
    match params["preset"].as_str() {
        "csidh512" => {
            let cs = Csidh::<FpM<8>>::csidh512();
            let exps = csidh_exps(cs.primes.len(), bound, index);
            let f = cs.fp.clone();
            let fr = &f;
            let (a, fme, tally, ms) = measure!(fr, counted, cal, |g| {
                let c = Csidh {
                    fp: g.clone(),
                    primes: cs.primes.clone(),
                    p1: cs.p1.clone(),
                    sqrt_velu_min: cs.sqrt_velu_min,
                };
                csidh_on(&c, &exps, method, seed)
            });
            Ok(Outcome {
                answer: Some(f.int(a).to_string()),
                cost: fme,
                tally,
                work: BTreeMap::new(),
                ms,
            })
        }
        s => {
            let n: usize = s
                .strip_prefix("n=")
                .and_then(|v| v.parse().ok())
                .ok_or("preset must be csidh512 or n=K (K small primes, p < 2^62)")?;
            let cs = Csidh::with_n_primes(n).ok_or("no CSIDH prime with that many small primes")?;
            let exps = csidh_exps(cs.primes.len(), bound, index);
            let f = cs.fp;
            let fr = &f;
            let (a, fme, tally, ms) = measure!(fr, counted, cal, |g| {
                let c = Csidh {
                    fp: *g,
                    primes: cs.primes.clone(),
                    p1: cs.p1.clone(),
                    sqrt_velu_min: cs.sqrt_velu_min,
                };
                csidh_on(&c, &exps, method, seed)
            });
            Ok(Outcome {
                answer: Some(a.to_string()),
                cost: fme,
                tally,
                work: BTreeMap::new(),
                ms,
            })
        }
    }
}

// ------------------------------------------------------------------ KLPT

/// p = 3 mod 4 with `bits` bits and a left O_0-ideal of prime norm N ~ p, both from the
/// instance stream.
fn klpt_instance(bits: u64, index: u64) -> Result<(Int, Int, crate::quat::Lattice), String> {
    use crate::quat::klpt::ideal_from;
    use crate::quat::{Alg, Quat};
    if !(20..=256).contains(&bits) {
        return Err("klpt: log2 p must be in 20..256".into());
    }
    let mut rng = inst_rng("klpt", bits, index, "ideal");
    let p = random_prime(bits, &mut rng, true);
    let n = random_prime(bits, &mut rng, false);
    let alg = Alg::new(&p);
    let o0 = alg.o0();
    for _ in 0..1_000_000 {
        let (b, c, d) = (
            Int::from(rng.next() >> 2),
            Int::from(rng.next() >> 2),
            Int::from(rng.next() >> 2),
        );
        let t = -&(&(&b * &b) + &(&alg.p * &(&(&c * &c) + &(&d * &d))));
        let Some(a) = Int::sqrt_mod_prime(&t, &n) else {
            continue;
        };
        let i = ideal_from(&alg, &o0, &n, &Quat::new([a, b, c, d], Int::one()));
        if i.norm(&o0) == (n.clone(), Int::one()) {
            return Ok((p, n, i));
        }
    }
    Err("klpt: no ideal found".into())
}

fn klpt_run(bits: u64, index: u64, seed: u64) -> Result<Outcome, String> {
    use crate::quat::klpt::klpt;
    use crate::quat::Alg;
    let (p, _, i) = klpt_instance(bits, index)?;
    let alg = Alg::new(&p);
    let mut rng = Rng::new(seed);
    let t0 = std::time::Instant::now();
    let res = klpt(&alg, &i, 2, &mut rng);
    let ms = t0.elapsed().as_secs_f64() * 1e3;
    Ok(match res {
        Some(r) => {
            let q = |x: &crate::quat::Quat| {
                format!("{},{},{},{},{}", x.c[0], x.c[1], x.c[2], x.c[3], x.d)
            };
            Outcome {
                answer: Some(format!("{};{}", r.e, q(&r.xi))),
                cost: r.e as f64,
                tally: None,
                work: BTreeMap::from([("attempts".to_string(), r.attempts as u64)]),
                ms,
            }
        }
        None => Outcome {
            answer: None,
            cost: 0.0,
            tally: None,
            work: BTreeMap::new(),
            ms,
        },
    })
}

// ------------------------------------------------------------------ radical isogeny chains

/// A deterministic prime of about `bits` bits with p = 3 mod 4 and gcd(l, p - 1) = 1 (so the
/// l-th root used by each radical step is unique).
fn radical_prime(bits: u64, ell: u64) -> Result<Int, String> {
    if !(16..=512).contains(&bits) {
        return Err(format!(
            "radical.chain: bits {bits} out of range (16..=512)"
        ));
    }
    let three = Int::from(3u64);
    let four = Int::from(4u64);
    let mut cand = &Int::from(2u64).pow((bits - 1) as u32) + &three; // 3 mod 4
    loop {
        if cand.mod_u64(ell) != 1 && cand.is_probable_prime() {
            return Ok(cand);
        }
        cand = &cand + &four;
    }
}

/// The Tate-normal-form coefficients of the X_1(l) member with parameter `param`
/// (l = 7: family7; l = 5: c = b = param).
fn radical_model<F: Field>(f: &F, ell: u64, param: F::E) -> [F::E; 5] {
    if ell == 7 {
        crate::kernel::radical::model7(f, param)
    } else {
        crate::kernel::radical::tate_model(f, param, param)
    }
}

/// A deterministic valid start parameter for the radical l-chain: the first draw of the
/// instance stream giving a non-singular curve with j not in {0, 1728} (so Phi_l applies and
/// the X_1(l) member is in general position).
fn radical_start<F: PrimeFieldInt>(f: &F, ell: u64, index: u64) -> Result<F::E, String> {
    let mut rng = inst_rng("radical.chain", 0, index, "start");
    let c1728 = f.from_u64(1728);
    for _ in 0..2000 {
        let param = f.random(&mut rng);
        let short = crate::kernel::radical::to_short(f, radical_model(f, ell, param));
        // non-singular: 4 a^3 + 27 b^2 != 0
        let disc = f.add(
            f.mul(f.from_u64(4), f.mul(short.a, f.mul(short.a, short.a))),
            f.mul(f.from_u64(27), f.mul(short.b, short.b)),
        );
        if f.is_zero(disc) {
            continue;
        }
        let j = crate::kernel::radical::weierstrass_j(f, radical_model(f, ell, param));
        if !f.is_zero(j) && j != c1728 {
            return Ok(param);
        }
    }
    Err("radical.chain: no valid start parameter".into())
}

/// Run a chain of `n` radical l-isogeny steps and return the j-path j_0, ..., j_n.
fn radical_chain(
    params: &Params,
    size: u64,
    index: u64,
    counted: bool,
    cal: &Calibration,
) -> Result<Outcome, String> {
    let ell = param_u64(params, "ell")?;
    if ell != 5 && ell != 7 {
        return Err("radical.chain: ell must be 5 or 7".into());
    }
    let n = size as usize;
    if n < 2 {
        return Err("radical.chain: n >= 2".into());
    }
    let p = radical_prime(param_u64(params, "bits")?, ell)?;
    struct T<'a>(u64, usize, u64, bool, &'a Calibration);
    impl Task for T<'_> {
        type Out = Result<Outcome, String>;
        fn run<F: PrimeFieldInt + CalKey>(self, f: &F) -> Self::Out {
            use crate::kernel::radical::{root_exponent, step5, step7, weierstrass_j};
            let (ell, n, index) = (self.0, self.1, self.2);
            let e = root_exponent(f, ell).ok_or("radical.chain: gcd(l, q - 1) != 1")?;
            let param0 = radical_start(f, ell, index)?;
            let (js, fme, tally, ms) = measure!(f, self.3, self.4, |g| {
                let mut param = param0;
                let mut js = vec![weierstrass_j(g, radical_model(g, ell, param))];
                for _ in 0..n {
                    param = if ell == 7 {
                        step7(g, param, &e)
                    } else {
                        step5(g, param, &e)
                    };
                    js.push(weierstrass_j(g, radical_model(g, ell, param)));
                }
                js
            });
            Ok(Outcome {
                answer: Some(ints(f, &js)),
                cost: fme,
                tally,
                work: BTreeMap::new(),
                ms,
            })
        }
    }
    with_field(&p, T(ell, n, index, counted, cal))
}

// ------------------------------------------------------------------ verification

/// Check `answer` for instance (size, index) by a route independent of the method that produced
/// it. Ok(detail) when it holds, Err(detail) when it does not.
pub fn verify(
    problem: &str,
    method: &str,
    params: &Params,
    size: u64,
    index: u64,
    answer: &str,
) -> Result<String, String> {
    match problem {
        "kernel.codomain" => {
            let (p, x, y) = kernel_instance(param_u64(params, "bits")?, size, 1, index)?;
            // Kohel from the kernel polynomial; for the Kohel arm, x-only Velu
            let other = if method == "kohel" { "xonly" } else { "kohel" };
            let r = plain_codomain(&p, &x, &y, size, 1, other, 0)?;
            agree(answer, &r, &format!("codomain equals {other}'s"))
        }
        "kernel.chain" => {
            let ell = param_u64(params, "ell")?;
            let (p, x, y) = kernel_instance(param_u64(params, "bits")?, ell, size as u32, index)?;
            let other = if method == "naive" {
                "balanced"
            } else {
                "naive"
            };
            let r = plain_codomain(&p, &x, &y, ell, size as usize, other, 1)?;
            agree(
                answer,
                &r,
                &format!("codomain equals the {other} strategy's"),
            )
        }
        "find.kernel" => {
            let p = tier_prime(param_u64(params, "bits")?)?;
            struct T<'a>(u64, u64, &'a str);
            impl Task for T<'_> {
                type Out = Result<String, String>;
                fn run<F: PrimeFieldInt + CalKey>(self, f: &F) -> Self::Out {
                    let (e, et, _) = bmss_instance(f, self.0, self.1)?;
                    let h: Vec<F::E> = parse_ints(self.2).iter().map(|v| f.elem(v)).collect();
                    let want = ((self.0 - 1) / 2) as usize;
                    if h.len() != want + 1 || h[want] != f.one() {
                        return Err(format!("kernel polynomial is not monic of degree {want}"));
                    }
                    let iso = crate::kernel::kohel::kohel(f, &e, &h, self.0);
                    if iso.cod == et {
                        Ok("Kohel's codomain of the kernel equals E~".into())
                    } else {
                        Err("Kohel's codomain of the kernel differs from E~".into())
                    }
                }
            }
            with_field(&p, T(size, index, answer))
        }
        "modpoly.phi" => {
            let (p, x, y) = kernel_instance(param_u64(params, "bits")?, size.max(3), 1, index)?;
            struct T<'a>(u64, &'a str, &'a str, &'a Int, &'a Int);
            impl Task for T<'_> {
                type Out = Result<String, String>;
                fn run<F: PrimeFieldInt + CalKey>(self, f: &F) -> Self::Out {
                    let ell = self.0 as usize;
                    // the other construction where it is affordable, Hecke otherwise
                    let other = if self.1 == "hecke" && ell <= 41 {
                        "linear_algebra"
                    } else {
                        "hecke"
                    };
                    let phi = if other == "hecke" {
                        Phi::compute_hecke(f, ell)
                    } else {
                        Phi::compute_linear_algebra(f, ell)
                    };
                    if phi_digest(f, &phi.c) != self.2 {
                        return Err(format!("Phi_l differs from the {other} construction"));
                    }
                    // and Phi_l(1728, j(E')) = 0 for the Velu image of y^2 = x^3 + x under a
                    // rational l-torsion point (l odd)
                    if self.0 >= 3 {
                        let (x, y) = (f.elem(self.3), f.elem(self.4));
                        let e = Curve::new(f.one(), f.zero());
                        let cod =
                            crate::kernel::velu::velu_cyclic(f, &e, &Pt::Aff(x, y), self.0).cod;
                        if !f.is_zero(phi.eval(f, jinv(f, &e), jinv(f, &cod))) {
                            return Err("Phi_l(j(E), j(E')) != 0 for a Velu pair".into());
                        }
                    }
                    Ok(format!(
                        "equals the {other} construction; vanishes on a Velu pair"
                    ))
                }
            }
            with_field(&p, T(size, method, answer, &x, &y))
        }
        "count.sea" => {
            let c = sea_instance(size, index)?;
            let n = dec(answer);
            let pc = crate::api::check_order(&c, &n, 7);
            if pc.certificate.certified {
                Ok(
                    "order certified by point orders (unique multiple in the Hasse interval)"
                        .into(),
                )
            } else if pc
                .certificate
                .checks
                .iter()
                .any(|c| c.status == crate::api::Status::Fail)
            {
                Err("the claimed order fails the point-order checks".into())
            } else {
                Err("the claimed order could not be certified".into())
            }
        }
        "path.ordinary" => {
            let ells = parse_ells(&params["ells"])?;
            let (p, j1, j2) = ordinary_instance(size, index, &ells)?;
            let fp = Zp::new(p);
            let cache = PhiCache::new(&fp, &ells);
            let path = parse_path(answer, |s| {
                s.parse::<u64>().map_err(|_| "bad j".to_string())
            })?;
            check_path(&fp, &cache, &path, j1, j2)
        }
        "path.supersingular" => {
            let (p, j1, j2) = supersingular_instance(size, index)?;
            let f2 = Zp2::new(p);
            let c2 = PhiCache::new(&Zp::new(p), &[2, 3]).lift(|x| (x, 0u64));
            let path = parse_path(answer, |s| {
                let (a, b) = s.trim_end_matches('i').split_once('+').ok_or("bad j")?;
                Ok((
                    a.parse::<u64>().map_err(|_| "bad j")?,
                    b.parse::<u64>().map_err(|_| "bad j")?,
                ))
            })?;
            check_path(&f2, &c2, &path, j1, j2)
        }
        "csidh.action" => {
            let other = if method == "one_at_a_time" {
                "tree"
            } else {
                "one_at_a_time"
            };
            let cal = Calibration {
                id: String::new(),
                fields: BTreeMap::new(),
            };
            let r = csidh_action(params, size, index, other, 99, false, &cal)?;
            agree(
                answer,
                r.answer.as_deref().unwrap_or(""),
                &format!("A equals the {other} action's"),
            )
        }
        "klpt" => {
            use crate::quat::{Alg, Quat};
            let (p, _, i) = klpt_instance(size, index)?;
            let alg = Alg::new(&p);
            let o0 = alg.o0();
            let (e, xi) = answer.split_once(';').ok_or("bad answer")?;
            let e: u32 = e.parse().map_err(|_| "bad e")?;
            let v = parse_ints(xi);
            if v.len() != 5 {
                return Err("bad xi".into());
            }
            let xi = Quat::new(
                [v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()],
                v[4].clone(),
            );
            let j = i.rmul(&alg, &xi);
            let two = Int::from(2u64);
            if j.norm(&o0) != (two.pow(e), Int::one()) {
                return Err("N(I xi) is not 2^e".into());
            }
            if !o0.contains_lattice(&j) || j.left_order(&alg, &o0) != o0 {
                return Err("I xi is not an integral left O_0-ideal".into());
            }
            if o0.scale(&two, &Int::one()).contains_lattice(&j) {
                return Err("I xi is not primitive".into());
            }
            Ok(format!(
                "J = I xi is a primitive integral left O_0-ideal of norm 2^{e}"
            ))
        }
        "radical.chain" => {
            let ell = param_u64(params, "ell")?;
            let p = radical_prime(param_u64(params, "bits")?, ell)?;
            struct V<'a>(u64, usize, u64, &'a str);
            impl Task for V<'_> {
                type Out = Result<String, String>;
                fn run<F: PrimeFieldInt + CalKey>(self, f: &F) -> Self::Out {
                    let (ell, n, index, answer) = (self.0, self.1, self.2, self.3);
                    let js: Vec<F::E> = answer.split(',').map(|s| f.elem(&dec(s))).collect();
                    if js.len() != n + 1 {
                        return Err(format!(
                            "radical.chain: path has {} nodes, expected {}",
                            js.len(),
                            n + 1
                        ));
                    }
                    // j_0 is the start curve's j (ties the path to the instance)
                    let param0 = radical_start(f, ell, index)?;
                    let j0 =
                        crate::kernel::radical::weierstrass_j(f, radical_model(f, ell, param0));
                    if js[0] != j0 {
                        return Err(
                            "radical.chain: path does not start at the instance curve".into()
                        );
                    }
                    // each step is an l-isogeny (root of Phi_l) and non-backtracking; Phi_l is
                    // built independently of the radical-isogeny formulas
                    let phi = Phi::via_crt(f, ell as usize);
                    for i in 0..n {
                        if !f.is_zero(phi.eval(f, js[i], js[i + 1])) {
                            return Err(format!(
                                "radical.chain: Phi_{ell}(j_{i}, j_{}) != 0",
                                i + 1
                            ));
                        }
                        if i >= 1 && js[i - 1] == js[i + 1] {
                            return Err("radical.chain: a step backtracks".into());
                        }
                    }
                    Ok(format!(
                        "{n} radical {ell}-isogeny steps, each a root of Phi_{ell}, non-backtracking"
                    ))
                }
            }
            with_field(&p, V(ell, size as usize, index, answer))
        }
        _ => Err(format!("unknown problem {problem}")),
    }
}

fn agree(answer: &str, reference: &str, what: &str) -> Result<String, String> {
    if answer == reference {
        Ok(what.to_string())
    } else {
        Err(format!("not true: {what}"))
    }
}

/// Codomain on the plain field by `method` (kind 0: one l-isogeny; 1: an l^n chain).
fn plain_codomain(
    p: &Int,
    x: &Int,
    y: &Int,
    ell: u64,
    n: usize,
    method: &str,
    kind: u8,
) -> Result<String, String> {
    struct T<'a>(&'a Int, &'a Int, u64, usize, &'a str, u8);
    impl Task for T<'_> {
        type Out = Result<String, String>;
        fn run<F: PrimeFieldInt + CalKey>(self, f: &F) -> Self::Out {
            let (x, y) = (f.elem(self.0), f.elem(self.1));
            let c = if self.5 == 0 {
                codomain(f, x, y, self.2, self.4)?
            } else {
                chain_codomain(f, x, y, self.2, self.3, self.4)?
            };
            Ok(format!("{},{}", f.int(c.a), f.int(c.b)))
        }
    }
    with_field(p, T(x, y, ell, n, method, kind))
}

fn parse_path<E>(s: &str, j: impl Fn(&str) -> Result<E, String>) -> Result<Path<E>, String> {
    let (js, ls) = s.split_once(';').ok_or("bad path")?;
    let js = js.split(',').map(&j).collect::<Result<Vec<E>, String>>()?;
    let ells = if ls.is_empty() {
        vec![]
    } else {
        ls.split(',')
            .map(|t| t.parse::<usize>().map_err(|_| "bad degree".to_string()))
            .collect::<Result<_, _>>()?
    };
    if ells.len() + 1 != js.len() {
        return Err("path: one degree per edge expected".into());
    }
    Ok(Path { js, ells })
}

fn check_path<F: Field>(
    f: &F,
    cache: &PhiCache<F>,
    path: &Path<F::E>,
    j1: F::E,
    j2: F::E,
) -> Result<String, String> {
    if path.js.first() != Some(&j1) || path.js.last() != Some(&j2) {
        return Err("the path does not join the two j-invariants".into());
    }
    if path.ells.iter().any(|l| !cache.ells().contains(l)) {
        return Err("an edge of a degree outside the problem's set".into());
    }
    if !verify_path(f, cache, path) {
        return Err("Phi_l(j_i, j_{i+1}) != 0 on some edge".into());
    }
    Ok(format!("{} edges, each a root of Phi_l", path.ells.len()))
}

/// The instance identity (hash of its defining data), for records.
pub fn instance_id(problem: &str, params: &Params, size: u64, index: u64) -> String {
    let pj: Vec<String> = params.iter().map(|(k, v)| format!("{k}={v}")).collect();
    let h =
        crate::sha256::sha256_hex(format!("{problem}|{}|{size}|{index}", pj.join("&")).as_bytes());
    format!("ISI1h{}", &h[..24])
}
