#!/usr/bin/env python3
# Validator replay: re-run every row's argv with the binary and compare all output fields except ms.
import json, subprocess, sys, hashlib
from concurrent.futures import ThreadPoolExecutor
BIN = sys.argv[3]
rows_path, out_path = sys.argv[1], sys.argv[2]
binsha = hashlib.sha256(open(BIN, "rb").read()).hexdigest()
rows = [json.loads(l) for l in open(rows_path)]
def go(i):
    r = rows[i]
    p = subprocess.run([BIN] + r["argv"], capture_output=True, text=True, timeout=3600)
    if p.returncode != 0:
        return i, None, p.stderr[-300:]
    o = json.loads(p.stdout.strip().splitlines()[-1])
    diffs = {k: (r.get(k), v) for k, v in o.items() if k != "ms" and r.get(k) != v}
    return i, diffs, None
res = list(ThreadPoolExecutor(8).map(go, range(len(rows))))
bad = [(i, d, e) for i, d, e in res if e or d]
json.dump(dict(binsha=binsha, n=len(rows), mismatches=len(bad), detail=bad[:20]), open(out_path, "w"), indent=1)
print("binary", binsha, "rows", len(rows), "mismatching/failed", len(bad))
