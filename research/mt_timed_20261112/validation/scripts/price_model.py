#!/usr/bin/env python3
"""VALIDATOR diagnostic (not pre-registered): does the measured opcost price list explain the measured solve-phase
times and the IC/rho ratios?  For every execution, predicted ns = I*inv + M*mul + S*sqr + H*lookup(table) +
N*native, with the opcost 'before' prices of that size (both 'before' and 'after' are reported), lookup(table) = the
std HashMap lookup price at the opcost table size nearest log2 of the run's own table (IC: 'table'; rho2xks/rhodpu:
'db'; bsgs2: 'baby'), and native = canon_hash_ns / 2 (the harness counts canonicalisation + hash as two native
operations).  L is 0 in every row.  Reported: measured/predicted per arm and size, and per configuration the
predicted vs measured icbest/rhobest ratio (keys as selected on measured medians).
usage: price_model.py <frozen study dir> <recompute.json> <out.json>"""
import json, sys, os, math, collections

FZ, RC, OUT = sys.argv[1:4]
H = os.path.join(FZ, "results/holdout")
rows = [json.loads(l) for l in open(os.path.join(H, "runs.jsonl"))]
op = [json.loads(l) for l in open(os.path.join(H, "opcost.jsonl"))]
rc = json.load(open(RC))


def med(xs):
    xs = sorted(xs); n = len(xs)
    return xs[n // 2] if n % 2 else 0.5 * (xs[n // 2 - 1] + xs[n // 2])


def prices(pb, when):
    o = [x for x in op if x["pb"] == pb and x["when"] == when][0]
    return o


def table_size(r):
    for k in ("table", "db", "baby"):
        if k in r and isinstance(r[k], (int, float)):
            return max(int(r[k]), 1)
    return 1


def predict(r, when):
    o = prices(r["pb"], when)
    lg = math.log2(table_size(r))
    tabs = sorted(o["tables"], key=lambda t: abs(t["log2_size"] - lg))
    lk = tabs[0]["std_lookup_ns"]
    ns = r["I"] * o["inv_ns"] + r["M"] * o["mul_ns"] + r["S"] * o["sqr_ns"] + r["H"] * lk + r["N"] * o["canon_hash_ns"] / 2
    return ns / 1e6, dict(field_ms=(r["I"] * o["inv_ns"] + r["M"] * o["mul_ns"] + r["S"] * o["sqr_ns"]) / 1e6, hash_ms=r["H"] * lk / 1e6,
                          native_ms=r["N"] * o["canon_hash_ns"] / 2e6, lookup_ns=lk, log2_table=lg)


out = dict(per_arm={}, per_cell=[], note=__doc__)
for when in ("before", "after"):
    acc = collections.defaultdict(list)
    for r in rows:
        p, _ = predict(r, when)
        acc[(r["arm"], r["pb"])].append(r["ms_arm"] / p)
    out["per_arm"][when] = {f"{a}/{pb}": dict(median_measured_over_predicted=med(v), n=len(v)) for (a, pb), v in sorted(acc.items())}
# per configuration: predicted ratio for the keys selected on measured medians
byk = collections.defaultdict(list)
for r in rows:
    byk[(r["pb"], r["T"], r["idx"], r["seed"], r["arm"], r["param"], r.get("doff"))].append(r)
cells = collections.defaultdict(list)
for e in rc["per_configuration"]:
    c = (e["bits"], e["T"], e["idx"], e["seed"])
    ki = tuple(e["ms_arm"]["ic_key"]); kr = tuple(e["ms_arm"]["rho_key"])
    ri = byk[c + ki][0]; rr = byk[c + kr][0]
    pi, di = predict(ri, "before"); pr, dr = predict(rr, "before")
    pi2, _ = predict(ri, "after"); pr2, _ = predict(rr, "after")
    cells[(e["bits"], e["T"])].append(dict(idx=e["idx"], measured=e["ms_arm"]["ic_rho"], predicted_before=pi / pr, predicted_after=pi2 / pr2,
                                            field_only=di["field_ms"] / dr["field_ms"], ic_parts=di, rho_parts=dr))
for (pb, T), v in sorted(cells.items()):
    out["per_cell"].append(dict(bits=pb, T=T, measured_median=med([x["measured"] for x in v]),
                                predicted_before_median=med([x["predicted_before"] for x in v]),
                                predicted_after_median=med([x["predicted_after"] for x in v]),
                                field_ops_only_median=med([x["field_only"] for x in v]),
                                ic_hash_share_median=med([x["ic_parts"]["hash_ms"] / (x["ic_parts"]["field_ms"] + x["ic_parts"]["hash_ms"] + x["ic_parts"]["native_ms"]) for x in v]),
                                rho_hash_share_median=med([x["rho_parts"]["hash_ms"] / (x["rho_parts"]["field_ms"] + x["rho_parts"]["hash_ms"] + x["rho_parts"]["native_ms"]) for x in v]),
                                ic_log2_table_median=med([x["ic_parts"]["log2_table"] for x in v]),
                                rho_log2_table_median=med([x["rho_parts"]["log2_table"] for x in v]),
                                configs=v))
json.dump(out, open(OUT, "w"), indent=1)
for c in out["per_cell"]:
    print(c["bits"], c["T"], "measured %.3f predicted(before) %.3f (after) %.3f field-only %.3f | IC hash share %.2f rho hash share %.2f | log2 tab ic %.1f rho %.1f" % (
        c["measured_median"], c["predicted_before_median"], c["predicted_after_median"], c["field_ops_only_median"], c["ic_hash_share_median"],
        c["rho_hash_share_median"], c["ic_log2_table_median"], c["rho_log2_table_median"]))
for k, v in out["per_arm"]["before"].items():
    print(k, "%.2f" % v["median_measured_over_predicted"], end="; ")
print()
