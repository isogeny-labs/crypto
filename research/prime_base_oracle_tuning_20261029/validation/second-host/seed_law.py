#!/usr/bin/env python3
"""Validator's re-derivation of the ecbench algorithm-seed law
(src/cryptanalysis/ecbench/spec.rs `plan`, canonical.rs `derive_u64`, identical
at 0e124fb1b207 and cb2ec6873f30): seed = first 8 bytes (big-endian) of
SHA-256 of the canonical JSON {"domain": D, "parts": [m.seed, wid, round]},
D = "ecbench.warmup_seed" on warm-up rounds, else "ecbench.algorithm_seed";
wid = the workload id's hex digits after the leading 'W'.

Usage: python3 -I seed_law.py SPEC.json RECORDS.jsonl [RECORDS.jsonl ...]
"""
import hashlib
import json
import sys


def derive_u64(domain, parts):
    text = json.dumps({"domain": domain, "parts": parts}, sort_keys=True, separators=(",", ":"))
    return int.from_bytes(hashlib.sha256(text.encode()).digest()[:8], "big")


def main():
    spec = json.load(open(sys.argv[1]))
    m = spec["measurement"]
    out = {"spec": sys.argv[1], "master_seed": m["seed"], "files": []}
    for path in sys.argv[2:]:
        n = bad = 0
        for line in open(path):
            r = json.loads(line)
            wid = int(r["workload"]["workload_id"][1:], 16)
            dom = "ecbench.warmup_seed" if r["warmup"] else "ecbench.algorithm_seed"
            want = derive_u64(dom, [m["seed"], wid, r["round"]])
            n += 1
            bad += str(want) != str(r["algorithm_seed"])
        out["files"].append({"records": path, "checked": n, "mismatches": bad})
    print(json.dumps(out, indent=1))


if __name__ == "__main__":
    main()
