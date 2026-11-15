#!/usr/bin/env python3
"""Amends validation/replay-certificate.json and validation/validator-report.json (and the benchmarks copies) for
amendment 1 of ic_gw_bl_reuse_20261115. Reads the round-1 files as committed (a1e16d39e4; their sha256 must equal
PREV_*), the validator's amendment outputs under validation/amendment-1/, and amendment-1/analysis.json (claimed, by key).
usage: make_reports_amd1.py STAGE_DIR  (plan.json, nonce.txt, nonce.VOID-mistyped-input.txt, binary.sha256, stress-binaries.sha256)"""
import hashlib, json, os, sys

REPO = "/home/user/crypto"; STUDY = "research/ic_gw_bl_reuse_20261115"
V = os.path.join(REPO, STUDY, "validation"); VA = os.path.join(V, "amendment-1"); AM = os.path.join(REPO, STUDY, "amendment-1")
B = os.path.join(REPO, "benchmarks/20261115-ic-gw-bl-reuse")
PREV_CERT, PREV_REP = "2b89d4da91bc9446b4f034feb5d5541831fb88348892b2627a1ab5a927224e17", "3ad84134ba6b63447880ad140a32f4922f81e3bcd3d0706e51a424dfe959db08"
PREV_REP_BENCH = "8db0ef3565f9716e0ca725df15fc75aad90689985b832590001cbbe9bb5d7782"
stage = sys.argv[1]
sha = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest()
J = lambda p: json.load(open(p))
cert_p, rep_p = os.path.join(V, "replay-certificate.json"), os.path.join(V, "validator-report.json")
if sha(cert_p) != PREV_CERT or sha(rep_p) != PREV_REP:
    sys.exit("round-1 files are not the committed versions; refusing to amend")
cert, rep = J(cert_p), J(rep_p)
rc = J(os.path.join(VA, "recompute-amd1.json")); cmpj = J(os.path.join(VA, "replay-compare-amd1.json"))
st = J(os.path.join(VA, "stress-summary-amd1.json")); dl = J(os.path.join(VA, "delta-amd1.json"))
cl = J(os.path.join(AM, "analysis.json")); plan = J(os.path.join(stage, "plan.json"))
nonce_str, nonce = open(os.path.join(stage, "nonce.txt")).read().split("\n")[:2]
void_str, void_nonce = open(os.path.join(stage, "nonce.VOID-mistyped-input.txt")).read().split("\n")[:2]
bins = {os.path.basename(l.split()[1]): l.split()[0] for l in open(os.path.join(stage, "binary.sha256"))}
sbins = {os.path.basename(l.split()[1]): l.split()[0] for l in open(os.path.join(stage, "stress-binaries.sha256"))}
QS = [262111, 524287, 1048507, 2097091, 4194271]
TAR = "amendment-1/replay/replay-runs-amd1-20261115.tar.gz"
SCR = "/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/val_blr/amd1"

