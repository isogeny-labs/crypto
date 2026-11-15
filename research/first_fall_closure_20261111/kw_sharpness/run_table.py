"""Measure D_ff (top-form / HPS-KW-CG convention) and D_ff (operational, HKY closure) for the Weil descent of S_4
over GF(2^n) with an n'-dimensional random subspace V, random curve, c = x(random point).
usage: python3 run_table.py OUT.jsonl "n:npr:samples,n:npr:samples,..." [procs]"""
import sys, json, random, time, os
from multiprocessing import Pool
import ffd
from kwgf import GF

def indep_nus(F, npr, rnd):
    while True:
        nus = [rnd.randrange(1, F.q) for _ in range(npr)]
        if ffd.rank_int(nus) == npr: return nus

def one(args):
    n, npr, idx = args
    t0 = time.time()
    rnd = random.Random(10_000 * n + 100 * npr + idx)
    F, a2, a6, c = ffd.random_instance(n, rnd)
    nus = indep_nus(F, npr, rnd)
    N = 3 * npr
    s = ffd.descend(F, nus, c, a6)
    M = ffd.masks(N)
    # check Lemma 3.1 / closed formula for the top part
    tf = ffd.top_formula(F, nus, c)
    formula_ok = all((s[k] & M.deg[6]) == tf[k] for k in range(n))
    degs = sorted({ffd.degree(v, M) for v in s})
    top = ffd.topform_analysis(s, N)
    # dim of F_2-span of E^3 computed in the field (independent of the matrix computation)
    e = ffd.moore_e(F, nus)
    ev = list(e.values())
    prods = set()
    import itertools
    for x, y, z in itertools.combinations_with_replacement(ev, 3):
        prods.add(F.mul(F.mul(x, y), z))
    dimE3 = ffd.rank_int(list(prods))
    dimE = ffd.rank_int(ev)
    t1 = time.time()
    c_op, log = ffd.operational_first_fall(s, N, cmax=8)
    return dict(n=n, npr=npr, idx=idx, N=N, a2=a2, a6=a6, c=c, nus=nus, poly=F.poly,
                s_degrees=degs, formula_ok=formula_ok, dimV=top['dimV'], dimVbar=top['dimVbar'], dimE=dimE, dimE3=dimE3,
                Dff_top=top.get('Dff_top'), nullity7=top.get('nullity_deg7'), rank7=top.get('rank_deg7'), rows7=top.get('rows_deg7'),
                six_n=6 * n, Dff_op=c_op, op_log=log, t_topform=round(t1 - t0, 2), t_op=round(time.time() - t1, 2))

if __name__ == "__main__":
    out = sys.argv[1]
    cells = [tuple(int(x) for x in t.split(":")) for t in sys.argv[2].split(",")]
    procs = int(sys.argv[3]) if len(sys.argv) > 3 else 4
    jobs = [(n, npr, i) for (n, npr, k) in cells for i in range(k)]
    with Pool(procs) as p, open(out, "a") as f:
        for r in p.imap_unordered(one, jobs):
            f.write(json.dumps(r) + "\n"); f.flush()
            print(r["n"], r["npr"], r["idx"], "dimV", r["dimV"], "dimVbar", r["dimVbar"], "dimE3", r["dimE3"], "top", r["Dff_top"], "null7", r["nullity7"], "op", r["Dff_op"], r["t_topform"], r["t_op"], flush=True)
