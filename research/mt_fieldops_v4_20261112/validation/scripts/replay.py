#!/usr/bin/env python3
"""Validator replay of the v4 holdout (independent of the producer's run.py/analyze.py).

usage: replay.py <mode> <binary> <frozen study dir> <out.json> [nonce]
  mode all      : every row of results/holdout/rows.jsonl (instrumented build; collects HOP/CANON/PIDX/DPT
                  and the retention-log samples written to stderr by validation/scripts/patch_instr.py)
  mode sample   : nonce-sampled rows, 4 per (arm, bits) = 60 rows (pristine build)
  mode producer : nonce-sampled rows, 1 per (arm, bits) = 15 rows (producer binary)
  mode dump     : every run of one nonce-chosen configuration per (bits, T) cell (instrumented build,
                  MTFO_DUMP=1); the per-target dumps are written next to out.json for ecverify.py
Each run is executed from the argv recorded in rows.jsonl with the instance path mapped to the frozen
git-archive copy of the study (blob-identical; checked here).  Every field the binary prints except
ms must equal the recorded row exactly.
"""
import json, sys, os, hashlib, subprocess, collections
from concurrent.futures import ThreadPoolExecutor
mode, BIN, FROZEN, OUT = sys.argv[1:5]
nonce = sys.argv[5] if len(sys.argv) > 5 else None
rows_path = os.path.join(FROZEN, "results/holdout/rows.jsonl")
raw = open(rows_path, "rb").read()
rows = [json.loads(l) for l in raw.decode().splitlines()]
inst = os.path.join(FROZEN, "instances")
man = {m["file"]: m["sha256"] for m in json.load(open(os.path.join(inst, "MANIFEST.json")))}
for fn, h in man.items():
    assert hashlib.sha256(open(os.path.join(inst, fn), "rb").read()).hexdigest() == h, fn
def key(i):
    return hashlib.sha256((nonce + ":" + str(i)).encode()).hexdigest()
if mode == "all":
    sel = list(range(len(rows)))
elif mode in ("sample", "producer"):
    per = 4 if mode == "sample" else 1
    order = sorted(range(len(rows)), key=key); cnt = collections.Counter(); sel = []
    for i in order:
        k = (rows[i]["arm"], rows[i]["pb"])
        if cnt[k] < per:
            cnt[k] += 1; sel.append(i)
elif mode == "dump":
    cfgs = sorted({(r["pb"], r["T"], r["idx"], r["seed"]) for r in rows}, key=lambda c: key(repr(c)))
    pick = {}
    for c in cfgs:
        pick.setdefault(c[:2], c)
    chosen = set(pick.values())
    sel = [i for i, r in enumerate(rows) if (r["pb"], r["T"], r["idx"], r["seed"]) in chosen]
else:
    raise SystemExit("mode?")
dumpdir = OUT + ".dumps"
if mode == "dump":
    os.makedirs(dumpdir, exist_ok=True)
SKIP = {"ms"}
def run(i):
    r = rows[i]
    argv = list(r["argv"]); recorded_path = argv[1]
    argv[1] = os.path.join(inst, os.path.basename(recorded_path))
    env = dict(os.environ, MTFO_DUMP="1" if mode == "dump" else "0")
    p = subprocess.run([BIN] + argv, capture_output=True, text=True, env=env)
    if p.returncode != 0:
        return dict(i=i, error=p.stderr[-300:])
    o = json.loads(p.stdout.strip().splitlines()[-1])
    diffs = [k for k in o if k not in SKIP and o[k] != r.get(k)]
    res = dict(i=i, arm=r["arm"], pb=r["pb"], T=r["T"], idx=r["idx"], seed=r["seed"], param=r["param"],
               doff=r.get("doff"), match=not diffs, diffs=diffs, ok=o["ok"], targets=o["targets"], H=o["H"])
    # internal consistency: phases monotone, last phase == totals
    ph = list(o["phases"].values())
    res["monotone"] = all(all(a <= b for a, b in zip(ph[j], ph[j + 1])) for j in range(len(ph) - 1))
    res["last_eq_total"] = ph[-1] == [o["I"], o["M"], o["S"], o["H"], o["L"]]
    if BIN.endswith("mtfo_instr"):
        e = json.loads(p.stderr.strip().splitlines()[-1])
        res.update(hop=e["hop"], canon=e["canon"], pidx=e["pidx"], dpt=e["dpt"], nsamp=e["nsamp"], samp=e["samp"][:300])
        if mode == "dump":
            fn = os.path.join(dumpdir, f"row{i}.json")
            json.dump(dict(i=i, argv=r["argv"], q=e["q"]), open(fn, "w"))
            res["dump"] = os.path.basename(fn)
    return res
out = []
with ThreadPoolExecutor(int(os.environ.get("JOBS", "4"))) as ex:
    for res in ex.map(run, sorted(sel, key=lambda i: -rows[i]["pb"])):
        out.append(res)
summary = dict(mode=mode, binary=BIN, binary_sha256=hashlib.sha256(open(BIN, "rb").read()).hexdigest(),
               nonce=nonce, rows_sha256=hashlib.sha256(raw).hexdigest(), n=len(out),
               mismatching=sum(1 for o in out if not o.get("match")),
               errors=sum(1 for o in out if "error" in o),
               ok_ne_targets=sum(1 for o in out if o.get("ok") != o.get("targets")),
               non_monotone=sum(1 for o in out if not o.get("monotone", True)),
               last_ne_total=sum(1 for o in out if not o.get("last_eq_total", True)),
               per_arm=dict(collections.Counter(o.get("arm") for o in out)),
               per_arm_bits=dict(collections.Counter(f'{o.get("arm")}/{o.get("pb")}' for o in out)))
json.dump(dict(summary=summary, runs=out), open(OUT, "w"))
print(json.dumps(summary))
