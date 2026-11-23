//! Cost-to-break table generator (std-only Rust, no dependencies).
//!
//! Reads `inputs-v1.json` (every input number with its source), derives the
//! work, time and cost columns, and writes:
//!
//! * `table-v1.json`  inputs verbatim plus the derived rows and checks;
//! * `chart-v1.svg`   GPU-years per target on a log scale;
//! * the generated block of `README.md` between the table markers.
//!
//! Build and run from the repository root:
//!
//! ```sh
//! rustc -O --edition 2021 docs/cost-to-break/gen_table.rs -o /tmp/gen_table
//! /tmp/gen_table docs/cost-to-break           # rewrite the outputs
//! /tmp/gen_table docs/cost-to-break --check   # exit 1 if any output is stale
//! ```
//!
//! Formulas (the README states them with their sources):
//! * elliptic curves: expected rho steps E = c * sqrt(pi * n / (2 * A)), with
//!   n the prime group order, A the number of automorphisms the walk folds by
//!   (2 for the negation map) and c the walk constant; time = E / rate.
//! * RSA: GNFS cost scaled from a published record by the ratio of
//!   L_N[1/3, (64/9)^(1/3)] with the o(1) term set to 0.
//! * ML-KEM: cited figures only; nothing is converted to GPU time.

use std::collections::BTreeMap;
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

// ---------------------------------------------------------------- JSON ----

#[derive(Clone, Debug)]
enum V {
    Null,
    Bool(bool),
    Num(String),
    Str(String),
    Arr(Vec<V>),
    Obj(Vec<(String, V)>),
}

struct Parser<'a> {
    s: &'a [u8],
    i: usize,
}

impl<'a> Parser<'a> {
    fn ws(&mut self) {
        while self.i < self.s.len() && matches!(self.s[self.i], b' ' | b'\n' | b'\r' | b'\t') {
            self.i += 1;
        }
    }
    fn err<T>(&self, m: &str) -> Result<T, String> {
        Err(format!("JSON error at byte {}: {}", self.i, m))
    }
    fn value(&mut self) -> Result<V, String> {
        self.ws();
        match self.s.get(self.i) {
            None => self.err("unexpected end"),
            Some(b'{') => {
                self.i += 1;
                let mut kv = Vec::new();
                self.ws();
                if self.s.get(self.i) == Some(&b'}') {
                    self.i += 1;
                    return Ok(V::Obj(kv));
                }
                loop {
                    self.ws();
                    let k = match self.value()? {
                        V::Str(k) => k,
                        _ => return self.err("object key must be a string"),
                    };
                    self.ws();
                    if self.s.get(self.i) != Some(&b':') {
                        return self.err("expected ':'");
                    }
                    self.i += 1;
                    let v = self.value()?;
                    if kv.iter().any(|(kk, _): &(String, V)| kk == &k) {
                        return self.err(&format!("duplicate key {k}"));
                    }
                    kv.push((k, v));
                    self.ws();
                    match self.s.get(self.i) {
                        Some(b',') => self.i += 1,
                        Some(b'}') => {
                            self.i += 1;
                            return Ok(V::Obj(kv));
                        }
                        _ => return self.err("expected ',' or '}'"),
                    }
                }
            }
            Some(b'[') => {
                self.i += 1;
                let mut a = Vec::new();
                self.ws();
                if self.s.get(self.i) == Some(&b']') {
                    self.i += 1;
                    return Ok(V::Arr(a));
                }
                loop {
                    a.push(self.value()?);
                    self.ws();
                    match self.s.get(self.i) {
                        Some(b',') => self.i += 1,
                        Some(b']') => {
                            self.i += 1;
                            return Ok(V::Arr(a));
                        }
                        _ => return self.err("expected ',' or ']'"),
                    }
                }
            }
            Some(b'"') => {
                self.i += 1;
                let mut out = String::new();
                loop {
                    let c = match self.s.get(self.i) {
                        None => return self.err("unterminated string"),
                        Some(c) => *c,
                    };
                    self.i += 1;
                    match c {
                        b'"' => return Ok(V::Str(out)),
                        b'\\' => {
                            let e = *self.s.get(self.i).ok_or("bad escape")?;
                            self.i += 1;
                            match e {
                                b'"' => out.push('"'),
                                b'\\' => out.push('\\'),
                                b'/' => out.push('/'),
                                b'n' => out.push('\n'),
                                b't' => out.push('\t'),
                                b'r' => out.push('\r'),
                                b'b' => out.push('\u{8}'),
                                b'f' => out.push('\u{c}'),
                                b'u' => {
                                    let h = std::str::from_utf8(&self.s[self.i..self.i + 4])
                                        .map_err(|e| e.to_string())?;
                                    let cp = u32::from_str_radix(h, 16).map_err(|e| e.to_string())?;
                                    self.i += 4;
                                    out.push(char::from_u32(cp).ok_or("surrogates unsupported")?);
                                }
                                _ => return self.err("bad escape"),
                            }
                        }
                        _ => {
                            // copy one UTF-8 sequence
                            let start = self.i - 1;
                            let len = match c {
                                0x00..=0x7f => 1,
                                0xc0..=0xdf => 2,
                                0xe0..=0xef => 3,
                                _ => 4,
                            };
                            self.i = start + len;
                            out.push_str(
                                std::str::from_utf8(&self.s[start..start + len]).map_err(|e| e.to_string())?,
                            );
                        }
                    }
                }
            }
            Some(b't') if self.s[self.i..].starts_with(b"true") => {
                self.i += 4;
                Ok(V::Bool(true))
            }
            Some(b'f') if self.s[self.i..].starts_with(b"false") => {
                self.i += 5;
                Ok(V::Bool(false))
            }
            Some(b'n') if self.s[self.i..].starts_with(b"null") => {
                self.i += 4;
                Ok(V::Null)
            }
            Some(_) => {
                let start = self.i;
                while self.i < self.s.len() && matches!(self.s[self.i], b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9') {
                    self.i += 1;
                }
                if start == self.i {
                    return self.err("unexpected character");
                }
                let t = std::str::from_utf8(&self.s[start..self.i]).unwrap().to_string();
                t.parse::<f64>().map_err(|_| format!("bad number {t}"))?;
                Ok(V::Num(t))
            }
        }
    }
}

