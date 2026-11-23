#!/usr/bin/env python3
"""Assessor's independent re-implementation of arms A and B for L09 (stdlib only).

Written by the assessor from SPEC.md, without importing or calling the producer's code.
Curves are regenerated with the SPEC section 3 RNG; points, lines and fibres are found by
brute force over the curve's point list (no root finding), and the stopping proxy Y is
computed as rows - rank_LP by exact Gaussian elimination (no union-find).

Per seed it reports:
  T_A_uf, T_B_uf     line trials at first Y >= n_cols - 1 (the frozen proxy; must equal raw cell T)
  T_A_ex2            arm A, exact stop at dim >= n_cols - 2  (the reachable maximum for A, see ASSESSMENT)
  T_A_ex1            arm A, exact stop at dim >= n_cols - 1  (frozen exact target; expected unreachable)
  T_B_ex1            arm B, exact stop at dim >= n_cols - 1  (frozen exact target)
Usage: indep_check.py P BETA_IDX SEED_FROM SEEDS   -> JSON lines on stdout. [pcc:unverified]
"""
import json
import sys

M = (1 << 64) - 1
ELL = (1 << 61) - 1
BETAS = [(1, 8), (3, 16), (1, 4), (3, 8), (1, 2), (3, 4)]


def mix(z):
    z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & M
    z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & M
    return z ^ (z >> 31)


class SM:
    def __init__(s, x):
        s.s = x & M

    def nx(s):
        s.s = (s.s + 0x9E3779B97F4A7C15) & M
        return mix(s.s)

    def below(s, n):
        return (s.nx() * n) >> 64


def derive(p, s, b, t):
    return mix(((p * 0x9E3779B97F4A7C15) & M) ^ (((s + 1) * 0xC2B2AE3D27D4EB4F) & M)
               ^ (((b + 1) * 0x165667B19E3779F9) & M) ^ ((t * 0xD6E8FEB86659FD93) & M))


def gen(p, s):
    r = SM(derive(p, s, 255, 1))
    while True:
        a = [[r.below(p) for j in range(4)] for i in range(4)]
        if a[3][3] == 0:
            continue
        ax = [a[3][j] for j in range(4)]
        ay = [a[i][3] for i in range(4)]
        if any(sum(c[k] * pow(t, k, p) for k in range(4)) % p == 0 for c in (ax, ay) for t in range(p)):
            continue
        pts = []
        for x in range(p):
            cy = [sum(a[i][j] * pow(x, i, p) for i in range(4)) % p for j in range(4)]
            for y in range(p):
                if (cy[0] + y * (cy[1] + y * (cy[2] + y * cy[3]))) % p == 0:
                    pts.append((x, y))

        def fx(x, y):
            return sum(i * a[i][j] * pow(x, i - 1, p) * pow(y, j, p) for i in range(1, 4) for j in range(4)) % p

        def fy(x, y):
            return sum(j * a[i][j] * pow(x, i, p) * pow(y, j - 1, p) for i in range(4) for j in range(1, 4)) % p

        if any(fx(x, y) == 0 and fy(x, y) == 0 for x, y in pts):
            continue
        if (len(pts) - (p + 1)) ** 2 > 64 * p or len(pts) < 16:
            continue
        pts.sort()
        return a, pts


