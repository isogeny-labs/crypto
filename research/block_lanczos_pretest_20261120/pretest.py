#!/usr/bin/env python3
"""Block Lanczos vs coordinate Wiedemann on exported genus_ladder cores (PROTOCOL.md).

Counts every mod-r operation in the unit of ic_control.rs (Nat {mul, add}).
usage: pretest.py EXPORT.jsonl [--line K] [--widths 1,2,4,8] [--rect] > out.json
All numbers [pcc:unverified].
"""
import argparse
import json
import random
import sys
import time


def pis(r):  # ic_boundary::pivot_inversion_steps
    return 2 * r.bit_length()


class Cnt:
    def __init__(self):
        self.mul = 0
        self.add = 0
        self.spmv = 0  # lane-SpMVs (one sparse product with one vector)
        self.passes = 0  # block passes (one sweep over the matrix per block)
        self.d_mul = 0  # dense-part split (mul+add), for the report
        self.d_add = 0

    def dense(self, m, a):
        self.mul += m
        self.add += a
        self.d_mul += m
        self.d_add += a

    def as_dict(self):
        return dict(mul=self.mul, add=self.add, total=self.mul + self.add, lane_spmv=self.spmv,
                    block_passes=self.passes, dense_mul=self.d_mul, dense_add=self.d_add)


class Mat:
    def __init__(self, rows, ncols, r):
        self.rows = [([c for c, _ in e], [v for _, v in e], rhs) for e, rhs in rows]
        self.ncols = ncols
        self.r = r
        self.nnz = sum(len(c) for c, _, _ in self.rows)
        self.nonunit_row = [sum(1 for v in vs if v != 1 and v != r - 1) for _, vs, _ in self.rows]
        self.nonunit = sum(self.nonunit_row)

    def mat_vec(self, x, cnt):  # ic_control::mat_vec
        r = self.r
        cnt.add += self.nnz
        cnt.mul += self.nonunit
        cnt.spmv += 1
        return [sum(v * x[c] for c, v in zip(cs, vs)) % r for cs, vs, _ in self.rows]

    def mat_t_vec(self, y, cnt):  # ic_control::mat_t_vec (rows with y_i = 0 skipped)
        r = self.r
        z = [0] * self.ncols
        cnt.spmv += 1
        for (cs, vs, _), yv, nu in zip(self.rows, y, self.nonunit_row):
            if yv == 0:
                continue
            cnt.add += len(cs)
            cnt.mul += nu
            for c, v in zip(cs, vs):
                z[c] += v * yv
        return [a % r for a in z]


def berlekamp_massey(s, r, cnt):  # ic_control::berlekamp_massey, same counts
    c, b = [1], [1]
    l, m, binv = 0, 1, 1
    for n in range(len(s)):
        d = s[n]
        for i in range(1, min(l, len(c) - 1) + 1):
            cnt.mul += 1
            cnt.add += 1
            d = (d + c[i] * s[n - i]) % r
        if d == 0:
            m += 1
            continue
        cnt.mul += 1
        coef = d * binv % r
        t = c[:]
        if len(c) < len(b) + m:
            c += [0] * (len(b) + m - len(c))
        for i, bv in enumerate(b):
            cnt.mul += 1
            cnt.add += 1
            c[i + m] = (c[i + m] - coef * bv) % r
        if 2 * l <= n:
            l = n + 1 - l
            b = t
            cnt.mul += pis(r)
            binv = pow(d, r - 2, r)
            m = 1
        else:
            m += 1
    c = c[: l + 1] + [0] * max(0, l + 1 - len(c))
    return c


def wiedemann_coord_square(sub, ct, r, cnt):
    """ic_control::wiedemann_coord(square = true) on the already-drawn subset."""
    n = sub.ncols
    length = 2 * n + 2
    v = [rhs % r for _, _, rhs in sub.rows]
    seq = []
    for _ in range(length):
        seq.append(v[ct])
        v = sub.mat_vec(v, cnt)
        cnt.passes += 1
    c = berlekamp_massey(seq, r, cnt)
    l = len(c) - 1
    if l == 0 or c[l] == 0:
        return None
    acc = 0
    for j in range(l):
        cnt.mul += 1
        cnt.add += 1
        acc = (acc + c[l - 1 - j] * seq[j]) % r
    cnt.mul += pis(r) + 1
    return acc * (-pow(c[l], r - 2, r)) % r


