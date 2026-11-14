//! `l2filters analyze RAWDIR OUTDIR`: tables, results.json and SVG from raw/*.json (std only).
use std::collections::BTreeMap;
use std::fmt::Write as _;

#[derive(Debug, Clone)]
pub enum J {
    Null,
    B(bool),
    N(f64),
    S(String),
    A(Vec<J>),
    O(BTreeMap<String, J>),
}
impl J {
    pub fn get(&self, k: &str) -> &J {
        match self {
            J::O(m) => m.get(k).unwrap_or(&J::Null),
            _ => &J::Null,
        }
    }
    pub fn f(&self) -> f64 {
        match self {
            J::N(x) => *x,
            _ => f64::NAN,
        }
    }
    pub fn b(&self) -> bool {
        matches!(self, J::B(true))
    }
    pub fn arr(&self) -> &[J] {
        match self {
            J::A(v) => v,
            _ => &[],
        }
    }
}

pub fn parse(s: &str) -> J {
    let b = s.as_bytes();
    let mut i = 0;
    let v = val(b, &mut i);
    v
}
fn ws(b: &[u8], i: &mut usize) {
    while *i < b.len() && (b[*i] as char).is_whitespace() {
        *i += 1
    }
}
fn val(b: &[u8], i: &mut usize) -> J {
    ws(b, i);
    match b[*i] {
        b'{' => {
            *i += 1;
            let mut m = BTreeMap::new();
            loop {
                ws(b, i);
                if b[*i] == b'}' {
                    *i += 1;
                    break;
                }
                let k = match val(b, i) {
                    J::S(s) => s,
                    _ => panic!("key"),
                };
                ws(b, i);
                assert_eq!(b[*i], b':');
                *i += 1;
                let v = val(b, i);
                m.insert(k, v);
                ws(b, i);
                if b[*i] == b',' {
                    *i += 1
                }
            }
            J::O(m)
        }
        b'[' => {
            *i += 1;
            let mut v = vec![];
            loop {
                ws(b, i);
                if b[*i] == b']' {
                    *i += 1;
                    break;
                }
                v.push(val(b, i));
                ws(b, i);
                if b[*i] == b',' {
                    *i += 1
                }
            }
            J::A(v)
        }
        b'"' => {
            *i += 1;
            let st = *i;
            while b[*i] != b'"' {
                *i += 1
            }
            let s = String::from_utf8(b[st..*i].to_vec()).unwrap();
            *i += 1;
            J::S(s)
        }
        b't' => {
            *i += 4;
            J::B(true)
        }
        b'f' => {
            *i += 5;
            J::B(false)
        }
        b'n' => {
            *i += 4;
            J::Null
        }
        _ => {
            let st = *i;
            while *i < b.len() && (b"+-.eE".contains(&b[*i]) || b[*i].is_ascii_digit()) {
                *i += 1
            }
            J::N(std::str::from_utf8(&b[st..*i]).unwrap().parse().unwrap())
        }
    }
}

fn wilson(k: f64, n: f64) -> (f64, f64) {
    if n == 0.0 {
        return (f64::NAN, f64::NAN);
    }
    let z = 1.96;
    let p = k / n;
    let den = 1.0 + z * z / n;
    let c = (p + z * z / (2.0 * n)) / den;
    let h = z * ((p * (1.0 - p) / n + z * z / (4.0 * n * n)).sqrt()) / den;
    ((c - h).max(0.0), (c + h).min(1.0))
}

fn binom(a: f64, k: u32) -> f64 {
    let mut r = 1.0;
    for i in 0..k {
        r *= (a - i as f64) / (i as f64 + 1.0)
    }
    r
}

