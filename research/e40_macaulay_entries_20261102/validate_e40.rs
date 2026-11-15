//! E40 independent recompute (validator, 2026-11-02).
//!
//! Standard library only: its own JSON parser and SHA-256.  Shares no code
//! with `examples/e40_*.rs` or any producer script.  Reads the committed raw
//! records of a snapshot export and the claims printed in note §9.50/§9.51,
//! recomputes every headline number from the per-run and per-call fields, and
//! writes one JSON report.
//!
//! ```text
//! rustc -O -o validate_e40 validate_e40.rs
//! ./validate_e40 <snapshot root> <out.json>
//! ```
//!
//! Definitions (note §9.50, "Band"): `S_X(g) = total_muls + 97 g cells_X`,
//! cells = `macaulay_entries` (+ `pivot_selection_ops`, 0 for A) for A and
//! E25, `macaulay_stored_entries + profile_offset_ops` for V;
//! `rho(g) = rho + 97 g steps`, rho = walk (iii): the mean over the 16
//! batched walks of `fp_muls - verify_fp_muls` (E25/E27 convention, the
//! `[d]G = Q` check excluded), steps = their mean steps.  g in {0, 0.1, 1}.
//! σ1 adds the lead-scan reads (A's for A and E25, V's for V); σ4 adds the
//! per-call structural work of Stage 0 (`enumerated_monomials + cols +
//! selection_tests + term_writes`, per-curve means) times the calls.

use std::fmt::Write as _;
use std::fs;

// ───────────────────────────── JSON ─────────────────────────────

#[derive(Clone, Debug, PartialEq)]
enum J {
    Null,
    B(bool),
    N(String),
    S(String),
    A(Vec<J>),
    O(Vec<(String, J)>),
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
    fn hex4(&mut self) -> u32 {
        let h = std::str::from_utf8(&self.s[self.i..self.i + 4]).unwrap();
        self.i += 4;
        u32::from_str_radix(h, 16).unwrap()
    }
    fn string(&mut self) -> String {
        assert_eq!(self.s[self.i], b'"');
        self.i += 1;
        let mut out: Vec<u8> = Vec::new();
        loop {
            let c = self.s[self.i];
            self.i += 1;
            match c {
                b'"' => break,
                b'\\' => {
                    let e = self.s[self.i];
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
                            let mut cp = self.hex4();
                            if (0xD800..0xDC00).contains(&cp) {
                                assert_eq!(&self.s[self.i..self.i + 2], b"\\u");
                                self.i += 2;
                                let lo = self.hex4();
                                cp = 0x10000 + ((cp - 0xD800) << 10) + (lo - 0xDC00);
                            }
                            let ch = char::from_u32(cp).unwrap();
                            let mut buf = [0u8; 4];
                            out.extend_from_slice(ch.encode_utf8(&mut buf).as_bytes());
                        }
                        _ => panic!("bad escape"),
                    }
                }
                _ => out.push(c),
            }
        }
        String::from_utf8(out).unwrap()
    }
    fn value(&mut self) -> J {
        self.ws();
        match self.s[self.i] {
            b'{' => {
                self.i += 1;
                let mut v = Vec::new();
                self.ws();
                if self.s[self.i] == b'}' {
                    self.i += 1;
                    return J::O(v);
                }
                loop {
                    self.ws();
                    let k = self.string();
                    self.ws();
                    assert_eq!(self.s[self.i], b':');
                    self.i += 1;
                    let x = self.value();
                    v.push((k, x));
                    self.ws();
                    match self.s[self.i] {
                        b',' => self.i += 1,
                        b'}' => {
                            self.i += 1;
                            return J::O(v);
                        }
                        c => panic!("object: unexpected {}", c as char),
                    }
                }
            }
            b'[' => {
                self.i += 1;
                let mut v = Vec::new();
                self.ws();
                if self.s[self.i] == b']' {
                    self.i += 1;
                    return J::A(v);
                }
                loop {
                    v.push(self.value());
                    self.ws();
                    match self.s[self.i] {
                        b',' => self.i += 1,
                        b']' => {
                            self.i += 1;
                            return J::A(v);
                        }
                        c => panic!("array: unexpected {}", c as char),
                    }
                }
            }
            b'"' => J::S(self.string()),
            b't' => {
                self.i += 4;
                J::B(true)
            }
            b'f' => {
                self.i += 5;
                J::B(false)
            }
            b'n' => {
                self.i += 4;
                J::Null
            }
            _ => {
                let st = self.i;
                while self.i < self.s.len()
                    && matches!(
                        self.s[self.i],
                        b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9'
                    )
                {
                    self.i += 1;
                }
                J::N(String::from_utf8(self.s[st..self.i].to_vec()).unwrap())
            }
        }
    }
}

fn parse(bytes: &[u8]) -> J {
    let mut p = Parser { s: bytes, i: 0 };
    let v = p.value();
    p.ws();
    assert_eq!(p.i, bytes.len(), "trailing data");
    v
}

impl J {
    fn get(&self, k: &str) -> Option<&J> {
        match self {
            J::O(v) => v.iter().find(|(a, _)| a == k).map(|(_, x)| x),
            _ => None,
        }
    }
    fn at(&self, k: &str) -> &J {
        self.get(k).unwrap_or_else(|| panic!("missing key {k}"))
    }
    fn f(&self) -> f64 {
        match self {
            J::N(s) => s.parse().unwrap(),
            _ => panic!("not a number: {self:?}"),
        }
    }
    fn u(&self) -> u64 {
        match self {
            J::N(s) => s.parse().unwrap_or_else(|_| panic!("not an integer: {s}")),
            _ => panic!("not a number: {self:?}"),
        }
    }
    fn b(&self) -> bool {
        match self {
            J::B(b) => *b,
            _ => panic!("not a bool"),
        }
    }
    fn s(&self) -> &str {
        match self {
            J::S(s) => s,
            _ => panic!("not a string"),
        }
    }
    fn arr(&self) -> &Vec<J> {
        match self {
            J::A(v) => v,
            _ => panic!("not an array"),
        }
    }
    fn fk(&self, k: &str) -> f64 {
        self.at(k).f()
    }
    fn uk(&self, k: &str) -> u64 {
        self.at(k).u()
    }
}

/// Paths (`a.b[3].c`) where `x` and `y` differ, skipping keys `skip` allows.
fn diff(x: &J, y: &J, path: &str, skip: &dyn Fn(&str) -> bool, out: &mut Vec<String>) {
    match (x, y) {
        (J::O(a), J::O(b)) => {
            let mut keys: Vec<&String> = a
                .iter()
                .map(|(k, _)| k)
                .chain(b.iter().map(|(k, _)| k))
                .collect();
            keys.sort();
            keys.dedup();
            for k in keys {
                if skip(k) {
                    continue;
                }
                let p = format!("{path}.{k}");
                match (x.get(k), y.get(k)) {
                    (Some(u), Some(v)) => diff(u, v, &p, skip, out),
                    _ => out.push(p),
                }
            }
        }
        (J::A(a), J::A(b)) => {
            if a.len() != b.len() {
                out.push(format!("{path}[len]"));
                return;
            }
            for (i, (u, v)) in a.iter().zip(b).enumerate() {
                diff(u, v, &format!("{path}[{i}]"), skip, out);
            }
        }
        _ => {
            if x != y {
                out.push(path.to_string());
            }
        }
    }
}

fn count_leaves(x: &J, skip: &dyn Fn(&str) -> bool) -> u64 {
    match x {
        J::O(v) => v
            .iter()
            .filter(|(k, _)| !skip(k))
            .map(|(_, y)| count_leaves(y, skip))
            .sum(),
        J::A(v) => v.iter().map(|y| count_leaves(y, skip)).sum(),
        _ => 1,
    }
}

fn timing(k: &str) -> bool {
    k.ends_with("_ms") || k == "ms" || k.contains("wall")
}

/// §9.50's allowed fields for V against A on the same row and stream.
fn allowed(k: &str) -> bool {
    timing(k)
        || matches!(
            k,
            "solver" | "lead_scan_reads" | "macaulay_stored_entries" | "profile_offset_ops"
        )
}

// ─────────────────────────── SHA-256 ───────────────────────────

fn sha256(data: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    let mut msg = data.to_vec();
    let bits = (data.len() as u64).wrapping_mul(8);
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bits.to_be_bytes());
    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for t in 0..16 {
            w[t] = u32::from_be_bytes([
                chunk[4 * t],
                chunk[4 * t + 1],
                chunk[4 * t + 2],
                chunk[4 * t + 3],
            ]);
        }
        for t in 16..64 {
            let s0 = w[t - 15].rotate_right(7) ^ w[t - 15].rotate_right(18) ^ (w[t - 15] >> 3);
            let s1 = w[t - 2].rotate_right(17) ^ w[t - 2].rotate_right(19) ^ (w[t - 2] >> 10);
            w[t] = w[t - 16]
                .wrapping_add(s0)
                .wrapping_add(w[t - 7])
                .wrapping_add(s1);
        }
        let mut v = h;
        for t in 0..64 {
            let s1 = v[4].rotate_right(6) ^ v[4].rotate_right(11) ^ v[4].rotate_right(25);
            let ch = (v[4] & v[5]) ^ (!v[4] & v[6]);
            let t1 = v[7]
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[t])
                .wrapping_add(w[t]);
            let s0 = v[0].rotate_right(2) ^ v[0].rotate_right(13) ^ v[0].rotate_right(22);
            let maj = (v[0] & v[1]) ^ (v[0] & v[2]) ^ (v[1] & v[2]);
            let t2 = s0.wrapping_add(maj);
            v = [
                t1.wrapping_add(t2),
                v[0],
                v[1],
                v[2],
                v[3].wrapping_add(t1),
                v[4],
                v[5],
                v[6],
            ];
        }
        for i in 0..8 {
            h[i] = h[i].wrapping_add(v[i]);
        }
    }
    h.iter().map(|x| format!("{x:08x}")).collect()
}

