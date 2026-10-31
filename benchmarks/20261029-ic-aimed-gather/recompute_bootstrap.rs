#![allow(dead_code)]
use std::collections::BTreeMap;
use std::process::exit;
// ------------------------------------------------------------------ JSON

#[derive(Debug, Clone)]
enum Json {
    Null,
    Bool(bool),
    Num(String),
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
}

impl Json {
    fn get(&self, k: &str) -> Option<&Json> {
        match self {
            Json::Obj(v) => v.iter().find(|(kk, _)| kk == k).map(|(_, x)| x),
            _ => None,
        }
    }
    fn at(&self, i: usize) -> Option<&Json> {
        match self {
            Json::Arr(v) => v.get(i),
            _ => None,
        }
    }
    fn path(&self, p: &[&str]) -> Option<&Json> {
        let mut cur = self;
        for k in p {
            cur = match k.parse::<usize>() {
                Ok(i) if matches!(cur, Json::Arr(_)) => cur.at(i)?,
                _ => cur.get(k)?,
            };
        }
        Some(cur)
    }
    fn str(&self) -> Option<&str> {
        if let Json::Str(s) = self { Some(s) } else { None }
    }
    fn arr(&self) -> Option<&Vec<Json>> {
        if let Json::Arr(v) = self { Some(v) } else { None }
    }
    fn u128(&self) -> Option<u128> {
        match self {
            Json::Num(s) => s.parse().ok(),
            Json::Str(s) => parse_int(s),
            _ => None,
        }
    }
    fn i128(&self) -> Option<i128> {
        match self {
            Json::Num(s) | Json::Str(s) => s.parse().ok(),
            _ => None,
        }
    }
    fn bool(&self) -> Option<bool> {
        if let Json::Bool(b) = self { Some(*b) } else { None }
    }
}

fn parse_int(s: &str) -> Option<u128> {
    if let Some(h) = s.strip_prefix("0x") {
        u128::from_str_radix(h, 16).ok()
    } else {
        s.parse().ok()
    }
}

struct Parser<'a> {
    b: &'a [u8],
    i: usize,
}

