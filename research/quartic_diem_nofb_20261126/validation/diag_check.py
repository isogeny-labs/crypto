#!/usr/bin/env python3
"""Validator: the mechanism behind the rescue claim and RESULTS.md:69 (lesson L-4c4c2ac6ef29).

RESULTS.md:69 says the multi-attempt runs "exist because attempt 1 leaves t undetermined
(parent ERRATA E6)".  The parent checked that on its seeds 1, 2 only; seeds 3, 4 are new.
It also decides whether a rescue was possible at all: a fallback (full-core Wiedemann on the
same attempt) can certify t only if the attempt's core determines t.

Adapted from the parent validation's diag_check.py (quartic_diem_la_20261123/validation/).
The binary's uncounted --quartic-diag records, per counted attempt, the dense rank of the
core over F_r with and without the t column (t is determined iff rank_no_t + 1 == rank) and
the outcome (1 no candidate, 2 wrong t, 3 certified, 4 skipped, 0 core too small)
(quartic_ladder.rs QAttemptDiag; one entry per counted attempt).

Usage: python3 -I diag_check.py STUDY_DIR DIAG_DIR OUT.json

  1. every diag run row equals the committed row on every committed key except wall_ns:
     the flag must not change a counted field (both arms);
  2. per run and attempt: outcome, rank, rank_no_t, t determined;
  3. per failed (non-final) attempt of either arm: was t determined?  If not, no solve of
     that attempt's core could certify t, so a rescue was impossible there.
Every key is read with [] (lesson L-99ee952350b2).
"""
import json
import os
import sys


def jl(p):
    with open(p) as fh:
        return [json.loads(l) for l in fh if l.strip()]


def main():
    study, dd, outp = sys.argv[1:4]
    com = {}
    for f in ("raw/run.jsonl", "raw/run131101.jsonl"):
        for r in jl(f"{study}/{f}"):
            if r["kind"] == "run":
                k = (r["arm"], r["p"], r["curve_seed"], r["seed"])
                assert k not in com
                com[k] = r
    out = {"runs": [], "summary": {}}
    for f in ("diag_main.jsonl", "diag_131101.jsonl"):
        if not os.path.exists(f"{dd}/{f}"):
            out.setdefault("missing_files", []).append(f)
            continue
        for r in jl(f"{dd}/{f}"):
            if r["kind"] != "run":
                continue
            k = (r["arm"], r["p"], r["curve_seed"], r["seed"])
            c = com[k]
            diff = sorted(x for x in c if x != "wall_ns" and c[x] != r[x])
            extra = sorted(set(r) - set(c))
            att = [{"outcome": d["outcome"], "rank": d["rank"], "rank_no_t": d["rank_no_t"],
                    "t_determined": d["rank_no_t"] + 1 == d["rank"],
                    "core": [d["core_rows"], d["core_cols"]], "t_weight": d["t_weight"]}
                   for d in r["diag"]]
            assert len(att) == r["attempts"], k
            out["runs"].append({"arm": r["arm"], "p": r["p"], "curve_seed": r["curve_seed"], "seed": r["seed"],
                                "committed_fields_equal": not diff, "differing": diff, "extra_keys": extra,
                                "attempts": r["attempts"], "la_fallbacks": r["la_fallbacks"],
                                "la_fallbacks_skipped": r["la_fallbacks_skipped"], "diag": att})
    runs = out["runs"]
    multi = [e for e in runs if e["attempts"] > 1]
    failed = [(e, a) for e in runs for a in e["diag"][:-1]]
    summ = {
        "diag_runs": len(runs),
        "committed_fields_equal": sum(e["committed_fields_equal"] for e in runs),
        "extra_keys": sorted({x for e in runs for x in e["extra_keys"]}),
        "by_seed_panel": {},
        "multi_attempt_runs": len(multi),
        "multi_attempt_by_arm": {a: sum(1 for e in multi if e["arm"] == a) for a in sorted({e["arm"] for e in runs})},
        "failed_attempts": len(failed),
        "failed_attempts_t_determined": sum(a["t_determined"] for _, a in failed),
        "failed_attempt_outcomes_by_arm": {arm: sorted({a["outcome"] for e, a in failed if e["arm"] == arm})
                                           for arm in sorted({e["arm"] for e in runs})},
        "failed_attempts_rank_below_cols": sum(a["rank"] < a["core"][1] for _, a in failed),
        "failed_attempts_rank_minus_rank_no_t": sorted({a["rank"] - a["rank_no_t"] for _, a in failed}),
        "final_attempt_t_determined": sum(e["diag"][-1]["t_determined"] for e in runs),
        "final_attempt_certified": sum(e["diag"][-1]["outcome"] == 3 for e in runs),
    }
    for name, ss in (("seeds_1_2", (1, 2)), ("seeds_3_4_held_out", (3, 4))):
        fe = [(e, a) for e, a in failed if e["seed"] in ss]
        summ["by_seed_panel"][name] = {
            "multi_attempt_runs": sum(1 for e in multi if e["seed"] in ss),
            "failed_attempts": len(fe),
            "failed_attempts_t_determined": sum(a["t_determined"] for _, a in fe),
        }
    summ["rescue_possible_in_any_failed_attempt"] = summ["failed_attempts_t_determined"] > 0
    out["summary"] = summ
    with open(outp, "w") as fh:
        json.dump(out, fh, indent=1)
        fh.write("\n")
    print(json.dumps(summ, indent=1))


if __name__ == "__main__":
    main()