fn parse(text: &str) -> Result<V, String> {
    let mut p = Parser { s: text.as_bytes(), i: 0 };
    let v = p.value()?;
    p.ws();
    if p.i != p.s.len() {
        return p.err("trailing data");
    }
    Ok(v)
}

fn esc_json(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

fn dump(v: &V, ind: usize, out: &mut String) {
    let pad = |n: usize| " ".repeat(n);
    match v {
        V::Null => out.push_str("null"),
        V::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        V::Num(t) => out.push_str(t),
        V::Str(s) => esc_json(s, out),
        V::Arr(a) if a.is_empty() => out.push_str("[]"),
        V::Obj(o) if o.is_empty() => out.push_str("{}"),
        V::Arr(a) => {
            out.push_str("[\n");
            for (k, x) in a.iter().enumerate() {
                out.push_str(&pad(ind + 2));
                dump(x, ind + 2, out);
                out.push_str(if k + 1 < a.len() { ",\n" } else { "\n" });
            }
            out.push_str(&pad(ind));
            out.push(']');
        }
        V::Obj(o) => {
            out.push_str("{\n");
            for (k, (key, x)) in o.iter().enumerate() {
                out.push_str(&pad(ind + 2));
                esc_json(key, out);
                out.push_str(": ");
                dump(x, ind + 2, out);
                out.push_str(if k + 1 < o.len() { ",\n" } else { "\n" });
            }
            out.push_str(&pad(ind));
            out.push('}');
        }
    }
}

impl V {
    fn get(&self, k: &str) -> Option<&V> {
        match self {
            V::Obj(o) => o.iter().find(|(kk, _)| kk == k).map(|(_, v)| v),
            _ => None,
        }
    }
    fn req(&self, k: &str) -> Result<&V, String> {
        self.get(k).ok_or_else(|| format!("missing key '{k}'"))
    }
    fn f(&self, k: &str) -> Result<f64, String> {
        match self.req(k)? {
            V::Num(t) => Ok(t.parse::<f64>().unwrap()),
            _ => Err(format!("key '{k}' is not a number")),
        }
    }
    fn s(&self, k: &str) -> Result<String, String> {
        match self.req(k)? {
            V::Str(t) => Ok(t.clone()),
            _ => Err(format!("key '{k}' is not a string")),
        }
    }
    fn arr(&self) -> &[V] {
        match self {
            V::Arr(a) => a,
            _ => &[],
        }
    }
}

fn num(x: f64) -> V {
    // four significant digits, exponent form: deterministic and JSON-valid
    V::Num(format!("{:.4e}", x))
}
fn num3(x: f64) -> V {
    V::Num(format!("{:.3}", x))
}
fn st(s: &str) -> V {
    V::Str(s.to_string())
}

// ------------------------------------------------------------ helpers ----

/// log2 of a decimal or 0x-prefixed hexadecimal integer string.
fn log2_int(s: &str) -> Result<f64, String> {
    let (digits, radix) = match s.strip_prefix("0x") {
        Some(h) => (h, 16u32),
        None => (s, 10u32),
    };
    let mut x = 0f64;
    for c in digits.chars() {
        let d = c.to_digit(radix).ok_or_else(|| format!("bad digit in {s}"))?;
        x = x * radix as f64 + d as f64;
    }
    Ok(x.log2())
}

/// ln L_N[1/3, (64/9)^(1/3)] with o(1) = 0, for an N of `bits` bits.
fn ln_l_gnfs(bits: f64) -> f64 {
    let c = (64.0f64 / 9.0).powf(1.0 / 3.0);
    let ln_n = bits * std::f64::consts::LN_2;
    c * ln_n.powf(1.0 / 3.0) * ln_n.ln().powf(2.0 / 3.0)
}

/// Round to three significant digits, written for people.
fn human(x: f64) -> String {
    if x == 0.0 {
        return "0".into();
    }
    let ax = x.abs();
    if !(0.01..1e6).contains(&ax) {
        let s = format!("{:.2e}", x);
        // 5.07e21 -> 5.07e21 (already compact)
        return s;
    }
    let e = ax.log10().floor() as i32;
    let scale = 10f64.powi(e - 2);
    let r = (x / scale).round() * scale;
    if r.abs() >= 100.0 {
        let n = r.round() as i64;
        let raw = n.abs().to_string();
        let mut out = String::new();
        for (k, ch) in raw.chars().enumerate() {
            if k > 0 && (raw.len() - k) % 3 == 0 {
                out.push(',');
            }
            out.push(ch);
        }
        if n < 0 {
            out.insert(0, '-');
        }
        out
    } else {
        let dec = (2 - e).max(0) as usize;
        format!("{:.*}", dec, r)
    }
}

fn esc_xml(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

fn sup(n: i32) -> String {
    let map = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
    let mut s = String::new();
    if n < 0 {
        s.push('⁻');
    }
    for ch in n.abs().to_string().chars() {
        s.push(map[ch.to_digit(10).unwrap() as usize]);
    }
    s
}

// ------------------------------------------------------------- derive ----

struct Derived {
    name: String,
    basis: String, // measured | proxy | extrapolated | not converted
    log10_gpu_years: Option<f64>,
    md: Vec<String>, // markdown cells
    hover: String,
    note: String,
    json: V,
}

fn derive(inp: &V) -> Result<(Vec<Derived>, Vec<V>), String> {
    let consts = inp.req("constants")?;
    let spy = consts.f("seconds_per_year")?;
    let hpy = consts.f("hours_per_year")?;
    let prices = inp.req("prices")?;
    let rates = inp.req("rates")?;
    let gnfs = inp.req("gnfs")?;
    let mut out = Vec::new();
    let mut checks = Vec::new();
    let pi = std::f64::consts::PI;

    for row in inp.req("rows")?.arr() {
        let id = row.s("id")?;
        let name = row.s("name")?;
        let fam = row.s("family")?;
        let quantum = row.s("quantum_vulnerable")?;
        let q_short = quantum.split(':').next().unwrap_or("").trim().to_string();
        if fam == "elliptic curve" {
            let n_log2 = log2_int(&row.s("group_order")?)?;
            let a = row.f("automorphisms")?;
            let c = row.f("walk_constant")?;
            let log2_work = c.log2() + 0.5 * (pi.log2() + n_log2 - (2.0 * a).log2());
            let work = 2f64.powf(log2_work);
            let rate_id = row.s("rate")?;
            let rate = rates.req(&rate_id)?;
            let r = rate.f("value")?;
            let price_id = rate.s("price")?;
            let price = prices.req(&price_id)?;
            let (pl, ph) = (price.f("low_usd_per_gpu_hour")?, price.f("high_usd_per_gpu_hour")?);
            let secs = work / r;
            let gh = secs / 3600.0;
            let gy = secs / spy;
            let basis = row.s("rate_basis")?;
            let formula = if c == 1.0 {
                format!("sqrt(pi*n/(2*{a}))")
            } else {
                format!("{c}*sqrt(pi*n/(2*{a}))")
            };
            let rate_cell = format!(
                "{} G/s, {} ({})",
                human(r / 1e9),
                if basis == "measured" { "measured here" } else { "proxy: P-256 rate" },
                rate.s("gpu")?.replace("NVIDIA ", "").replace(" Blackwell Server Edition", "")
            );
            let md = vec![
                name.clone(),
                row.s("method")?,
                format!("`{formula}` = 2^{:.2} group operations (n ≈ 2^{:.2})", log2_work, n_log2),
                rate_cell,
                format!("{pl:.2}–{ph:.2} (AWS g7e, spot–on-demand)"),
                format!("{}{}", human(gy), if basis == "proxy" { " (proxy rate)" } else { "" }),
                format!("{}–{}", human(gh * pl), human(gh * ph)),
                q_short.clone(),
            ];
            let mut j = vec![
                ("id".into(), st(&id)),
                ("name".into(), st(&name)),
                ("basis".into(), st(&basis)),
                ("formula".into(), st(&format!("E = {formula}; time = E / rate"))),
                ("log2_group_order".into(), num3(n_log2)),
                ("log2_expected_work".into(), num3(log2_work)),
                ("work_unit".into(), st("group operations (rho steps)")),
                ("expected_work".into(), num(work)),
                ("rate_used".into(), st(&rate_id)),
                ("rate_steps_per_second".into(), num(r)),
                ("gpu_seconds".into(), num(secs)),
                ("gpu_hours".into(), num(gh)),
                ("gpu_years".into(), num(gy)),
                ("price_used".into(), st(&price_id)),
                ("usd_low".into(), num(gh * pl)),
                ("usd_high".into(), num(gh * ph)),
            ];
            if let Some(pc) = row.get("published_check") {
                let pub_l = pc.f("log2_work")?;
                j.push(("check_published_log2_work".into(), num3(pub_l)));
                j.push(("check_log2_difference".into(), num3(log2_work - pub_l)));
                checks.push(V::Obj(vec![
                    ("id".into(), st(&format!("{id}-published-work"))),
                    ("what".into(), st("formula work against the published planning budget")),
                    ("derived_log2".into(), num3(log2_work)),
                    ("published_log2".into(), num3(pub_l)),
                    ("difference_bits".into(), num3(log2_work - pub_l)),
                    ("source".into(), st(&pc.s("source")?)),
                ]));
            }
            if let Some(rc) = row.get("repo_cost_check") {
                let (h0, s0, o0) = (rc.f("gpu_hours")?, rc.f("usd_spot")?, rc.f("usd_on_demand")?);
                checks.push(V::Obj(vec![
                    ("id".into(), st(&format!("{id}-repository-cost"))),
                    ("what".into(), st("derived GPU-hours and dollars against the figures already in the repository")),
                    ("derived_gpu_hours".into(), num(gh)),
                    ("repository_gpu_hours".into(), num(h0)),
                    ("ratio_gpu_hours".into(), num3(gh / h0)),
                    ("derived_usd_spot".into(), num(gh * pl)),
                    ("repository_usd_spot".into(), num(s0)),
                    ("ratio_usd_spot".into(), num3(gh * pl / s0)),
                    ("derived_usd_on_demand".into(), num(gh * ph)),
                    ("repository_usd_on_demand".into(), num(o0)),
                    ("ratio_usd_on_demand".into(), num3(gh * ph / o0)),
                    ("source".into(), st(&rc.s("source")?)),
                ]));
            }
            let hover = format!(
                "{name}: 2^{:.2} steps at {} G/s = {} GPU-years; ${}–${}",
                log2_work,
                human(r / 1e9),
                human(gy),
                human(gh * pl),
                human(gh * ph)
            );
            let note = format!("{} GPU-years · ${}–${}", human(gy), human(gh * pl), human(gh * ph));
            out.push(Derived {
                name,
                basis,
                log10_gpu_years: Some(gy.log10()),
                md,
                hover,
                note,
                json: V::Obj(j),
            });
        } else if fam == "integer factorisation" {
            let bits = row.f("modulus_bits")?;
            let anchor_id = row.s("anchor")?;
            let anchor = gnfs.req("anchors")?.req(&anchor_id)?;
            let ab = anchor.f("bits")?;
            let agy = anchor.f("gpu_years")?;
            let price_id = anchor.s("price")?;
            let price = prices.req(&price_id)?;
            let (pl, ph) = (price.f("low_usd_per_gpu_hour")?, price.f("high_usd_per_gpu_hour")?);
            let dln = ln_l_gnfs(bits) - ln_l_gnfs(ab);
            let ratio_log2 = dln / std::f64::consts::LN_2;
            let gy = agy * dln.exp();
            let gh = gy * hpy;
            let strength = row.f("nist_security_strength_bits")?;
            // CPU cross-check from the RSA-250 record
            let cpu = gnfs.req("checks")?.req("rsa250_cpu")?;
            let cpu_years = cpu.f("core_years")? * (ln_l_gnfs(bits) - ln_l_gnfs(cpu.f("bits")?)).exp();
            let md = vec![
                name.clone(),
                row.s("method")?,
                format!(
                    "NIST strength {strength:.0} bits; GNFS L-ratio to the {ab:.0}-bit RSA-260 record = 2^{:.1}",
                    ratio_log2
                ),
                format!("RSA-260 record: {agy} GPU-years on B200-class GPUs (cited)"),
                format!("{pl:.2} (the record's own assumption)"),
                format!("{} (extrapolated)", human(gy)),
                format!("{}", human(gh * pl)),
                q_short.clone(),
            ];
            let j = vec![
                ("id".into(), st(&id)),
                ("name".into(), st(&name)),
                ("basis".into(), st("extrapolated")),
                (
                    "formula".into(),
                    st("GPU-years = anchor GPU-years x L(N)/L(anchor), L = L_N[1/3,(64/9)^(1/3)], o(1) = 0"),
                ),
                ("nist_security_strength_bits".into(), num3(strength)),
                ("anchor".into(), st(&anchor_id)),
                ("log2_l_ratio".into(), num3(ratio_log2)),
                ("gpu_years".into(), num(gy)),
                ("gpu_hours".into(), num(gh)),
                ("price_used".into(), st(&price_id)),
                ("usd_low".into(), num(gh * pl)),
                ("usd_high".into(), num(gh * ph)),
                ("cpu_cross_check_core_years".into(), num(cpu_years)),
                (
                    "caveat".into(),
                    st("order of magnitude only: the o(1) term, memory, the linear-algebra share and hardware change are ignored"),
                ),
            ];
            let hover = format!(
                "{name}: RSA-260's 13.5 GPU-years x 2^{:.1} = {} GPU-years (extrapolated); ${}",
                ratio_log2,
                human(gy),
                human(gh * pl)
            );
            let note = format!("{} GPU-years · ${} (extrapolated)", human(gy), human(gh * pl));
            out.push(Derived {
                name,
                basis: "extrapolated".into(),
                log10_gpu_years: Some(gy.log10()),
                md,
                hover,
                note,
                json: V::Obj(j),
            });
        } else {
            let cl = row.f("core_svp_classical_bits")?;
            let qb = row.f("core_svp_quantum_bits")?;
            let g = row.f("refined_log2_gates")?;
            let unc = row.get("refined_log2_gates_uncertainty_bits").map(|_| row.f("refined_log2_gates_uncertainty_bits")).transpose()?;
            let gates = match unc {
                Some(u) => format!("2^{g} gates (±{u:.0} bits)"),
                None => format!("2^{g} gates"),
            };
            let md = vec![
                name.clone(),
                row.s("method")?,
                format!("core-SVP 2^{cl:.0} classical, 2^{qb:.0} quantum; refined {gates} (cited)"),
                "not converted (no GPU rate for lattice sieving here)".into(),
                "—".into(),
                "—".into(),
                "—".into(),
                q_short.clone(),
            ];
            let j = vec![
                ("id".into(), st(&id)),
                ("name".into(), st(&name)),
                ("basis".into(), st("not converted")),
                ("core_svp_classical_bits".into(), num3(cl)),
                ("core_svp_quantum_bits".into(), num3(qb)),
                ("refined_log2_gates".into(), num3(g)),
                ("gpu_years".into(), V::Null),
                ("usd_low".into(), V::Null),
                ("usd_high".into(), V::Null),
                (
                    "why_not_converted".into(),
                    st("the cited figures count operations or gates in a cost model; turning them into GPU time needs a measured lattice-sieving rate, which this repository does not have"),
                ),
            ];
            let hover = format!("{name}: cited core-SVP 2^{cl:.0}, refined {gates}; not converted to GPU time");
            let note = format!("cited: core-SVP 2^{cl:.0}; {gates}");
            out.push(Derived {
                name,
                basis: "not converted".into(),
                log10_gpu_years: None,
                md,
                hover,
                note,
                json: V::Obj(j),
            });
        }
    }

    // GNFS consistency check: RSA-260 scaled to RSA-896 against the report.
    let a = gnfs.req("anchors")?.req("rsa260")?;
    let c896 = gnfs.req("checks")?.req("rsa896")?;
    let pred = a.f("gpu_years")? * (ln_l_gnfs(c896.f("bits")?) - ln_l_gnfs(a.f("bits")?)).exp();
    let rep = c896.f("reported_gpu_years")?;
    checks.push(V::Obj(vec![
        ("id".into(), st("gnfs-rsa260-to-rsa896")),
        ("what".into(), st("the L-ratio extrapolation from RSA-260 against the reported RSA-896 cost")),
        ("predicted_gpu_years".into(), num(pred)),
        ("reported_gpu_years".into(), num(rep)),
        ("ratio_predicted_to_reported".into(), num3(pred / rep)),
        ("caveat".into(), st("different software and unnamed GPUs; the reported compute is not independently confirmed")),
        ("source".into(), st(&c896.s("source")?)),
    ]));
    Ok((out, checks))
}

// -------------------------------------------------------------- outputs ----

fn table_json(inp: &V, rows: &[Derived], checks: &[V]) -> String {
    let obj = V::Obj(vec![
        ("schema".into(), st("cost-to-break/v1")),
        ("table_version".into(), inp.get("table_version").cloned().unwrap_or(V::Null)),
        ("as_of".into(), inp.get("as_of").cloned().unwrap_or(V::Null)),
        ("generated_by".into(), st("docs/cost-to-break/gen_table.rs from docs/cost-to-break/inputs-v1.json")),
        (
            "label".into(),
            st("Derived table. Elliptic-curve times use rho rates measured in this repository on one GPU type (Curve25519 uses the P-256 rate as a labelled proxy); RSA figures are extrapolated from a cited record; ML-KEM figures are cited and not converted. Not a benchmark or leaderboard row; no number here is pcc-verified."),
        ),
        (
            "display_header".into(),
            V::Arr(
                ["Target", "Best known classical method", "Expected work", "Throughput used", "$/GPU-hour", "GPU-years", "Cost, USD", "Quantum-vulnerable"]
                    .iter()
                    .map(|h| st(h))
                    .collect(),
            ),
        ),
        (
            "rows".into(),
            V::Arr(
                rows.iter()
                    .map(|r| {
                        let mut j = r.json.clone();
                        if let V::Obj(o) = &mut j {
                            o.push(("display_cells".into(), V::Arr(r.md.iter().map(|c| st(c)).collect())));
                        }
                        j
                    })
                    .collect(),
            ),
        ),
        ("checks".into(), V::Arr(checks.to_vec())),
        ("inputs".into(), inp.clone()),
    ]);
    let mut s = String::new();
    dump(&obj, 0, &mut s);
    s.push('\n');
    s
}

fn readme_block(inp: &V, rows: &[Derived], checks: &[V]) -> Result<String, String> {
    let mut s = String::new();
    s.push_str("<!-- table:begin (generated by gen_table.rs from inputs-v1.json; do not edit by hand) -->\n");
    let _ = writeln!(
        s,
        "Table version {}, as of {}.\n",
        inp.s("table_version")?,
        inp.s("as_of")?
    );
    s.push_str("| Target | Best known classical method | Expected work | Throughput used | $/GPU-hour | GPU-years | Cost, USD | Quantum-vulnerable |\n");
    s.push_str("|:--|:--|:--|:--|:--|--:|--:|:--|\n");
    for r in rows {
        let cells: Vec<String> = r.md.iter().map(|c| c.replace('|', "\\|")).collect();
        let _ = writeln!(s, "| {} |", cells.join(" | "));
    }
    s.push_str("\nChecks the generator recomputes on every run:\n\n");
    for c in checks {
        let id = c.s("id")?;
        let line = match id.as_str() {
            x if x.ends_with("published-work") => format!(
                "- `{id}`: formula gives 2^{} against the published 2^{} (difference {} bits).",
                fmtv(c, "derived_log2"),
                fmtv(c, "published_log2"),
                fmtv(c, "difference_bits")
            ),
            x if x.ends_with("repository-cost") => format!(
                "- `{id}`: derived/repository ratios {} (GPU-hours), {} (spot dollars), {} (on-demand dollars).",
                fmtv(c, "ratio_gpu_hours"),
                fmtv(c, "ratio_usd_spot"),
                fmtv(c, "ratio_usd_on_demand")
            ),
            _ => format!(
                "- `{id}`: predicted {} GPU-years against a reported {} (ratio {}); different software and unnamed GPUs.",
                human(c.f("predicted_gpu_years")?),
                human(c.f("reported_gpu_years")?),
                fmtv(c, "ratio_predicted_to_reported")
            ),
        };
        s.push_str(&line);
        s.push('\n');
    }
    s.push_str("<!-- table:end -->\n");
    Ok(s)
}

fn fmtv(c: &V, k: &str) -> String {
    match c.get(k) {
        Some(V::Num(t)) => t.clone(),
        _ => "?".into(),
    }
}

fn chart_svg(inp: &V, rows: &[Derived]) -> Result<String, String> {
    let (w, left, plot_w, ann_x) = (1000.0, 200.0, 440.0, 660.0);
    let top = 118.0;
    let row_h = 40.0;
    let max_dec = 24.0;
    let h = top + row_h * rows.len() as f64 + 108.0;
    let x_of = |d: f64| left + plot_w * (d / max_dec);
    let (surface, ink, ink2, muted, grid, blue) = ("#fcfcfb", "#0b0b0b", "#52514e", "#8a8984", "#e4e3df", "#2a78d6");
    let mut s = String::new();
    let _ = writeln!(
        s,
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}" font-family="DejaVu Sans, Helvetica, Arial, sans-serif" role="img" aria-labelledby="t d">"#
    );
    let _ = writeln!(s, r#"<title id="t">Cost to break, table version {}, GPU-years on a log scale</title>"#, inp.s("table_version")?);
    let _ = writeln!(
        s,
        r#"<desc id="d">Horizontal bars of GPU-years per target. Solid bars use a rho rate measured in this repository; hatched bars use a proxy rate or an extrapolation from a cited record; ML-KEM rows carry cited figures and no bar.</desc>"#
    );
    let _ = writeln!(
        s,
        r#"<defs><pattern id="hatch" patternUnits="userSpaceOnUse" width="6" height="6" patternTransform="rotate(45)"><rect width="6" height="6" fill="{surface}"/><line x1="0" y1="0" x2="0" y2="6" stroke="{blue}" stroke-width="2.5"/></pattern></defs>"#
    );
    let _ = writeln!(s, r#"<rect x="0" y="0" width="{w}" height="{h}" fill="{surface}"/>"#);
    let _ = writeln!(
        s,
        r#"<text x="24" y="34" font-size="19" font-weight="700" fill="{ink}">Cost to break with today's best classical method (table v{})</text>"#,
        inp.s("table_version")?
    );
    let _ = writeln!(
        s,
        r#"<text x="24" y="57" font-size="13" fill="{ink2}">GPU-years, log scale: RTX PRO 6000 for the elliptic curves, B200-class for RSA. Derived from committed rates and cited records; not a benchmark.</text>"#
    );
    // legend
    let ly = 80.0;
    let _ = writeln!(s, r#"<rect x="24" y="{}" width="22" height="12" rx="2" fill="{blue}"/>"#, ly - 10.0);
    let _ = writeln!(s, r#"<text x="52" y="{ly}" font-size="12.5" fill="{ink2}">rho rate measured in this repository</text>"#);
    let _ = writeln!(s, r#"<rect x="290" y="{}" width="22" height="12" rx="2" fill="url(#hatch)" stroke="{blue}" stroke-width="1"/>"#, ly - 10.0);
    let _ = writeln!(s, r#"<text x="318" y="{ly}" font-size="12.5" fill="{ink2}">proxy rate (Curve25519) or extrapolation from a cited record (RSA)</text>"#);
    // grid and ticks
    let axis_y = top + row_h * rows.len() as f64;
    let mut d = 0.0;
    while d <= max_dec + 1e-9 {
        let x = x_of(d);
        let _ = writeln!(s, r#"<line x1="{x:.1}" y1="{}" x2="{x:.1}" y2="{axis_y}" stroke="{grid}" stroke-width="1"/>"#, top - 8.0);
        let _ = writeln!(
            s,
            r#"<text x="{x:.1}" y="{}" font-size="12" text-anchor="middle" fill="{ink2}">10{}</text>"#,
            axis_y + 18.0,
            sup(d as i32)
        );
        d += 4.0;
    }
    let _ = writeln!(
        s,
        r#"<text x="{}" y="{}" font-size="12.5" text-anchor="middle" fill="{ink2}">GPU-years (log scale)</text>"#,
        left + plot_w / 2.0,
        axis_y + 38.0
    );
    let _ = writeln!(s, r#"<text x="{ann_x}" y="{}" font-size="12" font-weight="700" fill="{ink2}">GPU-years · cost in USD (spot–on-demand)</text>"#, top - 12.0);
    for (k, r) in rows.iter().enumerate() {
        let y = top + row_h * k as f64;
        let cy = y + row_h / 2.0;
        let _ = writeln!(
            s,
            r#"<text x="{}" y="{:.1}" font-size="13.5" text-anchor="end" fill="{ink}">{}</text>"#,
            left - 12.0,
            cy + 4.5,
            esc_xml(&r.name)
        );
        match r.log10_gpu_years {
            Some(lg) => {
                let x0 = x_of(0.0);
                let x1 = x_of(lg.max(0.0)).max(x0 + 4.0);
                let fill = if r.basis == "measured" { blue.to_string() } else { "url(#hatch)".to_string() };
                let stroke = if r.basis == "measured" { "none" } else { blue };
                let _ = writeln!(
                    s,
                    r#"<g><title>{}</title><rect x="{x0:.1}" y="{:.1}" width="{:.1}" height="18" rx="2" fill="{fill}" stroke="{stroke}" stroke-width="1"/></g>"#,
                    esc_xml(&r.hover),
                    cy - 9.0,
                    x1 - x0
                );
            }
            None => {
                let _ = writeln!(
                    s,
                    r#"<g><title>{}</title><text x="{:.1}" y="{:.1}" font-size="12" font-style="italic" fill="{muted}">not converted to GPU time (cited lattice estimate)</text></g>"#,
                    esc_xml(&r.hover),
                    x_of(0.0) + 6.0,
                    cy + 4.0
                );
            }
        }
        let _ = writeln!(s, r#"<text x="{ann_x}" y="{:.1}" font-size="12.5" fill="{ink}">{}</text>"#, cy + 4.5, esc_xml(&r.note));
    }
    let foot = axis_y + 62.0;
    let foot_lines = [
        "Rates: crypto:gpu/nist/RESULTS.md (P-256); crypto:gpu/ecc/validation/20261017-modal-rtx-pro-6000 (secp256k1); crypto:ecc2k130/RTX-PRO6000.md (ECC2K-130).",
        "Prices: crypto:ecc2k130/aws/README.md (AWS g7e, 2026-09-11). RSA: the RSA-260 record (13.5 GPU-years, $3.50/GPU-h) scaled by the GNFS L-function.",
        "ML-KEM: CRYSTALS-Kyber specification v3.02, Table 4. Generated by docs/cost-to-break/gen_table.rs from inputs-v1.json.",
    ];
    for (k, line) in foot_lines.iter().enumerate() {
        let _ = writeln!(
            s,
            r#"<text x="24" y="{}" font-size="11" fill="{muted}">{}</text>"#,
            foot + 16.0 * k as f64,
            esc_xml(line)
        );
    }
    s.push_str("</svg>\n");
    Ok(s)
}

fn replace_block(readme: &str, block: &str) -> Result<String, String> {
    let b = readme.find("<!-- table:begin").ok_or("README.md has no table:begin marker")?;
    let e_tag = "<!-- table:end -->\n";
    let e = readme.find(e_tag).ok_or("README.md has no table:end marker")? + e_tag.len();
    Ok(format!("{}{}{}", &readme[..b], block, &readme[e..]))
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: gen_table <dir> [--check]");
        return ExitCode::from(2);
    }
    let dir = Path::new(&args[1]);
    let check = args.iter().any(|a| a == "--check");
    let run = || -> Result<BTreeMap<String, String>, String> {
        let text = fs::read_to_string(dir.join("inputs-v1.json")).map_err(|e| e.to_string())?;
        let inp = parse(&text)?;
        let (rows, checks) = derive(&inp)?;
        let mut files = BTreeMap::new();
        files.insert("table-v1.json".to_string(), table_json(&inp, &rows, &checks));
        files.insert("chart-v1.svg".to_string(), chart_svg(&inp, &rows)?);
        let readme = fs::read_to_string(dir.join("README.md")).map_err(|e| e.to_string())?;
        files.insert("README.md".to_string(), replace_block(&readme, &readme_block(&inp, &rows, &checks)?)?);
        Ok(files)
    };
    let files = match run() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("gen_table: {e}");
            return ExitCode::from(2);
        }
    };
    let mut stale = 0;
    for (name, content) in &files {
        let path = dir.join(name);
        let cur = fs::read_to_string(&path).unwrap_or_default();
        if check {
            if cur != *content {
                eprintln!("stale: {}", path.display());
                stale += 1;
            } else {
                println!("ok: {}", path.display());
            }
        } else if cur != *content {
            fs::write(&path, content).expect("write");
            println!("wrote {}", path.display());
        } else {
            println!("unchanged {}", path.display());
        }
    }
    if stale > 0 {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