runs = cmpj["runs"]
assert all(r.get("verified") for r in runs) and cmpj["total_mismatches_non_negative_roles"] == 0
aud = [r for r in runs if r["arm"] == "AUD1"]
assert all(r["oracle"]["id1_bad"] == 0 and r["oracle"]["id2_bad"] == 0 for r in aud)
recon = [r for r in aud if "reconciliation" in r]; assert recon and all(r["reconciliation"]["exact"] for r in recon)
assert st["summary"]["id2_bad"] == 0 and st["summary"]["id1_bad"] == 0 and st["summary"]["ref_blr_same_result_iters_narrowed"] == st["summary"]["trials"]
assert rc["decision"]["SUPPORTED"] and rc["vs_claimed"]["ratio_max_rel_diff"] == 0.0
cells = sorted({(r["q"], r["seed"]) for r in aud if r["block"] == 4 and r["seed"] <= 6})
otot = {k: sum(r["oracle"][k] for r in aud) for k in ("id1_used", "id1_bad", "id1_direct_fallback", "id2_used", "id2_bad", "id2_direct", "id2_reuse_after_a_narrowing")}
snap_amd = {"head": "69e368d44bafbc1f96581d9d668e66355d9eb74c",
            "amendment_freeze_commit": "98a4fc6639 (AMENDMENT.md and amendment-1/src, 2026-11-15T05:01:37Z; amendment run.log starts 05:06:26Z)",
            "analysis_commit": "ad67c57e1f (amendment-1/analyze.py = ../analyze.py with one line changed: R = HERE/../.. instead of HERE/..; committed 05:08:35Z, after the runs started, but it is the frozen script)",
            "data_commit": "69e368d44b (runs-blr1-20261115.tar.gz, analysis.json, tables.md, RESULT.md)",
            "source_diff": "amendment-1/src vs src: la2.rs 3 lines (narrowed0 -> narrowed_any, set on every narrowing, guard of the V_i^T V0 recurrence); v3.rs only the 5 #[path] includes gain one ../; ic2.rs, rho2.rs, solver2.rs, summ2.rs byte-identical",
            "dirty_paths_before_validation": [], "remote_main_at_nonce_time": nonce_str.split()[0] + " (git ls-remote; not fetched)",
            "observation": "the amendment run.log has two 'start' lines (05:06:26Z, 05:08:20Z) and one 'done'; run-local.sh skips finished units, and all 30 BLR1 units carry their unit record with rc 0; RESULT.md states an earlier ad-hoc rerun without unit records was discarded. Counted output is deterministic and every BLR1 record was re-derived here (oracle build on all 30 cells), so the restart has no bearing on the numbers."}
