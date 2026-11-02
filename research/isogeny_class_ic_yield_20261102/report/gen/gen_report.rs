// Report and visuals generator for research/isogeny_class_ic_yield_20261102.
//
// Rust, std only (AGENTS.md "Implementation language: no Python"). It reads the recorded analysis
// files and rows of P1, P2 and P3 and lays them out as REPORT.md, REPORT.typ and SVG graphs. It
// computes no result of its own beyond descriptive aggregates of recorded values (ratios of recorded
// numbers, means, ranges, counts), each labelled as such in the report. Missing cells render as
// "pending", never as zeros or nulls.
//
// Build and run: report/build.sh (compiles this file with rustc, runs it, renders the PDF).
// Usage: gen_report --study <study dir> [--out <report dir>]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

// ---------------------------------------------------------------- JSON

#[derive(Clone, Debug)]
enum J {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<J>),
    Obj(Vec<(String, J)>),
}
static NULL: J = J::Null;

impl J {
    fn get(&self, k: &str) -> &J {
        if let J::Obj(v) = self {
            for (kk, vv) in v {
                if kk == k {
                    return vv;
                }
            }
        }
        &NULL
    }
    fn path(&self, p: &str) -> &J {
        let mut cur = self;
        for part in p.split('.') {
            cur = cur.get(part);
        }
        cur
    }
    fn f(&self) -> Option<f64> {
        if let J::Num(x) = self {
            Some(*x)
        } else {
            None
        }
    }
    fn i(&self) -> Option<i64> {
        self.f().map(|x| x as i64)
    }
    fn s(&self) -> Option<&str> {
        if let J::Str(x) = self {
            Some(x)
        } else {
            None
        }
    }
    fn b(&self) -> Option<bool> {
        if let J::Bool(x) = self {
            Some(*x)
        } else {
            None
        }
    }
    fn arr(&self) -> &[J] {
        if let J::Arr(v) = self {
            v
        } else {
            &[]
        }
    }
    fn obj(&self) -> &[(String, J)] {
        if let J::Obj(v) = self {
            v
        } else {
            &[]
        }
    }
    fn is_null(&self) -> bool {
        matches!(self, J::Null)
    }
}

struct Parser<'a> {
    b: &'a [u8],
    i: usize,
}
impl<'a> Parser<'a> {
    fn ws(&mut self) {
        while self.i < self.b.len() && (self.b[self.i] as char).is_ascii_whitespace() {
            self.i += 1;
        }
    }
    fn lit(&mut self, s: &str, v: J) -> Result<J, String> {
        if self.b[self.i..].starts_with(s.as_bytes()) {
            self.i += s.len();
            Ok(v)
        } else {
            Err(format!("bad literal at {}", self.i))
        }
    }
    fn value(&mut self) -> Result<J, String> {
        self.ws();
        if self.i >= self.b.len() {
            return Err("eof".into());
        }
        match self.b[self.i] {
            b'{' => {
                self.i += 1;
                let mut v = Vec::new();
                self.ws();
                if self.b.get(self.i) == Some(&b'}') {
                    self.i += 1;
                    return Ok(J::Obj(v));
                }
                loop {
                    self.ws();
                    let k = self.string()?;
                    self.ws();
                    if self.b.get(self.i) != Some(&b':') {
                        return Err("expected :".into());
                    }
                    self.i += 1;
                    let x = self.value()?;
                    v.push((k, x));
                    self.ws();
                    match self.b.get(self.i) {
                        Some(b',') => self.i += 1,
                        Some(b'}') => {
                            self.i += 1;
                            return Ok(J::Obj(v));
                        }
                        _ => return Err("expected , or }".into()),
                    }
                }
            }
            b'[' => {
                self.i += 1;
                let mut v = Vec::new();
                self.ws();
                if self.b.get(self.i) == Some(&b']') {
                    self.i += 1;
                    return Ok(J::Arr(v));
                }
                loop {
                    v.push(self.value()?);
                    self.ws();
                    match self.b.get(self.i) {
                        Some(b',') => self.i += 1,
                        Some(b']') => {
                            self.i += 1;
                            return Ok(J::Arr(v));
                        }
                        _ => return Err("expected , or ]".into()),
                    }
                }
            }
            b'"' => Ok(J::Str(self.string()?)),
            b't' => self.lit("true", J::Bool(true)),
            b'f' => self.lit("false", J::Bool(false)),
            b'n' => self.lit("null", J::Null),
            b'N' => self.lit("NaN", J::Num(f64::NAN)),
            _ => {
                let st = self.i;
                while self.i < self.b.len() && matches!(self.b[self.i], b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9') {
                    self.i += 1;
                }
                let t = std::str::from_utf8(&self.b[st..self.i]).map_err(|e| e.to_string())?;
                t.parse::<f64>().map(J::Num).map_err(|_| format!("bad number '{t}'"))
            }
        }
    }
    fn string(&mut self) -> Result<String, String> {
        if self.b.get(self.i) != Some(&b'"') {
            return Err("expected string".into());
        }
        self.i += 1;
        let mut out: Vec<u8> = Vec::new();
        loop {
            let c = *self.b.get(self.i).ok_or("unterminated string")?;
            self.i += 1;
            match c {
                b'"' => break,
                b'\\' => {
                    let e = *self.b.get(self.i).ok_or("bad escape")?;
                    self.i += 1;
                    match e {
                        b'n' => out.push(b'\n'),
                        b't' => out.push(b'\t'),
                        b'r' => out.push(b'\r'),
                        b'b' => out.push(8),
                        b'f' => out.push(12),
                        b'u' => {
                            let h = std::str::from_utf8(self.b.get(self.i..self.i + 4).ok_or("bad \\u")?).map_err(|e| e.to_string())?;
                            let cp = u32::from_str_radix(h, 16).map_err(|e| e.to_string())?;
                            self.i += 4;
                            let ch = char::from_u32(cp).unwrap_or('?');
                            let mut buf = [0u8; 4];
                            out.extend_from_slice(ch.encode_utf8(&mut buf).as_bytes());
                        }
                        other => out.push(other),
                    }
                }
                _ => out.push(c),
            }
        }
        String::from_utf8(out).map_err(|e| e.to_string())
    }
}
fn parse_json(s: &str) -> Result<J, String> {
    let mut p = Parser { b: s.as_bytes(), i: 0 };
    p.value()
}

// ---------------------------------------------------------------- file helpers

fn is_gz(p: &Path) -> bool {
    p.extension().map(|e| e == "gz").unwrap_or(false)
}
fn read_text(p: &Path) -> Option<String> {
    if is_gz(p) {
        let o = Command::new("gzip").arg("-dc").arg(p).output().ok()?;
        if !o.status.success() {
            return None;
        }
        String::from_utf8(o.stdout).ok()
    } else {
        std::fs::read_to_string(p).ok()
    }
}
fn for_lines(p: &Path, mut f: impl FnMut(&str)) -> bool {
    if is_gz(p) {
        let mut ch = match Command::new("gzip").arg("-dc").arg(p).stdout(Stdio::piped()).spawn() {
            Ok(c) => c,
            Err(_) => return false,
        };
        let rd = BufReader::new(ch.stdout.take().unwrap());
        for l in rd.lines().map_while(Result::ok) {
            f(&l);
        }
        let _ = ch.wait();
        true
    } else {
        match std::fs::File::open(p) {
            Ok(fh) => {
                for l in BufReader::new(fh).lines().map_while(Result::ok) {
                    f(&l);
                }
                true
            }
            Err(_) => false,
        }
    }
}
fn read_json(p: &Path) -> Option<J> {
    let t = read_text(p)?;
    parse_json(&t).ok()
}
fn sha256(p: &Path) -> String {
    Command::new("sha256sum")
        .arg(p)
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| s.split_whitespace().next().map(|x| x.to_string()))
        .unwrap_or_else(|| "unavailable".into())
}

// ---------------------------------------------------------------- SVG

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}
struct Svg {
    w: f64,
    h: f64,
    b: String,
}
const TS: f64 = 1.2; // global text scale of the SVGs
const FONT: &str = "DejaVu Sans, Liberation Sans, Helvetica, Arial, sans-serif";
const INK: &str = "#222222";
const MUTED: &str = "#666666";
const BLUE: &str = "#0072B2";
const ORANGE: &str = "#E69F00";
const GREEN: &str = "#009E73";
const VERM: &str = "#D55E00";
const PURPLE: &str = "#CC79A7";
const GREY: &str = "#888888";
impl Svg {
    fn new(w: f64, h: f64) -> Svg {
        Svg { w, h, b: String::new() }
    }
    fn rect(&mut self, x: f64, y: f64, w: f64, h: f64, fill: &str, stroke: &str, extra: &str) {
        let _ = writeln!(self.b, "<rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{w:.1}\" height=\"{h:.1}\" fill=\"{fill}\" stroke=\"{stroke}\" {extra}/>");
    }
    fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, stroke: &str, sw: f64, extra: &str) {
        let _ = writeln!(self.b, "<line x1=\"{x1:.1}\" y1=\"{y1:.1}\" x2=\"{x2:.1}\" y2=\"{y2:.1}\" stroke=\"{stroke}\" stroke-width=\"{sw}\" {extra}/>");
    }
    fn circle(&mut self, cx: f64, cy: f64, r: f64, fill: &str, stroke: &str, sw: f64, extra: &str) {
        let _ = writeln!(self.b, "<circle cx=\"{cx:.1}\" cy=\"{cy:.1}\" r=\"{r:.1}\" fill=\"{fill}\" stroke=\"{stroke}\" stroke-width=\"{sw}\" {extra}/>");
    }
    fn ellipse(&mut self, cx: f64, cy: f64, rx: f64, ry: f64, fill: &str, stroke: &str, sw: f64, extra: &str) {
        let _ = writeln!(self.b, "<ellipse cx=\"{cx:.1}\" cy=\"{cy:.1}\" rx=\"{rx:.1}\" ry=\"{ry:.1}\" fill=\"{fill}\" stroke=\"{stroke}\" stroke-width=\"{sw}\" {extra}/>");
    }
    fn poly(&mut self, pts: &[(f64, f64)], fill: &str, stroke: &str, sw: f64, extra: &str) {
        let p: Vec<String> = pts.iter().map(|(x, y)| format!("{x:.1},{y:.1}")).collect();
        let _ = writeln!(self.b, "<polygon points=\"{}\" fill=\"{fill}\" stroke=\"{stroke}\" stroke-width=\"{sw}\" {extra}/>", p.join(" "));
    }
    fn text(&mut self, x: f64, y: f64, s: &str, size: f64, anchor: &str, fill: &str, extra: &str) {
        let _ = writeln!(self.b, "<text x=\"{x:.1}\" y=\"{y:.1}\" font-size=\"{:.1}\" text-anchor=\"{anchor}\" fill=\"{fill}\" {extra}>{}</text>", size * TS, esc(s));
    }
    /// Word-wrapped text block; returns the y of the line after the last one.
    fn wrap(&mut self, x: f64, y: f64, text: &str, size: f64, maxw: f64, fill: &str) -> f64 {
        let per = (maxw / (size * TS * 0.56)).max(10.0) as usize;
        let mut line = String::new();
        let mut yy = y;
        for word in text.split_whitespace() {
            if !line.is_empty() && line.chars().count() + 1 + word.chars().count() > per {
                self.text(x, yy, &line, size, "start", fill, "");
                yy += size * TS * 1.22;
                line.clear();
            }
            if !line.is_empty() {
                line.push(' ');
            }
            line.push_str(word);
        }
        if !line.is_empty() {
            self.text(x, yy, &line, size, "start", fill, "");
            yy += size * TS * 1.22;
        }
        yy
    }
    fn finish(self, title: &str) -> String {
        format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {w:.0} {h:.0}\" width=\"{w:.0}\" height=\"{h:.0}\" font-family=\"{FONT}\" role=\"img\" aria-label=\"{t}\">\n<title>{t}</title>\n<rect width=\"{w:.0}\" height=\"{h:.0}\" fill=\"#ffffff\"/>\n{body}</svg>\n",
            w = self.w,
            h = self.h,
            t = esc(title),
            body = self.b
        )
    }
}

fn nice_ticks(lo: f64, hi: f64, n: usize) -> Vec<f64> {
    let mut v = Vec::new();
    for i in 0..n {
        v.push(lo + (hi - lo) * i as f64 / (n - 1) as f64);
    }
    v
}
fn hash01(s: &str) -> f64 {
    // deterministic jitter in [-1, 1] (layout only)
    let mut h: u64 = 1469598103934665603;
    for b in s.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(1099511628211);
    }
    ((h >> 11) % 2001) as f64 / 1000.0 - 1.0
}

// ---------------------------------------------------------------- document model (md + typst)