pub fn run(raw: &str, outdir: &str) {
    let mut md = String::new();
    let mut js = String::from("{\"schema\":\"l2-refutation-filters/results/v1\",\"claims\":\"[pcc:unverified]\",\"cells\":[");
    // ---------- cells ----------
    writeln!(md, "### Table A: workload and degree<=k target filters (exhaustive, targets = x(<G>))\n").unwrap();
    writeln!(md, "| n | a | m | d | base pts | U | D cap U | UNSAT frac | Poisson model | dim k=1/2/3 (null) | rejected k=1/2/3 (null) | false neg |").unwrap();
    writeln!(md, "|--:|--:|--:|--:|--:|--:|--:|--:|--:|--|--|--:|").unwrap();
    let mut first = true;
    let mut pc2s = vec![];
    let mut cellpts = vec![]; // (n, m, d, unsat)
    for (n, a) in [(11, 0), (11, 1), (13, 0), (19, 0)] {
        let p = format!("{}/cells_n{}_a{}.json", raw, n, a);
        let j = parse(&std::fs::read_to_string(&p).expect(&p));
        let order = j.get("order").f();
        let u = j.get("U_size").f();
        for c in j.get("cells").arr() {
            let m = c.get("m").f() as u32;
            let d = c.get("d").f();
            let fb = c.get("base_points").f();
            let du = c.get("D_in_U").f();
            let unsat = c.get("unsat_fraction").f();
            let lam = binom(fb + m as f64 - 1.0, m) / order;
            let model = if m == 1 { f64::NAN } else { (-lam).exp() };
            let fl = c.get("filters").arr();
            let nl = c.get("null").arr();
            let dims: Vec<String> = fl.iter().zip(nl).map(|(x, y)| format!("{}({})", x.get("dim").f(), y.get("dim").f())).collect();
            let rej: Vec<String> = fl.iter().zip(nl).map(|(x, y)| format!("{}({})", x.get("rejected").f(), y.get("rejected").f())).collect();
            let fnn: f64 = fl.iter().map(|x| x.get("false_negatives").f()).sum();
            let excess: Vec<f64> = fl.iter().zip(nl).map(|(x, y)| x.get("dim").f() - y.get("dim").f()).collect();
            writeln!(md, "| {} | {} | {} | {} | {} | {} | {} | {:.4} | {} | {} | {} | {} |", n, a, m, d, fb, u, du, unsat,
                if m == 1 { "PC1".to_string() } else { format!("{:.3}", model) }, dims.join(" / "), rej.join(" / "), fnn).unwrap();
            if m > 1 && a == 0 {
                cellpts.push((n, m, d, unsat));
            }
            if !first { js.push(',') }
            first = false;
            write!(js, "{{\"n\":{},\"a\":{},\"m\":{},\"d\":{},\"base_points\":{},\"U\":{},\"D_in_U\":{},\"unsat_fraction\":{:.6},\"poisson_unsat\":{},\"dim_excess_over_null\":[{}],\"false_negatives\":{}}}",
                n, a, m, d, fb, u, du, unsat, if m == 1 { "null".into() } else { format!("{:.6}", model) },
                excess.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","), fnn).unwrap();
        }
        for c in j.get("pc2").arr() {
            pc2s.push(format!("| {} | {} | {} | {} | {} | {} | {} | {} |", n, a, c.get("m").f(), c.get("d").f(), c.get("filter_k1").get("dim").f(),
                c.get("filter_k1").get("rejected").f(), c.get("x_with_chi1").f(), c.get("filter_k1").get("false_negatives").f()));
        }
    }
    js.push_str("],\"linearisation\":[");
    writeln!(md, "\n### Table B: positive control PC2 (universe = all of E, base restricted to 2E)\n").unwrap();
    writeln!(md, "| n | a | m | d | dim k=1 | rejected | x with chi=1 | false neg |\n|--:|--:|--:|--:|--:|--:|--:|--:|").unwrap();
    for l in &pc2s {
        writeln!(md, "{}", l).unwrap();
    }
    // ---------- linearisation ----------
    writeln!(md, "\n### Table C: Macaulay linearisation refutation on the descent system (200 uniform targets from x(<G>) per cell, a = 0)\n").unwrap();
    writeln!(md, "| n | m | d | vars | UNSAT in sample | D | refuted / UNSAT [95% Wilson] | false neg | mean word ops / target | oracle cost / target (rootfind, GAE) | net GAE / target at 0 / 0.1 / 1 GAE per word op | break-even GAE per word op |").unwrap();
    writeln!(md, "|--:|--:|--:|--:|--:|--:|--|--:|--:|--:|--|--:|").unwrap();
    let mut first = true;
    let mut linpts = vec![]; // (label, D, frac)
    let mut shares: Vec<String> = vec![];
    for (n, m, d) in [(11, 2, 5), (11, 2, 6), (13, 2, 6), (13, 2, 7), (19, 2, 9), (19, 2, 10), (11, 3, 4), (13, 3, 4), (13, 3, 5), (19, 3, 6), (19, 3, 7)] {
        let p = format!("{}/linz_n{}_m{}_d{}.json", raw, n, m, d);
        let Ok(s) = std::fs::read_to_string(&p) else { continue };
        let j = parse(&s);
        let rows = j.get("rows").arr();
        let fb = j.get("base_points").f();
        let oracle = if m == 2 { 1.0 } else { fb }; // rootfind probes, 1 group add each
        let nv = rows[0].get("nvars").f();
        let unsat = rows.iter().filter(|r| !r.get("sat").b()).count() as f64;
        for dd in [2, 3, 4] {
            let key = format!("D{}", dd);
            if rows[0].get(&key).get("cols").f().is_nan() { continue }
            let refd = rows.iter().filter(|r| r.get(&key).get("refuted").b()).count() as f64;
            let fnn = rows.iter().filter(|r| r.get(&key).get("refuted").b() && r.get("sat").b()).count();
            let wo: f64 = rows.iter().map(|r| r.get(&key).get("word_ops").f()).sum::<f64>() / rows.len() as f64;
            let pi = refd / rows.len() as f64;
            let saved = pi * oracle;
            let nets: Vec<String> = [0.0, 0.1, 1.0].iter().map(|pr| format!("{:+.3e}", saved - pr * wo)).collect();
            let be = saved / wo;
            let (lo, hi) = wilson(refd, unsat);
            let wo_unsat: f64 = rows.iter().filter(|r| !r.get("sat").b()).map(|r| r.get(&key).get("word_ops").f()).sum();
            let wo_all: f64 = rows.iter().map(|r| r.get(&key).get("word_ops").f()).sum();
            shares.push(format!("| {} | {} | {} | {} | {:.3} | {:.3} |", n, m, d, dd, unsat / rows.len() as f64, wo_unsat / wo_all));
            writeln!(md, "| {} | {} | {} | {} | {} | {} | {}/{} = {:.3} [{:.3}, {:.3}] | {} | {:.3e} | {} | {} | {:.2e} |", n, m, d, nv, unsat, dd, refd, unsat, refd / unsat, lo, hi, fnn, wo, oracle, nets.join(" / "), be).unwrap();
            linpts.push((format!("n{} m{} d{}", n, m, d), dd, refd / unsat));
            if !first { js.push(',') }
            first = false;
            write!(js, "{{\"n\":{},\"m\":{},\"d\":{},\"D\":{},\"targets\":{},\"unsat\":{},\"refuted\":{},\"refuted_frac_of_unsat\":{:.6},\"wilson95\":[{:.6},{:.6}],\"false_negatives\":{},\"mean_word_ops\":{:.1},\"rootfind_oracle_gae\":{},\"net_gae_at_price_0_0.1_1\":[{}],\"breakeven_gae_per_word_op\":{:.6e}}}",
                n, m, d, dd, rows.len(), unsat, refd, refd / unsat, lo, hi, fnn, wo, oracle,
                [0.0, 0.1, 1.0].iter().map(|pr| format!("{:.6e}", saved - pr * wo)).collect::<Vec<_>>().join(","), be).unwrap();
        }
    }
    js.push_str("]}\n");
    writeln!(md, "\n### Table D: share of XL/linearisation word ops spent on UNSAT targets (same streams)\n\n| n | m | d | D | UNSAT share of targets | UNSAT share of word ops |\n|--:|--:|--:|--:|--:|--:|").unwrap();
    for l in &shares { writeln!(md, "{}", l).unwrap(); }
    std::fs::write(format!("{}/results.json", outdir), js).unwrap();
    std::fs::write(format!("{}/tables.md", outdir), md).unwrap();
    std::fs::write(format!("{}/figure.svg", outdir), svg(&cellpts, &linpts)).unwrap();
}

