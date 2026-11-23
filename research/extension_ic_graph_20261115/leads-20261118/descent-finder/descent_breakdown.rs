//! Where the extension-field descent work goes, from recorded Run50 data only.
//!
//! std-only (no crates), so it builds with a bare `rustc -O` and needs no
//! library build. It reads three byte-identical published Run50 files,
//! re-prices every recorded descent-phase primitive with the frozen
//! `bitop/v1` formulas (src/cryptanalysis/bitops.rs, prime field), checks the
//! re-priced sum against the recorded `diagnostic_priced_bitops` exactly, and
//! splits the original-IC descent by the legacy stage counters
//! (F4 / linear stage / Weil system / sextic roots+ordinates / residual stream
//! / verification). No new measurement; nothing here is a timing claim.
//! Every number it prints is a producer observation [pcc:unverified].
//!
//! Usage: descent_breakdown RUN50_DIR > breakdown.json
use std::collections::BTreeMap;
use std::fs;

#[derive(Clone, Debug)]
enum V {
    Null,
    Bool(bool),
    Num(String),
    Str(String),
    Arr(Vec<V>),
    Obj(Vec<(String, V)>),
}

struct P<'a> {
    s: &'a [u8],
    i: usize,
}

impl<'a> P<'a> {
    fn ws(&mut self) {
        while self.i < self.s.len() && (self.s[self.i] as char).is_ascii_whitespace() {
            self.i += 1;
        }
    }
    fn lit(&mut self, t: &str) -> Result<(), String> {
        if self.s[self.i..].starts_with(t.as_bytes()) {
            self.i += t.len();
            Ok(())
        } else {
            Err(format!("expected {t} at {}", self.i))
        }
    }
    fn string(&mut self) -> Result<String, String> {
        self.lit("\"")?;
        let mut out = Vec::new();
        while self.i < self.s.len() {
            let c = self.s[self.i];
            self.i += 1;
            match c {
                b'"' => return String::from_utf8(out).map_err(|e| e.to_string()),
                b'\\' => {
                    let e = self.s[self.i];
                    self.i += 1;
                    match e {
                        b'"' | b'\\' | b'/' => out.push(e),
                        b'n' => out.push(b'\n'),
                        b't' => out.push(b'\t'),
                        b'r' => out.push(b'\r'),
                        b'b' => out.push(8),
                        b'f' => out.push(12),
                        b'u' => {
                            let h = std::str::from_utf8(&self.s[self.i..self.i + 4])
                                .map_err(|e| e.to_string())?;
                            let cp = u32::from_str_radix(h, 16).map_err(|e| e.to_string())?;
                            self.i += 4;
                            let ch = char::from_u32(cp).unwrap_or('\u{fffd}');
                            let mut b = [0u8; 4];
                            out.extend_from_slice(ch.encode_utf8(&mut b).as_bytes());
                        }
                        _ => return Err("bad escape".into()),
                    }
                }
                _ => out.push(c),
            }
        }
        Err("unterminated string".into())
    }
    fn value(&mut self) -> Result<V, String> {
        self.ws();
        let c = *self.s.get(self.i).ok_or("eof")?;
        let v = match c {
            b'{' => {
                self.i += 1;
                let mut m = Vec::new();
                self.ws();
                if self.s[self.i] == b'}' {
                    self.i += 1;
                    return Ok(V::Obj(m));
                }
                loop {
                    self.ws();
                    let k = self.string()?;
                    self.ws();
                    self.lit(":")?;
                    let v = self.value()?;
                    m.push((k, v));
                    self.ws();
                    if self.s[self.i] == b',' {
                        self.i += 1;
                    } else {
                        self.lit("}")?;
                        break;
                    }
                }
                V::Obj(m)
            }
            b'[' => {
                self.i += 1;
                let mut a = Vec::new();
                self.ws();
                if self.s[self.i] == b']' {
                    self.i += 1;
                    return Ok(V::Arr(a));
                }
                loop {
                    a.push(self.value()?);
                    self.ws();
                    if self.s[self.i] == b',' {
                        self.i += 1;
                    } else {
                        self.lit("]")?;
                        break;
                    }
                }
                V::Arr(a)
            }
            b'"' => V::Str(self.string()?),
            b't' => {
                self.lit("true")?;
                V::Bool(true)
            }
            b'f' => {
                self.lit("false")?;
                V::Bool(false)
            }
            b'n' => {
                self.lit("null")?;
                V::Null
            }
            _ => {
                let st = self.i;
                while self.i < self.s.len() && b"+-0123456789.eE".contains(&self.s[self.i]) {
                    self.i += 1;
                }
                V::Num(String::from_utf8(self.s[st..self.i].to_vec()).unwrap())
            }
        };
        Ok(v)
    }
}

