//! The counted unit of isobench: field-multiplication equivalents (`isobench.fme`).
//!
//! [`Counted`] wraps any [`Field`] and charges its work to a thread-local tally:
//!
//! * element operations exactly: one per `mul` and `sq`, and an inversion at the calibrated
//!   ratio of that field (`pow`, `pow_big`, `div` and `batch_inv` run the trait's default bodies
//!   on the wrapper, so their multiplications are counted one by one);
//! * bulk kernels (`conv`, `conv_long`, `conv_sq`, `conv_trunc`, `conv_long`, `conv_cyclic`, `prepare`,
//!   `conv_prepared`, `dot_rev`, `dot_rev_rows`) are forwarded unchanged, so the algorithms keep
//!   their NTT and lazy-reduction paths, and priced from a pinned calibration table: the
//!   measured time of that kernel at that length in units of one multiplication of the same field
//!   (ecbench prices index-calculus work the same way, from `docs/ic/calibration.json`);
//! * `sqrt` is forwarded (its root must equal the plain field's) and priced as the
//!   exponentiation it performs: 1.5 log2 q multiplications.
//!
//! Additions, subtractions and negations are free, as group negations are in ecbench's unit.
//! Every charge is also kept raw (operation kind, length bucket, calls), so a record can be
//! re-priced under another calibration. Counted runs switch the AVX-512 IFMA path off
//! ([`crate::ntt::SIMD`]): thresholds then match on every host, so the calls, and the counts,
//! are identical wherever a run is replayed.
use crate::bigint::Big;
use crate::field::{Field, Prepared, Rng, Zp};
use crate::fpm::FpM;
use crate::json::Json;
use std::cell::RefCell;
use std::collections::BTreeMap;

pub const UNIT: &str = "isobench.fme";

/// The bulk kernels priced from the calibration table.
pub const KINDS: [&str; 8] = [
    "conv",
    "conv_long",
    "conv_sq",
    "conv_trunc",
    "dot",
    "prepare",
    "conv_prep",
    "conv_cyc",
];

/// Raw operation counts of one counted run.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Tally {
    pub mul: u64,
    pub sq: u64,
    pub inv: u64,
    pub sqrt: u64,
    /// (kind, length) -> calls; length is the kernel's output (or operand) length
    pub bulk: BTreeMap<(String, usize), u64>,
}

thread_local! {
    static TALLY: RefCell<Tally> = RefCell::new(Tally::default());
}

/// Start a fresh tally on this thread.
pub fn reset() {
    TALLY.with(|t| *t.borrow_mut() = Tally::default());
}

/// The tally since the last [`reset`].
pub fn take() -> Tally {
    TALLY.with(|t| std::mem::take(&mut *t.borrow_mut()))
}

fn bump(f: impl FnOnce(&mut Tally)) {
    TALLY.with(|t| f(&mut t.borrow_mut()));
}

fn bulk(kind: &str, n: usize, calls: u64) {
    bump(|t| *t.bulk.entry((kind.to_string(), n)).or_insert(0) += calls);
}

/// The calibration key of a field: which table prices its kernels.
pub trait CalKey {
    fn cal_key(&self) -> String;
    /// log2 of the field size (for the `sqrt` price)
    fn log2_q(&self) -> f64;
}
impl CalKey for Zp {
    fn cal_key(&self) -> String {
        "zp".into()
    }
    fn log2_q(&self) -> f64 {
        (self.p as f64).log2()
    }
}
impl<const N: usize> CalKey for FpM<N> {
    fn cal_key(&self) -> String {
        format!("fpm{N}")
    }
    fn log2_q(&self) -> f64 {
        self.modulus().bits() as f64
    }
}

/// A field whose work is tallied (see the module documentation).
#[derive(Clone, Debug)]
pub struct Counted<F>(pub F);

impl<F: Field + CalKey> CalKey for Counted<F> {
    fn cal_key(&self) -> String {
        self.0.cal_key()
    }
    fn log2_q(&self) -> f64 {
        self.0.log2_q()
    }
}

