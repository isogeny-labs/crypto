#!/usr/bin/env python3
"""Measured runs. usage: run.py OUT.jsonl n l first_target n_targets arms [rounds]
arms: comma list of cold,warm,xor,xproof,free.  One JSON row per (target, arm), appended."""
import sys, json, time, os
import scanproof as sp

OUT, n, l, first, cnt, arms = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4]), int(sys.argv[5]), sys.argv[6].split(",")
rounds = int(sys.argv[7]) if len(sys.argv) > 7 else 8
SOLVER = os.environ.get("E_SOLVER", "cd15")
FREE_BUDGET = 300000
sp.build_checker()
seed = sp.cell_seed(n, l)
cell = sp.make_cell(l, n, seed)
targets = sp.nondec_targets(cell, 6)
out = open(OUT, "a")


def q(v, f):
    return v[min(len(v) - 1, int(f * len(v)))]


def share_stats(pr, deps, sysm):
    sh = sorted(sp.sharing(pr, deps, sysm))
    if not sh:
        return None
    return {"n": len(sh), "median": q(sh, 0.5), "mean": round(sum(sh) / len(sh), 3), "p90": q(sh, 0.9),
            "max": sh[-1], "share_ge4": round(sum(1 for x in sh if x >= 4) / len(sh), 4)}


def measure(pr, cp, clauses, sysm, tag):
    row = {"raw": sp.size_stats(pr, range(len(pr.cl)))}
    t = time.time()
    if rounds <= 1:
        t1 = sp.check(cp, pr, tag, deps=True)
        if t1["status"] != "VERIFIED":
            row.update(status=t1["status"], checked=False)
            return row
        fin = pr.sub(t1["core"])
        last = sp.check(cp, fin, tag, deps=True)
        assert last["status"] == "VERIFIED"
        t2 = None
    else:
        t1, t2, fin, last = sp.trim(cp, pr, tag, rounds)
        if t1["status"] != "VERIFIED":
            row.update(status=t1["status"], checked=False)
            return row
    row.update(status="VERIFIED", checked=True, T1=sp.size_stats(pr, t1["core"]), core_orig_T1=t1["core_orig"],
               final=sp.size_stats(fin, range(len(fin.cl))), final_kind="T2" if t2 else "T1-rechecked",
               core_orig_final=last["core_orig"], n_orig=last["n_orig"], trim_s=round(time.time() - t, 2))
    row["sharing"] = share_stats(fin, last["deps"], sysm)
    if len(fin.cl) <= 60000:
        t = time.time()
        row["forward_check"] = bool(sp.forward_check(clauses, fin))
        row["forward_s"] = round(time.time() - t, 2)
    else:
        row["forward_check"] = None
    return row


for ti in range(first, first + cnt):
    R = targets[ti]
    sysm = sp.system(cell, R)
    nv, clauses = sp.render_cnf(sysm)
    cp = os.path.join(sp.SCR, "run_%d_%d_%d.cnf" % (n, l, ti))
    sp.write_cnf(cp, nv, clauses)
    base = dict(n=n, l=l, seed=seed, target=ti, xR=R[0], yR=R[1], B=cell.B, poly=cell.F.poly, vars=nv, clauses=len(clauses))
    for arm in arms:
        row = dict(base, arm=arm)
        t = time.time()
        if arm in ("cold", "warm"):
            pr, cf = sp.scan_proof(sysm, nv, clauses, SOLVER, arm == "warm")
            assert pr is not None, "leaf SAT on a non-decomposable target"
            row.update(solver=SOLVER, conflicts=sum(cf), leaf_conflicts=cf, solve_s=round(time.time() - t, 2))
            row.update(measure(pr, cp, clauses, sysm, "run_%s" % arm))
        elif arm == "free":
            pr, cf, r = sp.free_proof(nv, clauses, "g4", FREE_BUDGET)
            row.update(solver="g4", conflicts=cf, solve_s=round(time.time() - t, 2), censored=pr is None)
            if pr is not None:
                row.update(measure(pr, cp, clauses, sysm, "run_free"))
            else:
                row.update(status="CENSORED", checked=False)
        elif arm == "xproof":
            import xproof
            r = xproof.measure(sysm, "run_xp_%d_%d_%d" % (n, l, ti))
            assert r["engine_verdict"] == "UNSAT", "xproof engine disagrees with ground truth"
            row.update(r, solve_s=round(time.time() - t, 2))
        elif arm == "xor":
            xs = sp.xor_scan(sysm)
            assert xs["sat"] is False, "engine disagrees with ground truth"
            tot = {k: xs["root"][k] + sum(lf[k] for lf in xs["leaves"]) for k in ("rowops", "wsum", "props", "nodes")}
            row.update(checked=False, status="UNCHECKED_COUNT", engine_verdict="UNSAT", root=xs["root"], total=tot,
                       steps=tot["rowops"] + tot["props"] + tot["nodes"],
                       translation_clauses=sp.xor_translation_clauses(tot),
                       leaf_rowops=[lf["rowops"] for lf in xs["leaves"]],
                       leaf_wsum=[lf["wsum"] for lf in xs["leaves"]],
                       leaf_nodes=[lf["nodes"] for lf in xs["leaves"]], solve_s=round(time.time() - t, 2))
        out.write(json.dumps(row) + "\n")
        out.flush()
        print(n, l, ti, arm, row.get("status"), row.get("conflicts"), (row.get("final") or {}).get("clauses"),
              row.get("translation_clauses"), row.get("sharing"), "t", round(time.time() - t, 1), file=sys.stderr, flush=True)
    os.remove(cp)
