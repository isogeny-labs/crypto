#!/usr/bin/env python3
"""Validator recompute of the NRM/LZ study headline from the producer's raw tarball (does not read or import the
producer's analyze.py). Model from PROTOCOL.md: per unit and arm (D, DL1, DL), sum over that arm's ic records (all
attempts) and phases fb, relations, matrix, linear_algebra, log_recovery, descent, verify of
mul + r_inv*inv + r_lmul*lmul + w_add*(add + ladd) + w_h*gae*(hash + cmp), with r_inv, r_lmul, gae frozen per q
(summary.json kind ic_size arm DL). Also: gates G0-G3, the decision, the k = 22 gap, stderr logs, and a cell-by-cell
comparison with the producer's analysis.json.
usage: recompute.py RUNS_TGZ SUMMARY_JSONL VALIDATION_JSON ANALYSIS_JSON OUT_JSON"""
import json, math, re, sys, tarfile
tgz, summ, valj, anj, outp = sys.argv[1:6]
PH = ["fb", "relations", "matrix", "linear_algebra", "log_recovery", "descent", "verify"]
Q = [262111, 524287, 1048507, 2097091, 4194271]
SEEDS = range(1, 7)
ARMS = ["D", "DL1", "DL"]
XS = ["NRM", "LZ", "BOTH"]
WA = [0.0, 0.5, 1.0]
WH = [0.0, 0.1, 1.0]
K = {262111: 18, 524287: 19, 1048507: 20, 2097091: 21, 4194271: 22}

const = {}
for line in open(summ):
    line = line.strip()
    if not line:
        continue
    r = json.loads(line)
    if r.get("kind") == "ic_size" and r.get("arm") == "DL" and r["q"] in Q:
        const[r["q"]] = (r["r_inv"], r["r_lmul"], r["gae"])
assert sorted(const) == Q

units, errs, other = {}, {}, []
with tarfile.open(tgz) as tf:
    for m in tf.getmembers():
        if not m.isfile():
            continue
        name = m.name[2:] if m.name.startswith("./") else m.name
        data = tf.extractfile(m).read().decode()
        mm = re.fullmatch(r"units/(REF|A0|NRM|LZ|BOTH)_(\d+)_(\d+)\.jsonl(\.err)?", name)
        if not mm:
            other.append(name)
            continue
        key = (mm.group(1), int(mm.group(2)), int(mm.group(3)))
        if mm.group(4):
            errs[key] = data
        else:
            units[key] = [json.loads(l) for l in data.splitlines() if l.strip()]

def counted(r):
    return [k for k in r if not (k.endswith("wall") or k.endswith("cpu") or k == "load1" or k.startswith("s.controls"))]

def ic(key):
    return [r for r in units[key] if r.get("rec") == "ic"]

def meq(recs, arm, q, wa, wh):
    ri, rl, gae = const[q]
    t = 0.0
    for r in recs:
        if r["arm"] != arm:
            continue
        for p in PH:
            t += (r[p + ".mul"] + ri * r[p + ".inv"] + rl * r[p + ".lmul"] + wa * (r[p + ".add"] + r[p + ".ladd"])
                  + wh * gae * (r[p + ".hash"] + r[p + ".cmp"]))
    return t

out = {"constants": {str(q): dict(zip(["r_inv", "r_lmul", "gae"], const[q])) for q in Q},
       "files": {"units": len(units), "err": len(errs), "other": sorted(other)}}
expected = {(x, q, s) for x in ["REF", "A0"] + XS for q in Q for s in SEEDS}
out["files"]["all_150_present"] = set(units) == expected and set(errs) == expected

# ---- gates
allowed = {"REF": set(), "A0": set(),
           "NRM": {(a, f) for a in ARMS for f in ("relations.mul", "relations.add")},
           "LZ": {(a, f) for a in ("D", "DL1") for f in ("linear_algebra.lmul", "linear_algebra.ladd")}}
