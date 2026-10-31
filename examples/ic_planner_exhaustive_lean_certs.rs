//! Restate the band verdicts of the planner studies as integer facts checked
//! by Lean's `decide` (`research/ic_planner_exhaustive_20261026/PROTOCOL.md`,
//! "What is proven in Lean", item 3). Native port of the study's former
//! `scripts/lean_certificates.py`; it writes the same bytes.
//!
//! Input: a config (`scripts/lean_certificates.json`) naming committed
//! `ecbench band` JSON files and, per study, the reference arms and the
//! base-size families. Output: `Certificates.lean` (one theorem per verdict,
//! each closed by `decide`) and `certificates.json` (the same verdicts, with
//! theorem names).
//!
//! Every number in the Lean file is copied from a band JSON's exact sums:
//! `charged_sum` (Σ (G − C) over verified runs) and `native_sum` (Σ (U + n_C)).
//! An arm's mean priced cost at weight `w = p/q` is `(q·c + p·n) / (q·v)`, with
//! `v` its verified run count, so "A cheaper than B at w" is the integer fact
//! `v_B · scaled(c_A, n_A, p, q) < v_A · scaled(c_B, n_B, p, q)`
//! (`Band.scaled`). A charged sum that is not an integer (calibration-priced
//! canonicalisation) is taken as the exact dyadic rational of its f64 value,
//! and every sum of that band is multiplied by the common power-of-two
//! denominator.
//!
//! `m*(w)`: G(m, w) is the geometric mean over a family's curves of
//! S_w(m) / S_w(plan), so its argmin is the argmin of the product over curves
//! of mean priced cost (the plan and sqrt(r) factors are common to every m).
//! Ties go to the smaller m, as in the studies' `derive.jq`.
//!
//! Nothing here is a statistic: the bootstrap intervals stay in
//! `ecbench compare`.
//!
//! ```text
//! cargo run --release --example ic_planner_exhaustive_lean_certs -- CONFIG OUT.lean OUT.json
//! ```

use std::cmp::Ordering;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

use crypto_lib::cryptanalysis::ecc2k130_pyjson::{
    dumps_floats, parse_json, py_int, py_str, Json, Obj, Style,
};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use sha2::{Digest, Sha256};

fn die(msg: String) -> ! {
    eprintln!("{msg}");
    exit(1)
}

fn at<'a>(v: &'a Json, key: &str) -> &'a Json {
    v.as_obj()
        .and_then(|o| o.get(key))
        .unwrap_or_else(|| die(format!("KeyError: {key:?}")))
}

fn list(v: &Json) -> &[Json] {
    match v {
        Json::List(l) => l,
        _ => die("expected a list".into()),
    }
}

fn s(v: &Json) -> &str {
    v.as_str()
        .unwrap_or_else(|| die("expected a string".into()))
}

fn obj(pairs: Vec<(&str, Json)>) -> Json {
    let mut o = Obj::new();
    for (k, v) in pairs {
        o.set(k, v);
    }
    Json::Obj(o)
}

// ── fractions.Fraction ─────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Eq)]
struct Frac {
    n: BigInt,
    d: BigInt,
}

impl Frac {
    fn new(n: BigInt, d: BigInt) -> Self {
        if d.is_zero() {
            die(format!("Fraction({n}, 0)"));
        }
        let g = n.gcd(&d);
        let (mut n, mut d) = (n / &g, d / &g);
        if d.is_negative() {
            n = -n;
            d = -d;
        }
        Frac { n, d }
    }

    fn int(n: BigInt) -> Self {
        Frac {
            n,
            d: BigInt::one(),
        }
    }

    fn zero() -> Self {
        Self::int(BigInt::zero())
    }

    fn one() -> Self {
        Self::int(BigInt::one())
    }

