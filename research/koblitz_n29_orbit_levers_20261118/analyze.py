#!/usr/bin/env python3
"""Frozen analysis (PROTOCOL.md section 3). Reads the session, runs ecbench band/compare, writes tables/ and results.json.
Usage: ECBENCH=<pinned binary> python3 analyze.py [SESSION_DIR]"""
import json, os, subprocess, sys, collections, gzip, hashlib
HERE = os.path.dirname(os.path.abspath(__file__))
D = sys.argv[1] if len(sys.argv) > 1 else "/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/n29lv-sessions/n29"
EC = os.environ["ECBENCH"]
T = os.path.join(HERE, "tables"); os.makedirs(T, exist_ok=True)
W = ["0", "0.1", "1"]
CANDS = ["lv-all", "lv-lean", "lv-sib", "lv-tau"]
def cmp(a, b, w):
    out = subprocess.run([EC, "compare", "--dir", D, "--a", a, "--b", b, "--weight", w, "--resamples", "4000", "--seed", "20261002", "--json"],
                         capture_output=True, text=True, check=True).stdout
    return json.loads(out)
rec = os.path.join(D, "records.jsonl")
res = {"session": {"records_sha256": hashlib.sha256(open(rec, "rb").read()).hexdigest()}, "compare": {}, "phases": {}, "native": {}, "wall": {}}
ph = collections.defaultdict(lambda: collections.defaultdict(float)); nat = collections.defaultdict(lambda: collections.defaultdict(float))
wall = collections.defaultdict(lambda: collections.defaultdict(float)); cnt = collections.Counter(); status = collections.Counter()
by_key = collections.defaultdict(dict); rows = []
for l in open(rec):
    r = json.loads(l)
    status[(r["arm"], r["outcome"]["status"], r["outcome"]["matches_target"])] += 1
    if r.get("warmup"): continue
    a = r["arm"]; cnt[a] += 1
    ph[a]["total"] += r["cost"]["total_gae"]
    for p in r["phases"]:
        ph[a][p["name"]] += p["gae"]
        wall[a][p["name"]] += p.get("wall_ns") or 0
        for k, v in p.get("native", {}).items(): nat[a][p["name"] + "." + k] += v
        for k, v in p.get("work_native", {}).items(): nat[a][p["name"] + ".wn." + k] += v
    wall[a]["solve_wall_ns"] += r["time"]["solve_wall_ns"]
    # identity control: everything but wall time and record identity
    key = (r["round"], r["workload"]["target_index"])
    strip = {"cost": r["cost"], "phases": [{k: v for k, v in p.items() if k != "wall_ns"} for p in r["phases"]],
             "counters": r["counters"], "outcome": r["outcome"], "algorithm_seed": r["algorithm_seed"], "factor_base": r.get("factor_base")}
    by_key[a][key] = json.dumps(strip, sort_keys=True)
    rows.append([r["seq"], r["round"], a, r["workload"]["target_index"], r["outcome"]["status"], r["cost"]["total_gae"], r["cost"]["s"],
                 r["outcome"]["matches_target"], r["counters"].get("targets_tried")])
with gzip.open(os.path.join(T, "records-n29.csv.gz"), "wt") as f:
    f.write("seq,round,arm,target_index,status,total_gae,S,matches_target,targets_tried\n")
    for x in rows: f.write(",".join(map(str, x)) + "\n")
res["phases"] = {a: {k: v / cnt[a] for k, v in m.items()} for a, m in ph.items()}
res["native"] = {a: {k: v / cnt[a] for k, v in m.items()} for a, m in nat.items()}
res["wall"] = {a: {k: v / cnt[a] for k, v in m.items()} for a, m in wall.items()}
res["session"]["status"] = {f"{a}:{s}:{m}": c for (a, s, m), c in status.items()}
def same(a, b):
    ks = set(by_key[a]) | set(by_key[b])
    return {"records": len(ks), "identical": sum(1 for k in ks if by_key[a].get(k) == by_key[b].get(k))}
res["identity"] = {"gen-c1-off==gen-c1": same("gen-c1", "gen-c1-off"), "gen-c1-aa==gen-c1": same("gen-c1", "gen-c1-aa")}
# the levers must not change the walk: same targets_tried per (round, target) as the incumbent
tt = {a: {(int(x[1]), int(x[3])): x[8] for x in rows if x[2] == a} for a in cnt}
res["same_walk"] = {c: sum(1 for k, v in tt["gen-c1"].items() if tt[c].get(k) == v) for c in CANDS}
bj = os.path.join(T, "band-n29.json")
txt = subprocess.run([EC, "band", "--dir", D, "--ranks", "--json", bj], capture_output=True, text=True).stdout
open(os.path.join(T, "band-n29.txt"), "w").write(txt)
res["band"] = json.load(open(bj))
pairs = [("gen-c1", "gen-c1-aa"), ("gen-c1", "gen-c1-off"), ("rho-strong", "gen-c1"), ("rho-v5", "gen-c1"), ("rho-strong", "rho-v5")]
for c in CANDS:
    pairs += [("gen-c1", c), ("rho-strong", c), ("rho-v5", c)]
for a, b in pairs:
    for w in W:
        res["compare"][f"n29|{b}/{a}|w={w}"] = cmp(a, b, w)
json.dump(res, open(os.path.join(HERE, "results.json"), "w"), indent=1, default=str)
print("wrote results.json")
