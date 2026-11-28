//! Scores runs/*.jsonl with the frozen PROTOCOL.md decision rule.
use std::collections::BTreeMap;
fn field<'a>(l: &'a str, k: &str) -> Option<&'a str> {
    let p = l.find(&format!("\"{}\":", k))? + k.len() + 3; let r = &l[p..];
    let e = if r.starts_with('"') { return Some(&r[1..1 + r[1..].find('"')?]); } else { r.find(|c| c == ',' || c == '}')? };
    Some(&r[..e])
}
fn main() {
    let mut cells: BTreeMap<(u32, String), Vec<String>> = BTreeMap::new();
    for f in std::env::args().skip(1) { for l in std::fs::read_to_string(&f).unwrap().lines() {
        if field(l, "skipped").is_some() { println!("SKIPPED {}", l); continue; }
        cells.entry((field(l, "n").unwrap().parse().unwrap(), field(l, "curve").unwrap().to_string())).or_default().push(l.to_string()); } }
    let (mut nonflat_real, mut pint_ok, mut void) = (0, true, false);
    println!("n\tcurve\tarm\tseed\tfb\tD\tZ\tnull_med\tnull_max\tyield\tverdict\tnote");
    for ((n, c), rows) in &cells {
        for r in rows.iter().filter(|r| !field(r, "arm").unwrap().starts_with("null")) {
            let arm = field(r, "arm").unwrap(); let seed = field(r, "seed").unwrap();
            let z: f64 = field(r, "Z").unwrap().parse().unwrap(); let d: f64 = field(r, "D").unwrap().parse().unwrap();
            let key = format!("null:{}:{}", arm, seed);
            let mut nz: Vec<f64> = rows.iter().filter(|q| field(q, "arm") == Some(key.as_str())).map(|q| field(q, "Z").unwrap().parse().unwrap()).collect();
            let nd: Vec<f64> = rows.iter().filter(|q| field(q, "arm") == Some(key.as_str())).map(|q| field(q, "D").unwrap().parse().unwrap()).collect();
            // controls use the poly arm's nulls (same size B by construction)
            if nz.is_empty() { nz = rows.iter().filter(|q| field(q, "arm") == Some("null:poly:0")).map(|q| field(q, "Z").unwrap().parse().unwrap()).collect(); }
            nz.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let med = (nz[nz.len() / 2 - 1] + nz[nz.len() / 2]) / 2.0; let mx = *nz.last().unwrap();
            let nonflat = z > mx && z > 1.5 * med;
            let y = if nd.is_empty() { f64::NAN } else { d / (nd.iter().sum::<f64>() / nd.len() as f64) };
            let note = field(r, "note").unwrap_or("");
            if note.contains("invariance_violations=") && !note.ends_with("invariance_violations=0") { void = true; }
            if ["poly", "frob", "rsub"].contains(&arm) && nonflat { nonflat_real += 1; }
            if arm == "pint" && !nonflat { pint_ok = false; }
            println!("{}\t{}\t{}\t{}\t{}\t{}\t{:.3}\t{:.3}\t{:.3}\t{:.4}\t{}\t{}", n, c, arm, seed, field(r, "fb").unwrap(), d, z, med, mx, y, if nonflat { "NON-FLAT" } else { "flat" }, note);
        }
    }
    let outcome = if void || !pint_ok { "VOID" } else if nonflat_real > 0 { "SIGNAL" } else { "FLAT" };
    println!("OUTCOME {} (non-flat real cells: {}, pint detector valid: {}, invariance void: {})", outcome, nonflat_real, pint_ok, void);
}
