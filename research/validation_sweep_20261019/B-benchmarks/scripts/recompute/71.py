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