enum Block {
    H(usize, String),
    P(String),
    Bullets(Vec<String>),
    Table { head: Vec<String>, rows: Vec<Vec<String>>, widths: Option<Vec<&'static str>>, caption: Option<String> },
    Fig { path: String, caption: String },
    Pagebreak,
}
struct Doc {
    title: String,
    subtitle: String,
    blocks: Vec<Block>,
}
impl Doc {
    fn h(&mut self, l: usize, t: &str) {
        self.blocks.push(Block::H(l, t.into()));
    }
    fn p(&mut self, t: impl Into<String>) {
        self.blocks.push(Block::P(t.into()));
    }
    fn bullets(&mut self, v: Vec<String>) {
        self.blocks.push(Block::Bullets(v));
    }
    fn table(&mut self, head: &[&str], rows: Vec<Vec<String>>, caption: &str) {
        self.blocks.push(Block::Table { head: head.iter().map(|s| s.to_string()).collect(), rows, widths: None, caption: if caption.is_empty() { None } else { Some(caption.into()) } });
    }
    fn table_w(&mut self, head: &[&str], rows: Vec<Vec<String>>, widths: Vec<&'static str>, caption: &str) {
        self.blocks.push(Block::Table { head: head.iter().map(|s| s.to_string()).collect(), rows, widths: Some(widths), caption: if caption.is_empty() { None } else { Some(caption.into()) } });
    }
    fn fig(&mut self, path: &str, caption: &str) {
        self.blocks.push(Block::Fig { path: path.into(), caption: caption.into() });
    }
}

fn md_cell(s: &str) -> String {
    s.replace('|', "\\|").replace('\n', " ")
}
fn render_md(d: &Doc) -> String {
    let mut o = String::new();
    let mut nfig = 0;
    let _ = writeln!(o, "# {}\n\n{}\n", d.title, d.subtitle);
    for b in &d.blocks {
        match b {
            Block::H(l, t) => {
                let _ = writeln!(o, "\n{} {}\n", "#".repeat(*l + 1), t);
            }
            Block::P(t) => {
                let _ = writeln!(o, "{t}\n");
            }
            Block::Bullets(v) => {
                for x in v {
                    let _ = writeln!(o, "- {x}");
                }
                o.push('\n');
            }
            Block::Table { head, rows, caption, .. } => {
                if let Some(c) = caption {
                    let _ = writeln!(o, "*{c}*\n");
                }
                let _ = writeln!(o, "| {} |", head.iter().map(|x| md_cell(x)).collect::<Vec<_>>().join(" | "));
                let _ = writeln!(o, "|{}|", head.iter().map(|_| "---").collect::<Vec<_>>().join("|"));
                for r in rows {
                    let _ = writeln!(o, "| {} |", r.iter().map(|x| md_cell(x)).collect::<Vec<_>>().join(" | "));
                }
                o.push('\n');
            }
            Block::Fig { path, caption } => {
                nfig += 1;
                let _ = writeln!(o, "![Figure {nfig}]({path})\n\n*Figure {nfig}. {caption}*\n");
            }
            Block::Pagebreak => {}
        }
    }
    o
}

/// Convert the small inline markup of the report text (`code`, **bold**) to Typst markup,
/// escaping everything Typst treats as markup.
fn tx(s: &str) -> String {
    let mut o = String::new();
    let cs: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < cs.len() {
        let c = cs[i];
        if c == '`' {
            // raw span
            if let Some(j) = cs[i + 1..].iter().position(|&x| x == '`') {
                let inner: String = cs[i + 1..i + 1 + j].iter().collect();
                let _ = write!(o, "#raw(\"{}\")", inner.replace('\\', "\\\\").replace('"', "\\\""));
                i += j + 2;
                continue;
            }
        }
        if c == '*' && i + 1 < cs.len() && cs[i + 1] == '*' {
            o.push('*');
            i += 2;
            continue;
        }
        match c {
            '\\' | '#' | '$' | '@' | '*' | '_' | '<' | '>' | '[' | ']' | '~' | '/' | '`' => {
                o.push('\\');
                o.push(c);
            }
            _ => o.push(c),
        }
        i += 1;
    }
    o
}
fn render_typ(d: &Doc) -> String {
    let mut o = String::new();
    o.push_str("#set document(title: \"");
    o.push_str(&d.title.replace('"', "'"));
    o.push_str("\")\n#set page(paper: \"a4\", margin: (x: 1.6cm, y: 1.8cm), numbering: \"1\")\n#set text(size: 9.5pt, font: (\"Libertinus Serif\", \"DejaVu Serif\"))\n#set par(justify: false, leading: 0.6em)\n#show heading: set block(above: 1.1em, below: 0.6em)\n#show figure.caption: set text(size: 8.5pt)\n#set table(stroke: 0.4pt + luma(150), inset: 3.5pt)\n#show raw: set text(size: 8.5pt)\n\n");
    let _ = writeln!(o, "#align(center)[#text(size: 17pt, weight: \"bold\")[{}]\\ #v(2pt) #text(size: 10pt, fill: luma(90))[{}]]\n", tx(&d.title), tx(&d.subtitle));
    for b in &d.blocks {
        match b {
            Block::H(l, t) => {
                let _ = writeln!(o, "\n{} {}\n", "=".repeat(*l), tx(t));
            }
            Block::P(t) => {
                let _ = writeln!(o, "{}\n", tx(t));
            }
            Block::Bullets(v) => {
                for x in v {
                    let _ = writeln!(o, "- {}", tx(x));
                }
                o.push('\n');
            }
            Block::Table { head, rows, widths, caption } => {
                let n = head.len();
                let cols = match widths {
                    Some(w) => format!("({})", w.join(", ")),
                    None => format!("({})", vec!["1fr"; n].join(", ")),
                };
                let _ = writeln!(o, "#figure(kind: table, supplement: [Table], gap: 0.5em,{} [\n#set text(size: 7.5pt)\n#set align(left)\n#table(columns: {}, table.header({}),", caption.as_ref().map(|c| format!(" caption: [{}],", tx(c))).unwrap_or_default(), cols, head.iter().map(|h| format!("[*{}*]", tx(h))).collect::<Vec<_>>().join(", "));
                for r in rows {
                    let _ = writeln!(o, "  {},", r.iter().map(|c| format!("[{}]", tx(c))).collect::<Vec<_>>().join(", "));
                }
                o.push_str(")])\n\n");
            }
            Block::Fig { path, caption } => {
                let _ = writeln!(o, "#page(flipped: true, margin: (x: 1.3cm, y: 1.2cm))[\n#figure(image(\"{path}\", width: 100%, height: 15.2cm, fit: \"contain\"), caption: [{}])\n]\n", tx(caption));
            }
            Block::Pagebreak => o.push_str("#pagebreak()\n"),
        }
    }
    o
}

// ---------------------------------------------------------------- formatting helpers

fn fx(x: f64, d: usize) -> String {
    if x.is_nan() {
        "n/a".into()
    } else {
        format!("{x:.d$}")
    }
}
fn fsig(x: f64) -> String {
    if x.is_nan() {
        return "n/a".into();
    }
    if x == 0.0 {
        return "0".into();
    }
    let a = x.abs();
    if !(1e-3..1e4).contains(&a) {
        format!("{x:.3e}")
    } else if a >= 100.0 {
        format!("{x:.1}")
    } else if a >= 1.0 {
        format!("{x:.3}")
    } else {
        format!("{x:.4}")
    }
}
fn mean(v: &[f64]) -> f64 {
    if v.is_empty() {
        f64::NAN
    } else {
        v.iter().sum::<f64>() / v.len() as f64
    }
}
fn minmax(v: &[f64]) -> (f64, f64) {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for &x in v {
        lo = lo.min(x);
        hi = hi.max(x);
    }
    (lo, hi)
}

struct Inputs {
    files: BTreeMap<String, String>, // path relative to study -> sha256
}
impl Inputs {
    fn note(&mut self, study: &Path, p: &Path) {
        let rel = p.strip_prefix(study).unwrap_or(p).to_string_lossy().to_string();
        self.files.entry(rel).or_insert_with(|| sha256(p));
    }
}

// ================================================================ P1

#[derive(Clone)]
struct P1Arm {
    name: String,
    role: &'static str,
    // metric -> (ratio, lo, hi)
    m: BTreeMap<String, (f64, f64, f64)>,
    ident: f64,
}
struct P1Cell {
    config: String,
    kind: String,
    n_arms: i64,
    arms: Vec<P1Arm>,
    icc: Vec<(String, i64, i64, Vec<(String, bool, Option<f64>)>)>,
}
struct P1 {
    ok: bool,
    cells: Vec<P1Cell>,
    ci_level: f64,
    boot: f64,
    rows: i64,
    masked: bool,
    header: J,
    status: BTreeMap<(String, String, String), i64>, // (config, kind, status|not formed role) -> n
    not_formed: BTreeMap<(String, String), i64>,
    not_formed_reason: BTreeSet<String>,
    log_done: String,
    key_sha: String,
}
const P1_METRICS: [(&str, &str); 5] = [
    ("D_solve", "D_solve (XL closure degree)"),
    ("first_fall_degree_obs", "first fall degree (obs.)"),
    ("final_rank", "final rank"),
    ("field_mults", "field mults"),
    ("cpu_s", "cpu_s (descriptive)"),
];
fn p1_role(name: &str) -> &'static str {
    if name == "AA_repeat" {
        "AA_repeat"
    } else if name == "AA_perm" {
        "AA_perm"
    } else if name == "iso" {
        "iso"
    } else if name.starts_with("rnd") {
        "random"
    } else if name.starts_with('n') && name[1..].chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
        "neighbour"
    } else if name == "E0" {
        "E0"
    } else {
        "other"
    }
}
fn load_p1(study: &Path, inp: &mut Inputs) -> P1 {
    let dir = study.join("p1_eccp131/runs");
    let aj = dir.join("tier1_analysis.json");
    let kj = dir.join("tier1_analysis.json.key.json");
    let mut p1 = P1 { ok: false, cells: vec![], ci_level: f64::NAN, boot: f64::NAN, rows: 0, masked: false, header: J::Null, status: BTreeMap::new(), not_formed: BTreeMap::new(), not_formed_reason: BTreeSet::new(), log_done: String::new(), key_sha: String::new() };
    let Some(a) = read_json(&aj) else { return p1 };
    inp.note(study, &aj);
    let key: BTreeMap<String, String> = read_json(&kj)
        .map(|k| {
            inp.note(study, &kj);
            k.obj().iter().filter_map(|(a, b)| b.s().map(|s| (a.clone(), s.to_string()))).collect()
        })
        .unwrap_or_default();
    p1.key_sha = if kj.exists() { sha256(&kj) } else { "absent".into() };
    p1.ci_level = a.get("ci_level").f().unwrap_or(f64::NAN);
    p1.boot = a.get("boot").f().unwrap_or(f64::NAN);
    p1.rows = a.get("rows").i().unwrap_or(0);
    p1.masked = a.get("arm_labels_masked").b().unwrap_or(false);
    for c in a.get("cells").arr() {
        let mut cell = P1Cell { config: c.get("config").s().unwrap_or("?").into(), kind: c.get("kind").s().unwrap_or("?").into(), n_arms: c.get("n_arms").i().unwrap_or(0), arms: vec![], icc: vec![] };
        for pa in c.get("per_arm").arr() {
            let lab = pa.get("arm").s().unwrap_or("?");
            let name = key.get(lab).cloned().unwrap_or_else(|| lab.to_string());
            let mut m = BTreeMap::new();
            for (mk, mv) in pa.get("metrics").obj() {
                let r = mv.get("geo_mean_ratio_vs_E0").f().unwrap_or(f64::NAN);
                let lo = mv.get("ci95").at0().unwrap_or(f64::NAN);
                let hi = mv.get("ci95").at1().unwrap_or(f64::NAN);
                m.insert(mk.clone(), (r, lo, hi));
            }
            let role = p1_role(&name);
            cell.arms.push(P1Arm { name, role, m, ident: pa.get("identical_count_profile_fraction").f().unwrap_or(f64::NAN) });
        }
        for (g, gv) in c.get("icc").obj() {
            let mut ms = Vec::new();
            for (mk, mv) in gv.get("metrics").obj() {
                ms.push((mk.clone(), mv.get("all_values_equal").b().unwrap_or(false), mv.get("curve_variance_share").f()));
            }
            cell.icc.push((g.clone(), gv.get("n_curves").i().unwrap_or(0), gv.get("n_targets").i().unwrap_or(0), ms));
        }
        p1.cells.push(cell);
    }
    // rows: header and status counts
    let rp = dir.join("tier1_rows.jsonl.gz");
    if rp.exists() {
        inp.note(study, &rp);
        let mut first = true;
        let mut st: BTreeMap<(String, String, String), i64> = BTreeMap::new();
        let mut nf: BTreeMap<(String, String), i64> = BTreeMap::new();
        let mut reasons = BTreeSet::new();
        let mut hdr = J::Null;
        for_lines(&rp, |l| {
            if l.trim().is_empty() {
                return;
            }
            let Ok(j) = parse_json(l) else { return };
            if first {
                first = false;
                hdr = j;
                return;
            }
            let cfg = j.get("config").s().unwrap_or("?").to_string();
            let kind = j.get("kind").s().unwrap_or("?").to_string();
            match j.get("status").s() {
                Some(s) => *st.entry((cfg, kind, s.to_string())).or_default() += 1,
                None => {
                    *st.entry((cfg.clone(), kind.clone(), "not formed".into())).or_default() += 1;
                    *nf.entry((cfg, j.get("role").s().unwrap_or("?").to_string())).or_default() += 1;
                    if let Some(e) = j.get("error").s() {
                        reasons.insert(e.to_string());
                    }
                }
            }
        });
        p1.header = hdr;
        p1.status = st;
        p1.not_formed = nf;
        p1.not_formed_reason = reasons;
    }
    let lp = dir.join("tier1.log.gz");
    if lp.exists() {
        inp.note(study, &lp);
        let mut last = String::new();
        for_lines(&lp, |l| {
            if l.starts_with("done ") {
                last = l.to_string();
            }
        });
        p1.log_done = last;
    }
    p1.ok = !p1.cells.is_empty();
    p1
}
trait At {
    fn at0(&self) -> Option<f64>;
    fn at1(&self) -> Option<f64>;
}
impl At for J {
    fn at0(&self) -> Option<f64> {
        self.arr().first().and_then(|x| x.f())
    }
    fn at1(&self) -> Option<f64> {
        self.arr().get(1).and_then(|x| x.f())
    }
}

fn p1_range(cell: &P1Cell, metric: &str, skip_perm: bool) -> (f64, f64) {
    let v: Vec<f64> = cell.arms.iter().filter(|a| !(skip_perm && a.role == "AA_perm")).filter_map(|a| a.m.get(metric).map(|x| x.0)).filter(|x| !x.is_nan()).collect();
    minmax(&v)
}
fn p1_role_color(r: &str) -> &'static str {
    match r {
        "neighbour" => BLUE,
        "random" => ORANGE,
        "iso" => GREEN,
        "AA_repeat" => GREY,
        "AA_perm" => VERM,
        _ => INK,
    }
}
const P1_LANES: [&str; 5] = ["neighbour", "random", "iso", "AA_repeat", "AA_perm"];