// ─────────────────────────── helpers ───────────────────────────

const GAE: f64 = 97.0;
const GS: [f64; 3] = [0.0, 0.1, 1.0];
const GLS: [(u64, u64); 10] = [
    (101, 1),
    (101, 2),
    (149, 1),
    (149, 2),
    (197, 1),
    (197, 2),
    (269, 1),
    (269, 2),
    (397, 1),
    (397, 2),
];
const E24: [(u64, u64); 12] = [
    (101, 1),
    (101, 2),
    (149, 1),
    (149, 2),
    (197, 1),
    (197, 2),
    (269, 1),
    (269, 2),
    (397, 1),
    (397, 2),
    (521, 1),
    (521, 2),
];

struct Ctx {
    root: String,
    raw: Vec<(String, String)>,
}

impl Ctx {
    fn load(&mut self, rel: &str) -> J {
        let bytes =
            fs::read(format!("{}/{}", self.root, rel)).unwrap_or_else(|e| panic!("{rel}: {e}"));
        if !self.raw.iter().any(|(p, _)| p == rel) {
            self.raw.push((rel.to_string(), sha256(&bytes)));
        }
        parse(&bytes)
    }
}

fn mm(v: &[f64]) -> (f64, f64) {
    v.iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), &x| {
            (a.min(x), b.max(x))
        })
}

fn jnum(x: f64) -> String {
    if x.is_finite() && x == x.trunc() && x.abs() < 9.0e15 {
        format!("{}", x as i64)
    } else {
        format!("{x:e}")
    }
}

fn jstr(s: &str) -> String {
    let mut o = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            c if (c as u32) < 0x20 => {
                let _ = write!(o, "\\u{:04x}", c as u32);
            }
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

/// A printed claim: `1.2518`, `102,826`, `5.90·10⁴`; returns (value, half a
/// unit of its last printed digit).
fn printed(s: &str) -> (f64, f64) {
    let s = s.trim().trim_matches('`').replace(',', "");
    let (mant, exp) = match s.split_once("·10") {
        Some((m, e)) => {
            let mut x = 0i32;
            let mut neg = false;
            for c in e.chars() {
                let d = match c {
                    '⁰' => 0,
                    '¹' => 1,
                    '²' => 2,
                    '³' => 3,
                    '⁴' => 4,
                    '⁵' => 5,
                    '⁶' => 6,
                    '⁷' => 7,
                    '⁸' => 8,
                    '⁹' => 9,
                    '⁻' => {
                        neg = true;
                        continue;
                    }
                    _ => panic!("exponent {e}"),
                };
                x = 10 * x + d;
            }
            (m.to_string(), if neg { -x } else { x })
        }
        None => (s.clone(), 0),
    };
    let decimals = mant.split_once('.').map_or(0, |(_, f)| f.len()) as i32;
    let v: f64 = mant
        .parse::<f64>()
        .unwrap_or_else(|_| panic!("number {mant}"))
        * 10f64.powi(exp);
    (v, 0.5 * 10f64.powi(exp - decimals))
}

struct Head {
    out: Vec<String>,
    bad: Vec<String>,
}

impl Head {
    /// One headline: claim as printed, the recomputed value, tolerance =
    /// half a unit of the last printed digit (relative), never below 1e-12.
    fn add(&mut self, name: &str, source: &str, claim: &str, rec: f64, def: &str) {
        let (c, half) = printed(claim);
        let tol = (half / c.abs().max(1e-300)).max(1e-12) * (1.0 + 1e-9);
        let scale = c.abs().max(rec.abs()).max(1e-300);
        let ok = (c - rec).abs() <= tol * scale;
        if !ok {
            self.bad
                .push(format!("{name}: claimed {claim}, recomputed {rec}"));
        }
        self.out.push(format!(
            "{{\"name\": {}, \"source\": {}, \"printed\": {}, \"claimed\": {}, \"recomputed\": {}, \"rel_tol\": {:e}, \"agrees\": {}, \"definition\": {}}}",
            jstr(name), jstr(source), jstr(claim), jnum(c), jnum(rec), tol, ok, jstr(def)
        ));
    }
    fn exact(&mut self, name: &str, source: &str, claim: f64, rec: f64, def: &str) {
        let ok = claim == rec;
        if !ok {
            self.bad
                .push(format!("{name}: claimed {claim}, recomputed {rec}"));
        }
        self.out.push(format!(
            "{{\"name\": {}, \"source\": {}, \"printed\": {}, \"claimed\": {}, \"recomputed\": {}, \"rel_tol\": 0, \"agrees\": {}, \"definition\": {}}}",
            jstr(name), jstr(source), jstr(&jnum(claim)), jnum(claim), jnum(rec), ok, jstr(def)
        ));
    }
}

fn arm_rec<'a>(file: &'a J, arm: Option<&str>) -> &'a J {
    let v = file.arr();
    match arm {
        Some(a) => v.iter().find(|r| r.at("arm").s() == a).unwrap(),
        None => {
            assert_eq!(v.len(), 1);
            &v[0]
        }
    }
}

fn cells(x: &J) -> f64 {
    match x.get("macaulay_stored_entries") {
        Some(st) => st.f() + x.fk("profile_offset_ops"),
        None => x.fk("macaulay_entries") + x.get("pivot_selection_ops").map_or(0.0, |v| v.f()),
    }
}

fn s3(x: &J, extra: f64) -> [f64; 3] {
    let c = cells(x) + extra;
    GS.map(|g| x.fk("total_muls") + GAE * g * c)
}

/// Walk (iii): mean over the batched walks of `fp_muls - verify_fp_muls`, mean steps.
fn walk_iii(walks: &J) -> (f64, f64, bool, usize) {
    let w = walks.arr();
    let n = w.len() as f64;
    let rho = w
        .iter()
        .map(|x| x.fk("fp_muls") - x.fk("verify_fp_muls"))
        .sum::<f64>()
        / n;
    let steps = w.iter().map(|x| x.fk("steps")).sum::<f64>() / n;
    let correct = w.iter().all(|x| x.at("correct").b());
    (rho, steps, correct, w.len())
}

fn order(r: &[f64; 3], x: &[f64; 3]) -> &'static str {
    // r = reference, x = arm
    if (0..3).all(|g| x[g] < r[g]) {
        "ahead"
    } else if x[0] == r[0] && x[1] < r[1] && x[2] < r[2] {
        "equal at 0, ahead at 0.1 and 1"
    } else if (0..3).all(|g| x[g] > r[g]) {
        "behind"
    } else {
        "unresolved"
    }
}

/// `validate_e40 compare <committed.json> <replayed.json>`: every leaf of the
/// two files, timing keys (`*_ms`, `ms`, `*wall*`) skipped; prints
/// `<leaves compared> <differing> <sha256 committed> <sha256 replayed>` and the
/// differing paths.
fn compare(a: &str, b: &str) {
    let (x, y) = (fs::read(a).unwrap(), fs::read(b).unwrap());
    let (jx, jy) = (parse(&x), parse(&y));
    let mut d = Vec::new();
    diff(&jx, &jy, "", &timing, &mut d);
    println!(
        "{} {} {} {}",
        count_leaves(&jx, &timing),
        d.len(),
        sha256(&x),
        sha256(&y)
    );
    for p in d.iter().take(20) {
        println!("  {p}");
    }
}

