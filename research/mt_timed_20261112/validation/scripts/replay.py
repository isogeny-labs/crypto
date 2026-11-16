#!/usr/bin/env python3
"""VALIDATOR counter replay for research/mt_timed_20261112 (and v5 cross-check).  Counts are deterministic, so a replay
of a run key must reproduce every counted field of the recorded output exactly (all fields the binary prints except
the wall-clock fields ms_arm and ms).

modes
  timed    <bin> <frozen root> <out.json> <nonce> [sample]   replay run keys of the timed holdout (all 612, or a
                                                            nonce sample of `sample` keys); compares with every
                                                            repeat of that key in runs.jsonl
  v5       <bin> <frozen root> <out.json> <nonce> <sample>   replay a nonce sample of v5 rows (seeds 41/42) with the
                                                            mt_timed binary; compares I,M,S,H,L,N,ok,phases and all
                                                            printed fields with v5 rows.jsonl
  dump     <bin> <frozen root> <out.json> <nonce> <sample> <dumpdir>  run a nonce sample of timed keys with the dump
                                                            build (MTFO_DUMP) for the EC check
JOBS (default 3) parallel processes, pinned round-robin to CPUS (default 0,1,3: CPU 2 is kept for timing)."""
import json, sys, os, subprocess, hashlib, random, collections
from concurrent.futures import ThreadPoolExecutor

mode, BIN, ROOT, OUT, NONCE = sys.argv[1:6]
EXTRA = sys.argv[6:]
TD = os.path.join(ROOT, "research/mt_timed_20261112")
V5 = os.path.join(ROOT, "research/mt_fieldops_v5_20261112")
INST = os.path.join(V5, "instances")
JOBS = int(os.environ.get("JOBS", "3"))
CPUS = os.environ.get("CPUS", "0,1,3").split(",")
WALL = {"ms", "ms_arm"}
RUNPY = {"wall_s", "pb", "idx", "T", "rep", "seq", "key", "loadavg", "bin_sha256", "commit", "argv"}


def rng_for(tag):
    return random.Random(hashlib.sha256((NONCE + ":" + tag).encode()).digest())


def argv_of(r):
    a = [r["arm"], os.path.join(INST, f"W{r['pb']}-{r['idx']}.json"), str(r["T"]), str(r["seed"]), str(r["param"])]
    if r["arm"] in ("ic5", "ic5ks", "ic5w", "ic5rw"):
        a.append("4")
    if r["arm"] == "rhodpu":
        a.append(str(r["doff"]))
    return a


def run(i, a, env=None):
    cpu = CPUS[i % len(CPUS)]
    p = subprocess.run(["taskset", "-c", cpu, BIN] + a, capture_output=True, text=True, timeout=3600, env=env)
    if p.returncode != 0:
        return dict(error=p.stderr[-300:])
    return json.loads(p.stdout.strip().splitlines()[-1])


def compare(rec, got):
    diffs = []
    for k in sorted(set(rec) | set(got)):
        if k in WALL or k in RUNPY:
            continue
        if k == "doff" and k not in got:
            continue  # added by run.py for rhodpu
        if rec.get(k) != got.get(k):
            diffs.append([k, rec.get(k), got.get(k)])
    return diffs


binsha = hashlib.sha256(open(BIN, "rb").read()).hexdigest()
res = dict(mode=mode, bin=BIN, bin_sha256=binsha, nonce=NONCE, results=[])
if mode in ("timed", "dump"):
    rows = [json.loads(l) for l in open(os.path.join(TD, "results/holdout/runs.jsonl"))]
    bykey = collections.defaultdict(list)
    for j, r in enumerate(rows):
        bykey[r["key"]].append(j)
    keys = sorted(bykey)
    if EXTRA:
        keys = sorted(rng_for(mode).sample(keys, int(EXTRA[0])))
    dumpdir = EXTRA[1] if mode == "dump" else None
    if dumpdir:
        os.makedirs(dumpdir, exist_ok=True)

    def job(t):
        i, k = t
        r0 = rows[bykey[k][0]]
        env = None
        if dumpdir:
            env = dict(os.environ, MTFO_DUMP=os.path.join(dumpdir, f"key{k}.json"))
        got = run(i, argv_of(r0), env)
        d = {j: compare(rows[j], got) for j in bykey[k]}
        return dict(key=k, rows=bykey[k], arm=r0["arm"], pb=r0["pb"], T=r0["T"], idx=r0["idx"], seed=r0["seed"], param=r0["param"],
                    doff=r0.get("doff"), argv=argv_of(r0), error=got.get("error"), ok=got.get("ok"), targets=got.get("targets"),
                    match=all(not v for v in d.values()) and "error" not in got,
                    diffs={str(j): v for j, v in d.items() if v}, dump=os.path.join(dumpdir, f"key{k}.json") if dumpdir else None)
    with ThreadPoolExecutor(JOBS) as ex:
        res["results"] = list(ex.map(job, enumerate(keys)))
elif mode == "v5":
    rows = [json.loads(l) for l in open(os.path.join(V5, "results/holdout/rows.jsonl"))]
    # stratified nonce sample: per arm an equal share
    by = collections.defaultdict(list)
    for j, r in enumerate(rows):
        by[r["arm"]].append(j)
    n = int(EXTRA[0]); rr = rng_for("v5"); pick = []
    arms = sorted(by)
    for a in arms:
        pick += rr.sample(by[a], max(1, n // len(arms)))
    pick = sorted(pick)

    def job(t):
        i, j = t
        r = rows[j]
        a = list(r["argv"]); a[1] = os.path.join(INST, os.path.basename(a[1]))
        got = run(i, a)
        d = compare(r, got)
        return dict(row=j, arm=r["arm"], pb=r["pb"], T=r["T"], idx=r["idx"], seed=r["seed"], param=r["param"], argv=a,
                    counters_rec={c: r[c] for c in "IMSHLN"}, counters_got={c: got.get(c) for c in "IMSHLN"},
                    match=not d and "error" not in got, diffs=d)
    with ThreadPoolExecutor(JOBS) as ex:
        res["results"] = list(ex.map(job, enumerate(pick)))
rs = res["results"]
res["summary"] = dict(n=len(rs), matching=sum(x["match"] for x in rs), mismatching=sum(not x["match"] for x in rs),
                      errors=sum(bool(x.get("error")) for x in rs), per_arm=dict(collections.Counter(x["arm"] for x in rs)),
                      rows_covered=sum(len(x.get("rows", [0])) for x in rs))
json.dump(res, open(OUT, "w"), indent=1)
print(json.dumps(res["summary"]))