fn fig_p1(p1: &P1, figdir: &Path) -> Option<String> {
    if !p1.ok {
        return None;
    }
    let ncell = p1.cells.len();
    let lm = 150.0;
    let pw = 178.0;
    let gap = 14.0;
    let ph = 92.0;
    let rgap = 12.0;
    let top = 112.0;
    let w = lm + 5.0 * (pw + gap) + 10.0;
    let h = top + ncell as f64 * (ph + rgap) + 78.0;
    let mut s = Svg::new(w, h);
    s.text(12.0, 22.0, "P1 ECCp-131 tier 1: per-arm ratio to E0 (geometric mean over paired targets), one point per arm", 13.0, "start", INK, "font-weight=\"bold\"");
    s.wrap(12.0, 40.0, &format!("Recorded values of tier1_analysis.json (masked analysis, labels mapped with the key opened after the freeze). Whiskers: ci95 field at ci_level {} ({} resamples).", p1.ci_level, p1.boot), 9.5, w - 24.0, MUTED);
    // legend
    let mut lx = 12.0;
    for (i, r) in P1_LANES.iter().enumerate() {
        s.circle(lx + 4.0, 66.0, 4.0, p1_role_color(r), "none", 0.0, "");
        let label = match *r {
            "neighbour" => "Elkies neighbour (27)",
            "random" => "random non-isogenous curve (27)",
            "iso" => "isomorphic model of E0",
            "AA_repeat" => "A/A repeat of E0",
            _ => "A/A, variable order permuted",
        };
        s.text(lx + 12.0, 70.0, label, 9.5, "start", INK, "");
        lx += 18.0 + label.len() as f64 * 5.6 * TS + if i < 4 { 14.0 } else { 0.0 };
    }
    s.wrap(12.0, 86.0, "Lane order inside each panel, top to bottom: neighbour, random, isomorphic, A/A repeat, A/A permuted variable order. Panel x range is shared per column.", 9.0, w - 24.0, MUTED);
    // x ranges per metric
    let mut ranges = Vec::new();
    for (mk, _) in P1_METRICS.iter() {
        let mut lo = 1.0f64;
        let mut hi = 1.0f64;
        for c in &p1.cells {
            for a in &c.arms {
                if let Some(&(r, l, u)) = a.m.get(*mk) {
                    for x in [r, l, u] {
                        if !x.is_nan() {
                            lo = lo.min(x);
                            hi = hi.max(x);
                        }
                    }
                }
            }
        }
        let mut span = hi - lo;
        if span < 0.02 {
            lo = 1.0 - 0.05;
            hi = 1.0 + 0.05;
            span = 0.1;
        }
        let pad = span * 0.06;
        ranges.push((lo - pad, hi + pad));
    }
    for (ci, (mk, ml)) in P1_METRICS.iter().enumerate() {
        let x0 = lm + ci as f64 * (pw + gap);
        s.text(x0 + pw / 2.0, top - 6.0, ml, 10.0, "middle", INK, "font-weight=\"bold\"");
        let _ = mk;
    }
    for (ri, c) in p1.cells.iter().enumerate() {
        let y0 = top + ri as f64 * (ph + rgap);
        s.text(lm - 8.0, y0 + ph / 2.0 - 4.0, &format!("{} {}", c.config, c.kind), 10.5, "end", INK, "font-weight=\"bold\"");
        s.text(lm - 8.0, y0 + ph / 2.0 + 9.0, &format!("{} arm{} vs E0", c.n_arms - 1, if c.n_arms - 1 == 1 { "" } else { "s" }), 9.0, "end", MUTED, "");
        for (ci, (mk, _)) in P1_METRICS.iter().enumerate() {
            let x0 = lm + ci as f64 * (pw + gap);
            let (lo, hi) = ranges[ci];
            let xs = |v: f64| x0 + 6.0 + (v - lo) / (hi - lo) * (pw - 12.0);
            s.rect(x0, y0, pw, ph, "#f6f6f6", "#cccccc", "stroke-width=\"0.6\"");
            for t in nice_ticks(lo + (hi - lo) * 0.06, hi - (hi - lo) * 0.06, 5) {
                s.line(xs(t), y0, xs(t), y0 + ph, "#e2e2e2", 0.5, "");
            }
            s.line(xs(1.0), y0, xs(1.0), y0 + ph, INK, 0.9, "stroke-dasharray=\"3,2\"");
            let mut maxdev = 0.0f64;
            for (idx, a) in c.arms.iter().enumerate() {
                let Some(&(r, l, u)) = a.m.get(*mk) else { continue };
                if r.is_nan() {
                    continue;
                }
                maxdev = maxdev.max((r - 1.0).abs());
                let lane = P1_LANES.iter().position(|x| *x == a.role).unwrap_or(0) as f64;
                let y = y0 + 9.0 + lane * (ph - 18.0) / 4.0 + hash01(&format!("{}{}", a.name, idx)) * 2.4;
                if !l.is_nan() && !u.is_nan() && (u - l) > 1e-9 {
                    s.line(xs(l), y, xs(u), y, p1_role_color(a.role), 0.9, "opacity=\"0.45\"");
                }
                s.circle(xs(r), y, 2.4, p1_role_color(a.role), "#ffffff", 0.4, "opacity=\"0.9\"");
            }
            if maxdev < 5e-4 {
                s.text(x0 + 5.0, y0 + 11.0, "all arms = 1.000", 8.0, "start", MUTED, "");
            }
            if ri == ncell - 1 {
                for t in nice_ticks(lo + (hi - lo) * 0.06, hi - (hi - lo) * 0.06, 3) {
                    s.text(xs(t), y0 + ph + 12.0, &fx(t, 3), 8.5, "middle", MUTED, "");
                }
            }
        }
    }
    let yb = top + ncell as f64 * (ph + rgap) + 22.0;
    s.wrap(12.0, yb, "Dashed line: ratio 1 (E0). Ratio = arm over E0, geometric mean over the paired (target, rep) instances; counted metrics are exact, cpu_s is a clock taken at L0 on a loaded host and is descriptive only. Source data: p1_arm_ratio_strip.data.tsv. D_solve is the XL/Macaulay closure degree of this engine, not an F4 solving degree and not a degree of regularity.", 9.0, w - 24.0, MUTED);
    let svg = s.finish("P1 arm-ratio strip plot");
    let _ = std::fs::write(figdir.join("p1_arm_ratio_strip.svg"), svg);
    let mut t = String::from("cell\tarm\trole\tmetric\tratio_vs_E0\tci_lo\tci_hi\n");
    for c in &p1.cells {
        for a in &c.arms {
            for (mk, _) in P1_METRICS.iter() {
                if let Some(&(r, l, u)) = a.m.get(*mk) {
                    let _ = writeln!(t, "{}/{}\t{}\t{}\t{}\t{}\t{}\t{}", c.config, c.kind, a.name, a.role, mk, r, l, u);
                }
            }
        }
    }
    let _ = std::fs::write(figdir.join("p1_arm_ratio_strip.data.tsv"), t);
    Some("figures/p1_arm_ratio_strip.svg".into())
}

// ================================================================ P2

#[derive(PartialEq, Clone, Copy)]
enum P2State {
    Done,
    Running,
    Pending,
    Deferred,
}
struct P2Cell {
    id: String,
    bits: u32,
    class: char,
    m: u32,
    state: P2State,
    an: J,
    audit: J,
    receipt: bool,
}
fn load_p2(study: &Path, inp: &mut Inputs) -> Vec<P2Cell> {
    let base = study.join("p2_prime_analogues/runs");
    let mut v = Vec::new();
    for bits in [24u32, 28, 32, 36, 40] {
        for class in ['A', 'B'] {
            for m in [2u32, 3] {
                let id = format!("c{bits}{class}_m{m}");
                let dir = base.join(&id);
                let deferred = bits >= 36;
                let mut cell = P2Cell { id: id.clone(), bits, class, m, state: if deferred { P2State::Deferred } else { P2State::Pending }, an: J::Null, audit: J::Null, receipt: false };
                let ap = dir.join("analysis.json");
                if let Some(a) = read_json(&ap) {
                    inp.note(study, &ap);
                    cell.an = a;
                    cell.state = P2State::Done;
                    let au = dir.join("audit.json");
                    if let Some(x) = read_json(&au) {
                        inp.note(study, &au);
                        cell.audit = x;
                    }
                } else if !deferred && dir.exists() {
                    cell.state = P2State::Running;
                }
                let rc = study.join(format!("cards/CARD-isogeny-ic-yield-p2-c{bits}{class}-m{m}-20261102-3.receipt.json"));
                cell.receipt = rc.exists();
                v.push(cell);
            }
        }
    }
    v
}
fn p2_yield_points(c: &P2Cell) -> Vec<(i64, i64, f64)> {
    // (curve, j, pooled_yield)
    c.an.get("per_curve").arr().iter().map(|p| (p.get("curve").i().unwrap_or(-1), p.get("j").i().unwrap_or(-1), p.get("pooled_yield").f().unwrap_or(f64::NAN))).collect()
}
fn fig_p2(cells: &[P2Cell], figdir: &Path) -> String {
    let run: Vec<&P2Cell> = cells.iter().filter(|c| c.state != P2State::Deferred).collect();
    let classes: Vec<(u32, char)> = vec![(24, 'A'), (24, 'B'), (28, 'A'), (28, 'B'), (32, 'A'), (32, 'B')];
    let pw = 192.0;
    let ph = 175.0;
    let lm = 54.0;
    let gap = 10.0;
    let top = 128.0;
    let foot = 70.0;
    let w = lm + classes.len() as f64 * (pw + gap) + 6.0;
    let h = top + 2.0 * (ph + foot + 22.0) + 70.0;
    let mut s = Svg::new(w, h);
    s.text(12.0, 22.0, "P2 prime analogues: yield per curve relative to the class mean, with the A/A band and the planted-control effect", 13.0, "start", INK, "font-weight=\"bold\"");
    s.wrap(12.0, 40.0, "Each point is one curve of the class (sorted by value): pooled relation yield per attempt divided by the class grand mean. Counted metric: exact.", 9.5, w - 24.0, MUTED);
    s.rect(12.0, 56.0, 18.0, 9.0, "#cfe3f3", "none", "");
    s.text(36.0, 65.0, "A/A band: grand mean x exp(+-q95 |log ratio|) of the round-parity halves of the same curve (half-data scale, so conservative for full-data points)", 9.0, "start", INK, "");
    s.line(12.0, 80.0, 30.0, 80.0, VERM, 1.4, "stroke-dasharray=\"4,2\"");
    s.text(36.0, 83.0, "planted +20% control: median observed pc/main yield ratio when one curve's cells are replaced by the planted arm", 9.0, "start", INK, "");
    s.circle(21.0, 96.0, 3.0, VERM, "none", 0.0, "");
    s.text(36.0, 99.0, "curve farthest from the class mean in this cell.  Grey hatched panel: pending (not finished yet).  Deferred 36- and 40-bit cells: not run (see table).", 9.0, "start", INK, "");
    // shared y range over finished cells
    let mut ylo = 1.0f64;
    let mut yhi = 1.0f64;
    for c in &run {
        if c.state != P2State::Done {
            continue;
        }
        let gm = c.an.path("metrics.yield.grand_mean").f().unwrap_or(f64::NAN);
        for (_, _, y) in p2_yield_points(c) {
            if gm > 0.0 && !y.is_nan() {
                ylo = ylo.min(y / gm);
                yhi = yhi.max(y / gm);
            }
        }
        let q = c.an.path("metrics.yield.aa_round_halves.q95_abs_log_ratio").f().unwrap_or(0.0);
        ylo = ylo.min((-q).exp());
        yhi = yhi.max(q.exp());
        let pe = c.an.path("metrics.yield.positive_control.planted_effect_pc_over_main_median").f().unwrap_or(1.0);
        ylo = ylo.min(pe);
        yhi = yhi.max(pe);
    }
    let pad = (yhi - ylo) * 0.05;
    ylo = (ylo - pad).max(0.2);
    yhi += pad;
    let mut data = String::from("cell\tcurve\tj\tpooled_yield\tratio_to_grand_mean\n");
    for (mi, m) in [2u32, 3].iter().enumerate() {
        for (ci, (bits, cl)) in classes.iter().enumerate() {
            let x0 = lm + ci as f64 * (pw + gap);
            let y0 = top + mi as f64 * (ph + foot + 22.0) + 14.0;
            let cell = run.iter().find(|c| c.bits == *bits && c.class == *cl && c.m == *m).unwrap();
            let ys = |v: f64| y0 + ph - (v - ylo) / (yhi - ylo) * ph;
            s.rect(x0, y0, pw, ph, "#fafafa", "#bbbbbb", "stroke-width=\"0.6\"");
            if ci == 0 {
                for t in nice_ticks(ylo + (yhi - ylo) * 0.05, yhi - (yhi - ylo) * 0.05, 5) {
                    s.line(x0, ys(t), x0 + pw, ys(t), "#ececec", 0.5, "");
                    s.text(x0 - 4.0, ys(t) + 3.0, &fx(t, 2), 8.5, "end", MUTED, "");
                }
            } else {
                for t in nice_ticks(ylo + (yhi - ylo) * 0.05, yhi - (yhi - ylo) * 0.05, 5) {
                    s.line(x0, ys(t), x0 + pw, ys(t), "#ececec", 0.5, "");
                }
            }
            let title = match cell.state {
                P2State::Done => format!("c{}{} m={} ({} curves)", bits, cl, m, cell.an.path("metrics.yield.curves").i().unwrap_or(0)),
                _ => format!("c{}{} m={}", bits, cl, m),
            };
            s.text(x0, y0 - 4.0, &title, 9.5, "start", INK, "font-weight=\"bold\"");
            if cell.state != P2State::Done {
                s.rect(x0, y0, pw, ph, "url(#hatch)", "none", "");
                let label = if cell.state == P2State::Running { "pending (running)" } else { "pending" };
                s.text(x0 + pw / 2.0, y0 + ph / 2.0, label, 11.0, "middle", MUTED, "font-weight=\"bold\"");
                s.text(x0 + 4.0, y0 + ph + 13.0, "no values yet: not a null, not zero", 8.5, "start", MUTED, "");
                continue;
            }
            let gm = cell.an.path("metrics.yield.grand_mean").f().unwrap_or(f64::NAN);
            let q = cell.an.path("metrics.yield.aa_round_halves.q95_abs_log_ratio").f().unwrap_or(0.0);
            let (b_lo, b_hi) = ((-q).exp(), q.exp());
            s.rect(x0, ys(b_hi), pw, ys(b_lo) - ys(b_hi), "#cfe3f3", "none", "opacity=\"0.8\"");
            s.line(x0, ys(1.0), x0 + pw, ys(1.0), "#2b6a99", 0.8, "");
            let pe = cell.an.path("metrics.yield.positive_control.planted_effect_pc_over_main_median").f().unwrap_or(f64::NAN);
            if !pe.is_nan() {
                s.line(x0, ys(pe), x0 + pw, ys(pe), VERM, 1.3, "stroke-dasharray=\"4,2\"");
                s.text(x0 + pw - 3.0, ys(pe) + 11.0, &format!("planted {}", fx(pe, 3)), 8.5, "end", VERM, "");
            }
            let mut pts: Vec<(i64, i64, f64)> = p2_yield_points(cell);
            pts.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));
            let n = pts.len().max(1) as f64;
            let ext = pts.iter().map(|p| (p.2 / gm - 1.0).abs()).fold(0.0f64, f64::max);
            for (i, (cu, j, y)) in pts.iter().enumerate() {
                let r = y / gm;
                let x = x0 + 5.0 + (i as f64 + 0.5) / n * (pw - 10.0);
                let _ = writeln!(data, "{}\t{}\t{}\t{}\t{}", cell.id, cu, j, y, r);
                if ((r - 1.0).abs() - ext).abs() < 1e-12 {
                    s.circle(x, ys(r), 3.0, VERM, "#ffffff", 0.5, "");
                } else {
                    s.circle(x, ys(r), 1.7, BLUE, "none", 0.0, "opacity=\"0.85\"");
                }
            }
            let ym = "metrics.yield";
            let yv = |k: &str| cell.an.path(&format!("{ym}.{k}")).f().unwrap_or(f64::NAN);
            let pcd = |k: &str| cell.an.path(&format!("{ym}.positive_control.{k}")).f().unwrap_or(f64::NAN);
            let ly = y0 + ph + 11.0;
            s.text(x0 + 3.0, ly, &format!("ICC {} perm p {}", fx(yv("icc_raw"), 4), fx(yv("icc_perm_p"), 3)), 8.3, "start", INK, "");
            s.text(x0 + 3.0, ly + 11.5, &format!("max|z| {} (q95 {})", fx(yv("max_abs_z"), 2), fx(yv("max_abs_z_perm_null_q95"), 2)), 8.3, "start", INK, "");
            s.text(x0 + 3.0, ly + 23.0, &format!("   perm p {}", fx(yv("max_abs_z_perm_p"), 4)), 8.3, "start", INK, "");
            s.text(x0 + 3.0, ly + 34.5, &format!("max/min {} (q95 {})", fx(yv("max_over_min"), 3), fx(yv("max_over_min_perm_null_q95"), 3)), 8.3, "start", INK, "");
            let pw_min = pcd("detect_icc_gt_null_q95").min(pcd("detect_max_abs_z_gt_null_q95")).min(pcd("detect_max_over_min_gt_null_q95"));
            let ok = pw_min >= 0.9;
            s.text(x0 + 3.0, ly + 46.0, &format!("planted detect {}/{}/{}", fx(pcd("detect_icc_gt_null_q95"), 2), fx(pcd("detect_max_abs_z_gt_null_q95"), 2), fx(pcd("detect_max_over_min_gt_null_q95"), 2)), 8.3, "start", if ok { INK } else { VERM }, "");
            if !ok {
                s.text(x0 + 3.0, ly + 57.5, "power < 0.9: not informative", 8.3, "start", VERM, "font-weight=\"bold\"");
            }
        }
    }
    // defs for hatch
    let mut body = String::from("<defs><pattern id=\"hatch\" width=\"8\" height=\"8\" patternUnits=\"userSpaceOnUse\" patternTransform=\"rotate(45)\"><rect width=\"8\" height=\"8\" fill=\"#f1f1f1\"/><line x1=\"0\" y1=\"0\" x2=\"0\" y2=\"8\" stroke=\"#cfcfcf\" stroke-width=\"2\"/></pattern></defs>\n");
    body.push_str(&s.b);
    s.b = body;
    s.wrap(12.0, h - 44.0, "x axis: curves of the class sorted by yield (one point per curve). y axis: yield / class grand mean, shared across panels. Rows: m = 2 (top), m = 3 (bottom). Columns: class A (h = 128) and class B (h = 300) at 24, 28 and 32 bits. Source data: p2_yield_per_curve.data.tsv; per-cell statistics from runs/<cell>/analysis.json (frozen p2tool analyze). Positive-control detection: fraction of 200 plantings flagged at the permutation-null q95.", 9.0, w - 24.0, MUTED);
    let _ = std::fs::write(figdir.join("p2_yield_per_curve.svg"), s.finish("P2 yield per curve with A/A band and planted control"));
    let _ = std::fs::write(figdir.join("p2_yield_per_curve.data.tsv"), data);
    "figures/p2_yield_per_curve.svg".into()
}

