#!/usr/bin/env python3
"""Validator instrumentation of the frozen source (scratch copy only).  Adds, without touching any counted
operation: (1) MTFO_DUMP=<file>: per-target dump (t, planted x_t, recovered log, Q_t) after verification;
(2) MTFO_CORRUPT=1: corrupt found[0] before verification (negative control);
(3) stderr JSON with the field-op cost of the walk starts/restarts (phase 1 and phase 2) of the rho arms."""
import sys
src = open(sys.argv[1]).read()
n_before = src.count("cur = f.padd(f.padd(bank[k1].0, bank[k2].0), q);")
assert n_before == 3, n_before
assert src.count("cur = f.padd(f.padd(bank[k1].0, bank[k2].0), bank[k3].0);") == 2
hdr = '''use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
static VR1: [AtomicU64; 5] = [AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0)];
static VR2: [AtomicU64; 5] = [AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0)];
static VN1: AtomicU64 = AtomicU64::new(0);
static VN2: AtomicU64 = AtomicU64::new(0);
fn vsnap(f: &F) -> [u64; 5] { [f.ci.get(), f.cm.get(), f.cs.get(), f.ch.get(), f.cl.get()] }
fn vacc(acc: &[AtomicU64; 5], a: [u64; 5], b: [u64; 5]) { for z in 0..5 { acc[z].fetch_add(b[z] - a[z], Relaxed); } }
'''
src = src.replace("type Pt = Option<(u64, u64)>;", hdr + "type Pt = Option<(u64, u64)>;", 1)
src = src.replace("cur = f.padd(f.padd(bank[k1].0, bank[k2].0), q);",
                  "{ let v0 = vsnap(&f); cur = f.padd(f.padd(bank[k1].0, bank[k2].0), q); vacc(&VR2, v0, vsnap(&f)); VN2.fetch_add(1, Relaxed); }")
src = src.replace("cur = f.padd(f.padd(bank[k1].0, bank[k2].0), bank[k3].0);",
                  "{ let v0 = vsnap(&f); cur = f.padd(f.padd(bank[k1].0, bank[k2].0), bank[k3].0); vacc(&VR1, v0, vsnap(&f)); VN1.fetch_add(1, Relaxed); }")
anchor = "    // Verification (counted): every recovered log by double-and-add.\n"
assert src.count(anchor) == 1
src = src.replace(anchor, '''    if std::env::var("MTFO_CORRUPT").is_ok() && targets > 0 { found[0] = (found[0] + 1) % n; }
''' + anchor, 1)
anchor2 = "    snap.push((\"verify\".into(), take(&f)));\n"
assert src.count(anchor2) == 1
src = src.replace(anchor2, anchor2 + '''    if let Ok(path) = std::env::var("MTFO_DUMP") {
        use std::io::Write;
        let mut w = std::io::BufWriter::new(std::fs::File::create(path).unwrap());
        writeln!(w, "# {} p={} a={} b={} n={} gx={} gy={}", arm, p, f.a, f.b, n, g.unwrap().0, g.unwrap().1).unwrap();
        for t in 0..targets {
            let (qx, qy) = qs[t].unwrap();
            writeln!(w, "{} {} {} {} {}", t, xs[t], found[t], qx, qy).unwrap();
        }
    }
    let l = |a: &[AtomicU64; 5]| -> String { a.iter().map(|x| x.load(Relaxed).to_string()).collect::<Vec<_>>().join(",") };
    eprintln!("{{\\"restart_p1\\":[{}],\\"n_restart_p1\\":{},\\"restart_p2\\":[{}],\\"n_restart_p2\\":{}}}", l(&VR1), VN1.load(Relaxed), l(&VR2), VN2.load(Relaxed));
''', 1)
open(sys.argv[2], "w").write(src)
print("patched")
