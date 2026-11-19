#!/usr/bin/env python3
"""Build the gate artifacts for one study. Usage: mk.py STUDY_DIR KIND  (KIND = fewcol | sib)
Runs from the repository root. Reads own-numbers.json (recompute.py), ecbench band/compare --json (the producer tool's
full-precision output, the 'claimed' side) and replay receipts."""
import json, hashlib, subprocess, sys, os, glob, gzip
ECB = "target/release/ecbench"
std, kind = sys.argv[1:3]
val = sys.argv[3] if len(sys.argv) > 3 else os.path.join(std, "validation")
own = json.load(open(os.path.join(val, "own-numbers.json")))["sessions"]
def sha(p): return hashlib.sha256(open(p, "rb").read()).hexdigest()
if kind == "fewcol":
    pairs = [("ksum5-sib", "ksum5-ps"), ("ksum5-sib", "tw2l"), ("rho-neg", "tw2l"), ("rho-neg", "ksum5-ps"), ("tw2l", "tw2l-aa")]
    bitop = False
elif kind == "abl":
    pairs = [("rho-neg", "sib8-p256"), ("rho-neg", "tw2l-sib"), ("tw2l-sib", "sib8-p256"), ("tw2l", "tw2l-sg"), ("tw2l", "tw2l-sib")]
    bitop = True
else:
    pairs = [("tw2l", "sib8-p256"), ("tw2l", "sib8-p512"), ("rho-neg", "sib8-p256"), ("rho-neg", "sib8-p512"), ("rho-neg", "tw2l"), ("tw2l", "tw2l-aa")]
    bitop = True
def run(args):
    return subprocess.run([ECB] + args, capture_output=True, text=True, check=True).stdout
head, claims = [], {}
for ses in ("P20", "P22", "P24"):
    o = own[ses]; d = f"{std}/sessions/{ses}"
    tmp = f"/tmp/claude-0/v/band-{kind}-{ses}.json"
    run(["band", "--dir", d, "--json", tmp])
    band = json.load(open(tmp))
    c = {"band": {}, "pairs": {}}
    for row in band["rows"]:
        c["band"][row["arm"]] = row["mean_s"]
        for i, w in enumerate(["0", "0.1", "1"]):
            head.append({"name": f"{ses}/S/{row['arm']}/w{w}", "claimed": row["mean_s"][i], "recomputed": o["S"][row["arm"]][i]})
    c["ranks"] = [(r["a"], r["b"], r["verdict"]) for r in band["ranks"]]
    for a, b in pairs:
        cj = json.loads(run(["compare", "--dir", d, "--a", a, "--b", b, "--json"]))
        ops = cj["ops"]; mine = o["pairs"][f"{a}|{b}"]["w"]
        ent = {"raw_ratio": ops["ratio_b_over_a"], "raw_ci95": ops["ci95"], "band_ratio": ops["band"]["ratio_b_over_a"], "band_order": ops["band"]["order"]}
        n = f"{ses}/{b}_over_{a}"
        head.append({"name": n + "/raw_gae", "claimed": ops["ratio_b_over_a"], "recomputed": mine[3]["ratio"]})
        head.append({"name": n + "/raw_gae_ci_lo", "claimed": ops["ci95"][0], "recomputed": mine[3]["lo"]})
        head.append({"name": n + "/raw_gae_ci_hi", "claimed": ops["ci95"][1], "recomputed": mine[3]["hi"]})
        for i, w in enumerate(["0", "0.1", "1"]):
            head.append({"name": n + f"/band_w{w}", "claimed": ops["band"]["ratio_b_over_a"][i], "recomputed": mine[i]["ratio"]})
        if bitop:
            bj = json.loads(run(["compare", "--dir", d, "--a", a, "--b", b, "--unit", "bitop/v1", "--json"]))["ops"]
            ent["bitop_ratio"] = bj["ratio_b_over_a"]; ent["bitop_ci95"] = bj["ci95"]
            head.append({"name": n + "/bitop", "claimed": bj["ratio_b_over_a"], "recomputed": mine[4]["ratio"]})
            head.append({"name": n + "/bitop_ci_lo", "claimed": bj["ci95"][0], "recomputed": mine[4]["lo"]})
            head.append({"name": n + "/bitop_ci_hi", "claimed": bj["ci95"][1], "recomputed": mine[4]["hi"]})
        c["pairs"][f"{a}|{b}"] = ent
    claims[ses] = c
json.dump(claims, open(os.path.join(val, "ecbench-claimed.json"), "w"), indent=1, sort_keys=True)
mx = 0.0
for h in head:
    sc = max(abs(h["claimed"]), abs(h["recomputed"]), 1e-300); h["rel_diff"] = abs(h["claimed"]-h["recomputed"])/sc; mx = max(mx, h["rel_diff"])
json.dump({"headline": head, "max_rel_diff": mx}, open("/tmp/claude-0/v/head-%s.json" % kind, "w"))
print(kind, "headlines", len(head), "max rel diff", mx)
bad = [h for h in head if h["rel_diff"] > 1e-9]
print("over 1e-9:", len(bad)); [print(" ", h["name"], h["claimed"], h["recomputed"], h["rel_diff"]) for h in bad[:15]]
