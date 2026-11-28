#!/usr/bin/env python3
"""Compare the validator's recomputed numbers with the numbers printed in each study README, at the README's own precision.
Usage: readme_check.py OUT.json ; run from the repository root. Reads validation/own-numbers.json of each study."""
import json, sys, math, gzip
R1 = "research/ic_tableb_fewcol_20261118"; R2 = "research/ic_tableb_sib_20261119"
def load(p): return json.load(open(p + "/validation/own-numbers.json" if not p.endswith("ablation") else R2 + "/validation/ablation/own-numbers.json"))["sessions"]
out = []; 
def chk(study, what, shown, mine, dec):
    ok = abs(shown - mine) <= 0.5 * 10 ** (-dec) + 1e-9
    out.append({"study": study, "what": what, "readme": shown, "recomputed": mine, "decimals": dec, "match": ok})
def pr(o, ses, a, b, i): return o[ses]["pairs"][f"{a}|{b}"]["w"][i]
# ---------------- study 1
o = load(R1); S = lambda ses, arm, i: o[ses]["S"][arm][i]
tab1 = {  # ses: (ps raw [r,lo,hi], ps w0.1, ps w1, tw raw [r,lo,hi], tw w0.1, tw w1, tw/rho (w0,w0.1,w1))
 "P20": ((0.890, .886, .894), .902, .953, (.475, .453, .498), .448, .321, (2.18, 2.13, 1.84)),
 "P22": ((0.904, .901, .906), .915, .961, (.448, .430, .467), .422, .306, (2.12, 2.08, 1.88)),
 "P24": ((0.923, .921, .926), .933, .971, (.414, .389, .440), .389, .283, (2.14, 2.12, 2.03))}
for ses, t in tab1.items():
    p = pr(o, ses, "ksum5-sib", "ksum5-ps", 3); chk("fewcol", f"{ses} ps/sib 'w=0' (recorded S)", t[0][0], p["ratio"], 3); chk("fewcol", f"{ses} ps/sib 'w=0' lo", t[0][1], p["lo"], 3); chk("fewcol", f"{ses} ps/sib 'w=0' hi", t[0][2], p["hi"], 3)
    chk("fewcol", f"{ses} ps/sib w=0.1", t[1], pr(o, ses, "ksum5-sib", "ksum5-ps", 1)["ratio"], 3); chk("fewcol", f"{ses} ps/sib w=1", t[2], pr(o, ses, "ksum5-sib", "ksum5-ps", 2)["ratio"], 3)
    p = pr(o, ses, "ksum5-sib", "tw2l", 3); chk("fewcol", f"{ses} tw2l/sib 'w=0' (recorded S)", t[3][0], p["ratio"], 3); chk("fewcol", f"{ses} tw2l/sib lo", t[3][1], p["lo"], 3); chk("fewcol", f"{ses} tw2l/sib hi", t[3][2], p["hi"], 3)
    chk("fewcol", f"{ses} tw2l/sib w=0.1", t[4], pr(o, ses, "ksum5-sib", "tw2l", 1)["ratio"], 3); chk("fewcol", f"{ses} tw2l/sib w=1", t[5], pr(o, ses, "ksum5-sib", "tw2l", 2)["ratio"], 3)
    for i in range(3): chk("fewcol", f"{ses} tw2l/rho-neg band w{i}", t[6][i], pr(o, ses, "rho-neg", "tw2l", i)["ratio"], 2)
for arm, (a, b) in {"ksum5-sib": (5.18, 13.89), "ksum5-ps": (4.78, 13.49), "tw2l": (2.18, 3.93), "rho-neg": (1.02, 1.93)}.items():
    chk("fewcol", f"P24 S {arm} w=0", a, S("P24", arm, 0), 2); chk("fewcol", f"P24 S {arm} w=1", b, S("P24", arm, 2), 2)
