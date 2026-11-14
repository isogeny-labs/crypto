#!/usr/bin/env python3
"""VALIDATOR SENSITIVITY ARM (not pre-registered, not the producer's): adds arm `ic5rw` to a copy of
examples/mt_fieldops.rs.  ic5rw is ic5ks unchanged except for the shared trial sequence W:
  ic5ks : W_k = W_0 + k D (arithmetic progression).  Then a retained class x(Q_t +- W_k) meets a later
          trial x(Q' +- W_k') only if Q' -+ Q_t lies in {j D} or {+-(2 W_0 + j D)}, a set of O(K) points:
          retention covers O(K) points per solved target instead of O(K K').
  ic5rw : W_(k+1) = W_k + D_(j(k)), j(k) = splitmix64(k) mod 32, a sum over 32 random known-log points D_j
          (32 counted scalar multiplications, one counted addition per W as before), so the differences
          W_k - W_k' are spread and retention covers O(K K') points, like a retention rho's walk points.
Control arm `ic5w`: ic5 (no retention) with the same random-walk W, so the W change alone is measured.
Everything else (factor base, table, relations, one hash per trial class, one hash per retained insert,
counted recomputation on a retained hit, counted verification) is ic5ks's code, copied.
usage: patch_ic5rw.py <pristine main.rs> <out main.rs>"""
import sys
src = open(sys.argv[1]).read()
a = src.index('        "ic5ks" => {'); b = src.index('        "rho" | "rho2" => {')
blk = src[a:b]
blk = blk.replace('"ic5ks" => {', '"ic5rw" => {', 1)
old_dpt = "            let dpt = f.smul(g, dd);\n"
assert blk.count(old_dpt) == 1
blk = blk.replace(old_dpt, old_dpt + """            let _ = dpt;
            const RWN: usize = 32;
            let rwd: Vec<(Pt, u64)> = (0..RWN).map(|_| { let c = rng.gen_range(1..n); (f.smul(g, c), c) }).collect();
""")
old_w = """                    let last = *w_pts.last().unwrap();
                    w_pts.push(f.padd(last, dpt));
                    w_log.push((w_log.last().unwrap() + dd) % n);"""
assert blk.count(old_w) == 1
blk = blk.replace(old_w, """                    let last = *w_pts.last().unwrap();
                    // step index from the position k (splitmix64), not from the point: a point-driven
                    // r-adding W walk is a rho walk and cycles after ~sqrt(n) terms (first attempt, 9 runs hit
                    // the trial budget); a position-driven sum never cycles
                    let j = {
                        let mut z = (w_pts.len() as u64).wrapping_add(0x9E37_79B9_7F4A_7C15);
                        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
                        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
                        ((z ^ (z >> 31)) >> 59) as usize
                    };
                    w_pts.push(f.padd(last, rwd[j].0));
                    w_log.push((w_log.last().unwrap() + rwd[j].1) % n);
                    let _ = dd;""")
src = src[:b] + blk + src[b:]
# control arm ic5w: ic5 (no retention) with the same random-walk W, to separate the W change from retention
a = src.index('        "ic5" => {'); b = src.index('        "ic5ks" => {')
blk = src[a:b].replace('"ic5" => {', '"ic5w" => {', 1)
assert blk.count(old_dpt) == 1 and blk.count(old_w) == 1
blk = blk.replace(old_dpt, old_dpt + """            let _ = dpt;
            const RWN: usize = 32;
            let rwd: Vec<(Pt, u64)> = (0..RWN).map(|_| { let c = rng.gen_range(1..n); (f.smul(g, c), c) }).collect();
""")
blk = blk.replace(old_w, """                    let last = *w_pts.last().unwrap();
                    // step index from the position k (splitmix64), not from the point: a point-driven
                    // r-adding W walk is a rho walk and cycles after ~sqrt(n) terms (first attempt, 9 runs hit
                    // the trial budget); a position-driven sum never cycles
                    let j = {
                        let mut z = (w_pts.len() as u64).wrapping_add(0x9E37_79B9_7F4A_7C15);
                        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
                        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
                        ((z ^ (z >> 31)) >> 59) as usize
                    };
                    w_pts.push(f.padd(last, rwd[j].0));
                    w_log.push((w_log.last().unwrap() + rwd[j].1) % n);
                    let _ = dd;""")
src = src[:b] + blk + src[b:]
open(sys.argv[2], "w").write(src)