fn parse(path: &str) -> V {
    let b = fs::read(path).unwrap_or_else(|e| panic!("{path}: {e}"));
    let mut p = P { s: &b, i: 0 };
    let v = p.value().unwrap_or_else(|e| panic!("{path}: {e}"));
    p.ws();
    assert_eq!(p.i, b.len(), "{path}: trailing bytes");
    v
}

impl V {
    fn get(&self, k: &str) -> &V {
        match self {
            V::Obj(m) => m.iter().find(|(a, _)| a == k).map(|(_, v)| v).unwrap_or(&V::Null),
            _ => &V::Null,
        }
    }
    fn arr(&self) -> &[V] {
        match self {
            V::Arr(a) => a,
            _ => &[],
        }
    }
    /// Integers may be recorded as JSON numbers or as decimal strings.
    fn u(&self) -> Option<u128> {
        match self {
            V::Num(s) | V::Str(s) => s.parse().ok(),
            _ => None,
        }
    }
    fn s(&self) -> &str {
        match self {
            V::Str(s) => s,
            _ => "",
        }
    }
}

// ── bitop/v1, prime field (copied formulas; src/cryptanalysis/bitops.rs) ──
fn int_add(w: u128) -> u128 {
    5 * w
}
fn int_mul(w: u128) -> u128 {
    w * w + 5 * w * w.saturating_sub(1)
}
fn fp_add(b: u128) -> u128 {
    2 * int_add(b) + 3 * b
}
fn fp_mul(b: u128) -> u128 {
    2 * int_mul(b) + fp_add(b)
}
fn fp_inv(b: u128) -> u128 {
    2 * b * fp_add(b)
}
fn bits(x: u128) -> u128 {
    128 - x.leading_zeros() as u128
}
/// `rng_words` are already inside `word_op` (jv_bitops::rng_words) and carry
/// no price of their own; sqr/sqrt/legendre are priced but zero here.
fn price(prim: &str, b: u128, r: u128) -> Option<u128> {
    Some(match prim {
        "field_add" => fp_add(b),
        "field_mul" | "field_sqr" => fp_mul(b),
        "field_inv" | "legendre" => fp_inv(b),
        "field_sqrt" => 2 * b * fp_mul(b),
        "hash_probe" => 4 * b,
        "word_op" => 64,
        "modr_mul" => fp_mul(r),
        "modr_add" => fp_add(r),
        "modr_inv" => fp_inv(r),
        "rng_words" => 0,
        _ => return None,
    })
}

fn pct(a: u128, b: u128) -> String {
    if b == 0 {
        return "null".into();
    }
    format!("{:.4}", 100.0 * a as f64 / b as f64)
}

