#!/usr/bin/env python3
"""Validator instrumentation of examples/mt_fieldops.rs (v4 validation; adds NO counted operation).

usage: patch_instr.py <pristine main.rs> <out main.rs>

Adds, all to stderr (stdout and every counter are left unchanged):
  * HOP: an independent count of every HashMap get/entry/insert call in every arm, so the harness
    hash counter H can be compared with the number of hash-table operations actually performed;
  * a dump of (planted x_t, recovered log, Q_t) for every target, for scalar verification by
    independent Python code (validation/scripts/ecverify.py);
  * CANON, PIDX, DPT: counts of the uncounted per-step native work of the rho walks (rhodp
    canonicalisations, step-index hashes of every rho arm, rhodp distinguished-point tests);
  * a sample (x % 211 == 0, at most 4000 per run) of the entries the retention arms insert with a
    claimed log (rho2xks retention, rhodp precomputation and retention, ic5ks retention), so the
    claimed logs can be checked independently.
The dump is only written when MTFO_DUMP=1 (HOP and the sample counts are always written).
"""
import re, sys
src = open(sys.argv[1]).read()
n0 = len(src)
head = '''use std::sync::atomic::{AtomicU64, Ordering as AOrd};
static HOP: AtomicU64 = AtomicU64::new(0);
fn hop() { HOP.fetch_add(1, AOrd::Relaxed); }
static CANON: AtomicU64 = AtomicU64::new(0);
static PIDX: AtomicU64 = AtomicU64::new(0);
static DPT: AtomicU64 = AtomicU64::new(0);
thread_local! { static SAMP: std::cell::RefCell<Vec<String>> = std::cell::RefCell::new(Vec::new()); }
fn samp(s: String) { SAMP.with(|v| { let mut v = v.borrow_mut(); if v.len() < 4000 { v.push(s) } }) }
'''
src = src.replace("type Pt = Option<(u64, u64)>;", head + "type Pt = Option<(u64, u64)>;", 1)
# count every hash-map operation (immutable get, mutable entry/insert)
cnt = 0
def rep(m):
    global cnt; cnt += 1
    var, op = m.group(1), m.group(2)
    ref = "&" if op == "get" else "&mut "
    return "{ hop(); %s%s }.%s(" % (ref, var, op)
src = re.sub(r"\b(table|seen|seen_pairs|db|baby|pending|ret)\.(get|entry|insert)\(", rep, src)
# uncounted native work of the rho walks: canonicalisations, partition (step-index) hashes, DP tests
for old, new in [
    ("let canon = |p: Pt| -> (Pt, i64) {", "let canon = |p: Pt| -> (Pt, i64) { CANON.fetch_add(1, AOrd::Relaxed);"),
    ("let idx = |p: Pt| -> usize { p.map(", "let idx = |p: Pt| -> usize { PIDX.fetch_add(1, AOrd::Relaxed); p.map("),
    ("|p: Pt| -> bool { p.map(|(x, _)| (mix(x) >> 8) & mask == 0)", "|p: Pt| -> bool { DPT.fetch_add(1, AOrd::Relaxed); p.map(|(x, _)| (mix(x) >> 8) & mask == 0)"),
]:
    assert src.count(old) == 1, old
    src = src.replace(old, new)
k = src.count("let hidx = |pt: Pt| -> usize { pt.map(")
assert k >= 4, k
src = src.replace("let hidx = |pt: Pt| -> usize { pt.map(", "let hidx = |pt: Pt| -> usize { PIDX.fetch_add(1, AOrd::Relaxed); pt.map(")
# retention samples
def must(old, new):
    global src
    assert src.count(old) == 1, old
    src = src.replace(old, new)
must("""                for (x, off, y, aux) in trail {
                    f.hash();""",
"""                for (x, off, y, aux) in trail {
                    if x % 211 == 0 {
                        let ap = arms[aux.2].0.unwrap();
                        samp(format!("[\\"ks\\",{},{},{},{},{},{},{},{}]", x, (off + v) % n, if y == NOY { 0 } else { 1 }, if y == NOY { 0 } else { y }, aux.0, aux.1, ap.0, ap.1));
                    }
                    f.hash();""")
must("""                        { hop(); &mut db }.entry(p.unwrap().0).or_insert(a);""",
"""                        if p.unwrap().0 % 211 == 0 { samp(format!("[\\"dppre\\",{},{}]", p.unwrap().0, a)); }
                        { hop(); &mut db }.entry(p.unwrap().0).or_insert(a);""")
must("""                for (x, (a, b)) in pending {
                    f.hash();""",
"""                for (x, (a, b)) in pending {
                    if x % 211 == 0 { samp(format!("[\\"dpret\\",{},{}]", x, md(a as i128 + b as i128 * v as i128, n))); }
                    f.hash();""")
must("""                for (xx, kk2, s2) in trail {
                    f.hash();""",
"""                for (xx, kk2, s2) in trail {
                    if xx % 211 == 0 { samp(format!("[\\"icret\\",{},{}]", xx, md(found[t] as i128 + s2 as i128 * w_log[kk2 as usize] as i128, n))); }
                    f.hash();""")
# dump after verification (uncounted: reads only)
must("""    snap.push(("verify".into(), take(&f)));""",
"""    snap.push(("verify".into(), take(&f)));
    {
        let samples = SAMP.with(|v| v.borrow().clone());
        let dump = std::env::var("MTFO_DUMP").map(|s| s == "1").unwrap_or(false);
        let mut s = format!("{{\\"hop\\":{},\\"canon\\":{},\\"pidx\\":{},\\"dpt\\":{},\\"nsamp\\":{},\\"samp\\":[{}]", HOP.load(AOrd::Relaxed), CANON.load(AOrd::Relaxed), PIDX.load(AOrd::Relaxed), DPT.load(AOrd::Relaxed), samples.len(), samples.join(","));
        if dump {
            let v: Vec<String> = (0..targets).map(|t| { let (qx, qy) = qs[t].unwrap(); format!("[{},{},{},{}]", xs[t], found[t], qx, qy) }).collect();
            s.push_str(&format!(",\\"q\\":[{}]", v.join(",")));
        }
        s.push('}');
        eprintln!("{}", s);
    }""")
open(sys.argv[2], "w").write(src)
print("hashmap call sites instrumented:", cnt, file=sys.stderr)
