#!/usr/bin/env python3
"""Arm R (PROTOCOL.md): the frame selector statistic on the real Curve25519 (p = 2^255 - 19, A = 486662).

Frames are translation frames in the Montgomery coordinate: u in [L, L + 2^t), L uniform in [0, p) from a
derived seed (a translation of x_W = u + A/3 is a translation of u, so this is the frame-search family).
For an on-curve u (chi(u^3 + A u^2 + u) = 1, u != 0) the folded coset label is read from x([r]P), computed by
an x-only Montgomery ladder (counted: one xDBL + one xADD per bit of r), against the rational 8-torsion
x-coordinates: O -> class 0, x = 0 -> class 4, x(T4) -> class 2 ({2,6}), x(T8) -> class 1, x(3T8) -> class 3.
T8 = [r]P0 for a derived reference point P0 of order 8r (labels are defined up to the unit 3, which J ignores).

Sub-arms:
  R1 (enumerated, small t): every u of the frame is tested; t and N from --r1 (e.g. 6:2000 8:1000 10:400).
  R1c (columns only, Legendre symbols, no labels): --r1c (e.g. 12:200 14:200 16:200).
  R2 (sampled, t = t*): --r2-frames K frames, --r2-n on-curve samples per frame, y uniform in [0, 2^t*).
Output: <out>.npz and a JSON summary on stdout. Each u is used once per frame (R2 samples with replacement
from 2^t* values; a repeat has probability ~ n^2 / 2^(t*+1), negligible).
"""
import argparse, json, os, sys, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import numpy as np
from l5common import derived_seed
import random

P = 2 ** 255 - 19
A = 486662
R = 2 ** 252 + 27742317777372353535851937790883648493
A24 = (A - 2) // 4
E_ = (P - 1) // 2

def chi(v):
    v %= P
    if v == 0: return 0
    return 1 if pow(v, E_, P) == 1 else -1

def ladder(k, u):
    """x([k]P) as (X, Z), RFC 7748 ladder; returns also the counted group operations (xDBL + xADD per bit)."""
    X1 = u; X2, Z2, X3, Z3 = 1, 0, u, 1
    nb = k.bit_length()
    for i in range(nb - 1, -1, -1):
        b = (k >> i) & 1
        if b: X2, X3, Z2, Z3 = X3, X2, Z3, Z2
        Ap = X2 + Z2; AA = Ap * Ap % P; Bm = X2 - Z2; BB = Bm * Bm % P; Ee = AA - BB
        C = X3 + Z3; D = X3 - Z3; DA = D * Ap % P; CB = C * Bm % P
        s = DA + CB; d = DA - CB
        X3 = s * s % P; Z3 = X1 * (d * d % P) % P
        X2 = AA * BB % P; Z2 = Ee * (AA + A24 * Ee) % P
        if b: X2, X3, Z2, Z3 = X3, X2, Z3, Z2
    return X2 % P, Z2 % P, 2 * nb

def affine(X, Z):
    return None if Z == 0 else X * pow(Z, P - 2, P) % P

class Labeller:
    def __init__(self):
        rng = random.Random(derived_seed('real', 'reference'))
        while True:
            u = rng.randrange(2, P)
            if chi(u * u * u + A * u * u + u) != 1: continue
            X, Z, _ = ladder(R, u); x8 = affine(X, Z)
            if x8 is None: continue
            X4, Z4, _ = ladder(2, x8); x4 = affine(X4, Z4)
            if x4 is None or x4 == 0: continue        # [r]P0 must have order 8
            X2_, Z2_, _ = ladder(2, x4)
            if affine(X2_, Z2_) != 0: continue
            break
        X, Z, _ = ladder(3, x8)
        self.table = {0: 4, x4: 2, x8: 1, affine(X, Z): 3}
        self.ref_u = u
        assert len(self.table) == 4
        self.ops = 0
    def label(self, u):
        X, Z, ops = ladder(R, u); self.ops += ops
        x = affine(X, Z)
        return 0 if x is None else self.table[x]

