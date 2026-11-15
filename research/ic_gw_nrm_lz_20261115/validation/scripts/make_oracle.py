#!/usr/bin/env python3
"""Validator oracle build of CAND: same counted work, plus uncounted raw-arithmetic checks of every NRM skip
and every LZ_REUSE reuse (stderr lines ORACLE_NRM / ORACLE_LZ). Usage: make_oracle.py SRC_DIR (edited in place)."""
import sys, pathlib
d = pathlib.Path(sys.argv[1])
def sub(path, old, new, count=1):
    p = d / path; t = p.read_text()
    n = t.count(old)
    assert n == count, (path, old[:60], n)
    p.write_text(t.replace(old, new))

# ---------------- ic2.rs: NRM checks (raw u128 arithmetic, never through Fp, so no counter moves)
sub("ic2.rs", "pub fn find_relations_2lp(cv: &Curve, fb: &[P3]) -> DOut {\n    let f = &cv.f;\n    let q = f.q;",
"""pub fn find_relations_2lp(cv: &Curve, fb: &[P3]) -> DOut {
    let f = &cv.f;
    let q = f.q;
    let rm = |x: u64, y: u64| -> u64 { ((x as u128 * y as u128) % q as u128) as u64 };
    let ra = |x: u64, y: u64| -> u64 { ((x as u128 + y as u128) % q as u128) as u64 };
    let rs = |x: u64, y: u64| -> u64 { ((x as u128 + q as u128 - (y % q) as u128) % q as u128) as u64 };
    // [g3, g1, g2, disc, lin, norm] skips used / bad; pairs with b0==0 / a0==0 seen; full-value mismatches
    let mut o_used = [0u64; 6];
    let mut o_bad = [0u64; 6];
    let mut o_b0zero = 0u64;
    let mut o_p0zero = 0u64;
    let mut o_full_bad = 0u64;""")
sub("ic2.rs", """            for t in 1..6 {
                g2 = f.add(g2, f.mul(ha[t], mb[t]));
            }
""", """            for t in 1..6 {
                g2 = f.add(g2, f.mul(ha[t], mb[t]));
            }
            if nrm {
                // REF values by raw arithmetic
                let g3r = ra(ra(rm(ga[0], b[0]), rm(ga[1], b[1])), rm(ga[2], b[2]));
                let g1r = ra(ra(rm(gb[0], a[0]), rm(gb[1], a[1])), rm(gb[2], a[2]));
                let mut g2r = 0u64;
                for t in 0..6 { g2r = ra(g2r, rm(ha[t], mb[t])); }
                if b[0] == 0 { o_b0zero += 1; }
                if nb { o_used[0] += 1; if rm(ga[0], b[0]) != ga[0] || b[0] != 1 { o_bad[0] += 1; } }
                if na { o_used[1] += 1; if rm(gb[0], a[0]) != gb[0] || a[0] != 1 { o_bad[1] += 1; } }
                if nb { o_used[2] += 1; if ra(0, rm(ha[0], mb[0])) != ha[0] || mb[0] != 1 { o_bad[2] += 1; } }
                if g3 != g3r || g1 != g1r || g2 != g2r { o_full_bad += 1; }
            }
""")
sub("ic2.rs", """            let disc = if nrm {
                let t = f.add(g1, g1);
                f.sub(f.mul(g2, g2), f.mul(f.add(t, t), g3))
            } else {
                f.sub(f.mul(g2, g2), f.mul(f.mul(4, g1), g3))
            };
""", """            let disc = if nrm {
                let t = f.add(g1, g1);
                f.sub(f.mul(g2, g2), f.mul(f.add(t, t), g3))
            } else {
                f.sub(f.mul(g2, g2), f.mul(f.mul(4, g1), g3))
            };
            if nrm {
                o_used[3] += 1;
                let dr = rs(rm(g2, g2), rm(rm(4, g1), g3));
                if dr != disc || ra(ra(g1, g1), ra(g1, g1)) != rm(4, g1) { o_bad[3] += 1; }
            }
""")
sub("ic2.rs", """            let lin = |u: u64| -> P3 { [if nrm && a[0] == 1 { f.add(u, b[0]) } else { f.add(f.mul(u, a[0]), b[0]) }, f.add(f.mul(u, a[1]), b[1]), f.add(f.mul(u, a[2]), b[2])] };
""", """            let lin = |u: u64| -> P3 { [if nrm && a[0] == 1 { f.add(u, b[0]) } else { f.add(f.mul(u, a[0]), b[0]) }, f.add(f.mul(u, a[1]), b[1]), f.add(f.mul(u, a[2]), b[2])] };
            let lin_ref = |u: u64| -> P3 { [ra(rm(u, a[0]), b[0]), ra(rm(u, a[1]), b[1]), ra(rm(u, a[2]), b[2])] };
            let mut lchk = |u: u64, p: &P3| { if nrm && a[0] == 1 { o_used[4] += 1; if lin_ref(u) != *p { o_bad[4] += 1; } } };
""")
sub("ic2.rs", """                    let u1 = f.mul(f.add(ng2, c.r), den[ci]);
                    let u2 = f.mul(f.sub(ng2, c.r), den[ci]);
                    pts.push(lin(u1));
                    owner.push((ci, 0));
                    pts.push(lin(u2));
""", """                    let u1 = f.mul(f.add(ng2, c.r), den[ci]);
                    let u2 = f.mul(f.sub(ng2, c.r), den[ci]);
                    let p1 = lin(u1); lchk(u1, &p1);
                    let p2 = lin(u2); lchk(u2, &p2);
                    pts.push(p1);
                    owner.push((ci, 0));
                    pts.push(p2);
""")
sub("ic2.rs", """                    let u0 = f.mul(f.neg(c.g2), den[ci]);
                    pts.push(lin(u0));
""", """                    let u0 = f.mul(f.neg(c.g2), den[ci]);
                    let p0 = lin(u0); lchk(u0, &p0);
                    pts.push(p0);
""")
sub("ic2.rs", """        for (k, p) in pts.iter_mut().enumerate() {
            if pv[k] != 1 {
                if nrm && p[0] != 0 {
""", """        for (k, p) in pts.iter_mut().enumerate() {
            if nrm && pv[k] != 1 {
                let pr = [rm(p[0], pv[k]), rm(p[1], pv[k]), rm(p[2], pv[k])];
                if p[0] != 0 { o_used[5] += 1; if pr[0] != 1 { o_bad[5] += 1; } } else { o_p0zero += 1; }
                let _ = pr;
            }
            let pr_full = [rm(p[0], pv[k]), rm(p[1], pv[k]), rm(p[2], pv[k])];
            let need = pv[k] != 1;
            let old = *p;
            if pv[k] != 1 {
                if nrm && p[0] != 0 {
""")
sub("ic2.rs", """                    *p = [f.mul(p[0], pv[k]), f.mul(p[1], pv[k]), f.mul(p[2], pv[k])];
                }
            }
        }
""", """                    *p = [f.mul(p[0], pv[k]), f.mul(p[1], pv[k]), f.mul(p[2], pv[k])];
                }
            }
            if nrm && need && *p != pr_full { o_full_bad += 1; }
            if nrm && !need && *p != old { o_full_bad += 1; }
        }
""")
sub("ic2.rs", """                    out.edges.push(DRel { c: cs, u: ids[0], v: ids[1] })
                }
            }
        }
    }
    out
}
""", """                    out.edges.push(DRel { c: cs, u: ids[0], v: ids[1] })
                }
            }
        }
    }
    eprintln!("ORACLE_NRM nrm={} n={} used={:?} bad={:?} b0zero_pairs={} p0zero_resid={} full_value_bad={} skipped_mul={} add_delta_ref_minus_cand={}",
        nrm as u8, n, o_used, o_bad, o_b0zero, o_p0zero, o_full_bad,
        o_used[0] + o_used[1] + o_used[2] + o_used[3] + o_used[4] + o_used[5],
        o_used[2] as i64 - 2 * o_used[3] as i64);
    out
}
""")