class Pairs:
    def __init__(s, seed, n):
        s.r = SM(seed); s.n = n; s.tot = n * (n - 1) // 2; s.used = set(); s.lst = None; s.pos = 0

    def next(s):
        n = s.n
        while True:
            if s.lst is None:
                if len(s.used) == s.tot:
                    return None
                if len(s.used) * 10 >= 9 * s.tot:
                    l = [i * n + j for i in range(n) for j in range(i + 1, n) if i * n + j not in s.used]
                    for i in range(len(l) - 1, 0, -1):
                        j = s.r.below(i + 1)
                        l[i], l[j] = l[j], l[i]
                    s.lst = l; s.pos = 0
            if s.lst is not None:
                if s.pos < len(s.lst):
                    k = s.lst[s.pos]; s.pos += 1; s.used.add(k)
                    return (k // n, k % n)
                return None
            i = s.r.below(n); j = s.r.below(n)
            if i == j:
                continue
            i, j = min(i, j), max(i, j)
            k = i * n + j
            if k in s.used:
                continue
            s.used.add(k)
            return (i, j)


class Ech:
    """Sparse incremental echelon form over F_ell; rows are dicts col->val."""

    def __init__(s):
        s.piv = {}; s.rank = 0

    def insert(s, row):
        row = {c: v % ELL for c, v in row.items() if v % ELL}
        while row:
            c = min(row)
            if c in s.piv:
                f = row[c]
                for cc, vv in s.piv[c].items():
                    nv = (row.get(cc, 0) - f * vv) % ELL
                    if nv:
                        row[cc] = nv
                    else:
                        row.pop(cc, None)
            else:
                iv = pow(row[c], ELL - 2, ELL)
                s.piv[c] = {cc: vv * iv % ELL for cc, vv in row.items()}
                s.rank += 1
                return True
        return False


def line_pts(a, p, pts, O, P1, P2):
    """(1,1)-form through O,P1,P2 by determinants; brute-force intersection. Returns pts or None."""
    rows = []
    for (x, y) in (O, P1, P2):
        rows.append([1, x, y, x * y % p])
    b = []
    for m in range(4):
        mat = [[r[c] for c in range(4) if c != m] for r in rows]
        (A_, B_, C_), (D_, E_, F_), (G_, H_, I_) = mat
        d = (A_ * (E_ * I_ - F_ * H_) - B_ * (D_ * I_ - F_ * G_) + C_ * (D_ * H_ - E_ * G_)) % p
        b.append((-d) % p if m % 2 else d)
    if (b[0] * b[3] - b[1] * b[2]) % p == 0:
        return None
    on = [q for q in pts if (b[0] + b[1] * q[0] + b[2] * q[1] + b[3] * q[0] * q[1]) % p == 0]
    assert O in on and P1 in on and P2 in on
    res = [q for q in on if q not in (O, P1, P2)]
    xs3 = {O[0], P1[0], P2[0]}
    if len(res) != 3 or any(q[0] in xs3 for q in res):
        return None
    assert len(on) == 6
    return [O, P1, P2] + res


def run(p, bi, s, arm, exact_target_off, pts, a, idx, O, fbpts, fbcol, pencil):
    """arm 'A' or 'B'. exact_target_off: None (proxy stop), or k (exact stop at dim >= n_cols - k)."""
    n_fb = len(fbpts)
    btype = arm == 'B'
    n_cols = n_fb + (3 if btype else 2)
    num, den = BETAS[bi]
    cap = min(64 * ((p * den + num - 1) // num), n_fb * (n_fb - 1) // 2)
    full = Ech(); lp = Ech(); rows = 0; seen = set()
    target_y = n_cols - 1
    target_ex = None if exact_target_off is None else n_cols - exact_target_off

    def add(kind, ps):
        nonlocal rows
        key = (kind, tuple(sorted(idx[q] for q in ps)))
        if key in seen:
            return False
        seen.add(key)
        sm = {}; lps = {}
        for q in ps:
            if q == O:
                sm[n_fb] = sm.get(n_fb, 0) + 1
            elif q in fbcol:
                sm[fbcol[q]] = sm.get(fbcol[q], 0) + 1
            else:
                lps[n_cols + idx[q]] = 1
        if kind == 0:
            sm[n_fb + 1] = -1
            if btype:
                sm[n_fb + 2] = -1
        elif kind == 1:
            sm[n_fb + 1] = -1
        else:
            sm[n_fb + 2] = -1
        full.insert({**sm, **lps}); lp.insert(dict(lps)); rows += 1
        return True

    def done():
        if target_ex is None:
            return rows - lp.rank >= target_y
        return full.rank - lp.rank >= target_ex

    if btype:
        for kind, ps in pencil:
            if add(kind, ps) and done():
                return 0, False
    st = Pairs(derive(p, s, bi, 11), n_fb)
    t = 0
    while True:
        if t >= cap:
            return t, True
        pr = st.next()
        if pr is None:
            return t, True
        t += 1
        r = line_pts(a, p, pts, O, fbpts[pr[0]], fbpts[pr[1]])
        if r is None:
            continue
        nlp = sum(1 for q in r if q != O and q not in fbcol)
        if nlp > 2:
            continue
        if add(0, r) and done():
            return t, False


def run_A_full(p, bi, s, pts, a, idx, O, fbpts, fbcol):
    """Arm A to pair exhaustion: proxy T, final exact dim, and first T reaching the final dim."""
    n_fb = len(fbpts); n_cols = n_fb + 2
    full = Ech(); lp = Ech(); rows = 0; seen = set()
    st = Pairs(derive(p, s, bi, 11), n_fb)
    t = 0; t_uf = -1; dim = 0; t_dim = 0
    while True:
        pr = st.next()
        if pr is None:
            break
        t += 1
        r = line_pts(a, p, pts, O, fbpts[pr[0]], fbpts[pr[1]])
        if r is None:
            continue
        if sum(1 for q in r if q != O and q not in fbcol) > 2:
            continue
        key = tuple(sorted(idx[q] for q in r))
        if key in seen:
            continue
        seen.add(key)
        sm = {n_fb + 1: -1}
        lps = {}
        for q in r:
            if q == O:
                sm[n_fb] = 1
            elif q in fbcol:
                sm[fbcol[q]] = 1
            else:
                lps[n_cols + idx[q]] = 1
        full.insert({**sm, **lps}); lp.insert(dict(lps)); rows += 1
        if t_uf < 0 and rows - lp.rank >= n_cols - 1:
            t_uf = t
        d = full.rank - lp.rank
        if d > dim:
            dim = d; t_dim = t
    return {"A_full_trials": t, "A_T_uf": t_uf, "A_dim_max": dim, "A_T_dim_max": t_dim, "A_ncols": n_cols}


def main():
    p, bi, s0, ns = map(int, sys.argv[1:5])
    for s in range(s0, s0 + ns):
        a, pts = gen(p, s)
        idx = {q: i for i, q in enumerate(pts)}
        n1 = len(pts)
        r = SM(derive(p, s, 255, 2))
        perm = list(range(n1))
        for i in range(n1 - 1, 0, -1):
            j = r.below(i + 1); perm[i], perm[j] = perm[j], perm[i]
        num, den = BETAS[bi]
        n_fb = max(4, (2 * (n1 - 1) * num + den) // (2 * den))
        O = pts[perm[0]]
        fbpts = [pts[k] for k in perm[1:n_fb + 1]]
        fbcol = {q: k for k, q in enumerate(fbpts)}
        small = set(fbpts) | {O}
        pencil = []
        for kind in (1, 2):
            for t in range(p):
                f = [q for q in pts if (q[0] if kind == 1 else q[1]) == t]
                if len(f) == 3:
                    nlp = sum(1 for q in f if q not in small)
                    if nlp <= 2:
                        pencil.append((kind, f))
        k_shared = sum(1 for q in fbpts if q[0] == O[0] or q[1] == O[1])
        out = {"k_shared": k_shared, "p": p, "beta_idx": bi, "seed": s, "N1": n1, "n_fb": n_fb, "pencil_rows": len(pencil)}
        out.update(run_A_full(p, bi, s, pts, a, idx, O, fbpts, fbcol))
        for name, arm, off in [("A_uf", 'A', None), ("B_uf", 'B', None), ("B_ex1", 'B', 1)]:
            t, c = run(p, bi, s, arm, off, pts, a, idx, O, fbpts, fbcol, pencil)
            out["T_" + name] = t
            out["cens_" + name] = c
        print(json.dumps(out), flush=True)


if __name__ == "__main__":
    main()
