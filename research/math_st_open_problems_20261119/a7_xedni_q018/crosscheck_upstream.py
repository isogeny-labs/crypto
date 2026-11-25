"""Port check only: compare the Rust crate's lattice/model components with
upstream math-st `reproduce_xedni_p17.py` (commit f631bcc) on fixed inputs.
Reads results/fixtures.json (written by `a7_xedni_q018 fixtures results`),
prints match/mismatch per item. Computes no reported result."""
import json, sys
from fractions import Fraction
from pathlib import Path

UP = Path(sys.argv[1] if len(sys.argv) > 1 else "/home/user/math-street/math-st")
sys.path.insert(0, str(UP))
sys.path.insert(0, str(UP / "problems/P1.6-height-obstruction/code"))
import reproduce_xedni_p17 as up  # noqa: E402
from sympy import Matrix  # noqa: E402
from lib.curves import Curve  # noqa: E402

def in_lattice(vecs, basis):
    B = Matrix(basis).T
    for v in vecs:
        sol = B.gauss_jordan_solve(Matrix(v))[0]
        if any(not x.is_integer for x in sol):
            return False
    return True

def frac(s):
    return Fraction(s)

items = json.load(open("results/fixtures.json"))
fc = Curve(17, 2, 2)
bad = 0
for it in items:
    mu = it["mu2"]
    ok = {}
    P2 = fc.scalar_mul(mu, (3, 1))
    ok["pool_P0"] = [list(v) for v in up.projective_lift_pool((3, 1), 17, 8)] == it["pool_P0"]
    ok["pool_P2"] = [list(v) for v in up.projective_lift_pool(P2, 17, 8)] == it["pool_P2"]
    pts = [tuple(v) for v in it["lifts"]]
    lat = up.coefficient_lattice(pts, 17, 2, 2)
    ok["rows"] = [list(up.constraint_row(p)) for p in pts] == [[int(x) for x in r] for r in it["rows"]]
    rb = [[int(x) for x in r] for r in it["basis"]]
    pb = [list(r) for r in lat.reduced_basis_rows]
    ok["same_kernel_lattice"] = in_lattice(pb, rb) and in_lattice(rb, pb)
    diff = [int(a) - b for a, b in zip(it["particular"], lat.particular)]
    ok["same_affine_coset"] = in_lattice([diff], rb)
    u = [int(x) for x in it["ustar"]]
    curve, points = up.standard_curve_and_points(u, pts, 17, 2, 2)
    m = it["integral_model"]
    alpha = 1 + 17 * u[0]
    want = [frac(m[k]) / alpha**e for k, e in (("a1", 1), ("a2", 2), ("a3", 3), ("a4", 4), ("a6", 6))]
    ok["standard_model"] = [curve.a1, curve.a2, curve.a3, curve.a4, curve.a6] == want
    rp = []
    for key in ("P1", "P2"):
        x, y = m[key].strip("()").split(":")
        rp.append((frac(x) / alpha**2, frac(y) / alpha**3))
    ok["points"] = rp == [tuple(p) for p in points]
    good = all(ok.values())
    bad += not good
    print(f"mu2={mu} lifts={pts} {'MATCH' if good else 'MISMATCH'} {ok}")
print(f"items={len(items)} mismatches={bad}")
sys.exit(1 if bad else 0)
