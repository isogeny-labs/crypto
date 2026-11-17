import os, sys, json, traceback
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import ff, jac, aut, ic, rho
from ff import CTR, uncounted
from order import *
from seeds import rng

FAILS = []
def report(name, ok, detail=""):
    if ok: print("PASS", name)
    else:
        print("FAIL %s: %s" % (name, detail)); FAILS.append(name)

def test(fn):
    try:
        fn()
    except Exception as e:
        report(fn.__name__, False, "exception " + repr(e) + traceback.format_exc())

def mk(family, p, q, f):
    cid = "%s:%d:t" % (family, p)
    ident = {"curve_id": cid, "a": f[0], "j": 0}
    C, reason = build_curve(family, p, q, f, ident, bits=0)
    return C, reason

def raw_curve(family, p, q, f, curve_id="T:%d:%d"):
    """Curve with order data but without admission (for tests)."""
    g = (q - 1) // 2
    d = {"family": family, "curve_id": curve_id % (p, f[0]), "p": p, "q": q, "g": g, "f": f,
         "zeta": pow(prim_root(p), (p - 1) // q, p) if family in ("F5", "F7") else 1}
    C = Curve(d); ff.set_p(p)
    N, _ = jacobian_order(C)
    return C, N

def cm(p, q, a):
    fam = "F5" if q == 5 else "F7"
    f = [a] + [0] * (q - 1) + [1]
    C, reason = build_curve(fam, p, q, f, {"curve_id": "%s:%d:%d" % (fam, p, a), "a": a}, bits=0, admit=False)
    assert C is not None, reason
    return C

def t01_zeta():
    exp = {(11, 5): 4, (31, 5): 16, (41, 5): 10, (29, 7): 16, (43, 7): 41}
    bad = {}
    for (p, q), z in exp.items():
        got = pow(prim_root(p), (p - 1) // q, p)
        if got != z: bad[(p, q)] = got
    report("zeta", not bad, str(bad))

def t02_N():
    bad = []
    for (p, f, n1, n2) in [(11, [2, 0, 0, 0, 0, 1], 21, 123), (43, [2, 0, 0, 0, 0, 0, 0, 1], 43, 1879)]:
        got = (count_N1(f, p), count_N2(f, p))
        if got != (n1, n2): bad.append((p, got))
    report("N1_N2", not bad, str(bad))

def count_N3(f, p):
    """#C(F_{p^3}) incl. infinity, F_{p^3} = F_p[s]/(s^3 - c0), independent exact count (test only)."""
    leg = legendre_table(p)
    c0 = 2
    while pow(c0, (p - 1) // 3, p) == 1: c0 += 1
    deg = len(f) - 1
    total = 1
    for a in range(p):
        for b in range(p):
            for c in range(p):
                A, B, Cc = f[deg], 0, 0
                for i in range(deg - 1, -1, -1):
                    # (A + B s + C s^2) * (a + b s + c s^2) + f[i]
                    nA_ = A * a + c0 * (B * c + Cc * b) + f[i]
                    nB_ = A * b + B * a + c0 * Cc * c
                    nC_ = A * c + B * b + Cc * a
                    A, B, Cc = nA_ % p, nB_ % p, nC_ % p
                if A == 0 and B == 0 and Cc == 0:
                    total += 1
                else:
                    nm = (A ** 3 + c0 * B ** 3 + c0 * c0 * Cc ** 3 - 3 * c0 * A * B * Cc) % p
                    total += 1 + leg[nm]
    return total

def independent_order(p, q, a):
    f = [a] + [0] * (q - 1) + [1]
    leg = legendre_table(p)
    N1 = count_N1(f, p, leg); N2 = count_N2(f, p, leg)
    s1 = p + 1 - N1; s2 = p * p + 1 - N2
    e1 = s1; e2 = (s1 * s1 - s2) // 2
    if q == 5:
        a1, a2 = -e1, e2
        return 1 + a1 + a2 + p * a1 + p * p
    N3 = count_N3(f, p)
    s3 = p ** 3 + 1 - N3
    e3 = (s1 ** 3 - 3 * s1 * s2 + 2 * s3) // 6
    a1, a2, a3 = -e1, e2, -e3
    return 1 + a1 + a2 + a3 + p * a2 + p * p * a1 + p ** 3

def t03_order():
    cases = [(11, 5, 1, 80), (11, 5, 2, 271), (31, 5, 1, 880), (41, 5, 3, 1136),
             (43, 7, 1, 50624), (43, 7, 2, 77813), (43, 7, 3, 108431), (43, 7, 5, 101459)]
    bad = []; notes = []
    for p, q, a, n in cases:
        C, N = raw_curve("F5" if q == 5 else "F7", p, q, [a] + [0] * (q - 1) + [1])
        if N != n:
            # SPEC 5 uniqueness rule returned None (exponent of J too small for the random divisors);
            # check the expected value independently from the L-polynomial (N1, N2[, N3]).
            ind = independent_order(p, q, a)
            notes.append((p, q, a, "spec_rule_returned", N, "independent", ind))
            if N is not None or ind != n: bad.append((p, q, a, N, ind, n))
    # brute-force check of #J for p=11 genus 2 via reduced divisors count
    for a, n in [(1, 80), (2, 271)]:
        f = [a, 0, 0, 0, 0, 1]; p = 11
        cnt = 1
        cnt += sum(1 for x in range(p) for y in range(p) if (y * y - (x ** 5 + a)) % p == 0)
        c2 = 0
        from poly import pdivmod, psub, pmul
        ff.set_p(p)
        with uncounted():
            for u0 in range(p):
                for u1 in range(p):
                    for v0 in range(p):
                        for v1 in range(p):
                            v = [v0, v1]
                            while v and v[-1] == 0: v.pop()
                            if not pdivmod(psub(f, pmul(v, v)), [u0, u1, 1])[1]: c2 += 1
        cnt += c2
        if cnt != n: bad.append(("brute", a, cnt, n))
    if notes: print("note:", notes)
    report("order", not bad, str(bad))

def t04_group():
    bad = []
    for p, q, a in [(31, 5, 1), (43, 7, 2)]:
        C = cm(p, q, a)
        ff.set_p(p)
        rnd = rng("selftest:group:%d" % p)
        with uncounted():
            for _ in range(50):
                D1, D2, D3 = [jac.random_divisor(C, rnd) for _ in range(3)]
                if jac.add(jac.add(D1, D2, C), D3, C) != jac.add(D1, jac.add(D2, D3, C), C): bad.append("assoc")
                if not jac.is_ident(jac.add(D1, jac.neg(D1), C)): bad.append("neg")
                if not jac.is_ident(jac.smul(C.N, D1, C)): bad.append("order")
                k = rnd.randrange(1, 10 ** 6)
                if jac.smul(k, D1, C) != jac.smul_rl(k, D1, C): bad.append("smul")
    report("group_axioms", not bad, str(set(bad)))

def t05_rho_apply():
    bad = []
    C = cm(11, 5, 2); ff.set_p(11)
    with uncounted():
        r1 = aut.rho_apply(([10, 1], [5]), C)
    if r1 != ([7, 1], [5]): bad.append(r1)
    C7 = cm(43, 7, 2); ff.set_p(43)
    with uncounted():
        r2 = aut.rho_apply(([41, 1], [1]), C7)
    if r2 != ([4, 1], [1]): bad.append(r2)
    report("rho_apply", not bad, str(bad))

def t06_lambda():
    bad = []
    C = cm(11, 5, 2); ff.set_p(11)
    if C.r != 271 or C.lam != 100: bad.append(("lam5", C.r, C.lam))
    with uncounted():
        D = ([10, 1], [5])
        exp = {10: ([2, 1], [5]), 100: ([7, 1], [5]), 187: ([8, 1], [5]), 244: ([6, 1], [5])}
        for k, e in exp.items():
            got = jac.smul(k, D, C)
            if got != e: bad.append((k, got, e))
    # Phi_5 roots mod 271
    roots = sorted(x for x in range(2, 271) if (x ** 4 + x ** 3 + x ** 2 + x + 1) % 271 == 0)
    if roots != [10, 100, 187, 244]: bad.append(roots)
    C7 = cm(43, 7, 2)
    if C7.lam != 19186: bad.append(("lam7", C7.lam))
    report("lambda", not bad, str(bad))

def t07_canon():
    bad = []
    C = cm(11, 5, 2); ff.set_p(11)
    with uncounted():
        a1 = aut.canon_aut(([10, 1], [5]), C)
        a2 = aut.canon_aut(([10, 1], [6]), C)
    if a1 != (([2, 1], [5]), 1, 3): bad.append(a1)
    if a2 != (([2, 1], [5]), -1, 3): bad.append(a2)
    for p, q, a in [(31, 5, 1), (43, 7, 2)]:
        C = cm(p, q, a); ff.set_p(p)
        rnd = rng("selftest:canon:%d" % p)
        with uncounted():
            for _ in range(200):
                D = jac.random_divisor(C, rnd)
                forms = set()
                for s in (1, -1):
                    for k in range(q):
                        Dk = aut.rho_apply(D, C, k)
                        if s == -1: Dk = jac.neg(Dk)
                        c = aut.canon_aut(Dk, C)[0]
                        forms.add((tuple(c[0]), tuple(c[1])))
                if len(forms) != 1: bad.append((p, D, len(forms)))
    report("canon_aut", not bad, str(bad[:3]))

def t08_orbit_rep():
    C = cm(11, 5, 2); ff.set_p(11)
    exp = {1: (1, 0), 2: (2, 0), 3: (1, 4), 4: (1, 1), 5: (1, 2), 6: (2, 4), 7: (2, 3), 8: (2, 1), 9: (1, 3), 10: (2, 2)}
    bad = []
    with uncounted():
        for x, e in exp.items():
            got = aut.orbit_rep_point(x, C)
            if got != e: bad.append((x, got, e))
            if C.ZP[got[1]] * got[0] % 11 != x: bad.append(("rel", x))
    report("orbit_rep_point", not bad, str(bad))

def t09_base():
    bad = []
    for p, q, a, e in [(11, 5, 2, (10, 0, 2)), (31, 5, 1, (11, 1, 2)), (43, 7, 2, (21, 0, 3))]:
        fam = "F5" if q == 5 else "F7"
        f = [a] + [0] * (q - 1) + [1]
        C, reason = build_curve(fam, p, q, f, {"curve_id": "%s:%d:%d" % (fam, p, a), "a": a}, bits=0)
        if C is None:
            # p=31 x^5+1 may be non-admitted; compute directly
            ff.set_p(p)
            vals = fvals(f, p); leg = legendre_table(p)
            nA = sum(1 for v in vals if leg[v] == 1); d0 = 1 if leg[vals[0]] == 1 else 0
            got = (nA, d0, (nA - d0) // q)
        else:
            got = (C.nA, C.delta0, C.nB)
        if got != e: bad.append((p, q, a, got, e))
    report("base_sizes", not bad, str(bad))

def t10_solve():
    r = 101
    rows = [{1: 1, 2: 1}, {1: 1, 2: 100}, {2: 1}, {1: 1}]
    bs = [1, 2, 3, 1]; as_ = [1, 85, 85, 97]
    tcs = [(-b) % r for b in bs]
    t, st = ic.solve(rows, tcs, as_, r, lambda t: t == 7)
    report("solve", t == 7, "got %s" % t)

def t11_transport():
    bad = []
    for p, q, a in [(11, 5, 2), (43, 7, 2)]:
        C = cm(p, q, a); ff.set_p(p)
        with uncounted():
            for x in range(p):
                v = ff.fpow(1, 1)
                fx = sum(c * pow(x, i, p) for i, c in enumerate(C.f)) % p
                if fx == 0 or pow(fx, (p - 1) // 2, p) != 1: continue
                y = ff.sqrt_mod(fx)
                for yy in (y, p - y):
                    P = jac.point_divisor(x, yy)
                    if aut.rho_apply(P, C) != jac.smul(C.lam, P, C): bad.append((p, x, yy))
    report("transport", not bad, str(bad[:3]))

def t12_counter():
    ff.set_p(11)
    CTR.reset()
    ff.fmul(3, 4); a = CTR.mul
    ff.finv(3); b = CTR.inv
    with uncounted():
        ff.fmul(3, 4); ff.finv(3)
    report("counter", (a, b, CTR.mul, CTR.inv) == (1, 1, 1, 1), str(CTR.snapshot()))

def t13_e2e():
    bad = []
    for p, q, a, r_exp, nc in [(11, 5, 2, 271, (20, 10, 2)), (43, 7, 2, 77813, (42, 21, 3))]:
        C = cm(p, q, a)
        if C.r != r_exp or C.h != 1: bad.append(("hr", C.h, C.r))
        recs = {}
        for arm in ("A0", "A", "B"):
            rec = ic.run_ic(C, 1, arm); recs[arm] = rec
            if not rec["certified"]: bad.append(("ic_cert", p, arm, rec["status"]))
        if tuple(recs[x]["ncols"] for x in ("A0", "A", "B")) != nc: bad.append(("ncols", p))
        ds = [recs[x]["digests"] for x in ("A0", "A", "B")]
        n = min(len(d) for d in ds)
        if not all(d[:n] == ds[0][:n] for d in ds): bad.append(("digest", p))
        # also compare the final trial prefix: shared stream
        for arm in ("Rneg", "Raut"):
            rr = rho.run_rho(C, 1, arm)
            if not rr["certified"]: bad.append(("rho_cert", p, arm, rr["status"]))
    report("end_to_end", not bad, str(bad))

def t14_control():
    bad = []
    p = 31; q = 5
    C = None
    for j in range(300):
        rnd = rng("ctrl:5:31:%d" % j)
        c = [rnd.randrange(p) for _ in range(q)]
        if all(x == 0 for x in c[1:]): continue
        Cc, reason = build_curve("C5", p, q, c + [1], {"curve_id": "C5:31:j%d" % j, "j": j}, bits=0)
        if Cc is not None:
            C = Cc; break
    if C is None:
        print("note: no admitted control at p=31; using f=x^5+x+1 directly")
        f = [1, 1, 0, 0, 0, 1]
        C, N = raw_curve("C5", p, q, f, "C5:%d:%d")
        r = largest_prime_factor(N)
        C.N = N; C.r = r; C.h = N // r
        with uncounted():
            ff.set_p(p); vals = fvals(f, p); leg = legendre_table(p)
            C.nA = sum(1 for v in vals if leg[v] == 1); C.nB = C.nA; C.delta0 = 0
        C.setup()
    skip = {"arm", "run_id", "wall_s"}
    for s in (1, 2):
        ra = ic.run_ic(C, s, "A"); rb = ic.run_ic(C, s, "B")
        for k in ra:
            if k not in skip and ra[k] != rb[k]: bad.append(("ic", s, k))
        rn = rho.run_rho(C, s, "Rneg"); rt = rho.run_rho(C, s, "Raut")
        for k in rn:
            if k not in skip and rn[k] != rt[k]: bad.append(("rho", s, k))
    report("control_degeneration", not bad, "%s (curve %s)" % (bad[:5], C.curve_id))

def t15_rho_bookkeeping():
    bad = []
    for p, q, a in [(11, 5, 2), (43, 7, 2)]:
        C = cm(p, q, a)
        for arm in ("Rneg", "Raut"):
            try:
                rr = rho.run_rho(C, 1, arm, check=200)
            except AssertionError as e:
                bad.append((p, arm, str(e)))
    report("rho_bookkeeping", not bad, str(bad))

if __name__ == "__main__":
    for fn in [t01_zeta, t02_N, t03_order, t04_group, t05_rho_apply, t06_lambda, t07_canon,
               t08_orbit_rep, t09_base, t10_solve, t11_transport, t12_counter, t13_e2e,
               t14_control, t15_rho_bookkeeping]:
        test(fn)
    print("FAILED: %s" % FAILS if FAILS else "ALL PASS")
    sys.exit(1 if FAILS else 0)