def frame_enumerated(L, t, lab, with_labels=True):
    m = np.zeros(5, dtype=np.int64); c = 0; leg = 0; parity_bad = 0
    for y in range(1 << t):
        u = (L + y) % P
        if u == 0: continue
        leg += 1
        if chi(u * u % P * u + A * u * u + u) != 1: continue
        c += 1
        if with_labels:
            cl = lab.label(u); m[cl] += 1; leg += 1
            parity_bad += int((1 if cl in (0, 2, 4) else -1) != chi(u))
    return c, m, leg, parity_bad

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--r1', nargs='*', default=[])
    ap.add_argument('--r1c', nargs='*', default=[])
    ap.add_argument('--tstar', type=int, default=128)
    ap.add_argument('--r2-frames', type=int, default=0)
    ap.add_argument('--r2-n', type=int, default=0)
    ap.add_argument('--out', required=True)
    a = ap.parse_args()
    lab = Labeller()
    res = {}; summ = dict(reference_u=str(lab.ref_u), r_bits=R.bit_length())
    t0 = time.time()
    for spec in a.r1:
        t, N = map(int, spec.split(':'))
        rng = random.Random(derived_seed('real', 'r1', t))
        rows = np.zeros((N, 6), dtype=np.int64); bad = 0; leg = 0; ops0 = lab.ops; ts = time.time()
        Ls = [rng.randrange(P) for _ in range(N)]
        for i, L in enumerate(Ls):
            c, m, lg, pb = frame_enumerated(L, t, lab)
            rows[i] = [c, *m]; bad += pb; leg += lg
        res[f'r1_t{t}'] = rows
        summ[f'r1_t{t}'] = dict(N=N, mean_c=float(rows[:, 0].mean()), parity_mismatches=int(bad),
                                group_ops=lab.ops - ops0, labels=int(rows[:, 0].sum()),
                                group_ops_per_label=(lab.ops - ops0) / max(1, int(rows[:, 0].sum())),
                                legendre=leg, seconds=round(time.time() - ts, 1))
        print(json.dumps(dict(event='r1', t=t, **summ[f'r1_t{t}'])), flush=True)
    for spec in a.r1c:
        t, N = map(int, spec.split(':'))
        rng = random.Random(derived_seed('real', 'r1c', t))
        cs = np.zeros(N, dtype=np.int64); ts = time.time()
        for i in range(N):
            L = rng.randrange(P)
            cs[i] = frame_enumerated(L, t, lab, with_labels=False)[0]
        res[f'r1c_t{t}'] = cs
        summ[f'r1c_t{t}'] = dict(N=N, mean_c=float(cs.mean()), var_c=float(cs.var(ddof=1)),
                                 legendre=N * (1 << t), seconds=round(time.time() - ts, 1))
        print(json.dumps(dict(event='r1c', t=t, **summ[f'r1c_t{t}'])), flush=True)
    if a.r2_frames:
        t = a.tstar; K = a.r2_frames; n = a.r2_n
        rng = random.Random(derived_seed('real', 'r2', t))
        counts = np.zeros((K, 2, 5), dtype=np.int64)   # two independent halves (A, B) for cross-validation
        trials = np.zeros((K, 2), dtype=np.int64); bad = 0; ops0 = lab.ops; ts = time.time()
        for f in range(K):
            L = rng.randrange(P); got = 0
            while got < n:
                u = (L + rng.randrange(1 << t)) % P
                trials[f, got % 2] += 1           # a trial belongs to the half its success would join
                if u == 0 or chi(u * u % P * u + A * u * u + u) != 1: continue
                cl = lab.label(u); counts[f, got % 2, cl] += 1; got += 1
                bad += int((1 if cl in (0, 2, 4) else -1) != chi(u))
        res['r2_counts'] = counts; res['r2_trials'] = trials
        summ['r2'] = dict(tstar=t, K=K, n=n, parity_mismatches=int(bad), group_ops=lab.ops - ops0,
                          group_ops_per_label=(lab.ops - ops0) / (K * n), trials=int(trials.sum()),
                          seconds=round(time.time() - ts, 1))
        print(json.dumps(dict(event='r2', **summ['r2'])), flush=True)
    np.savez_compressed(a.out, **res)
    summ['seconds_total'] = round(time.time() - t0, 1)
    print(json.dumps(dict(event='done', **summ)), flush=True)

if __name__ == '__main__':
    main()