impl<'a> Parser<'a> {
    fn ws(&mut self) {
        while self.i < self.b.len() && matches!(self.b[self.i], b' ' | b'\n' | b'\r' | b'\t') {
            self.i += 1;
        }
    }
    fn err<T>(&self, m: &str) -> Result<T, String> {
        Err(format!("JSON: {m} at byte {}", self.i))
    }
    fn expect(&mut self, c: u8) -> Result<(), String> {
        self.ws();
        if self.b.get(self.i) == Some(&c) {
            self.i += 1;
            Ok(())
        } else {
            self.err(&format!("expected '{}'", c as char))
        }
    }
    fn lit(&mut self, s: &str, v: Json) -> Result<Json, String> {
        if self.b[self.i..].starts_with(s.as_bytes()) {
            self.i += s.len();
            Ok(v)
        } else {
            self.err("bad literal")
        }
    }
    fn hex4(&mut self) -> Result<u32, String> {
        let h = self.b.get(self.i..self.i + 4).ok_or("JSON: short \\u escape")?;
        self.i += 4;
        u32::from_str_radix(std::str::from_utf8(h).map_err(|e| e.to_string())?, 16).map_err(|e| e.to_string())
    }
    fn string(&mut self) -> Result<String, String> {
        self.expect(b'"')?;
        let mut out: Vec<u8> = Vec::new();
        loop {
            let c = *self.b.get(self.i).ok_or("JSON: unterminated string")?;
            self.i += 1;
            match c {
                b'"' => break,
                b'\\' => {
                    let e = *self.b.get(self.i).ok_or("JSON: bad escape")?;
                    self.i += 1;
                    match e {
                        b'"' => out.push(b'"'),
                        b'\\' => out.push(b'\\'),
                        b'/' => out.push(b'/'),
                        b'b' => out.push(8),
                        b'f' => out.push(12),
                        b'n' => out.push(b'\n'),
                        b'r' => out.push(b'\r'),
                        b't' => out.push(b'\t'),
                        b'u' => {
                            let mut cp = self.hex4()?;
                            if (0xD800..0xDC00).contains(&cp) && self.b[self.i..].starts_with(b"\\u") {
                                self.i += 2;
                                let lo = self.hex4()?;
                                cp = 0x10000 + ((cp - 0xD800) << 10) + (lo.wrapping_sub(0xDC00) & 0x3FF);
                            }
                            let ch = char::from_u32(cp).unwrap_or('\u{FFFD}');
                            let mut buf = [0u8; 4];
                            out.extend_from_slice(ch.encode_utf8(&mut buf).as_bytes());
                        }
                        _ => return self.err("bad escape"),
                    }
                }
                _ => out.push(c),
            }
        }
        String::from_utf8(out).map_err(|e| e.to_string())
    }
    fn value(&mut self) -> Result<Json, String> {
        self.ws();
        match self.b.get(self.i) {
            Some(b'{') => {
                self.i += 1;
                let mut v = Vec::new();
                self.ws();
                if self.b.get(self.i) == Some(&b'}') {
                    self.i += 1;
                    return Ok(Json::Obj(v));
                }
                loop {
                    self.ws();
                    let k = self.string()?;
                    self.expect(b':')?;
                    let x = self.value()?;
                    v.push((k, x));
                    self.ws();
                    match self.b.get(self.i) {
                        Some(b',') => self.i += 1,
                        Some(b'}') => {
                            self.i += 1;
                            return Ok(Json::Obj(v));
                        }
                        _ => return self.err("expected ',' or '}'"),
                    }
                }
            }
            Some(b'[') => {
                self.i += 1;
                let mut v = Vec::new();
                self.ws();
                if self.b.get(self.i) == Some(&b']') {
                    self.i += 1;
                    return Ok(Json::Arr(v));
                }
                loop {
                    v.push(self.value()?);
                    self.ws();
                    match self.b.get(self.i) {
                        Some(b',') => self.i += 1,
                        Some(b']') => {
                            self.i += 1;
                            return Ok(Json::Arr(v));
                        }
                        _ => return self.err("expected ',' or ']'"),
                    }
                }
            }
            Some(b'"') => Ok(Json::Str(self.string()?)),
            Some(b't') => self.lit("true", Json::Bool(true)),
            Some(b'f') => self.lit("false", Json::Bool(false)),
            Some(b'n') => self.lit("null", Json::Null),
            Some(_) => {
                let s = self.i;
                while self.i < self.b.len() && matches!(self.b[self.i], b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9') {
                    self.i += 1;
                }
                if s == self.i {
                    return self.err("unexpected byte");
                }
                Ok(Json::Num(String::from_utf8_lossy(&self.b[s..self.i]).into_owned()))
            }
            None => self.err("unexpected end"),
        }
    }
}

fn load(path: &str) -> Json {
    let b = std::fs::read(path).unwrap_or_else(|e| die(&format!("{path}: {e}")));
    let mut p = Parser { b: &b, i: 0 };
    let v = p.value().unwrap_or_else(|e| die(&format!("{path}: {e}")));
    p.ws();
    if p.i != b.len() {
        die(&format!("{path}: trailing data"));
    }
    v
}

fn die(m: &str) -> ! {
    eprintln!("verify_scalars: {m}");
    exit(2)
}

fn jstr(s: &str) -> String {
    let mut o = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}


// ------------------------------------------------------------------ bootstrap recompute
// Validator's native recompute of research/ic_aimed_gather_20261023: the as-run
// summary.json of 2026-10-29 and summary-corrected.json (v2) of both sessions.
// Usage: recompute_bootstrap <study dir>   (prints JSON)

fn f64v(j: &Json) -> f64 {
    match j {
        Json::Num(s) => s.parse().unwrap_or_else(|_| die("bad number")),
        _ => die("not a number"),
    }
}

fn med(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = s.len();
    if n % 2 == 1 { s[n / 2] } else { (s[n / 2 - 1] + s[n / 2]) / 2.0 }
}

struct Run { collect: f64, total: f64, status: String, all_verified: bool, recovered: String }

fn per_run(p: &str) -> Run {
    let d = load(p);
    let reps = d.get("repetitions").and_then(|v| v.arr()).unwrap_or_else(|| die("repetitions"));
    let c: Vec<f64> = reps.iter().map(|r| f64v(r.path(&["phases_ns", "collect"]).unwrap())).collect();
    let t: Vec<f64> = reps.iter().map(|r| f64v(r.get("total_ns").unwrap())).collect();
    let rec = d.get("recovered").and_then(|v| v.arr()).map(|v| v.iter().map(|x| x.str().unwrap_or("?").to_string()).collect::<Vec<_>>().join(",")).unwrap_or_default();
    Run { collect: med(&c), total: med(&t), status: d.get("status").and_then(|v| v.str()).unwrap_or("").to_string(),
          all_verified: d.get("all_verified").and_then(|v| v.bool()) == Some(true), recovered: rec }
}

