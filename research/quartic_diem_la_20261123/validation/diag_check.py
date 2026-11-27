#!/usr/bin/env python3
"""Test the amendment's mechanism claim (lesson L-4c4c2ac6ef29).

PROTOCOL.md:56 says, from development data, that on the fallback pairs "the first
attempt's system has no solution for either arm, so the fallback re-runs a solve
that also fails".  The binary's uncounted --quartic-diag records, per counted
attempt, the dense rank of the core over F_r with and without the t column
(t is determined iff rank_no_t + 1 == rank) and the outcome
(1 Wiedemann None, 2 wrong t, 3 certified, 4 skipped, 0 core too small)
(quartic_ladder.rs, QAttemptDiag; one entry per counted attempt).

Usage: python3 -I diag_check.py STUDY_DIR DIAG_DIR OUT.json

  1. every diag run row equals the committed row (raw/ for the grid curves,
     dev/dev_W8_F1.5_E64.jsonl for the dev curves) on every committed key except
     wall_ns: the flag must not change a counted field;
  2. per run: the attempt outcomes and whether t was determined in each attempt.
"""
import json
import os
import sys


def jl(p):
    return [json.loads(l) for l in open(p) if l.strip()]


def main():
    study, dd, outp = sys.argv[1:4]
    com = {}
    for f in ("raw/run.jsonl", "raw/run131101.jsonl", "dev/dev_W8_F1.5_E64.jsonl"):
        for r in jl(f"{study}/{f}"):
            if r["kind"] == "run":
                k = (r["arm"], r["p"], r["curve_seed"], r["seed"])
                assert k not in com
                com[k] = (f, r)
    out = {"runs": [], "summary": {}}
    for f in ("diag_main.jsonl", "diag_131101.jsonl", "diag_dev.jsonl"):
        if not os.path.exists(f"{dd}/{f}"):
            continue
        for r in jl(f"{dd}/{f}"):
            if r["kind"] != "run":
                continue
            k = (r["arm"], r["p"], r["curve_seed"], r["seed"])
            cf, c = com[k]
            diff = sorted(x for x in c if x != "wall_ns" and c[x] != r[x])
            att = [{"outcome": d["outcome"], "rank": d["rank"], "rank_no_t": d["rank_no_t"],
                    "t_determined": d["rank_no_t"] + 1 == d["rank"], "core": [d["core_rows"], d["core_cols"]]}
                   for d in r["diag"]]
            assert len(att) == r["attempts"], k
            out["runs"].append({"set": "dev" if cf.startswith("dev/") else "grid", "committed_file": cf,
                                "arm": r["arm"], "p": r["p"], "curve_seed": r["curve_seed"], "seed": r["seed"],
                                "committed_fields_equal": not diff, "differing": diff,
                                "attempts": r["attempts"], "la_fallbacks": r["la_fallbacks"], "diag": att})
    runs = out["runs"]
    summ = {"diag_runs": len(runs), "committed_fields_equal": sum(e["committed_fields_equal"] for e in runs)}
    for s in ("grid", "dev"):
        rs = [e for e in runs if e["set"] == s]
        multi = [e for e in rs if e["attempts"] > 1]
        summ[s] = {
            "runs": len(rs),
            "multi_attempt_runs": len(multi),
            "multi_attempt_runs_with_fallback_sge": sum(1 for e in multi if e["arm"].endswith("sge")
                                                        and e["la_fallbacks"] > 0),
            "attempt1_t_determined_in_multi_attempt_runs": sum(e["diag"][0]["t_determined"] for e in multi),
            "attempt1_outcomes_in_multi_attempt_runs": sorted({e["diag"][0]["outcome"] for e in multi}),
            "attempt1_rank_minus_rank_no_t_in_multi_attempt_runs": sorted({e["diag"][0]["rank"] - e["diag"][0]["rank_no_t"]
                                                                          for e in multi}),
            "attempt1_rank_below_cols_in_multi_attempt_runs": sum(e["diag"][0]["rank"] < e["diag"][0]["core"][1]
                                                                  for e in multi),
            "final_attempt_t_determined_all_runs": sum(e["diag"][-1]["t_determined"] for e in rs),
            "final_attempt_certified_all_runs": sum(e["diag"][-1]["outcome"] == 3 for e in rs),
        }
    out["summary"] = summ
    json.dump(out, open(outp, "w"), indent=1)
    print(json.dumps(summ, indent=1))


if __name__ == "__main__":
    main()