def small_inverse(h, b, r, cnt):
    """Gauss-Jordan inverse of a b x b matrix; None if singular. Charged per PROTOCOL 2."""
    a = [row[:] + [1 if i == j else 0 for j in range(b)] for i, row in enumerate(h)]
    for col in range(b):
        piv = next((i for i in range(col, b) if a[i][col] % r), None)
        if piv is None:
            return None
        a[col], a[piv] = a[piv], a[col]
        cnt.dense(pis(r), 0)
        inv = pow(a[col][col], r - 2, r)
        cnt.dense(2 * b, 0)
        a[col] = [x * inv % r for x in a[col]]
        for i in range(b):
            if i != col and a[i][col]:
                f = a[i][col]
                cnt.dense(2 * b, 2 * b)
                a[i] = [(x - f * y) % r for x, y in zip(a[i], a[col])]
    return [row[b:] for row in a]


def gram(X, Y, n, r, cnt):  # X^T Y, lanes are lists of length n
    b = len(X)
    cnt.dense(b * b * n, b * b * n)
    return [[sum(p * q for p, q in zip(X[i], Y[j])) % r for j in range(b)] for i in range(b)]


def small_prod(A, B, b, r, cnt):
    cnt.dense(b ** 3, b ** 3)
    return [[sum(A[i][k] * B[k][j] for k in range(b)) % r for j in range(b)] for i in range(b)]


def axpy(T, X, S, n, r, cnt, sign):  # T += sign * X S  (lanes: X[k] is a length-n list)
    b = len(X)
    cnt.dense(b * b * n, b * b * n)
    out = []
    for j in range(b):
        coeffs = [S[k][j] for k in range(b)]
        col = T[j][:]
        for k in range(b):
            ck = coeffs[k] * sign
            if ck:
                xk = X[k]
                col = [a + ck * x for a, x in zip(col, xk)]
        out.append([a % r for a in col])
    return out


