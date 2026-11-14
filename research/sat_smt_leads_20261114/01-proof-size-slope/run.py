"""Measured runs. usage: run.py SCRATCH SERIES n l n_unsat n_sat OUT.jsonl

Appends one JSON row per (instance, solver). Baseline (solver conflicts) and
treatment (trim of the same proof) are produced back to back per instance,
solvers interleaved per instance.
"""
import os, sys, json, time, subprocess, random
from pysat.solvers import Solver
import pdp

SCR, SERIES, n, l, N_UNSAT, N_SAT, OUT = sys.argv[1:8]
n, l, N_UNSAT, N_SAT = int(n), int(l), int(N_UNSAT), int(N_SAT)
SOLVERS = os.environ.get("SOLVERS", "cd15,g4").split(",")
TRIM = os.path.join(SCR, "rup_trim")
seed = 20261114 * 1000003 + 1000 * n + 10 * l
curve, rng = pdp.make_curve(n, seed)
t0 = time.time()
tab = pdp.brute_table(curve, l)
t_tab = time.time() - t0
tag = "%s_%d_%d_%s" % (SERIES, n, l, os.environ.get("PART", "0/1").replace("/", "of"))


def trim(cp, pp, rounds=6):
    """T1 + T2: returns list of per-round dicts."""
    res = []
    cur = pp
    for r in range(rounds):
        cl = os.path.join(SCR, "%s.core%d.drup" % (tag, r))
        cc = os.path.join(SCR, "%s.core.cnf" % tag)
        t = time.time()
        p = subprocess.run([TRIM, cp, cur, cl, cc], capture_output=True, text=True)
        line = p.stdout.strip()
        d = dict(kv.split("=") for kv in line.split()[1:]) if line.startswith("RESULT") else {"status": "ERROR"}
        d["trim_s"] = round(time.time() - t, 3)
        res.append(d)
        if d["status"] != "VERIFIED":
            break
        if r > 0 and int(d["core_lemmas"]) >= int(res[-2]["core_lemmas"]):
            break
        cur = cl
    return res


def solve(cnf, sv, proof):
    with Solver(name=sv, bootstrap_with=cnf.clauses, with_proof=proof) as s:
        t = time.time()
        r = s.solve()
        dt = time.time() - t
        st = s.accum_stats()
        pr = s.get_proof() if (proof and r is False) else None
    return r, st, dt, pr


PK, PM = map(int, os.environ.get('PART', '0/1').split('/'))
n_unsat = n_sat = draws = 0
out = open(OUT, "a")
while (n_unsat < N_UNSAT or n_sat < N_SAT) and draws < 60:
    idx = draws
    draws += 1
    planted = n_unsat >= N_UNSAT  # after the UNSAT quota: planted SAT targets
    if planted:
        pt = pdp.planted_target(curve, l, rng)
        if pt is None:
            break
        xr, xs = pt
    else:
        xr = pdp.random_point(curve, rng)[0]
    sols = pdp.brute_solutions(curve, l, xr, tab)
    truth = bool(sols)
    if truth and n_sat >= N_SAT:
        continue
    if not truth and n_unsat >= N_UNSAT:
        continue
    if idx % PM != PK:
        if truth:
            n_sat += 1
        else:
            n_unsat += 1
        continue
    cnf, vm = pdp.encode(curve, l, xr)
    cp = os.path.join(SCR, tag + ".cnf")
    cnf.write(cp)
    for sv in SOLVERS:
        r, st, dt, pr = solve(cnf, sv, True)
        assert r == truth, ("solver disagrees with brute force", sv, n, l, idx)
        row = dict(series=SERIES, n=n, l=l, seed=seed, target_index=idx, xr=xr, b=curve.b,
                   field_poly=curve.F.f, planted=planted, truth="SAT" if truth else "UNSAT",
                   n_solutions_ordered=len(sols), solver=sv, vars=cnf.nv, clauses=len(cnf.clauses),
                   conflicts=st["conflicts"], decisions=st["decisions"],
                   propagations=st["propagations"], restarts=st["restarts"], solve_s=round(dt, 3))
        if pr is not None:
            pp = os.path.join(SCR, "%s.%s.drup" % (tag, sv))
            with open(pp, "w") as f:
                f.write("\n".join(pr) + "\n")
            row["proof_lines"] = len(pr)
            row["proof_adds"] = sum(1 for x in pr if not x.startswith("d"))
            tr = trim(cp, pp, int(os.environ.get('ROUNDS', '6')))
            row["trim_rounds"] = tr
            if tr[0]["status"] == "VERIFIED":
                row["core_lemmas_T1"] = int(tr[0]["core_lemmas"])
                row["core_lits_T1"] = int(tr[0]["core_lits"])
                row["core_orig_T1"] = int(tr[0]["core_orig"])
                good = [x for x in tr if x["status"] == "VERIFIED"]
                row["core_lemmas_T2"] = min(int(x["core_lemmas"]) for x in good)
                row["core_orig_T2"] = int(good[-1]["core_orig"])
            else:
                row["trim_failed"] = True
        out.write(json.dumps(row) + "\n")
        out.flush()
    if truth:
        n_sat += 1
    else:
        n_unsat += 1
print("done", tag, "unsat", n_unsat, "sat", n_sat, "draws", draws, "table_s", round(t_tab, 1),
      "total_s", round(time.time() - t0, 1))
