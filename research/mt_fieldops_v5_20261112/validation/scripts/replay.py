#!/usr/bin/env python3
"""VALIDATOR replay of research/mt_fieldops_v5_20261112 rows (independent of the producer's run.py).
usage: replay.py <mode> <binary> <frozen study dir> <out.json> <nonce> [dumpdir]
modes: all      every row of results/holdout/rows.jsonl
       sample   nonce-stratified: 1 row per (arm, param, doff) x bits  (51 rows)
       producer nonce-stratified: 1 row per (arm, param, doff), bits drawn by nonce (17 rows)
       dump     one (idx, seed) per (bits, T) cell drawn by nonce, all 17 runs, with MTFO_DUMP target dumps
Compares every field the binary prints except "ms" with the committed row."""
import json, sys, os, subprocess, hashlib, random, collections
from concurrent.futures import ThreadPoolExecutor
mode, BIN, FZ, OUT, NONCE = sys.argv[1:6]
DUMP = sys.argv[6] if len(sys.argv) > 6 else None
rows = [json.loads(l) for l in open(os.path.join(FZ, "results/holdout/rows.jsonl"))]
man = {e["file"]: e["sha256"] for e in json.load(open(os.path.join(FZ, "instances/MANIFEST.json")))}
for fn, h in man.items():
    assert hashlib.sha256(open(os.path.join(FZ, "instances", fn), "rb").read()).hexdigest() == h, fn
META = {"pb", "idx", "T", "argv", "bin_sha256", "commit", "doff"}
def key(r): return (r["arm"], r["param"], r.get("doff"))
rnd = random.Random(NONCE + ":" + mode)
idxs = list(range(len(rows)))
if mode == "all":
    pick = idxs
elif mode == "sample":
    g = collections.defaultdict(list)
    for i in idxs: g[(key(rows[i]), rows[i]["pb"])].append(i)
    pick = sorted(rnd.choice(v) for k, v in sorted(g.items(), key=lambda kv: str(kv[0])))
elif mode == "producer":
    g = collections.defaultdict(list)
    for i in idxs: g[key(rows[i])].append(i)
    pick = sorted(rnd.choice(v) for k, v in sorted(g.items(), key=lambda kv: str(kv[0])))
elif mode == "dump":
    cells = sorted({(r["pb"], r["T"]) for r in rows}); pick = []
    for c in cells:
        cfgs = sorted({(r["idx"], r["seed"]) for r in rows if (r["pb"], r["T"]) == c})
        ch = rnd.choice(cfgs)
        pick += [i for i in idxs if (rows[i]["pb"], rows[i]["T"], rows[i]["idx"], rows[i]["seed"]) == c + ch]
else:
    raise SystemExit("mode")
def run(i):
    r = rows[i]
    argv = list(r["argv"]); argv[1] = os.path.join(FZ, "instances", os.path.basename(argv[1]))
    env = dict(os.environ)
    dpath = None
    if DUMP:
        dpath = os.path.join(DUMP, f"row{i}.json"); env["MTFO_DUMP"] = dpath
    p = subprocess.run([BIN] + argv, capture_output=True, text=True, env=env, timeout=7200)
    if p.returncode != 0:
        return dict(row=i, ok=False, error=p.stderr[-300:])
    o = json.loads(p.stdout.strip().splitlines()[-1])
    xc = None
    for l in p.stderr.splitlines():
        if l.startswith("XC "): xc = json.loads(l[3:])
    bad = []
    for k, v in o.items():
        if k == "ms": continue
        if k not in r or r[k] != v: bad.append(k)
    extra = sorted(set(r) - set(o) - META)
    return dict(row=i, arm=r["arm"], param=r["param"], doff=r.get("doff"), pb=r["pb"], T=r["T"], idx=r["idx"], seed=r["seed"],
                match=not bad and not extra, bad=bad, extra_keys=extra, ok_eq_targets=o["ok"] == o["targets"],
                xc=xc, out={k: o[k] for k in ("I", "M", "S", "H", "L", "N", "ok", "targets")}, dump=dpath)
with ThreadPoolExecutor(int(os.environ.get("JOBS", "4"))) as ex:
    res = list(ex.map(run, pick))
binsha = hashlib.sha256(open(BIN, "rb").read()).hexdigest()
summ = dict(mode=mode, binary=BIN, binary_sha256=binsha, nonce=NONCE, rows_replayed=len(res),
            mismatching=sum(not x.get("match") for x in res), errors=sum("error" in x for x in res),
            ok_ne_targets=sum(not x.get("ok_eq_targets", False) for x in res),
            arms=dict(collections.Counter(x.get("arm") for x in res)),
            bits=dict(collections.Counter(x.get("pb") for x in res)))
json.dump(dict(summary=summ, results=res), open(OUT, "w"))
print(json.dumps(summ))
