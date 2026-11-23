#!/usr/bin/env python3
"""Validator's independent check of the algebraic conic-net descent.

For plane quartics C: F = 0 over F_p (the four p = 1031 grid curves and the
four p = 2053 grid curves from raw/run.jsonl, plus random quartics at small
p, some forced through the coordinate points), build the net of conics
through an effective degree-3 divisor D on C and compare:

  scan   : rational points of C on all three conics (Ctx::decompose: Some iff
           exactly three distinct ones);
  alg    : a Python port of Ctx::decompose_alg (pivot elimination of v^2,
           resultant cubic g = b1 c2 - b2 c1, roots of g found here by brute
           force over F_p instead of Cantor-Zassenhaus, one division or the
           pivot quadratic per root, check on all three conics);
  brute  : every point of P^2(F_p) on all three conics (to see whether the
           point-index lookup after the conic check ever rejects a candidate,
           i.e. whether it acts as a hidden on-curve test).

Divisor types: A three distinct rational non-collinear points; B one rational
point plus a degree-2 point; C a degree-3 point; D 2Q1 + Q2 (tangent at Q1).
Usage: python3 -I decomp_check.py STUDY_DIR > decomp_check.json
"""
import json
import random
import sys

import numpy as np

STUDY = sys.argv[1]
rnd = random.Random(20261121 + 7)
M4 = [(a, b, 4 - a - b) for a in range(4, -1, -1) for b in range(4 - a, -1, -1)]
M2 = [(a, b, 2 - a - b) for a in range(2, -1, -1) for b in range(2 - a, -1, -1)]
assert M2 == [(2, 0, 0), (1, 1, 0), (1, 0, 1), (0, 2, 0), (0, 1, 1), (0, 0, 2)]
I2 = [[0, 1, 2], [1, 3, 4], [2, 4, 5]]


# ---------- F_{p^k} as coefficient lists modulo an irreducible polynomial ----------
class GF:
    def __init__(self, p, k):
        self.p, self.k = p, k
        if k == 1:
            self.mod = None
            return
        while True:  # random monic irreducible of degree k (k <= 3: no root => irreducible)
            m = [rnd.randrange(p) for _ in range(k)] + [1]
            if all(sum(c * pow(x, i, p) for i, c in enumerate(m)) % p for x in range(p)):
                self.mod = m
                return

    def mul(self, a, b):
        p, k = self.p, self.k
        if k == 1:
            return [a[0] * b[0] % p]
        c = [0] * (2 * k - 1)
        for i, x in enumerate(a):
            if x:
                for j, y in enumerate(b):
                    c[i + j] = (c[i + j] + x * y) % p
        for d in range(2 * k - 2, k - 1, -1):
            t = c[d]
            if t:
                for i in range(k):
                    c[d - k + i] = (c[d - k + i] - t * self.mod[i]) % p
        return c[:k]

    def add(self, a, b):
        return [(x + y) % self.p for x, y in zip(a, b)]

    def const(self, c):
        return [c % self.p] + [0] * (self.k - 1)

    def pw(self, a, e):
        r = self.const(1)
        for _ in range(e):
            r = self.mul(r, a)
        return r

    def elems(self):
        p, k = self.p, self.k
        for n in range(p ** k):
            v = []
            for _ in range(k):
                v.append(n % p)
                n //= p
            yield v


def F_eval(f, P, p):
    x, y, z = P
    return sum(c * pow(x, a, p) * pow(y, b, p) * pow(z, cc, p) for c, (a, b, cc) in zip(f, M4)) % p


def norm(P, p):
    for i in range(3):
        if P[i] % p:
            iv = pow(P[i], p - 2, p)
            return tuple((P[j] * iv) % p for j in range(3))
    return None


def curve_points(f, p):
    pts = []
    # [1, y, z] by numpy, then [0, 1, z], [0, 0, 1]
    y = np.arange(p, dtype=np.int64)
    Y, Z = np.meshgrid(y, y, indexing="ij")
    acc = np.zeros_like(Y)
    for c, (a, b, cc) in zip(f, M4):
        if c:
            t = np.full_like(Y, c)
            for _ in range(b):
                t = t * Y % p
            for _ in range(cc):
                t = t * Z % p
            acc = (acc + t) % p
    for yy, zz in zip(*np.nonzero(acc == 0)):
        pts.append((1, int(yy), int(zz)))
    for zz in range(p):
        if F_eval(f, (0, 1, zz), p) == 0:
            pts.append((0, 1, zz))
    if F_eval(f, (0, 0, 1), p) == 0:
        pts.append((0, 0, 1))
    return pts


