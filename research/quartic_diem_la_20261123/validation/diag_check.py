#!/usr/bin/env python3
"""Test the amendment's mechanism claim on grid rows (lesson L-4c4c2ac6ef29).

PROTOCOL.md section 4 says, from development data, that on the fallback pairs "the
first attempt's system has no solution for either arm, so the fallback re-runs a
solve that also fails".  The binary's uncounted --quartic-diag records, per attempt,
the dense rank of the core over F_r with and without the t column; t is determined
iff rank_no_t + 1 == rank (quartic_ladder.rs, QAttemptDiag).

Usage: python3 -I diag_check.py STUDY_DIR DIAG_DIR OUT.json

  1. every diag run row equals the committed row on every committed key except
     wall_ns (the flag must not change a counted field);
  2. per run: the attempt outcomes and whether t was determined in each attempt.
"""
import json
import sys

SFB, SGE = "Q_diem_2lp_sfb", "Q_diem_2lp_sfb_sge"


def jl(p):
    return [json.loads(l) for l in open(p) if l.strip()]


def main():
    study, dd, outp = sys.argv[1:4]
    com = {}
    for f in ("raw/run.jsonl", "raw/run131101.jsonl"):
        for r in jl(f"{study}/{f}"):
            if r["kind"] == "run":
                com[(r["arm"], r["p"], r["curve_seed"], r["seed"])] = r
    out = {"runs": [], "summary": {}}
    for f in ("diag_main.jsonl", "diag_131101.jsonl"):
        for r in jl(f"{dd}/{f}"):
            if r["kind"] != "run":
                continue
            k = (r["arm"], r["p"], r["curve_seed"], r["seed"])
            c = com[k]
            diff = sorted(x for x in c if x != "wall_ns" and c[x] != r[x])
            att = [{"outcome": d["outcome"], "rank": d["rank"], "rank_no_t": d["rank_no_t"],
                    "t_determined": d["rank_no_t"] + 1 == d["rank"], "core": [d["core_rows"], d["core_cols"]]}
                   for d in r["diag"]]
            out["runs"].append({"arm": r["arm"], "p": r["p"], "curve_seed": r["curve_seed"], "seed": r["seed"],
                                "committed_fields_equal": not diff, "differing": diff,
                                "attempts": r["attempts"], "la_fallbacks": r["la_fallbacks"], "diag": att})
    runs = out["runs"]
    fb = [e for e in runs if e["attempts"] > 1]
    # attempts that reached a solve (outcome 1, 2 or 3); outcome 4 = skipped (no t row), 0 = core too small
    solved_attempts = lambda e: [a for a in e["diag"] if a["outcome"] in (1, 2, 3)]
    out["summary"] = {
        "diag_runs": len(runs),
        "committed_fields_equal": sum(e["committed_fields_equal"] for e in runs),
        "multi_attempt_runs": len(fb),
        "multi_attempt_first_solved_attempt_t_determined": [
            solved_attempts(e)[0]["t_determined"] if solved_attempts(e) else None for e in fb],
        "multi_attempt_first_solved_attempt_outcome": [
            solved_attempts(e)[0]["outcome"] if solved_attempts(e) else None for e in fb],
        "single_attempt_runs_t_determined": [
            solved_attempts(e)[-1]["t_determined"] for e in runs if e["attempts"] == 1 and solved_attempts(e)],
    }
    json.dump(out, open(outp, "w"), indent=1)
    print(json.dumps(out["summary"], indent=1))


if __name__ == "__main__":
    main()