impl<F: Field> Field for Counted<F> {
    type E = F::E;
    fn zero(&self) -> F::E {
        self.0.zero()
    }
    fn one(&self) -> F::E {
        self.0.one()
    }
    fn add(&self, a: F::E, b: F::E) -> F::E {
        self.0.add(a, b)
    }
    fn sub(&self, a: F::E, b: F::E) -> F::E {
        self.0.sub(a, b)
    }
    fn neg(&self, a: F::E) -> F::E {
        self.0.neg(a)
    }
    fn mul(&self, a: F::E, b: F::E) -> F::E {
        bump(|t| t.mul += 1);
        self.0.mul(a, b)
    }
    fn sq(&self, a: F::E) -> F::E {
        bump(|t| t.sq += 1);
        self.0.sq(a)
    }
    fn inv(&self, a: F::E) -> F::E {
        bump(|t| t.inv += 1);
        self.0.inv(a)
    }
    fn from_u64(&self, n: u64) -> F::E {
        self.0.from_u64(n)
    }
    fn char(&self) -> u64 {
        self.0.char()
    }
    fn characteristic_exceeds(&self, n: u64) -> bool {
        self.0.characteristic_exceeds(n)
    }
    fn size(&self) -> u128 {
        self.0.size()
    }
    fn random(&self, rng: &mut Rng) -> F::E {
        self.0.random(rng)
    }
    fn is_zero(&self, a: F::E) -> bool {
        self.0.is_zero(a)
    }
    fn karatsuba_threshold(&self) -> usize {
        self.0.karatsuba_threshold()
    }
    fn q(&self) -> Big {
        self.0.q()
    }
    fn conv(&self, a: &[F::E], b: &[F::E]) -> Vec<F::E> {
        charge_product("conv", a.len(), b.len());
        self.0.conv(a, b)
    }
    fn conv_sq(&self, a: &[F::E]) -> Vec<F::E> {
        if !a.is_empty() {
            bulk("conv_sq", a.len(), 1);
        }
        self.0.conv_sq(a)
    }
    fn powmod_table_max(&self) -> usize {
        self.0.powmod_table_max()
    }
    fn sqr_threshold(&self) -> usize {
        self.0.sqr_threshold()
    }
    fn prepare(&self, b: &[F::E], max_len: usize, cyc: usize) -> Option<Prepared> {
        let r = self.0.prepare(b, max_len, cyc);
        if r.is_some() {
            bulk("prepare", max_len.max(cyc).max(b.len()), 1);
        }
        r
    }
    fn conv_prepared(&self, a: &[F::E], b: &Prepared) -> Option<Vec<F::E>> {
        let r = self.0.conv_prepared(a, b);
        if r.is_some() {
            bulk("conv_prep", a.len(), 1);
        }
        r
    }
    fn conv_cyclic(&self, a: &[F::E], b: &[F::E], len: usize) -> Option<Vec<F::E>> {
        let r = self.0.conv_cyclic(a, b, len);
        if r.is_some() {
            bulk("conv_cyc", len, 1);
        }
        r
    }
    fn conv_long(&self, a: &[F::E], b: &[F::E]) -> Option<Vec<F::E>> {
        let r = self.0.conv_long(a, b);
        if r.is_some() {
            charge_product("conv_long", a.len(), b.len());
        }
        r
    }
    fn conv_trunc(&self, a: &[F::E], b: &[F::E], n: usize) -> Vec<F::E> {
        if !a.is_empty() && !b.is_empty() && n > 0 {
            bulk("conv_trunc", n.min(a.len().max(b.len())), 1);
        }
        self.0.conv_trunc(a, b, n)
    }
    fn dot_rev_rows(&self, a: &[F::E], rows: &[&[F::E]]) -> Vec<F::E> {
        let n = a.len();
        if n > 0 && !rows.is_empty() {
            bulk("dot", n, rows.len() as u64);
        }
        self.0.dot_rev_rows(a, rows)
    }
    fn dot_rev(&self, a: &[F::E], b: &[F::E]) -> F::E {
        let n = a.len().min(b.len());
        if n > 0 {
            bulk("dot", n, 1);
        }
        self.0.dot_rev(a, b)
    }
    fn sqrt(&self, a: F::E) -> Option<F::E> {
        bump(|t| t.sqrt += 1);
        self.0.sqrt(a)
    }
}

/// An unbalanced product n x m is charged as ceil(max / min) balanced products of the smaller
/// length (how a long-by-short product is computed anyway).
fn charge_product(kind: &str, n: usize, m: usize) {
    if n == 0 || m == 0 {
        return;
    }
    let (lo, hi) = (n.min(m), n.max(m));
    bulk(kind, lo, hi.div_ceil(lo) as u64);
}

