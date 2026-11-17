#!/usr/bin/env python3
"""Run depth.py's round-indexed closure on the SAME instance a Rust row (raw.jsonl) describes.
usage: crosscheck.py RAW.jsonl T C IDX   (p must be < 2^16; the row must have been drawn at that p)"""
import sys, json
sys.path.insert(0, "/home/user/crypto/research/kummer_wave3_20261115/depth")
import depth
from depth import *
rows = [json.loads(l) for l in open(sys.argv[1])]
T, Cc, IDX = int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4])
row = [r for r in rows if r["t"] == T and r["c"] == Cc and r["idx"] == IDX and r["kind"] == "random"][0]
p, a, b, g, xR = row["p"], row["a"], row["b"], row["g"], row["x_r"]
C = nsd.Ctx(p, T, 2)
x = (g * C.Z) % p
c2, c1, c0 = nsd.S3coef(p, a, b, x[0], x[1])
f = (c2 * xR * xR + c1 * xR + c0) % p
assert f.all()
S = Space(C, Cc, False)
gens = [S.vec_from_coefs([C.invdft(f).reshape(-1)])]
rstar, hist = rounds(S, gens, p)
print(json.dumps(dict(p=p, t=T, c=Cc, idx=IDX, py_r_star=rstar, py_G=sum(1 for h in hist[1:] if h.get("added", 0) > 0),
    py_dims=[h["dim"] for h in hist], rust_r_star=row["r_star"], rust_G=row["G"], rust_dims=[h["dim"] for h in row["hist"]])))
