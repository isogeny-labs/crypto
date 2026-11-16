#!/usr/bin/env python3
"""Follow-up placement for VAL-20261029-33a286 (validator's own code).

Writes exactly these repository paths (no commit):
  research/ic_dg_blocked_20261029/VALIDATION.json             (blockers updated, followup_review added)
  benchmarks/20261029-ic-dg-blocked/VALIDATION.json           (same content, local pins, evidence_note)
  benchmarks/20261029-ic-dg-blocked/replay-certificate.json   (byte copy)
  benchmarks/20261029-ic-dg-blocked/validator-report.json     (raw_data paths rebased to ../../research/...)
The pinned research files (validation/validator-report.json, validation/replay-certificate.json) are not
changed, so their sha256 pins and validation/SHA256SUMS stay valid.  Staged under followup/out/ first.
"""
import hashlib
import json
import os
import shutil

WT = "/dev/shm/wt-blk"
STUDY = "research/ic_dg_blocked_20261029"
EV = f"{WT}/{STUDY}"
BD = f"{WT}/benchmarks/20261029-ic-dg-blocked"
F = "/dev/shm/dgb-validation/followup"
STAGE = f"{F}/out"
SNAP1 = "313ec5d2af38fe7994a6eab34aa1ef98e374f309"


def sha(p):
    return hashlib.sha256(open(p, "rb").read()).hexdigest()


def dumps(obj):
    return json.dumps(obj, indent=1) + "\n"


fu = json.load(open(f"{F}/followup.json"))
cf = fu["phase_counterfactual"]
assert fu["head"] == SNAP1 and fu["remote_tip"] == SNAP1
assert not fu["files_modified_or_deleted_since_bf899979a6"] and fu["validation_dir_matches_my_SHA256SUMS"]
assert fu["VALIDATION_json_equals_my_draft"] and fu["launch_all_hold"] and not fu["claims_disagreeing"]
assert fu["figure"]["png_rerender_equals_committed"] and fu["figure"]["svg_rerender_equal_modulo_ids_and_date"]
assert not fu["pdf"]["missing_in_pdf_text"] and not fu["readme_has_position_table"]

base_text = open(f"{EV}/VALIDATION.json").read()
base = json.loads(base_text)
assert sha(f"{EV}/VALIDATION.json") == sha(f"{EV}/validation/VALIDATION.draft.json")

blockers = [
    ("README.md (and README.pdf, copied as benchmarks REPORT.pdf) lacks the position table PROTOCOL section 10 requires: "
     "'per size, each arm's block positions (from records.jsonl)'. The position-control figures are present (0.9302 [0.9268, 0.9336], "
     "0.9077, 0.8977). The validator's tabulation, identical at 24, 28 and 32 bits over the 288 measured blocks: dg and dgb-edge at "
     "positions 0 and 3 (144 blocks each), dg-aa and dgb at positions 1 and 2 (144 each); 144 forward and 144 reversed blocks; dgb "
     "follows dg-aa in 144 blocks and precedes it in 144 (validation/work/recompute.json, sizes.P<b>.positions)."),
    ("README.md 'Where the time went' (and both PDFs) misstates the phase evidence: 'The probe cost gives back part of the set-up gain, "
     "which is why every ratio missed its prediction.' With the target-PDP phase held at dg's time, the summed-solve ratio "
     f"would still be {cf['P24']['ratio_if_pdp_unchanged']:.3f}, {cf['P28']['ratio_if_pdp_unchanged']:.3f} and "
     f"{cf['P32']['ratio_if_pdp_unchanged']:.3f} at 24, 28 and 32 bits, above every predicted range (upper ends 0.87, 0.84, 0.85). "
     f"At 24 and 28 bits the set-up fell only to {cf['P24']['setup_ratio']:.3f} and {cf['P28']['setup_ratio']:.3f} of dg's "
     "(predicted 0.25-0.5), so the smaller set-up saving and the slower target-PDP phase together explain the misses; only at 32 bits "
     f"(set-up {cf['P32']['setup_ratio']:.3f}, inside the range) is the PDP rise nearly the whole miss. Suggested text: 'The probe "
     "cost gives back part of the set-up gain. With a set-up saving smaller than predicted at 24 and 28 bits (0.594 and 0.565 of "
     "dg's, against 0.25-0.5), it explains why every ratio missed its prediction: with the target-PDP phase unchanged the ratios "
     "would still be 0.898, 0.867 and 0.851.' (Descriptive arithmetic on phase_sums.json, taking phase times as additive; at 28 "
     "bits the unexplained remainder is +0.9 ms of 5,363 ms.)"),
    ("Placement is not yet in a committed record. The main session announced a new section dg_blocked_component ('Component "
     "speed-up: a cache-blocked packed pair table in wall time') outside tables A-C. That is the branch PROTOCOL section 10 "
     "prescribes, because pair_table_component at 313ec5d2af holds set-up figures only (research/ic_oracle_setup_blocked_20261020). "
     "This blocker clears when the evidence review records the choice and the scripts/build_ic_leaderboard.py patch is committed."),
]

