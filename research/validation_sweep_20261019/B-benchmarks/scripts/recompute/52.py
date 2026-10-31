import csv, statistics, os, re
H = "/home/user/crypto/ecc2k130/benchmarks/sigma-table/headline"
rows = list(csv.DictReader(open(os.path.join(H, "samples.tsv")), delimiter="\t"))
print("sample rows", len(rows), "phases", sorted(set(r["phase"] for r in rows)))
by = {}
for r in rows:
    by.setdefault((r["phase"], r["pair"]), {})[r["variant"]] = float(r["rateMps"])
ab = sorted((k for k in by if k[0] == "ab"), key=lambda k: int(k[1]))
aa = sorted((k for k in by if k[0] == "aa"), key=lambda k: int(k[1]))
print("ab pairs", len(ab), "aa pairs", len(aa))
ratios = [by[k]["candidate"] / by[k]["control"] for k in ab if "candidate" in by[k] and "control" in by[k]]
print("ab ratios (candidate/control)", [round(x, 7) for x in ratios])
print("ab median ratio", round(statistics.median(ratios), 7), "min", round(min(ratios), 7), "max", round(max(ratios), 7))
cand = [by[k]["candidate"] for k in ab]; ctrl = [by[k]["control"] for k in ab]
print("ab median candidate M/s", round(statistics.median(cand), 3), "median control M/s", round(statistics.median(ctrl), 3))
allc = [r["rateMps"] for r in rows if r["phase"] == "ab" and r["variant"] == "candidate"]
allk = [r["rateMps"] for r in rows if r["phase"] == "ab" and r["variant"] == "control"]
print("all ab candidate n", len(allc), "median", round(statistics.median(map(float, allc)), 3), "all ab control n", len(allk), "median", round(statistics.median(map(float, allk)), 3))
# A/A drift: pairs of control vs control
aa_r = []
for k in aa:
    vals = list(by[k].values())
    if len(vals) == 2: aa_r.append(vals[0] / vals[1] - 1.0)
print("aa pairs with two values", len(aa_r), "max abs drift", round(max(abs(x) for x in aa_r), 9) if aa_r else None)
print("aa variants", sorted(set(r["variant"] for r in rows if r["phase"] == "aa")))
res = __import__("json").load(open(os.path.join(H, "result.json")))
print("result.json ab_median_ratio", res["ab_median_ratio"], "ab_min", res["ab_min_ratio"], "aa drift", res["aa_max_absolute_drift"], "control M/s", res["control_median_million_per_second"], "candidate M/s", res["candidate_median_million_per_second"])
print("result ab_ratios", [round(x, 7) for x in res["ab_ratios"]])
# verification counts
for f in ["verify-control.log", "verify-candidate.log"]:
    txt = open(os.path.join(H, f)).read()
    print(f, "finished lines", len(re.findall(r"finished:", txt)), "300 verified lines", len(re.findall(r"300 verified against the reference, 0 dropped", txt)))