# prose ranges (min/max over sizes)
def rng(vals): return (min(vals), max(vals))
for what, vals, shown in [
  ("tw2l/sib recorded-S range", [pr(o, s, "ksum5-sib", "tw2l", 3)["ratio"] for s in o], (0.41, 0.48)),
  ("tw2l/sib w=1 range", [pr(o, s, "ksum5-sib", "tw2l", 2)["ratio"] for s in o], (0.28, 0.32)),
  ("ps cut at w=0 (band) 8-11%", [1 - pr(o, s, "ksum5-sib", "ksum5-ps", 0)["ratio"] for s in o], (0.08, 0.11)),
  ("ps cut at w=1 (band) 3-5%", [1 - pr(o, s, "ksum5-sib", "ksum5-ps", 2)["ratio"] for s in o], (0.03, 0.05)),
  ("ps distance to rho, w=0 band, '4.0-4.7x'", [pr(o, s, "rho-neg", "ksum5-ps", 0)["ratio"] for s in o], (4.0, 4.7)),
  ("ps distance to rho, recorded S", [pr(o, s, "rho-neg", "ksum5-ps", 3)["ratio"] for s in o], (4.0, 4.7)),
  ("sib distance to rho w=0 band '4.6-5.3x'", [S(s, "ksum5-sib", 0)/S(s, "rho-neg", 0) for s in o], (4.6, 5.3)),
  ("sib distance to rho w=0 recorded S", [pr(o, s, "rho-neg", "ksum5-sib", 3)["ratio"] if "rho-neg|ksum5-sib" in o[s]["pairs"] else None for s in o], (4.6, 5.3)),
  ("sib distance to rho w=1 band '6.3-6.7x'", [S(s, "ksum5-sib", 2)/S(s, "rho-neg", 2) for s in o], (6.3, 6.7)),
  ("tw2l distance to rho w=0 '2.1-2.2x'", [pr(o, s, "rho-neg", "tw2l", 0)["ratio"] for s in o], (2.1, 2.2)),
  ("tw2l distance to rho w=1 '1.8-2.0x'", [pr(o, s, "rho-neg", "tw2l", 2)["ratio"] for s in o], (1.8, 2.0))]:
    if any(v is None for v in vals): continue
    lo, hi = rng(vals); out.append({"study": "fewcol", "what": what + " (prose range)", "readme": list(shown), "recomputed": [lo, hi], "match": abs(lo-shown[0]) <= 0.0501 and abs(hi-shown[1]) <= 0.0501 if shown[0] > 1 else abs(lo-shown[0]) <= 0.0051 and abs(hi-shown[1]) <= 0.0051})
# S-a and adds
cnt = {}
for ses in ("P20", "P22", "P24"):
    ident = n = 0; adds = {"ksum5-sib": [], "ksum5-ps": []}; per = {}
    for l in open(f"{R1}/sessions/{ses}/records.jsonl"):
        r = json.loads(l)
        if r["warmup"] or r["arm"] not in adds: continue
        key = (r["workload"]["workload_id"], r["round"])
        per.setdefault(key, {})[r["arm"]] = r
        adds[r["arm"]].append(sum(p["adds"] for p in r["phases"] if p["name"] == "oracle_setup"))
    for k, v in per.items():
        if len(v) == 2:
            n += 1
            a, b = v["ksum5-sib"]["counters"], v["ksum5-ps"]["counters"]
            ident += (a.get("trials") == b.get("trials") and a.get("relations") == b.get("relations") and v["ksum5-sib"]["workload"]["target"] == v["ksum5-ps"]["workload"]["target"])
    cnt[ses] = {"pairs": n, "identical_trials_relations": ident, "mean_adds_sib": sum(adds["ksum5-sib"])/len(adds["ksum5-sib"]), "mean_adds_ps": sum(adds["ksum5-ps"])/len(adds["ksum5-ps"])}
out.append({"study": "fewcol", "what": "S-a identity and mean oracle_setup-phase adds per run (README: 144/144; adds 2,424->1,738; 4,266->3,290; 7,106->5,768)", "recomputed": cnt})
# ---------------- study 2
o = load(R2); S = lambda ses, arm, i: o[ses]["S"][arm][i]
tab2 = {"P20": ((0.480, .459, .502), .540, .880, .532, (1.108, .994, 1.236), 1.73, (1.234, 1.106, 1.377)),
        "P22": ((0.441, .417, .467), .504, .835, .504, (0.956, .861, 1.063), 1.61, (1.088, .978, 1.209)),
        "P24": ((0.471, .442, .502), .547, .932, .550, (0.934, .844, 1.033), 1.74, (1.072, .969, 1.185))}