#[derive(Clone, Copy, PartialEq)]
enum Mode { HighBit, Stratified, AsRun }

const A: u64 = 1103515245;
const C: u64 = 12345;
const M: u64 = 2147483648;

fn draws(seed: u64, count: usize, n: usize, mode: Mode) -> Vec<usize> {
    let mut out = Vec::with_capacity(count);
    match mode {
        Mode::AsRun => {
            // jq: (.s * 1103515245 + 12345) % 2147483648 in IEEE-754 doubles; '%' truncates to an integer first.
            let mut s = seed as f64;
            for _ in 0..count {
                let p = s * (A as f64) + (C as f64);
                s = ((p as i64) % (M as i64)) as f64;
                out.push(((s as i64) % (n as i64)) as usize);
            }
        }
        _ => {
            let mut s = seed;
            for _ in 0..count {
                s = (s * A + C) % M; // s < 2^31, s*A < 2^62: exact in u64
                out.push(if mode == Mode::HighBit { ((s * n as u64) >> 31) as usize } else { (s % n as u64) as usize });
            }
        }
    }
    out
}

struct Boot { lo: f64, hi: f64, odd_share: f64, odd_min: usize, odd_max: usize }

fn boot(xs: &[f64], seed: u64, mode: Mode) -> Boot {
    let n = xs.len();
    let idx = draws(seed, 2000 * n, n, mode);
    let mut meds: Vec<f64> = Vec::with_capacity(2000);
    let (mut omin, mut omax) = (usize::MAX, 0usize);
    for b in 0..2000 {
        let w = &idx[b * n..(b + 1) * n];
        let v: Vec<f64> = w.iter().map(|&i| xs[i]).collect();
        meds.push(med(&v));
        let odd = w.iter().filter(|&&i| i % 2 == 0).count(); // 0-based even index = odd-numbered round
        omin = omin.min(odd);
        omax = omax.max(odd);
    }
    meds.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let share = idx.iter().filter(|&&i| i % 2 == 0).count() as f64 / idx.len() as f64;
    Boot { lo: meds[50], hi: meds[1949], odd_share: share, odd_min: omin, odd_max: omax }
}

fn num(x: f64) -> String {
    if x.is_finite() { format!("{x:?}") } else { die("non-finite") }
}

fn summ(xs: &[f64], seed: u64) -> (String, [Boot; 3]) {
    let hb = boot(xs, seed, Mode::HighBit);
    let st = boot(xs, seed, Mode::Stratified);
    let ar = boot(xs, seed, Mode::AsRun);
    let s = format!(
        "{{\"median\": {}, \"n\": {}, \"above_one\": {}, \"ci95\": {{\"lo\": {}, \"hi\": {}}}, \"ci95_stratified\": {{\"lo\": {}, \"hi\": {}}}, \"ci95_as_run\": {{\"lo\": {}, \"hi\": {}}}, \"as_run_odd_round_share\": {}}}",
        num(med(xs)), xs.len(), xs.iter().filter(|&&x| x > 1.0).count(), num(hb.lo), num(hb.hi), num(st.lo), num(st.hi),
        num(ar.lo), num(ar.hi), num(ar.odd_share));
    (s, [hb, st, ar])
}

