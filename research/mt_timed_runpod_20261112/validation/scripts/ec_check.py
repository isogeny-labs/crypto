#!/usr/bin/env python3
"""VALIDATOR certificate check with the validator's own elliptic-curve code (shares no code with the harness):
 1. the 12 instances W{24,28,32}-{0..3}: p prime, n prime, G on the curve, [n]G = O, cofactor 1 by Hasse
    (2n > p + 1 + 2 sqrt p), nonsingular, bit lengths, sha256;
 2. every dumped run (dump builds, nonce sample, both builds): the planted list is identical across runs of one
    configuration (and across builds), every Q_t is on the curve, 0 <= log < n, recovered log == planted log, and
    [log]G == Q_t (computed once per distinct (curve, log, Q) and compared for every run that claims it).
Short Weierstrass y^2 = x^3 + a x + b over F_p, affine, Python modular inverse, right-to-left double-and-add;
deterministic Miller-Rabin with the first 13 prime bases (valid far beyond 2^64).
usage: ec_check.py <study dir> <out.json> <replay_dump json>..."""
import collections
import hashlib
import json
import math
import os
import sys

STUDY, OUT = sys.argv[1:3]
RDS = sys.argv[3:]
INST = os.path.join(STUDY, "../mt_fieldops_v5_20261112/instances")


def is_prime(n):
    if n < 2:
        return False
    B = (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41)
    for q in B:
        if n % q == 0:
            return n == q
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2
        s += 1
    for a in B:
        x = pow(a, d, n)
        if x in (1, n - 1):
            continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1:
                break
        else:
            return False
    return True


class E:
    def __init__(s, p, a, b):
        s.p, s.a, s.b = p, a % p, b % p

    def on(s, P):
        return P is None or (P[1] * P[1] - P[0] ** 3 - s.a * P[0] - s.b) % s.p == 0

    def add(s, P, Q):
        if P is None:
            return Q
        if Q is None:
            return P
        p = s.p
        if P[0] == Q[0]:
            if (P[1] + Q[1]) % p == 0:
                return None
            l = (3 * P[0] * P[0] + s.a) * pow(2 * P[1], -1, p) % p
        else:
            l = (Q[1] - P[1]) * pow(Q[0] - P[0], -1, p) % p
        x = (l * l - P[0] - Q[0]) % p
        return (x, (l * (P[0] - x) - P[1]) % p)

    def mul(s, k, P):
        R = None
        while k:
            if k & 1:
                R = s.add(R, P)
            P = s.add(P, P)
            k >>= 1
        return R


def load(name):
    path = os.path.join(INST, name + ".json")
    d = json.load(open(path))
    v = lambda x: int(str(x).strip('"'))
    p = v(d["field"]["modulus"])
    return path, E(p, v(d["a"]), v(d["b"])), (v(d["generator"]["x"]), v(d["generator"]["y"])), v(d["subgroup_order"])


inst = {}
curves = {}
for pb in (24, 28, 32):
    for i in range(4):
        name = f"W{pb}-{i}"
        path, C, G, n = load(name)
        curves[name] = (C, G, n)
        p = C.p
        inst[name] = dict(sha256=hashlib.sha256(open(path, "rb").read()).hexdigest(), p=p, n=n, p_bits=p.bit_length(), n_bits=n.bit_length(),
                          p_prime=is_prime(p), n_prime=is_prime(n), G_on=C.on(G), nG_is_O=C.mul(n, G) is None,
                          cofactor_1=2 * n > p + 1 + 2 * math.isqrt(p) + 2, nonsingular=(4 * C.a ** 3 + 27 * C.b ** 2) % p != 0)
inst_ok = all(all(v[k] for k in ("p_prime", "n_prime", "G_on", "nG_is_O", "cofactor_1", "nonsingular")) for v in inst.values())

cache = {}
planted = {}
fails = collections.Counter()
per = []
nt = 0
for rd in RDS:
    R = json.load(open(rd))
    build = R["summary"]["build"]
    for x in R["results"]:
        if "error" in x or not x.get("dump"):
            fails["no_dump_or_error"] += 1
            continue
        pb, T, idx, seed, arm, param, doff = x["key"]
        name = f"W{pb}-{idx}"
        C, G, n = curves[name]
        dd = json.load(open(x["dump"]))
        xs, qx, qy, fd = ([int(v) for v in dd[k]] for k in ("xs", "qx", "qy", "found"))
        ck = (pb, T, idx, seed)
        if ck in planted and planted[ck] != (xs, qx, qy):
            fails["planted_differs_across_runs"] += 1
        planted.setdefault(ck, (xs, qx, qy))
        f = collections.Counter()
        if not (len(xs) == len(qx) == len(qy) == len(fd) == T):
            f["length_ne_T"] += 1
        for t in range(len(fd)):
            nt += 1
            Q = (qx[t], qy[t])
            if not C.on(Q):
                f["Q_off_curve"] += 1
            if not 0 <= fd[t] < n:
                f["log_out_of_range"] += 1
            if fd[t] != xs[t] % n:
                f["log_ne_planted"] += 1
            kk = (name, fd[t], Q)
            if kk not in cache:
                cache[kk] = C.mul(fd[t], G) == Q
            if not cache[kk]:
                f["logG_ne_Q"] += 1
        fails.update(f)
        per.append(dict(build=build, key=x["key"], targets=len(fd), counters_match_runpod_rows=x["match"], failures=dict(f)))
summ = dict(instances_ok=inst_ok, runs=len(per), targets_checked=nt, distinct_scalar_mults=len(cache), configurations=len(planted),
            per_build=dict(collections.Counter(p["build"] for p in per)), per_arm=dict(collections.Counter(p["key"][4] for p in per)),
            per_bits=dict(collections.Counter(p["key"][0] for p in per)), failures=dict(fails),
            all_counters_match=all(p["counters_match_runpod_rows"] for p in per),
            all_pass=inst_ok and not fails and len(per) > 0 and all(p["counters_match_runpod_rows"] for p in per))
json.dump(dict(summary=summ, instances=inst, runs=per), open(OUT, "w"), indent=1)
print(json.dumps(summ, indent=1))
print({k: (v["p_bits"], v["n_bits"], v["cofactor_1"]) for k, v in inst.items()})
