#!/usr/bin/env python3
"""Validator: independent discrete-log certificate check of dumped runs (own code, nothing shared with the
Rust harness).  Projective (homogeneous) coordinates, Montgomery-ladder scalar multiplication, one Fermat
inversion at the end.  For every dumped target: Q_t on the curve, 0 <= log_t < n, log_t == planted x_t mod n,
and [log_t] G == Q_t.  Also: the dump header names the instance's curve; planted logs are distinct and spread
over [1, n); runs of the same configuration (pb, T, idx, seed) plant identical target lists in every arm.
usage: eccheck.py <instances dir> <replay out.jsonl> <rows.jsonl> <out.json> [--jobs N]"""
import json, sys, os, hashlib, collections, argparse
from multiprocessing import Pool
ap = argparse.ArgumentParser(); ap.add_argument("inst"); ap.add_argument("replay"); ap.add_argument("rows"); ap.add_argument("out")
ap.add_argument("--jobs", type=int, default=3); a = ap.parse_args()

def inst(path):
    d = json.load(open(path))
    return dict(p=int(d["field"]["modulus"]), a=int(d["a"]), b=int(d["b"]), n=int(d["subgroup_order"]),
                G=(int(d["generator"]["x"]), int(d["generator"]["y"])))

def padd(P, Q, A, p):
    # homogeneous projective, complete-enough formulas with explicit special cases (Z = 0 is O)
    X1, Y1, Z1 = P; X2, Y2, Z2 = Q
    if Z1 == 0: return Q
    if Z2 == 0: return P
    U1 = Y2 * Z1 % p; U2 = Y1 * Z2 % p; V1 = X2 * Z1 % p; V2 = X1 * Z2 % p
    if V1 == V2:
        if U1 != U2 or Y1 == 0: return (0, 1, 0)
        return pdbl(P, A, p)
    U = (U1 - U2) % p; V = (V1 - V2) % p; W = Z1 * Z2 % p
    VV = V * V % p; VVV = V * VV % p; R = VV * V2 % p
    Aa = (U * U * W - VVV - 2 * R) % p
    return (V * Aa % p, (U * (R - Aa) - VVV * U2) % p, VVV * W % p)

def pdbl(P, A, p):
    X, Y, Z = P
    if Z == 0 or Y == 0: return (0, 1, 0)
    W = (A * Z * Z + 3 * X * X) % p; S = Y * Z % p; B = X * Y * S % p
    H = (W * W - 8 * B) % p
    return (2 * H * S % p, (W * (4 * B - H) - 8 * Y * Y * S * S) % p, 8 * S * S * S % p)

def smul(k, G, A, p):
    R0 = (0, 1, 0); R1 = (G[0], G[1], 1)
    for bit in bin(k)[2:] if k > 0 else "":
        if bit == "1": R0 = padd(R0, R1, A, p); R1 = pdbl(R1, A, p)
        else: R1 = padd(R0, R1, A, p); R0 = pdbl(R0, A, p)
    X, Y, Z = R0
    if Z == 0: return None
    zi = pow(Z, p - 2, p)
    return (X * zi % p, Y * zi % p)

def check(job):
    i, path, ipath, run = job
    c = inst(ipath); p, A, b, n, G = c["p"], c["a"], c["b"], c["n"], c["G"]
    lines = open(path).read().splitlines()
    hdr = dict(kv.split("=") for kv in lines[0].split()[2:])
    hdr_ok = (int(hdr["p"]), int(hdr["a"]), int(hdr["b"]), int(hdr["n"]), int(hdr["gx"]), int(hdr["gy"])) == (p, A, b, n, G[0], G[1])
    bad = []; xs = []
    for ln in lines[1:]:
        t, x, f, qx, qy = map(int, ln.split())
        xs.append(x)
        ok = (qy * qy - (qx ** 3 + A * qx + b)) % p == 0 and 0 <= f < n and f == x % n and smul(f, G, A, p) == (qx, qy)
        if not ok: bad.append(t)
    h = hashlib.sha256(",".join(map(str, xs)).encode()).hexdigest()
    return dict(i=i, run=run, header_ok=hdr_ok, targets_checked=len(xs), failures=len(bad), failed=bad[:5],
                planted_distinct=len(set(xs)) == len(xs), planted_min_frac=min(xs) / n, planted_max_frac=max(xs) / n,
                planted_mean_frac=sum(xs) / len(xs) / n, planted_sha=h)

if __name__ == "__main__":
    rows = [json.loads(l) for l in open(a.rows)]
    recs = [json.loads(l) for l in open(a.replay)]
    jobs = [(r["i"], r["dump"], os.path.join(a.inst, os.path.basename(rows[r["i"]]["argv"][1])), r["run"]) for r in recs if r.get("dump")]
    with Pool(a.jobs) as pool: res = pool.map(check, jobs)
    cfgsha = collections.defaultdict(set)
    for r in res:
        rw = rows[r["i"]]; cfgsha[(rw["pb"], rw["T"], rw["idx"], rw["seed"])].add(r["planted_sha"])
    multi = {str(k): len(v) for k, v in cfgsha.items()}
    per_arm = collections.Counter(r["run"].split("/")[0] for r in res)
    out = dict(runs=len(res), runs_passing=sum(r["failures"] == 0 and r["header_ok"] for r in res),
               targets=sum(r["targets_checked"] for r in res), failures=sum(r["failures"] for r in res),
               per_arm=dict(per_arm), cells=sorted({"/".join(r["run"].split("/")[1:3]) for r in res}),
               configs_with_conflicting_planting=[k for k, v in multi.items() if v > 1], results=res)
    json.dump(out, open(a.out, "w"), indent=1)
    print({k: v for k, v in out.items() if k != "results"})