fn svg(cells: &[(i32, u32, f64, f64)], lin: &[(String, i32, f64)]) -> String {
    let (w, h) = (900.0, 380.0);
    let mut s = format!("<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' viewBox='0 0 {w} {h}' font-family='sans-serif' font-size='11'>\n<rect width='100%' height='100%' fill='white'/>\n");
    // panel 1: UNSAT fraction vs d - ceil(n/m)
    let (x0, y0, pw, ph) = (60.0, 40.0, 340.0, 280.0);
    panel(&mut s, x0, y0, pw, ph, "UNSAT fraction vs d - ceil(n/m) (exhaustive, a=0)", "d - ceil(n/m)", "UNSAT fraction");
    let cols = ["#1f77b4", "#d62728", "#2ca02c", "#9467bd", "#ff7f0e", "#8c564b"];
    let mut series: BTreeMap<(i32, u32), Vec<(f64, f64)>> = BTreeMap::new();
    for &(n, m, d, u) in cells {
        let dc = ((n as f64) / m as f64).ceil();
        series.entry((n, m)).or_default().push((d - dc, u));
    }
    for (i, ((n, m), pts)) in series.iter().enumerate() {
        let c = cols[i % cols.len()];
        let mut path = String::new();
        for (k, (x, y)) in pts.iter().enumerate() {
            let px = x0 + (x + 1.5) / 3.0 * pw;
            let py = y0 + ph - y * ph;
            write!(path, "{}{:.1},{:.1} ", if k == 0 { "M" } else { "L" }, px, py).unwrap();
            write!(s, "<circle cx='{px:.1}' cy='{py:.1}' r='3' fill='{c}'/>").unwrap();
        }
        let dash = if *m == 3 { " stroke-dasharray='5,3'" } else { "" };
        write!(s, "<path d='{path}' stroke='{c}' fill='none'{dash}/>\n").unwrap();
        write!(s, "<text x='{:.1}' y='{:.1}' fill='{c}'>n={n} m={m}</text>\n", x0 + pw - 80.0, y0 + 14.0 + 13.0 * i as f64).unwrap();
    }
    for (k, lab) in ["-1", "0", "+1"].iter().enumerate() {
        let px = x0 + (k as f64 - 0.5) / 3.0 * pw + pw / 3.0 * 0.5 + pw / 6.0 - pw / 6.0;
        write!(s, "<text x='{:.1}' y='{:.1}' text-anchor='middle'>{lab}</text>", x0 + (k as f64 + 0.5) / 3.0 * pw, y0 + ph + 14.0).unwrap();
        let _ = px;
    }
    // panel 2: refuted fraction of UNSAT vs D
    let (x1, pw2) = (500.0, 340.0);
    panel(&mut s, x1, y0, pw2, ph, "Refuted share of UNSAT targets vs Macaulay degree D", "Macaulay degree D", "refuted / UNSAT");
    let mut ls: BTreeMap<String, Vec<(i32, f64)>> = BTreeMap::new();
    for (l, dd, f) in lin {
        ls.entry(l.clone()).or_default().push((*dd, *f));
    }
    for (i, (l, pts)) in ls.iter().enumerate() {
        let c = ["#1f77b4", "#d62728", "#2ca02c", "#9467bd", "#ff7f0e", "#8c564b", "#e377c2", "#7f7f7f", "#bcbd22", "#17becf", "#000000"][i % 11];
        let mut path = String::new();
        for (k, (dd, f)) in pts.iter().enumerate() {
            let px = x1 + (*dd as f64 - 1.5) / 3.0 * pw2;
            let py = y0 + ph - f * ph + (i as f64 - 5.0) * 0.8; // tiny offset so overlapping lines stay visible
            write!(path, "{}{:.1},{:.1} ", if k == 0 { "M" } else { "L" }, px, py).unwrap();
            write!(s, "<circle cx='{px:.1}' cy='{py:.1}' r='2.5' fill='{c}'/>").unwrap();
        }
        let dash = if l.contains("m3") { " stroke-dasharray='5,3'" } else { "" };
        write!(s, "<path d='{path}' stroke='{c}' fill='none'{dash}/>\n").unwrap();
        write!(s, "<text x='{:.1}' y='{:.1}' fill='{c}' font-size='10'>{l}</text>\n", x1 + 6.0, y0 + 14.0 + 12.0 * i as f64).unwrap();
    }
    for dd in 2..=4 {
        write!(s, "<text x='{:.1}' y='{:.1}' text-anchor='middle'>{dd}</text>", x1 + (dd as f64 - 1.5) / 3.0 * pw2, y0 + ph + 14.0).unwrap();
    }
    s.push_str("<text x='450' y='370' text-anchor='middle' font-size='10'>L2 refutation filters, toy Koblitz y^2+xy=x^3+1, exact counts [pcc:unverified]; dashed = m=3 (split S3 chain)</text>\n</svg>\n");
    s
}

fn panel(s: &mut String, x: f64, y: f64, w: f64, h: f64, title: &str, xl: &str, yl: &str) {
    write!(s, "<rect x='{x}' y='{y}' width='{w}' height='{h}' fill='none' stroke='#444'/>\n").unwrap();
    for k in 0..=4 {
        let v = k as f64 / 4.0;
        let py = y + h - v * h;
        write!(s, "<line x1='{x}' x2='{}' y1='{py}' y2='{py}' stroke='#ddd'/><text x='{}' y='{}' text-anchor='end'>{:.2}</text>\n", x + w, x - 4.0, py + 4.0, v).unwrap();
    }
    write!(s, "<text x='{}' y='{}' text-anchor='middle' font-weight='bold'>{title}</text>\n", x + w / 2.0, y - 10.0).unwrap();
    write!(s, "<text x='{}' y='{}' text-anchor='middle'>{xl}</text>\n", x + w / 2.0, y + h + 28.0).unwrap();
    write!(s, "<text transform='translate({},{}) rotate(-90)' text-anchor='middle'>{yl}</text>\n", x - 40.0, y + h / 2.0).unwrap();
}