// ================================================================ P3

#[derive(Clone)]
struct R3 {
    stem: String,
    n: i64,
    k: i64,
    arm: String,
    label: String,
    curve: i64,
    orbit: String,
    attempt: i64,
    pres: i64,
    outcome: String,
    dreg: i64,
    refuted: bool,
    solutions: i64,
    planted_ok: i64,
}
struct P3Card {
    tag: &'static str,
    stems: Vec<&'static str>,
    desc: &'static str,
}
fn p3_cards() -> Vec<P3Card> {
    vec![
        P3Card { tag: "a-ctrl-k3", stems: vec!["p3a_ctrl_k3"], desc: "P3a controls and C2 family, n=43, k=3" },
        P3Card { tag: "a-main-k3", stems: vec!["p3a_main_k3"], desc: "P3a main, n=43, k=3, 20 attempts" },
        P3Card { tag: "a-m2", stems: vec!["p3a_m2"], desc: "P3a m=2, n=43, k=3,4,5,8" },
        P3Card { tag: "a-main-k5", stems: vec!["p3a_main_k5"], desc: "P3a main, n=43, k=5 (bound)" },
        P3Card { tag: "a-ctrl-k4-s0", stems: vec!["p3a_ctrl_k4.s0"], desc: "P3a controls and C2 family, k=4, attempt 0" },
        P3Card { tag: "a-ctrl-k4-s1", stems: vec!["p3a_ctrl_k4.s1"], desc: "P3a controls and C2 family, k=4, attempt 1" },
        P3Card { tag: "a-main-k4-s0", stems: vec!["p3a_main_k4.s0"], desc: "P3a main, k=4, attempt 0" },
        P3Card { tag: "a-main-k4-s1", stems: vec!["p3a_main_k4.s1"], desc: "P3a main, k=4, attempt 1" },
        P3Card { tag: "b-badapt", stems: vec!["p3b_badapt"], desc: "P3b b-adapted presentations, n=131, k=3..6" },
        P3Card { tag: "b-bsparse", stems: vec!["p3b_bsparse"], desc: "P3b b-sparse and nbadapt, n=131, k=5,6" },
        P3Card { tag: "b-main-k5-s0", stems: vec!["p3b_main_k5.s0"], desc: "P3b random-V main, k=5, attempt 0" },
        P3Card { tag: "b-main-k5-s1", stems: vec!["p3b_main_k5.s1"], desc: "P3b random-V main, k=5, attempt 1" },
        P3Card { tag: "b-ctrl", stems: vec!["p3b_ctrl"], desc: "P3b controls, CTRL131, k=3,4,5" },
        P3Card { tag: "b-main-k34", stems: vec!["p3b_main_k34"], desc: "P3b random-V main, k=3,4" },
        P3Card { tag: "b-stdv-s0", stems: vec!["p3b_stdv.s0a", "p3b_stdv.s0b", "p3b_stdv.s0c"], desc: "P3b standard V, curves 0..131 (E0 and orbit A)" },
        P3Card { tag: "b-stdv-s1", stems: vec!["p3b_stdv.s1a", "p3b_stdv.s1b", "p3b_stdv.s1c"], desc: "P3b standard V, curves 132..262 (orbit B)" },
    ]
}
fn p3_file(full: &Path, stem: &str) -> Option<PathBuf> {
    let gz = full.join(format!("{stem}.jsonl.gz"));
    let pl = full.join(format!("{stem}.jsonl"));
    if gz.exists() {
        Some(gz)
    } else if pl.exists() {
        Some(pl)
    } else {
        None
    }
}
struct P3 {
    rows: Vec<R3>,
    stem_rows: BTreeMap<String, usize>,
    stem_unparsed: BTreeMap<String, usize>,
    card_state: Vec<(String, String, Vec<(String, Option<usize>)>, String)>, // tag, desc, stems with line counts, state
}
fn load_p3(study: &Path, inp: &mut Inputs) -> P3 {
    let full = study.join("p3_ecc2k130/full");
    let mut p3 = P3 { rows: vec![], stem_rows: BTreeMap::new(), stem_unparsed: BTreeMap::new(), card_state: vec![] };
    for card in p3_cards() {
        // A card may be reissued (-2 after a mis-sized timeout, TIMEOUT_NOTE.md): any completed revision counts.
        let has_receipt = (1..=9).any(|rev| {
            let receipt = study.join(format!("cards/CARD-isogeny-ic-yield-p3-{}-20261102-{}.receipt.json", card.tag, rev));
            read_json(&receipt).map(|j| j.get("status").s() == Some("completed")).unwrap_or(false)
        });
        let mut stems = Vec::new();
        let mut any = false;
        for st in &card.stems {
            if let Some(p) = p3_file(&full, st) {
                any = true;
                inp.note(study, &p);
                let mut cnt = 0usize;
                let mut bad = 0usize;
                let parse = !st.starts_with("p3a_m2"); // m=2 rows are only counted
                let mut rows_here = Vec::new();
                for_lines(&p, |l| {
                    if l.trim().is_empty() {
                        return;
                    }
                    cnt += 1;
                    if !parse {
                        return;
                    }
                    match parse_json(l) {
                        Ok(j) => {
                            let g = |k: &str| j.get(k).i().unwrap_or(-1);
                            rows_here.push(R3 {
                                stem: st.to_string(),
                                n: g("n"),
                                k: g("k"),
                                arm: j.get("arm").s().unwrap_or("?").into(),
                                label: j.get("label").s().unwrap_or("?").into(),
                                curve: g("curve"),
                                orbit: j.get("orbit").s().unwrap_or("?").into(),
                                attempt: g("attempt"),
                                pres: g("presentation"),
                                outcome: j.get("outcome").s().unwrap_or("?").into(),
                                dreg: g("dreg"),
                                refuted: j.get("refuted").b().unwrap_or(false),
                                solutions: g("solutions"),
                                planted_ok: g("planted_ok"),
                            });
                        }
                        Err(_) => bad += 1,
                    }
                });
                p3.stem_rows.insert(st.to_string(), cnt);
                p3.stem_unparsed.insert(st.to_string(), bad);
                p3.rows.extend(rows_here);
                stems.push((st.to_string(), Some(cnt)));
            } else {
                stems.push((st.to_string(), None));
            }
        }
        let state = if has_receipt {
            "completed (receipt)"
        } else if any {
            "running or partial (output present, no receipt)"
        } else {
            "pending"
        };
        p3.card_state.push((card.tag.into(), card.desc.into(), stems, state.into()));
    }
    p3
}