def conic_vals(P, p):
    x, y, z = P
    return [x * x % p, x * y % p, x * z % p, y * y % p, y * z % p, z * z % p]


def kernel(rows, n, p):
    """Basis of {c in F_p^n : rows c = 0}."""
    m = [list(r) for r in rows]
    piv = []
    rk = 0
    for col in range(n):
        pr = next((i for i in range(rk, len(m)) if m[i][col] % p), None)
        if pr is None:
            continue
        m[rk], m[pr] = m[pr], m[rk]
        iv = pow(m[rk][col], p - 2, p)
        m[rk] = [v * iv % p for v in m[rk]]
        for i in range(len(m)):
            if i != rk and m[i][col] % p:
                t = m[i][col]
                m[i] = [(a - t * b) % p for a, b in zip(m[i], m[rk])]
        piv.append(col)
        rk += 1
    free = [c for c in range(n) if c not in piv]
    basis = []
    for fc in free:
        v = [0] * n
        v[fc] = 1
        for i, pc in enumerate(piv):
            v[pc] = (-m[i][fc]) % p
        basis.append(v)
    return basis


def rand_basis(B, p):
    while True:
        T = [[rnd.randrange(p) for _ in range(3)] for _ in range(3)]
        det = (T[0][0] * (T[1][1] * T[2][2] - T[1][2] * T[2][1])
               - T[0][1] * (T[1][0] * T[2][2] - T[1][2] * T[2][0])
               + T[0][2] * (T[1][0] * T[2][1] - T[1][1] * T[2][0])) % p
        if det:
            return [[sum(T[i][j] * B[j][s] for j in range(3)) % p for s in range(6)] for i in range(3)]


def on_conic(c, P, p):
    return sum(a * b for a, b in zip(c, conic_vals(P, p))) % p == 0


def scan(cs, pts, p):
    found = [P for P in pts if all(on_conic(c, P, p) for c in cs)]
    return found  # the Rust scan returns Some iff it meets 3 (it stops at the third)


def roots_bf(g, p):
    return [t for t in range(p) if (g[0] + g[1] * t + g[2] * t * t + g[3] * t * t * t) % p == 0]


def sqrt_bf(a, p):
    a %= p
    for s in range(p):
        if s * s % p == a:
            return s
    return None


STATS = {"pivot_quadratic_branch": 0, "root_at_infinity": 0, "vsel": {"0": 0, "1": 0, "2": 0, "none": 0},
         "candidates_on_conics_not_on_curve": 0}


def alg(cs, ptset, p, sqrt_fn):
    cands = []
    vsel = next((v for v in [1, 0, 2] if any(c[I2[v][v]] % p for c in cs)), None)
    STATS["vsel"]["none" if vsel is None else str(vsel)] += 1
    if vsel is None:
        cands = [(1, 0, 0), (0, 1, 0), (0, 0, 1)]
    else:
        v = vsel
        u, w = {0: (1, 2), 1: (0, 2), 2: (0, 1)}[v]
        i0 = next(k for k in range(3) if cs[k][I2[v][v]] % p)
        a0 = cs[i0][I2[v][v]]
        d = []
        for k in range(3):
            if k == i0:
                continue
            ak = cs[k][I2[v][v]]
            d.append([(a0 * cs[k][s] - ak * cs[i0][s]) % p for s in range(6)])
        lin = lambda c: [c[I2[v][w]], c[I2[u][v]]]
        quad = lambda c: [c[I2[w][w]], c[I2[u][w]], c[I2[u][u]]]
        b1, c1, b2, c2 = lin(d[0]), quad(d[0]), lin(d[1]), quad(d[1])
        g = [0] * 4
        for i in range(2):
            for j in range(3):
                g[i + j] = (g[i + j] + b1[i] * c2[j] - b2[i] * c1[j]) % p
        if not any(g):
            return "ERR"
        roots = [(t, 1) for t in roots_bf(g, p)]
        if g[3] == 0:
            roots.append((1, 0))
            STATS["root_at_infinity"] += 1
        ev_lin = lambda b, rt: b[1] % p if rt[1] == 0 else (b[0] + b[1] * rt[0]) % p
        ev_quad = lambda c, rt: c[2] % p if rt[1] == 0 else (c[0] + c[1] * rt[0] + c[2] * rt[0] ** 2) % p
        for rt in roots:
            bb1, bb2 = ev_lin(b1, rt), ev_lin(b2, rt)
            vs = []
            if bb1 or bb2:
                bb, cc = (bb1, ev_quad(c1, rt)) if bb1 else (bb2, ev_quad(c2, rt))
                vs.append((-cc * pow(bb, p - 2, p)) % p)
            else:
                STATS["pivot_quadratic_branch"] += 1
                b0, c0 = ev_lin(lin(cs[i0]), rt), ev_quad(quad(cs[i0]), rt)
                disc = (b0 * b0 - 4 * a0 * c0) % p
                sd = sqrt_fn(disc, p)
                if sd is not None:
                    ia = pow(2 * a0, p - 2, p)
                    vs.append(((-b0 + sd) * ia) % p)
                    if sd:
                        vs.append(((-b0 - sd) * ia) % p)
            for y in vs:
                pt = [0, 0, 0]
                pt[u], pt[w], pt[v] = rt[0], rt[1], y
                cands.append(tuple(pt))
    found = []
    for pt in cands:
        if not all(on_conic(c, pt, p) for c in cs):
            continue
        n = norm(pt, p)
        if n is None:
            continue
        if n in ptset:
            if n not in found:
                found.append(n)
        else:
            STATS["candidates_on_conics_not_on_curve"] += 1
    if len(found) == 3:
        return sorted(found)
    if len(found) <= 2:
        return None
    return "ERR"


