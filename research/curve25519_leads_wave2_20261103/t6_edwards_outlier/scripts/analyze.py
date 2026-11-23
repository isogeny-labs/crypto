#!/usr/bin/env python3
"""T6 analysis: applies the frozen PROTOCOL.md decision rules to data/*.json.
Written after the freeze (not frozen); it only reads data and prints/writes analysis/summary.json."""
import json, os, statistics as st, sys

D = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")
DATA = os.path.join(D, "data")
ORIG = 2026111505
FRESH = [3110300000 + i for i in range(1, 7)]
FRESH3 = FRESH[:3]


def load(name):
    p = os.path.join(DATA, name + ".json")
    if not os.path.exists(p) or os.path.getsize(p) == 0:
        return None
    return json.loads(open(p).read().strip().splitlines()[-1])


def ok(d):
    return d is not None and d["targets_done"] == d["targets_requested"] and not d["capped"] and d["undecided"] == 0


excluded = []


def R(a, b, label):
    if not ok(a) or not ok(b):
        excluded.append(label)
        return None
    return a["muladds_per_target"] / b["muladds_per_target"]


out = {"cells": {}, "excluded": excluded}
for f in sorted(os.listdir(DATA)):
    if f.endswith(".json") and ".try1." not in f:
        d = load(f[:-5])
        out["cells"][f[:-5]] = {k: d.get(k) for k in (
            "muladds_per_target", "columns", "covered_points", "coverage_exact", "solving_degree_max",
            "poly_terms_per_specialisation", "calls_per_target", "f4_calls", "undecided", "capped",
            "targets_done", "targets_requested", "hits", "hits_verified",
            "muladds_per_relation_at_exact_coverage", "native", "solver_extra", "wall_s")} if d else None

# ---- Block A
A = {}
for s in [ORIG] + FRESH:
    b = load(f"A_k20_bits_t5_g0_s{s}")
    e = load(f"A_k20_edwards_t5_g0_s{s}")
    tr = load(f"A_k20_bits976503_t5_g0_s{s}")
    A[s] = dict(bits=b and b["muladds_per_target"], edw=e and e["muladds_per_target"],
                tr=tr and tr["muladds_per_target"], R=R(e, b, f"A R s{s}"), Rtr=R(tr, b, f"A Rtr s{s}"))
out["A"] = A
ref = dict(bits=127284344.31, edw=82230218.94, tr=127284307.74)
o = A[ORIG]
p0_dev = {k: (abs(o[k] - v) / v if o[k] else None) for k, v in ref.items()}
P0 = all(x is not None and x <= 1e-3 for x in p0_dev.values())
fr = [A[s]["R"] for s in FRESH if A[s]["R"] is not None]
n_rep = sum(r <= 0.90 for r in fr)
n_van = sum(0.97 <= r <= 1.03 for r in fr)
P1 = "replicates" if n_rep >= 5 else ("vanishes" if n_van >= 5 else "mixed/inconclusive")
rtr = [A[s]["Rtr"] for s in [ORIG] + FRESH if A[s]["Rtr"] is not None]
P2 = len(rtr) == 7 and all(0.99 <= r <= 1.01 for r in rtr)

# ---- Block B
B = {}
for k in (16, 18, 22, 24):
    rs = []
    for s in FRESH3:
        b = load(f"B_k{k}_bits_t5_g0_s{s}"); e = load(f"B_k{k}_edwards_t5_g0_s{s}")
        r = R(e, b, f"B k{k} s{s}")
        B.setdefault(k, {})[s] = dict(bits=b and b["muladds_per_target"], edw=e and e["muladds_per_target"], R=r,
                                      bcols=b and b["columns"], ecols=e and e["columns"],
                                      bcov=b and b["covered_points"], ecov=e and e["covered_points"],
                                      bdeg=b and b["solving_degree_max"], edeg=e and e["solving_degree_max"])
        if r is not None: rs.append(r)
    B[k]["median"] = st.median(rs) if rs else None
out["B"] = B
meds = [B[k]["median"] for k in (16, 18, 22, 24)]
n_le = sum(m is not None and m <= 0.90 for m in meds)
n_band = sum(m is not None and 0.97 <= m <= 1.03 for m in meds)
P3 = "holds" if n_le >= 3 else ("k-specific" if n_le <= 1 and n_le + n_band == 4 else "mixed")

# ---- Block C
C = {0: {s: A[s]["R"] for s in FRESH3}}
for g in (1, 2, 3):
    C[g] = {}
    for s in FRESH3:
        b = load(f"C_k20_bits_t5_g{g}_s{s}"); e = load(f"C_k20_edwards_t5_g{g}_s{s}")
        C[g][s] = R(e, b, f"C g{g} s{s}")