#[derive(Clone)]
struct Series {
    key: &'static str,
    name: &'static str,
}
const SERIES: [Series; 6] = [Series { key: "main", name: "random V (main)" }, Series { key: "stdv", name: "standard V" }, Series { key: "badapt", name: "b-adapted" }, Series { key: "bsparse", name: "b-sparse" }, Series { key: "nbadapt", name: "nbadapt (dense-b control)" }, Series { key: "null", name: "null curves (random b)" }];
fn series_style(k: &str) -> (&'static str, &'static str) {
    // (colour, shape)
    match k {
        "main" => (GREY, "circle"),
        "stdv" => (BLUE, "circle"),
        "badapt" => (VERM, "diamond"),
        "bsparse" => (GREEN, "square"),
        "nbadapt" => (PURPLE, "cross"),
        "null" => (GREY, "ring"),
        _ => (INK, "circle"),
    }
}
fn exp_curves(n: i64) -> usize {
    // E0 plus the level curves: n=43 -> 258 (259 curves); n=131 -> 262 (263 curves)
    if n == 43 {
        259
    } else {
        263
    }
}
struct Pt {
    curve: i64,
    label: String,
    orbit: String,
    mean: f64,
    bound: bool,
    att: usize,
}
fn series_points(rows: &[R3], n: i64, k: i64, arm: &str) -> Vec<Pt> {
    // per curve: per attempt the minimum Dreg over presentations (bound if the minimising row is not resolved), then mean over attempts
    let mut by: BTreeMap<(i64, i64), Vec<&R3>> = BTreeMap::new();
    for r in rows.iter().filter(|r| r.n == n && r.k == k && r.arm == arm) {
        by.entry((r.curve, r.attempt)).or_default().push(r);
    }
    // dedupe identical (curve, attempt, pres) rows that appear in two shards
    let mut per_curve: BTreeMap<i64, Vec<(f64, bool, String, String)>> = BTreeMap::new();
    for ((curve, _att), v) in by {
        let mut seen = BTreeSet::new();
        let mut best: Option<(i64, bool)> = None;
        let mut lab = String::new();
        let mut orb = String::new();
        for r in v {
            if !seen.insert(r.pres) {
                continue;
            }
            lab = r.label.clone();
            orb = r.orbit.clone();
            let b = r.outcome != "resolved";
            match best {
                None => best = Some((r.dreg, b)),
                Some((d, _)) if r.dreg < d => best = Some((r.dreg, b)),
                _ => {}
            }
        }
        if let Some((d, b)) = best {
            per_curve.entry(curve).or_default().push((d as f64, b, lab, orb));
        }
    }
    per_curve
        .into_iter()
        .map(|(curve, v)| Pt { curve, label: v[0].2.clone(), orbit: v[0].3.clone(), mean: mean(&v.iter().map(|x| x.0).collect::<Vec<_>>()), bound: v.iter().any(|x| x.1), att: v.len() })
        .collect()
}
fn orbit_class(o: &str) -> &str {
    o
}

fn fig_p3(p3: &P3, n: i64, ks: &[i64], figdir: &Path) -> String {
    let w = 1000.0;
    let lm = 62.0;
    let rm = 18.0;
    let pwid = w - lm - rm;
    let ph = 100.0;
    let top = 120.0;
    let gap = 42.0;
    let h = top + ks.len() as f64 * (ph + gap) + 52.0;
    let mut s = Svg::new(w, h);
    let total = exp_curves(n);
    let maxidx = total as f64 - 1.0;
    s.text(12.0, 22.0, &format!("P3 ECC2K-130 analogue, n = {n}: Dreg per curve, by presentation family (E0 ringed)"), 13.0, "start", INK, "font-weight=\"bold\"");
    s.wrap(12.0, 40.0, "Point = mean over attempts of the per-attempt minimum Dreg over a curve's presentations in that family (recorded dreg; hollow triangle = bound, Dreg > dmax). x = curve index, 0 = E0; bands = Frobenius orbits. Each family drawn at a small vertical offset for legibility (the value is the point's centre line). Right strip: random-b curves (arm null, and the nbadapt control).", 9.3, w - 24.0, MUTED);
    // legend
    let mut lx = 12.0;
    for sr in SERIES.iter() {
        let (c, shape) = series_style(sr.key);
        draw_marker(&mut s, lx + 5.0, 100.0, shape, c, 4.0);
        s.text(lx + 14.0, 103.5, sr.name, 9.5, "start", INK, "");
        lx += 30.0 + sr.name.len() as f64 * 5.7 * TS;
    }
    s.circle(lx + 5.0, 100.0, 7.0, "none", INK, 1.4, "");
    s.text(lx + 17.0, 103.5, "E0", 9.5, "start", INK, "font-weight=\"bold\"");
    let mut data = String::from("n\tk\tseries\tcurve\tlabel\torbit\tmean_dreg_over_attempts\tbound\tattempts\n");
    let all_orbits: BTreeMap<i64, String> = p3.rows.iter().filter(|r| r.n == n && r.curve < 100000).map(|r| (r.curve, r.orbit.clone())).collect();
    for (pi, &k) in ks.iter().enumerate() {
        let y0 = top + pi as f64 * (ph + gap);
        let xs = |c: f64| if c >= 100000.0 { lm + pwid - 62.0 + (c - 100000.0) * 3.2 } else { lm + 8.0 + c / maxidx * (pwid - 88.0) };
        s.rect(lm, y0, pwid, ph, "#fbfbfb", "#bbbbbb", "stroke-width=\"0.6\"");
        // orbit bands
        let mut seg: Vec<(String, i64, i64)> = Vec::new();
        for (&c, o) in &all_orbits {
            match seg.last_mut() {
                Some(l) if l.0 == *o && l.2 + 1 == c => l.2 = c,
                _ => seg.push((o.clone(), c, c)),
            }
        }
        for (i, (o, a, b)) in seg.iter().enumerate() {
            let xa = xs(*a as f64) - 3.0;
            let xb = xs(*b as f64) + 3.0;
            if i % 2 == 1 {
                s.rect(xa, y0, xb - xa, ph, "#eef1f5", "none", "");
            }
            if xb - xa > 18.0 {
                s.text((xa + xb) / 2.0, y0 + 10.0, &format!("orbit {}", orbit_class(o)), 8.5, "middle", MUTED, "");
            }
        }
        // series
        let mut ser: Vec<(&Series, Vec<Pt>)> = Vec::new();
        for sr in SERIES.iter() {
            ser.push((sr, series_points(&p3.rows, n, k, sr.key)));
        }
        let nullpts: Vec<f64> = Vec::new();
        let mut vals: Vec<f64> = ser.iter().flat_map(|(_, p)| p.iter().map(|x| x.mean)).collect();
        vals.extend(nullpts.iter().cloned());
        if vals.is_empty() {
            s.text(lm + pwid / 2.0, y0 + ph / 2.0, "pending: no rows for this cell", 12.0, "middle", MUTED, "font-weight=\"bold\"");
            s.text(lm - 6.0, y0 + 14.0, &format!("k={k}"), 11.0, "end", INK, "font-weight=\"bold\"");
            continue;
        }
        let (lo, hi) = minmax(&vals);
        let ylo = lo.floor() - 0.7;
        let yhi = (hi.ceil() + 0.7).max(ylo + 2.0);
        let ys = |v: f64| y0 + ph - (v - ylo) / (yhi - ylo) * ph;
        let mut t = ylo.ceil();
        while t <= yhi {
            s.line(lm, ys(t), lm + pwid, ys(t), "#e4e4e4", 0.5, "");
            s.text(lm - 5.0, ys(t) + 3.0, &format!("{t:.0}"), 8.5, "end", MUTED, "");
            t += 1.0;
        }
        s.text(lm - 28.0, y0 + ph / 2.0, &format!("k={k}"), 11.0, "end", INK, "font-weight=\"bold\"");
        s.text(lm - 28.0, y0 + ph / 2.0 + 12.0, "Dreg", 8.5, "end", MUTED, "");
        if !nullpts.is_empty() {
            let (nl, nh) = minmax(&nullpts);
            s.rect(lm, ys(nh) - 2.0, pwid, (ys(nl) - ys(nh)).max(0.0) + 4.0, "#d9d9d9", "none", "opacity=\"0.55\"");
        }
        let offs = [0.0, -0.11, 0.11, -0.22, 0.22, 0.0];
        s.line(lm + pwid - 70.0, y0, lm + pwid - 70.0, y0 + ph, "#bbbbbb", 0.8, "stroke-dasharray=\"3,3\"");
        s.text(lm + pwid - 36.0, y0 + 10.0, "random b", 8.5, "middle", MUTED, "");
        let mut status = Vec::new();
        for (si, (sr, pts)) in ser.iter().enumerate() {
            let (c, shape) = series_style(sr.key);
            if pts.is_empty() {
                status.push(format!("{}: pending", sr.key));
                continue;
            }
            if sr.key == "null" || sr.key == "nbadapt" { status.push(format!("{}: {} curves", sr.key, pts.len())); } else { status.push(format!("{}: {}/{} curves", sr.key, pts.len(), total)); }
            for p in pts {
                let _ = writeln!(data, "{n}\t{k}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", sr.key, p.curve, p.label, p.orbit, p.mean, p.bound, p.att);
                let x = xs(p.curve as f64);
                let y = ys(p.mean + offs[si]);
                if p.bound {
                    s.poly(&[(x, y - 3.5), (x - 3.2, y + 2.5), (x + 3.2, y + 2.5)], "#ffffff", c, 1.0, "");
                } else {
                    draw_marker(&mut s, x, y, shape, c, if p.curve == 0 { 3.6 } else { 2.2 });
                }
                if p.curve == 0 {
                    s.circle(x, y, 7.0, "none", INK, 1.4, "");
                }
            }
        }
        s.text(lm + 4.0, y0 + ph + 12.0, &status.join("   |   "), 8.3, "start", INK, "");
        // E0 values
        let e0: Vec<String> = ser.iter().filter_map(|(sr, p)| p.iter().find(|x| x.curve == 0).map(|x| format!("{} {}{}", sr.key, fx(x.mean, 2), if x.bound { " (bound)" } else { "" }))).collect();
        s.text(lm + pwid - 4.0, y0 + ph + 24.0, &format!("E0 mean Dreg: {}", if e0.is_empty() { "pending".to_string() } else { e0.join(", ") }), 8.5, "end", INK, "font-weight=\"bold\"");
        s.text(lm + 4.0, y0 + ph + 24.0, "right strip: random-b curves (null population and nbadapt control), not curves of the class", 8.3, "start", MUTED, "");
    }
    s.wrap(12.0, h - 42.0, &format!("Counted metrics (dreg, outcome) are exact; presentations: stdv = standard V in several moduli, badapt = V = span(1,g,..,g^(k-1)) with g adapted to b (E0 borrows the floor curves' moduli), bsparse and nbadapt as in PROTOCOL section 4. Source data: p3_dreg_n{n}.data.tsv (pooled recorded rows of p3_ecc2k130/full). A series with no row in a cell is shown as pending; bounds are never plotted as values."), 9.0, w - 24.0, MUTED);
    let name = format!("p3_dreg_n{n}");
    let _ = std::fs::write(figdir.join(format!("{name}.svg")), s.finish(&format!("P3 Dreg per curve and presentation family, n={n}")));
    let _ = std::fs::write(figdir.join(format!("{name}.data.tsv")), data);
    format!("figures/{name}.svg")
}
fn draw_marker(s: &mut Svg, x: f64, y: f64, shape: &str, c: &str, r: f64) {
    match shape {
        "diamond" => s.poly(&[(x, y - r - 0.8), (x + r + 0.8, y), (x, y + r + 0.8), (x - r - 0.8, y)], c, "none", 0.0, "opacity=\"0.9\""),
        "square" => s.rect(x - r, y - r, 2.0 * r, 2.0 * r, c, "none", "opacity=\"0.9\""),
        "cross" => {
            s.line(x - r, y - r, x + r, y + r, c, 1.3, "");
            s.line(x - r, y + r, x + r, y - r, c, 1.3, "");
        }
        "ring" => s.circle(x, y, r, "#ffffff", c, 1.0, ""),
        _ => s.circle(x, y, r, c, "none", 0.0, "opacity=\"0.85\""),
    }
}

// ================================================================ class structure diagram

