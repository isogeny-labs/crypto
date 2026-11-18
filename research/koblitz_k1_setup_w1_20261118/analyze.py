#!/usr/bin/env python3
"""Frozen analysis (PROTOCOL.md section 3; adapted from koblitz_n37_sibwalk_k8_20261118/analyze.py: three curves, arm names, the two single-lever arms). Reads the 3 sessions, runs ecbench band/compare (per replicate and pooled per
curve), writes tables/ and results.json. Usage: ECBENCH=<pinned binary> python3 analyze.py [SESS_DIR]"""
import json, os, subprocess, sys, collections, gzip, hashlib
HERE = os.path.dirname(os.path.abspath(__file__))
SESS = sys.argv[1] if len(sys.argv) > 1 else "/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/w1-sessions"
EC = os.environ["ECBENCH"]
T = os.path.join(HERE, "tables"); os.makedirs(T, exist_ok=True)
W = ["0", "0.1", "1"]
REPS = (1, 2, 3)
INC, CAND = "inc", "w1"
SINGLE = ("w1-neg", "w1-ids")
def cmp(dirs, a, b, w):
    out = subprocess.run([EC, "compare", "--dir", *dirs, "--a", a, "--b", b, "--weight", w, "--resamples", "4000", "--seed", "20261002", "--json"],
                         capture_output=True, text=True, check=True).stdout
    return json.loads(out)
def ident(r):
    # Everything a default-off flag must leave alone, minus wall time and parameter echoes
    # (method params, method/fb ids, fb_sha256 and detail echo the parameter string).
    return json.dumps({"cost": r["cost"], "phases": [{k: v for k, v in p.items() if k != "wall_ns"} for p in r["phases"]],
                       "counters": r["counters"], "outcome": r["outcome"],
                       "points_sha256": (r.get("factor_base") or {}).get("points_sha256")}, sort_keys=True)
res = {"sessions": {}, "compare": {}, "phases": {}, "native": {}, "wall": {}, "identity": {}, "same_walk": {}, "status": {}, "band": {}}
for n in (23, 29, 37):
    rows = []
    ph = collections.defaultdict(lambda: collections.defaultdict(float)); nat = collections.defaultdict(lambda: collections.defaultdict(float))
    wall = collections.defaultdict(lambda: collections.defaultdict(float)); cnt = collections.Counter(); status = collections.Counter()
    by_key = collections.defaultdict(dict); tt = collections.defaultdict(dict); rec_d = collections.defaultdict(dict)
    dirs = []
    for rep in REPS:
        name = f"n{n}-r{rep}"; d = os.path.join(SESS, name); dirs.append(d)
        rec = os.path.join(d, "records.jsonl")
        res["sessions"][name] = {"records_sha256": hashlib.sha256(open(rec, "rb").read()).hexdigest(),
                                 "session": json.load(open(os.path.join(d, "session.json"))).get("session_id")}
        for l in open(rec):
            r = json.loads(l)
            status[(r["arm"], r["outcome"]["status"], r["outcome"]["matches_target"])] += 1
            if r.get("warmup"): continue
            a = r["arm"]; cnt[a] += 1
            ph[a]["total"] += r["cost"]["total_gae"]
            for p in r["phases"]:
                ph[a][p["name"]] += p["gae"]
                ph[a][p["name"] + ".group_ops"] += (p.get("adds") or 0) + (p.get("doubles") or 0)
                wall[a][p["name"]] += p.get("wall_ns") or 0
                for k, v in p.get("native", {}).items(): nat[a][p["name"] + "." + k] += v
                for k, v in p.get("work_native", {}).items(): nat[a][p["name"] + ".wn." + k] += v
            wall[a]["solve_wall_ns"] += r["time"]["solve_wall_ns"]
            key = (rep, r["round"], r["workload"]["target_index"])
            by_key[a][key] = ident(r); tt[a][key] = r["counters"].get("targets_tried"); rec_d[a][key] = r["outcome"]["recovered"]
            rows.append([rep, r["seq"], r["round"], a, r["workload"]["target_index"], r["outcome"]["status"], r["cost"]["total_gae"], r["cost"]["s"],
                         r["outcome"]["matches_target"], r["counters"].get("targets_tried")])
        bj = os.path.join(T, f"band-{name}.json")
        txt = subprocess.run([EC, "band", "--dir", d, "--ranks", "--json", bj], capture_output=True, text=True).stdout
        open(os.path.join(T, f"band-{name}.txt"), "w").write(txt)
        res["band"][name] = json.load(open(bj))
    bj = os.path.join(T, f"band-n{n}-pooled.json")
    txt = subprocess.run([EC, "band", "--dir", *dirs, "--ranks", "--json", bj], capture_output=True, text=True).stdout
    open(os.path.join(T, f"band-n{n}-pooled.txt"), "w").write(txt)
    res["band"][f"n{n}-pooled"] = json.load(open(bj))
    with gzip.open(os.path.join(T, f"records-n{n}.csv.gz"), "wt") as f:
        f.write("rep,seq,round,arm,target_index,status,total_gae,S,matches_target,targets_tried\n")
        for x in rows: f.write(",".join(map(str, x)) + "\n")
    res["phases"][n] = {a: {k: v / cnt[a] for k, v in m.items()} for a, m in ph.items()}
    res["native"][n] = {a: {k: v / cnt[a] for k, v in m.items()} for a, m in nat.items()}
    res["wall"][n] = {a: {k: v / cnt[a] for k, v in m.items()} for a, m in wall.items()}
    res["status"][n] = {f"{a}:{s}:{m}": c for (a, s, m), c in status.items()}
    def same(a, b):
        ks = set(by_key[a]) | set(by_key[b])
        return {"records": len(ks), "identical": sum(1 for k in ks if by_key[a].get(k) == by_key[b].get(k))}
    res["identity"][n] = {"inc-off==inc": same(INC, "inc-off"), "inc-aa==inc": same(INC, "inc-aa")}
    res["same_walk"][n] = {c: {"pairs": len(tt[INC]), "same_targets_tried": sum(1 for k, v in tt[INC].items() if tt[c].get(k) == v),
                               "same_recovered": sum(1 for k, v in rec_d[INC].items() if rec_d[c].get(k) == v)} for c in (CAND,) + SINGLE}
    pairs = [(INC, "inc-aa"), (INC, "inc-off"), (INC, CAND), (INC, SINGLE[0]), (INC, SINGLE[1]), ("rho-strong", INC), ("rho-v5", INC),
             ("rho-strong", CAND), ("rho-v5", CAND), ("rho-strong", "rho-v5")]
    for a, b in pairs:
        for w in W:
            res["compare"][f"{n}|pooled|{b}/{a}|w={w}"] = cmp(dirs, a, b, w)
            if (a, b) == (INC, CAND):
                for rep, d in zip(REPS, dirs):
                    res["compare"][f"{n}|r{rep}|{b}/{a}|w={w}"] = cmp([d], a, b, w)
json.dump(res, open(os.path.join(HERE, "results.json"), "w"), indent=1, default=str)
print("wrote results.json")