    /// `Fraction(x)` of a float: its exact dyadic value.
    fn from_f64(x: f64) -> Self {
        if !x.is_finite() {
            die(format!("cannot convert {x} to a Fraction"));
        }
        let bits = x.to_bits();
        let neg = bits >> 63 == 1;
        let exp = ((bits >> 52) & 0x7ff) as i64;
        let frac = bits & ((1u64 << 52) - 1);
        let (mant, e) = if exp == 0 {
            (frac, -1074)
        } else {
            (frac | (1u64 << 52), exp - 1075)
        };
        let mut n = BigInt::from(mant);
        if neg {
            n = -n;
        }
        if e >= 0 {
            Self::int(n << e as usize)
        } else {
            Self::new(n, BigInt::one() << (-e) as usize)
        }
    }

    /// `Fraction(text)`: `[sign] digits [. digits] [e exp]` or `n/d`.
    fn parse(text: &str) -> Self {
        let t = text.trim();
        let bad = || -> ! { die(format!("Invalid literal for Fraction: {text:?}")) };
        if let Some((a, b)) = t.split_once('/') {
            let n: BigInt = a.trim().parse().unwrap_or_else(|_| bad());
            let d: BigInt = b.trim().parse().unwrap_or_else(|_| bad());
            return Self::new(n, d);
        }
        let (neg, body) = match t.strip_prefix('-') {
            Some(r) => (true, r),
            None => (false, t.strip_prefix('+').unwrap_or(t)),
        };
        let (mant, exp) = match body.find(['e', 'E']) {
            Some(i) => (
                &body[..i],
                body[i + 1..].parse::<i64>().unwrap_or_else(|_| bad()),
            ),
            None => (body, 0),
        };
        let (whole, fracpart) = mant.split_once('.').unwrap_or((mant, ""));
        let digits = format!("{whole}{fracpart}").replace('_', "");
        if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
            bad();
        }
        let mut n: BigInt = digits.parse().unwrap();
        if neg {
            n = -n;
        }
        let e = exp - fracpart.replace('_', "").len() as i64;
        let ten = BigInt::from(10u32);
        if e >= 0 {
            Self::int(n * num_traits::pow(ten, e as usize))
        } else {
            Self::new(n, num_traits::pow(ten, (-e) as usize))
        }
    }

    /// `Fraction(str(v))` of a JSON value.
    fn of_str_of(v: &Json) -> Self {
        Self::parse(&py_str(v))
    }

    /// `Fraction(v)` of a JSON number.
    fn of_number(v: &Json) -> Self {
        match v {
            Json::Int(i) => Self::int(BigInt::from(*i)),
            Json::Float(f) => Self::from_f64(*f),
            Json::Bool(b) => Self::int(BigInt::from(u8::from(*b))),
            _ => die("expected a number".into()),
        }
    }

    /// `float(self)`: `n / d` correctly rounded.
    fn to_f64(&self) -> f64 {
        if self.n.is_zero() {
            return 0.0;
        }
        let neg = self.n.is_negative();
        let n = self.n.abs();
        let d = &self.d;
        // Scale so the quotient has at least 66 bits; fold the remainder
        // into a sticky bit, then let the u128 -> f64 cast round to even.
        let shift = 66 + d.bits() as i64 - n.bits() as i64;
        let (num, den) = if shift >= 0 {
            (n << shift as usize, d.clone())
        } else {
            (n, d << (-shift) as usize)
        };
        let (q, r) = num.div_rem(&den);
        let mut q = q.to_u128().expect("quotient fits 128 bits");
        if !r.is_zero() {
            q |= 1;
        }
        let x = (q as f64) * 2f64.powi(-shift as i32);
        if neg {
            -x
        } else {
            x
        }
    }
}

impl PartialOrd for Frac {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Frac {
    fn cmp(&self, other: &Self) -> Ordering {
        (&self.n * &other.d).cmp(&(&other.n * &self.d))
    }
}

impl fmt::Display for Frac {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.d.is_one() {
            write!(f, "{}", self.n)
        } else {
            write!(f, "{}/{}", self.n, self.d)
        }
    }
}