def brute_P2(cs, p):
    """All points of P^2(F_p) on the three conics (numpy)."""
    out = []
    y = np.arange(p, dtype=np.int64)
    Y, Z = np.meshgrid(y, y, indexing="ij")
    X = np.ones_like(Y)
    mask = np.ones_like(Y, dtype=bool)
    mons = [X, Y, Z, Y * Y % p, Y * Z % p, Z * Z % p]
    for c in cs:
        acc = (c[0] * mons[0] + c[1] * mons[1] + c[2] * mons[2] + c[3] * mons[3]
               + c[4] * mons[4] + c[5] * mons[5]) % p
        mask &= acc == 0
    out += [(1, int(a), int(b)) for a, b in zip(*np.nonzero(mask))]
    for zz in range(p):
        if all(on_conic(c, (0, 1, zz), p) for c in cs):
            out.append((0, 1, zz))
    if all(on_conic(c, (0, 0, 1), p) for c in cs):
        out.append((0, 0, 1))
    return out


def collinear(P, Q, R, p):
    det = (P[0] * (Q[1] * R[2] - Q[2] * R[1]) - P[1] * (Q[0] * R[2] - Q[2] * R[0])
           + P[2] * (Q[0] * R[1] - Q[1] * R[0])) % p
    return det == 0


def grad(f, P, p):
    g = [0, 0, 0]
    x = list(P)
    for c, e in zip(f, M4):
        for i in range(3):
            if e[i]:
                t = c * e[i]
                for j in range(3):
                    t = t * pow(x[j], e[j] - (1 if j == i else 0), p)
                g[i] = (g[i] + t) % p
    return g


def ext_point(f, p, k):
    """A point (1, y, z) of C over F_{p^k}, y not in F_p (brute force in z)."""
    K = GF(p, k)
    while True:
        yv = [rnd.randrange(p) for _ in range(k)]
        if all(c == 0 for c in yv[1:]):
            continue
        ypw = [K.pw(yv, e) for e in range(5)]
        co = [K.const(0) for _ in range(5)]  # coefficients in z
        for c, (a, b, cc) in zip(f, M4):
            if c:
                co[cc] = K.add(co[cc], K.mul(K.const(c), ypw[b]))
        for zv in K.elems():
            acc = K.const(0)
            zp = K.const(1)
            for e in range(5):
                acc = K.add(acc, K.mul(co[e], zp))
                zp = K.mul(zp, zv)
            if not any(acc):
                return K, yv, zv


def ext_conditions(K, yv, zv):
    one = K.const(1)
    vals = [one, yv, zv, K.mul(yv, yv), K.mul(yv, zv), K.mul(zv, zv)]
    return [[vals[s][j] for s in range(6)] for j in range(K.k)]


def smooth_random_quartic(p, through_coord=False):
    while True:
        f = [rnd.randrange(p) for _ in range(15)]
        if through_coord:
            for i, e in enumerate(M4):
                if 4 in e:
                    f[i] = 0
        pts = curve_points(f, p)
        if len(pts) < 6:
            continue
        sing = any(all(v == 0 for v in grad(f, P, p)) for P in pts)
        if not sing:
            return f, pts


