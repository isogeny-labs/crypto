#!/usr/bin/env python3
"""AGENTS.md §5a pricing band for the native-XOR SAT arm of ecbench_sat_pdp count-qualified-main.

Reads the committed session records only (no new measurement). For every measured,
non-warm-up record of the SAT arm and of the matched strong rho it adds w GAE per unpriced
native operation (SAT: solver_ops = CDCL conflicts; rho: canonicalisations, partition
hashes, table inserts and table queries) at w in {0, 0.1, 1} and reports the ratio of the
mean S. It also derives the break-even w and a same-host wall-time price per conflict.

  python3 analyze.py RECORDS.jsonl > band.json
"""
import json, math, statistics, sys

RHO_UNPRICED = ("canonicalisations_uncharged", "partition_hashes_uncharged",
                "table_inserts_uncharged", "table_queries_uncharged")

def load(path):
    out = {}
    for line in open(path):
        r = json.loads(line)
        if r.get("warmup") or r["outcome"]["status"] != "verified":
            continue
        out.setdefault(r["arm"], []).append(r)
    return out

def unpriced(r):
    if r["arm"] == "ic-sat-xor":
        return sum(p["native"].get("solver_ops", 0) for p in r["phases"])
    if r["arm"].startswith("rho"):
        return sum(r["counters"].get(k, 0) for k in RHO_UNPRICED)
    return 0

def main(path):
    arms = load(path)
    sqrt_r = {a: rs[0]["cost"]["total_gae"] / rs[0]["cost"]["s"] for a, rs in arms.items()}
    res = {"source": path, "n": {a: len(v) for a, v in arms.items()},
           "sqrt_r": sqrt_r["rho-strong"], "band": {}}
    gae = {a: sum(r["cost"]["total_gae"] for r in rs) for a, rs in arms.items()}
    u = {a: sum(unpriced(r) for r in rs) for a, rs in arms.items()}
    res["sum_total_gae"] = gae; res["sum_unpriced_ops"] = u
    for w in (0.0, 0.1, 1.0):
        row = {}
        for a in arms:
            row[a] = (gae[a] + w * u[a]) / len(arms[a]) / sqrt_r[a]
        row["sat_over_rho"] = row["ic-sat-xor"] / row["rho-strong"]
        row["frobmitm_over_rho"] = row["ic-mitm-frob"] / row["rho-strong"]
        res["band"][str(w)] = row
    # break-even: gae_sat + w u_sat = gae_rho + w u_rho (same 40 runs per arm)
    res["break_even_w"] = (gae["rho-strong"] - gae["ic-sat-xor"]) / (u["ic-sat-xor"] - u["rho-strong"])
    # rho's native ops at the top of the band (1 GAE), SAT conflicts free to vary
    res["break_even_w_conflict_rho_w1"] = (gae["rho-strong"] + u["rho-strong"] - gae["ic-sat-xor"]) / u["ic-sat-xor"]
    # same-host wall prices (descriptive, L0 host): ns per conflict from the SAT relations phase,
    # ns per GAE from phases that are pure group arithmetic in the IC arms
    sat = arms["ic-sat-xor"]
    per_conf = [sum(p["wall_ns"] for p in r["phases"] if p["name"] == "relations") /
                max(1, unpriced(r)) for r in sat]
    agg_conf = sum(sum(p["wall_ns"] for p in r["phases"] if p["name"] == "relations") for r in sat) / u["ic-sat-xor"]
    ns_gae = {}
    for a in ("ic-mitm", "ic-mitm-frob", "ic-sat-xor"):
        for ph in ("oracle_setup", "verify"):
            g = sum(p["gae"] for r in arms[a] for p in r["phases"] if p["name"] == ph)
            ns = sum(p["wall_ns"] for r in arms[a] for p in r["phases"] if p["name"] == ph)
            if g > 100:
                ns_gae[f"{a}.{ph}"] = ns / g
    lo, hi = min(ns_gae.values()), max(ns_gae.values())
    res["wall"] = {"ns_per_conflict_aggregate": agg_conf,
                   "ns_per_conflict_median_record": statistics.median(per_conf),
                   "ns_per_gae": ns_gae,
                   "implied_gae_per_conflict": [agg_conf / hi, agg_conf / lo]}
    for tag, w in (("implied_lo", agg_conf / hi), ("implied_hi", agg_conf / lo)):
        s = (gae["ic-sat-xor"] + w * u["ic-sat-xor"]) / 40 / sqrt_r["ic-sat-xor"]
        rho = (gae["rho-strong"] + 1.0 * u["rho-strong"]) / 40 / sqrt_r["rho-strong"]
        res["wall"][tag + "_sat_over_rho_at_rho_w1"] = s / rho
    json.dump(res, sys.stdout, indent=1); print()

if __name__ == "__main__":
    main(sys.argv[1])
