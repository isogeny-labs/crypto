#!/usr/bin/env python3
"""Compare the validator's replay rows (out/replay/*.jsonl) with the committed rows at COMMIT.
A run row must equal its committed row in every field except wall_ns (both directions: the same key
set, then the same values). Instance rows must be equal. Cell rows are compared without wall fields.
Usage: compare_replay.py REPO REPLAY_DIR OUT_JSON [COMMIT]"""
import glob, gzip, json, os, subprocess, sys
REPO, RD, OUT = sys.argv[1:4]
COMMIT = sys.argv[4] if len(sys.argv) > 4 else "6213ecbc0da0ed4917ab75805258201d6629b477"
D = "research/genus4_followup_20261118"


def blob(path):
    raw = subprocess.run(["git", "-C", REPO, "show", f"{COMMIT}:{path}"], check=True, capture_output=True).stdout
    txt = gzip.decompress(raw).decode() if path.endswith(".gz") else raw.decode()
    return [json.loads(l) for l in txt.splitlines() if l.strip()]


tracked = subprocess.run(["git", "-C", REPO, "ls-tree", "-r", "--name-only", COMMIT], check=True,
                         capture_output=True).stdout.decode().splitlines()
src = {"A": {cc: blob(f"{D}/a_cc{cc}.jsonl.gz") for cc in ("0.5", "0.7", "1.0", "1.4", "2.0")},
       "B": blob(f"{D}/b_wx.jsonl.gz"),
       "Q": blob(f"{D}/b_q.jsonl.gz") + blob(f"{D}/b_q.jsonl.r1.gz")
       + [r for f in sorted(t for t in tracked if t.startswith("research/genus4_lean_rho_20261116/p16411") and t.endswith(".jsonl"))
          for r in blob(f)]}


def index(rows):
    out = {}
    for r in rows:
        if r["kind"] == "run":
            out.setdefault((r["arm"], r["p"], r["curve_seed"], r["target_seed"], r["seed"]), []).append(r)
        elif r["kind"] == "instance":
            out.setdefault(("instance", r["p"], r["curve_seed"]), []).append(r)
    return out


nowall = lambda r: {k: v for k, v in r.items() if k != "wall_ns"}
results, ok_all = [], True
for f in sorted(glob.glob(os.path.join(RD, "*.jsonl"))):
    name = os.path.basename(f)
    rows = [json.loads(l) for l in open(f) if l.strip()]
    cfg = next(r for r in rows if r["kind"] == "config")
    if name.startswith("a_cc"):
        ref = index(src["A"][f"{cfg['cc']:.1f}"])
    elif name.startswith("b_"):
        ref = index(src["B"])
    else:
        ref = index(src["Q"])
    for r in rows:
        if r["kind"] == "run":
            k = (r["arm"], r["p"], r["curve_seed"], r["target_seed"], r["seed"])
        elif r["kind"] == "instance":
            k = ("instance", r["p"], r["curve_seed"])
        else:
            continue
        cands = ref.get(k, [])
        same = bool(cands) and all(nowall(c) == nowall(r) for c in cands)
        diff = [] if same or not cands else sorted(set(map(str, nowall(cands[0]).items())) ^ set(map(str, nowall(r).items())))[:8]
        ok_all &= same
        results.append({"file": name, "key": list(k), "committed_rows": len(cands), "identical_except_wall_ns": same,
                        "fields_compared": len(nowall(r)) if r["kind"] == "run" else None, "diff": diff,
                        "certified": r.get("certified") if r["kind"] == "run" else None,
                        "ok": r.get("ok") if r["kind"] == "run" else None})
runs = [x for x in results if x["key"][0] != "instance"]
summary = {"replay_dir": RD, "commit": COMMIT, "run_rows": len(runs), "instance_rows": len(results) - len(runs),
           "identical": sum(x["identical_except_wall_ns"] for x in runs), "all_identical": ok_all,
           "by_arm": {}, "rows": results}
for x in runs:
    a = x["key"][0]
    s = summary["by_arm"].setdefault(a, {"rows": 0, "identical": 0, "p": set(), "cc_files": set()})
    s["rows"] += 1
    s["identical"] += x["identical_except_wall_ns"]
    s["p"].add(x["key"][1])
    if x["file"].startswith("a_cc"):
        s["cc_files"].add(x["file"].split("_")[1])
for s in summary["by_arm"].values():
    s["p"], s["cc_files"] = sorted(s["p"]), sorted(s["cc_files"])
json.dump(summary, open(OUT, "w"), indent=1)
print(json.dumps({k: v for k, v in summary.items() if k != "rows"}, indent=1))
for x in results:
    if not x["identical_except_wall_ns"]:
        print("MISMATCH", x)
