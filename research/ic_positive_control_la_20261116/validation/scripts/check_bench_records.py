#!/usr/bin/env python3
"""Bench records of the four studies: commit, dirty flag, commands, raw-file
hashes against the files, metric values against the validator's recomputation.
Usage: check_bench_records.py REPO_ROOT OUTDIR"""
import hashlib, json, os, subprocess, sys
ROOT, O = sys.argv[1], sys.argv[2]
recs = {"lucas": "20261116T0122Z-ic-positive-control-lucas", "la": "20261116T0636Z-ic-positive-control-la",
        "unit_t": "20261116T0751Z-ic-positive-control-unit-t", "capped": "20261116T1001Z-ic-positive-control-capped"}
def src_hash(c, path):
    return hashlib.sha256(subprocess.run(["git", "-C", ROOT, "show", f"{c}:{path}"], capture_output=True, check=True).stdout).hexdigest()[:12]
for st, rid in recs.items():
    d = json.load(open(os.path.join(ROOT, "benchmarks/records", rid + ".json")))
    c = d["commit"]
    print(f"== {rid}: schema {d['schema']} status {d['status']} commit {c[:10]} commit_dirty {d['commit_dirty']} validation {d['validation']}")
    print(f"   instrument at record commit: ic_control={src_hash(c, 'src/cryptanalysis/chained_transfer/ic_control.rs')} bin={src_hash(c, 'src/bin/ic_positive_control.rs')}")
    for cmd in d["commands"]:
        print(f"   command: {' '.join(cmd['argv'])} | started {cmd['started_utc']} wall {cmd['wall_s']} exit {cmd['exit']}")
    for r in d["raw"]:
        p = os.path.join(ROOT, r["path"])
        h = hashlib.sha256(open(p, "rb").read()).hexdigest()
        print(f"   raw {r['path']}: {'sha256 OK' if h == r['sha256'] else 'SHA256 MISMATCH ' + h} bytes {r['bytes']} uploaded {r['uploaded']}")
    for m in d["metrics"]:
        print(f"   metric {m['name']} = {m['value']} spread {m['spread']} n {m['n']} baseline {m['baseline']}")
    print("   host:", d["host"])
