#!/usr/bin/env python3
"""Frozen analysis (PROTOCOL.md section 3). Reads the sessions (SESS), runs ecbench band/compare, writes tables/ and results.json.
Usage: python3 analyze.py [SESS_DIR]"""
import json, os, subprocess, sys, collections, gzip, hashlib
HERE = os.path.dirname(os.path.abspath(__file__)); ROOT = os.path.abspath(os.path.join(HERE, "..", ".."))
SESS = sys.argv[1] if len(sys.argv) > 1 else "/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/klc-sessions"
EC = os.environ.get("ECBENCH", os.path.join(ROOT, "target/release/ecbench"))
T = os.path.join(HERE, "tables"); os.makedirs(T, exist_ok=True)
W = ["0", "0.1", "1"]
PLAN = {23: dict(c0=4, kp=3, ks=[4], extra=[3]), 29: dict(c0=1, kp=None, ks=[2], extra=[]), 37: dict(c0=8, kp=7, ks=[6, 8], extra=[])}
def cmp(d, a, b, w):
    out = subprocess.run([EC, "compare", "--dir", d, "--a", a, "--b", b, "--weight", w, "--resamples", "4000", "--seed", "20261002", "--json"],
                         capture_output=True, text=True, check=True).stdout
    j = json.loads(out)
    def find(o):
        if isinstance(o, dict):
            if "ratio" in o and ("ci95" in o or "ci" in o or "ci_low" in o):
                return o
            for v in o.values():
                r = find(v)
                if r: return r
        return None
    return j
res = {"sessions": {}, "compare": {}, "phases": {}, "band": {}}
for n, P in PLAN.items():
    d = os.path.join(SESS, f"n{n}")
    if not os.path.exists(os.path.join(d, "session.json")):
        print("missing", d); continue
    rec = os.path.join(d, "records.jsonl")
    res["sessions"][n] = {"records_sha256": hashlib.sha256(open(rec, "rb").read()).hexdigest()}
    # compact per-record table (gzip) and phase split
    ph = collections.defaultdict(lambda: collections.defaultdict(float)); cnt = collections.Counter(); status = collections.Counter()
    rows = []
    for l in open(rec):
        r = json.loads(l)
        status[(r["arm"], r["outcome"]["status"])] += 1
        if r.get("warmup"): continue
        a = r["arm"]; cnt[a] += 1
        ph[a]["total"] += r["cost"]["total_gae"]
        for p in r["phases"]:
            ph[a][p["name"]] += p["gae"]
        rows.append([r["seq"], r["round"], a, r["workload"]["target_index"], r["outcome"]["status"], r["cost"]["total_gae"], r["cost"]["s"],
                     r["outcome"]["matches_target"]])
    with gzip.open(os.path.join(T, f"records-n{n}.csv.gz"), "wt") as f:
        f.write("seq,round,arm,target_index,status,total_gae,S,matches_target\n")
        for x in rows: f.write(",".join(map(str, x)) + "\n")
    res["phases"][n] = {a: {k: v / cnt[a] for k, v in m.items()} for a, m in ph.items()}
    res["sessions"][n]["status"] = {f"{a}:{s}": c for (a, s), c in status.items()}
    bj = os.path.join(T, f"band-n{n}.json")
    txt = subprocess.run([EC, "band", "--dir", d, "--ranks", "--json", bj], capture_output=True, text=True).stdout
    open(os.path.join(T, f"band-n{n}.txt"), "w").write(txt)
    res["band"][n] = json.load(open(bj))
    c0 = f"gen-c{P['c0']}"
    pairs = [(c0, f"{c0}-aa"), (c0, f"{c0}-m1"), ("rho-strong", c0), ("rho-v5", c0), ("rho-strong", "rho-v5")]
    for k in ([P["kp"]] if P["kp"] else []) + P["ks"]:
        pairs += [(c0, f"kl-c{k}"), ("rho-strong", f"kl-c{k}"), ("rho-v5", f"kl-c{k}")]
    for c in P["extra"]:
        pairs += [(c0, f"gen-c{c}")]
    for a, b in pairs:
        for w in W:
            res["compare"][f"n{n}|{b}/{a}|w={w}"] = cmp(d, a, b, w)
json.dump(res, open(os.path.join(HERE, "results.json"), "w"), indent=1, default=str)
print("wrote results.json")
