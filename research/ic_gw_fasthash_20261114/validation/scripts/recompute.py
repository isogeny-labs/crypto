# Validator recompute of the ic_gw_fasthash_20261114 headline from raw JSONL.
# Written from PROTOCOL.md only (analyze.py not read or imported).
# usage: python3 -I recompute.py RAW_DIR UNITS_TSV OUT_JSON
import json, math, os, sys
from collections import defaultdict

raw, units_tsv, out_json = sys.argv[1], sys.argv[2], sys.argv[3]
PH = ["fb", "relations", "matrix", "linear_algebra", "log_recovery", "descent", "verify"]
Q = [262111, 524287, 1048507, 2097091, 4194271]
SEEDS = [1, 2, 3]


def counted(rec):
    return {k: v for k, v in rec.items()
            if not (k.endswith("wall") or k.endswith("cpu") or k == "load1" or k.startswith("s.controls"))}


def load(path):
    return [json.loads(l) for l in open(path) if l.strip()]


units = []
for line in open(units_tsv):
    if not line.strip():
        continue
    p, arm, b, q, s = line.split()
    units.append((int(p), arm, b, int(q), int(s)))

problems = []
data = {}
for (p, arm, b, q, s) in units:
    f = os.path.join(raw, "units", f"p{p}_{arm}_{q}_{s}.jsonl")
    if not os.path.exists(f):
        problems.append(f"missing {f}")
        continue
    recs = load(f)
    data[(p, arm, q, s)] = recs
    ur = [r for r in recs if r["rec"] == "unit"]
    rr = [r for r in recs if r["rec"] == "rss"]
    if len(ur) != 1 or ur[0]["rc"] != 0 or ur[0]["arm"] != arm or ur[0]["q"] != q or ur[0]["seed"] != s or ur[0]["pass"] != p or ur[0]["bin"] != b:
        problems.append(f"unit record {f}: {ur}")
    if len(rr) != 1 or rr[0]["rc"] != 0:
        problems.append(f"rss record {f}: {rr}")

# extra files in the tarball not in units.tsv
listed = {f"p{p}_{arm}_{q}_{s}.jsonl" for (p, arm, b, q, s) in units}
present = {x for x in os.listdir(os.path.join(raw, "units")) if x.endswith(".jsonl")}
extra = sorted(present - listed)

# gate: counted fields of every ic record, in order, vs p1 REF of the same (q, seed)
gate_mm, verify_fail, retried = [], [], []
n_cmp_units = n_cmp_recs = 0
for (p, arm, q, s), recs in sorted(data.items()):
    ics = [r for r in recs if r["rec"] == "ic"]
    for r in ics:
        if not (r["ok"] is True and r["k_found"] == r["k"] and r["fail"] == ""):
            verify_fail.append((p, arm, q, s, r["arm"], r["attempt"]))
        if r["attempt"] != 1:
            retried.append((p, arm, q, s, r["arm"], r["attempt"]))
    if (p, arm) == (1, "REF"):
        continue
    ref = [counted(r) for r in data[(1, "REF", q, s)] if r["rec"] == "ic"]
    me = [counted(r) for r in ics]
    n_cmp_units += 1
    n_cmp_recs += len(me)
    if len(ref) != len(me):
        gate_mm.append((p, arm, q, s, "record count"))
    for i, (a, b) in enumerate(zip(ref, me)):
        if json.dumps(a) != json.dumps(b):  # full-line: same keys, same order, same values
            gate_mm.append((p, arm, q, s, i, sorted(k for k in set(a) | set(b) if a.get(k) != b.get(k))))


def dl(recs, field):
    return sum(r[field] for r in recs if r["rec"] == "ic" and r["arm"] == "DL")


def dl_total(recs):
    return sum(r[ph + ".cpu"] for r in recs if r["rec"] == "ic" and r["arm"] == "DL" for ph in PH)


def rss(recs):
    return [r for r in recs if r["rec"] == "rss"][0]["vmhwm_kb"]


def gm(xs):
    return math.exp(sum(math.log(x) for x in xs) / len(xs))


