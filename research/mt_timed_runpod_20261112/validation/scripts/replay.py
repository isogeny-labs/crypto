#!/usr/bin/env python3
"""VALIDATOR counter replay for research/mt_timed_runpod_20261112.  Counted operations are deterministic, so one local
execution of a run key must reproduce every field the binary prints except the wall-clock fields (ms_arm, ms), for
every one of the key's 3 RunPod repeats.

modes
  full <build> <bin> <study> <out.json>                      every key of that build (612), compared with all 3 rows
  dump <build> <bin> <study> <out.json> <nonce> <dumpdir>    nonce sample: for each size x arm one key (random T,
                                                             configuration and parameter), run with MTFO_DUMP
JOBS parallel processes (default 3), pinned round-robin to CPUS (default 0,1,3).  Nothing here is timed."""
import collections
import hashlib
import json
import os
import random
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor

mode, BUILD, BIN, STUDY, OUT = sys.argv[1:6]
EXTRA = sys.argv[6:]
INST = os.path.join(STUDY, "../mt_fieldops_v5_20261112/instances")
JOBS = int(os.environ.get("JOBS", "3"))
CPUS = os.environ.get("CPUS", "0,1,3").split(",")
WALL = {"ms", "ms_arm"}
ADDED = {"wall_s", "build", "pb", "idx", "T", "rep", "seq", "key", "loadavg", "steal_ticks", "bin_sha256", "doff"}
IC = ("ic5", "ic5ks", "ic5w", "ic5rw")

rows = [json.loads(l) for l in open(os.path.join(STUDY, "results/holdout/runs.jsonl"))]
by = collections.defaultdict(list)
for r in rows:
    if r["build"] == BUILD:
        by[(r["pb"], r["T"], r["idx"], r["seed"], r["arm"], r["param"], r.get("doff"))].append(r)
keys = sorted(by, key=lambda k: (k[0], k[1], k[2], k[4], k[5], -99 if k[6] is None else k[6]))


def argv(k):
    pb, T, idx, seed, arm, param, doff = k
    a = [arm, os.path.join(INST, f"W{pb}-{idx}.json"), str(T), str(seed), str(param)]
    if arm in IC:
        a.append("4")
    if arm == "rhodpu":
        a.append(str(doff))
    return a


def execute(i, k, dump=None):
    env = dict(os.environ)
    if dump:
        env["MTFO_DUMP"] = dump
    p = subprocess.run(["taskset", "-c", CPUS[i % len(CPUS)], BIN] + argv(k), capture_output=True, text=True, timeout=7200, env=env)
    if p.returncode != 0:
        return dict(error=p.stderr[-300:])
    return json.loads(p.stdout.strip().splitlines()[-1])


def compare(k, got):
    out = []
    for rec in by[k]:
        d = []
        for f in sorted(set(rec) | set(got)):
            if f in WALL or f in ADDED:
                continue
            if rec.get(f) != got.get(f):
                d.append([f, rec.get(f), got.get(f)])
        out.append(d)
    return out


if mode == "full":
    sel = list(enumerate(keys))
    dumps = {}
elif mode == "dump":
    nonce, ddir = EXTRA
    os.makedirs(ddir, exist_ok=True)
    rnd = random.Random(hashlib.sha256((nonce + ":" + BUILD).encode()).digest())
    sel = []
    for pb in (24, 28, 32):
        for arm in ("ic5", "ic5ks", "ic5w", "ic5rw", "rho2xks", "rhodpu", "bsgs2"):
            cand = [k for k in keys if k[0] == pb and k[4] == arm]
            sel.append(rnd.choice(cand))
    sel = list(enumerate(sel))
    dumps = {i: os.path.join(ddir, f"{BUILD}-{i:02d}.json") for i, _ in sel}
else:
    sys.exit("mode")

with ThreadPoolExecutor(JOBS) as ex:
    res = list(ex.map(lambda ik: (ik[1], execute(ik[0], ik[1], dumps.get(ik[0]))), sel))

out = []
nmatch = nrows = 0
for (i, k), (_, got) in zip(sel, res):
    if "error" in got:
        out.append(dict(key=list(k), error=got["error"]))
        continue
    d = compare(k, got)
    ok = all(not x for x in d)
    nmatch += ok
    nrows += len(d)
    out.append(dict(key=list(k), match=ok, rows=len(d), diffs=[x for x in d if x][:1], ok=got["ok"], targets=got["targets"],
                    counters=[got[c] for c in ("I", "M", "S", "H", "L", "N")], dump=dumps.get(i)))
summ = dict(mode=mode, build=BUILD, bin=BIN, bin_sha256=hashlib.sha256(open(BIN, "rb").read()).hexdigest(), keys=len(sel),
            matching=nmatch, mismatching=sum(1 for x in out if x.get("match") is False), errors=sum(1 for x in out if "error" in x),
            rows_covered=nrows, per_arm=dict(collections.Counter(k[4] for _, k in sel)), per_bits=dict(collections.Counter(k[0] for _, k in sel)),
            ok_eq_targets=sum(1 for x in out if x.get("ok") == x.get("targets")))
if mode == "dump":
    summ["nonce"] = EXTRA[0]
json.dump(dict(summary=summ, results=out), open(OUT, "w"), indent=1)
print(json.dumps(summ))