/// `validate_e40 cert <snapshot root> <replay out dir> <out.json>`: the runs of
/// the replay certificate.  Each replayed output against the committed record
/// it must reproduce: leaves compared (timing keys skipped), leaves differing,
/// both SHA-256, and the scalars the gate reads, taken from the replay.
fn cert(snap: &str, out: &str, dest: &str) {
    let e40 = format!("{snap}/experiments/47_macaulay_entries");
    let e37 = format!("{snap}/experiments/44_pivot_rules_confirm/e2e");
    let mut runs: Vec<(String, String, String, String)> = Vec::new(); // name, kind, committed rel, replay file
    for f in [
        "gls_chain-profile_p101s1",
        "gls_chain-profile_p397s1",
        "gls_chain-profile_p397s2",
        "e24_chain-profile_p269s1",
        "e24_chain-profile_p397s2",
        "e24_chain-profile_p521s2",
    ] {
        runs.push((
            f.into(),
            "end to end, V on E37's stream (nonce-selected)".into(),
            format!("experiments/47_macaulay_entries/e2e/{f}.json"),
            format!("{f}.json"),
        ));
    }
    for f in [
        "gls_chain-counted_E40E0002_p397s1",
        "gls_chain-profile_E40E0002_p397s1",
    ] {
        runs.push((
            f.into(),
            "negative control N, A′ / V′ on E40E0002 (nonce-selected curve)".into(),
            format!("experiments/47_macaulay_entries/e2e/{f}.json"),
            format!("{f}.json"),
        ));
    }
    for f in ["gls_macaulay_p101s1", "e24_macaulay_p521s1"] {
        runs.push((
            f.into(),
            "E25 reference record of E37 (nonce-selected), replayed with the E40 build".into(),
            format!("experiments/44_pivot_rules_confirm/e2e/{f}.json"),
            format!("{f}.json"),
        ));
    }
    for c in [
        "macaulay",
        "chain",
        "chainsparsest",
        "chain_karatsuba",
        "gls_counted_e37",
        "gls_fifo_e37",
        "e24_counted_e37",
    ] {
        runs.push((
            format!("id_{c}"),
            "identity G2.204, existing configuration under the E40 build".into(),
            format!("experiments/47_macaulay_entries/identity/{c}.json"),
            format!("id_{c}.json"),
        ));
    }
    runs.push((
        "checks_v2".into(),
        "controls G2.205 (e40_checks_v2, 320 calls)".into(),
        "experiments/47_macaulay_entries/checks.json".into(),
        "checks_v2.json".into(),
    ));
    runs.push((
        "checks_v1".into(),
        "controls, first harness (e40_checks, kept as disclosed)".into(),
        "experiments/47_macaulay_entries/checks_v1.json".into(),
        "checks_v1.json".into(),
    ));
    runs.push((
        "stage_a".into(),
        "Stage A, all 10 curves x 1,000 calls x 4 solvers".into(),
        "experiments/47_macaulay_entries/stage_a.json".into(),
        "stage_a.json".into(),
    ));
    runs.push((
        "walk_e24".into(),
        "matched rho on E24's curves, walk (iii) (e37_walk_e24, same crypto_lib build)".into(),
        "experiments/44_pivot_rules_confirm/walk_e24.json".into(),
        "walk_e24.json".into(),
    ));
    runs.push((
        "stage_w".into(),
        "matched rho on E25's GLS curves, walk (iii) (e27_walk, same crypto_lib build)".into(),
        "experiments/32_walk_karatsuba/stage_w.json".into(),
        "stage_w.json".into(),
    ));
    let _ = (&e40, &e37);
    let mut o = String::from("[");
    let (mut n_ok, mut n_run, mut leaves_all, mut diff_all) = (0, 0, 0u64, 0u64);
    for (name, kind, rel, rep) in &runs {
        let rp = format!("{out}/{rep}");
        let Ok(yb) = fs::read(&rp) else {
            eprintln!("{name}: not replayed");
            continue;
        };
        let xb = fs::read(format!("{snap}/{rel}")).unwrap();
        let (jx, jy) = (parse(&xb), parse(&yb));
        let mut d = Vec::new();
        diff(&jx, &jy, "", &timing, &mut d);
        let leaves = count_leaves(&jx, &timing);
        let raw_equal = xb == yb;
        n_run += 1;
        n_ok += usize::from(d.is_empty());
        leaves_all += leaves;
        diff_all += d.len() as u64;
        let mut sc = String::new();
        if let J::A(v) = &jy {
            if v.first().is_some_and(|r| r.get("total_muls").is_some()) {
                let parts: Vec<String> = v
                    .iter()
                    .map(|r| {
                        let g = |k: &str| r.get(k).map_or("null".to_string(), |x| match x {
                            J::N(s) => s.clone(),
                            J::B(b) => b.to_string(),
                            J::S(s) => jstr(s),
                            _ => "null".into(),
                        });
                        format!(
                            "{{\"arm\": {}, \"solver\": {}, \"residuals\": {}, \"solver_fallbacks\": {}, \"total_muls\": {}, \"macaulay_entries\": {}, \"macaulay_stored_entries\": {}, \"profile_offset_ops\": {}, \"lead_scan_reads\": {}, \"ic_correct\": {}, \"ic_recovered\": {}, \"walks_correct\": {}}}",
                            g("arm"), g("solver"), g("residuals"), g("solver_fallbacks"), g("total_muls"), g("macaulay_entries"),
                            g("macaulay_stored_entries"), g("profile_offset_ops"), g("lead_scan_reads"),
                            r.at("folded").at("correct").b(),
                            match r.at("folded").get("recovered") {
                                Some(J::N(s)) => s.clone(),
                                _ => "null".into(),
                            },
                            r.at("rho").arr().iter().all(|w| w.at("correct").b())
                        )
                    })
                    .collect();
                sc = format!("[{}]", parts.join(", "));
            }
        }
        if sc.is_empty() {
            sc = "null".into();
        }
        let _ = write!(
            o,
            "{}{{\"run\": {}, \"kind\": {}, \"committed\": {{\"path\": {}, \"sha256\": \"{}\"}}, \"replay_output\": {{\"file\": {}, \"sha256\": \"{}\"}}, \"raw_bytes_equal\": {}, \"fields_compared\": {}, \"fields_identical\": {}, \"differing_paths\": [{}], \"matched\": {}, \"scalars\": {}}}",
            if o.len() > 1 { ", " } else { "" },
            jstr(name), jstr(kind), jstr(&format!("../../{rel}")), sha256(&xb), jstr(rep), sha256(&yb), raw_equal,
            leaves, leaves - d.len() as u64, d.iter().take(20).map(|s| jstr(s)).collect::<Vec<_>>().join(", "),
            d.is_empty(), sc
        );
    }
    o.push(']');
    fs::write(dest, &o).unwrap();
    eprintln!("runs replayed {n_run}, matched {n_ok}; leaves {leaves_all}, differing {diff_all}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args[1] == "cert" {
        return cert(&args[2], &args[3], &args[4]);
    }
    if args[1] == "compare" {
        return compare(&args[2], &args[3]);
    }
    if args[1] == "sha256" {
        for f in &args[2..] {
            println!("{}  {f}", sha256(&fs::read(f).unwrap()));
        }
        return;
    }
    let root = args[1].clone();
    let outp = args[2].clone();
    let mut cx = Ctx {
        root,
        raw: Vec::new(),
    };
    let mut h = Head {
        out: Vec::new(),
        bad: Vec::new(),
    };
    let mut rep = String::new(); // extra JSON sections, comma-led
    let e40 = "experiments/47_macaulay_entries";
    let e37 = "experiments/44_pivot_rules_confirm";
    let n950 = "note §9.50 (Stage 0 table)";

    // ── The note: §9.51's per-row table (claims) ──
    let note = String::from_utf8(
        fs::read(format!(
            "{}/research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md",
            cx.root
        ))
        .unwrap(),
    )
    .unwrap();
    cx.raw.push((
        "research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md".into(),
        sha256(note.as_bytes()),
    ));
    let s951 =
        &note[note.find("### 9.51 E40 measured").unwrap()..note.find("### 9.52 E41").unwrap()];
    let mut table_claims: Vec<(String, u64, u64, String, Vec<String>)> = Vec::new();
    for line in s951.lines() {
        if !(line.starts_with("| GLS |") || line.starts_with("| E24 |")) {
            continue;
        }
        let cols: Vec<String> = line
            .split('|')
            .map(|c| c.trim().to_string())
            .filter(|c| !c.is_empty())
            .collect();
        if cols.len() != 14 {
            continue;
        }
        table_claims.push((
            cols[0].to_lowercase(),
            cols[1].parse().unwrap(),
            cols[2].parse().unwrap(),
            cols[3].clone(),
            cols[4..].to_vec(),
        ));
    }
    assert_eq!(table_claims.len(), 32, "§9.51 per-row table");

    // ── Stage 0 ──
    let s0 = cx.load(&format!("{e40}/stage0.json"));
    let s0f = |k: &str| -> (f64, f64) {
        mm(&s0
            .arr()
            .iter()
            .map(|c| c.at("fields").at(k).fk("mean"))
            .collect::<Vec<_>>())
    };
    let s0min = |k: &str| -> f64 {
        s0.arr()
            .iter()
            .map(|c| c.at("fields").at(k).fk("min"))
            .fold(f64::INFINITY, f64::min)
    };
    let s0max = |k: &str| -> f64 {
        s0.arr()
            .iter()
            .map(|c| c.at("fields").at(k).fk("max"))
            .fold(f64::NEG_INFINITY, f64::max)
    };
    assert_eq!(s0.arr().len(), 22);
    let s0calls: u64 = s0.arr().iter().map(|c| c.uk("calls")).sum();
    h.exact(
        "Stage 0 calls",
        n950,
        4400.0,
        s0calls as f64,
        "Σ calls over the 22 curves of stage0.json",
    );
    let (d0, d1) = s0f("dense_entries");
    h.add(
        "Stage 0 dense cells rows·cols, min curve mean",
        n950,
        "176,064",
        d0,
        "stage0 dense_entries mean",
    );
    h.add(
        "Stage 0 dense cells rows·cols, max curve mean",
        n950,
        "176,064",
        d1,
        "stage0 dense_entries mean",
    );
    h.exact(
        "Stage 0 dense cells, every call (min)",
        n950,
        176064.0,
        s0min("dense_entries"),
        "min over all calls",
    );
    h.exact(
        "Stage 0 dense cells, every call (max)",
        n950,
        176064.0,
        s0max("dense_entries"),
        "max over all calls",
    );
    let (a, b) = s0f("prefix_entries");
    h.add(
        "Stage 0 zero prefix Σ lead_r, min curve mean",
        n950,
        "73,237",
        a,
        "stage0 prefix_entries mean",
    );
    h.add(
        "Stage 0 zero prefix Σ lead_r, max curve mean",
        n950,
        "73,239",
        b,
        "stage0 prefix_entries mean",
    );
    h.add(
        "Stage 0 prefix share",
        n950,
        "0.416",
        b / 176064.0,
        "prefix / dense (41.6 %)",
    );
    let (a, b) = s0f("tail_entries");
    h.add(
        "Stage 0 tail, min curve mean",
        n950,
        "102,825",
        a,
        "stage0 tail_entries mean",
    );
    h.add(
        "Stage 0 tail, max curve mean",
        n950,
        "102,827",
        b,
        "stage0 tail_entries mean",
    );
    let (a, b) = s0f("initial_lead_scan");
    h.add(
        "Stage 0 first lead scan, min",
        n950,
        "73,630",
        a,
        "stage0 initial_lead_scan mean",
    );
    h.add(
        "Stage 0 first lead scan, max",
        n950,
        "73,632",
        b,
        "stage0 initial_lead_scan mean",
    );
    let (a, b) = s0f("lead_scan");
    h.add(
        "Stage 0 all lead-scan reads, min",
        n950,
        "79,114",
        a,
        "stage0 lead_scan mean",
    );
    h.add(
        "Stage 0 all lead-scan reads, max",
        n950,
        "79,116",
        b,
        "stage0 lead_scan mean",
    );
    let (a, b) = s0f("rows");
    h.add("Stage 0 rows, min", n950, "393", a, "stage0 rows mean");
    h.add("Stage 0 rows, max", n950, "393", b, "stage0 rows mean");
    let (a, b) = s0f("cols");
    h.add("Stage 0 cols, min", n950, "448", a, "stage0 cols mean");
    h.add("Stage 0 cols, max", n950, "448", b, "stage0 cols mean");
    let (a, b) = s0f("term_writes");
    h.add(
        "Stage 0 coefficient writes, min curve mean",
        n950,
        "11,154",
        a,
        "stage0 term_writes mean",
    );
    h.add(
        "Stage 0 coefficient writes, max curve mean",
        n950,
        "11,251",
        b,
        "stage0 term_writes mean",
    );
    let (a, b) = s0f("zero_rows");
    h.add(
        "Stage 0 zero rows, min",
        n950,
        "9",
        a,
        "stage0 zero_rows mean",
    );
    h.add(
        "Stage 0 zero rows, max",
        n950,
        "9",
        b,
        "stage0 zero_rows mean",
    );
    let (a, b) = s0f("outside_dependency_tail_entries");
    h.add(
        "Stage 0 tails outside the dependency closure, min",
        n950,
        "16,105",
        a,
        "mean",
    );
    h.add(
        "Stage 0 tails outside the dependency closure, max",
        n950,
        "16,124",
        b,
        "mean",
    );
    let (a, b) = s0f("enumerated_monomials");
    h.add("Stage 0 monomials enumerated, min", n950, "928", a, "mean");
    h.add("Stage 0 monomials enumerated, max", n950, "928", b, "mean");
    let (a, b) = s0f("selection_tests");
    h.add("Stage 0 selection tests, min", n950, "659", a, "mean");
    h.add("Stage 0 selection tests, max", n950, "659", b, "mean");
    let lead_not = s0
        .arr()
        .iter()
        .map(|c| c.at("fields").at("lead_not_lm_shift").fk("max"))
        .fold(0.0, f64::max);
    h.exact(
        "Stage 0 rows whose lead is not LM·shift (max over calls)",
        n950,
        0.0,
        lead_not,
        "lead_not_lm_shift max",
    );
    let struct_of = |set: &str, p: u64, seed: u64| -> f64 {
        let c = s0
            .arr()
            .iter()
            .find(|c| c.at("set").s() == set && c.uk("p") == p && c.uk("seed") == seed)
            .unwrap();
        [
            "enumerated_monomials",
            "cols",
            "selection_tests",
            "term_writes",
        ]
        .iter()
        .map(|k| c.at("fields").at(k).fk("mean"))
        .sum()
    };

    // ── Stage A (per call) ──
    let sa = cx.load(&format!("{e40}/stage_a.json"));
    let mut sa_rows = String::new();
    let (mut vcm, mut eratio, mut ops, mut alead, mut vlead) =
        (vec![], vec![], vec![], vec![], vec![]);
    let (mut r0, mut r1, mut r2, mut s1r1, mut s1r2, mut tail_share) =
        (vec![], vec![], vec![], vec![], vec![], vec![]);
    let (mut vmin, mut vmax) = (u64::MAX, 0u64);
    let (mut a_clean_all, mut calls_total, mut c_equal, mut fb_equal, mut rel_ok, mut rel_n) =
        (true, 0u64, 0u64, 0u64, 0u64, 0u64);
    let (mut fb_cells_equal, mut fb_n, mut fb_total) = (0u64, 0u64, 0u64);
    for c in sa.arr() {
        let arms = c.at("arms").arr();
        assert_eq!(arms[2].at("solver").s(), "chain-counted");
        assert_eq!(arms[3].at("solver").s(), "chain-profile");
        let get =
            |k: usize, f: &str| -> Vec<u64> { arms[k].at(f).arr().iter().map(J::u).collect() };
        let getb =
            |k: usize| -> Vec<bool> { arms[k].at("fallback").arr().iter().map(J::b).collect() };
        let (ca, cv) = (get(2, "call"), get(3, "call"));
        let (ea, ev) = (get(2, "cells"), get(3, "cells"));
        let (oa, ov) = (get(2, "offset_ops"), get(3, "offset_ops"));
        let (la, lv) = (get(2, "lead_scan"), get(3, "lead_scan"));
        let (fa, fv) = (getb(2), getb(3));
        let n = ca.len();
        assert_eq!(n, c.uk("calls") as usize);
        calls_total += n as u64;
        let mut vclean = Vec::new();
        for i in 0..n {
            c_equal += u64::from(ca[i] == cv[i]);
            fb_equal += u64::from(fa[i] == fv[i]);
            if !fa[i] {
                a_clean_all &= ea[i] == 176_064;
            }
            if !fv[i] {
                vclean.push(ev[i]);
                vmin = vmin.min(ev[i]);
                vmax = vmax.max(ev[i]);
                // Independent consistency: stored = rows·cols − Σ lead_r, and
                // Σ lead_r = A counted's lead scans − V's (first scans differ by
                // exactly the prefix; every later scan is shared).
                rel_n += 1;
                rel_ok += u64::from(ev[i] + (la[i] - lv[i]) == 176_064);
            } else {
                fb_n += 1;
                fb_cells_equal += u64::from(ev[i] == ea[i]);
            }
            fb_total += u64::from(fv[i]);
            assert_eq!(oa[i], 0);
        }
        let mean = |v: &[u64]| v.iter().sum::<u64>() as f64 / v.len() as f64;
        let vc = mean(&vclean);
        vcm.push(vc);
        eratio.push(176_064.0 / vc);
        ops.push(mean(&ov));
        alead.push(mean(&la));
        vlead.push(mean(&lv));
        let smean = |call: &[u64], cl: &[u64], op: &[u64], ls: Option<&[u64]>, g: f64| -> f64 {
            (0..n)
                .map(|i| call[i] as f64 + GAE * g * (cl[i] + op[i] + ls.map_or(0, |l| l[i])) as f64)
                .sum::<f64>()
                / n as f64
        };
        let ra: Vec<f64> = GS
            .iter()
            .map(|&g| smean(&ca, &ea, &oa, None, g) / smean(&cv, &ev, &ov, None, g))
            .collect();
        let rs: Vec<f64> = GS
            .iter()
            .map(|&g| smean(&ca, &ea, &oa, Some(&la), g) / smean(&cv, &ev, &ov, Some(&lv), g))
            .collect();
        r0.push(ra[0]);
        r1.push(ra[1]);
        r2.push(ra[2]);
        s1r1.push(rs[1]);
        s1r2.push(rs[2]);
        let share = GAE * mean(&ev) / smean(&cv, &ev, &ov, None, 1.0);
        tail_share.push(share);
        // Not a claim: the same ratio at weights below the band (sensitivity
        // of the size of the gain to the per-cell weight).
        let low: Vec<f64> = [0.001, 0.01]
            .iter()
            .map(|&g| smean(&ca, &ea, &oa, None, g) / smean(&cv, &ev, &ov, None, g))
            .collect();
        let _ = write!(
            sa_rows,
            "{}{{\"p\": {}, \"seed\": {}, \"calls\": {}, \"v_clean_cells_mean\": {}, \"a_over_v_cells\": {}, \"offset_ops_mean\": {}, \"lead_a\": {}, \"lead_v\": {}, \"c_mean\": {}, \"ratio\": [{}, {}, {}], \"ratio_sigma1\": [{}, {}, {}], \"ratio_at_0.001_and_0.01_gae\": [{}, {}], \"fallbacks_v\": {}, \"stored_cells_share_of_S_V_at_1\": {}}}",
            if sa_rows.is_empty() { "" } else { ", " },
            c.uk("p"), c.uk("seed"), n, vc, 176_064.0 / vc, mean(&ov), mean(&la), mean(&lv), mean(&ca),
            ra[0], ra[1], ra[2], rs[0], rs[1], rs[2], low[0], low[1], fv.iter().filter(|&&f| f).count(), share
        );
    }
    let src_a = "note §9.51 (Stage A table)";
    h.exact(
        "Stage A calls",
        src_a,
        10000.0,
        calls_total as f64,
        "Σ calls",
    );
    h.exact(
        "Stage A: A counted clean calls all 176,064 cells",
        src_a,
        1.0,
        f64::from(u8::from(a_clean_all)),
        "1 = true",
    );
    let (a, b) = mm(&vcm);
    h.add(
        "Stage A V stored cells (clean), min curve mean",
        src_a,
        "102,826",
        a,
        "mean over calls without fallback",
    );
    h.add(
        "Stage A V stored cells (clean), max curve mean",
        src_a,
        "102,827",
        b,
        "mean over calls without fallback",
    );
    h.exact(
        "Stage A V stored cells (clean), min call",
        src_a,
        102749.0,
        vmin as f64,
        "min over clean calls",
    );
    h.exact(
        "Stage A V stored cells (clean), max call",
        src_a,
        102827.0,
        vmax as f64,
        "max over clean calls",
    );
    let (a, b) = mm(&eratio);
    h.add(
        "Stage A macaulay_entries / stored (clean), min",
        src_a,
        "1.7122",
        a,
        "176,064 / clean mean",
    );
    h.add(
        "Stage A macaulay_entries / stored (clean), max",
        src_a,
        "1.7123",
        b,
        "176,064 / clean mean",
    );
    let (a, b) = mm(&ops);
    h.add(
        "Stage A V offset operations, min curve mean",
        src_a,
        "22,340",
        a,
        "mean over all calls",
    );
    h.add(
        "Stage A V offset operations, max curve mean",
        src_a,
        "22,489",
        b,
        "mean over all calls",
    );
    let (a, b) = mm(&alead);
    h.add("Stage A lead scans A, min", src_a, "79,114", a, "mean");
    h.add("Stage A lead scans A, max", src_a, "79,117", b, "mean");
    let (a, b) = mm(&vlead);
    h.add("Stage A lead scans V, min", src_a, "5,877", a, "mean");
    h.add("Stage A lead scans V, max", src_a, "5,879", b, "mean");
    let (a, b) = mm(&r0);
    h.add(
        "Stage A S_A/S_V at 0 GAE, min",
        src_a,
        "1.000000",
        a,
        "ratio of per-call means",
    );
    h.add(
        "Stage A S_A/S_V at 0 GAE, max",
        src_a,
        "1.000000",
        b,
        "ratio of per-call means",
    );
    let (a, b) = mm(&r1);
    h.add(
        "Stage A S_A/S_V at 0.1 GAE, min",
        src_a,
        "1.2447",
        a,
        "ratio of per-call means",
    );
    h.add(
        "Stage A S_A/S_V at 0.1 GAE, max",
        src_a,
        "1.2658",
        b,
        "ratio of per-call means",
    );
    let (a, b) = mm(&r2);
    h.add(
        "Stage A S_A/S_V at 1 GAE, min",
        src_a,
        "1.3534",
        a,
        "ratio of per-call means",
    );
    h.add(
        "Stage A S_A/S_V at 1 GAE, max",
        src_a,
        "1.3831",
        b,
        "ratio of per-call means",
    );
    let (a, b) = mm(&s1r1);
    h.add(
        "Stage A σ1 S_A/S_V at 0.1, min",
        src_a,
        "1.5881",
        a,
        "lead scans priced",
    );
    h.add(
        "Stage A σ1 S_A/S_V at 0.1, max",
        src_a,
        "1.6305",
        b,
        "lead scans priced",
    );
    let (a, b) = mm(&s1r2);
    h.add(
        "Stage A σ1 S_A/S_V at 1, min",
        src_a,
        "1.8389",
        a,
        "lead scans priced",
    );
    h.add(
        "Stage A σ1 S_A/S_V at 1, max",
        src_a,
        "1.8968",
        b,
        "lead scans priced",
    );
    h.exact(
        "Stage A calls with C_V = C_A",
        src_a,
        10000.0,
        c_equal as f64,
        "per call total_muls equal",
    );
    h.exact(
        "Stage A calls with V's fallback flag = A's",
        src_a,
        10000.0,
        fb_equal as f64,
        "G2.215",
    );
    h.exact(
        "Stage A clean calls with stored = 176,064 − (lead_A − lead_V)",
        "validator's consistency check (not printed by the producer)",
        rel_n as f64,
        rel_ok as f64,
        "ties V's stored-cell counter to A counted's lead-scan counter, call by call",
    );
    h.exact(
        "Stage A fallback calls charged A's cells (V cells = A cells)",
        "note §9.50 (i)",
        fb_n as f64,
        fb_cells_equal as f64,
        "every V fallback call",
    );
    // "≈ 78 %" (§9.51) is an approximate statement, not a headline number:
    // reported with its range (validator report, errata), not gated.
    let (a, b) = mm(&tail_share);
    let _ = write!(
        rep,
        ", \"stage_a\": {{\"curves\": [{sa_rows}], \"fallback_calls_v\": {fb_total}, \"stored_share_min\": {a}, \"stored_share_max\": {b}, \"consistency_stored_vs_lead_scans\": [{rel_ok}, {rel_n}]}}"
    );
    let g206 = vcm.iter().all(|&x| (1.02e5..=1.04e5).contains(&x))
        && eratio.iter().all(|&x| (1.69..=1.73).contains(&x));
    let g207 = ops.iter().all(|&x| (1.5e4..=3.0e4).contains(&x));
    let g208 = r0.iter().all(|&x| x == 1.0)
        && c_equal == calls_total
        && r1.iter().all(|&x| (1.18..=1.34).contains(&x))
        && r2.iter().all(|&x| (1.28..=1.48).contains(&x));
    let g208_kill = r2.iter().any(|&x| x < 1.15);
    let g209 = vlead.iter().all(|&x| (5.0e3..=7.0e3).contains(&x));

    // ── Controls and identity of existing configurations ──
    let ck = cx.load(&format!("{e40}/checks.json"));
    let ck1 = cx.load(&format!("{e40}/checks_v1.json"));
    let sc = |j: &J, f: &str| -> u64 { j.at("curves").arr().iter().map(|c| c.uk(f)).sum() };
    for (nm, f, claim) in [
        ("controls: calls", "calls", 320.0),
        ("controls: V = A counted", "v_equals_a_counted", 320.0),
        ("controls: P1 detected", "p1_detected", 320.0),
        ("controls: P2 detected", "p2_detected", 320.0),
        ("controls: non-empty clean solution sets", "nonempty", 213.0),
        (
            "controls: perturbed coefficient changed the set",
            "perturbed_changed",
            213.0,
        ),
    ] {
        h.exact(
            nm,
            "note §9.51 (G2.205), checks.json per-curve rows",
            claim,
            sc(&ck, f) as f64,
            "Σ over the ten curves",
        );
    }
    h.exact(
        "controls v1 (vacuous run, kept): non-empty sets",
        "note §9.51 disclosed harness fix",
        0.0,
        sc(&ck1, "nonempty") as f64,
        "checks_v1.json",
    );
    h.exact(
        "controls v1: P1 detected",
        "note §9.51 (same in both runs)",
        320.0,
        sc(&ck1, "p1_detected") as f64,
        "checks_v1.json",
    );
    h.exact(
        "controls v1: P2 detected",
        "note §9.51 (same in both runs)",
        320.0,
        sc(&ck1, "p2_detected") as f64,
        "checks_v1.json",
    );
    let ide = cx.load(&format!("{e40}/identity_existing_solvers.json"));
    let n_ident = ide
        .at("runs")
        .arr()
        .iter()
        .filter(|r| r.at("reference_hash").s() == r.at("e40").s())
        .count();
    h.exact(
        "existing configurations byte-identical (hash equality re-read)",
        "note §9.51 (G2.204)",
        7.0,
        n_ident as f64,
        "reference_hash == e40",
    );
    let e33 = cx.load("experiments/37_solver_b_pivot_count/identity_existing_solvers.json");
    let mut e33_refs_ok = 0;
    for r in ide.at("runs").arr().iter().take(4) {
        let cfg = r.at("config").s();
        let refh = e33
            .at("runs")
            .arr()
            .iter()
            .find(|x| x.at("config").s() == cfg)
            .map(|x| x.at("new").s().to_string());
        e33_refs_ok += usize::from(refh.as_deref() == Some(r.at("reference_hash").s()));
    }
    h.exact(
        "E33 reference hashes cited by E40 equal E33's recorded 'new' hashes",
        "identity_existing_solvers.json",
        4.0,
        e33_refs_ok as f64,
        "experiments/37_solver_b_pivot_count/identity_existing_solvers.json",
    );

    // ── End to end ──
    let s27 = cx.load("experiments/32_walk_karatsuba/stage_w.json");
    let s27sum = cx.load("experiments/32_walk_karatsuba/summary.json");
    let w24 = cx.load(&format!("{e37}/walk_e24.json"));
    let mut rows = String::new();
    let mut ident_fields_total = 0u64;
    let mut ident_bad: Vec<String> = Vec::new();
    let mut allowed_seen: Vec<String> = Vec::new();
    let mut pair_bad: Vec<String> = Vec::new();
    let (mut av, mut ev, mut av1, mut ev1, mut av4, mut ev4, mut vr, mut ar) = (
        [vec![], vec![], vec![]],
        [vec![], vec![], vec![]],
        [vec![], vec![], vec![]],
        [vec![], vec![], vec![]],
        [vec![], vec![], vec![]],
        [vec![], vec![], vec![]],
        [vec![], vec![], vec![]],
        [vec![], vec![], vec![]],
    );
    let (mut ord_va, mut ord_ve, mut ord_va1, mut ord_ve1, mut ord_va4, mut ord_ve4, mut rho_ahead) =
        (0, 0, 0, 0, 0, 0, 0);
    let (mut fb_sum_v, mut fb_sum_a, mut fb_rows_equal, mut health) = (0u64, 0u64, 0, true);
    let (mut tot_equal, mut rho_summary_ok, mut rho_rec_ok) = (0, 0, 0);
    let mut nrow = 0;
    let mut prefix_per_clean: Vec<f64> = Vec::new();
    let mut best_v = (f64::INFINITY, String::new());
    for (set, curves) in [("gls", &GLS[..]), ("e24", &E24[..])] {
        for &(p, seed) in curves {
            let fa = cx.load(&format!("{e37}/e2e/{set}_chain-counted_p{p}s{seed}.json"));
            let fe = cx.load(&format!("{e37}/e2e/{set}_macaulay_p{p}s{seed}.json"));
            let fv = cx.load(&format!("{e40}/e2e/{set}_chain-profile_p{p}s{seed}.json"));
            let arms: Vec<Option<&str>> = if set == "gls" {
                vec![Some("A"), Some("B")]
            } else {
                vec![None]
            };
            for arm in arms {
                let (xa, xe, xv) = (arm_rec(&fa, arm), arm_rec(&fe, arm), arm_rec(&fv, arm));
                let tag = format!("{set} p{p} s{seed} {}", arm.unwrap_or("-"));
                // pairing: same instance, stream, flags
                for (nm, x, solver) in [
                    ("A", xa, "chain-counted"),
                    ("E25", xe, ""),
                    ("V", xv, "chain-profile"),
                ] {
                    let ok = x.uk("p") == p
                        && x.uk("seed") == seed
                        && x.uk("n") == xa.uk("n")
                        && x.at("stream_salt").s() == "0xe37e0001"
                        && x.at("fp4_product").s() == "karatsuba"
                        && (solver.is_empty() || x.at("solver").s() == solver)
                        && (nm != "E25" || x.get("solver").is_none());
                    if !ok {
                        pair_bad.push(format!("{tag} {nm}"));
                    }
                    health &= x.at("folded").at("correct").b()
                        && x.at("rho").arr().iter().all(|w| w.at("correct").b());
                }
                // identity V vs A
                let mut d = Vec::new();
                diff(xa, xv, "", &allowed, &mut d);
                ident_fields_total += count_leaves(xa, &allowed);
                if !d.is_empty() {
                    ident_bad.push(format!("{tag}: {d:?}"));
                }
                if let J::O(kv) = xv {
                    for (k, _) in kv {
                        if allowed(k) && !allowed_seen.contains(k) {
                            allowed_seen.push(k.clone());
                        }
                    }
                }
                tot_equal += usize::from(xa.uk("total_muls") == xv.uk("total_muls"));
                // Fallbacks charged alike: A's entries minus V's stored cells,
                // per call without fallback, must be the mean zero prefix
                // (Stage 0: 73,237–73,315 per call), since every fallback call
                // carries 2·rows·cols in both arms.
                let clean_calls = xa.fk("residuals") - xa.fk("solver_fallbacks");
                prefix_per_clean.push(
                    (xa.fk("macaulay_entries") - xv.fk("macaulay_stored_entries")) / clean_calls,
                );
                let (fbv, fba) = (xv.uk("solver_fallbacks"), xa.uk("solver_fallbacks"));
                fb_sum_v += fbv;
                fb_sum_a += fba;
                fb_rows_equal += usize::from(fbv == fba);
                // matched rho, walk (iii), from the per-walk records
                let (rho, steps, wok, nw) = if set == "gls" {
                    let c = s27
                        .at("curves")
                        .arr()
                        .iter()
                        .find(|c| c.uk("p") == p && c.uk("seed") == seed)
                        .unwrap();
                    assert_eq!(c.uk("n"), xa.uk("n"), "rho instance");
                    let gl = c.at("folds").at("Gls");
                    let r = walk_iii(gl.at("walks").at("batched"));
                    rho_rec_ok += usize::from(
                        r.0 == gl.at("mean_fp_muls").fk("batched") && r.1 == gl.fk("mean_steps"),
                    );
                    let srow = s27sum
                        .at("rows")
                        .arr()
                        .iter()
                        .find(|x| {
                            x.uk("p") == p
                                && x.uk("seed") == seed
                                && x.at("arm").s() == arm.unwrap()
                        })
                        .unwrap();
                    rho_summary_ok +=
                        usize::from(srow.fk("rho_e27") == r.0 && srow.fk("walk_steps") == r.1);
                    r
                } else {
                    let c = w24
                        .at("curves")
                        .arr()
                        .iter()
                        .find(|c| c.uk("p") == p && c.uk("seed") == seed)
                        .unwrap();
                    assert_eq!(c.uk("n"), xa.uk("n"), "rho instance");
                    assert_eq!(c.at("fold").s(), "Negation");
                    let r = walk_iii(c.at("walks").at("batched"));
                    rho_rec_ok += usize::from(
                        r.0 == c.at("mean_fp_muls").fk("batched") && r.1 == c.fk("mean_steps"),
                    );
                    rho_summary_ok += 1; // no summary row for E24 (walk_e24.json is the record)
                    r
                };
                assert!(wok && nw == 16);
                let rho_g = GS.map(|g| rho + GAE * g * steps);
                let calls = xa.fk("residuals");
                let st = struct_of(set, p, seed) * calls;
                let (la, lv) = (xa.fk("lead_scan_reads"), xv.fk("lead_scan_reads"));
                let (sa_, se, sv) = (s3(xa, 0.0), s3(xe, 0.0), s3(xv, 0.0));
                let (sa1, se1, sv1) = (s3(xa, la), s3(xe, la), s3(xv, lv));
                let (sa4, se4, sv4) = (s3(xa, st), s3(xe, st), s3(xv, st));
                let rat = |r: &[f64; 3], x: &[f64; 3]| -> [f64; 3] {
                    [r[0] / x[0], r[1] / x[1], r[2] / x[2]]
                };
                let (q_av, q_ev, q_av1, q_ev1, q_av4, q_ev4, q_vr, q_ar) = (
                    rat(&sa_, &sv),
                    rat(&se, &sv),
                    rat(&sa1, &sv1),
                    rat(&se1, &sv1),
                    rat(&sa4, &sv4),
                    rat(&se4, &sv4),
                    rat(&sv, &rho_g),
                    rat(&sa_, &rho_g),
                );
                for g in 0..3 {
                    av[g].push(q_av[g]);
                    ev[g].push(q_ev[g]);
                    av1[g].push(q_av1[g]);
                    ev1[g].push(q_ev1[g]);
                    av4[g].push(q_av4[g]);
                    ev4[g].push(q_ev4[g]);
                    vr[g].push(q_vr[g]);
                    ar[g].push(q_ar[g]);
                }
                let want = "equal at 0, ahead at 0.1 and 1";
                ord_va += usize::from(order(&sa_, &sv) == want);
                ord_ve += usize::from(order(&se, &sv) == "ahead");
                ord_va1 += usize::from(order(&sa1, &sv1) == want);
                ord_ve1 += usize::from(order(&se1, &sv1) == "ahead");
                ord_va4 += usize::from(order(&sa4, &sv4) == want);
                ord_ve4 += usize::from(order(&se4, &sv4) == "ahead");
                rho_ahead += usize::from((0..3).all(|g| rho_g[g] < sv[g]));
                if q_vr[0] < best_v.0 {
                    best_v = (q_vr[0], tag.clone());
                }
                // claims of the per-row table
                let tc = table_claims
                    .iter()
                    .find(|t| t.0 == set && t.1 == p && t.2 == seed && t.3 == arm.unwrap_or("—"))
                    .unwrap_or_else(|| panic!("row {tag} not in §9.51 table"));
                let names = [
                    "A/V 0.1",
                    "A/V 1",
                    "E25/V 0",
                    "E25/V 0.1",
                    "E25/V 1",
                    "V/rho 0",
                    "V/rho 0.1",
                    "V/rho 1",
                    "A/rho 0.1",
                    "A/rho 1",
                ];
                let vals = [
                    q_av[1], q_av[2], q_ev[0], q_ev[1], q_ev[2], q_vr[0], q_vr[1], q_vr[2],
                    q_ar[1], q_ar[2],
                ];
                for k in 0..10 {
                    h.add(
                        &format!("row {tag}: {}", names[k]),
                        "note §9.51 (distance table)",
                        &tc.4[k],
                        vals[k],
                        "S(g) = total_muls + 97 g cells; rho(g) = rho + 97 g steps",
                    );
                }
                let _ = write!(
                    rows,
                    "{}{{\"row\": {}, \"calls\": {}, \"fallbacks\": {}, \"rho_walk_iii\": {}, \"steps\": {}, \"S_A\": [{}, {}, {}], \"S_E25\": [{}, {}, {}], \"S_V\": [{}, {}, {}], \"rho\": [{}, {}, {}], \"a_over_v\": [{}, {}, {}], \"e25_over_v\": [{}, {}, {}], \"v_over_rho\": [{}, {}, {}], \"a_over_rho\": [{}, {}, {}], \"sigma1_a_over_v\": [{}, {}, {}], \"sigma4_a_over_v\": [{}, {}, {}], \"v_cells_per_call\": {}, \"v_ops_per_call\": {}, \"a_cells_per_call\": {}, \"order_v_vs_a\": {}, \"order_v_vs_e25\": {}}}",
                    if rows.is_empty() { "" } else { ", " },
                    jstr(&tag), calls, fbv, rho, steps, sa_[0], sa_[1], sa_[2], se[0], se[1], se[2], sv[0], sv[1], sv[2],
                    rho_g[0], rho_g[1], rho_g[2], q_av[0], q_av[1], q_av[2], q_ev[0], q_ev[1], q_ev[2], q_vr[0], q_vr[1], q_vr[2],
                    q_ar[0], q_ar[1], q_ar[2], q_av1[0], q_av1[1], q_av1[2], q_av4[0], q_av4[1], q_av4[2],
                    xv.fk("macaulay_stored_entries") / calls, xv.fk("profile_offset_ops") / calls, xa.fk("macaulay_entries") / calls,
                    jstr(order(&sa_, &sv)), jstr(order(&se, &sv))
                );
                nrow += 1;
            }
        }
    }
    assert_eq!(nrow, 32);
    let src_e = "note §9.51 (end-to-end table)";
    let band = |h: &mut Head, nm: &str, v: &[Vec<f64>; 3], claims: [(&str, &str); 3]| {
        for g in 0..3 {
            let (a, b) = mm(&v[g]);
            h.add(
                &format!("{nm} at {} GAE, min over 32 rows", GS[g]),
                src_e,
                claims[g].0,
                a,
                "per-row ratio",
            );
            h.add(
                &format!("{nm} at {} GAE, max over 32 rows", GS[g]),
                src_e,
                claims[g].1,
                b,
                "per-row ratio",
            );
        }
    };
    band(
        &mut h,
        "S_A/S_V",
        &av,
        [
            ("1.0000", "1.0000"),
            ("1.2518", "1.2618"),
            ("1.3629", "1.3807"),
        ],
    );
    band(
        &mut h,
        "S_E25/S_V",
        &ev,
        [
            ("2.5054", "2.5766"),
            ("1.7455", "1.8033"),
            ("1.4105", "1.4558"),
        ],
    );
    band(
        &mut h,
        "σ1 S_A/S_V",
        &av1,
        [
            ("1.0000", "1.0000"),
            ("1.6024", "1.6217"),
            ("1.8572", "1.8921"),
        ],
    );
    band(
        &mut h,
        "σ1 S_E25/S_V",
        &ev1,
        [
            ("2.5054", "2.5766"),
            ("2.0819", "2.1472"),
            ("1.9028", "1.9640"),
        ],
    );
    band(
        &mut h,
        "σ4 S_A/S_V",
        &av4,
        [
            ("1.0000", "1.0000"),
            ("1.2362", "1.2449"),
            ("1.3312", "1.3461"),
        ],
    );
    band(
        &mut h,
        "σ4 S_E25/S_V",
        &ev4,
        [
            ("2.5054", "2.5766"),
            ("1.6992", "1.7516"),
            ("1.3746", "1.4144"),
        ],
    );
    band(
        &mut h,
        "S_V/rho",
        &vr,
        [
            ("6.00·10³", "1.98·10⁵"),
            ("1.58·10⁴", "5.41·10⁵"),
            ("6.17·10⁴", "2.36·10⁶"),
        ],
    );
    band(
        &mut h,
        "S_A/rho",
        &ar,
        [
            ("6.00·10³", "1.98·10⁵"),
            ("1.99·10⁴", "6.80·10⁵"),
            ("8.51·10⁴", "3.24·10⁶"),
        ],
    );
    h.exact(
        "rows with total_muls(V) = total_muls(A)",
        src_e,
        32.0,
        tot_equal as f64,
        "0 GAE equality, integer",
    );
    h.exact(
        "rows: V equal to A at 0, ahead at 0.1 and 1",
        src_e,
        32.0,
        ord_va as f64,
        "§9.50 ranking rule",
    );
    h.exact(
        "rows: V ahead of E25 at every weight",
        src_e,
        32.0,
        ord_ve as f64,
        "ordinary rule",
    );
    h.exact(
        "rows: σ1 order V vs A as registered",
        src_e,
        32.0,
        ord_va1 as f64,
        "G2.213",
    );
    h.exact(
        "rows: σ1 V ahead of E25",
        src_e,
        32.0,
        ord_ve1 as f64,
        "G2.213",
    );
    h.exact(
        "rows: σ4 order V vs A as registered",
        src_e,
        32.0,
        ord_va4 as f64,
        "G2.213",
    );
    h.exact(
        "rows: σ4 V ahead of E25",
        src_e,
        32.0,
        ord_ve4 as f64,
        "G2.213",
    );
    h.exact(
        "rows: rho ahead of V at every weight",
        src_e,
        32.0,
        rho_ahead as f64,
        "G2.212",
    );
    h.add(
        "fallbacks over the 32 rows (V)",
        "note §9.51 (G2.215)",
        "8,947",
        fb_sum_v as f64,
        "Σ solver_fallbacks of V",
    );
    h.exact(
        "fallbacks over the 32 rows (A, E37)",
        "note §9.51 (G2.215)",
        8947.0,
        fb_sum_a as f64,
        "Σ solver_fallbacks of A",
    );
    h.exact(
        "rows whose V fallbacks equal A's",
        "note §9.51 (G2.215)",
        32.0,
        fb_rows_equal as f64,
        "per row",
    );
    let (a, b) = mm(&prefix_per_clean);
    let inside = prefix_per_clean
        .iter()
        .filter(|&&x| (73_237.0..=73_315.0).contains(&x))
        .count();
    h.exact(
        "e2e rows whose (A entries − V stored) per call without fallback lies in Stage 0's per-call prefix range [73,237, 73,315]",
        "validator's fallback-charging check (fallback calls carry 2·rows·cols in both arms)",
        32.0,
        inside as f64,
        "per row",
    );
    let _ = write!(rep, ", \"e2e_prefix_per_clean_call\": [{a}, {b}]");
    h.exact(
        "rows whose V record equals A's outside the allowed fields",
        "note §9.51 (G2.204)",
        32.0,
        (32 - ident_bad.len()) as f64,
        "validator's own deep compare",
    );
    h.exact(
        "rows whose walk (iii) recomputes the record's mean",
        "walk records",
        32.0,
        rho_rec_ok as f64,
        "mean(fp_muls − verify_fp_muls), mean steps",
    );
    h.exact(
        "GLS rows whose walk (iii) equals E27's summary rho_e27/walk_steps",
        "experiments/32_walk_karatsuba/summary.json",
        32.0,
        rho_summary_ok as f64,
        "E24 rows count as 1 (no summary row)",
    );
    let g210 = ord_va == 32
        && tot_equal == 32
        && mm(&av[1]).0 >= 1.18
        && mm(&av[1]).1 <= 1.38
        && mm(&av[2]).0 >= 1.28
        && mm(&av[2]).1 <= 1.48;
    let g211 = ord_ve == 32
        && mm(&ev[0]).0 >= 2.2
        && mm(&ev[0]).1 <= 2.9
        && mm(&ev[1]).0 >= 1.6
        && mm(&ev[1]).1 <= 2.0
        && mm(&ev[2]).0 >= 1.30
        && mm(&ev[2]).1 <= 1.58;
    let g212 = rho_ahead == 32;
    let g213 = ord_va1 == 32
        && ord_ve1 == 32
        && ord_va4 == 32
        && ord_ve4 == 32
        && mm(&av1[2]).0 >= 1.6
        && mm(&av1[2]).1 <= 2.0;

    // ── Negative control: A′ and V′ on E40E0002 ──
    let (mut neg_rows, mut n_ahead, mut n_behind, mut n_unres, mut v2_order, mut v2_ident) =
        (String::new(), 0, 0, 0, 0, 0);
    let (mut aa, mut a2v2) = ([vec![], vec![], vec![]], [vec![], vec![], vec![]]);
    let mut v2_bad = Vec::new();
    let (mut v2_fb_equal, mut v2_prefix_inside) = (0usize, 0usize);
    for &(p, seed) in &GLS {
        let fa = cx.load(&format!("{e37}/e2e/gls_chain-counted_p{p}s{seed}.json"));
        let fa2 = cx.load(&format!(
            "{e40}/e2e/gls_chain-counted_E40E0002_p{p}s{seed}.json"
        ));
        let fv2 = cx.load(&format!(
            "{e40}/e2e/gls_chain-profile_E40E0002_p{p}s{seed}.json"
        ));
        for arm in ["A", "B"] {
            let (xa, xa2, xv2) = (
                arm_rec(&fa, Some(arm)),
                arm_rec(&fa2, Some(arm)),
                arm_rec(&fv2, Some(arm)),
            );
            assert_eq!(xa2.at("stream_salt").s(), "0xe40e0002");
            assert_eq!(xv2.at("stream_salt").s(), "0xe40e0002");
            assert_eq!(xa2.uk("n"), xa.uk("n"));
            let (sa_, sa2, sv2) = (s3(xa, 0.0), s3(xa2, 0.0), s3(xv2, 0.0));
            let o = order(&sa_, &sa2);
            match o {
                "ahead" => n_ahead += 1,
                "behind" => n_behind += 1,
                _ => n_unres += 1,
            }
            for g in 0..3 {
                aa[g].push(sa_[g] / sa2[g]);
                a2v2[g].push(sa2[g] / sv2[g]);
            }
            v2_order += usize::from(order(&sa2, &sv2) == "equal at 0, ahead at 0.1 and 1");
            v2_fb_equal += usize::from(xa2.uk("solver_fallbacks") == xv2.uk("solver_fallbacks"));
            let clean2 = xa2.fk("residuals") - xa2.fk("solver_fallbacks");
            let pre2 = (xa2.fk("macaulay_entries") - xv2.fk("macaulay_stored_entries")) / clean2;
            v2_prefix_inside += usize::from((73_237.0..=73_315.0).contains(&pre2));
            let mut d = Vec::new();
            diff(xa2, xv2, "", &allowed, &mut d);
            if d.is_empty() {
                v2_ident += 1;
            } else {
                v2_bad.push(format!("p{p} s{seed} {arm}: {d:?}"));
            }
            let _ = write!(
                neg_rows,
                "{}{{\"row\": \"gls p{p} s{seed} {arm}\", \"order_a2_vs_a\": {}, \"a_over_a2\": [{}, {}, {}], \"a2_over_v2\": [{}, {}, {}], \"residuals_a\": {}, \"residuals_a2\": {}}}",
                if neg_rows.is_empty() { "" } else { ", " },
                jstr(o), sa_[0] / sa2[0], sa_[1] / sa2[1], sa_[2] / sa2[2], sa2[0] / sv2[0], sa2[1] / sv2[1], sa2[2] / sv2[2],
                xa.uk("residuals"), xa2.uk("residuals")
            );
        }
    }
    let src_n = "note §9.51 (negative control G2.214)";
    h.exact(
        "negative: A′ ahead of A across the band (rows)",
        src_n,
        10.0,
        n_ahead as f64,
        "unpaired across streams",
    );
    h.exact(
        "negative: A′ behind A across the band (rows)",
        src_n,
        10.0,
        n_behind as f64,
        "unpaired across streams",
    );
    h.exact(
        "negative: A′ vs A unresolved rows",
        src_n,
        0.0,
        n_unres as f64,
        "",
    );
    let all_aa: Vec<f64> = aa.iter().flatten().copied().collect();
    let (a, b) = mm(&all_aa);
    h.add(
        "negative: S_A/S_A′ min over rows and weights",
        src_n,
        "0.373",
        a,
        "",
    );
    h.add(
        "negative: S_A/S_A′ max over rows and weights",
        src_n,
        "2.15",
        b,
        "",
    );
    for g in 0..3 {
        let (a, b) = mm(&a2v2[g]);
        let c = [
            ("1.0000", "1.0000"),
            ("1.2506", "1.2608"),
            ("1.3613", "1.3790"),
        ][g];
        h.add(
            &format!("negative: S_A′/S_V′ at {} GAE, min", GS[g]),
            src_n,
            c.0,
            a,
            "paired on E40E0002",
        );
        h.add(
            &format!("negative: S_A′/S_V′ at {} GAE, max", GS[g]),
            src_n,
            c.1,
            b,
            "paired on E40E0002",
        );
    }
    h.exact(
        "negative: V′ order vs A′ as registered (rows)",
        src_n,
        20.0,
        v2_order as f64,
        "",
    );
    h.exact(
        "negative: V′ records = A′ outside the allowed fields (rows)",
        src_n,
        20.0,
        v2_ident as f64,
        "",
    );
    h.exact(
        "negative: V′ fallbacks = A′ fallbacks (rows)",
        src_n,
        20.0,
        v2_fb_equal as f64,
        "G2.215 on the second stream",
    );
    h.exact(
        "negative: rows whose (A′ entries − V′ stored) per call without fallback lies in [73,237, 73,315]",
        "validator's fallback-charging check on E40E0002",
        20.0,
        v2_prefix_inside as f64,
        "per row",
    );
    let g214 = n_ahead > 0
        && n_behind > 0
        && v2_order == 20
        && v2_ident == 20
        && mm(&a2v2[1]).0 >= 1.18
        && mm(&a2v2[1]).1 <= 1.38
        && mm(&a2v2[2]).0 >= 1.28
        && mm(&a2v2[2]).1 <= 1.48;
    let g215 = fb_equal == calls_total && fb_rows_equal == 32;
    let g204 = ident_bad.is_empty()
        && v2_ident == 20
        && c_equal == calls_total
        && n_ident == 7
        && rel_ok == rel_n;
    let g205 = sc(&ck, "p1_detected") == 320
        && sc(&ck, "p2_detected") == 320
        && sc(&ck, "perturbed_changed") == sc(&ck, "nonempty")
        && sc(&ck, "v_equals_a_counted") == 320;

    // ── Output ──
    let mut o = String::new();
    o.push_str("{\"schema\": \"e40-validator-recompute/v1\", \"program\": \"research/e40_macaulay_entries_20261102/validate_e40.rs (std only; own JSON parser and SHA-256)\"");
    let _ = write!(o, ", \"raw_data\": [");
    for (i, (p, s)) in cx.raw.iter().enumerate() {
        let _ = write!(
            o,
            "{}{{\"path\": {}, \"sha256\": \"{}\"}}",
            if i > 0 { ", " } else { "" },
            jstr(&format!("../../{p}")),
            s
        );
    }
    o.push(']');
    let _ = write!(o, ", \"headline\": [{}]", h.out.join(", "));
    let _ = write!(
        o,
        ", \"headline_disagreements\": [{}]",
        h.bad.iter().map(|s| jstr(s)).collect::<Vec<_>>().join(", ")
    );
    let _ = write!(
        o,
        ", \"g2_recomputed\": {{\"G2.204\": {g204}, \"G2.205\": {g205}, \"G2.206\": {g206}, \"G2.207\": {g207}, \"G2.208\": {g208}, \"G2.208_kill_fired\": {g208_kill}, \"G2.209\": {g209}, \"G2.210\": {g210}, \"G2.211\": {g211}, \"G2.212\": {g212}, \"G2.213\": {g213}, \"G2.214\": {g214}, \"G2.215\": {g215}, \"logs_recovered_and_walks_correct\": {health}}}"
    );
    let _ = write!(
        o,
        ", \"identity_e2e\": {{\"rows\": 32, \"rows_differing\": [{}], \"leaf_fields_compared_per_A_record_sum\": {ident_fields_total}, \"allowed_keys_present_in_V\": [{}], \"pairing_problems\": [{}], \"negative_rows_differing\": [{}]}}",
        ident_bad.iter().map(|s| jstr(s)).collect::<Vec<_>>().join(", "),
        allowed_seen.iter().map(|s| jstr(s)).collect::<Vec<_>>().join(", "),
        pair_bad.iter().map(|s| jstr(s)).collect::<Vec<_>>().join(", "),
        v2_bad.iter().map(|s| jstr(s)).collect::<Vec<_>>().join(", ")
    );
    let _ = write!(
        o,
        ", \"best_row_v_over_rho_at_0\": [{}, {}]",
        jstr(&best_v.1),
        best_v.0
    );
    let _ = write!(o, ", \"rows\": [{rows}], \"negative\": [{neg_rows}]");
    o.push_str(&rep);
    o.push('}');
    fs::write(&outp, &o).unwrap();
    eprintln!(
        "headlines {} ({} disagree); G2.204 {g204} .205 {g205} .206 {g206} .207 {g207} .208 {g208} (kill {g208_kill}) .209 {g209} .210 {g210} .211 {g211} .212 {g212} .213 {g213} .214 {g214} .215 {g215}; health {health}",
        h.out.len(),
        h.bad.len()
    );
    for b in &h.bad {
        eprintln!("  DISAGREE {b}");
    }
    for b in ident_bad.iter().chain(&v2_bad).chain(&pair_bad) {
        eprintln!("  IDENTITY/PAIRING {b}");
    }
}