Cmed = {g: st.median([v for v in C[g].values() if v is not None]) for g in range(4)}
p4a = all(C[3][s] is not None and 0.98 <= C[3][s] <= 1.02 for s in FRESH3)
p4b = all(Cmed[g] <= Cmed[g + 1] + 0.01 for g in range(3))
out["C"] = {"R": C, "median": Cmed}
Craw = {}
for g in (1, 2, 3):
    for s in FRESH3:
        b = load(f"C_k20_bits_t5_g{g}_s{s}"); e = load(f"C_k20_edwards_t5_g{g}_s{s}")
        Craw[f"g{g}_s{s}"] = (b and b["muladds_per_target"], e and e["muladds_per_target"])
out["C"]["raw"] = Craw

# ---- Block D
Dd = {}
for s in FRESH3:
    b = load(f"D_k20_bits_t4_g0_s{s}"); e = load(f"D_k20_edwards_t4_g0_s{s}")
    Dd[f"t4_s{s}"] = dict(bits=b and b["muladds_per_target"], edw=e and e["muladds_per_target"], R=R(e, b, f"D t4 s{s}"))
b = load("D_k20_bits_t6_g0_s3110300001"); e = load("D_k20_edwards_t6_g0_s3110300001")
Dd["t6_s3110300001"] = dict(bits=b and b["muladds_per_target"], edw=e and e["muladds_per_target"], R=R(e, b, "D t6"))
P5 = all(v["R"] is not None and 0.97 <= v["R"] <= 1.03 for v in Dd.values())
out["D"] = Dd

# ---- Block E
E = []
for i in range(1, 41):
    s = 3110301000 + i
    b = load(f"E_k20_bits_t5_g0_n1_s{s}"); e = load(f"E_k20_edwards_t5_g0_n1_s{s}")
    E.append(dict(seed=s, bits=b["muladds_per_target"] if ok(b) else None,
                  edw=e["muladds_per_target"] if ok(e) else None,
                  bdeg=b and b["solving_degree_max"], edeg=e and e["solving_degree_max"],
                  bcalls=b and b["calls_per_target"], ecalls=e and e["calls_per_target"],
                  bsteps=b and b["solver_extra"].get("steps"), esteps=e and e["solver_extra"].get("steps"),
                  bref=b and b["solver_extra"].get("refutations"), eref=e and e["solver_extra"].get("refutations"),
                  bhits=b and b["hits"], ehits=e and e["hits"]))
bv = [x["bits"] for x in E if x["bits"] is not None]
ev = [x["edw"] for x in E if x["edw"] is not None]
cv = lambda v: st.pstdev(v) / st.mean(v)
mb = st.median(bv)
p6a = cv(bv) <= 0.02
cheap = [x for x in E if x["edw"] is not None and x["edw"] <= 0.8 * mb]
near = [x for x in E if x["edw"] is not None and abs(x["edw"] / mb - 1) <= 0.05]
mixture = len(cheap) >= 0.2 * len(ev) and len(near) >= 0.2 * len(ev)
uniform = cv(ev) <= 0.05 and st.median(ev) / mb <= 0.90
cheap_low = sum(1 for x in cheap if x["edeg"] is not None and x["edeg"] <= 5)
p6c = ((cheap_low >= 0.8 * len(cheap)) if cheap else None) if mixture else "not judged (P6(b) mixture did not hold)"
out["E"] = dict(rows=E, bits_cv=cv(bv), edw_cv=cv(ev), bits_median=mb, edw_median=st.median(ev),
                bits_mean=st.mean(bv), edw_mean=st.mean(ev), n_bits=len(bv), n_edw=len(ev),
                n_cheap=len(cheap), n_near=len(near), mixture=mixture, uniform=uniform,
                cheap_deg_le5=cheap_low,
                edw_distinct=sorted(set(round(v, 2) for v in ev)), bits_distinct=sorted(set(round(v, 2) for v in bv)))

out["verdicts"] = dict(P0=dict(reproduced=P0, rel_dev=p0_dev), P1=dict(outcome=P1, n_le_090=n_rep, n_in_band=n_van, n=len(fr)),
                       P2=dict(holds=P2, ratios=rtr), P3=dict(outcome=P3, n_pairs_valid_curves=sum(m is not None for m in meds), medians=meds, n_le_090=n_le, n_band=n_band),
                       P4=dict(holds=p4a and p4b, part_a=p4a, part_b=p4b), P5=dict(holds=P5),
                       P6=dict(a=p6a, b_mixture=mixture, b_uniform=uniform, c=p6c))
os.makedirs(os.path.join(D, "analysis"), exist_ok=True)
json.dump(out, open(os.path.join(D, "analysis", "summary.json"), "w"), indent=1, default=str)
json.dump(out["verdicts"], sys.stdout, indent=1, default=str)
print("\nexcluded:", excluded)
