#!/usr/bin/env python3
"""Validator instrumentation of the frozen mt_fieldops.rs (scratch copy only; never the repository file).
Adds, without touching any counted operation (replay must still match every row exactly):
 (1) MTFO_DUMP=<file>: per-target dump (t, planted x_t, recovered log, Q_t) after verification;
 (2) MTFO_CORRUPT=1: corrupt found[0] before verification (negative control);
 (3) stderr JSON: field-op cost and count of the chained restarts of the rho arms (phase 1 sp <- sp + B,
     phase 2 tp <- tp + B); target-phase lookups and lookups of an x already looked up for the same
     target (rho family db.get and the ic5 target-phase table.get); ic5 mod-n LA products/inverses.
usage: patch_dbg.py <in.rs> <out.rs>"""
import sys
src = open(sys.argv[1]).read()
def rep(old, new, count):
    global src
    c = src.count(old)
    assert c == count, (old, c, count)
    src = src.replace(old, new)
hdr = '''use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
static VR1: [AtomicU64; 5] = [AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0)];
static VR2: [AtomicU64; 5] = [AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0)];
static VN1: AtomicU64 = AtomicU64::new(0);
static VN2: AtomicU64 = AtomicU64::new(0);
static VLOOK: AtomicU64 = AtomicU64::new(0);
static VREP: AtomicU64 = AtomicU64::new(0);
static VLAM: AtomicU64 = AtomicU64::new(0);
static VLAI: AtomicU64 = AtomicU64::new(0);
thread_local! { static VSEEN: std::cell::RefCell<std::collections::HashSet<u64>> = std::cell::RefCell::new(std::collections::HashSet::new()); }
fn vl(x: u64) -> u64 { VLOOK.fetch_add(1, Relaxed); VSEEN.with(|s| if !s.borrow_mut().insert(x) { VREP.fetch_add(1, Relaxed); }); x }
fn vclear() { VSEEN.with(|s| s.borrow_mut().clear()); }
fn vsnap(f: &F) -> [u64; 5] { [f.ci.get(), f.cm.get(), f.cs.get(), f.ch.get(), f.cl.get()] }
fn vacc(acc: &[AtomicU64; 5], a: [u64; 5], b: [u64; 5]) { for z in 0..5 { acc[z].fetch_add(b[z] - a[z], Relaxed); } }
'''
rep("type Pt = Option<(u64, u64)>;", hdr + "type Pt = Option<(u64, u64)>;", 1)
rep("sp = f.padd(sp, bank[k1].0);",
    "{ let v0 = vsnap(&f); sp = f.padd(sp, bank[k1].0); vacc(&VR1, v0, vsnap(&f)); VN1.fetch_add(1, Relaxed); }", 2)
rep("tp = f.padd(tp, bank[k1].0);",
    "{ let v0 = vsnap(&f); tp = f.padd(tp, bank[k1].0); vacc(&VR2, v0, vsnap(&f)); VN2.fetch_add(1, Relaxed); }", 3)
# per-target repeated-lookup tracking
rep("let q = qs[t];", "let q = qs[t]; vclear();", 6)
rep("if let Some(&(a2, y2)) = db.get(&x) {", "if let Some(&(a2, y2)) = db.get(&vl(x)) {", 2)
rep("if let Some(&(a2, y2)) = db.get(&xm) {", "if let Some(&(a2, y2)) = db.get(&vl(xm)) {", 1)
rep("if let Some(&(a2, y2, aux)) = db.get(&x) {", "if let Some(&(a2, y2, aux)) = db.get(&vl(x)) {", 1)
rep("if let Some(&(a2, y2, aux)) = db.get(&xm) {", "if let Some(&(a2, y2, aux)) = db.get(&vl(xm)) {", 1)
rep("""                            if let Some(&e) = table.get(&xx) {
                                let dl = md(""", """                            if let Some(&e) = table.get(&vl(xx)) {
                                let dl = md(""", 1)
# ic5 mod-n LA counters
rep("""                f.cm.set(f.cm.get() + 1);
                ((a as u128 * b as u128) % n as u128) as u64""", """                f.cm.set(f.cm.get() + 1); VLAM.fetch_add(1, Relaxed);
                ((a as u128 * b as u128) % n as u128) as u64""", 1)
rep("""                f.ci.set(f.ci.get() + 1);
                let (mut r0, mut r1) = (n as i128, a as i128);""", """                f.ci.set(f.ci.get() + 1); VLAI.fetch_add(1, Relaxed);
                let (mut r0, mut r1) = (n as i128, a as i128);""", 1)
anchor = "    // Verification (counted): every recovered log by double-and-add.\n"
rep(anchor, '''    if std::env::var("MTFO_CORRUPT").is_ok() && targets > 0 { found[0] = (found[0] + 1) % n; }
''' + anchor, 1)
anchor2 = "    snap.push((\"verify\".into(), take(&f)));\n"
rep(anchor2, anchor2 + '''    if let Ok(path) = std::env::var("MTFO_DUMP") {
        use std::io::Write;
        let mut w = std::io::BufWriter::new(std::fs::File::create(path).unwrap());
        writeln!(w, "# {} p={} a={} b={} n={} gx={} gy={}", arm, p, f.a, f.b, n, g.unwrap().0, g.unwrap().1).unwrap();
        for t in 0..targets {
            let (qx, qy) = qs[t].unwrap();
            writeln!(w, "{} {} {} {} {}", t, xs[t], found[t], qx, qy).unwrap();
        }
    }
    let l = |a: &[AtomicU64; 5]| -> String { a.iter().map(|x| x.load(Relaxed).to_string()).collect::<Vec<_>>().join(",") };
    eprintln!("{{\\"restart_p1\\":[{}],\\"n_restart_p1\\":{},\\"restart_p2\\":[{}],\\"n_restart_p2\\":{},\\"lookups_p2\\":{},\\"repeat_lookups_p2\\":{},\\"la_modn_M\\":{},\\"la_modn_I\\":{}}}", l(&VR1), VN1.load(Relaxed), l(&VR2), VN2.load(Relaxed), VLOOK.load(Relaxed), VREP.load(Relaxed), VLAM.load(Relaxed), VLAI.load(Relaxed));
''', 1)
open(sys.argv[2], "w").write(src)
print("patched")