// ── Band files ─────────────────────────────────────────────────────

struct Arm {
    c: Frac,
    n: Option<BigInt>,
    v: BigInt,
    big_c: BigInt,
    big_n: Option<BigInt>,
}

struct BandFile {
    rel: String,
    sha: String,
    weights: Vec<Frac>,
    curve: String,
    /// Insertion order, as a Python dict keeps it.
    arms: Vec<(String, Arm)>,
    scale: BigInt,
    nonintegral: Vec<String>,
}

impl BandFile {
    fn load(repo: &Path, rel: &str) -> Self {
        let path = repo.join(rel);
        let bytes = fs::read(&path).unwrap_or_else(|e| die(format!("{rel}: {e}")));
        let sha = hex::encode(Sha256::digest(&bytes));
        let text = String::from_utf8(bytes).unwrap_or_else(|_| die(format!("{rel}: not UTF-8")));
        let data = parse_json(&text).unwrap_or_else(|e| die(format!("{rel}: {e}")));
        let schema = data.as_obj().and_then(|o| o.get("schema"));
        if schema.and_then(|x| x.as_str()) != Some("ecbench.band/v1") {
            die(format!("{rel}: not an ecbench.band/v1 report"));
        }
        let weights = list(at(&data, "weights"))
            .iter()
            .map(Frac::of_str_of)
            .collect();
        let rows = list(at(&data, "rows"));
        let mut curves: Vec<&str> = rows.iter().map(|r| s(at(r, "curve"))).collect();
        curves.sort();
        curves.dedup();
        if curves.len() != 1 {
            let shown: Vec<String> = curves.iter().map(|c| format!("'{c}'")).collect();
            die(format!(
                "{rel}: expected one curve, found [{}]",
                shown.join(", ")
            ));
        }
        let curve = curves[0].to_string();
        let mut arms: Vec<(String, Arm)> = Vec::new();
        let mut scale = BigInt::one();
        for r in rows {
            let name = s(at(r, "arm")).to_string();
            let verified = at(r, "verified");
            if verified != at(r, "runs") || *verified == Json::Int(0) {
                die(format!(
                    "{rel}: arm {name} verified {} of {}",
                    py_str(verified),
                    py_str(at(r, "runs"))
                ));
            }
            let cs = r.as_obj().and_then(|o| o.get("charged_sum"));
            let c = match cs {
                None | Some(Json::Null) => die(format!(
                    "{rel}: arm {name} has no charged_sum (band built before the exact sums)"
                )),
                Some(v) => Frac::of_number(v),
            };
            scale = scale.lcm(&c.d);
            let n = match r.as_obj().and_then(|o| o.get("native_sum")) {
                None | Some(Json::Null) => None,
                Some(v) => Some(BigInt::from(py_int(v).unwrap_or_else(|e| die(e)))),
            };
            let v = BigInt::from(py_int(verified).unwrap_or_else(|e| die(e)));
            let arm = Arm {
                c,
                n,
                v,
                big_c: BigInt::zero(),
                big_n: None,
            };
            match arms.iter_mut().find(|(k, _)| *k == name) {
                Some(slot) => slot.1 = arm,
                None => arms.push((name, arm)),
            }
        }
        // `lcm(*dens)` over every row, including any a repeated arm replaced.
        let nonintegral = arms
            .iter()
            .filter(|(_, x)| !x.c.d.is_one())
            .map(|(k, _)| k.clone())
            .collect();
        for (_, x) in arms.iter_mut() {
            let scaled = Frac::new(&x.c.n * &scale, x.c.d.clone());
            assert!(scaled.d.is_one());
            x.big_c = scaled.n;
            x.big_n = x.n.as_ref().map(|n| n * &scale);
        }
        BandFile {
            rel: rel.to_string(),
            sha,
            weights,
            curve,
            arms,
            scale,
            nonintegral,
        }
    }

