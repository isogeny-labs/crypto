import json, glob, math, os, re, collections, statistics
RUN = "/home/user/crypto/research/ic_improve_20261015/06-genus3-kappa-bw/results/run"
ent = []   # one entry per (line, result)
for p in sorted(glob.glob(os.path.join(RUN, "b*_c*_*.jsonl"))):
    for l in open(p):
        if not l.strip(): continue
        r = json.loads(l)
        for x in r.get("results", []):
            ent.append(dict(bits=r["bits"], curve=r["curve"], q=r["q"], file=os.path.basename(p), arm=x["arm"],
                            base=re.sub(r"@\d+", "", x["arm"]), cost=x.get("cost"), la=x.get("la_cost"), correct=x.get("correct")))
print("result entries", len(ent), "files", len(set(e["file"] for e in ent)))
print("correct True", sum(1 for e in ent if e["correct"]), "of", len(ent))
print("arm labels", sorted(set(e["arm"] for e in ent)))
def cell(bits, curve, arm, field="cost"):
    v = [e[field] for e in ent if e["bits"] == bits and e["curve"] == curve and e["arm"] == arm]
    return v[0] if len(v) == 1 else (None if not v else ("multi", v))
def gm_ratio(bits, cand, field="cost", inc="main:lpsgewd"):
    per = []
    for c in sorted(set(e["curve"] for e in ent if e["bits"] == bits)):
        a = cell(bits, c, cand, field); b = cell(bits, c, inc, field)
        if isinstance(a, (int, float)) and isinstance(b, (int, float)):
            per.append(a / b)
    return round(math.exp(statistics.fmean(map(math.log, per))), 4) if per else None, len(per)
print("bits 21 curves", sorted(set(e["curve"] for e in ent if e["bits"] == 21)), "arms", sorted(set(e["arm"] for e in ent if e["bits"] == 21)))
for name, arm, field in [("BW2 whole @21", "main:lpsgebw2@8", "cost"), ("BW2 LA @21", "main:lpsgebw2@8", "la"),
                         ("schedule whole @21", "sched:lpsgewd@16", "cost"), ("combination whole @21", "sched:lpsgebw2@16", "cost"),
                         ("BW3 whole @21", "main:lpsgebw3@8", "cost")]:
    print(name, arm, gm_ratio(21, arm, field))
# OLS exponents, pooled over solves, base names, bits 12..21
def slope(v):
    n = len(v); mx = sum(a for a, _ in v) / n; my = sum(b for _, b in v) / n
    return sum((a - mx) * (b - my) for a, b in v) / sum((a - mx) ** 2 for a, _ in v), n
pts = collections.defaultdict(list)
for e in ent:
    if e["cost"] and e["bits"] <= 21:
        pts[e["base"]].append((math.log(e["q"]), math.log(e["cost"])))
inc_s, _ = slope(pts["main:lpsgewd"])
for base in ["main:lpsgebw2", "sched:lpsgewd", "sched:lpsgebw2", "aa:lpsgewd", "main:lpsgebw3"]:
    if base in pts:
        s, n = slope(pts[base]); print("exponent Delta 12..21", base, round(s - inc_s, 6), "n", n)
print("schedule whole @21 (sched:lpsgewd@8)", gm_ratio(21, "sched:lpsgewd@8", "cost"))
print("combination whole @21 (sched:lpsgebw2@8)", gm_ratio(21, "sched:lpsgebw2@8", "cost"))
print("BW3 whole @21", gm_ratio(21, "main:lpsgebw3@8", "cost"))
# paired A/A: aa:lpsgewd vs main:lpsgewd, pairs by (bits, curve), bits 12..21
pairs = collections.defaultdict(dict)
for e in ent:
    if e["cost"] and e["bits"] <= 21 and e["base"] in ("main:lpsgewd", "aa:lpsgewd"):
        pairs[(e["bits"], e["curve"])][e["base"]] = (math.log(e["q"]), math.log(e["cost"]))
both = [v for v in pairs.values() if len(v) == 2]
si, _ = slope([v["main:lpsgewd"] for v in both]); sa, n = slope([v["aa:lpsgewd"] for v in both])
print("A/A exponent Delta 12..21 paired", round(sa - si, 6), "n pairs", n)
# local exponent 20 -> 21 (mean ln cost over curves at each size, curves present at that size)
def level(bits, arm):
    v = [(math.log(e["q"]), math.log(e["cost"])) for e in ent if e["bits"] == bits and e["arm"] == arm and e["cost"]]
    return statistics.fmean(a for a, _ in v), statistics.fmean(b for _, b in v), len(v)
for arm in ["main:lpsgewd", "sched:lpsgewd@8"]:
    x20, y20, n20 = level(20, arm); x21, y21, n21 = level(21, arm)
    print("local exponent 20-21", arm, round((y21 - y20) / (x21 - x20), 4), "n", n20, n21)
# logs: count of correct:true lines in progress logs and per-record checks
import subprocess
pl = open(os.path.join(RUN, "progress.log")).read().splitlines()
print("progress.log lines", len(pl), "with correct true", sum(1 for l in pl if '"correct":true' in l))
print("result entries correct true", sum(1 for e in ent if e["correct"] is True))
