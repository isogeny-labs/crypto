"""Numerical (complex, double precision) theta machinery for the affine (2,2,2) codomain
B = J/L_S, tau_B = (tau + S)/2.

Provides: period matrix of the real even model (reuses the frozen
g3_transfer_gates theta_arm.period_matrix, imported read-only), theta constants and
odd-theta gradients at z = 0, the Aronhold-system frame and Riemann's quartic, and a
numerical bitangency test.
"""
import importlib.util
import itertools
import os
import random

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
_ARM = os.path.join(HERE, "..", "g3_transfer_gates_20261120", "theta_arm.py")
_spec = importlib.util.spec_from_file_location("theta_arm", _ARM)
theta_arm = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(theta_arm)

BITS3 = list(itertools.product((0, 1), repeat=3))
CHARS = [(a, b) for a in BITS3 for b in BITS3]


def q(c):
    a, b = c
    return sum(x * y for x, y in zip(a, b)) % 2


EVEN = [c for c in CHARS if q(c) == 0]
ODD = [c for c in CHARS if q(c) == 1]


def cadd(*cs):
    a = tuple(sum(c[0][i] for c in cs) % 2 for i in range(3))
    b = tuple(sum(c[1][i] for c in cs) % 2 for i in range(3))
    return (a, b)


def period_matrix_seed(seed):
    from mpmath import mpf
    rng = random.Random(seed)
    es = [mpf(3 * i) + mpf(rng.uniform(0.2, 1.8)) for i in range(8)]
    asym, posdef, signs, tau = theta_arm.period_matrix(es, 5)
    T = np.array([[complex((tau[i, j] + tau[j, i]) / 2) for j in range(3)] for i in range(3)])
    return [float(e) for e in es], T, float(asym)


def theta_all(T, R=None):
    """theta[a;b](0,T) for all 64 chars and grad_z theta[a;b](0,T) (np arrays)."""
    im = T.imag
    lam = np.linalg.eigvalsh(im).min()
    if R is None:
        R = int(np.ceil(np.sqrt(40.0 / max(lam, 1e-3)))) + 2
    rng_ = np.arange(-R, R + 1)
    N = np.array(list(itertools.product(rng_, repeat=3)), dtype=float)
    val, grad = {}, {}
    for a in BITS3:
        V = N + np.array(a) / 2.0
        Q = np.einsum("ni,ij,nj->n", V, T, V)
        E = np.exp(1j * np.pi * Q)
        for b in BITS3:
            ph = np.exp(1j * np.pi * (V @ np.array(b, dtype=float)))
            w = E * ph
            val[(a, b)] = w.sum()
            grad[(a, b)] = (2j * np.pi) * (V * w[:, None]).sum(axis=0)
    return val, grad


def is_aronhold(ms):
    for t in itertools.combinations(ms, 3):
        if q(cadd(*t)) != 0:
            return False
    return True


def find_aronhold(rng=None):
    """Greedy/backtracking search for an ordered Aronhold set of 7 odd chars."""
    order = list(ODD)
    if rng is not None:
        rng.shuffle(order)

    def bt(cur, start):
        if len(cur) == 7:
            return cur
        for k in range(start, len(order)):
            m = order[k]
            ok = all(q(cadd(x, y, m)) == 0 for x, y in itertools.combinations(cur, 2))
            if ok:
                r = bt(cur + [m], k + 1)
                if r:
                    return r
        return None
    return bt([], 0)


def all_aronhold_sets():
    out = []

    def bt(cur, start):
        if len(cur) == 7:
            out.append(tuple(cur))
            return
        for k in range(start, len(ODD)):
            m = ODD[k]
            if all(q(cadd(x, y, m)) == 0 for x, y in itertools.combinations(cur, 2)):
                bt(cur + [m], k + 1)
    bt([], 0)
    return out


