#!/usr/bin/env python3
"""Validator: replay all rows with the instrumented scratch build (counters must still match the rows),
collect the walk start/restart costs from stderr, and dump per-target logs for a sample of runs.
usage: replay_dbg.py <dbg binary> <rows.jsonl> <instances dir> <dump dir> <out jsonl> <sample json> [jobs]"""
import json, subprocess, sys, os, random, collections
from concurrent.futures import ThreadPoolExecutor

BIN, ROWS, INST, DUMP, OUT, SAMPLE = sys.argv[1:7]
JOBS = int(sys.argv[7]) if len(sys.argv) > 7 else 4
rows = [json.loads(l) for l in open(ROWS)]
os.makedirs(DUMP, exist_ok=True)

# sample: per (arm, cell) three runs; for the parametrised arms one per parameter value; idx/seed drawn
# with the validator's own RNG (seed fixed here, not by the producer)
rnd = random.Random("validator-v2-20261111")
by = collections.defaultdict(list)
for i, r in enumerate(rows):
    by[(r["arm"], r["pb"], r["T"], r["param"])].append(i)
sample = []
for (arm, pb, T, param), idxs in sorted(by.items()):
    k = 3 if arm == "ic4" else 1
    sample += rnd.sample(idxs, k)
sample = sorted(sample)
json.dump(sample, open(SAMPLE, "w"))

def name(r):
    return f"{r['arm']}/{r['pb']}/{r['T']}/{r['idx']}/{r['seed']}/{r['param']}"

def one(i):
    r = rows[i]
    argv = list(r["argv"]); argv[1] = os.path.join(INST, os.path.basename(argv[1]))
    env = dict(os.environ)
    dump = None
    if i in SS:
        dump = os.path.join(DUMP, f"run{i:04d}.txt"); env["MTFO_DUMP"] = dump
    p = subprocess.run([BIN] + argv, capture_output=True, text=True, timeout=3600, env=env)
    o = json.loads(p.stdout.strip().splitlines()[-1])
    e = json.loads(p.stderr.strip().splitlines()[-1])
    mism = [k for k, v in o.items() if k not in ("ms", "param") and r.get(k) != v]
    return dict(i=i, run=name(r), rc=p.returncode, mismatch=mism, dump=dump, **e)

SS = set(sample)
with ThreadPoolExecutor(JOBS) as ex, open(OUT, "w") as fh:
    for rec in ex.map(one, range(len(rows))):
        fh.write(json.dumps(rec) + "\n")
recs = [json.loads(l) for l in open(OUT)]
print("replayed", len(recs), "mismatching", sum(bool(r["mismatch"]) or r["rc"] != 0 for r in recs), "dumped", len(SS))
