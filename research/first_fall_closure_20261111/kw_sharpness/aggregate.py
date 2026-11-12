import json, glob, collections
rows = []
for f in sorted(glob.glob("raw_*.jsonl")):
    rows += [json.loads(l) for l in open(f)]
rho = {r["nprime"]: r["rho"] for r in json.load(open("rho_generic.json"))}
cells = collections.defaultdict(list)
for r in rows: cells[(r["npr"], r["n"])].append(r)
def dist(vals):
    c = collections.Counter(vals); return " ".join("%s:%d" % (k, c[k]) for k in sorted(c, key=lambda x: (x is None, x)))
out = []
out.append("| n' | n | N=3n' | samples | rho(n') | dim V (all) | dim Vbar = dim E^3 | D_ff top-form (HPS/KW/CG) | D_ff operational (HKY closure) | nullity of top syzygies in deg 7 | 6n |")
out.append("|--:|--:|--:|--:|--:|--|--|--|--|--|--:|")
allok = True
for (npr, n) in sorted(cells):
    rs = cells[(npr, n)]
    assert all(r["dimVbar"] == r["dimE3"] for r in rs), "dim Vbar != dim E3"
    assert all(r["formula_ok"] for r in rs)
    nullset = [r["nullity7"] for r in rs if r["nullity7"] is not None]
    out.append("| %d | %d | %d | %d | %d | %s | %s | %s | %s | %s | %d |" % (npr, n, 3 * npr, len(rs), rho[npr] if npr in rho else -1,
        dist([r["dimV"] for r in rs]), dist([r["dimVbar"] for r in rs]), dist([r["Dff_top"] for r in rs]), dist([r["Dff_op"] for r in rs]),
        dist(nullset) if nullset else "-", 6 * n))
open("table_measured.md", "w").write("\n".join(out) + "\n")
print("\n".join(out))
# consistency checks used in the NOTE
bad = [r for r in rows if r["dimVbar"] == r["dimV"] and r["Dff_top"] != 7]
print("independent-top-form samples with D_ff_top != 7:", len(bad))
bad = [r for r in rows if r["dimVbar"] == r["dimV"] and r["nullity7"] < r["six_n"]]
print("independent samples with nullity < 6n:", len(bad))
eq = [(r["npr"], r["n"]) for r in rows if r["dimVbar"] == r["dimV"] and r["nullity7"] == r["six_n"]]
ne = sorted({(r["npr"], r["n"]) for r in rows if r["dimVbar"] == r["dimV"] and r["nullity7"] != r["six_n"]})
print("cells with some independent sample whose nullity != 6n:", ne)
print("total samples:", len(rows))
ind = [r for r in rows if r["dimVbar"] == r["dimV"]]
print("independent samples:", len(ind), " with op==7:", sum(1 for r in ind if r["Dff_op"] == 7))
dep = [r for r in rows if r["dimVbar"] < r["dimV"]]
print("dependent samples:", len(dep), " op distribution:", dist([r["Dff_op"] for r in dep]), " top distribution:", dist([r["Dff_top"] for r in dep]))
# frequency of full rank dimE3 == n for n'=3
print("n'=3 frequency of dim E^3 = n (top forms independent) vs random-matrix heuristic prod(1-2^(i-n)), i=0..min(n,rho)-1:")
for n in range(7, 13):
    rs = cells.get((3, n), [])
    if not rs: continue
    k = len(rs); f = sum(1 for r in rs if r["dimE3"] == n) / k
    rmax = min(n, 10); h = 1.0
    for i in range(rmax): h *= (1 - 2.0 ** (i - n)) if rmax == 10 or True else 1
    # probability that a random 10 x n... (n x 10 matrix) has rank min(n,10)
    import math
    if n >= 10:
        h = 1.0
        for i in range(10): h *= 1 - 2.0 ** (i - n)
    else:
        h = 1.0
        for i in range(n): h *= 1 - 2.0 ** (i - 10)
    print("  n=%d samples=%d  measured P(full rank min(n,10))=%.3f  heuristic %.3f   dimE3 distribution %s" % (n, k, sum(1 for r in rs if r["dimE3"] == min(n, 10)) / k, h, dist([r["dimE3"] for r in rs])))
# stage-one operational lower bound versus n + nullity (no Boolean syzygies of coefficient degree <= 1 iff equality)
eqc = neq = 0; neq_cells = set()
for r in ind:
    lg = r["op_log"][-1]
    if lg["method"] == "stage1" and lg["c"] == 7:
        if lg["low_lower_bound"] == r["n"] + r["nullity7"]: eqc += 1
        else: neq += 1; neq_cells.add((r["npr"], r["n"]))
print("independent samples: stage-one dim(W7 cap B<=6) lower bound == n + nullity7 :", eqc, " not equal:", neq, sorted(neq_cells))
# degrees of the s_k
print("degree sets of s_k over all samples:", dist([tuple(r["s_degrees"]) for r in rows]))
