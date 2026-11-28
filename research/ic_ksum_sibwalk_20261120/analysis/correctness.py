#!/usr/bin/env python3
"""Registered correctness check (PROTOCOL.md section 4). usage: python3 analysis/correctness.py <sessions dir> <out json> [names...]
For every (session, round, target): `off` equals `inc` in every deterministic field (outcome, cost, counters, workload,
algorithm seed, boundaries, and every phase's counts with its wall time removed): bit-identical default-off; `cand` equals `inc` in outcome status, recovered log, targets tried, relations
found, matrix rank, the oracle set-up phase, scalar multiplications and the stride counters, and its relation-phase
additions plus doublings equal the registered count model exactly. Reads records only; computes no cost ratio."""
import json, sys, os
D, OUT = sys.argv[1], sys.argv[2]
names = sys.argv[3:] or sorted(n for n in os.listdir(D) if os.path.isdir(os.path.join(D, n)))
CAND = os.environ.get("CAND", "cand")
STRIDE = ["stride_steps", "stride_restarts", "stride_renewals", "direct_relations_skipped"]


def ph(r, name):
    return next(p for p in r["phases"] if p["name"] == name)


DET = ["outcome", "cost", "counters", "workload", "algorithm_seed", "boundaries"]


def nowall(p):
    return {k: v for k, v in p.items() if k != "wall_ns"}


def strip(r):
    return {**{k: r.get(k) for k in DET}, "phases": [nowall(p) for p in r["phases"]]}


res = {}
for n in names:
    recs = {}
    for line in open(os.path.join(D, n, "records.jsonl")):
        r = json.loads(line)
        recs[(r["arm"], r["round"], r["warmup"], r["workload"]["target_index"])] = r
    keys = sorted(k[1:] for k in recs if k[0] == "inc")
    off_bad, cand_bad, model_bad, d_gae, lifts, pairs = [], [], [], [], 0, 0
    for k in keys:
        x, o, c = recs[("inc",) + k], recs[("off",) + k], recs[(CAND,) + k]
        if strip(x) != strip(o):
            off_bad.append(list(k))
        rx, rc = ph(x, "relations"), ph(c, "relations")
        nx, nc = rx["native"], rc["native"]
        same = (x["outcome"]["status"] == c["outcome"]["status"] and x["outcome"]["recovered"] == c["outcome"]["recovered"]
                and all(x["counters"].get(f) == c["counters"].get(f) for f in ("targets_tried", "relations_found", "matrix_rank"))
                and nowall(ph(x, "oracle_setup")) == nowall(ph(c, "oracle_setup")) and rx["scalar_mults"] == rc["scalar_mults"]
                and all(nx.get(f) == nc.get(f) for f in STRIDE))
        if not same:
            cand_bad.append(list(k))
        g = lambda f: nc.get(f, 0)
        model = (rx["adds"] + rx["doubles"]) - nx["stride_steps"] + g("walk_sibling_pairs") + 2 * g("walk_sibling_full_pairs") \
            + g("walk_sibling_centres") + g("walk_sibling_lifts") + g("walk_sibling_multiple_ops")
        if rc["adds"] + rc["doubles"] != model:
            model_bad.append(list(k))
        d_gae.append(x["cost"]["total_gae"] - c["cost"]["total_gae"])
        lifts += g("walk_sibling_lifts"); pairs += g("walk_sibling_pairs")
    res[n] = {"pairs": len(keys), "off_mismatches": len(off_bad), "cand_mismatches": len(cand_bad),
              "count_model_mismatches": len(model_bad), "first": (off_bad + cand_bad + model_bad)[:5],
              "gae_saved_mean": sum(d_gae) / len(d_gae), "gae_saved_min": min(d_gae), "gae_saved_max": max(d_gae),
              "sibling_pairs": pairs, "lifts": lifts}
json.dump(res, open(OUT, "w"), indent=1)
for n, v in res.items():
    print(n, f"{v['pairs']} pairs: off {v['off_mismatches']}, cand {v['cand_mismatches']}, model {v['count_model_mismatches']} mismatches;",
          f"GAE saved {v['gae_saved_min']:.0f}..{v['gae_saved_max']:.0f} (mean {v['gae_saved_mean']:.1f}); pairs {v['sibling_pairs']}, lifts {v['lifts']}")