for ses, t in tab2.items():
    p = pr(o, ses, "tw2l", "sib8-p256", 3); chk("sib", f"{ses} sib8-p256/tw2l 'w=0' (recorded S)", t[0][0], p["ratio"], 3); chk("sib", f"{ses} lo", t[0][1], p["lo"], 3); chk("sib", f"{ses} hi", t[0][2], p["hi"], 3)
    chk("sib", f"{ses} sib8-p256/tw2l w=0.1", t[1], pr(o, ses, "tw2l", "sib8-p256", 1)["ratio"], 3); chk("sib", f"{ses} sib8-p256/tw2l w=1", t[2], pr(o, ses, "tw2l", "sib8-p256", 2)["ratio"], 3)
    chk("sib", f"{ses} sib8-p256/tw2l bitop", t[3], pr(o, ses, "tw2l", "sib8-p256", 4)["ratio"], 3)
    p = pr(o, ses, "rho-neg", "sib8-p256", 3); chk("sib", f"{ses} sib8-p256/rho-neg 'w=0'", t[4][0], p["ratio"], 3); chk("sib", f"{ses} lo", t[4][1], p["lo"], 3); chk("sib", f"{ses} hi", t[4][2], p["hi"], 3)
    chk("sib", f"{ses} sib8-p256/rho-neg w=1", t[5], pr(o, ses, "rho-neg", "sib8-p256", 2)["ratio"], 2)
    p = pr(o, ses, "rho-neg", "sib8-p256", 4); chk("sib", f"{ses} sib8-p256/rho-neg bitop", t[6][0], p["ratio"], 3); chk("sib", f"{ses} lo", t[6][1], p["lo"], 3); chk("sib", f"{ses} hi", t[6][2], p["hi"], 3)
for ses, vals in {"P20": {"rho-neg": (1.29, 2.41), "tw2l": (2.97, 4.74), "sib8-p512": (1.66, 4.25), "sib8-p256": (1.42, 4.17)},
                  "P22": {"rho-neg": (1.17, 2.24), "tw2l": (2.52, 4.32), "sib8-p512": (1.30, 3.89), "sib8-p256": (1.11, 3.61)},
                  "P24": {"rho-neg": (1.09, 2.06), "tw2l": (2.15, 3.86), "sib8-p512": (1.15, 3.90), "sib8-p256": (1.01, 3.59)}}.items():
    for arm, (a, b) in vals.items(): chk("sib", f"{ses} S {arm} w=0", a, S(ses, arm, 0), 2); chk("sib", f"{ses} S {arm} w=1", b, S(ses, arm, 2), 2)
chk("sib", "P24 sib8-p512/tw2l w=1 band ratio ('1.010')", 1.010, pr(o, "P24", "tw2l", "sib8-p512", 2)["ratio"], 3)
# ablation
oa = load(R2 + "/ablation")
tabA = {"P20": ((.802, 1.092), (.683, .800), (.605, .817), (1.075, .990, 1.168), 1.66, (1.197, 1.101, 1.300)),
        "P22": ((.684, 1.010), (.775, .868), (.672, .870), (.972, .885, 1.068), 1.65, (1.106, 1.006, 1.215)),
        "P24": ((.616, 1.004), (.849, .916), (.755, .916), (.891, .802, .990), 1.67, (1.024, .921, 1.137))}
for ses, t in tabA.items():
    for (a, b), tt in (("tw2l", "tw2l-sib"), t[0]), (("tw2l", "tw2l-sg"), t[1]), (("tw2l-sib", "sib8-p256"), t[2]):
        chk("ablation", f"{ses} {b}/{a} band w0", tt[0], pr(oa, ses, a, b, 0)["ratio"], 3); chk("ablation", f"{ses} {b}/{a} band w1", tt[1], pr(oa, ses, a, b, 2)["ratio"], 3)
    p = pr(oa, ses, "rho-neg", "sib8-p256", 3); chk("ablation", f"{ses} sib8-p256/rho-neg raw", t[3][0], p["ratio"], 3); chk("ablation", f"{ses} lo", t[3][1], p["lo"], 3); chk("ablation", f"{ses} hi", t[3][2], p["hi"], 3)
    chk("ablation", f"{ses} sib8-p256/rho-neg band w1", t[4], pr(oa, ses, "rho-neg", "sib8-p256", 2)["ratio"], 2)
    p = pr(oa, ses, "rho-neg", "sib8-p256", 4); chk("ablation", f"{ses} bitop sib8-p256/rho-neg", t[5][0], p["ratio"], 3); chk("ablation", f"{ses} lo", t[5][1], p["lo"], 3); chk("ablation", f"{ses} hi", t[5][2], p["hi"], 3)
json.dump(out, open(sys.argv[1], "w"), indent=1)
bad = [x for x in out if x.get("match") is False]
print(len(out), "checks;", len(bad), "mismatches")
for x in bad: print(json.dumps(x))
print(json.dumps(out[-1 if False else [i for i,x in enumerate(out) if 'S-a' in x['what']][0]], indent=0))
