#!/usr/bin/env python3
"""Validation of generator, encoding, engine and checker on the smallest cell. usage: validate.py [l n]"""
import sys, json, random
import scanproof as sp
import sub
from pysat.solvers import Solver

l, n = (int(sys.argv[1]), int(sys.argv[2])) if len(sys.argv) > 2 else (3, 11)
sp.build_checker()
res = {"l": l, "n": n}
cell = sp.make_cell(l, n, sp.cell_seed(n, l) + 7)  # validation stream, not the measured one
E_ = cell.E
T = (0, None)


def per_x1_truth(R):
    """set of x1 for which a signed decomposition with x(P1) = x1 exists (plain enumeration)."""
    out = set()
    for a in cell.signed:
        for b in cell.signed:
            for c in cell.signed:
                if E_.add(E_.add(a, b), c) == R:
                    out.add(a[0])
    return out


def solve(clauses, ass=()):
    with Solver(name="cd15", bootstrap_with=clauses) as s:
        if s.solve(assumptions=list(ass)):
            m = s.get_model()
            return True, [None] + [x > 0 for x in m]
        return False, None


# 1. CNF vs enumeration, lifting, per-leaf agreement, engine
tg = []
while len(tg) < 14:
    R = cell.random_target()
    if not sp.u_zero_event(cell, R):
        tg.append(R)
tg += sp.planted_targets(cell, 10)
n_sat = n_unsat = leaf_checks = 0
for R in tg:
    truth = cell.brute(R)
    assert truth == cell.decomposable(R)
    sysm = sp.system(cell, R)
    nv, cl = sp.render_cnf(sysm)
    sat, model = solve(cl)
    assert sat == truth, ("CNF vs enumeration", R)
    if sat:
        n_sat += 1
        assert cell.lifts(sysm.x_of(model), R), ("model does not lift", R)
    else:
        n_unsat += 1
    good = per_x1_truth(R)
    for x1 in range(1 << l):
        s1, m1 = solve(cl, sp.leaf_assumptions(sysm, x1))
        assert s1 == (x1 in good), ("leaf", R, x1)
        if s1:
            assert sysm.x_of(m1)[0] == x1 and cell.lifts(sysm.x_of(m1), R)
        leaf_checks += 1
    assert sp.xor_scan(sysm)["sat"] == truth, ("engine", R)
res["cnf_vs_enumeration"] = {"targets": len(tg), "sat": n_sat, "unsat": n_unsat, "leaf_checks": leaf_checks, "pass": True}

# 2. u = 0 crafted targets: R = P1 + T
Tp = (0, cell.F.sq(cell.F.sq(cell.B)))  # placeholder, fixed below
y = cell.B
for _ in range(cell.n - 1):
    y = cell.F.sq(y)
Tp = (0, y)  # y^2 = B
assert E_.on(Tp)
crafted = false_sat_without = agree_with = strict_true = 0
tried = 0
seedB = 0
while crafted < 6 and tried < 400:
    tried += 1
    c2 = sub.Cell(l, 5000 + tried)
    y = c2.B
    for _ in range(c2.n - 1):
        y = c2.F.sq(y)
    T2 = (0, y)
    for P in c2.signed:
        R = c2.E.add(P, T2)
        if R is None or not c2.ok_target(R):
            continue
        strict = any(c2.E.add(c2.E.add(a, b), c) == R and (lambda q: q is not None and q[0] != 0)(c2.E.add(b, c))
                     for a in c2.signed for b in c2.signed for c in c2.signed)
        plain = c2.brute(R)
        s0 = sub.System(c2, R)
        sat0, _ = solve(sp.render_cnf(s0)[1])
        s1 = sp.system(c2, R)
        sat1, m1 = solve(sp.render_cnf(s1)[1])
        crafted += 1
        false_sat_without += int(sat0 and not plain)
        agree_with += int(sat1 == strict)
        strict_true += int(strict)
        if sat1:
            assert c2.lifts(s1.x_of(m1), R)
        assert sp.u_zero_event(c2, R)
        break
res["u_zero"] = {"crafted": crafted, "false_sat_without_clause": false_sat_without,
                 "with_clause_agrees_with_strict_enumeration": agree_with, "strict_true": strict_true,
                 "pass": crafted > 0 and agree_with == crafted}

# 3. checker
cell = sp.make_cell(l, n, sp.cell_seed(n, l) + 7)
non = sp.nondec_targets(cell, 2)
chk = []
for k, R in enumerate(non):
    sysm = sp.system(cell, R)
    nv, cl = sp.render_cnf(sysm)
    cp = sp.SCR + "/val.cnf"
    sp.write_cnf(cp, nv, cl)
    for warm in (False, True):
        pr, cf = sp.scan_proof(sysm, nv, cl, "cd15", warm)
        t1, t2, fin, last = sp.trim(cp, pr, "val")
        assert t1["status"] == "VERIFIED"
        fwd = sp.forward_check(cl, fin)
        core = [ln.split()[:-1] for ln in open(sp.SCR + "/val.drup.corecnf")][1:] if False else None
        r = sp.check(cp, fin, "val", core_cnf=True)
        cc = [[int(x) for x in ln.split()[:-1]] for ln in open(r["core_cnf"]) if not ln.startswith("p")]
        with Solver(name="g3", bootstrap_with=cc) as s:
            core_unsat = not s.solve()
        stripped = pr.sub([i for i, ro in enumerate(pr.role) if ro[0] != "lemma"])
        rej1 = sp.check(cp, stripped, "val")["status"]
        chk.append({"target": k, "warm": warm, "raw": len(pr.cl), "T2": len(fin.cl), "forward": fwd,
                    "core_originals_unsat_g3": core_unsat, "stripped_proof": rej1})
        assert fwd and core_unsat and rej1 == "FAILED"
# forged proof for a decomposable target
R = sp.planted_targets(cell, 1)[0]
sysm = sp.system(cell, R)
nv, cl = sp.render_cnf(sysm)
cp = sp.SCR + "/val.cnf"
sp.write_cnf(cp, nv, cl)
pr = sp.Proof()
for x1 in range(1 << l):
    with Solver(name="cd15", bootstrap_with=cl, with_proof=True) as s:
        if not s.solve(assumptions=sp.leaf_assumptions(sysm, x1)):
            for c in sp.parse_proof(s.get_proof()):
                pr.add(c, ("lemma", x1))
    pr.add([-a for a in sp.leaf_assumptions(sysm, x1)], ("leaf", x1))  # forged on SAT leaves
pr.close_tree(sysm)
forged = sp.check(cp, pr, "val")["status"]
assert forged == "FAILED" and not sp.forward_check(cl, pr)
res["checker"] = {"proofs": chk, "forged_proof_for_decomposable_target": forged, "pass": True}
res["all_pass"] = all(res[k]["pass"] for k in ("cnf_vs_enumeration", "u_zero", "checker"))
json.dump(res, open("validation.json", "w"), indent=1)
print(json.dumps(res, indent=1))