amd = {
    "claim_under_test": "Amendment 1: BLR1 = BLR with identity 2 disabled after any narrowing (narrowed_any). BLR1 ran the 30 BLR units; differs from BLR only in DL linear_algebra.lmul/ladd of the 23 units whose single narrowing is in the final iteration (+3,352..13,800 lmul); the study's numbers are re-derived from BLR1 (SUPPORTED; DL ratio worst seed 0.8223/0.8656/0.8859 at k = 18 ... 0.8129/0.8687/0.8905 at k = 22; k = 22 BLR1 mean 8.4401e8 / 1.01401e9 / 2.54399e9 M-eq at w = 0 / 0.1 / 1).",
    "status": "verified", "scalars_verified": True, "validated_snapshot": snap_amd,
    "binaries": {"BLR1": {"sha256": bins["blr1"], "recorded": "e236855d7fef58d1c6618b25c399e462c7099bcb1f634047a7258e5f5d074035",
                          "matches_recorded": bins["blr1"] == "e236855d7fef58d1c6618b25c399e462c7099bcb1f634047a7258e5f5d074035",
                          "build": f"cd {STUDY}/amendment-1/src && rustc --edition 2021 -O v3.rs --cfg combo -o {SCR}/bin/blr1 (also from a scratch copy of the tree: same sha256)"},
                 "REF": {"sha256": bins["ref"], "matches_recorded": bins["ref"] == "00355034924e06661aed665f433063524c2e3964839c3ca671d54d64aaaa19a9", "build": "round-1 validator build (rebuilt bit-identically)"},
                 "AUD1": {"sha256": bins["aud1"], "build": "amendment-1/src tree with la2.rs from validation/amendment-1/scripts/make_audit_la2_v2.py (v2 = v1 with the identity-2 use condition read from the audited source; v2 output on the frozen BLR la2.rs is byte-identical to v1's)"},
                 "stress_harness": sbins, "rustc": "rustc 1.97.0 (2d8144b78 2026-07-07)"},
    "identity_2_after_guard": "With narrowed_any set inside the narrowing block (before vt0 is formed), the recurrence is used at iteration i only if no iteration 0..i narrowed. Then V_j = V_{j,full} for every j <= i, so V_a^T M V_b = V_{a+1}^T M V_{b-1} = ... = 0 for all a < b <= i (each chain step uses an unnarrowed V_{a+1}), in particular V_{i-1}^T M V_0 = 0 and v0 = V_0; and sel_used is the identity. So V_i^T V_0 = -c1^T (V_{i-1}^T V0) - c2^T (V_{i-2}^T V0) exactly, in every path. Once set, the flag stays set and the direct gram is used for the rest of the call. The guard is conservative: it also uses the direct gram in the narrowing iteration itself, where the recurrence was still exact; that is the whole BLR1 - BLR difference.",
    "nonce": {"value": nonce, "derivation": f"sha256 of the UTF-8 string '{nonce_str}' = (`git ls-remote origin refs/heads/main`) + ' ' + (`date -u +%Y-%m-%dT%H:%M:%S.%NZ`), drawn after the amendment data commit 69e368d44b and after scripts/select_amd1.py was written, before any amendment unit ran",
              "void_derivation": f"a first derivation used a mistyped input string ('{void_str}' -> {void_nonce}) that is not what the two commands returned; it was discarded before any unit ran and replaced by the derivation above from the values actually returned. Its plan is kept in the replay tarball as plan.VOID-mistyped-input.json."},
    "selection_rule": "amendment-1/scripts/select_amd1.py: universe q x seed 1..12, rank by sha256('<nonce>:amd1:<q>:<seed>'); first 4, extended until >= 1 seed in 7..12, >= 1 with q >= 2097091, >= 1 seed in 1..6 (no extension needed); block 8 and block 2 at the first two ranked pairs with q <= 1048507. Each configuration ran REF, BLR1 and AUD1; AUD1 also ran on the 28 producer cells not selected, so all 30 producer cells are oracle-audited.",
    "sample": plan["sample"], "block_units": plan["block_units"],
    "comparison_rule": "as round 1 (counted fields = all keys except *wall, *cpu, load1, s.controls*; arms/attempts equal). BLR1 and AUD1 vs replayed REF: all counted fields except DL linear_algebra.lmul/ladd, plus s.bl_iters, s.bl_narrowed. AUD1 vs replayed BLR1 and vs the committed amendment BLR1 records: every counted field. Replayed REF/BLR1 at block 4, seeds 1-6 vs the committed amendment records: every counted field. Replayed REF seeds 1-3 vs fasthash p1/p2_COMBO.",
    "fields_compared": cmpj["fields_compared_non_negative_roles"], "total_mismatches": cmpj["total_mismatches_non_negative_roles"], "mismatches": [],
    "coverage": {"configurations": len(plan["sample"]) + len(plan["block_units"]), "seeds_outside_1_6": sorted({u["seed"] for u in plan["sample"] if u["seed"] > 6}),
                 "q_ge_2097091": sorted({u["q"] for u in plan["sample"] if u["q"] >= 2097091}), "block_widths": [u["block"] for u in plan["block_units"]],
                 "producer_cells_oracle_audited": len(cells), "runs": len(runs)},
    "all_exit_codes_zero": all(r["exit_code"] == 0 for r in runs), "all_ok_and_k_found_equals_k": all(r["k_found_equals_k_and_ok"] for r in runs),
    "oracle_audit": {"totals": otot, "reconciliation_runs": len(recon), "reconciliation_all_exact": True,
                     "meaning": "both reuses equal the direct grams at every iteration of every audited run; REF - BLR1 linear_algebra lmul/ladd equals the skipped grams minus the recurrence exactly in every run (vs replayed or committed REF)"},
    "stress_test": {"what": "round-1 harness (scripts/bltest.rs.in, 20,000 trials, seed 20261115) with the amended la2.rs (BLR1) and its oracle build (AUD1); REF outputs are the round-1 ones (same binary)",
                    "summary": st["summary"], "by_class": st["by_class"],
                    "result": "0 inexact uses of identity 2 (was 19,631 in 1,922 calls), 0 of identity 1, 0 REF/BLR1 differences in kernel vector, iterations or narrowing in all 20,000 trials incl. the 2,990 mid-run narrowings (was 24 kernel differences); count reconciliation exact in all 19,842 audited calls (was 18,813)"},
    "blr1_vs_blr": {k: v for k, v in dl.items() if k != "cells"},
    "ratios_record": cmpj["replayed_ratios"],
    "raw_replay_files": {"tarball": TAR, "sha256": sha(os.path.join(V, TAR)), "inner_sums": "amendment-1/replay/SHA256SUMS.inner"},
}
cert2 = dict(cert)
cert2["claim_under_test"] = cert["claim_under_test"] + " AMENDMENT 1 (validated below in amendment_1): " + amd["claim_under_test"]
cert2["runs"] = cert["runs"] + [dict(r, amendment=1) for r in runs]
cert2["total_mismatches"] = cert["total_mismatches"] + amd["total_mismatches"]
cert2["fields_compared"] = cert["fields_compared"] + amd["fields_compared"]
cert2["all_exit_codes_zero"] = cert["all_exit_codes_zero"] and amd["all_exit_codes_zero"]
cert2["all_ok_and_k_found_equals_k"] = cert["all_ok_and_k_found_equals_k"] and amd["all_ok_and_k_found_equals_k"]
cert2["amendment_1"] = amd
cert2["supersedes"] = {"sha256": PREV_CERT, "commit": "a1e16d39e4", "note": "round-1 certificate; this version keeps every round-1 field and run and adds amendment_1 (runs tagged amendment: 1); totals are round 1 + amendment"}
cert2["notes"] = cert["notes"] + ["Amendment 1 resolves validator-report discrepancy D1: the guarded build is exact in every path (proof in amendment_1.identity_2_after_guard; stress harness 0 inexact uses)."]
json.dump(cert2, open(cert_p, "w"), indent=1); open(cert_p, "a").write("\n")

