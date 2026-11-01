#!/usr/bin/env python3
"""Independent [d]G = Q verifier for the Round-5 boundary ledger (Table B).

Pure Python, stdlib only. Shares no code with the repository. For every prime and
random-binary instance in a ledger it checks: G is on the curve, [r]G = O, and for
every recorded run (IC variant repetition, rho run) that [recovered]G equals [t]G, t the
planted scalar of that run's target. Negative control: recovered + 1 must be rejected.

usage: ec_verify.py LEDGER.json OUT.json
"""
import json
import sys

def parse(v):
    return int(v, 16) if isinstance(v, str) and v.startswith("0x") else int(v)

class Fp:
    def __init__(self, p, a, b):
        self.p, self.a, self.b = p, a % p, b % p
    def on(self, P):
        if P is None: return True
        x, y = P
        return (y * y - (x ** 3 + self.a * x + self.b)) % self.p == 0
    def neg(self, P):
        return None if P is None else (P[0], (-P[1]) % self.p)
    def add(self, P, Q):
        p = self.p
        if P is None: return Q
        if Q is None: return P
        if P[0] == Q[0]:
            if (P[1] + Q[1]) % p == 0: return None
            lam = (3 * P[0] * P[0] + self.a) * pow(2 * P[1], -1, p) % p
        else:
            lam = (Q[1] - P[1]) * pow(Q[0] - P[0], -1, p) % p
        x = (lam * lam - P[0] - Q[0]) % p
        return (x, (lam * (P[0] - x) - P[1]) % p)

class F2m:
    def __init__(self, n, low, a, b):
        self.n = n
        self.mod = (1 << n) | sum(1 << t for t in low)
        self.a, self.b = a, b
    def mul(self, x, y):
        r = 0
        while y:
            if y & 1: r ^= x
            y >>= 1
            x <<= 1
            if (x >> self.n) & 1: x ^= self.mod
        return r
    def inv(self, x):
        # extended Euclid over GF(2)[x]
        u, v, g1, g2 = x, self.mod, 1, 0
        while u != 1:
            j = u.bit_length() - v.bit_length()
            if j < 0:
                u, v, g1, g2, j = v, u, g2, g1, -j
            u ^= v << j
            g1 ^= g2 << j
        return g1
    def on(self, P):
        if P is None: return True
        x, y = P
        m = self.mul
        lhs = m(y, y) ^ m(x, y)
        rhs = m(m(x, x), x) ^ m(self.a, m(x, x)) ^ self.b
        return lhs == rhs
    def neg(self, P):
        return None if P is None else (P[0], P[0] ^ P[1])
    def add(self, P, Q):
        m = self.mul
        if P is None: return Q
        if Q is None: return P
        if P[0] == Q[0]:
            if P[1] ^ Q[1] == P[0]: return None
            lam = P[0] ^ m(P[1], self.inv(P[0]))
            x = m(lam, lam) ^ lam ^ self.a
        else:
            lam = m(P[1] ^ Q[1], self.inv(P[0] ^ Q[0]))
            x = m(lam, lam) ^ lam ^ P[0] ^ Q[0] ^ self.a
        y = m(P[0] ^ x, lam) ^ x ^ P[1]
        return (x, y)

def smul(C, k, P):
    R = None
    while k:
        if k & 1: R = C.add(R, P)
        P = C.add(P, P)
        k >>= 1
    return R

def curve_of(inst):
    c = inst["curve"]
    g = c["generator"]
    if inst["regime"] == "prime":
        C = Fp(c["p"], parse(c["a"]), parse(c["b"]))
        G = (parse(g["x"]), parse(g["y"]))
    else:
        f = c["field"]
        C = F2m(f["degree"], f["polynomial_low_terms"], parse(c["a"]), parse(c["b"]))
        G = (parse(g["x"]), parse(g["y"]))
    return C, G

def main(ledger_path, out_path):
    led = json.load(open(ledger_path))
    out = {"ledger": ledger_path, "instances": [], "runs": 0, "scalars_verified": True,
           "negative_control_rejected": 0, "negative_control_total": 0}
    for inst in led["ledger"]["instances"]:
        if inst["regime"] not in ("prime", "char2"):
            continue
        C, G = curve_of(inst)
        r = int(inst["r"])
        rec = {"curve": inst["curve"]["name"], "regime": inst["regime"], "r": r,
               "G_on_curve": C.on(G), "rG_is_infinity": smul(C, r, G) is None,
               "targets": inst["targets"], "runs": []}
        Q = {t: smul(C, t % r, G) for t in inst["targets"]}
        rec["Q_on_curve"] = all(C.on(q) for q in Q.values())
        nv = len(inst["variants"]) // len(inst["targets"])
        nr = len(inst["rho"]) // len(inst["targets"])
        plans = [("ic", i, v["recovered"], inst["targets"][i // nv], v.get("verified"))
                 for i, v in enumerate(inst["variants"])]
        plans += [("rho", i, v["recovered"], inst["targets"][i // nr], v.get("verified"))
                  for i, v in enumerate(inst["rho"])]
        ok_all = True
        cache = {}
        for kind, idx, d, t, flag in plans:
            if (d, t) not in cache:
                good = smul(C, d % r, G) == Q[t]
                bad = smul(C, (d + 1) % r, G) == Q[t]   # must be False
                cache[(d, t)] = (good, bad)
            good, bad = cache[(d, t)]
            out["negative_control_total"] += 1
            out["negative_control_rejected"] += 0 if bad else 1
            ok_all &= good and flag is True
            out["runs"] += 1
        rec["n_runs"] = len(plans)
        rec["distinct_pairs_checked"] = len(cache)
        rec["all_scalars_verified"] = bool(ok_all and rec["G_on_curve"] and rec["rG_is_infinity"]
                                           and rec["Q_on_curve"])
        out["scalars_verified"] &= rec["all_scalars_verified"]
        out["instances"].append(rec)
    json.dump(out, open(out_path, "w"), indent=1)
    print(f"instances={len(out['instances'])} runs={out['runs']} scalars_verified={out['scalars_verified']} "
          f"negctl={out['negative_control_rejected']}/{out['negative_control_total']}")

if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
