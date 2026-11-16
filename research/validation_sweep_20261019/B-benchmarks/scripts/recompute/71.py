# Row 71 recompute: 20261014-ic-improve-03-tracezero-freerel. Reads committed files only. Run from repo root with python3 -I.
import csv, json, statistics
from collections import defaultdict
D = "research/ic_improve_20261014/03-tracezero-freerel"
rows = list(csv.DictReader(open(D + "/data/rows.csv")))
print("rows:", len(rows), "arms:", sorted(set(r["arm"] for r in rows)), "sizes:", sorted(set(int(r["n"]) for r in rows)))
res = json.load(open(D + "/results.json"))
print("results top keys:", [k for k in res.keys()][:20])
print("replicates per size:", {n: sum(1 for r in rows if int(r["n"]) == n and r["arm"] == "A") for n in (41, 53, 61)})
def m(n, arm, col):
    return statistics.fmean(float(r[col]) for r in rows if int(r["n"]) == n and r["arm"] == arm)
sizes = (41, 53, 61)
for arm in sorted(set(r["arm"] for r in rows)):
    if arm == "A": continue
    per = []
    for n in sizes:
        if not any(int(r["n"]) == n and r["arm"] == arm for r in rows): continue
        t = m(n, arm, "c_total") / m(n, "A", "c_total")
        rk = m(n, arm, "c_rank") / m(n, "A", "c_rank")
        w = m(n, arm, "w_total") / m(n, "A", "w_total")
        per.append((n, round(t, 6), round(rk, 4), round(w, 4)))
    print("arm %s per size (n, counted_total, counted_rank, wall_total) vs A: %s" % (arm, per))
    if per:
        num = sum(m(n, arm, "c_total") for n in sizes if any(int(r["n"]) == n and r["arm"] == arm for r in rows))
        den = sum(m(n, "A", "c_total") for n in sizes if any(int(r["n"]) == n and r["arm"] == arm for r in rows))
        wn = sum(m(n, arm, "w_total") for n in sizes if any(int(r["n"]) == n and r["arm"] == arm for r in rows))
        wd = sum(m(n, "A", "w_total") for n in sizes if any(int(r["n"]) == n and r["arm"] == arm for r in rows))
        print("   pooled counted_total (sum of means) = %.6f ; pooled wall_total = %.4f" % (num / den, wn / wd))
# filter rejections and attempts
for arm in sorted(set(r["arm"] for r in rows)):
    fr = sum(int(float(r["filter_rejections"])) for r in rows if r["arm"] == arm)
    ra = sum(int(float(r["rank_attempts"])) for r in rows if r["arm"] == arm)
    print("arm %s: filter_rejections total %d ; rank_attempts total %d ; count rows %d" % (arm, fr, ra, sum(1 for r in rows if r["arm"] == arm)))
tt = [json.loads(x) for x in open(D + "/data/trace_theory.jsonl") if x.strip()]
print("trace_theory rows:", len(tt), "keys:", sorted(tt[0].keys())[:20] if tt else None)
# size-normalised pooling (the study's stored rule: each pair divided by its size's mean A; sums over all pairs)
def byk(n, arm): return {int(r["seed"]): r for r in rows if int(r["n"]) == n and r["arm"] == arm}
def snorm(arm, col):
    num = den = 0.0
    for n in sizes:
        A = byk(n, "A"); X = byk(n, arm)
        seeds = sorted(set(A) & set(X))
        ma = statistics.fmean(float(A[s][col]) for s in seeds)
        num += sum(float(X[s][col]) / ma for s in seeds)
        den += sum(float(A[s][col]) / ma for s in seeds)
    return num / den
for arm in ["R", "FR", "F"]:
    print("size-normalised %s counted_total=%.6f wall_total=%.6f" % (arm, snorm(arm, "c_total"), snorm(arm, "w_total")))
# free rank per size (arm R)
for n in sizes:
    fr = [int(float(r["free_rank"])) for r in rows if int(r["n"]) == n and r["arm"] == "R"]
    print("R free_rank n=%d mean=%.2f" % (n, statistics.fmean(fr)))
# A/A: inc vs A counted equality per seed
for n in sizes:
    A = byk(n, "A"); I = byk(n, "inc")
    seeds = sorted(set(A) & set(I))
    eq = sum(1 for s in seeds if A[s]["c_total"] == I[s]["c_total"] and A[s]["rank_attempts"] == I[s]["rank_attempts"] and A[s]["c_index"] == I[s]["c_index"])
    print("A/A inc vs A n=%d pairs=%d equal(counted_total, rank_attempts, c_index)=%d" % (n, len(seeds), eq))
# verified targets in one raw file
rec = [json.loads(x) for x in open(D + "/data/n61_K500_s15_R.jsonl") if x.strip()]
print("n61_K500_s15_R.jsonl target records:", len(rec), "group_verified true:", sum(1 for r in rec if r.get("group_verified")), "filter_rejected sum:", sum(int(r.get("filter_rejected") or 0) for r in rec))
print("PROTOCOL line 102:", open(D + "/PROTOCOL.md").read().split("\n")[101][:300])