# ---------------------------------------------------------------- report
H = []
def h(name, claimed, recomputed, readme=None):
    d = {"name": name, "claimed": claimed, "recomputed": recomputed, "rel_tol": 1e-9}
    if readme is not None: d["readme"] = readme
    d["agrees"] = abs(claimed - recomputed) <= 1e-9 * max(abs(claimed), abs(recomputed), 1e-300)
    H.append(d)
worst = {"262111": ("0.8223", "0.8656", "0.8859"), "524287": ("0.8136", "0.8651", "0.8863"), "1048507": ("0.8163", "0.8663", "0.8870"),
         "2097091": ("0.8194", "0.8700", "0.8905"), "4194271": ("0.8129", "0.8687", "0.8905")}
for wi, w in enumerate(("0.0", "0.5", "1.0")):
    for q in QS:
        k = f"DL/{q}/{w}/0.0"
        h(f"[amendment 1] DL cold M-eq BLR1/REF geomean over seeds 1-6, q={q}, w_add={w}, w_h=0", cl["ratios"][k]["geomean"], rc["ratios"][k]["geomean"])
        h(f"[amendment 1] DL cold M-eq BLR1/REF max over seeds 1-6, q={q}, w_add={w}, w_h=0", cl["ratios"][k]["max"], rc["ratios"][k]["max"], f"RESULT.md {worst[str(q)][wi]}")
    per_c = [x for q in QS for x in cl["ratios"][f"DL/{q}/{w}/0.0"]["per_seed"]]; per_r = [x for q in QS for x in rc["ratios"][f"DL/{q}/{w}/0.0"]["per_seed"]]
    h(f"[amendment 1] DL ratio min over the 30 (q, seed) cells, w_add={w}", min(per_c), min(per_r))
    h(f"[amendment 1] DL ratio max over the 30 (q, seed) cells, w_add={w}", max(per_c), max(per_r))
for wh in ("0.1", "1.0"):
    k = f"DL/4194271/0.0/{wh}"
    h(f"[amendment 1] DL ratio geomean k=22, w_add=0, hash+cmp at {wh} GAE", cl["ratios"][k]["geomean"], rc["ratios"][k]["geomean"])
h("[amendment 1] max DL ratio over every cell, w_add and w_h", max(x for k, v in cl["ratios"].items() if k.startswith("DL/") for x in v["per_seed"]),
  max(x for k, v in rc["ratios"].items() if k.startswith("DL/") for x in v["per_seed"]))
for q in QS:
    h(f"[amendment 1] DL linear_algebra.lmul BLR1/REF geomean, q={q}", cl["la_lmul_ratio"][str(q)]["geomean"], rc["la_lmul_ratio"][str(q)]["geomean"])