followup = {
    "reviewed_commit": SNAP1,
    "date": "2026-10-29",
    "by": "VAL-20261029-33a286 (same validator, own code)",
    "scope": "README.md, README.pdf and figures/ against the validator's numbers; P<b>/{wall,counted}/launch/ against the S3 launcher receipts; placement record",
    "code": {"path": "/dev/shm/dgb-validation/followup/followup.py (outside the repository)", "sha256": sha(f"{F}/followup.py")},
    "output": {"path": "/dev/shm/dgb-validation/followup/followup.json (outside the repository)", "sha256": sha(f"{F}/followup.json")},
    "checks": [
        f"313ec5d2af equals the origin tip; it adds {fu['files_added_since_bf899979a6']} files to bf899979a6 and modifies or deletes none; "
        "validation/ equals the validator's output byte for byte (82 files, SHA256SUMS 15cc8caa...); VALIDATION.json was the draft (01d4bc34...); "
        f"{fu['artifact_manifest_files']} artifact-store files match their committed .artifacts.json manifests",
        "launch/ committed for all six sessions: receipts/request.json and result.json byte-equal to the S3 launcher receipts; request-P*.json "
        "spec = frozen spec and = the receipt job except the launcher-added memory key; CodeBuild build id = the S3 launcher id; start-request "
        "idempotency token = sha256(request.json) = result.request_sha256; CodeBuild SUCCEEDED for 28-bit wall and FAILED for the five exit-42 "
        "sessions, matching the launcher states; each log names its launcher id and state; no credentials",
        f"README.md: {len(fu['claims'])} numeric claims checked against the validator's recomputation or the committed ecbench output, "
        "0 disagree: decision table, A/A, edge control (A/A-corrected 0.9302 [0.9268, 0.9336], 0.9077, 0.8977; recomputed exactly), "
        "phase sums, PDP rises +5/+34/+21%, set-up 0.565, counted band points 12.96/21.79/33.07 etc.; the band intervals are ecbench "
        "ops.ci95_bootstrap (cluster, 4,000 resamples) and contain the validator's own target-cluster bootstraps, so they are conservative",
        "README.md facts: 28-bit 1,344 executions all L2; native A/A 'includes arm A', 288 pairs; card sha256 edc0da03...7ccd and "
        "64754a62...cfda; APPROVAL.md E1-E10 and the residual-risk 'reported and not rerun'; linalg incremental-gauss; node pool cpu-spot; "
        "pods, jobs and curves as in the validator report",
        "figures/: plot.py reads only the committed compare-dg__dgb-w0, compare-dg__dgb-edge-w0 (wall block) and phase_sums.json files, "
        "all equal to the recompute; re-rendered in scratch, ratios.png is byte-identical (e07a13a0...) and ratios.svg identical apart "
        "from matplotlib's random ids and date; the benchmarks copies are equal",
        f"README.pdf ({fu['pdf']['sha256'][:8]}...): all {fu['pdf']['readme_decimal_numbers']} decimal numbers of README.md appear in its text; "
        "the figure is embedded; benchmarks REPORT.pdf is the same file",
    ],
    "resolved_blockers": [
        "Section 10 figure and PDF: committed at 313ec5d2af and checked (README.md also committed; see the two README blockers that remain).",
        "launch/ folders: committed at 313ec5d2af, byte-equal to the S3 launcher receipts.",
    ],
    "notes_not_blocking": [
        "README.md says every number is read from ecbench compare or band output or from phase_sums.json and 'nothing here is computed by "
        "hand', but +5/+34/+21% and 0.565 are derived from phase_sums.json and the Validation section quotes the validator report.",
        "README.md 'so their counted S is identical (see the next table)': the next table is the phase-sums table; the counted table follows it.",
        "README.md cites PR #2844's table-floor micro-benchmark (2.3-2.9x slower on misses); that is outside this evidence and was not checked.",
        "The uncommitted benchmarks/README.md row gives 'edge control 0.9303', the uncorrected dgb-edge/dg ratio (0.930331); README.md gives "
        "the A/A-corrected 0.9302 [0.9268, 0.9336]. Name which.",
        "scripts/validation_gate.py does not read promotion_blockers; a 'valid' gate result does not clear them.",
    ],
}