fn fig_class(study: &Path, figdir: &Path, inp: &mut Inputs) -> String {
    let tp = study.join("p3_ecc2k130/data/ecc2k130_curves.tsv");
    let tsv = std::fs::read_to_string(&tp).unwrap_or_default();
    if tp.exists() {
        inp.note(study, &tp);
    }
    let mut orb: BTreeMap<String, usize> = BTreeMap::new();
    for l in tsv.lines().skip(1) {
        let f: Vec<&str> = l.split('\t').collect();
        if f.len() > 2 {
            *orb.entry(f[2].to_string()).or_default() += 1;
        }
    }
    let na = orb.get("A").copied().unwrap_or(0);
    let nb = orb.get("B").copied().unwrap_or(0);
    let w = 1000.0;
    let h = 565.0;
    let mut s = Svg::new(w, h);
    s.text(12.0, 22.0, "Class structure: ECC2K-130 (conductor 263 x p') beside ECCp-131 (conductor 1)", 13.0, "start", INK, "font-weight=\"bold\"");
    s.wrap(12.0, 40.0, "Facts from DESIGN.md (PARI 2.17, agrees with ecdb curve 22) and p3_ecc2k130/data/ecc2k130_curves.tsv. Drawn structure, not measurements. p' = 146505763881528721, about 2^57.0; only the 263 part of the conductor is walked.", 9.3, w - 24.0, MUTED);
    // ---- left: volcano
    s.text(250.0, 92.0, "ECC2K-130: 263-isogeny volcano, depth 1", 11.5, "middle", INK, "font-weight=\"bold\"");
    s.rect(20.0, 104.0, 460.0, 124.0, "#fff3e0", "#e0c9a0", "stroke-width=\"0.7\"");
    s.text(30.0, 122.0, "Level 0 · crater · 1 curve", 9.8, "start", INK, "font-weight=\"bold\"");
    s.text(30.0, 137.0, "End conductor 1 at 263", 8.8, "start", MUTED, "");
    s.ellipse(205.0, 178.0, 24.0, 13.0, "none", "#555555", 1.2, "stroke-dasharray=\"4,2\"");
    s.ellipse(295.0, 178.0, 24.0, 13.0, "none", "#555555", 1.2, "stroke-dasharray=\"4,2\"");
    s.circle(250.0, 178.0, 22.0, "#ffffff", INK, 2.0, "");
    s.text(250.0, 182.5, "E0", 12.0, "middle", INK, "font-weight=\"bold\"");
    s.text(30.0, 208.0, "2 horizontal loops (264 - 262, derived)", 8.6, "start", MUTED, "");
    s.rect(20.0, 268.0, 460.0, 168.0, "#e8f1f8", "#a8c4dc", "stroke-width=\"0.7\"");
    for x in [70.0, 130.0, 190.0, 250.0, 310.0, 370.0, 430.0] {
        s.line(250.0, 200.0, x, 320.0, "#9a9a9a", 0.8, "");
    }
    s.rect(120.0, 238.0, 260.0, 18.0, "#ffffff", "none", "opacity=\"0.9\"");
    s.text(250.0, 251.0, "262 descending 263-isogenies (verified neighbours)", 8.8, "middle", INK, "");
    for i in 0..24 {
        let x = 40.0 + i as f64 * 17.6;
        s.circle(x, 330.0, 3.6, if i < 12 { BLUE } else { VERM }, "none", 0.0, "");
    }
    s.text(40.0, 358.0, &format!("orbit A: {na} curves"), 9.0, "start", BLUE, "font-weight=\"bold\"");
    s.text(460.0, 358.0, &format!("orbit B: {nb} curves"), 9.0, "end", VERM, "font-weight=\"bold\"");
    s.text(30.0, 396.0, "Level 1 · floor · 262 curves", 9.8, "start", INK, "font-weight=\"bold\"");
    s.text(30.0, 411.0, "End conductor 263; two Frobenius orbits", 8.8, "start", MUTED, "");
    s.wrap(20.0, 458.0, "E0: b = 1 in F_2 (Koblitz curve), trace -22283658519494248867, CM by Z[(1+sqrt(-7))/2]. Isogenies preserve the group order; the descended system depends on a curve through b only. Studied here (P3): Dreg, yield and F4 time on E0 and the 262 floor curves (n = 131), and the n = 43 analogue (258 level curves, 6 orbits).", 9.0, 460.0, INK);
    // ---- right: ECCp-131
    s.text(750.0, 92.0, "ECCp-131: one crater, no vertical levels", 11.5, "middle", INK, "font-weight=\"bold\"");
    s.rect(520.0, 104.0, 460.0, 332.0, "#f1f7ee", "#b5cfa8", "stroke-width=\"0.7\"");
    s.text(530.0, 122.0, "Level 0 · crater only · h about 2^63.6 curves", 9.8, "start", INK, "font-weight=\"bold\"");
    s.wrap(530.0, 137.0, "conductor of Z[pi] = 1; fundamental discriminant -5328143084860676590809120034441032302419", 8.6, 440.0, MUTED);
    s.ellipse(750.0, 268.0, 190.0, 72.0, "#ffffff", "#7a9a6a", 1.5, "");
    s.text(750.0, 316.0, "class of E0 (not enumerable)", 9.0, "middle", MUTED, "");
    let ells = [5, 7, 13, 17, 29, 37, 61, 73, 83, 89, 107, 131, 149];
    let mut idx = 0usize;
    for l in ells.iter().chain(std::iter::once(&67)) {
        let reps = if *l == 67 { 1 } else { 2 };
        for _ in 0..reps {
            let ang = (idx as f64 / 27.0) * std::f64::consts::PI * 2.0 - std::f64::consts::FRAC_PI_2;
            s.circle(750.0 + 190.0 * ang.cos(), 268.0 + 72.0 * ang.sin(), 4.0, BLUE, "#ffffff", 0.6, "");
            idx += 1;
        }
    }
    s.circle(750.0, 268.0, 22.0, "#ffffff", INK, 2.0, "");
    s.text(750.0, 272.5, "E0", 12.0, "middle", INK, "font-weight=\"bold\"");
    s.wrap(530.0, 368.0, "Blue dots: the 27 Elkies neighbours reachable at l <= 149 by verified Velu: l = 5, 7, 13, 17, 29, 37, 61, 73, 83, 89, 107, 131, 149 (2 each) and l = 67 (1, ramified).", 8.8, 440.0, BLUE);
    s.text(750.0, 468.0, "Outside the class (null population, other group orders)", 9.0, "middle", INK, "");
    for i in 0..27 {
        s.circle(560.0 + i as f64 * 14.3, 484.0, 3.2, ORANGE, "none", 0.0, "");
    }
    s.text(750.0, 504.0, "27 random non-isogenous curves; the isomorphic model of E0 is the same curve", 8.8, "middle", ORANGE, "");
    s.text(12.0, h - 8.0, "Not a measurement figure: no data file. Facts in the tables of REPORT.md section 3.", 9.0, "start", MUTED, "");
    let _ = std::fs::write(figdir.join("class_structure.svg"), s.finish("Class structure of ECC2K-130 and ECCp-131"));
    "figures/class_structure.svg".into()
}

// ================================================================ main

fn arg(name: &str) -> Option<String> {
    let a: Vec<String> = std::env::args().collect();
    a.iter().position(|x| x == name).and_then(|i| a.get(i + 1).cloned())
}