def run_case(f, pts, p, rows_of_D, kind, expect_pts, res):
    B = kernel(rows_of_D, 6, p)
    if len(B) != 3:
        res["skipped_not_net"] += 1
        return
    cs = rand_basis(B, p)
    ptset = set(pts)
    s = scan(cs, pts, p)
    s_out = sorted(s) if len(s) == 3 else ("FOURPLUS" if len(s) > 3 else None)
    a_out = alg(cs, ptset, p, sqrt_bf)
    br = brute_P2(cs, p)
    rec = res.setdefault(kind, {"n": 0, "agree": 0, "agree_after_fallback": 0, "decomposed": 0,
                                "brute_eq_curve_base": 0, "matches_expected": 0, "err": 0,
                                "err_with_common_component": 0, "mismatch_examples": []})
    rec["n"] += 1
    rec["agree"] += s_out == a_out
    # Rust: Err(()) -> decomp_fallbacks += 1 and the counted scan decides
    final = (sorted(s) if len(s) >= 3 and s_out != "FOURPLUS" else None) if a_out == "ERR" else a_out
    rec["agree_after_fallback"] += final == (s_out if s_out != "FOURPLUS" else sorted(s)[:3])
    rec["err_with_common_component"] += a_out == "ERR" and len(br) > 3
    rec["decomposed"] += isinstance(a_out, list)
    rec["err"] += a_out == "ERR"
    rec["brute_eq_curve_base"] += sorted(br) == sorted(s)
    rec["matches_expected"] += sorted(s) == sorted(expect_pts)
    if s_out != a_out and a_out != "ERR" and len(rec["mismatch_examples"]) < 3:
        rec["mismatch_examples"].append({"p": p, "cs": cs, "scan": s, "alg": a_out})


def main():
    res = {"skipped_not_net": 0}
    curves = []
    for l in open(f"{STUDY}/raw/run.jsonl"):
        r = json.loads(l)
        if r["kind"] == "instance" and r["family"] == "quartic" and r["p"] in (1031, 2053):
            curves.append((r["p"], r["f"], f"grid p={r['p']} seed={r['curve_seed']}", r["n_points"]))
    out_curves = []
    for p, f, name, npts in curves:
        pts = curve_points(f, p)
        out_curves.append({"curve": name, "n_points_recomputed": len(pts), "n_points_record": npts})
        for _ in range(60 if p == 1031 else 30):
            while True:
                Q = rnd.sample(pts, 3)
                if not collinear(*Q, p):
                    break
            run_case(f, pts, p, [conic_vals(P, p) for P in Q], "A_grid", Q, res)
        for _ in range(15 if p == 1031 else 8):
            Q1, Q2 = rnd.sample(pts, 2)
            g = grad(f, Q1, p)
            # a point T != Q1 on the tangent line g . X = 0
            T = next(T for T in [(g[1], (-g[0]) % p, 0), (g[2], 0, (-g[0]) % p), (0, g[2], (-g[1]) % p)]
                     if any(T) and norm(T, p) != norm(Q1, p))
            # derivative of the conic at Q1 along T
            x, y, z = Q1
            dr = [2 * x * T[0], y * T[0] + x * T[1], z * T[0] + x * T[2], 2 * y * T[1],
                  z * T[1] + y * T[2], 2 * z * T[2]]
            if collinear(Q1, T, Q2, p):
                continue
            run_case(f, pts, p, [conic_vals(Q1, p), [d % p for d in dr], conic_vals(Q2, p)],
                     "D_grid_tangent", [Q1, Q2], res)
    # small primes: random smooth quartics, types A, B, C; some through coordinate points
    for p in (31, 37, 101):
        for through in (False, True):
            f, pts = smooth_random_quartic(p, through)
            coord = [P for P in [(1, 0, 0), (0, 1, 0), (0, 0, 1)] if P in set(pts)]
            for _ in range(40):
                while True:
                    Q = rnd.sample(pts, 3)
                    if through and rnd.random() < 0.6:
                        k = rnd.randrange(1, len(coord) + 1)
                        Q = coord[:k] + [P for P in rnd.sample(pts, 3) if P not in coord][: 3 - k]
                        if len(set(Q)) < 3:
                            continue
                    if not collinear(*Q, p):
                        break
                run_case(f, pts, p, [conic_vals(P, p) for P in Q], "A_small" + ("_coord" if through else ""), Q, res)
            if p <= 37:
                for _ in range(8):
                    K, yv, zv = ext_point(f, p, 2)
                    Q = rnd.choice(pts)
                    run_case(f, pts, p, [conic_vals(Q, p)] + ext_conditions(K, yv, zv), "B_small", [Q], res)
                for _ in range(6):
                    K, yv, zv = ext_point(f, p, 3)
                    run_case(f, pts, p, ext_conditions(K, yv, zv), "C_small", [], res)
    res["stats"] = STATS
    res["curves"] = out_curves
    json.dump(res, sys.stdout, indent=1, default=str)
    print()


main()