// ------------------------------------------------------------------ calibration

/// Prices for one field: an inversion, and each kernel at lengths 2^0 .. 2^K, all in units of
/// one multiplication of that field.
#[derive(Clone, Debug, PartialEq)]
pub struct FieldPrices {
    pub inv: f64,
    /// kind -> price at length 2^k
    pub tables: BTreeMap<String, Vec<f64>>,
}

/// The pinned calibration: field key -> prices.
#[derive(Clone, Debug, PartialEq)]
pub struct Calibration {
    pub id: String,
    pub fields: BTreeMap<String, FieldPrices>,
}

impl FieldPrices {
    /// Price of one call at length n: log-log interpolation between powers of two, the last
    /// segment's slope beyond the table.
    pub fn price(&self, kind: &str, n: usize) -> f64 {
        let t = &self.tables[kind];
        let x = (n.max(1) as f64).log2();
        let k = (x.floor() as usize).min(t.len() - 2);
        let (y0, y1) = (t[k].ln(), t[k + 1].ln());
        (y0 + (y1 - y0) * (x - k as f64)).exp()
    }
}

impl Calibration {
    pub fn to_json(&self) -> Json {
        Json::obj(vec![
            ("schema", Json::str("isobench.calibration/v1")),
            ("unit", Json::str(UNIT)),
            (
                "fields",
                Json::obj(
                    self.fields
                        .iter()
                        .map(|(k, p)| {
                            (
                                k.clone(),
                                Json::obj(vec![
                                    ("inv", Json::Float(round3(p.inv))),
                                    (
                                        "tables",
                                        Json::obj(
                                            p.tables
                                                .iter()
                                                .map(|(kind, v)| {
                                                    (
                                                        kind.clone(),
                                                        Json::Arr(
                                                            v.iter()
                                                                .map(|&x| Json::Float(round3(x)))
                                                                .collect(),
                                                        ),
                                                    )
                                                })
                                                .collect(),
                                        ),
                                    ),
                                ]),
                            )
                        })
                        .collect(),
                ),
            ),
        ])
    }

    pub fn from_json(text: &str) -> Result<Calibration, String> {
        let j = Json::parse(text)?;
        if j.get("schema").and_then(|s| s.as_str()) != Some("isobench.calibration/v1") {
            return Err("not an isobench.calibration/v1 document".into());
        }
        let mut fields = BTreeMap::new();
        let Some(Json::Obj(fs)) = j.get("fields") else {
            return Err("calibration: no fields".into());
        };
        for (key, fj) in fs {
            let inv = fj
                .get("inv")
                .and_then(|v| v.as_f64())
                .ok_or("calibration: inv")?;
            let mut tables = BTreeMap::new();
            let Some(Json::Obj(ts)) = fj.get("tables") else {
                return Err("calibration: tables".into());
            };
            for (kind, arr) in ts {
                let Json::Arr(v) = arr else {
                    return Err("calibration: table".into());
                };
                let v: Vec<f64> = v.iter().filter_map(|x| x.as_f64()).collect();
                if v.len() < 2 || v.iter().any(|&x| x <= 0.0) {
                    return Err(format!("calibration: bad table {key}/{kind}"));
                }
                tables.insert(kind.clone(), v);
            }
            for k in KINDS {
                if !tables.contains_key(k) {
                    return Err(format!("calibration: {key} lacks {k}"));
                }
            }
            fields.insert(key.clone(), FieldPrices { inv, tables });
        }
        let id = format!(
            "ISCAL1h{}",
            &crate::sha256::sha256_hex(text.as_bytes())[..32]
        );
        Ok(Calibration { id, fields })
    }

    /// The fme of a tally under the prices of field `key`.
    pub fn price(&self, key: &str, t: &Tally, log2_q: f64) -> Result<f64, String> {
        let p = self
            .fields
            .get(key)
            .ok_or_else(|| format!("no calibration for field {key}"))?;
        let mut s = (t.mul + t.sq) as f64 + t.inv as f64 * p.inv + t.sqrt as f64 * 1.5 * log2_q;
        for ((kind, n), &calls) in &t.bulk {
            s += calls as f64 * p.price(kind, *n);
        }
        Ok(s)
    }
}

