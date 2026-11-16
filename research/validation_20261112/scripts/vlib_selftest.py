#!/usr/bin/env python3
"""Self-test of the validator's own arithmetic (vlib.py) before it is used to judge
anything: group laws on E and on J(C), [#J]D = 0 on random divisors of the v3 curves
(#J = #E1 #E2 for the bielliptic cover), Velu codomain point counts, and tamper
rejection on the v3 certificates.  Usage: vlib_selftest.py certs_v3.jsonl results_v3.json"""
import json, sys, os, random
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from vlib import *
random.seed(20261113)
certs = [json.loads(l) for l in open(sys.argv[1]) if l.strip()]
rows = {r["p"]: r for r in json.load(open(sys.argv[2]))["rows"]}
fails = []
def rpt(E):
    while True:
        x = random.randrange(E.p); y2 = E.rhs(x)
        if y2 == 0: return (x, 0)
        if pow(y2, (E.p - 1) // 2, E.p) == 1: return (x, tonelli(y2, E.p))
def rdiv(J):
    D = Jac2.ID
    for _ in range(2):
        while True:
            x = random.randrange(J.p); fx = peval(J.f, x, J.p)
            if fx and pow(fx, (J.p - 1) // 2, J.p) == 1: break
        D = J.add(D, ([(-x) % J.p, 1], [tonelli(fx, J.p)]))
    return D
for c in certs:
    p = c["p"]; row = rows[p]
    if c["arm"] == "rho_e1":
        E = EC(p, c["group"]["a2"], c["group"]["a4"], c["group"]["a6"]); n = E.count()
        for _ in range(20):
            P, Q, R = rpt(E), rpt(E), rpt(E)
            if E.add(E.add(P, Q), R) != E.add(P, E.add(Q, R)): fails.append((p, "E assoc"))
            if E.mul(n, P) is not None: fails.append((p, "E order"))
    if c["arm"] == "rho_e1prime":
        E2 = EC(p, c["group"]["a2"], c["group"]["a4"], c["group"]["a6"])
        if E2.count() != row["e1_order"]: fails.append((p, "isogenous curve count != #E1"))
    if c["arm"] == "ic_jc":
        J = Jac2(p, c["group"]["f"]); N = row["jac_order"]
        for _ in range(6):
            A, B, C = rdiv(J), rdiv(J), rdiv(J)
            if J.add(J.add(A, B), C) != J.add(A, J.add(B, C)): fails.append((p, "J assoc"))
            if J.add(A, J.neg(A)) != Jac2.ID and not J.is_id(J.add(A, J.neg(A))): fails.append((p, "J inverse"))
            if not J.is_id(J.mul(N, A)): fails.append((p, "[#J]D != 0"))
        G = (c["g"]["u"], c["g"]["v"]); T = (c["t"]["u"], c["t"]["v"]); k = c["claimed_k"]; r = c["r"]
        if J.norm(J.mul(k + 1, G)) == J.norm(T): fails.append((p, "tamper k+1 accepted"))
        if J.norm(J.mul(k, G)) != J.norm(T): fails.append((p, "true k rejected"))
    if c["group"]["kind"] == "ec":
        E = EC(p, c["group"]["a2"], c["group"]["a4"], c["group"]["a6"])
        if E.mul(c["claimed_k"] + 1, tuple(c["g"])) == tuple(c["t"]): fails.append((p, c["arm"], "EC tamper accepted"))
print(f"vlib self-test: {len(certs)} certificates' groups exercised; failures {len(fails)}")
for f in fails: print("  ", f)
sys.exit(1 if fails else 0)
