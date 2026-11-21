"""Validation (not measured data): encoding vs brute force, planted lifts, trimmer checks."""
import os, random, subprocess, sys, json
from pysat.solvers import Solver
import pdp

HERE = os.path.dirname(os.path.abspath(__file__))
SCR = sys.argv[1]
TRIM = os.path.join(SCR, "rup_trim")
out = {}


def all_models(cnf, vm, cap=100000):
    sols = set()
    with Solver(name="cd15", bootstrap_with=cnf.clauses) as s:
        while s.solve():
            m = set(v for v in s.get_model() if v > 0)
            t = tuple(pdp.bits_to_int(m, vm[k]) for k in "abc")
            sols.add(t)
            block = []
            for k in "abc":
                for v in vm[k]:
                    block.append(-v if v in m else v)
            s.add_clause(block)
            assert len(sols) < cap
    return sols


# 1. model set == brute force, smallest cells
ok = True
nsat = 0
for (n, l) in [(13, 3), (11, 2), (11, 3)]:
    curve, rng = pdp.make_curve(n, 777 + n * 10 + l)
    tab = pdp.brute_table(curve, l)
    for t in range(8):
        if t < 5:
            xr = pdp.random_point(curve, rng)[0]
        else:
            xr, _ = pdp.planted_target(curve, l, rng)
        bs = pdp.brute_solutions(curve, l, xr, tab)
        cnf, vm = pdp.encode(curve, l, xr)
        ms = all_models(cnf, vm)
        nsat += bool(bs)
        if ms != bs:
            ok = False
            print("MISMATCH", n, l, t, len(ms), len(bs))
        # every brute solution really satisfies S4 (resultant semantics): check S3 pair directly
        for (x1, x2, x3) in list(bs)[:5]:
            F = curve.F
            r1 = F.quad_roots(F.sq(x1 ^ x2), F.mul(x1, x2), F.sq(F.mul(x1, x2)) ^ curve.b)
            assert any(curve.S3(x1, x2, X) == 0 and curve.S3(x3, xr, X) == 0 for X in r1)
out["model_set_equals_bruteforce"] = ok
out["cells_checked"] = 24
out["sat_targets_among_checked"] = nsat
print("1 model-set equality:", ok, "sat targets:", nsat)

# 2. planted lifts
ok2 = True
for (n, l) in [(13, 3), (17, 3), (17, 4)]:
    curve, rng = pdp.make_curve(n, 4242 + n * 10 + l)
    # curve arithmetic sanity
    for _ in range(20):
        P, Q, R = (pdp.random_point(curve, rng) for _ in range(3))
        assert curve.on_curve(curve.add(P, Q)) and curve.on_curve(curve.add(P, P))
        assert curve.add(curve.add(P, Q), R) == curve.add(P, curve.add(Q, R))
        S = curve.add(P, Q)
        if S is not None:
            assert curve.S3(P[0], Q[0], S[0]) == 0
    for t in range(6):
        xr, xs = pdp.planted_target(curve, l, rng)
        cnf, vm = pdp.encode(curve, l, xr)
        assume = []
        for k, x in zip("abc", xs):
            assume += [v if (x >> i) & 1 else -v for i, v in enumerate(vm[k])]
        with Solver(name="cd15", bootstrap_with=cnf.clauses) as s:
            r = s.solve(assumptions=assume)
            if not r:
                ok2 = False
                print("PLANTED UNSAT", n, l, t)
                continue
            m = set(v for v in s.get_model() if v > 0)
            got = tuple(pdp.bits_to_int(m, vm[k]) for k in "abc")
            if got != xs or not pdp.lifts_to_sum(curve, got, xr):
                ok2 = False
                print("PLANTED LIFT FAIL", n, l, t)
            if not s.solve():
                ok2 = False
out["planted_sat_and_lift"] = ok2
print("2 planted:", ok2)


# 3. trimmer checks
def py_rup_check(orig, lemmas):
    """independent forward RUP checker (naive propagation)."""
    db = [list(c) for c in orig]
    for lem in lemmas + [[]]:
        assign = {}
        for x in lem:
            assign[-x] = True
        changed, confl = True, False
        while changed and not confl:
            changed = False
            for c in db:
                un = None
                nun = 0
                sat = False
                for x in c:
                    if assign.get(x):
                        sat = True
                        break
                    if not assign.get(-x):
                        nun += 1
                        un = x
                if sat:
                    continue
                if nun == 0:
                    confl = True
                    break
                if nun == 1:
                    assign[un] = True
                    changed = True
        if not confl:
            return False
        db.append(lem)
    return True


def readcl(path):
    res = []
    for line in open(path):
        if line[0] in "pc":
            continue
        xs = [int(t) for t in line.split()]
        res.append(xs[:-1])
    return res


ok3 = True
done = 0
while done < 4:
    LL = 2 if done < 2 else 3
    curve, rng = pdp.make_curve(11 if LL == 2 else 13, 99 + done)
    tab = pdp.brute_table(curve, LL)
    xr = pdp.random_point(curve, rng)[0]
    if pdp.brute_solutions(curve, LL, xr, tab):
        done += 0
        continue
    cnf, vm = pdp.encode(curve, LL, xr)
    for sv in ("cd15", "g4"):
        with Solver(name=sv, bootstrap_with=cnf.clauses, with_proof=True) as s:
            assert s.solve() is False
            proof = s.get_proof()
        cp, pp = os.path.join(SCR, "v.cnf"), os.path.join(SCR, "v.drup")
        cl, cc = os.path.join(SCR, "v.core.drup"), os.path.join(SCR, "v.core.cnf")
        cnf.write(cp)
        open(pp, "w").write("\n".join(proof) + "\n")
        r = subprocess.run([TRIM, cp, pp, cl, cc], capture_output=True, text=True)
        print("   ", sv, r.stdout.strip())
        if "VERIFIED" not in r.stdout:
            ok3 = False
            continue
        core_orig = readcl(cc)
        core_lem = [c for c in readcl(cl) if c]
        with Solver(name="g3", bootstrap_with=core_orig) as s:
            if s.solve() is not False:
                ok3 = False
                print("core originals SAT?!")
        if LL == 2 and not py_rup_check(core_orig, core_lem):
            ok3 = False
            print("python RUP recheck failed")
        # corrupted proof: only the empty clause -> must FAIL unless UP refutes formula
        open(pp, "w").write("0\n")
        r2 = subprocess.run([TRIM, cp, pp], capture_output=True, text=True)
        if "FAILED" not in r2.stdout:
            ok3 = False
            print("empty-only proof accepted?!", r2.stdout)
    done += 1
# bogus proof on a satisfiable formula
curve, rng = pdp.make_curve(13, 5)
xr, xs = pdp.planted_target(curve, 3, rng)
cnf, vm = pdp.encode(curve, 3, xr)
cnf.write(os.path.join(SCR, "v.cnf"))
open(os.path.join(SCR, "v.drup"), "w").write("%d 0\n-%d 0\n0\n" % (vm["a"][0], vm["a"][0]))
r = subprocess.run([TRIM, os.path.join(SCR, "v.cnf"), os.path.join(SCR, "v.drup")], capture_output=True, text=True)
print("    bogus:", r.stdout.strip())
if "FAILED" not in r.stdout:
    ok3 = False
out["trimmer_checks"] = ok3
print("3 trimmer:", ok3)
json.dump(out, open(os.path.join(HERE, "validation.json"), "w"), indent=1)