/// `compare SUMMARY.json`: old-vs-new counted descent work from one
/// traced_descent_check summary (identical residuals in every arm).
/// Reported three ways: (1) the jv_bitops-hooked descent subtotal as the
/// pipeline records it; (2) (1) plus the linear stage, which the hooks barely
/// see, priced at one fp_mul per counted lin_mul (adds not counted: a lower
/// bound on its price), minus the part the hooks did see; (3) (2) plus the
/// traced arm's learning RREF bound priced at fp_mul+fp_add per multiply-add.
fn compare(path: &str, pbits: u128) {
    let s = parse(path);
    let arm = |name: &str| -> V {
        s.get("arms").arr().iter().find(|a| a.get("arm").s() == name).cloned().expect("arm")
    };
    let (prod, traced, oracle) = (arm("production"), arm("traced"), arm("oracle"));
    let u = |v: &V, k: &str| v.get(k).u().unwrap_or_else(|| panic!("missing {k}"));
    let tr = s.get("trace");
    let lin_hooked = tr.get("linear_stage_hooked_bitops").u().expect("lin hooked");
    let learn = tr.get("learning_rref_upper_bound_uncounted_by_f4fp").u().expect("learn");
    let fm = fp_mul(pbits);
    let fa = fp_add(pbits);
    let with_lin = |v: &V| u(v, "hooked_descent_bitops") + u(v, "lin_muls") * fm - lin_hooked;
    let p1 = u(&prod, "hooked_descent_bitops");
    let t1 = u(&traced, "hooked_descent_bitops");
    let (p2, t2) = (with_lin(&prod), with_lin(&traced));
    let t3 = t2 + learn * (fm + fa);
    let ratio = |a: u128, b: u128| format!("{:.4}", a as f64 / b as f64);
    println!(
        "{{\"schema\":\"descent-finder.traced-compare/v1\",\"pcc\":\"unverified\",\"summary\":\"{path}\",\"residuals\":{},\"admitted\":{},\
\"f4_ops\":{{\"production\":{},\"traced\":{},\"oracle\":{},\"production_over_traced\":{}}},\
\"hooked_descent_bitops\":{{\"production\":{p1},\"traced\":{t1},\"traced_over_production\":{},\"production_over_traced\":{}}},\
\"with_linear_stage_priced\":{{\"production\":{p2},\"traced\":{t2},\"traced_over_production\":{},\"production_over_traced\":{}}},\
\"with_linear_stage_and_learning_bound\":{{\"production\":{p2},\"traced\":{t3},\"traced_over_production\":{},\"production_over_traced\":{}}},\
\"linear_stage\":{{\"lin_muls_per_arm\":{},\"hooked_bitops_seen\":{lin_hooked},\"unhooked_price_lower_bound\":{}}}}}",
        u(&s, "residuals"),
        matches!(s.get("admitted"), V::Bool(true)),
        u(&prod, "f4_ops"),
        u(&traced, "f4_ops"),
        u(&oracle, "f4_ops"),
        ratio(u(&prod, "f4_ops"), u(&traced, "f4_ops")),
        ratio(t1, p1),
        ratio(p1, t1),
        ratio(t2, p2),
        ratio(p2, t2),
        ratio(t3, p2),
        ratio(p2, t3),
        u(&prod, "lin_muls"),
        u(&prod, "lin_muls") * fm - lin_hooked,
    );
}