    fn arm(&self, name: &str) -> &Arm {
        self.arms
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, a)| a)
            .unwrap_or_else(|| die(format!("KeyError: {name:?}")))
    }

    fn has(&self, name: &str) -> bool {
        self.arms.iter().any(|(k, _)| k == name)
    }

    /// (numerator, v) with mean priced cost = numerator / (q·v·scale), or None.
    fn priced(&self, arm: &str, w: &Frac) -> Option<(BigInt, BigInt)> {
        let x = self.arm(arm);
        if !w.n.is_zero() && x.big_n.is_none() {
            return None;
        }
        let n = x.big_n.clone().unwrap_or_default();
        Some((&w.d * &x.big_c + &w.n * n, x.v.clone()))
    }
}

fn ident(parts: &[&str]) -> String {
    parts.join("_").replace(['-', '.', '/'], "_")
}

/// Keep doc text from opening or closing a Lean comment.
fn safe(text: &str) -> String {
    text.replace("/-", "/ -").replace("-/", "- /")
}

fn lit<T: fmt::Display>(z: T) -> String {
    format!("({z} : Int)")
}

fn scaled_expr(x: &Arm, w: &Frac) -> String {
    let n = x.big_n.clone().unwrap_or_default();
    format!(
        "scaled {} {} {} {}",
        lit(&x.big_c),
        lit(n),
        lit(&w.n),
        lit(&w.d)
    )
}

fn wtag(w: &Frac) -> String {
    format!("w{}_{}", w.n, w.d)
}

fn arm_for<'a>(arms: &'a Json, tag: &str) -> &'a str {
    match arms {
        Json::Obj(_) => s(at(arms, tag)),
        _ => s(arms),
    }
}

fn eval_path<'a>(mut d: &'a Json, path: &str) -> &'a Json {
    for k in path.split('.') {
        d = at(d, k);
    }
    d
}

struct Out {
    thms: Vec<String>,
    verdicts: Vec<Json>,
}

impl Out {
    fn theorem(&mut self, name: String, prop: String, doc: String, record: Json) {
        self.thms.push(format!(
            "/-- {} -/\ntheorem {name} :\n    {prop} := by decide\n",
            safe(&doc)
        ));
        let Json::Obj(mut o) = record else {
            unreachable!()
        };
        o.set("theorem", Json::Str(name));
        self.verdicts.push(Json::Obj(o));
    }
}

