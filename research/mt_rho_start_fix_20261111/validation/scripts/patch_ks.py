#!/usr/bin/env python3
"""Validator sensitivity arm (scratch only, never the repository file): `rho2xks` = the harness's rho2x with the
Kuhn-Struik retention rule: after target t is solved, every point its walk computed (main points with y, x-only
siblings with the (prev, arm) needed to recompute them) has a known log and is inserted into the database
(one counted hash insert each, no field work).  Everything else (phase 1, the r-adding walk, the 32-step restarts
and their cost, lookups, sibling recompute on a hit, verification) is byte-for-byte the rho2x code, so this arm
only adds the standard multi-target-rho retention.  Also `ic5ks`: ic5 with the same retention of its solved
target trials x(Q_t +- W_k) (one counted hash insert per class; on a later hit the trial point is recomputed in
full with one counted padd to resolve the sign, as for a table entry).
usage: patch_ks.py <in.rs> <out.rs>"""
import sys, re
src = open(sys.argv[1]).read()
# ---- rho2xks ----
s = src.index('        "rho2x" => {'); e = src.index('        "rhoseq" => {')
blk = src[s:e]
ks = blk.replace('"rho2x" => {', '"rho2xks" => {', 1)
old_tgt_head = """                let mut tp: Pt = q;
                let mut ta = 0u64;
                let v = loop {"""
assert ks.count(old_tgt_head) == 1
ks = ks.replace(old_tgt_head, """                let mut tp: Pt = q;
                let mut ta = 0u64;
                let mut trail: Vec<(u64, u64, u64, (u64, u64, usize))> = Vec::new();
                let v = loop {""")
old_step = """                    let (nx, sx) = step_x(&f, prev, arms[h].0);
                    cur = nx;
                    ca = (ca + arms[h].1) % n;
                    st += 1;
                    steps_total += 1;
                    if let Some((x, y)) = cur {"""
assert ks.count(old_step) == 1
ks = ks.replace(old_step, """                    let (nx, sx) = step_x(&f, prev, arms[h].0);
                    cur = nx;
                    ca = (ca + arms[h].1) % n;
                    st += 1;
                    steps_total += 1;
                    if let Some((x, y)) = cur { trail.push((x, ca, y, (0, 0, 0))); }
                    if let (Some(xm), Some((px, py))) = (sx, prev) { trail.push((xm, (pa + n - arms[h].1) % n, NOY, (px, py, h))); }
                    if let Some((x, y)) = cur {""")
old_found = """                found[t] = v;
            }
            snap.push(("targets".into(), take(&f)));"""
assert ks.count(old_found) == 1
ks = ks.replace(old_found, """                found[t] = v;
                for (x, off, y, aux) in trail {
                    f.hash();
                    db.entry(x).or_insert(((off + v) % n, y, aux));
                }
            }
            snap.push(("targets".into(), take(&f)));""")
src = src[:e] + ks + src[e:]
# ---- ic5ks ----
s = src.index('        "ic5" => {'); e = src.index('        "rho" | "rho2" => {')
blk = src[s:e]
k5 = blk.replace('"ic5" => {', '"ic5ks" => {', 1)
# retained trial classes: x -> (t, kq, sign)
old = """            let mut trials = 0u64;
            for t in 0..targets {
                let q = qs[t];
                let mut kq = 0usize;
                let v = 'tgt: loop {"""
assert k5.count(old) == 1
k5 = k5.replace(old, """            let mut trials = 0u64;
            let mut ret: HashMap<u64, (u32, u32, i8)> = HashMap::new();
            for t in 0..targets {
                let q = qs[t];
                let mut kq = 0usize;
                let mut trail: Vec<(u64, u32, i8)> = Vec::new();
                let v = 'tgt: loop {""")
old = """                    if let Some((xp, xm)) = f.xpair(q, w_pts[kq]) {
                        for (xx, sw) in [(xp, 1i64), (xm, -1i64)] {
                            f.hash();
                            if let Some(&e) = table.get(&xx) {
                                let dl = md("""
assert k5.count(old) == 1
k5 = k5.replace(old, """                    if let Some((xp, xm)) = f.xpair(q, w_pts[kq]) {
                        for (xx, sw) in [(xp, 1i64), (xm, -1i64)] {
                            f.hash();
                            trail.push((xx, kq as u32, sw as i8));
                            if let Some(&(t2, k2, s2)) = ret.get(&xx) {
                                // Q + sw W_kq = eps (Q_t2 + s2 W_k2), Q_t2 solved: recompute the stored point (counted)
                                let wp2 = if s2 > 0 { w_pts[k2 as usize] } else { f.pneg(w_pts[k2 as usize]) };
                                let dpt2 = f.padd(qs[t2 as usize], wp2);
                                let dl = md(found[t2 as usize] as i128 + s2 as i128 * w_log[k2 as usize] as i128, n);
                                break 'tgt resolve(&f, q, w_pts[kq], sw, w_log[kq], dpt2, dl, n);
                            }
                            if let Some(&e) = table.get(&xx) {
                                let dl = md(""")
old = """                found[t] = v;
            }
            snap.push(("targets".into(), take(&f)));"""
assert k5.count(old) == 1
k5 = k5.replace(old, """                found[t] = v;
                for (xx, kk2, s2) in trail {
                    f.hash();
                    ret.entry(xx).or_insert((t as u32, kk2, s2));
                }
            }
            snap.push(("targets".into(), take(&f)));""")
src = src[:e] + k5 + src[e:]
open(sys.argv[2], "w").write(src)
print("patched")
