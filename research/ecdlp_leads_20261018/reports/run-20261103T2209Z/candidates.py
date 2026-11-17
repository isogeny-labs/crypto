#!/usr/bin/env python3
"""Derives candidates.csv from g5_sourced.jsonl: every genus-5 L-polynomial with n | P(lambda),
ord lambda <= 4, for n = 409, 571, with L(1), N_1, N_3 and #J(F_8) = prod (1 - alpha^3) computed
exactly from L. Usage: python3 candidates.py g5_sourced.jsonl > candidates.csv"""
import json, sys
from fractions import Fraction

def power_sums(a, kmax):
    # L = sum a_j X^j with a_0 = 1; p_k = -k a_k - sum_{i<k} p_i a_{k-i}
    g = lambda j: a[j] if j < len(a) else 0
    p = [0] * (kmax + 1)
    for k in range(1, kmax + 1):
        p[k] = -k * g(k) - sum(p[i] * g(k - i) for i in range(1, k))
    return p

def j_over_f8(a):
    # e_k of alpha^3 from power sums p_{3k}, then L_8(1) = sum (-1)^k e_k
    d = len(a) - 1
    p = power_sums(a, 3 * d)
    e = [Fraction(1)] + [Fraction(0)] * d
    for k in range(1, d + 1):
        e[k] = sum((-1) ** (i - 1) * e[k - i] * p[3 * i] for i in range(1, k + 1)) / k
    assert all(x.denominator == 1 for x in e)
    return sum((-1) ** k * int(e[k]) for k in range(d + 1))

print("n,ord_lambda,family,curves,L1,N1,N3,J_F8,J_F8_mod_n,L")
seen = {}
for line in open(sys.argv[1]):
    d = json.loads(line)
    if d["kind"] != "candidate":
        continue
    fam = d["family"].replace("g5_", "").replace("_sourced", "")
    lp = d["L"]
    p = power_sums(lp, 3)
    n1, n3 = 3 + lp[1] * -1 * -1 * 0 + 0, 0
    n1 = 2 + 1 - p[1]
    n3 = 8 + 1 - p[3]
    j8 = j_over_f8(lp)
    print(f'{d["n"]},{d["ord_lambda"]},{fam},{d["models"]},{d["L1"]},{n1},{n3},{j8},{j8 % d["n"]},"{lp}"')
