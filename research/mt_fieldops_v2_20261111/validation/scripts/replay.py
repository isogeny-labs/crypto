#!/usr/bin/env python3
"""Validator: replay every row of rows.jsonl from its argv with a given binary; compare every
binary-emitted field except ms.  usage: replay.py <binary> <rows.jsonl> <instances dir> <out jsonl> [jobs]"""
import json, subprocess, sys, os, hashlib
from concurrent.futures import ThreadPoolExecutor

BIN, ROWS, INST, OUT = sys.argv[1:5]
JOBS = int(sys.argv[5]) if len(sys.argv) > 5 else 4
rows = [json.loads(l) for l in open(ROWS)]

def remap(path):
    # the recorded argv names the working-tree instance; replay from the frozen copy of the same blob
    return os.path.join(INST, os.path.basename(path))

def one(i):
    r = rows[i]
    argv = list(r["argv"]); argv[1] = remap(argv[1])
    p = subprocess.run([BIN] + argv, capture_output=True, text=True, timeout=3600)
    rec = {"i": i, "rc": p.returncode}
    if p.returncode != 0:
        rec["stderr"] = p.stderr[-300:]; return rec
    o = json.loads(p.stdout.strip().splitlines()[-1])
    mism = []
    for k, v in o.items():
        if k == "ms":
            continue
        if k == "param":
            if float(v) != float(r["param"]): mism.append(k)
            continue
        if k not in r:
            mism.append("missing:" + k); continue
        if r[k] != v:
            mism.append(k)
    # fields in the row that the binary should have emitted (everything except runner-added keys)
    runner_keys = {"pb", "idx", "T", "g", "argv", "bin_sha256", "commit", "ms"}
    for k in r:
        if k not in o and k not in runner_keys:
            mism.append("extra_in_row:" + k)
    rec["mismatch"] = mism
    rec["ok"] = o["ok"]; rec["targets"] = o["targets"]
    # phase snapshots must be monotone and the last must equal the totals
    ph = list(o["phases"].values())
    rec["phase_last_eq_total"] = ph[-1] == [o["I"], o["M"], o["S"], o["H"], o["L"]]
    rec["phase_monotone"] = all(all(a <= b for a, b in zip(ph[j], ph[j + 1])) for j in range(len(ph) - 1))
    rec["out_sha256"] = hashlib.sha256(json.dumps({k: v for k, v in o.items() if k != "ms"}, sort_keys=True).encode()).hexdigest()
    return rec

with ThreadPoolExecutor(JOBS) as ex, open(OUT, "w") as fh:
    for rec in ex.map(one, range(len(rows))):
        fh.write(json.dumps(rec) + "\n")
recs = [json.loads(l) for l in open(OUT)]
bad = [r for r in recs if r["rc"] != 0 or r.get("mismatch")]
print("replayed", len(recs), "mismatching", len(bad), "rc!=0", sum(r["rc"] != 0 for r in recs),
      "ok!=targets", sum(r.get("ok") != r.get("targets") for r in recs),
      "phase_last!=total", sum(not r.get("phase_last_eq_total", False) for r in recs),
      "non-monotone", sum(not r.get("phase_monotone", False) for r in recs))
for b in bad[:10]:
    print(b)
