"""PC1 (positive control): the l1_paired run on Run50's public input, seeds 389/389,
N 30000, PRIMARY_N 3415, must reproduce every committed l1-steps-2-3 number.
Glue only: equality checks and extraction (python3 -I). Prints JSON; exit 0 = pass.
Usage: python3 -I compare_pc1.py PC1_DIR STEPS_RUNS_DIR"""
import hashlib, json, sys

d, runs = sys.argv[1], sys.argv[2]
pipe = json.load(open(f"{d}/pipeline.summary.json"))
stream = json.load(open(f"{d}/stream.primary.summary.json"))
ctl = json.load(open(f"{runs}/run50-control.json"))["arms"]["local_full_max_descent_residuals_8192"]
m3415 = json.load(open(f"{runs}/match-n3415.summary.json"))
m30000 = json.load(open(f"{runs}/match-n30000.summary.json"))
raw = {}
for line in open(f"{runs}/RAW.sha256"):
    h, name = line.split()
    raw[name] = h
sha = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest()
cols = stream["columns"]
phases = {r["phase"]: int(r["diagnostic_priced_bitops"]) for r in pipe["phases"]}
checks = {
    "counts": pipe["counts"] == ctl["counts"],
    "graph_stats": pipe["graph_stats"] == ctl["graph_stats"],
    "last_checkpoint": pipe["last_checkpoint"] == ctl["last_checkpoint"],
    "projections": pipe["projections"] == ctl["projections"][1],
    "screen_calls": pipe["screen_calls"] == ctl["screen_calls"][1],
    "screen_refusals": pipe["screen_refusals"] == ctl["screen_refusals_vs_early_refusals"][1],
    "descent_residuals": pipe["descent_residuals"] == ctl["descent_residuals"][1] == 3415,
    "accepted_descents": pipe["accepted_descents"] == ctl["accepted_descents"][1] == 2,
    "scalar": pipe["actual_scalar"] == ctl["actual_scalar"][1] == 242624768583643540 and pipe["verified"] is True,
    "phase_subtotal": sum(phases.values()) == int(ctl["subtotal"][1]) == 5913875255822,
    "descent_phase": phases.get("descent") == 3426280619436,
    "d_exact_classes": pipe["d_exact_classes"] == ctl["d_exact_classes"] == 20510,
    "anchor_replay": pipe["anchor_witness_replay_ok"] == 20510 and pipe["anchor_witness_replay_bad"] == 0,
    "pipeline_rows_match": pipe["pipeline_rows_match"] is True,
    "pipeline_consistency": stream["pipeline_consistency"] is True,
    "dexact_sha256": sha(f"{d}/dexact.jsonl") == raw["full.dexact.jsonl"],
    "rows_sha256_n30000": sha(f"{d}/rows.primary.jsonl") == raw["match-n30000.rows.jsonl"],
    "tally_primary_3415": stream["tally_primary"] == m3415["primary"]["tally"],
    "tally_all_30000": stream["tally_all"] == m30000["extended"]["tally"],
    "decomposing_indices": stream["decomposing_residual_indices"] == m30000["decomposing_residual_indices"],
    # new on this instrument: the affine path and elliptic replay of every row
    "F_replayed_31": cols["F"] == "31" and cols["F_rows"] == "31",
    "M_replayed_57": cols["M"] == "57" and cols["M_rows"] == "57",
    "M_by_pattern": (cols["M_1^4,2"], cols["M_1^2,2^2"], cols["M_2^3"]) == ("47", "10", "0"),
    "replay_failures_0": cols["replay_fail_F"] == "0" and cols["replay_fail_M"] == "0" and cols["unreplayable"] == "0",
    "negative_control_flips": cols["flip_total"] == "57" and cols["flip_refused"] == "57",
    "affine_consistency": cols["affine_inconsistent"] == "0",
    "group_anchor": cols["group_bad"] == "0" and cols["anchor_bad"] == "0",
}
ok = all(checks.values())
print(json.dumps({"schema": "l1-paired.pc1/v1", "pcc": "unverified", "pass": ok, "checks": checks}, sort_keys=True))
sys.exit(0 if ok else 1)
