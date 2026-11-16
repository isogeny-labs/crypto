"""Cost probe on the development seed 999 (not measured data). usage: probe.py n l [solver]"""
import sys, time
import scanproof as sp
n, l = int(sys.argv[1]), int(sys.argv[2]); sv = sys.argv[3] if len(sys.argv) > 3 else "cd15"
sp.build_checker()
cell = sp.make_cell(l, n, 999)
R = sp.nondec_targets(cell, 1)[0]
sysm = sp.system(cell, R); nv, cl = sp.render_cnf(sysm)
print("n", n, "l", l, "vars", nv, "clauses", len(cl))
t = time.time(); xs = sp.xor_scan(sysm); print("xor", xs["sat"], xs["root"], xs["leaves"][3], round(time.time() - t, 2))
for warm in (False, True):
    t = time.time(); pr, cf = sp.scan_proof(sysm, nv, cl, sv, warm); t1 = time.time() - t
    p = sp.SCR + "/probe.cnf"; sp.write_cnf(p, nv, cl)
    t = time.time(); a, b, fin, last = sp.trim(p, pr, "probe"); t2 = time.time() - t
    print(sv, "warm" if warm else "cold", "confl/leaf", sum(cf) / len(cf), "max", max(cf), "raw", len(pr.cl), a["status"],
          "T1", sp.size_stats(pr, a["core"]), "T2", len(fin.cl) if fin else None, "solve_s", round(t1, 1), "trim_s", round(t2, 1))
    if fin:
        sh = sorted(sp.sharing(fin, last["deps"], sysm)); print(" sharing median", sh[len(sh) // 2], "mean", sum(sh) / len(sh), "p90", sh[int(.9 * len(sh))], "max", sh[-1], "n", len(sh), "of", len(fin.cl))