allowed["BOTH"] = allowed["NRM"] | allowed["LZ"]
g = {"G0": [], "G1": [], "G2": [], "G3": []}
changed = {x: 0 for x in XS}
nfields = 0
for q in Q:
    for s in SEEDS:
        R = ic(("REF", q, s))
        for x in ["A0"] + XS:
            X = ic((x, q, s))
            gate = "G0" if x == "A0" else "G1"
            if [(r["arm"], r["attempt"]) for r in R] != [(r["arm"], r["attempt"]) for r in X]:
                g[gate].append(f"{x} {q} {s}: record arms/attempts differ")
                continue
            ch = False
            for r, y in zip(R, X):
                if counted(r) != counted(y):
                    g[gate].append(f"{x} {q} {s} {r['arm']}: counted key lists differ")
                for k in counted(r):
                    nfields += 1
                    if r[k] != y.get(k):
                        if (r["arm"], k) in allowed[x]:
                            ch = True
                        else:
                            g[gate].append(f"{x} {q} {s} {r['arm']} {k}: {r[k]} vs {y.get(k)}")
                    if k.startswith("s.") and r[k] != y.get(k):
                        g["G3"].append(f"{x} {q} {s} {r['arm']} {k}")
            if ch:
                changed[x] += 1
        for x in ["REF", "A0"] + XS:
            recs = units[(x, q, s)]
            u = [r for r in recs if r.get("rec") == "unit"]
            if len(u) != 1 or u[0]["rc"] != 0:
                g["G2"].append(f"{x} {q} {s}: unit rc {u}")
            for r in ic((x, q, s)):
                if not (r["ok"] is True and r["k_found"] == r["k"]):
                    g["G2"].append(f"{x} {q} {s} {r['arm']} attempt {r['attempt']}: ok={r['ok']} fail={r['fail']}")
out["gates"] = {k: {"pass": not v, "mismatches": v} for k, v in g.items()}
out["gates"]["fields_compared"] = nfields
out["gates"]["units_with_allowed_changes"] = changed
attempts = sorted({(r["arm"], r["attempt"]) for key in units for r in ic(key)})
out["record_arm_attempts_seen"] = attempts

# ---- stderr logs, wall time masked
wall = re.compile(r"wall=[0-9.]+s")
logdiff = []
for q in Q:
    for s in SEEDS:
        ref = wall.sub("wall=*", errs[("REF", q, s)])
        for x in ["A0"] + XS:
            if wall.sub("wall=*", errs[(x, q, s)]) != ref:
                logdiff.append(f"{x} {q} {s}")
out["stderr_logs_equal_mod_wall"] = {"pass": not logdiff, "diff": logdiff}

# ---- ratios
cells = {}
for x in XS:
    for a in ARMS:
        for q in Q:
            for wa in WA:
                for wh in WH:
                    ps = []
                    for s in SEEDS:
                        ps.append(meq(ic((x, q, s)), a, q, wa, wh) / meq(ic(("REF", q, s)), a, q, wa, wh))
                    gm = math.exp(sum(math.log(v) for v in ps) / len(ps))
                    cells[f"{x}/{a}/{q}/{wa}/{wh}"] = {"geomean": gm, "max": max(ps), "min": min(ps), "per_seed": ps}
out["ratios"] = cells

# ---- decision
dec = {}
for x in XS:
    dec[x] = {}
    for a in ARMS:
        prim = [v for q in Q for wa in WA for v in cells[f"{x}/{a}/{q}/{wa}/0.0"]["per_seed"]]
        band = [v for q in Q for wa in WA for wh in WH for v in cells[f"{x}/{a}/{q}/{wa}/{wh}"]["per_seed"]]
        gates_ok = all(out["gates"][k]["pass"] for k in ("G0", "G1", "G2", "G3"))
        dec[x][a] = {"improvement": all(v < 1 for v in prim) and gates_ok, "all_cells_below_1": all(v < 1 for v in prim),
                     "band_below_1": all(v < 1 for v in band), "max_ratio": max(prim), "n_cells": len(prim)}
out["decision"] = dec

# ---- component ratios (geomean over seeds of per-unit DL record sums)
def comp(x, arm, field, q):
    ps = []
    for s in SEEDS:
        a = sum(r[field] for r in ic((x, q, s)) if r["arm"] == arm)
        b = sum(r[field] for r in ic(("REF", q, s)) if r["arm"] == arm)
        ps.append(a / b)
    return math.exp(sum(math.log(v) for v in ps) / len(ps))
out["relations_mul_ratio_DL"] = {x: {str(q): comp(x, "DL", "relations.mul", q) for q in Q} for x in ("NRM", "BOTH")}
out["la_lmul_ratio"] = {f"{x}/{a}": {str(q): comp(x, a, "linear_algebra.lmul", q) for q in Q} for x in ("LZ", "BOTH") for a in ("D", "DL1")}