g = cl["gap_k22"]
for w, rd in (("0", "8.4401e8"), ("0.1", "1.01401e9"), ("1", "2.54399e9")):
    h(f"[amendment 1] k=22 IC REF mean cold M-eq (seeds 1-3, w_add 0, hash+cmp at {w}*gae)", g["ic_mean_meq"][f"REF/{w}"], rc["gap_k22"][w]["ic_ref"])
    h(f"[amendment 1] k=22 IC BLR1 mean cold M-eq (seeds 1-3, w_add 0, hash+cmp at {w}*gae)", g["ic_mean_meq"][f"BLR/{w}"], rc["gap_k22"][w]["ic_blr"], f"RESULT.md {rd}")
    h(f"[amendment 1] k=22 rho/IC BLR1 at w={w}", g["rho_over_ic"][f"BLR/{w}"], rc["gap_k22"][w]["rho_over_ic_blr"])
    h(f"[amendment 1] k=22 IC/rho BLR1 at w={w}", g["ic_over_rho"][f"BLR/{w}"], rc["gap_k22"][w]["ic_over_rho_blr"])
    h(f"[amendment 1] k=22 frozen band ic reproduced by REF at w={w}", rc["gap_k22"][w]["ic_frozen"], rc["gap_k22"][w]["ic_ref"])
h("[amendment 1] BLR1 - BLR DL linear_algebra.lmul, min over the 23 narrowing cells", 3352, dl["d_lmul_range_narrowed"][0], "RESULT.md 3,352")
h("[amendment 1] BLR1 - BLR DL linear_algebra.lmul, max over the 23 narrowing cells", 13800, dl["d_lmul_range_narrowed"][1], "RESULT.md 13,800")
assert all(x["agrees"] for x in H)

raw_old = [x for x in rep["raw_data"]]
add = [("../amendment-1/runs-blr1-20261115.tar.gz", "amendment 1 raw JSONL: 30 BLR1 units + 30 REF units (byte-identical to the round-1 REF files) + binary.sha256, run.log, SHA256SUMS (123/123 inner OK); the headline's BLR1 side"),
       ("../amendment-1/AMENDMENT.md", "amendment 1, frozen before its runs (98a4fc6639)"),
       ("../amendment-1/analysis.json", "amendment 1 analysis output (claimed values, read by key)"),
       ("../amendment-1/src/la2.rs", "BLR1 change set (narrowed_any guard)"),
       (TAR, "validator amendment replay JSONL (46 runs, stderr with BLAUDIT oracle lines), plan, nonce (and the void derivation), run log, stress outputs digests"),
       ("amendment-1/recompute-amd1.json", "validator recompute of the amended headline (scripts/recompute.py on the amendment tarball)"),
       ("amendment-1/replay-compare-amd1.json", "validator amendment replay comparison (amendment-1/scripts/compare_amd1.py)"),
       ("amendment-1/stress-summary-amd1.json", "validator stress comparison REF vs BLR1 vs AUD1 (scripts/stress_compare.py)"),
       ("amendment-1/delta-amd1.json", "BLR1 vs frozen BLR per cell (amendment-1/scripts/delta_amd1.py)")]
raw = [{"path": x["path"], "repo_path": x["repo_path"], "sha256": sha(os.path.join(V, x["path"])), "role": ("round 1: " + x["role"])} for x in raw_old] + \
      [{"path": p, "repo_path": os.path.normpath(os.path.join(STUDY, "validation", p)), "sha256": sha(os.path.join(V, p)), "role": r} for p, r in add]
disc = rep["discrepancies"]
d1 = dict(disc[0]); d1["status"] = "resolved by amendment 1"
d1["resolution"] = ("amendment-1 (98a4fc6639, data 69e368d44b): the guard is narrowed_any, set on every narrowing before vt0 is formed, so identity 2 is used only "
                    "while no iteration has narrowed, where it is exact (proof in the replay certificate, amendment_1.identity_2_after_guard). Stress harness on the amended "
                    "la2.rs: 0 inexact uses in 80,955, 0 kernel/iteration/narrowing differences from REF in 20,000 trials incl. all 2,990 mid-run narrowings. The study's "
                    "numbers are now BLR1's: they differ from the frozen BLR only in DL linear_algebra lmul/ladd of the 23 narrowing cells (+3,352..13,800 lmul, <= 0.041%), "
                    "largest ratio change 0.00021; the headline below is BLR1's.")