fn round3(x: f64) -> f64 {
    (x * 1000.0).round() / 1000.0
}

/// The tally as JSON: element counts and the bulk calls by kind and length.
pub fn tally_json(t: &Tally) -> Json {
    Json::obj(vec![
        ("mul", Json::Num(t.mul as i64)),
        ("sq", Json::Num(t.sq as i64)),
        ("inv", Json::Num(t.inv as i64)),
        ("sqrt", Json::Num(t.sqrt as i64)),
        (
            "bulk",
            Json::Arr(
                t.bulk
                    .iter()
                    .map(|((k, n), c)| {
                        Json::Arr(vec![
                            Json::str(k),
                            Json::Num(*n as i64),
                            Json::Num(*c as i64),
                        ])
                    })
                    .collect(),
            ),
        ),
    ])
}

/// Measure the calibration of one field: one multiplication, one inversion and every kernel at
/// lengths 2^0 .. 2^max_log, with the IFMA path off (as in counted runs). Times are the best of
/// repeated blocks.
pub fn calibrate_field<F: Field>(f: &F, max_log: usize, rng: &mut Rng) -> FieldPrices {
    use std::hint::black_box;
    use std::time::Instant;
    crate::ntt::SIMD.with(|s| s.set(false));
    let best = |reps: usize, g: &mut dyn FnMut()| -> f64 {
        let mut b = f64::MAX;
        for _ in 0..5 {
            let t = Instant::now();
            for _ in 0..reps {
                g();
            }
            b = b.min(t.elapsed().as_secs_f64() / reps as f64);
        }
        b
    };
    let xs: Vec<F::E> = (0..256).map(|_| f.random(rng)).collect();
    let mut acc = f.one();
    let t_mul = best(20000, &mut || {
        for &x in &xs {
            acc = f.mul(acc, x);
        }
        black_box(acc);
    }) / xs.len() as f64;
    let t_inv = best(200, &mut || {
        for &x in &xs[..32] {
            black_box(f.inv(x));
        }
    }) / 32.0;
    let mut tables: BTreeMap<String, Vec<f64>> = BTreeMap::new();
    for k in 0..=max_log {
        let n = 1usize << k;
        let a: Vec<F::E> = (0..n).map(|_| f.random(rng)).collect();
        let b: Vec<F::E> = (0..n).map(|_| f.random(rng)).collect();
        let reps = (4096 >> k.min(12)).max(1);
        let t_conv = best(reps, &mut || drop(black_box(f.conv(&a, &b))));
        let t_long = match f.conv_long(&a, &b) {
            Some(_) => best(reps, &mut || drop(black_box(f.conv_long(&a, &b)))),
            None => t_conv,
        };
        let t_sq = best(reps, &mut || drop(black_box(f.conv_sq(&a))));
        let t_trunc = best(reps, &mut || drop(black_box(f.conv_trunc(&a, &b, n))));
        let t_dot = best(reps * 4, &mut || {
            black_box(f.dot_rev(&a, &b));
        });
        let cyc = n.next_power_of_two();
        // no transform at this length: a prepared product is an ordinary one
        let (t_prep, t_cprep) = match f.prepare(&b, 2 * n, cyc) {
            Some(prep) => (
                best(reps, &mut || drop(black_box(f.prepare(&b, 2 * n, cyc)))),
                best(reps, &mut || drop(black_box(f.conv_prepared(&a, &prep)))),
            ),
            None => (0.5 * t_conv, t_conv),
        };
        let t_cyc = match f.conv_cyclic(&a, &b, cyc) {
            Some(_) => best(reps, &mut || drop(black_box(f.conv_cyclic(&a, &b, cyc)))),
            None => t_conv,
        };
        for (kind, t) in [
            ("conv", t_conv),
            ("conv_long", t_long),
            ("conv_sq", t_sq),
            ("conv_trunc", t_trunc),
            ("dot", t_dot),
            ("prepare", t_prep),
            ("conv_prep", t_cprep),
            ("conv_cyc", t_cyc),
        ] {
            tables
                .entry(kind.into())
                .or_default()
                .push((t / t_mul).max(1e-3));
        }
    }
    crate::ntt::SIMD.with(|s| s.set(true));
    FieldPrices {
        inv: t_inv / t_mul,
        tables,
    }
}