# ---- k = 22 gap
vj = json.load(open(valj))
band = vj["rho_reference"]["gae_band"]
gap = {"ic_mean_meq": {}, "rho": {w: band[w]["rho"] for w in ("0", "0.1", "1")}, "ic_over_rho": {}, "rho_over_ic": {}}
for x in ("REF", "BOTH", "NRM", "LZ"):
    for a in ("DL", "DL1", "D"):
        for w in ("0", "0.1", "1"):
            v = sum(meq(ic((x, 4194271, s)), a, 4194271, 0.0, float(w)) for s in (1, 2, 3)) / 3
            gap["ic_mean_meq"][f"{x}/{a}/{w}"] = v
            gap["ic_over_rho"][f"{x}/{a}/{w}"] = v / band[w]["rho"]
            gap["rho_over_ic"][f"{x}/{a}/{w}"] = band[w]["rho"] / v
gap["ref_reproduces_band"] = {w: {"ref": gap["ic_mean_meq"][f"REF/DL/{w}"], "band_ic": band[w]["ic"],
                                  "rel_err": abs(gap["ic_mean_meq"][f"REF/DL/{w}"] - band[w]["ic"]) / band[w]["ic"]} for w in ("0", "0.1", "1")}
gap["order_holds_across_band_BOTH_DL_vs_rho"] = all(gap["ic_over_rho"][f"BOTH/DL/{w}"] < 1 for w in ("0", "0.1", "1"))
out["gap_k22"] = gap

# ---- compare with the producer's analysis.json
an = json.load(open(anj))
md, worst = 0.0, None
miss = []
for k, v in cells.items():
    c = an["ratios"].get(k)
    if c is None:
        miss.append(k)
        continue
    for f in ("geomean", "max", "min"):
        d = abs(c[f] - v[f]) / abs(v[f])
        if d > md:
            md, worst = d, (k, f)
    for a, b in zip(c["per_seed"], v["per_seed"]):
        d = abs(a - b) / abs(b)
        if d > md:
            md, worst = d, (k, "per_seed")
extra = [k for k in an["ratios"] if k not in cells]
cmpd = {"cells": len(cells), "missing_in_analysis": miss, "extra_in_analysis": extra, "max_rel_diff": md, "worst": worst}
dd = []
for x in XS:
    for a in ARMS:
        for f in ("improvement", "all_cells_below_1", "band_below_1"):
            if an["decision"][x][a][f] != dec[x][a][f]:
                dd.append(f"{x}/{a}/{f}")
        if abs(an["decision"][x][a]["max_ratio"] - dec[x][a]["max_ratio"]) > 1e-12:
            dd.append(f"{x}/{a}/max_ratio")
cmpd["decision_diffs"] = dd
gd = []
for k, v in an["gap_k22"]["ic_mean_meq"].items():
    m = gap["ic_mean_meq"][k]
    if abs(v - m) / m > 1e-12:
        gd.append((k, v, m))
cmpd["gap_ic_mean_diffs"] = gd
cmpd["gates_claimed"] = {k: an["gates"][k]["pass"] for k in ("G0", "G1", "G2", "G3")}
cmpd["changed_claimed"] = an["gates"]["G1"].get("changed")
cmpd["relations_mul_ratio_maxdiff"] = max(abs(an["relations_mul_ratio"][x][q] - out["relations_mul_ratio_DL"][x][q]) for x in ("NRM", "BOTH") for q in out["relations_mul_ratio_DL"][x])
cmpd["lmul_ratio_maxdiff"] = max(abs(an["lmul_ratio"][xa][q] - out["la_lmul_ratio"][xa][q]) for xa in out["la_lmul_ratio"] for q in out["la_lmul_ratio"][xa])
out["vs_analysis_json"] = cmpd
json.dump(out, open(outp, "w"), indent=1)
print(json.dumps({"files": out["files"], "gates": {k: (v.get("pass") if isinstance(v, dict) and "pass" in v else v) for k, v in out["gates"].items()},
                  "logs": out["stderr_logs_equal_mod_wall"]["pass"], "decision": dec, "vs_analysis": {k: v for k, v in cmpd.items()}}, indent=1))