prim, tot, rssr = {}, {}, {}
for arm in ["HASH", "COMBO"]:
    for q in Q:
        for s in SEEDS:
            r = [dl(data[(p, arm, q, s)], "relations.cpu") / dl(data[(p, "REF", q, s)], "relations.cpu") for p in (1, 2)]
            prim[(arm, q, s)] = {"pass1": r[0], "pass2": r[1], "mean": (r[0] + r[1]) / 2}
            tot[(arm, q, s)] = [dl_total(data[(p, arm, q, s)]) / dl_total(data[(p, "REF", q, s)]) for p in (1, 2)]
            rssr[(arm, q, s)] = [rss(data[(p, arm, q, s)]) / rss(data[(p, "REF", q, s)]) for p in (1, 2)]

aa = {}
for q in (2097091, 4194271):
    r = [dl(data[(p, "REF2", q, 1)], "relations.cpu") / dl(data[(p, "REF", q, 1)], "relations.cpu") for p in (1, 2)]
    aa[q] = {"pass1": r[0], "pass2": r[1], "mean": (r[0] + r[1]) / 2,
             "total_cpu": [dl_total(data[(p, "REF2", q, 1)]) / dl_total(data[(p, "REF", q, 1)]) for p in (1, 2)]}
floor = max(abs(v[k] - 1) for v in aa.values() for k in ("pass1", "pass2", "mean"))

verdict = {}
for arm in ["HASH", "COMBO"]:
    vals = [v for (a, q, s), d in prim.items() if a == arm for v in (d["pass1"], d["pass2"], d["mean"])]
    means = [d["mean"] for (a, q, s), d in prim.items() if a == arm]
    singles = [v for (a, q, s), d in prim.items() if a == arm for v in (d["pass1"], d["pass2"])]
    below1 = all(v < 1 for v in vals)
    beyond = all(v < 1 - floor for v in vals)
    verdict[arm] = {
        "decision": "SUPPORTED" if beyond else ("below 1 but not beyond the A/A floor" if below1 else "NOT SUPPORTED"),
        "all_below_1": below1, "all_below_1_minus_floor": beyond,
        "mean_min": min(means), "mean_max": max(means),
        "worst_single_pass": max(singles), "best_single_pass": min(singles),
        "geomean_per_size": {q: gm([prim[(arm, q, s)]["mean"] for s in SEEDS]) for q in Q},
        "pass_mean_range_per_size": {q: [min(prim[(arm, q, s)]["mean"] for s in SEEDS), max(prim[(arm, q, s)]["mean"] for s in SEEDS)] for q in Q},
        "total_cpu_geomean_per_size": {q: gm([x for s in SEEDS for x in tot[(arm, q, s)]]) for q in Q},
        "total_cpu_minmax_per_size": {q: [min(x for s in SEEDS for x in tot[(arm, q, s)]), max(x for s in SEEDS for x in tot[(arm, q, s)])] for q in Q},
        "rss_geomean_per_size": {q: gm([x for s in SEEDS for x in rssr[(arm, q, s)]]) for q in Q},
        "singles_above_1_minus_floor": [(q, s, p, prim[(arm, q, s)][p]) for q in Q for s in SEEDS for p in ("pass1", "pass2") if prim[(arm, q, s)][p] >= 1 - floor],
    }

loads = [float(r["load1"]) for recs in data.values() for r in recs if r["rec"] == "unit"]
res = {
    "units_listed": len(units), "units_loaded": len(data), "extra_files": extra, "problems": problems,
    "gate": {"units_compared": n_cmp_units, "records_compared": n_cmp_recs, "mismatches": gate_mm,
             "verify_failures": verify_fail, "retried": retried, "pass": not gate_mm and not verify_fail and not problems},
    "primary": {f"{a}/{q}/{s}": v for (a, q, s), v in prim.items()},
    "aa": {str(q): v for q, v in aa.items()}, "aa_floor": floor, "verdict": verdict,
    "load1": [min(loads), max(loads)],
}
json.dump(res, open(out_json, "w"), indent=1, default=str)
print(json.dumps({"gate": res["gate"], "aa": res["aa"], "aa_floor": floor, "verdict": verdict, "problems": problems, "extra": extra, "load1": res["load1"]}, indent=1, default=str))