out = {}
for k, v in base.items():
    if k == "promotion_blockers":
        out[k] = blockers
    elif k == "promotion_note":
        out[k] = ("Evidence admissibility only for the two 28-bit rows; 24 and 32 bits are undecidable and get no row. The evidence "
                  "blockers of bf899979a6 are cleared (figure, PDF, launch receipts); three promotion blockers remain: two README "
                  "corrections and the placement record. Promotion and placement are the Coordinator's decision.")
    elif k == "draft_note":
        out["placement_note"] = ("Placed at 313ec5d2af. The validator's draft is kept unchanged as validation/VALIDATION.draft.json; this "
                                 "file differs from it only in promotion_blockers, promotion_note, this note (was draft_note) and "
                                 "followup_review. validation/validator-report.json still lists the blockers as of bf899979a6; "
                                 "followup_review records what changed.")
    else:
        out[k] = v
out["followup_review"] = followup
for k in ("schema", "rows", "replay_certificate", "validator_report", "rho_reference", "verdict", "size_verdict", "caveats"):
    assert out[k] == base[k], k

# benchmarks copies
rep = json.load(open(f"{EV}/validation/validator-report.json"))
assert sha(f"{EV}/validation/validator-report.json") == base["validator_report"]["sha256"]
rep_b = dict(rep)
rep_b["raw_data"] = [dict(x, path=f"../../{STUDY}/" + x["path"]) for x in rep["raw_data"]]
assert {k for k in rep if rep[k] != rep_b[k]} == {"raw_data"}

os.makedirs(f"{STAGE}/research", exist_ok=True)
os.makedirs(f"{STAGE}/benchmarks", exist_ok=True)
open(f"{STAGE}/research/VALIDATION.json", "w").write(dumps(out))
open(f"{STAGE}/benchmarks/validator-report.json", "w").write(dumps(rep_b))
shutil.copyfile(f"{EV}/validation/replay-certificate.json", f"{STAGE}/benchmarks/replay-certificate.json")
bench = {}
for k, v in out.items():
    if k == "replay_certificate":
        bench[k] = {"path": "replay-certificate.json", "sha256": sha(f"{STAGE}/benchmarks/replay-certificate.json")}
    elif k == "validator_report":
        bench[k] = {"path": "validator-report.json", "sha256": sha(f"{STAGE}/benchmarks/validator-report.json")}
    elif k == "placement_note":
        bench[k] = ("Copy of research/ic_dg_blocked_20261029/VALIDATION.json (as updated after 313ec5d2af) with the two pins pointing "
                    "to this directory's copies.")
    else:
        bench[k] = v
bench["evidence_note"] = ("replay-certificate.json is a copy of research/ic_dg_blocked_20261029/validation/replay-certificate.json; "
                          "validator-report.json is a copy of that directory's validator-report.json with raw_data paths rebased to this "
                          "directory (../../research/ic_dg_blocked_20261029/...); raw data is under research/ic_dg_blocked_20261029/P24, "
                          "P28 and P32 (artifact-store files restored by tools/artifacts/artifacts.py pull research/ic_dg_blocked_20261029)")
assert bench["replay_certificate"]["sha256"] == base["replay_certificate"]["sha256"]
open(f"{STAGE}/benchmarks/VALIDATION.json", "w").write(dumps(bench))

# place (only these four paths)
for p in (f"{BD}/VALIDATION.json", f"{BD}/replay-certificate.json", f"{BD}/validator-report.json"):
    assert not os.path.exists(p), f"refusing to overwrite {p}"
shutil.copyfile(f"{STAGE}/research/VALIDATION.json", f"{EV}/VALIDATION.json")
for fn in ("VALIDATION.json", "replay-certificate.json", "validator-report.json"):
    shutil.copyfile(f"{STAGE}/benchmarks/{fn}", f"{BD}/{fn}")
for p in (f"{EV}/VALIDATION.json", f"{BD}/VALIDATION.json", f"{BD}/replay-certificate.json", f"{BD}/validator-report.json"):
    print(sha(p), os.path.getsize(p), p)