d4 = {"id": "D4", "status": "open (coordinator record)", "severity": "record consistency; no effect on the validated numbers",
      "what": "VALIDATION.json in both directories (a1e16d39e4) predates the amendment results: (a) its sha256 pins name the round-1 report/certificate and no longer match the amended files; (b) binary_or_process says the amendment-1 guarded build is 'identical in every counted field on all 30 units', which RESULT.md and this validation contradict (23 cells differ in DL linear_algebra lmul/ladd); (c) gae_band ic holds the frozen-BLR k = 22 means (843987004.8361, 1013984140.2361, 2543958358.8361) while the amended study's IC side is BLR1's (844014769.0561, 1014011904.4561, 2543986123.0561; relative change 3.3e-5, 2.7e-5, 1.1e-5). The validator did not edit VALIDATION.json (outside this task); it needs the new pins, the BLR1 binary (e236855d...) and the BLR1 band values."}
rep2 = dict(rep)
rep2["status"] = "pass"
rep2["validated_snapshot"] = dict(rep["validated_snapshot"], amendment_1=snap_amd)
rep2["replay_certificate"] = {"path": "replay-certificate.json", "sha256": sha(cert_p)}
rep2["raw_data"] = raw
rep2["headline"] = H
rep2["headline_note"] = "Headline items are the amended (BLR1) values: claimed from amendment-1/analysis.json (and RESULT.md where marked), recomputed by the validator from amendment-1/runs-blr1-20261115.tar.gz. The round-1 (frozen BLR) headline is in the superseded report (sha256 " + PREV_REP + ", commit a1e16d39e4); it agreed exactly as well."
rep2["decisions"] = {"SUPPORTED_recomputed": rc["decision"]["SUPPORTED"], "SUPPORTED_claimed": cl["supported"], "holds_all_w_h": rc["decision"]["holds_all_wh"],
                     "D_DL1_exactly_1": rc["decision"]["D_DL1_exactly_1"], "ref_reproduces_frozen_band_1e9": rc["ref_reproduces_band_1e9"],
                     "gates": {k: rc["gates"][k]["pass"] for k in ("G1", "G2", "G3", "G4")},
                     "analysis_json_cells_equal": f"{rc['vs_claimed']['ratio_cells_compared']} ratio cells + per-seed values, max relative difference {rc['vs_claimed']['ratio_max_rel_diff']}; LA ratios {rc['vs_claimed']['la_max_rel_diff']}",
                     "amendment_pre_registered_rule": "AMENDMENT.md: pass iff every BLR1 unit identical to BLR, else re-derive the numbers from BLR1; not identical (23/30), so re-derived, as RESULT.md states",
                     "round_1_decision": rep["decisions"]}
rep2["checks"] = dict(rep["checks"], amendment_1={
    "artifacts": f"amendment tarball sha256 {sha(os.path.join(AM, 'runs-blr1-20261115.tar.gz'))}, 123/123 inner OK; 60 REF files byte-identical to round 1; 30 BLR1 units rc 0",
    "build": "BLR1 rebuilt bit-identically (e236855d7fef...); REF 00355034924e...",
    "freeze": snap_amd["source_diff"] + "; amendment frozen 05:01:37Z before the runs (05:06:26Z); analyze.py differs from the frozen one in the repository path only",
    "replay": f"{len(runs)} runs ({len([r for r in runs if r['arm'] != 'AUD1'])} REF/BLR1, {len(aud)} AUD1), {cmpj['fields_compared_non_negative_roles']} counted-field comparisons, 0 mismatches; seeds outside 1-6 and block widths 8, 2",
    "identity_2": amd["identity_2_after_guard"], "stress": amd["stress_test"]["result"],
    "reconciliation": f"REF - BLR1 = skipped grams - recurrence exactly in {len(recon)} audited runs",
    "blr1_vs_blr": f"{dl['narrowed_cells_differing']}/{dl['narrowed_cells']} narrowing cells differ, only in DL linear_algebra lmul/ladd (+{dl['d_lmul_range_narrowed'][0]}..{dl['d_lmul_range_narrowed'][1]} lmul, max rel {dl['d_lmul_rel_max']:.2e}); {dl['unnarrowed_cells_differing']}/{dl['unnarrowed_cells']} unnarrowed cells differ; largest ratio change {dl['max_abs_change_any_ratio_cell']:.5f}"})