def weber_frame(lines7):
    """lines7: 7 vectors (covectors). Returns H (3x3) and A (3x3) with
    beta_i = H[i] for i<3, beta_4 ~ sum H rows, beta_{5+j} ~ A[j] . H."""
    first = np.array(lines7[:3])
    co = np.linalg.solve(first.T, lines7[3])  # lines7[3] = co @ first
    H = np.diag(co) @ first
    Hi = np.linalg.inv(H)
    A = np.array([l @ Hi for l in lines7[4:]])
    A = A / A[:, :1]
    return H, A


def riemann_xi(A):
    """Riemann's linear forms xi (rows, in the frame coordinates X) and k_j for the
    Aronhold frame u1..u4 = X1, X2, X3, X1+X2+X3 and u_{5+j} = A[j] . X.
    Equations: xi1+xi2+xi3 + u4 = 0, sum_i xi_i / A[j,i] + k_j u_{5+j} = 0."""
    M = np.array([[1 / A[j, i] for j in range(3)] for i in range(3)])  # M[i][j] = 1/A[j,i]
    U = np.linalg.solve(M, -np.ones(3))
    k = np.linalg.solve(A.T @ np.diag(U), -np.ones(3))
    Left = np.vstack([np.ones(3)] + [1 / A[j] for j in range(3)])
    Right = -np.vstack([np.ones(3)] + [k[j] * A[j] for j in range(3)])
    Xi = np.linalg.solve(Left[:3], Right[:3])
    res = np.abs(Left @ Xi - Right).max()
    return Xi, k, res


def quartic_eval_coeffs(Xi):
    """Coefficients dict {(i,j,k): c} of F = (X1 xi1 + X2 xi2 - X3 xi3)^2 - 4 X1 X2 xi1 xi2,
    xi_r = sum_s Xi[r,s] X_s."""
    from collections import defaultdict

    def lin(v):
        return {tuple(1 if t == s else 0 for t in range(3)): v[s] for s in range(3)}

    def mul(P, Q):
        R = defaultdict(complex)
        for e1, c1 in P.items():
            for e2, c2 in Q.items():
                R[tuple(x + y for x, y in zip(e1, e2))] += c1 * c2
        return dict(R)

    def add(P, Q, s=1):
        R = defaultdict(complex)
        for e, c in P.items():
            R[e] += c
        for e, c in Q.items():
            R[e] += s * c
        return dict(R)
    X = [lin([1 if t == s else 0 for t in range(3)]) for s in range(3)]
    xi = [lin(Xi[r]) for r in range(3)]
    G = add(add(mul(X[0], xi[0]), mul(X[1], xi[1])), mul(X[2], xi[2]), -1)
    F = add(mul(G, G), mul(mul(mul(X[0], X[1]), xi[0]), xi[1]), -4)
    return F


def restrict_to_line(F, l):
    """F (coeff dict) restricted to the line l.X = 0, as univariate poly coefficients."""
    l = np.asarray(l, dtype=complex)
    piv = int(np.argmax(np.abs(l)))
    fr = [i for i in range(3) if i != piv]
    # X[fr0] = t, X[fr1] = 1, X[piv] = -(l[fr0] t + l[fr1]) / l[piv]
    comps = [None] * 3
    comps[fr[0]] = np.array([0, 1], dtype=complex)  # poly in t, low->high
    comps[fr[1]] = np.array([1, 0], dtype=complex)
    comps[piv] = np.array([-l[fr[1]] / l[piv], -l[fr[0]] / l[piv]])
    out = np.zeros(5, dtype=complex)
    for e, c in F.items():
        p = np.array([1], dtype=complex)
        for s in range(3):
            for _ in range(e[s]):
                p = np.convolve(p, comps[s])
        out[:len(p)] += c * p
    return out


def bitangent_defect(F, l):
    """0 if F|_l is a square: relative distance of the root multiset to a doubled set."""
    co = restrict_to_line(F, l)
    co = co / np.abs(co).max()
    r = np.roots(co[::-1])
    if len(r) < 4:
        return 1.0
    best = 1e9
    for perm in [(0, 1, 2, 3), (0, 2, 1, 3), (0, 3, 1, 2)]:
        d = abs(r[perm[0]] - r[perm[1]]) + abs(r[perm[2]] - r[perm[3]])
        best = min(best, d / (1 + np.abs(r).max()))
    return best
