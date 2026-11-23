//! AMENDMENT-1 scoring: R1 (fresh-null percentile), R2 (fresh rsub non-flat rate), R3 (null-vs-null base rate).
use std::collections::BTreeMap;
fn field<'a>(l: &'a str, k: &str) -> Option<&'a str> {
    let p = l.find(&format!("\"{}\":", k))? + k.len() + 3; let r = &l[p..];
    if r.starts_with('"') { return Some(&r[1..1 + r[1..].find('"')?]); }
    Some(&r[..r.find(|c| c == ',' || c == '}')?])
}
fn z(l: &str) -> f64 { field(l, "Z").unwrap().parse().unwrap() }
fn rule(x: f64, nulls: &mut Vec<f64>) -> bool { nulls.sort_by(|a, b| a.partial_cmp(b).unwrap()); let n = nulls.len();
    let med = (nulls[n / 2 - 1] + nulls[n / 2]) / 2.0; x > nulls[n - 1] && x > 1.5 * med }
fn binom_tail(n: u64, k: u64, p: f64) -> f64 { // P(X >= k)
    let mut s = 0.0; for i in k..=n { let mut c = 1.0f64; for j in 0..i { c *= (n - j) as f64 / (j + 1) as f64; } s += c * p.powi(i as i32) * (1.0 - p).powi((n - i) as i32); } s }
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let r1: Vec<String> = std::fs::read_to_string(&a[1]).unwrap().lines().map(String::from).collect();
    let zf = z(r1.iter().find(|l| field(l, "arm") == Some("rsub")).unwrap());
    let mut nz: Vec<f64> = r1.iter().filter(|l| field(l, "arm").unwrap().starts_with("null")).map(|l| z(l)).collect();
    nz.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let above = nz.iter().filter(|&&v| v >= zf).count();
    println!("R1 flagged Z={:.3} fresh nulls={} p99(198th)={:.3} max={:.3} nulls>=Z: {} (empirical p={:.4}) replicated={}",
        zf, nz.len(), nz[197], nz[nz.len()-1], above, (above as f64 + 1.0) / (nz.len() as f64 + 1.0), zf > nz[197]);
    // R3 base rate from primary files
    let mut groups: BTreeMap<String, Vec<f64>> = BTreeMap::new();
    for f in &a[3..] { for l in std::fs::read_to_string(f).unwrap().lines() { if let Some(arm) = field(l, "arm") { if arm.starts_with("null") {
        groups.entry(format!("{}:{}:{}", field(l, "n").unwrap(), field(l, "curve").unwrap(), arm)).or_default().push(z(l)); } } } }
    let (mut tot, mut hit) = (0u64, 0u64);
    for (_, g) in &groups { for i in 0..g.len() { let mut rest: Vec<f64> = g.iter().enumerate().filter(|(j, _)| *j != i).map(|(_, v)| *v).collect(); tot += 1; if rule(g[i], &mut rest) { hit += 1; } } }
    let base = hit as f64 / tot as f64;
    println!("R3 null-vs-null base rate: {}/{} = {:.4}", hit, tot, base);
    // R2
    let r2: Vec<String> = std::fs::read_to_string(&a[2]).unwrap().lines().map(String::from).collect();
    let (mut n2, mut k2) = (0u64, 0u64);
    for l in r2.iter().filter(|l| field(l, "arm") == Some("rsub")) {
        let sd = field(l, "seed").unwrap(); let key = format!("null:rsub:{}", sd);
        let mut ns: Vec<f64> = r2.iter().filter(|q| field(q, "arm") == Some(key.as_str())).map(|q| z(q)).collect();
        let nf = rule(z(l), &mut ns); n2 += 1; if nf { k2 += 1; }
        println!("R2 rsub seed {} fb={} Z={:.3} null_max={:.3} {}", sd, field(l, "fb").unwrap(), z(l), ns[ns.len()-1], if nf { "NON-FLAT" } else { "flat" });
    }
    let p = binom_tail(n2, k2, base);
    println!("R2 non-flat {}/{} ; one-sided binomial p vs base rate = {:.4} ; replicated={}", k2, n2, p, p < 0.01);
}
