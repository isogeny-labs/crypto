#!/usr/bin/env python3
"""Fresh validator specs for research/ic_ksum_prune_20261115 (VAL-20261115-276156).

Seeds come from the nonce in nonce.txt (drawn after the results commit b7666dcd8f):
  n = sha256(ls-remote line + "\n" + UTC + "\n")
  FB15: target_seed = int(n[0:12], 16),  measurement seed = int(n[12:28], 16) >> 1   (largest-gain row)
  FP24: target_seed = int(n[28:40], 16), measurement seed = int(n[40:56], 16) >> 1   (largest prime row)
  third row = [P20, P22, B18, B21, B24, B27][int(n[56:64], 16) mod 6], seeds from m = sha256(n + ":third"):
        target_seed = int(m[0:12], 16), measurement seed = int(m[12:28], 16) >> 1
Each spec is the study's registered <row>-r1 spec with only the label and the two seeds changed
(same curve, arms, factor-base sizes, 48 targets, 3 rounds, 1 warm-up, order alternate, L0).
usage: python3 -I mkfresh.py <study dir> <out dir>
"""
import hashlib, json, os, sys

study, out = sys.argv[1:3]
kv = dict(l.split("=", 1) for l in open(os.path.join(study, "validator/nonce.txt")).read().splitlines())
pre = kv["ls_remote"] + "\n" + kv["utc"] + "\n"
n = hashlib.sha256(pre.encode()).hexdigest()
assert n == kv["nonce"], "nonce does not recompute"
m = hashlib.sha256((n + ":third").encode()).hexdigest()
third = ["P20", "P22", "B18", "B21", "B24", "B27"][int(n[56:64], 16) % 6]
plan = {"FB15": ("B15", int(n[0:12], 16), int(n[12:28], 16) >> 1),
        "FP24": ("P24", int(n[28:40], 16), int(n[40:56], 16) >> 1),
        "F" + third: (third, int(m[0:12], 16), int(m[12:28], 16) >> 1)}
os.makedirs(out, exist_ok=True)
for name, (row, ts, ms) in plan.items():
    spec = json.load(open(os.path.join(study, "specs", row + "-r1.json")))
    spec["label"] = "ic_ksum_prune_20261115 validator fresh %s (VAL-20261115-276156, nonce-derived seeds)" % row
    spec["workloads"]["target_seed"] = ts
    spec["measurement"]["seed"] = ms
    json.dump(spec, open(os.path.join(out, name + ".spec.json"), "w"), indent=1)
    print(name, row, ts, ms)
