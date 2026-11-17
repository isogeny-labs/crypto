#!/usr/bin/env python3
"""Validator: replay rows of a rows.jsonl from their recorded argv with a given binary and compare every
binary-emitted field except ms (arm, n, p, targets, seed, param, ok, I, M, S, H, L, every phase snapshot,
every arm extra).  The instance path in argv is remapped to the frozen git-archive copy (same blob).
usage: replay.py <binary> <rows.jsonl> <frozen instances dir> <out.jsonl> [--jobs N] [--subset idx.json]
                 [--dump-dir D --dump idx.json] [--stderr-json]"""
import json, subprocess, sys, os, hashlib, argparse
from concurrent.futures import ThreadPoolExecutor
ap = argparse.ArgumentParser()
ap.add_argument("binary"); ap.add_argument("rows"); ap.add_argument("inst"); ap.add_argument("out")
ap.add_argument("--jobs", type=int, default=4); ap.add_argument("--subset"); ap.add_argument("--dump-dir")
ap.add_argument("--dump"); ap.add_argument("--stderr-json", action="store_true"); ap.add_argument("--tag", default="")
a = ap.parse_args()
rows = [json.loads(l) for l in open(a.rows)]
idx = json.load(open(a.subset)) if a.subset else list(range(len(rows)))
dump = set(json.load(open(a.dump))) if a.dump else set()
if a.dump_dir: os.makedirs(a.dump_dir, exist_ok=True)
RUNNER = {"pb", "idx", "T", "k", "g", "argv", "bin_sha256", "commit", "ms"}
def name(r): return f"{r['arm']}/{r['pb']}/{r['T']}/{r['idx']}/{r['seed']}/{r['param']}"
def one(i):
    r = rows[i]
    argv = list(r["argv"]); argv[1] = os.path.join(a.inst, os.path.basename(argv[1]))
    env = dict(os.environ); d = None
    if i in dump:
        d = os.path.join(a.dump_dir, f"{a.tag}row{i:05d}.txt"); env["MTFO_DUMP"] = d
    p = subprocess.run([a.binary] + argv, capture_output=True, text=True, timeout=7200, env=env)
    rec = {"i": i, "run": name(r), "rc": p.returncode, "dump": d}
    if p.returncode != 0:
        rec["stderr"] = p.stderr[-300:]; rec["mismatch"] = ["rc"]; return rec
    o = json.loads(p.stdout.strip().splitlines()[-1])
    mism = []
    for k, v in o.items():
        if k == "ms": continue
        if k == "param":
            if float(v) != float(r["param"]): mism.append(k)
            continue
        if k not in r: mism.append("missing:" + k); continue
        if r[k] != v: mism.append(k)
    for k in r:
        if k not in o and k not in RUNNER: mism.append("extra_in_row:" + k)
    rec["mismatch"] = mism; rec["ok"] = o["ok"]; rec["targets"] = o["targets"]
    ph = list(o["phases"].values())
    rec["phase_last_eq_total"] = ph[-1] == [o["I"], o["M"], o["S"], o["H"], o["L"]]
    rec["phase_monotone"] = all(all(x <= y for x, y in zip(ph[j], ph[j + 1])) for j in range(len(ph) - 1))
    rec["out_sha256"] = hashlib.sha256(json.dumps({k: v for k, v in o.items() if k != "ms"}, sort_keys=True).encode()).hexdigest()
    if a.stderr_json:
        try: rec["instr"] = json.loads(p.stderr.strip().splitlines()[-1])
        except Exception: rec["instr"] = None
    return rec
with ThreadPoolExecutor(a.jobs) as ex, open(a.out, "w") as fh:
    for rec in ex.map(one, idx):
        fh.write(json.dumps(rec) + "\n"); fh.flush()
recs = [json.loads(l) for l in open(a.out)]
bad = [r for r in recs if r["rc"] != 0 or r.get("mismatch")]
print(a.rows, "replayed", len(recs), "mismatching", len(bad), "rc!=0", sum(r["rc"] != 0 for r in recs),
      "ok!=targets", sum(r.get("ok") != r.get("targets") for r in recs),
      "phase_last!=total", sum(not r.get("phase_last_eq_total", False) for r in recs),
      "non-monotone", sum(not r.get("phase_monotone", False) for r in recs))
for b in bad[:10]: print(b)