fn main() {
    let study = PathBuf::from(arg("--study").unwrap_or_else(|| ".".into())).canonicalize().expect("study dir");
    let out = arg("--out").map(PathBuf::from).unwrap_or_else(|| study.join("report"));
    let figdir = out.join("figures");
    std::fs::create_dir_all(&figdir).unwrap();
    let mut inp = Inputs { files: BTreeMap::new() };

    let p1 = load_p1(&study, &mut inp);
    let p2 = load_p2(&study, &mut inp);
    let p3 = load_p3(&study, &mut inp);

    let fig_cls = fig_class(&study, &figdir, &mut inp);
    let fig1 = fig_p1(&p1, &figdir);
    let fig2 = fig_p2(&p2, &figdir);
    let fig3a = fig_p3(&p3, 131, &[3, 4, 5, 6], &figdir);
    let fig3b = fig_p3(&p3, 43, &[3, 4, 5], &figdir);

    let mut d = Doc { title: "Isogeny-class index-calculus yield: ECCp-131 and ECC2K-130".into(), subtitle: "Characterization study report and visuals (branch claude/isogeny-class-ic-yield-20261102; generated by report/gen/gen_report.rs, interim until every cell has finished)".into(), blocks: vec![] };

    // ---------------- 1 scope
    d.h(1, "1. What this report is");
    d.p("Question (user, 2026-11-02): across the isogeny classes of ECC2K-130 and ECCp-131, do index-calculus relation yield, solve cost, resolving degree and related parameters differ between curves of one class? The study measures this with one instrument per part, the same on every curve, with an A/A noise floor, an isomorphic-model control, a negative-control population and, where the design has one, a planted positive control (DESIGN.md; PROTOCOL.md of each part).");
    d.bullets(vec![
        "**Characterization study, not an improvement.** The preregistered expectation is a null. A null closes a lead. Nothing here is an improvement over the current index-calculus baseline, and the CLAUDE.md success criterion (every verified, measured improvement over that baseline) is not engaged: no baseline cost is changed, no phase is sped up.".into(),
        "**No leaderboard row and no benchmarks row.** There is no cross-method claim and no matched rho reference, so the validation gate (replay certificate, validator report, matched rho with the 5a band) does not apply. `docs/ic-leaderboard.html`, `docs/ic/LEADERBOARD.md`, `docs/ic/leaderboard.json` and `benchmarks/README.md` are not touched. Per-row gaps to rho are not computed here, by design.".into(),
        "**L0 isolation.** All three parts ran locally on a loaded host (P1 about 2 workers, P2 one ecbench cell at a time, P3 at most 2 workers, other agent sessions on the same 3 to 4 cores). Counted metrics (degrees, ranks, relation counts, field multiplications, row operations, solution counts) are exact and are the only ones that decide. Clocks (`cpu_s`, `wall_s`, `online_wall_ns_per_attempt`, `dreg_s`, `build_s`) are descriptive only and decide nothing.".into(),
        "**Deferred, not dropped.** P1 tier 2 (`m2t4`), P2 at 36 and 40 bits (8 cells) and P3a-Y (n=43, k=13 yield line, claim C3) were not run under the amended protocols. They stay in the preregistration and are reported as \"not run (deferred)\", never filled.".into(),
        "**Pending cells.** Cells still running at generation time show as \"pending\" in tables and graphs, never as nulls or zeros. Rerun `report/build.sh` when the runs finish.".into(),
    ]);

    // ---------------- 2 requirement table
    d.h(1, "2. Requirement to evidence");
    let p2_done = p2.iter().filter(|c| c.state == P2State::Done).count();
    let p2_run_scope = p2.iter().filter(|c| c.state != P2State::Deferred).count();
    let p3_done = p3.card_state.iter().filter(|c| c.3.starts_with("completed")).count();
    d.table_w(
        &["Requirement", "Status", "Evidence"],
        vec![
            vec!["Report states: characterization, not an improvement over the IC baseline".into(), "verified complete".into(), "section 1, DESIGN.md, three PROTOCOL.md".into()],
            vec!["No leaderboard or benchmarks row; no rho comparison".into(), "verified complete".into(), "section 1; P2 PROTOCOL section 9, P3 PROTOCOL section 1".into()],
            vec!["L0 isolation stated (counted exact, clock descriptive)".into(), "verified complete".into(), "section 1 and 3; P1 A1 section 14, P2 A1 and 10.1, P3 A1".into()],
            vec!["Deferred cells stated (P1 tier 2, P2 36/40-bit, P3a-Y)".into(), "verified complete".into(), "sections 1 and 8".into()],
            vec!["P1 ECCp-131 tier 1, arm-ratio strip plot".into(), if p1.ok { "complete at tier-1 scope (tier 2 deferred)".into() } else { "blocked: analysis file missing".into() }, "section 5, figure 2, `p1_eccp131/runs/tier1_analysis.json`".into()],
            vec!["P2 per-cell yield per curve, A/A band, planted control".into(), format!("partial: {p2_done} of {p2_run_scope} run-scope cells finished"), "section 6, figure 3, `p2_prime_analogues/runs/<cell>/analysis.json`".into()],
            vec!["P3 Dreg per curve and presentation family, E0 highlighted, standard V versus b-adapted".into(), format!("partial: {p3_done} of {} cards completed (receipt); more output present", p3.card_state.len()), "section 7, figures 4 and 5, `p3_ecc2k130/full/`".into()],
            vec!["Class-structure diagram (ECC2K-130 volcano levels beside ECCp-131 crater)".into(), "verified complete (structure from DESIGN.md)".into(), "section 3, figure 1".into()],
            vec!["Generator, REPORT.md, REPORT.typ, REPORT.pdf, SVGs".into(), "see section 9".into(), "`report/`".into()],
        ],
        vec!["2fr", "1.4fr", "2fr"],
        "",
    );

    // ---------------- 3 class structure and conditions
    d.h(1, "3. Class structure and measurement conditions");
    d.fig(&fig_cls, "Isogeny-class structure the instruments are applied to. Left: ECC2K-130 as a 263-isogeny volcano of depth 1 (E0 on the crater, 262 curves on the floor in two Frobenius orbits). Right: ECCp-131, a single crater of about 2^63.6 curves with 27 Elkies neighbours reachable at l <= 149, and the 27 random non-isogenous controls outside the class. Source: DESIGN.md, p3_ecc2k130/data/ecc2k130_curves.tsv.");
    d.table(
        &["", "ECC2K-130", "ECCp-131"],
        vec![
            vec!["trace".into(), "-22283658519494248867".into(), "29529377007934231835".into()],
            vec!["fundamental discriminant".into(), "-7".into(), "-5328143084860676590809120034441032302419".into()],
            vec!["conductor of Z[pi]".into(), "263 x 146505763881528721".into(), "1 (crater only, h about 2^63.6)".into()],
            vec!["computable neighbours".into(), "262 at level 263".into(), "Elkies l <= 150: 5,7,13,17,29,37,61,73,83,89,107,131,149 (2 each), 67 (1, ramified): 27".into()],
            vec!["scaled analogue measured".into(), "n = 43 (level 257/258 curves, 6 orbits), full width n = 131 for Dreg".into(), "P2: primes of 24 to 32 bits (36, 40 deferred), conductor 1, full class enumerated".into()],
        ],
        "Class facts (computed 2026-11-02, PARI 2.17 via cypari2; agrees with ecdb curve 22; copied from DESIGN.md).",
    );
    // conditions table from recorded headers
    let mut cond: Vec<Vec<String>> = Vec::new();
    if p1.ok {
        let h = &p1.header;
        cond.push(vec![
            "P1 tier 1".into(),
            format!("{} workers, nproc {}, {}", h.get("workers").i().unwrap_or(-1), h.get("nproc").i().unwrap_or(-1), h.get("cpu").s().unwrap_or("?").replace('\t', " ")),
            format!("rows {}; binary sha256 {}; {}", p1.rows, h.get("binary_sha256").s().unwrap_or("?").chars().take(16).collect::<String>(), p1.log_done),
        ]);
    }
    for c in p2.iter().filter(|c| c.state == P2State::Done) {
        let iso = c.an.get("isolation_levels_of_rows");
        let isos: Vec<String> = iso.obj().iter().map(|(k, v)| format!("{k}: {}", v.i().unwrap_or(0))).collect();
        cond.push(vec![
            format!("P2 {}", c.id),
            format!("ecbench isolation level recorded on rows: {}", isos.join(", ")),
            format!("audit: {}, {} records, {} replays; in-session A/A: {}", if c.audit.get("ok").b() == Some(true) { "OK" } else { "NOT ok" }, c.audit.get("records").i().unwrap_or(0), c.audit.get("replays").arr().len(), { let np = c.an.path("aa_in_session_identical_relation_counts.pairs").i().unwrap_or(0); if np == 0 { "not run (dropped at 32 bits, PROTOCOL 4; round-parity A/A used)".to_string() } else { format!("counts identical {}/{}", c.an.path("aa_in_session_identical_relation_counts.identical").i().unwrap_or(0), np) } }),
        ]);
    }
    d.table_w(&["Part / cell", "Host and isolation as recorded", "Checks as recorded"], cond, vec!["1fr", "2fr", "2.4fr"], "Measurement conditions read from the recorded files. The protocols accept L0 (loaded host); the ecbench level on P2 rows is what ecbench graded and is reported as recorded. In every case the clock is descriptive only.");

    // ---------------- 5 P1
    d.h(1, "4. Method in one paragraph per part");
    d.bullets(vec![
        "**P1 (ECCp-131, full width, tier 1).** XL/Macaulay engine (`xl-incremental-sparse-echelon`) on the fixed bounded decomposition system, configs `m2t3` and `m3t2`, kinds null, planted and ladder. 58 arms: E0, an A/A repeat, an A/A permuted variable order-order arm, 1 isomorphic model, 27 Elkies neighbours, 27 random non-isogenous curves; 32 null targets, 8 planted targets, 3 reps, seed 20261102. Masked analysis frozen first, key opened after (A1 section 14).".into(),
        "**P2 (prime analogues).** `ecbench` prime-abscissa mitm pipeline on every curve of an enumerated class (D = -84171, h = 128 for class A; D = -414651, h = 300 for class B), m = 2 and 3. Per curve pooled relation yield per attempt; ICC curve share of variance, max/min and max |z| against a 2000-permutation null; round-parity A/A halves; isomorphic-model control; planted +20% positive control (power must be at least 0.9 for a null to be informative).".into(),
        "**P3 (ECC2K-130).** `p3h measure`: Weil-descended decomposition system over F_2, recorded `dreg` (resolving degree), exact `solutions`, rows and cols, for random V, standard V and b-adapted presentations, at n = 43 (all 258 level curves plus E0) and n = 131 (E0 plus 262 level-263 curves), k = 3 to 6.".into(),
    ]);

    d.h(1, "5. P1: ECCp-131 at full width (tier 1)");
    if !p1.ok {
        d.p("P1 analysis file not found: pending.");
    } else {
        let key_note = format!("The analysis was produced with arm labels masked (`arm_labels_masked` = {}); the key (sha256 {}) maps the masked labels to arms and is applied here only to name the points.", p1.masked, p1.key_sha);
        d.p(format!("`tier1_analysis.json` covers {} rows in {} cells (config x kind), CI level {}, {} bootstrap resamples. {}", p1.rows, p1.cells.len(), p1.ci_level, p1.boot, key_note));
        // headline table
        let mut rows = Vec::new();
        for c in &p1.cells {
            let nb = c.arms.iter().filter(|a| a.role == "neighbour").count();
            let nr = c.arms.iter().filter(|a| a.role == "random").count();
            let mut r = vec![format!("{} {}", c.config, c.kind), format!("{} ({} nb, {} rnd)", c.n_arms - 1, nb, nr)];
            for mk in ["D_solve", "final_rank", "field_mults", "cpu_s"] {
                let (lo, hi) = p1_range(c, mk, true);
                r.push(format!("{} to {}", fx(lo, 3), fx(hi, 3)));
            }
            let perm = c.arms.iter().find(|a| a.role == "AA_perm");
            r.push(perm.map(|a| fx(a.m.get("field_mults").map(|x| x.0).unwrap_or(f64::NAN), 3)).unwrap_or("n/a".into()));
            let ident: Vec<f64> = c.arms.iter().map(|a| a.ident).collect();
            r.push(fx(minmax(&ident).0, 2));
            rows.push(r);
        }
        d.table(&["cell", "arms vs E0", "D_solve ratio", "final_rank ratio", "field_mults ratio", "cpu_s ratio (descriptive)", "AA_perm field_mults ratio", "min identical-profile fraction"], rows, "Measured (derived from the recorded per-arm geometric-mean ratios vs E0): range over arms other than AA_perm, per cell. AA_perm is the A/A arm with permuted variable order (re-encoding floor) and is listed separately.");
        d.fig(fig1.as_deref().unwrap_or(""), "P1 arm-ratio strip plot. One point per arm and metric: ratio to E0 (geometric mean over paired target and rep instances), whiskers the recorded confidence interval. Counted metrics (D_solve, first fall degree, final rank, field mults) are exact; cpu_s is descriptive (L0, loaded host). Run: tier1 card CARD-isogeny-ic-yield-p1-tier1-20261102-1, analysis tier1_analysis.json.");
        // not formed
        let mut nf_rows = Vec::new();
        for ((cfg, role), n) in &p1.not_formed {
            nf_rows.push(vec![cfg.clone(), role.clone(), n.to_string()]);
        }
        let nf_total: i64 = p1.not_formed.values().sum();
        let mut st_rows = Vec::new();
        for ((cfg, kind, st), n) in &p1.status {
            st_rows.push(vec![cfg.clone(), kind.clone(), st.clone(), n.to_string()]);
        }
        d.table(&["config", "kind", "row status", "rows"], st_rows, "Recorded row status of tier1_rows.jsonl.gz (counted from the rows file).");
        d.p(format!("**Planted rows not formed: {nf_total}.** Recorded reason: {}. Rows by config and role:", if p1.not_formed_reason.is_empty() { "none recorded".to_string() } else { p1.not_formed_reason.iter().cloned().collect::<Vec<_>>().join("; ") }));
        d.table(&["config", "role", "rows not formed"], nf_rows, "These rows are missing observations (the factor base of that curve held fewer abscissae than m), not values; they are excluded from every ratio above.");
        // ICC
        let mut icc_rows = Vec::new();
        for c in &p1.cells {
            for (g, nc, nt, ms) in &c.icc {
                let eq: Vec<String> = ms.iter().filter(|m| m.1).map(|m| m.0.clone()).collect();
                let ne: Vec<String> = ms.iter().filter(|m| !m.1).map(|m| format!("{} (share {})", m.0, m.2.map(|x| fx(x, 3)).unwrap_or("n/a".into()))).collect();
                icc_rows.push(vec![format!("{} {}", c.config, c.kind), g.clone(), format!("{nc} curves, {nt} targets"), if eq.is_empty() { "none".into() } else { eq.join(", ") }, if ne.is_empty() { "none".into() } else { ne.join(", ") }]);
            }
        }
        d.table_w(&["cell", "group", "size", "metrics with all values equal", "metrics that vary (curve variance share)"], icc_rows, vec!["0.8fr", "1.3fr", "1fr", "2fr", "2.2fr"], "Recorded `icc` block of the analysis: for the A/A, class (E0 plus neighbours) and random-curve groups, which counted metrics take one value on every curve and, where they vary, the curve share of variance.");
        d.h(2, "5.1 What it shows now");
        let mut deds = Vec::new();
        let mut max_dev_counted = 0.0f64;
        let mut max_dev_cpu = 0.0f64;
        let mut perm_vals = Vec::new();
        let mut perm_cpu = Vec::new();
        let mut perm_dev = 0.0f64;
        for c in &p1.cells {
            for mk in ["D_solve", "first_fall_degree_obs", "final_rank", "field_mults"] {
                let (lo, hi) = p1_range(c, mk, true);
                max_dev_counted = max_dev_counted.max((lo - 1.0).abs()).max((hi - 1.0).abs());
            }
            let (lo, hi) = p1_range(c, "cpu_s", true);
            max_dev_cpu = max_dev_cpu.max((lo - 1.0).abs()).max((hi - 1.0).abs());
            for a in c.arms.iter().filter(|a| a.role == "AA_perm") {
                for mk in ["D_solve", "final_rank"] {
                    if let Some(x) = a.m.get(mk) {
                        perm_dev = perm_dev.max((x.0 - 1.0).abs());
                    }
                }
                if let Some(x) = a.m.get("field_mults") {
                    perm_vals.push(x.0);
                }
                if let Some(x) = a.m.get("cpu_s") {
                    perm_cpu.push(x.0);
                }
            }
        }
        let (pl, ph_) = minmax(&perm_vals);
        let (cl, ch) = minmax(&perm_cpu);
        deds.push(format!("**Measured.** In every cell, every arm other than AA_perm has ratio to E0 within {} of 1 on D_solve, first fall degree, final rank and field mults (the largest deviation over all cells and these four metrics is {}; 0 means the recorded ratio is exactly 1). The arms present are the Elkies neighbours, the isomorphic model and the random non-isogenous curves (27 each, minus the planted instances that were not formed), for configs m2t3 and m3t2 and kinds null, planted and ladder.", fsig(max_dev_counted), fsig(max_dev_counted)));
        deds.push(format!("**Measured.** The arm that moves the operation count is AA_perm (field_mults ratio {} to {} over the cells in which it appears); its D_solve and final rank ratios deviate from 1 by at most {}. By construction (PROTOCOL section 3) this arm is the E0 instance with a seeded permutation of the variable order, the re-encoding noise floor: it sets how far the operation count moves under a re-encoding of the same curve, and it is not a curve effect.", fx(pl, 3), fx(ph_, 3), fsig(perm_dev)));
        deds.push(format!("**Measured, descriptive.** `cpu_s` ratios of arms other than AA_perm lie within {} of 1 (cells above); AA_perm cpu ratio {} to {}. The clock was taken at L0 on a loaded host and decides nothing.", fx(max_dev_cpu, 3), fx(cl, 3), fx(ch, 3)));
        deds.push(format!("**Measured.** {nf_total} planted rows were not formed (see table); they are missing, not zero."));
        d.bullets(deds);
        d.p("**Reading (main session, 2026-11-02; interpretation, labelled as such).** At 131 bits the instrument's counted profile (XL closure degree, rank, field multiplications, per-degree profiles) is independent of the curve: it is the same on the 27 neighbours, on the isomorphic model and on the 27 random non-isogenous curves. Because random non-isogenous curves match as well, the null population is uninformative about isogeny specifically: this instrument at this system size does not distinguish any curve from any other, so it cannot see an isogeny-class effect even if one existed in a quantity it does not count. `D_solve` is the XL/Macaulay closure degree of this study's engine; on the 147 comparable mirror systems of `p1_eccp131/CROSSCHECK.md` it is strictly above F4's top step degree, so it is not F4's solving degree and not a degree of regularity, and no statement here calls it one. Full-width yield is predicted, not observed (exact-curve screens found zero relations).");
    }

    // ---------------- 6 P2
    d.h(1, "6. P2: prime analogues of ECCp-131's class");
    let mut rows = Vec::new();
    for bits in [24u32, 28, 32, 36, 40] {
        for cl in ['A', 'B'] {
            let mut r = vec![format!("c{bits}{cl}")];
            let mut fact = "-".to_string();
            for m in [2u32, 3] {
                let c = p2.iter().find(|c| c.bits == bits && c.class == cl && c.m == m).unwrap();
                r.push(match c.state {
                    P2State::Done => { fact = format!("p 2^{}, D {}, h {}", fx(c.an.path("class_facts.log2_p").f().unwrap_or(f64::NAN), 2), c.an.path("class_facts.D").i().unwrap_or(0), c.an.path("class_facts.h").i().unwrap_or(0)); format!("finished ({} curves)", c.an.path("metrics.yield.curves").i().unwrap_or(0)) }
                    P2State::Running => "pending (running)".to_string(),
                    P2State::Pending => "pending".to_string(),
                    P2State::Deferred => "not run (deferred)".to_string(),
                });
            }
            r.push(fact);
            rows.push(r);
        }
    }
    d.table(&["class", "m = 2", "m = 3", "class facts (from a finished cell)"], rows, "Cell inventory: 12 run-scope cells (24, 28, 32 bits, classes A and B, m = 2, 3) and 8 deferred cells (36 and 40 bits). Pending cells have no values; deferred cells were not run.");
    d.fig(&fig2, "P2 yield per curve against the class mean with the A/A band and the planted-control effect, per finished cell (hatched panels are pending). Source: runs/<cell>/analysis.json (p2tool analyze, 2000 permutations, 2000 bootstrap resamples, seed 20261102).");
    // stat table
    let mut srows = Vec::new();
    for c in p2.iter().filter(|c| c.state == P2State::Done) {
        let y = |k: &str| c.an.path(&format!("metrics.yield.{k}")).f().unwrap_or(f64::NAN);
        let g = |k: &str| c.an.path(&format!("metrics.rel_gae_per_attempt.{k}")).f().unwrap_or(f64::NAN);
        let pw = y("positive_control.detect_icc_gt_null_q95").min(y("positive_control.detect_max_abs_z_gt_null_q95")).min(y("positive_control.detect_max_over_min_gt_null_q95"));
        let alpha = 0.0025;
        let mut flags = Vec::new();
        if y("icc_perm_p") < alpha {
            flags.push("ICC p < 0.0025".to_string());
        }
        if y("max_abs_z_perm_p") < alpha {
            flags.push("max|z| p < 0.0025".to_string());
        }
        if pw < 0.9 {
            flags.push("power < 0.9".to_string());
        }
        if flags.is_empty() {
            flags.push("none".into());
        }
        srows.push(vec![
            c.id.clone(),
            fsig(y("grand_mean")),
            format!("{} / {}", fx(y("icc_raw"), 4), fx(y("icc_bootstrap_upper95"), 3)),
            fx(y("icc_perm_p"), 4),
            format!("{} ({}) p {}", fx(y("max_abs_z"), 2), fx(y("max_abs_z_perm_null_q95"), 2), fx(y("max_abs_z_perm_p"), 4)),
            format!("{} ({})", fx(y("max_over_min"), 3), fx(y("max_over_min_perm_null_q95"), 3)),
            format!("{} / {}", fx(y("aa_round_halves.sd_log_ratio"), 4), fx(y("isomorphic_model_control.rms_log_ratio"), 4)),
            format!("{} / {}", fx(y("positive_control.planted_effect_pc_over_main_median"), 3), fx(pw, 2)),
            fx(g("icc_perm_p"), 3),
            flags.join(", "),
        ]);
    }
    d.table_w(
        &["cell", "yield mean / attempt", "ICC raw / boot upper95", "ICC perm p", "max abs z (null q95) p", "max/min (null q95)", "A/A sd / iso RMS (log ratio)", "planted median pc/main / min power", "rel_gae ICC p", "PROTOCOL 6 triggers (mechanical)"],
        srows,
        vec!["0.8fr", "0.8fr", "1fr", "0.7fr", "1.2fr", "1fr", "1fr", "1fr", "0.6fr", "1.3fr"],
        "Recorded statistics of the frozen analysis for the finished cells. The last column only lists which numeric triggers of PROTOCOL section 6 are met (Bonferroni alpha 0.0025 over 20 tests; positive-control power below 0.9 means a null is not informative at 20%); the verdict per cell is assigned by the main session, and a trigger starts a follow-up protocol, it is not a claim.",
    );
    d.p("**Observation to carry into the reading.** Cells whose last column lists a trigger are the ones the main session must read against the isomorphic-model control (presentation effect of the factor-base rule) before calling anything a curve effect. Cells that list `power < 0.9` are \"not informative at 20%\", never reported as nulls (PROTOCOL section 4).");

    // ---------------- 7 P3
    d.h(1, "7. P3: ECC2K-130 analogue and full width");
    let mut crow = Vec::new();
    for (tag, desc, stems, state) in &p3.card_state {
        let files: Vec<String> = stems.iter().map(|(s, c)| format!("{s}: {}", c.map(|x| x.to_string() + " rows").unwrap_or("absent".into()))).collect();
        crow.push(vec![tag.clone(), desc.clone(), files.join("; "), state.clone()]);
    }
    d.table_w(&["card", "what", "output files (rows present)", "state"], crow, vec!["0.9fr", "2fr", "2.2fr", "1.3fr"], "P3 card inventory. A file without a receipt may still be growing; its rows are included in the figures as present. P3a-Y (n=43, k=13 yield line, about 19.4 CPU-hours) is deferred, not run.");
    // checks
    let mut chk = Vec::new();
    {
        let mut key_main: BTreeMap<(String, i64, i64, i64), (i64, i64, bool)> = BTreeMap::new();
        for r in p3.rows.iter().filter(|r| r.arm == "main") {
            key_main.insert((r.stem.clone(), r.k, r.attempt, r.curve), (r.dreg, r.solutions, r.refuted));
        }
        let mut stats: BTreeMap<(i64, i64), [usize; 7]> = BTreeMap::new(); // iso tot, iso same, aa tot, aa same, planted ok1, ok0, na
        for r in &p3.rows {
            let e = stats.entry((r.n, r.k)).or_insert([0; 7]);
            if r.arm == "iso" || r.arm == "aa" {
                if let Some(m) = key_main.get(&(r.stem.clone(), r.k, r.attempt, r.curve)) {
                    let same = *m == (r.dreg, r.solutions, r.refuted);
                    if r.arm == "iso" {
                        e[0] += 1;
                        e[1] += same as usize;
                    } else {
                        e[2] += 1;
                        e[3] += same as usize;
                    }
                }
            }
            if r.arm == "planted" {
                match r.planted_ok {
                    1 => e[4] += 1,
                    0 => e[5] += 1,
                    _ => e[6] += 1,
                }
            }
        }
        for ((n, k), e) in &stats {
            if e[0] + e[2] + e[4] + e[5] + e[6] == 0 {
                continue;
            }
            chk.push(vec![format!("n={n}, k={k}"), format!("{}/{}", e[1], e[0]), format!("{}/{}", e[3], e[2]), format!("{} / {} / {}", e[4], e[5], e[6])]);
        }
    }
    d.table(&["cell", "isomorphic-model rows identical to main (dreg, solutions, refuted)", "A/A rows identical to main", "planted rows: planted_ok = 1 / 0 / not applicable (-1)"], chk, "PROTOCOL 6.1 instrument checks, recounted from the recorded rows (same (k, attempt, curve) key, same shard file). A cell with an identity below 100% is void by the protocol. Planted rows whose `outcome` is `atleast` are bounds and are not Dreg values.");
    d.fig(&fig3a, "P3 at full width (n = 131): Dreg per curve for random V, standard V and b-adapted presentations, E0 ringed (curve 0), orbits A and B shaded. Pending series are named under each panel. Source: p3_dreg_n131.data.tsv from the recorded rows.");
    d.fig(&fig3b, "P3a analogue (n = 43, E0 plus 258 level curves, 6 orbits): Dreg per curve. The control and family arms were measured on a subset of curves (`--select`), the main arm on all curves. Hollow triangles are bounds (Dreg > dmax). Source: p3_dreg_n43.data.tsv.");
    // gap table, attempt 0
    let mut grows = Vec::new();
    for &n in &[131i64, 43] {
        let ks: Vec<i64> = if n == 131 { vec![3, 4, 5, 6] } else { vec![3, 4] };
        for &k in &ks {
            let fam_arms = ["stdv", "badapt", "bsparse"];
            let get = |arms: &[&str]| -> BTreeMap<i64, (f64, String)> {
                let mut m: BTreeMap<i64, (i64, String, bool)> = BTreeMap::new();
                for r in p3.rows.iter().filter(|r| r.n == n && r.k == k && r.attempt == 0 && arms.contains(&r.arm.as_str()) && r.curve < 100000) {
                    let e = m.entry(r.curve).or_insert((i64::MAX, r.orbit.clone(), false));
                    if r.dreg < e.0 {
                        e.0 = r.dreg;
                        e.2 = r.outcome != "resolved";
                    }
                }
                m.into_iter().map(|(c, (d, o, b))| (c, (d as f64 + if b { 1000.0 } else { 0.0 }, o))).collect()
            };
            let fam = get(&fam_arms);
            let std_only = get(&["stdv"]);
            let present: Vec<&str> = fam_arms.iter().cloned().filter(|a| p3.rows.iter().any(|r| r.n == n && r.k == k && r.arm == *a)).collect();
            // bsparse runs at n=131 only for k=5,6 by design (PROTOCOL.md section 5); it is not pending at k=3,4.
            let missing: Vec<&str> = fam_arms.iter().cloned().filter(|a| !present.contains(a) && !(n == 131 && k < 5 && *a == "bsparse")).collect();
            let e0f = fam.get(&0).map(|x| x.0);
            let e0s = std_only.get(&0).map(|x| x.0);
            let orbits: BTreeSet<String> = p3.rows.iter().filter(|r| r.n == n && r.curve < 100000 && r.orbit != "crater").map(|r| r.orbit.clone()).collect();
            if orbits.is_empty() {
                grows.push(vec![format!("n={n}, k={k}"), "-".into(), "pending".into(), "-".into(), "-".into(), present.join(", "), if missing.is_empty() { "all three family arms have rows (curve coverage in figures 4 and 5)".to_string() } else { format!("incomplete, arms pending: {}", missing.join(", ")) }]);
                continue;
            }
            for o in orbits {
                let vals = |m: &BTreeMap<i64, (f64, String)>| -> Vec<f64> { m.values().filter(|x| x.1 == o && x.0 < 1000.0).map(|x| x.0).collect() };
                let fv = vals(&fam);
                let sv = vals(&std_only);
                let fmt = |v: &Vec<f64>, e0: Option<f64>| -> String {
                    if v.is_empty() {
                        return "pending".into();
                    }
                    let (lo, hi) = minmax(v);
                    match e0 {
                        Some(e) if e < 1000.0 => format!("{} (range {} to {}; {} curve(s)); E0 {}; difference {}", fx(mean(v), 3), fx(lo, 0), fx(hi, 0), v.len(), fx(e, 0), fx(mean(v) - e, 3)),
                        _ => format!("{} (range {} to {}; {} curve(s)); E0 pending", fx(mean(v), 3), fx(lo, 0), fx(hi, 0), v.len()),
                    }
                };
                grows.push(vec![format!("n={n}, k={k}"), o.clone(), fmt(&sv, e0s), fmt(&fv, e0f), present.join(", "), if missing.is_empty() { "all three family arms have rows (curve coverage in figures 4 and 5)".to_string() } else { format!("incomplete, arms pending: {}", missing.join(", ")) }, String::new()]);
                let l = grows.len() - 1;
                grows[l].pop();
            }
        }
    }
    d.table_w(&["cell", "orbit", "standard V only: floor-curve mean of min Dreg against E0", "family minimum (arms present): floor-curve mean against E0", "family arms with rows", "family status"], grows.into_iter().map(|mut r| { r.truncate(6); r }).collect(), vec!["0.7fr", "0.5fr", "2.2fr", "2.2fr", "1fr", "1.2fr"], "Descriptive recorded-value summary, attempt 0 only; n = 43 family cells are k = 3 and 4 only (PROTOCOL A1) (the one attempt common to every family arm). Per curve: the minimum recorded Dreg over the presentations of the arms listed; per orbit: mean, range and count over curves, against E0 under the same procedure. This is not the frozen bootstrap analysis (`p3h analyze`, archived under report/data/p3_analysis/) and not PROTOCOL 6.3 outcome N or G: that reading needs the b-adapted and b-sparse arms to be complete, and cells marked incomplete must not be read as a family-minimum gap.");
    d.p("The preregistered reading is C2: after minimising over the declared presentation family F on both sides, E0 and the floor curves coincide, with the single-presentation (standard V) gap present as the positive control (standard V, n = 131, k = 6: E0 Dreg 6 against floor Dreg 7). Where the table shows the single-presentation gap and an incomplete family, only the positive control is visible so far. A surviving gap would be read only as a statement about b in F_2 (E0 alone has b = 1), not about the isogeny class, and is not a speedup (no end-to-end S, no rho gap).");
    if study.join("p3_ecc2k130/RESULTS.md").exists() {
        d.p("Main-session reading of the frozen pooled analyses (masked first, unmasked after the freeze; p3_ecc2k130/RESULTS.md): outcome N. At n = 131, k = 5 and 6 the presentation-family minimum is Dreg 6 for E0 and for all 262 level-263 curves on both attempts; the b-adapted presentation alone gives 6 on every curve, and random-b controls match. Random V: Dreg and matrix shape identical to E0 on every class curve at n = 131 (k = 3, 4); at n = 43 Dreg is identical and one target of 20 shows b-dependent column counts with E0 in the modal shape.");
    }

    // ---------------- 8 deferred and pending
    d.h(1, "8. Deferred and pending");
    let pend2: Vec<String> = p2.iter().filter(|c| c.state == P2State::Pending || c.state == P2State::Running).map(|c| c.id.clone()).collect();
    let def2: Vec<String> = p2.iter().filter(|c| c.state == P2State::Deferred).map(|c| c.id.clone()).collect();
    d.bullets(vec![
        "**Not run (deferred), P1 tier 2:** config `m2t4`, 39 arms, 8 plus 2 targets (about 6.3 CPU-hours). In the preregistered design, no data taken under protocol version 1 amendment A1. Needs a further amendment before running.".into(),
        format!("**Not run (deferred), P2 at 36 and 40 bits:** {} (8 cells). In the population and the preregistration; no data taken under amendment A1.", def2.join(", ")),
        "**Not run (deferred), P3a-Y:** n = 43, k = 13 yield line (about 19.4 CPU-hours); claim C3 (yield against the Poisson prediction) is not run, and the `solutions` column of the cells that did run is descriptive.".into(),
        format!("**Pending at generation time, P2:** {}.", if pend2.is_empty() { "none".to_string() } else { pend2.join(", ") }),
        format!("**Pending or partial at generation time, P3:** {}.", { let v: Vec<String> = p3.card_state.iter().filter(|c| !c.3.starts_with("completed")).map(|c| format!("{} ({})", c.0, c.3.split(' ').next().unwrap_or(""))).collect(); if v.is_empty() { "none".to_string() } else { v.join(", ") } }),
        "Statements about pending cells are not made. In particular the C2 outcome (N or G) is open until the b-adapted and b-sparse arms and the remaining standard-V shards finish.".into(),
    ]);

    // ---------------- 9 reproduce
    d.h(1, "9. Reproduce, inputs and files");
    d.p("Rerun from the repository root: `research/isogeny_class_ic_yield_20261102/report/build.sh`. It compiles `report/gen/gen_report.rs` (Rust, std only, `rustc -O`), pools the P3 shards and archives the masked then unmasked `p3h analyze` outputs under `report/data/p3_analysis/` (needs the built `target/release/p3h`; skipped with a note if absent), runs the generator and renders `REPORT.pdf` from `REPORT.typ` with Typst. Typst is document rendering only; if it is not installed the script installs the `typst` wheel in a scratch virtualenv outside the repository, as the research-visuals skill prescribes. No Python computes any value in this report.");
    d.bullets(vec![
        "`report/REPORT.md`, `report/REPORT.typ`, `report/REPORT.pdf`: this report in three forms (generated, do not edit by hand).".into(),
        "`report/figures/*.svg` with `*.data.tsv` beside each: class_structure, p1_arm_ratio_strip, p2_yield_per_curve, p3_dreg_n131, p3_dreg_n43.".into(),
        "`report/data/inputs.sha256.tsv`: sha256 of every input file read in this build.".into(),
        "`report/data/p3_analysis/`: `p3h analyze` text output per pooled P3 file family (masked first, then unmasked).".into(),
    ]);
    let mut irows = Vec::new();
    for (p, h) in &inp.files {
        irows.push(vec![format!("`{p}`"), format!("`{}`", &h[..h.len().min(16)])]);
    }
    d.table_w(&["input file (relative to the study directory)", "sha256 (first 16 hex; full in inputs.sha256.tsv)"], irows, vec!["4fr", "1.5fr"], "Inputs read by this build.");

    // ---------------- write
    let md = render_md(&d);
    let typ = render_typ(&d);
    std::fs::write(out.join("REPORT.md"), md).unwrap();
    std::fs::write(out.join("REPORT.typ"), typ).unwrap();
    std::fs::create_dir_all(out.join("data")).unwrap();
    let mut t = String::from("path\tsha256\n");
    for (p, h) in &inp.files {
        let _ = writeln!(t, "{p}\t{h}");
    }
    std::fs::write(out.join("data/inputs.sha256.tsv"), t).unwrap();
    eprintln!("wrote REPORT.md, REPORT.typ, figures; P1 ok={}, P2 done {}/{}, P3 rows {}", p1.ok, p2_done, p2_run_scope, p3.rows.len());
}