fn session(xs: &[(&str, Vec<f64>)], seed: u64, extra: &mut Vec<String>, tag: &str) -> String {
    let mut parts = Vec::new();
    let mut b: Vec<(&str, [Boot; 3])> = Vec::new();
    for (k, v) in xs {
        let (s, bt) = summ(v, seed);
        parts.push(format!("\"{k}\": {s}"));
        b.push((k, bt));
    }
    let g = |k: &str| &b.iter().find(|x| x.0 == k).unwrap().1;
    for m in ["collect", "total"] {
        let (ab, aa) = (g(&format!("ab_{m}")), g(&format!("aa_{m}")));
        for (sfx, i) in [("", 0usize), ("_stratified", 1), ("_as_run", 2)] {
            let est = ab[i].lo > 1.0 && ab[i].lo > aa[i].hi;
            parts.push(format!("\"established_{m}{sfx}\": {est}"));
        }
        parts.push(format!("\"margin_{m}\": {}", num(ab[0].lo - aa[0].hi)));
    }
    // ci95 vs ci95_stratified: largest bound shift, verdict equality; odd-count range (high-bit index)
    let mut shift = 0f64;
    for (_, bt) in &b {
        shift = shift.max((bt[0].lo - bt[1].lo).abs()).max((bt[0].hi - bt[1].hi).abs());
    }
    let mut same = true;
    for m in ["collect", "total"] {
        let (ab, aa) = (g(&format!("ab_{m}")), g(&format!("aa_{m}")));
        same &= (ab[0].lo > 1.0 && ab[0].lo > aa[0].hi) == (ab[1].lo > 1.0 && ab[1].lo > aa[1].hi);
    }
    let abc = g("ab_collect");
    extra.push(format!(
        "\"{tag}\": {{\"max_bound_shift_ci95_vs_stratified\": {}, \"verdicts_same_ci95_vs_stratified\": {same}, \"ab_n\": {}, \"odd_count_per_resample_highbit\": [{}, {}], \"odd_count_per_resample_stratified\": [{}, {}], \"odd_count_per_resample_as_run\": [{}, {}]}}",
        num(shift), xs[0].1.len(), abc[0].odd_min, abc[0].odd_max, abc[1].odd_min, abc[1].odd_max, abc[2].odd_min, abc[2].odd_max));
    format!("{{{}}}", parts.join(", "))
}

