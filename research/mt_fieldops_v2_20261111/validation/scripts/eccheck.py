#!/usr/bin/env python3
"""Validator: independent discrete-log certificate check of dumped runs.
Own Jacobian-coordinate arithmetic (no code shared with the Rust harness): for every dumped target,
Q_t on the curve, 0 <= found_t < n, found_t == planted x_t mod n, and [found_t] G == Q_t.
usage: eccheck.py <instances dir> <replay_dbg.jsonl> <out jsonl> [files...]
If files are given, check only those (negative control)."""
import json, sys, os
from multiprocessing import Pool

INST = sys.argv[1]

def jdbl(P, a, p):
    X, Y, Z = P
    if Z == 0 or Y == 0:
        return (1, 1, 0)
    YY = Y * Y % p; S = 4 * X * YY % p
    ZZ = Z * Z % p
    M = (3 * X * X + a * ZZ * ZZ) % p
    X3 = (M * M - 2 * S) % p
    Y3 = (M * (S - X3) - 8 * YY * YY) % p
    Z3 = 2 * Y * Z % p
    return (X3, Y3, Z3)

def jadd(P, Q, a, p):
    X1, Y1, Z1 = P; X2, Y2, Z2 = Q
    if Z1 == 0: return Q
    if Z2 == 0: return P
    Z1Z1 = Z1 * Z1 % p; Z2Z2 = Z2 * Z2 % p
    U1 = X1 * Z2Z2 % p; U2 = X2 * Z1Z1 % p
    S1 = Y1 * Z2 * Z2Z2 % p; S2 = Y2 * Z1 * Z1Z1 % p
    if U1 == U2:
        if S1 != S2:
            return (1, 1, 0)
        return jdbl(P, a, p)
    Hh = (U2 - U1) % p; R = (S2 - S1) % p
    HH = Hh * Hh % p; HHH = Hh * HH % p
    V = U1 * HH % p
    X3 = (R * R - HHH - 2 * V) % p
    Y3 = (R * (V - X3) - S1 * HHH) % p
    Z3 = Z1 * Z2 * Hh % p
    return (X3, Y3, Z3)

def jmul(k, G, a, p):
    # right-to-left binary, Jacobian; returns affine or None
    R = (1, 1, 0); B = (G[0], G[1], 1)
    while k:
        if k & 1:
            R = jadd(R, B, a, p)
        B = jdbl(B, a, p); k >>= 1
    if R[2] == 0:
        return None
    zi = pow(R[2], p - 2, p)  # Fermat, not the extended Euclid used by the harness
    return (R[0] * zi * zi % p, R[1] * zi * zi * zi % p)

def load_inst(name):
    d = json.load(open(os.path.join(INST, name)))
    return dict(p=int(d["field"]["modulus"]), a=int(d["a"]), b=int(d["b"]), n=int(d["subgroup_order"]),
                G=(int(d["generator"]["x"]), int(d["generator"]["y"])))

def check(job):
    path, inst_name, run = job
    c = load_inst(inst_name); p, a, b, n, G = c["p"], c["a"], c["b"], c["n"], c["G"]
    lines = open(path).read().splitlines()
    hdr = dict(kv.split("=") for kv in lines[0].split()[2:])
    hdr_ok = (int(hdr["p"]), int(hdr["a"]), int(hdr["b"]), int(hdr["n"]), int(hdr["gx"]), int(hdr["gy"])) == (p, a, b, n, G[0], G[1])
    bad = []; cnt = 0; first = None
    for ln in lines[1:]:
        t, x, f, qx, qy = map(int, ln.split())
        cnt += 1
        ok = (qy * qy - (qx ** 3 + a * qx + b)) % p == 0 and 0 <= f < n and f == x % n and jmul(f, G, a, p) == (qx, qy)
        if not ok:
            bad.append(t)
        if first is None:
            first = dict(t=t, log=f, Qx=qx, Qy=qy)
    return dict(run=run, file=os.path.basename(path), header_ok=hdr_ok, targets_checked=cnt, failures=len(bad),
                failed_targets=bad[:5], sample_log=first)

if __name__ == "__main__":
    recs = [json.loads(l) for l in open(sys.argv[2])]
    rows_argv = {}
    jobs = []
    files = sys.argv[4:]
    if files:
        for f in files:
            # negative control files: name encodes the instance after '__'
            inst = os.path.basename(f).split("__")[1].replace(".txt", ".json")
            jobs.append((f, inst, os.path.basename(f)))
    else:
        rows = None
        for r in recs:
            if r.get("dump"):
                arm, pb, T, idx, seed, param = r["run"].split("/")
                jobs.append((r["dump"], f"W{pb}-{idx}.json", r["run"]))
    with Pool(4) as pool, open(sys.argv[3], "w") as fh:
        res = pool.map(check, jobs)
        for x in res:
            fh.write(json.dumps(x) + "\n")
    print("runs", len(res), "targets", sum(x["targets_checked"] for x in res), "failures", sum(x["failures"] for x in res),
          "header mismatches", sum(not x["header_ok"] for x in res))