fn main() {
    let first = std::env::args().nth(1).expect("usage: descent_breakdown RUN50_DIR | compare SUMMARY.json P");
    if first == "compare" {
        let path = std::env::args().nth(2).expect("summary path");
        let p: u128 = std::env::args().nth(3).expect("p").parse().expect("p");
        return compare(&path, bits(p));
    }
    let dir = first;
    let q = parse(&format!("{dir}/qualified-original.json"));
    let study = parse(&format!("{dir}/run-study.json"));
    let raw = parse(&format!("{dir}/original-ic.raw.json"));
    let pr = q.get("public").get("p").u().expect("p");
    let l = q.get("public").get("l").u().expect("l");
    let (b, r) = (bits(pr), bits(l));
    let mut residuals = BTreeMap::new();
    for o in study.get("outcomes").arr() {
        residuals.insert(
            o.get("mode").s().to_string(),
            (o.get("descent_residuals").u(), o.get("accepted_descents").u(), o.get("observed_incomplete_bitops").u()),
        );
    }
    let mut arms = Vec::new();
    let mut all_exact = true;
    for o in q.get("outcomes").arr() {
        let mode = o.get("mode").s();
        let phases = o.get("proof").get("accounting").get("phases").arr();
        let Some(d) = phases.iter().find(|p| p.get("phase").s() == "descent") else { continue };
        let recorded = d.get("diagnostic_priced_bitops").u().expect("recorded price");
        let mut rows = Vec::new();
        let mut sum = 0u128;
        if let V::Obj(m) = d.get("primitives") {
            for (k, v) in m {
                let n = v.u().expect("count");
                let w = price(k, b, r).unwrap_or_else(|| panic!("no price for {k}"));
                sum += n * w;
                rows.push((k.clone(), n, w, n * w));
            }
        }
        let exact = sum == recorded;
        all_exact &= exact;
        let (res, acc, total) = residuals.get(mode).cloned().unwrap_or((None, None, None));
        let mut prim = String::new();
        for (i, (k, n, w, t)) in rows.iter().enumerate() {
            if i > 0 {
                prim.push(',');
            }
            prim.push_str(&format!(
                "\"{k}\":{{\"count\":{n},\"bitops_each\":{w},\"bitops\":{t},\"share_percent\":{},\"per_residual\":{}}}",
                pct(*t, sum),
                res.map_or("null".into(), |x| format!("{:.3}", *n as f64 / x as f64))
            ));
        }
        arms.push(format!(
            "{{\"mode\":\"{mode}\",\"descent_residuals\":{},\"accepted_descents\":{},\"recorded_descent_bitops\":{recorded},\"repriced_descent_bitops\":{sum},\"reprice_exact\":{exact},\"descent_bitops_per_residual\":{},\"descent_share_of_observed_subtotal_percent\":{},\"primitives\":{{{prim}}}}}",
            res.map_or("null".into(), |x| x.to_string()),
            acc.map_or("null".into(), |x| x.to_string()),
            res.map_or("null".into(), |x| format!("{:.1}", sum as f64 / x as f64)),
            total.map_or("null".into(), |t| pct(sum, t)),
        ));
    }
    // Legacy stage counters of the original-IC descent (same nagao_decompose).
    let w = raw.get("work");
    let g = |k: &str| w.get(k).u().unwrap_or_else(|| panic!("missing work.{k}"));
    let stages = [
        ("f4_groebner", g("descent_f4_muls")),
        ("linear_stage_quotient_to_points", g("descent_lin_muls")),
        ("weil_system_construction", g("descent_weil_muls")),
        ("sextic_roots_ordinates_signs", g("descent_post_muls")),
        ("residual_stream", g("descent_stream_muls")),
        ("decomposition_verification", g("descent_verify_muls")),
    ];
    let legacy_total = g("descent_muls");
    let stage_sum: u128 = stages.iter().map(|s| s.1).sum();
    let mut st = String::new();
    for (i, (k, v)) in stages.iter().enumerate() {
        if i > 0 {
            st.push(',');
        }
        st.push_str(&format!("\"{k}\":{{\"fp_muls\":{v},\"share_percent\":{}}}", pct(*v, legacy_total)));
    }
    let observed_mul = q
        .get("outcomes")
        .arr()
        .iter()
        .find(|o| o.get("mode").s() == "original-ic")
        .and_then(|o| {
            o.get("proof").get("accounting").get("phases").arr().iter().find(|p| p.get("phase").s() == "descent").cloned()
        })
        .and_then(|d| d.get("primitives").get("field_mul").u())
        .expect("observed descent field_mul");
    let without_lin = stage_sum - g("descent_lin_muls");
    let res = g("descent_residuals");
    let succ = g("descent_successes");
    println!(
        "{{\"schema\":\"descent-finder.breakdown/v1\",\"pcc\":\"unverified\",\"inputs\":[\"{dir}/qualified-original.json\",\"{dir}/run-study.json\",\"{dir}/original-ic.raw.json\"],\
\"field_bits\":{b},\"r_bits\":{r},\"all_reprices_exact\":{all_exact},\"arms\":[{}],\
\"original_ic_legacy_stages\":{{\"descent_muls\":{legacy_total},\"stage_sum\":{stage_sum},\"stage_sum_equals_total\":{},\"stages\":{{{st}}},\
\"residuals\":{res},\"successes\":{succ},\"failed_residual_share_percent\":{},\"lin_fallbacks\":{},\"stopped_at_staircase\":{},\"replayed\":{},\"trace_mismatches\":{}}},\
\"counter_reconciliation\":{{\"observed_hooked_descent_field_mul\":{observed_mul},\"legacy_without_linear_stage\":{without_lin},\"legacy_with_linear_stage\":{stage_sum},\
\"gap_observed_minus_without_lin\":{},\"gap_with_lin_minus_observed\":{},\"note\":\"if the hooked field_mul omits the linear stage, recorded descent bitops undercount by about lin_muls*fp_mul\",\
\"implied_unhooked_linear_stage_bitops_if_omitted\":{}}}}}",
        arms.join(","),
        stage_sum == legacy_total,
        pct(res - succ, res),
        g("descent_lin_fallback"),
        g("descent_stopped"),
        g("descent_replayed"),
        g("descent_trace_mismatches"),
        observed_mul as i128 - without_lin as i128,
        stage_sum as i128 - observed_mul as i128,
        g("descent_lin_muls") * fp_mul(b),
    );
}
