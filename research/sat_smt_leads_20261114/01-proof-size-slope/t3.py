"""T3 exploratory minimisation at small cells: usage t3.py SCRATCH n l n_targets OUT.jsonl

For each UNSAT target (same generator and seed as run.py, series A):
 start: best iterated-trim core over solvers cd15,g4,g3,lgl on the full CNF.
 loop (max 5 rounds): re-solve ONLY the core original clauses of the current best
 proof with each solver, trim (iterated), keep the smallest verified core.
Also a 'lemma-seeded' variant: core originals + current core lemmas added as
clauses, re-solve; lemmas counted = seeded lemmas that stay in the core + new ones
(checked as one proof: seeded core lemmas followed by the new proof).
"""
import os, sys, json, subprocess, time
from pysat.solvers import Solver
import pdp

SCR, n, l, NT, OUT = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4]), sys.argv[5]
TRIM = os.path.join(SCR, "rup_trim")
seed = 20261114 * 1000003 + 1000 * n + 10 * l
curve, rng = pdp.make_curve(n, seed)
tab = pdp.brute_table(curve, l)
SOLVERS = ["cd15", "g4", "g3", "lgl"]
tag = os.path.join(SCR, "t3_%d_%d" % (n, l))


def readcl(path):
    res = []
    for line in open(path):
        if line[0] in "pc":
            continue
        res.append([int(t) for t in line.split()][:-1])
    return res


def wcnf(path, cls):
    with open(path, "w") as f:
        for c in cls:
            f.write(" ".join(map(str, c)) + " 0\n")


def trim_iter(cnf_path, proof_path, pref):
    """iterated trim; returns (core_lemmas, core_lemma_file, core_cnf_file) or None"""
    best = None
    cur = proof_path
    for r in range(8):
        cl, cc = "%s.%d.drup" % (pref, r), "%s.%d.cnf" % (pref, r)
        p = subprocess.run([TRIM, cnf_path, cur, cl, cc], capture_output=True, text=True)
        if "VERIFIED" not in p.stdout:
            return best
        d = dict(kv.split("=") for kv in p.stdout.split()[1:])
        k = int(d["core_lemmas"])
        if best is not None and k >= best[0]:
            break
        best = (k, cl, cc)
        cur = cl
    return best


def solve_trim(cls, sv, pref, seedlemmas=None):
    with Solver(name=sv, bootstrap_with=cls + (seedlemmas or []), with_proof=True) as s:
        assert s.solve() is False
        pr = s.get_proof()
        confl = s.accum_stats()["conflicts"]
    cp, pp = pref + ".in.cnf", pref + ".in.drup"
    wcnf(cp, cls)
    with open(pp, "w") as f:
        for c in (seedlemmas or []):
            f.write(" ".join(map(str, c)) + " 0\n")
        f.write("\n".join(pr) + "\n")
    b = trim_iter(cp, pp, pref)
    return confl, b


out = open(OUT, "a")
done = idx = 0
while done < NT:
    xr = pdp.random_point(curve, rng)[0]
    idx += 1
    if pdp.brute_solutions(curve, l, xr, tab):
        continue
    cnf, vm = pdp.encode(curve, l, xr)
    hist = []
    best = None
    cls = cnf.clauses
    row = dict(n=n, l=l, seed=seed, target_index=idx - 1, start={}, rounds=[])
    for sv in SOLVERS:
        confl, b = solve_trim(cls, sv, "%s.%s" % (tag, sv))
        row["start"][sv] = dict(conflicts=confl, core_T2=b[0] if b else None)
        if b and (best is None or b[0] < best[0]):
            best = (b[0], readcl(b[1]), readcl(b[2]))
    for rnd in range(5):
        improved = False
        rr = {}
        cur_best = best
        for sv in SOLVERS:
            for variant in ("core_only", "lemma_seeded"):
                seedl = [c for c in cur_best[1] if c] if variant == "lemma_seeded" else None
                confl, b = solve_trim(cur_best[2], sv, "%s.r.%s" % (tag, sv), seedl)
                rr[sv + ":" + variant] = dict(conflicts=confl, core_T2=b[0] if b else None)
                if b and b[0] < best[0]:
                    best = (b[0], readcl(b[1]), readcl(b[2]))
                    improved = True
        row["rounds"].append(rr)
        if not improved:
            break
    row["best_core_lemmas_T3"] = best[0]
    row["best_core_orig_T3"] = len(best[2])
    out.write(json.dumps(row) + "\n")
    out.flush()
    done += 1
print("t3 done", n, l)
