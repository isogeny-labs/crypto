#!/usr/bin/env python3
"""Build the Route-3 append-only JSONL store (one record per measured id).

Records are written once and never rewritten; re-running appends only ids that
are not already present, so the file is a durable, replayable index of every
measurement behind the memo.
"""
import json
import os
from pathlib import Path

HERE = Path(__file__).resolve().parent
STORE = HERE / "route3_store.jsonl"
SHA = "1ae5170f9b3362f75a636909e1f5d01fb0ead0fd4886627d6012e2b9de9d8d08"


def bench_verdict(path, group):
    for line in Path(path).read_text().splitlines():
        item = json.loads(line)
        if item["id"] == f"{group}:verdict":
            return item
    raise SystemExit(f"no verdict for {group} in {path}")


def main():
    m2 = bench_verdict(HERE / "m2_m3_13bit_20261185.jsonl", "m2_ref")
    m3 = bench_verdict(HERE / "m2_m3_13bit_20261185.jsonl", "m3_f4_censored")
    t4 = json.loads((HERE / "m3_t4_b5.json").read_text())
    prime = json.loads((HERE / "prime_toy13_s18.json").read_text())
    sweep = json.loads((HERE / "sumset_sweep.json").read_text())
    struct = json.loads((HERE / "structure_probe.json").read_text())
    model = json.loads((HERE / "cost_model.json").read_text())

    def arm(v, name, keys):
        return {k: v["arms"][name][k] for k in keys}

    keys = ["vars", "equations", "degrees", "solving_degree_max", "solver_calls",
            "solver_ops", "budget_exceeded_rows", "relations", "trials",
            "relation_prob_per_trial", "total_gae", "s_over_rho_median",
            "max_cols", "max_rows", "max_nnz", "verified_all"]

    recs = [
        {"id": "header", "schema": "pkm.route3-m3-exponent/v1",
         "binary_sha256": SHA, "instance": "icv1-fp13-tm13-9e2d11f6",
         "bits": 13, "seed": 20261185, "date": "2026-11-25",
         "purpose": "does m>=3 change the p-exponent of the PKM relation route?",
         "route": "Route 3 of the subexponential ECDLP programme",
         "note": "base size 18 abscissae = 2^4.17, i.e. p^0.30, not the ideal "
                 "p^(1/3)=p^0.333 but the closest matched size available at 13 bits; "
                 "both arms share it, so arm-vs-arm comparisons are unaffected"},
        {"id": "m2_reference", "kind": "bench", "protocol": "16 repeats x 1000 trials, "
         "5 s solver budget, 1 rho run, order interval,candidate,candidate,interval",
         "raw": "m2_m3_13bit_20261185.jsonl (group m2_ref)",
         "candidate": arm(m2, "candidate", keys), "interval": arm(m2, "interval", keys),
         "gae_ratio_interval_over_candidate": m2["gae_ratio_int_over_cand"],
         "ops_per_call_ratio": (m2["arms"]["interval"]["solver_ops"] /
                                m2["arms"]["interval"]["solver_calls"]) /
                               (m2["arms"]["candidate"]["solver_ops"] /
                                m2["arms"]["candidate"]["solver_calls"]),
         "complete": m2["complete"]},
        {"id": "m3_f4_t5", "kind": "bench", "protocol": "2 repeats x 2 trials (16x1000 "
         "infeasible: every call is censored at 5 s), 5 s budget, same order",
         "raw": "m2_m3_13bit_20261185.jsonl (group m3_f4_censored)",
         "candidate": arm(m3, "candidate", keys), "interval": arm(m3, "interval", keys),
         "verified_rows": 0, "censored_rows": 8,
         "ops_per_call_candidate_censored": m3["arms"]["candidate"]["solver_ops"] /
                                            m3["arms"]["candidate"]["solver_calls"],
         "ops_per_call_interval_censored": m3["arms"]["interval"]["solver_ops"] /
                                           m3["arms"]["interval"]["solver_calls"],
         "claim": "charged work spent without resolution; not a solve cost"},
        {"id": "m3_f4_t4", "kind": "probe", "protocol": "2 repeats x 2 trials, 5 s and "
         "30 s budgets give identical ops (deterministic)",
         "raw": "m3_t4_b5.json", "factor_base": "prime-isogeny:t=4,seed=5,aux=ordinary",
         "abscissae": 11, "vars": t4["rows"][0]["decomposition"]["system"]["n_vars"],
         "equations": t4["rows"][0]["decomposition"]["system"]["n_equations"],
         "degrees": t4["rows"][0]["decomposition"]["system"]["degrees"],
         "solving_degree_max": max(r["decomposition"]["solver"]["solving_degree_max"]
                                   for r in t4["rows"]),
         "solver_ops_total": sum(r["decomposition"]["solver"]["ops"] for r in t4["rows"]),
         "solver_calls": sum(r["decomposition"]["solver"]["calls"] for r in t4["rows"]),
         "ops_per_call": sum(r["decomposition"]["solver"]["ops"] for r in t4["rows"]) /
                         sum(r["decomposition"]["solver"]["calls"] for r in t4["rows"]),
         "censored": 0},
        {"id": "exact_m2_yield_toy13", "kind": "binary-control",
         "raw": "prime_toy13_s18.json", "command": "ic prime --curve toy:13,1 --size 18",
         "sums": prime["report"]["sums"], "distinct_sums": prime["report"]["distinct_sums"],
         "exact_yield": prime["report"]["exact_yield"],
         "model_yield": prime["report"]["model_yield"],
         "meaning": "validates q ~ c*|V|^2/p with c = 1.94 on a 13-bit prime curve"},
        {"id": "sumset_sweep", "kind": "exact-count", "raw": "sumset_sweep.json",
         "rows": sweep["rows"],
         "meaning": "|x(V+-V)| = 0.89-1.00 |V|^2 and |x(V+-V+-V)| = 0.10-0.66 |V|^3: "
                    "the m=3 intermediate is a generic field element at every T "
                    "measured, so there is no per-level pruning"},
        {"id": "coset_control", "kind": "exact-count", "raw": "structure_probe.json",
         "coset": struct["coset_control"],
         "meaning": "a subgroup-coset base DOES prune (s2 = 1.5|V|, s3 = 2|V|), and its "
                    "m=3 yield falls 40x, which is worse than the 32x decision saving"},
        {"id": "cost_model", "kind": "derivation", "raw": "cost_model.json",
         "engines": model["engines"], "closed_forms": model["closed_forms"],
         "table_exponent_by_m": model["table_exponent_by_m"],
         "anchors_13bit": model["anchors_13bit"]},
        {"id": "verdict", "kind": "conclusion",
         "question": "does m>=3 change the cost exponent?",
         "answer": "no route to < p^0.5 in this family",
         "numbers": {
             "m2_exponent_measured_ops_law": 0.78,
             "m2_exponent_f4_paper_law": 2.487,
             "m3_exponent_f4_measured_break": ">=1.66 and not realised: the m=3 chain "
             "is 4.4e3x the m=2 per-call cost at a *lower* T and unresolved at equal T",
             "m3_exponent_exact_noreuse": 1.0,
             "m3_exponent_exact_with_p2_3_table": 0.667,
             "rho": 0.5}},
    ]

    seen = set()
    if STORE.exists():
        for line in STORE.read_text().splitlines():
            seen.add(json.loads(line)["id"])
    with STORE.open("a") as f:
        for rec in recs:
            if rec["id"] in seen:
                continue
            f.write(json.dumps(rec, sort_keys=True) + "\n")
            f.flush()
            os.fsync(f.fileno())
    print(f"{STORE}: {len(recs)} records, {len(seen)} pre-existing")


if __name__ == "__main__":
    main()