fn product(xs: impl Iterator<Item = BigInt>) -> BigInt {
    xs.fold(BigInt::one(), |a, b| a * b)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 3 {
        die("usage: ic_planner_exhaustive_lean_certs CONFIG OUT.lean OUT.json".into());
    }
    let (cfg_path, out_lean, out_json) = (
        PathBuf::from(&args[0]),
        PathBuf::from(&args[1]),
        PathBuf::from(&args[2]),
    );
    let top = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .unwrap_or_else(|e| die(format!("git: {e}")));
    if !top.status.success() {
        die("git rev-parse --show-toplevel failed".into());
    }
    let repo = PathBuf::from(String::from_utf8_lossy(&top.stdout).trim());
    let cfg_text = fs::read_to_string(&cfg_path)
        .unwrap_or_else(|e| die(format!("{}: {e}", cfg_path.display())));
    let cfg = parse_json(&cfg_text).unwrap_or_else(|e| die(e));

    let mut out = Out {
        thms: Vec::new(),
        verdicts: Vec::new(),
    };
    let mut sources: Vec<Json> = Vec::new();
    let mut source_lines: Vec<String> = Vec::new();

    for study in list(at(&cfg, "studies")) {
        let sid = s(at(study, "id"));
        let mut bands: Vec<(String, BandFile)> = Vec::new();
        for (tag, rel) in at(study, "bands").as_obj().unwrap().iter() {
            let b = BandFile::load(&repo, s(rel));
            match bands.iter_mut().find(|(k, _)| k == tag) {
                Some(slot) => slot.1 = b,
                None => bands.push((tag.clone(), b)),
            }
        }
        let band = |tag: &str| -> &BandFile {
            bands
                .iter()
                .find(|(k, _)| k == tag)
                .map(|(_, b)| b)
                .unwrap_or_else(|| die(format!("KeyError: {tag:?}")))
        };
        for (tag, b) in &bands {
            sources.push(obj(vec![
                ("study", Json::str(sid)),
                ("band", Json::str(tag)),
                ("path", Json::str(&b.rel)),
                ("sha256", Json::str(&b.sha)),
                ("curve", Json::str(&b.curve)),
                (
                    "scale",
                    Json::Int(b.scale.to_i128().expect("scale fits 128 bits")),
                ),
                (
                    "nonintegral_charged",
                    Json::List(b.nonintegral.iter().map(|a| Json::str(a)).collect()),
                ),
            ]));
            let mut line = format!("* {sid} {tag}: `{}` sha256 `{}`", b.rel, b.sha);
            if !b.scale.is_one() {
                line += &format!(
                    " (scale {}: non-integral charged sums of {})",
                    b.scale,
                    b.nonintegral.join(", ")
                );
            }
            source_lines.push(safe(&line));
        }

        // A. Pairwise orders, every arm against each reference, at every weight.
        let refs: Vec<&str> = list(at(study, "refs")).iter().map(s).collect();
        for (tag, b) in &bands {
            for r in &refs {
                if !b.has(r) {
                    continue;
                }
                let mut names: Vec<&str> = b.arms.iter().map(|(k, _)| k.as_str()).collect();
                names.sort();
                for arm in names {
                    if arm == *r {
                        continue;
                    }
                    let mut orders: Vec<(Frac, &str)> = Vec::new();
                    for w in &b.weights {
                        let (Some(pa), Some(pb)) = (b.priced(arm, w), b.priced(r, w)) else {
                            continue;
                        };
                        let lhs = &pb.1 * &pa.0;
                        let rhs = &pa.1 * &pb.0;
                        let rel = match lhs.cmp(&rhs) {
                            Ordering::Less => "<",
                            Ordering::Greater => ">",
                            Ordering::Equal => "=",
                        };
                        match orders.iter_mut().find(|(k, _)| k == w) {
                            Some(slot) => slot.1 = rel,
                            None => orders.push((w.clone(), rel)),
                        }
                        let (a_, b_) = (b.arm(arm), b.arm(r));
                        let el = format!("{} * {}", lit(&b_.v), scaled_expr(a_, w));
                        let er = format!("{} * {}", lit(&a_.v), scaled_expr(b_, w));
                        let prop = match rel {
                            "<" => format!("{el} < {er}"),
                            ">" => format!("{er} < {el}"),
                            _ => format!("{el} = {er}"),
                        };
                        let (word, short) = match rel {
                            "<" => ("cheaper than", "lt"),
                            ">" => ("dearer than", "gt"),
                            _ => ("tied with", "eq"),
                        };
                        out.theorem(
                            ident(&[sid, tag, arm, short, r, &wtag(w)]),
                            prop,
                            format!(
                                "{sid} {tag}: `{arm}` is {word} `{r}` at w = {w} (mean priced cost; {}).",
                                b.rel
                            ),
                            obj(vec![
                                ("study", Json::str(sid)),
                                ("band", Json::str(tag)),
                                ("kind", Json::str("order")),
                                ("a", Json::str(arm)),
                                ("b", Json::str(r)),
                                ("w", Json::Str(w.to_string())),
                                ("order", Json::str(rel)),
                            ]),
                        );
                    }
                    // B. A strict reversal between w = 0 and w = 1 has an exact crossing weight.
                    let get = |w: &Frac| orders.iter().find(|(k, _)| k == w).map(|(_, o)| *o);
                    let (o0, o1) = (get(&Frac::zero()), get(&Frac::one()));
                    if let (Some(o0), Some(o1)) = (o0, o1) {
                        if o0 != "=" && o1 != "=" && o0 != o1 {
                            let (a_, b_) = (b.arm(arm), b.arm(r));
                            let (an, bn) = (a_.big_n.as_ref().unwrap(), b_.big_n.as_ref().unwrap());
                            let ca = &b_.v * &a_.big_c;
                            let na = &b_.v * an;
                            let cb = &a_.v * &b_.big_c;
                            let nb = &a_.v * bn;
                            let ws = Frac::new(cb - ca, na - nb);
                            assert!(Frac::zero() < ws && ws < Frac::one());
                            let (p, q) = (&ws.n, &ws.d);
                            let prop = format!(
                                "{} * scaled {} {} {} {} = {} * scaled {} {} {} {} ∧ 0 < {} ∧ {} < {}",
                                lit(&b_.v), lit(&a_.big_c), lit(an), lit(p), lit(q),
                                lit(&a_.v), lit(&b_.big_c), lit(bn), lit(p), lit(q),
                                lit(p), lit(p), lit(q)
                            );
                            let wf = ws.to_f64();
                            out.theorem(
                                ident(&[sid, tag, arm, "crosses", r]),
                                prop,
                                format!(
                                    "{sid} {tag}: `{arm}` and `{r}` reverse inside the band; their mean priced \
                                     costs are equal exactly at w* = {p}/{q} ≈ {wf:.6} (Band.sides: \
                                     one order below, the other above)."
                                ),
                                obj(vec![
                                    ("study", Json::str(sid)),
                                    ("band", Json::str(tag)),
                                    ("kind", Json::str("crossing")),
                                    ("a", Json::str(arm)),
                                    ("b", Json::str(r)),
                                    ("w_star", Json::Str(format!("{p}/{q}"))),
                                    ("w_star_float", Json::Float(wf)),
                                    ("below", Json::str(o0)),
                                    ("above", Json::str(o1)),
                                ]),
                            );
                        }
                    }
                }
            }
        }

        // C. m*(w): argmin over a family's grid of the product over its curves.
        let families = study.as_obj().and_then(|o| o.get("families"));
        for fam in families.map(list).unwrap_or(&[]) {
            let fname = s(at(fam, "name"));
            let ftags: Vec<&str> = list(at(fam, "bands")).iter().map(s).collect();
            let grid: Vec<(Frac, &Json)> = list(at(fam, "grid"))
                .iter()
                .map(|g| (Frac::of_str_of(at(g, "m")), at(g, "arms")))
                .collect();
            let weights = &band(ftags[0]).weights;
            let mut expect: Vec<(Frac, Frac)> = Vec::new();
            if let Some(e) = fam
                .as_obj()
                .and_then(|o| o.get("expect"))
                .filter(|e| e.truthy())
            {
                let p = repo.join(s(at(e, "path")));
                let text = fs::read_to_string(&p)
                    .unwrap_or_else(|er| die(format!("{}: {er}", p.display())));
                let d = parse_json(&text).unwrap_or_else(|er| die(er));
                for row in list(eval_path(&d, s(at(e, "rule")))) {
                    let w = Frac::of_str_of(at(row, "w"));
                    let m = Frac::of_str_of(at(row, "m_star"));
                    match expect.iter_mut().find(|(k, _)| *k == w) {
                        Some(slot) => slot.1 = m,
                        None => expect.push((w, m)),
                    }
                }
            }
            for w in weights {
                let mut vals: Vec<(&Frac, &Json, BigInt, BigInt)> = Vec::new();
                let mut complete = true;
                for (m, arms) in &grid {
                    let ps: Vec<Option<(BigInt, BigInt)>> = ftags
                        .iter()
                        .map(|t| band(t).priced(arm_for(arms, t), w))
                        .collect();
                    if ps.iter().any(|x| x.is_none()) {
                        complete = false;
                        break;
                    }
                    let ps: Vec<(BigInt, BigInt)> = ps.into_iter().map(|x| x.unwrap()).collect();
                    let num = product(ps.iter().map(|x| x.0.clone()));
                    let den = product(ps.iter().map(|x| x.1.clone()));
                    vals.push((m, *arms, num, den));
                }
                if !complete {
                    continue;
                }
                let mut bi = 0;
                for i in 1..vals.len() {
                    if &vals[i].2 * &vals[bi].3 < &vals[bi].2 * &vals[i].3 {
                        bi = i;
                    }
                }
                let ms = vals[bi].0;
                let best_arms = vals[bi].1;
                let reported = expect.iter().find(|(k, _)| k == w).map(|(_, m)| m);
                if let Some(rep) = reported {
                    if rep != ms {
                        die(format!(
                            "{sid} {fname} w={w}: certificate m* = {ms}, study reported {rep}"
                        ));
                    }
                }
                let prod = |arms: &Json, scaled: bool| -> String {
                    ftags
                        .iter()
                        .map(|t| {
                            let x = band(t).arm(arm_for(arms, t));
                            let e = if scaled { scaled_expr(x, w) } else { lit(&x.v) };
                            format!("({e})")
                        })
                        .collect::<Vec<_>>()
                        .join(" * ")
                };
                for (m, arms, _, _) in &vals {
                    if *m == ms {
                        continue;
                    }
                    let lhs = format!("({}) * ({})", prod(best_arms, true), prod(arms, false));
                    let rhs = format!("({}) * ({})", prod(arms, true), prod(best_arms, false));
                    let strict = *m < ms;
                    let prop = format!("{lhs} {} {rhs}", if strict { "<" } else { "≤" });
                    out.theorem(
                        ident(&[sid, fname, &wtag(w), "mstar", &ms.to_string(), "vs", &m.to_string()]),
                        prop,
                        format!(
                            "{sid} {fname}: at w = {w}, m* = {ms} is {} m = {m} (product over {} of mean priced cost).",
                            if strict { "strictly cheaper than" } else { "no dearer than" },
                            ftags.join(", ")
                        ),
                        obj(vec![
                            ("study", Json::str(sid)),
                            ("family", Json::str(fname)),
                            ("kind", Json::str("argmin")),
                            ("w", Json::Str(w.to_string())),
                            ("m_star", Json::Str(ms.to_string())),
                            ("versus", Json::Str(m.to_string())),
                            ("strict", Json::Bool(strict)),
                            ("reported", reported.map_or(Json::Null, |r| Json::Str(r.to_string()))),
                        ]),
                    );
                }
            }
        }
    }

    let header = format!(
        "/-!
# Certificates: the band verdicts of the planner studies, in integers

Generated by `scripts/lean_certificates.sh` from the committed `ecbench band`
JSON files listed below (path, sha256). Do not edit by hand. Each theorem is a
point verdict restated with the band's exact per-arm sums and closed by
`decide`; `Band.lean` proves what such facts imply across the band (crossing
weights, the direction the optimum moves). {} verdicts.

Sources:
",
        out.verdicts.len()
    ) + &source_lines.join("\n")
        + "\n-/\n";
    let lean = format!(
        "import Band\n\n{header}\nnamespace Certificates\nopen Band\n\n{}\nend Certificates\n",
        out.thms.join("\n")
    );
    fs::write(&out_lean, lean).unwrap_or_else(|e| die(format!("{}: {e}", out_lean.display())));
    let n_verdicts = out.verdicts.len();
    let n_sources = sources.len();
    let json = obj(vec![
        ("schema", Json::str("ic_planner_exhaustive.certificates/v1")),
        ("sources", Json::List(sources)),
        ("verdicts", Json::List(out.verdicts)),
    ]);
    fs::write(
        &out_json,
        dumps_floats(&json, Style::Indent(1), false) + "\n",
    )
    .unwrap_or_else(|e| die(format!("{}: {e}", out_json.display())));
    println!("{n_verdicts} verdicts from {n_sources} band files");
}