rep2["discrepancies"] = [d1] + disc[1:] + [d4]
rep2["limitations"] = rep["limitations"] + ["Amendment 1 is validated on the same L0 container and session; its runs and this replay are counted-work comparisons only."]
rep2["method"] = rep["method"] + " Amendment 1: amendment-1/scripts/ (select_amd1.py, replay_amd1.sh, compare_amd1.py, make_audit_la2_v2.py, delta_amd1.py, make_reports_amd1.py); scripts/recompute.py and scripts/stress_compare.py reused unchanged."
rep2["validation_report"] = dict(rep["validation_report"], id="VAL-20261115-755b33", supersedes="VAL-20261115-b1e7a3",
    run_ids=rep["validation_report"]["run_ids"] + ["research/ic_gw_bl_reuse_20261115/amendment-1 runs-blr1-20261115 (30 BLR1 units, 69e368d44b)"],
    metric_recomputations=["amended: 405 cells = amendment-1/analysis.json (rel diff 0), SUPPORTED, k=22 BLR1 means equal RESULT.md"] + rep["validation_report"]["metric_recomputations"],
    proof_architecture_checks=["identity 1 exact in all paths", "identity 2 exact in every path under the amendment-1 guard (D1 resolved)"],
    artifact_paths=rep["validation_report"]["artifact_paths"] + [f"{STUDY}/validation/{TAR}"], verdict="passed")
att = dict(rep["review_attestation"])
att["sources_read"] = att["sources_read"] + [f"{STUDY}/amendment-1/AMENDMENT.md", f"{STUDY}/amendment-1/RESULT.md", f"{STUDY}/amendment-1/analysis.json (by key)",
                                            f"{STUDY}/amendment-1/src/la2.rs and v3.rs (diff against src/)", f"{STUDY}/amendment-1/analyze.py (diff against ../analyze.py only)",
                                            f"{STUDY}/validation/VALIDATION.json and benchmarks/20261115-ic-gw-bl-reuse/VALIDATION.json"]
att["not_read"] = att["not_read"] + [f"{STUDY}/amendment-1/analyze.py body (only its one-line diff)", f"{STUDY}/amendment-1/tables.md"]
rep2["review_attestation"] = att
rep2["supersedes"] = {"sha256": PREV_REP, "commit": "a1e16d39e4"}
json.dump(rep2, open(rep_p, "w"), indent=1); open(rep_p, "a").write("\n")

open(os.path.join(B, "replay-certificate.json"), "wb").write(open(cert_p, "rb").read())
rb = json.loads(json.dumps(rep2))
rb["copy_of"] = {"path": f"{STUDY}/validation/validator-report.json", "sha256": sha(rep_p),
                 "note": "same content; raw_data[].path rebased to benchmarks/20261115-ic-gw-bl-reuse/, replay_certificate points to the byte-identical copy in this directory"}
for it in rb["raw_data"]:
    it["path"] = os.path.relpath(os.path.join(REPO, it["repo_path"]), B)
rb["raw_data_path_base"] = "paths in raw_data[].path are relative to benchmarks/20261115-ic-gw-bl-reuse/ (the directory of this copy); repo_path is repository-relative"
rb["supersedes"] = {"sha256": PREV_REP_BENCH, "commit": "a1e16d39e4"}
json.dump(rb, open(os.path.join(B, "validator-report.json"), "w"), indent=1); open(os.path.join(B, "validator-report.json"), "a").write("\n")
print("ok", len(H), "headline items")
