#!/usr/bin/env python3
"""Step 3: lower-bound probe.  usage: min3.py OUT.jsonl n l first_target n_targets
Per target: scan-guided proofs from several sources, stronger minimisation, the smallest
verified proof, its sharing statistic, and the no-sharing reference."""
import sys, json, time, os
import scanproof as sp
from pysat.solvers import Solver

OUT, n, l, first, cnt = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4]), int(sys.argv[5])
sp.build_checker()
seed = sp.cell_seed(n, l)
cell = sp.make_cell(l, n, seed)
targets = sp.nondec_targets(cell, 6)
SOURCES = [("cd15", False), ("cd15", True), ("g4", True)] + ([("g4", False)] if l <= 3 else [])
out = open(OUT, "a")


def q(v, f):
    return v[min(len(v) - 1, int(f * len(v)))]


for ti in range(first, first + cnt):
    t0 = time.time()
    R = targets[ti]
    sysm = sp.system(cell, R)
    nv, clauses = sp.render_cnf(sysm)
    cp = os.path.join(sp.SCR, "min_%d_%d_%d.cnf" % (n, l, ti))
    sp.write_cnf(cp, nv, clauses)
    cand = []  # (label, Proof, last check)

    def consider(label, pr):
        t1, t2, fin, last = sp.trim(cp, pr, "min")
        if t1["status"] != "VERIFIED":
            cand.append((label, None, None, len(pr.cl)))
            return None
        cand.append((label, fin, last, len(pr.cl)))
        return fin, last

    for sv, warm in SOURCES:
        pr, cf = sp.scan_proof(sysm, nv, clauses, sv, warm)
        consider("%s_%s_T2" % (sv, "warm" if warm else "cold"), pr)

    def best():
        ok = [c for c in cand if c[1] is not None]
        return min(ok, key=lambda c: len(c[1].cl))

    # re-solve on the core original clauses only, without and with the core lemmas seeded
    for rnd in range(int(os.environ.get("E2ROUNDS", "3"))):
        lab, bpr, blast, _ = best()
        before = len(bpr.cl)
        r = sp.check(cp, bpr, "min", core_cnf=True)
        core = [[int(x) for x in ln.split()[:-1]] for ln in open(r["core_cnf"]) if not ln.startswith("p")]
        os.remove(r["core_cnf"])
        for sv in ("cd15", "g4"):
            pr, cf = sp.scan_proof(sysm, nv, core, sv, True)
            consider("resolve%d_%s_core" % (rnd, sv), pr)
            cvars = set(sysm.c)
            seedl = [c for c in bpr.cl if c and not all(abs(x) in cvars for x in c)]
            pr2, cf2 = sp.scan_proof(sysm, nv, core, sv, True, extra=seedl)
            full = sp.Proof()
            for c in seedl:
                full.add(c, ("lemma", -1))
            for c, ro in zip(pr2.cl, pr2.role):
                full.add(c, ro)
            consider("resolve%d_%s_seeded" % (rnd, sv), full)
        if len(best()[1].cl) >= before:
            break
    lab, bpr, blast, _ = best()
    consider("subsume", sp.subsume_filter(clauses, bpr))
    lab, bpr, blast, _ = best()
    assert sp.check(cp, bpr, "min")["status"] == "VERIFIED"
    fwd = sp.forward_check(clauses, bpr)
    sh = sorted(sp.sharing(bpr, blast["deps"], sysm))
    # no-sharing reference: each leaf's own cold cd15 proof, checked alone
    ref, ref_raw = 0, 0
    for x1 in range(1 << l):
        ass = sp.leaf_assumptions(sysm, x1)
        with Solver(name="cd15", bootstrap_with=clauses, with_proof=True) as s:
            assert not s.solve(assumptions=ass)
            lem = sp.parse_proof(s.get_proof())
        pr = sp.Proof()
        for c in lem:
            pr.add(c, ("lemma", x1))
        pr.add([-a for a in ass], ("leaf", x1))
        pr.add([], ("empty",))
        cpl = cp + ".leaf"
        sp.write_cnf(cpl, nv, clauses + [[a] for a in ass[:1]] * 0)
        # the leaf clause is the goal: check it by adding the assumptions as units
        sp.write_cnf(cpl, nv, clauses + [[a] for a in ass])
        pl = sp.Proof()
        for c in lem:
            pl.add(c, ("lemma", x1))
        pl.add([], ("empty",))
        t1, t2, fin, last = sp.trim(cpl, pl, "minleaf")
        assert t1["status"] == "VERIFIED"
        ref += len(fin.cl)
        ref_raw += len(pl.cl)
        os.remove(cpl)
    row = dict(n=n, l=l, seed=seed, target=ti, xR=R[0],
               candidates=[{"label": c[0], "raw": c[3], "trimmed": len(c[1].cl) if c[1] else None,
                            "verified": c[1] is not None} for c in cand],
               best_label=lab, best_clauses=len(bpr.cl), best_forward_check=bool(fwd),
               best_roles=sp.size_stats(bpr, range(len(bpr.cl))),
               sharing={"n": len(sh), "median": q(sh, 0.5), "mean": round(sum(sh) / len(sh), 3), "p90": q(sh, 0.9),
                        "max": sh[-1], "share_ge4": round(sum(1 for x in sh if x >= 4) / len(sh), 4),
                        "share_all_leaves": round(sum(1 for x in sh if x == (1 << l)) / len(sh), 4)},
               noshare_reference=ref, noshare_reference_raw=ref_raw, ratio_best_to_reference=round(len(bpr.cl) / ref, 4),
               leaves=1 << l, seconds=round(time.time() - t0, 1))
    out.write(json.dumps(row) + "\n")
    out.flush()
    print(n, l, ti, lab, len(bpr.cl), "ref", ref, row["sharing"], row["seconds"], file=sys.stderr, flush=True)
    os.remove(cp)
