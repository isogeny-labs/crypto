#!/usr/bin/env python3
"""VALIDATOR check of the producer's summary.json against the validator's recompute.json (field by field), plus the
extra diagnostics the RESULTS wording needs (per-arm fast/std, A/A deviation without the zero self-term, prediction
outcomes, host-replication per-cell differences).
usage: results_check.py <study dir> <recompute.json> <out.json>"""
import collections
import json
import math
import os
import statistics
import sys

STUDY, RC, OUT = sys.argv[1:4]
S = json.load(open(os.path.join(STUDY, "results/holdout/summary.json")))
R = json.load(open(RC))
cmp = []


def c(name, prod, mine, tol=1e-12):
    ok = prod is not None and mine is not None and abs(prod - mine) <= tol * max(1.0, abs(mine))
    cmp.append(dict(name=name, producer=prod, validator=mine, ok=ok))


for b in ("std", "fast"):
    pc = {(x["bits"], x["T"]): x for x in S[b]["cells"]}
    for x in R[b]["cells"]:
        p = pc[(x["bits"], x["T"])]
        tag = f"{b} {x['bits']}/{x['T']}"
        for f in ("ic_rho_solve", "ic_rho_all", "ic_bsgs_solve", "aa_rho"):
            c(f"{tag} {f} median", p[f]["median"], x[f]["median"])
            c(f"{tag} {f} lo", p[f]["lo"], x[f]["protocol_ci"][0])
            c(f"{tag} {f} hi", p[f]["hi"], x[f]["protocol_ci"][1])
        c(f"{tag} fixed median", p["fixed_ic_rho_solve"]["median"], x["fixed_ic_rho_solve"]["median"])
        c(f"{tag} fixed lo", p["fixed_ic_rho_solve"]["lo"], x["fixed_ic_rho_solve"]["protocol_ci"][0])
        c(f"{tag} fixed hi", p["fixed_ic_rho_solve"]["hi"], x["fixed_ic_rho_solve"]["protocol_ci"][1])
        dmap = {"cheaper": "faster", "dearer": "slower", "unranked": "unranked"}
        cmp.append(dict(name=f"{tag} decision", producer=p["verdict_solve"], validator=x["ic_rho_solve"]["decision"],
                        ok=dmap[p["verdict_solve"]] == x["ic_rho_solve"]["decision"]))
        cmp.append(dict(name=f"{tag} fixed keys", producer=p["fixed_keys"], validator=x["fixed_ic_rho_solve"]["keys"],
                        ok=p["fixed_keys"] == x["fixed_ic_rho_solve"]["keys"]))
    c(f"{b} aa median |log|", S[b].get("aa_median_abs_log_dev"), R[b]["drift"]["median_abs_log_dev"])
    c(f"{b} aa p95 |log|", S[b].get("aa_p95_abs_log_dev"), R[b]["drift"]["p95_abs_log_dev"])
    for po, vo in zip(S[b]["opcost"], R["opcost"]):
        c(f"{b} opcost {po['pb']} {po['when']} gae_ns", po["gae_ns"], vo["gae_ns"])
        c(f"{b} opcost {po['pb']} {po['when']} I/M", po["I_over_M"], vo["I_over_M"])
pf = {(x["bits"], x["T"]): x for x in S["fast_over_std"]}
for x in R["fast_over_std"]:
    p = pf[(x["bits"], x["T"])]
    for s in ("ic", "rho"):
        c(f"fast/std {x['bits']}/{x['T']} {s} median", p[s]["median"], x[s]["median"])
        c(f"fast/std {x['bits']}/{x['T']} {s} lo", p[s]["lo"], x[s]["protocol_ci"][0])
        c(f"fast/std {x['bits']}/{x['T']} {s} hi", p[s]["hi"], x[s]["protocol_ci"][1])

# ---------------------------------------------------------------- extra diagnostics from the raw table
J = json.load(open(os.path.join(STUDY, "results/holdout/runs.json")))
rows = [dict(zip(J["fields"], r)) for r in J["rows"]]
kt = collections.defaultdict(list)
for r in rows:
    kt[(r["build"], r["pb"], r["T"], r["idx"], r["arm"], r["param"], r["doff"])].append(r["ms_arm"])
# per-arm fast/std of key times (median over all configuration keys of that arm)
arm = collections.defaultdict(list)
for k, v in kt.items():
    if k[0] == "std":
        arm[k[4]].append(statistics.median(kt[("fast",) + k[1:]]) / statistics.median(v))
per_arm = {a: dict(median=statistics.median(v), lo=min(v), hi=max(v), n=len(v)) for a, v in sorted(arm.items())}
# A/A deviation without the zero term (with 3 repeats the median repeat is the median itself)
nz = []
for v in kt.values():
    m = statistics.median(v)
    d = sorted(abs(math.log(x / m)) for x in v)
    nz += d[1:]
nz.sort()
extra = dict(per_arm_fast_over_std=per_arm, aa_abs_log_dev_excluding_self_median=statistics.median(nz), aa_abs_log_dev_excluding_self_p95=nz[int(0.95 * len(nz))])
# host replication: RunPod std cell median / VM cell median
vm = {(x["bits"], x["T"]): x["median"] for x in R["earlier_vm_cells"]}
rp = {(x["bits"], x["T"]): x["ic_rho_solve"]["median"] for x in R["std"]["cells"]}
extra["runpod_std_over_vm"] = {f"{k[0]}/{k[1]}": rp[k] / vm[k] for k in sorted(vm)}
extra["cells_beyond_10pct_of_vm"] = [k for k, v in extra["runpod_std_over_vm"].items() if abs(v - 1) > 0.10]
# IC/rho change fast vs std per cell
extra["ic_rho_fast_minus_std"] = {f"{a['bits']}/{a['T']}": b["ic_rho_solve"]["median"] - a["ic_rho_solve"]["median"]
                                  for a, b in zip(R["std"]["cells"], R["fast"]["cells"])}
bad = [x for x in cmp if not x["ok"]]
out = dict(compared=len(cmp), mismatches=bad, extra=extra, comparisons=cmp)
json.dump(out, open(OUT, "w"), indent=1, default=str)
print("compared", len(cmp), "mismatches", len(bad))
for x in bad[:20]:
    print(" ", x)
print(json.dumps(extra, indent=1))
