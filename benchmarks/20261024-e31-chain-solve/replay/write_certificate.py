#!/usr/bin/env python3
"""Assemble replay-certificate.json from the validator's replay outputs (stdlib only).

Inputs: replay/replay_comparison.json (replay_compare.py), replay/agreement_comparison.json
(agreement_compare.py), replay/unit_test.log, replay/build.json (build facts recorded at build
time).  Output: ../replay-certificate.json.
"""
import hashlib, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
EVD = os.path.dirname(HERE)


def load(p):
    return json.load(open(os.path.join(HERE, p)))


rc = load("replay_comparison.json")
ag = load("agreement_comparison.json")
build = load("build.json")
ut = open(os.path.join(HERE, "unit_test.log"), encoding="utf-8", errors="replace").read()
ut_pass = bool(re.search(r"test cryptanalysis::klein_v4::tests::e31_chain_solve_agrees_with_the_existing_solver \.\.\. ok", ut)) \
    and bool(re.search(r"test result: ok\. 1 passed; 0 failed", ut))

runs = []
for r in rc["e2e_runs"]:
    runs.append({k: r[k] for k in ("experiment", "solver", "p", "seed", "arm", "committed", "replay_output", "replayed",
                                   "fields_compared", "fields_identical", "mismatches", "fields_added_in_replay",
                                   "timing_not_compared", "matched", "logarithm", "key_scalars")})
sa = rc["stage_a"]
for c in sa["curves"]:
    runs.append({"experiment": "e31 stage A", "p": c["p"], "seed": c["seed"], "calls": c["calls"],
                 "committed": sa["committed"], "replay_output": sa["replay_output"], "replayed": True,
                 "fields_compared": c["fields_compared"], "fields_identical": c["fields_identical"],
                 "mismatches": c["mismatches"], "timing_not_compared": c["wall_not_compared"], "matched": c["matched"],
                 "agree": c["agree"], "quotient_agree": c["quotient_agree"], "quotient_checked": c["quotient_checked"],
                 "fallbacks": c["fallbacks"], "C_median": c["C_median"]})

e2e_all = all(r["matched"] for r in rc["e2e_runs"])
sa_all = sa["curves_matched"] == sa["curves_replayed"] == 10
logs_ok = all(r["logarithm"]["replay_correct"] and r["logarithm"]["walks_correct"] for r in rc["e2e_runs"])
e31_e2e = [r for r in rc["e2e_runs"] if r["solver"] != "macaulay"]
status = "verified" if (e2e_all and sa_all and logs_ok and ut_pass and ag["all_agree"]) else "failed"

cert = {
    "schema": "replay-certificate/v1",
    "row": "E31 (GLV note §9.23): solve_weighted_chain A (chain) and B (chain-sparsest) on E25's ten GLS curves, both arms; evidence directory benchmarks/20261024-e31-chain-solve",
    "date": "2026-10-24",
    "validator": "validator subagent (review-adversarial policy), independent of the session that produced E31: fresh build in a separate target directory, fresh runs, own comparison scripts (replay_compare.py, agreement_compare.py) and own recompute (validate.py)",
    "repository": build["repository"],
    "recorded_binary": "a8f52422d5 (experiments/31_solve_cost/binary_commit.txt)",
    "code_drift": build["code_drift"],
    "build": build["build"],
    "environment": {
        "host": build["host"],
        "RAYON_NUM_THREADS": {"end_to_end": 4, "stage_a": 1, "agreement_stage_a": 1, "agreement_end_to_end": 4},
        "determinism_note": "collect_and_solve draws each round's 2 x rayon-threads targets from the run's seeded RNG, so RAYON_NUM_THREADS=4 (the recording host's count, run_e2e.sh) is part of the end-to-end trajectory; Stage A draws its stream with a fixed --threads 4 batch size independent of rayon and is one-threaded as recorded. Field counters are per-call (Cell/u64 in each solve), not shared across threads.",
        "concurrency": "the end-to-end replays, the Stage A replay, the agreement sample and the unit-test build ran concurrently on the same 4 vCPUs (load average up to ~11), beside another agent's session in the same worktree; only wall-clock fields are affected, and they are reported, not compared",
    },
    "command": "BIN=<fresh quartic_pipeline_gls> replay/replay_e2e.sh (run_e2e.sh's arguments: --sizes P --first-seed S --seeds 1 --rho-runs 16 --max-residuals 5000000 --solver {chain|chain-sparsest|macaulay} --json OUT, RAYON_NUM_THREADS=4); BIN=<fresh e31_stage_a> replay/replay_stage_a.sh (--sizes 101,149,197,269,397 --seeds 1,2 --targets 1000, RAYON_NUM_THREADS=1); replay/agreement.sh; cargo test --release --lib (the E31 agreement test)",
    "comparison": "replay/replay_compare.py: every leaf of every committed record compared with the replay, integers and floats exactly; end-to-end wall_ms and solve_ms and Stage A wall_us / wall summaries are wall-clock, reported and not compared",
    "scalar_verification": "For every replayed end-to-end arm the fresh binary recovered the logarithm by Wiedemann, checked [x]G = Q on the curve and set folded.correct = (x == d) for the instance's planted d (quartic_folds.rs Arm::attempt); every replay reports folded.correct = true, and all 16 x 3 walks per record report correct = true. The committed records carry correct = true but not x; x is tied to them through the instance: the replay regenerates it from (p, seed) and n, #E, h, the base and every deterministic counter (targets, decompositions, relations, Wiedemann operations, walks) are identical. The E25 replay certificate (benchmarks/20261018-k4-gls-v4/replay-certificate.json) checked these ten instances independently (G, Q on curve, n prime, nG = O, dG = Q, #E = h n, j in F_p2 \\ F_p); the E31 records carry identical n, group_order, cofactor and base_points.",
    "agreement": {
        "unit_test": {"name": "cryptanalysis::klein_v4::tests::e31_chain_solve_agrees_with_the_existing_solver", "passed": ut_pass,
                      "log": "replay/unit_test.log", "covers": "E25 p = 101 s1, 149 s2; E24 p = 101 s2, 53 s1; 24 targets each; both pivot rules; M_P2 and charpoly; negative control"},
        "validator_sample": ag,
    },
    "status": status,
    "independent": True,
    "scalars_verified": bool(e2e_all and sa_all and logs_ok),
    "runs_total": len(runs),
    "runs_matched": sum(1 for r in runs if r["matched"]),
    "coverage": {"e31_end_to_end_records_replayed": len(e31_e2e), "e31_end_to_end_records_committed": 40,
                 "aa_control_records": len(rc["e2e_runs"]) - len(e31_e2e), "stage_a_curves_replayed": sa["curves_replayed"],
                 "stage_a_curves_committed": 10},
    "runs": runs,
}
with open(os.path.join(EVD, "replay-certificate.json"), "w") as f:
    json.dump(cert, f, indent=1, ensure_ascii=False)
    f.write("\n")
print(f"status {status}: {cert['runs_matched']}/{cert['runs_total']} runs matched; unit test {'passed' if ut_pass else 'NOT passed'}; "
      f"agreement sample {'agrees' if ag['all_agree'] else 'DISAGREES'}")
sys.exit(0 if status == "verified" else 1)