# ---------------- la2.rs: LZ checks
sub("la2.rs", """    let lz = std::env::var("LZ_REUSE").is_ok();
    let mut wy_cur = 0u64; // w_i^T y by the recurrence (lz)
""", """    let lz = std::env::var("LZ_REUSE").is_ok();
    struct OG { lz: bool, pm1: bool, n: usize, it: u64, vvp_used: u64, vvp_bad: u64, wy_used: u64, wy_bad: u64, wa0_bad: u64, res: &'static str }
    impl Drop for OG { fn drop(&mut self) {
        let pred = if self.it >= 1 { (self.it as i64 - 1) * (2 * self.n as i64 - 2) - 1 } else { 0 };
        eprintln!("ORACLE_LZ lz={} pm1={} n={} passed_iters={} vvp_used={} vvp_bad={} wy_used={} wy_bad={} wA0_nonzero={} result={} pred_lmul_saving={}",
            self.lz as u8, self.pm1 as u8, self.n, self.it, self.vvp_used, self.vvp_bad, self.wy_used, self.wy_bad, self.wa0_bad, self.res, if self.lz { pred } else { 0 });
    } }
    let mut og = OG { lz, pm1, n, it: 0, vvp_used: 0, vvp_bad: 0, wy_used: 0, wy_bad: 0, wa0_bad: 0, res: "none" };
    let rdot = |a: &[u64], b: &[u64]| -> u64 { let mut s: u128 = 0; for i in 0..a.len() { s = (s + a[i] as u128 * b[i] as u128) % ell as u128; } s as u64 };
    let mut av0: Vec<u64> = Vec::new(); // A w_0 (= v_0), for the global A-orthogonality check w_i^T A w_0 = 0
    let mut wy_cur = 0u64; // w_i^T y by the recurrence (lz)
""")
sub("la2.rs", """        let wy = if lz && !first { wy_cur } else { dot(&w, &y, ell) };
        if first { s0 = s; }
""", """        let wy = if lz && !first { wy_cur } else { dot(&w, &y, ell) };
        og.it += 1;
        if lz && !first { og.wy_used += 1; if rdot(&w, &y) != wy { og.wy_bad += 1; } if rdot(&w, &av0) != 0 { og.wa0_bad += 1; } }
        if lz && first { av0 = v.clone(); }
        if first { s0 = s; }
""")
sub("la2.rs", """            let vvp = if lz { s } else { dot(&v, &v_prev, ell) };
""", """            let vvp = if lz { s } else { dot(&v, &v_prev, ell) };
            if lz { og.vvp_used += 1; if rdot(&v, &v_prev) != vvp { og.vvp_bad += 1; } }
""")
sub("la2.rs", """    let z: Vec<u64> = (0..n).map(|i| ls2(x[i], b[i], ell)).collect();
    if z.iter().all(|&e| e == 0) {
        return None;
    }
    Some(z)
}
""", """    let z: Vec<u64> = (0..n).map(|i| ls2(x[i], b[i], ell)).collect();
    if z.iter().all(|&e| e == 0) {
        og.res = "zero";
        return None;
    }
    og.res = "some";
    Some(z)
}
""")
print("oracle patched")