fn main() {
    let study = std::env::args().nth(1).unwrap_or_else(|| die("usage: recompute_bootstrap <study dir>"));
    let aa2 = format!("{study}/aa2_20261029");
    let mut extra = Vec::new();

    // 2026-10-23
    let ab: Vec<(Run, Run)> = (1..=30).map(|r| (per_run(&format!("{study}/runs/time/ab/baseline-r{r}.json")),
                                                per_run(&format!("{study}/runs/time/ab/candidate-r{r}.json")))).collect();
    let aa: Vec<(Run, Run)> = (1..=5).map(|k| (per_run(&format!("{study}/runs/time/aa/baseline-r{}.json", 2 * k - 1)),
                                               per_run(&format!("{study}/runs/time/aa/baseline-r{}.json", 2 * k)))).collect();
    let s1 = vec![
        ("ab_collect", ab.iter().map(|(b, c)| b.collect / c.collect).collect::<Vec<_>>()),
        ("aa_collect", aa.iter().map(|(x, y)| x.collect / y.collect).collect()),
        ("ab_total", ab.iter().map(|(b, c)| b.total / c.total).collect()),
        ("aa_total", aa.iter().map(|(x, y)| x.total / y.total).collect()),
    ];
    let j1 = session(&s1, 20261023, &mut extra, "2026-10-23");

    // 2026-10-29
    let rows: Vec<(Run, Run, Run)> = (1..=42).map(|r| (per_run(&format!("{aa2}/runs/a-r{r}.json")),
        per_run(&format!("{aa2}/runs/q-r{r}.json")), per_run(&format!("{aa2}/runs/c-r{r}.json")))).collect();
    let s2 = vec![
        ("ab_collect", rows.iter().map(|(a, _, c)| a.collect / c.collect).collect::<Vec<_>>()),
        ("aa_collect", rows.iter().map(|(a, q, _)| a.collect / q.collect).collect()),
        ("ab_total", rows.iter().map(|(a, _, c)| a.total / c.total).collect()),
        ("aa_total", rows.iter().map(|(a, q, _)| a.total / q.total).collect()),
    ];
    let j2 = session(&s2, 20261029, &mut extra, "2026-10-29");

    // as-run summary.json claims of 2026-10-29 (rounds kept by the isolation records)
    let iso_txt = std::fs::read_to_string(format!("{aa2}/runs/isolation.jsonl")).unwrap_or_else(|e| die(&e.to_string()));
    let mut iso: BTreeMap<String, (bool, bool)> = BTreeMap::new(); // label -> (contended, exit != 0); last wins
    let (mut n_iso, mut n_cont, mut n_exit) = (0, 0, 0);
    for line in iso_txt.lines().filter(|l| !l.trim().is_empty()) {
        let mut p = Parser { b: line.as_bytes(), i: 0 };
        let r = p.value().unwrap_or_else(|e| die(&e));
        let lab = r.get("label").and_then(|v| v.str()).unwrap_or("").trim_start_matches("aa2/").to_string();
        let cont = r.path(&["run", "contended"]).and_then(|v| v.bool()) != Some(false);
        let ex = r.path(&["run", "exit_status"]).and_then(|v| v.u128()) != Some(0);
        n_iso += 1;
        n_cont += cont as usize;
        n_exit += ex as usize;
        iso.insert(lab, (cont, ex));
    }
    let kept: Vec<usize> = (1..=42).filter(|r| ["a", "q", "c"].iter().all(|l| iso.get(&format!("{l}-r{r}")) == Some(&(false, false)))).collect();
    let pick = |f: &dyn Fn(&(Run, Run, Run)) -> f64| kept.iter().map(|&r| f(&rows[r - 1])).collect::<Vec<f64>>();
    let k_abc = pick(&|x| x.0.collect / x.2.collect);
    let k_aac = pick(&|x| x.0.collect / x.1.collect);
    let k_abt = pick(&|x| x.0.total / x.2.total);
    let k_aat = pick(&|x| x.0.total / x.1.total);
    let mut asrun: Vec<String> = Vec::new();
    let mut put = |k: &str, v: String| asrun.push(format!("\"{k}\": {v}"));
    let mut est = [false; 2];
    for (name, xs, i) in [("collect_ab", &k_abc, 0usize), ("collect_aa", &k_aac, 0), ("total_ab", &k_abt, 1), ("total_aa", &k_aat, 1)] {
        let b = boot(xs, 20261029, Mode::AsRun);
        put(&format!("{name}_median"), num(med(xs)));
        put(&format!("{name}_ci95_lo"), num(b.lo));
        put(&format!("{name}_ci95_hi"), num(b.hi));
        put(&format!("{name}_rounds_above_one"), xs.iter().filter(|&&x| x > 1.0).count().to_string());
        let _ = i;
    }
    for (m, abv, aav, i) in [("collect", &k_abc, &k_aac, 0usize), ("total", &k_abt, &k_aat, 1)] {
        let (ab, aa) = (boot(abv, 20261029, Mode::AsRun), boot(aav, 20261029, Mode::AsRun));
        est[i] = ab.lo > 1.0 && ab.lo > aa.hi;
        put(&format!("established_{m}"), (est[i] as u8).to_string());
    }
    let all: Vec<&Run> = rows.iter().flat_map(|(a, q, c)| [a, q, c]).collect();
    let ident = all.iter().all(|r| r.status == "complete" && r.all_verified && r.recovered == "518509155375");
    put("rounds_complete", "42".into());
    put("rounds_kept", kept.len().to_string());
    put("rounds_dropped", (42 - kept.len()).to_string());
    put("processes", all.len().to_string());
    put("isolation_records", n_iso.to_string());
    put("contended_processes", n_cont.to_string());
    put("nonzero_exit_processes", n_exit.to_string());
    let refusals: String = std::fs::read_to_string(format!("{aa2}/runs/refusals.count")).unwrap_or_default().trim().to_string();
    let log_lines = std::fs::read_to_string(format!("{aa2}/runs/refusals.log")).unwrap_or_default().lines().filter(|l| l.contains("machine is busy")).count();
    put("refused_then_retried", if refusals.parse::<usize>().ok() == Some(log_lines) { refusals } else { "-1".into() });
    put("identity_ok", (ident as u8).to_string());
    put("processes_recovering_518509155375_all_verified", all.iter().filter(|r| r.status == "complete" && r.all_verified && r.recovered == "518509155375").count().to_string());
    for (li, l) in ["a", "q", "c"].iter().enumerate() {
        let sel: Vec<&Run> = rows.iter().map(|t| match li { 0 => &t.0, 1 => &t.1, _ => &t.2 }).collect();
        put(&format!("median_ms_collect_{l}"), num(med(&sel.iter().map(|r| r.collect).collect::<Vec<_>>()) / 1e6));
        put(&format!("median_ms_total_{l}"), num(med(&sel.iter().map(|r| r.total).collect::<Vec<_>>()) / 1e6));
    }
    println!("{{\"sessions\": {{\"2026-10-23\": {j1}, \"2026-10-29\": {j2}}},\n \"derived\": {{{}}},\n \"as_run_summary_2026_10_29\": {{{}}}}}",
             extra.join(", "), asrun.join(", "));
}