def block_lanczos(A, rhs, ct, b, r, rng, cnt, pad=0):
    """koblitz_sparse_la::block_lanczos_solve recurrence on M = A^T D A (lanes as lists).
    pad > 0 appends `pad` identity unknowns (rows and columns, rhs 0) so b divides N.
    Only x[ct] (lane 0) is formed. Returns x[ct] or None (breakdown / no convergence)."""
    if pad:
        n0 = A.ncols
        rows = [(list(zip(cs, vs)), c) for cs, vs, c in A.rows]
        rows += [([(n0 + i, 1)], 0) for i in range(pad)]
        A = Mat(rows, n0 + pad, r)
        rhs = rhs + [0] * pad
    n, nr = A.ncols, len(A.rows)
    d = [rng.randrange(1, r) for _ in range(nr)]

    def apply_m(V):  # each lane: A v, D, A^T
        out = []
        for v in V:
            y = A.mat_vec(v, cnt)
            cnt.mul += nr
            y = [a * di % r for a, di in zip(y, d)]
            out.append(A.mat_t_vec(y, cnt))
        cnt.passes += 2
        return out

    def rhs_m(V):  # A^T D v
        out = []
        for v in V:
            cnt.mul += nr
            y = [a * di % r for a, di in zip(v, d)]
            out.append(A.mat_t_vec(y, cnt))
        cnt.passes += 1
        return out

    right = [[x % r for x in rhs]] + [[rng.randrange(r) for _ in range(nr)] for _ in range(b - 1)]
    residual = rhs_m(right)
    direction = [x[:] for x in residual]
    xct = 0
    limit = -(-n // b) + 2
    for it in range(limit):
        image = apply_m(direction)
        h = gram(direction, image, n, r, cnt)
        hinv = small_inverse(h, b, r, cnt)
        if hinv is None:
            return None, it
        proj = gram(direction, residual, n, r, cnt)
        alpha = small_prod(hinv, proj, b, r, cnt)
        # x[ct] (lane 0) += direction[ct, :] . alpha[:, 0]
        cnt.dense(b, b)
        xct = (xct + sum(direction[k][ct] * alpha[k][0] for k in range(b))) % r
        residual = axpy(residual, image, alpha, n, r, cnt, -1)
        if not any(residual[0]):  # convergence test: comparisons, not charged
            return xct, it + 1
        cross = gram(image, residual, n, r, cnt)
        beta = small_prod(hinv, cross, b, r, cnt)
        direction = axpy(residual, direction, beta, n, r, cnt, -1)
    return None, limit


def run_one(rec, widths, rect, seed):
    r, n, ct = rec["r"], rec["ncols"], rec["ct"]
    rows = [([tuple(e) for e in ent], c) for ent, c in rec["rows"]]
    full = Mat(rows, n, r)
    sub = Mat([rows[i] for i in rec["subset"]], n, r)
    planted = rec["t_secret"] % r
    to_log = (lambda t: t * rec["h_inv"] % r) if rec["unit_t"] else (lambda t: t)
    out = dict(p=rec["p"], g=rec["g"], r=r, ncols=n, core_rows=len(rows), nnz_full=full.nnz,
               nnz_square=sub.nnz, nonunit_square=sub.nonunit, planted=planted,
               binary_wied=dict(mul=rec["wied_mul"], add=rec["wied_add"],
                                total=rec["wied_mul"] + rec["wied_add"]),
               binary_coord_out=rec["coord_out"], arms={})
    t0 = time.time()
    cw = Cnt()
    xw = wiedemann_coord_square(sub, ct, r, cw)
    out["arms"]["W"] = dict(cnt=cw.as_dict(), x_ct=xw, ok=(xw is not None and to_log(xw) == planted),
                            matches_binary_counts=(cw.mul == rec["wied_mul"] and cw.add == rec["wied_add"]),
                            matches_binary_value=(xw == rec["coord_out"]), wall_s=round(time.time() - t0, 1))
    print(f"W: {out['arms']['W']}", file=sys.stderr, flush=True)
    rhs_sub = [c for _, _, c in sub.rows]
    rhs_full = [c for _, _, c in full.rows]
    variants = [("L%d" % b, sub, rhs_sub, b) for b in widths]
    if rect:
        variants += [("L%d_rect" % b, full, rhs_full, b) for b in widths if b == 1]
    for name, A, rhs, b in variants:
        t0 = time.time()
        pad = (-A.ncols) % b
        cl = Cnt()
        rng = random.Random(seed * 1000 + b + (500 if A is full else 0))
        x, its = block_lanczos(A, rhs, ct, b, r, rng, cl, pad=pad)
        ok = x is not None and to_log(x) == planted
        out["arms"][name] = dict(b=b, pad=pad, iterations=its, cnt=cl.as_dict(), x_ct=x, ok=ok,
                                 rho_spmv=cl.spmv / cw.spmv, rho_ops=(cl.mul + cl.add) / (cw.mul + cw.add),
                                 wall_s=round(time.time() - t0, 1))
        print(f"{name}: {out['arms'][name]}", file=sys.stderr, flush=True)
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("export")
    ap.add_argument("--line", type=int, default=0)
    ap.add_argument("--widths", default="1,2,4,8")
    ap.add_argument("--rect", action="store_true")
    ap.add_argument("--seed", type=int, default=20261120)
    a = ap.parse_args()
    lines = [l for l in open(a.export) if l.strip()]
    rec = json.loads(lines[a.line])
    res = run_one(rec, [int(x) for x in a.widths.split(",")], a.rect, a.seed)
    json.dump(res, sys.stdout, indent=1)
    print()


if __name__ == "__main__":
    main()
