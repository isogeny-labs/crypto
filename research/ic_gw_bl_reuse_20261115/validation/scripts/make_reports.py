#!/usr/bin/env python3
"""Writes validation/replay-certificate.json and validation/validator-report.json for ic_gw_bl_reuse_20261115
from the validator's own outputs (recompute.json, replay-compare.json, stress-summary.json, the replay tarball,
replay/plan.json and replay/nonce.txt inside it) and the producer's claimed analysis.json (read by key).
usage: make_reports.py STAGE_DIR   (STAGE_DIR holds plan.json, nonce.txt, binary.sha256, stress-binaries.sha256)"""
import hashlib, json, os, sys

REPO = "/home/user/crypto"
STUDY = "research/ic_gw_bl_reuse_20261115"
V = os.path.join(REPO, STUDY, "validation")
stage = sys.argv[1]
sha = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest()
J = lambda p: json.load(open(p))
rc = J(os.path.join(V, "recompute.json"))
cmpj = J(os.path.join(V, "replay-compare.json"))
st = J(os.path.join(V, "stress-summary.json"))
cl = J(os.path.join(REPO, STUDY, "analysis.json"))
plan = J(os.path.join(stage, "plan.json"))
nonce_str, nonce = open(os.path.join(stage, "nonce.txt")).read().split("\n")[:2]
bins = {os.path.basename(l.split()[1]): l.split()[0] for l in open(os.path.join(stage, "binary.sha256"))}
sbins = {os.path.basename(l.split()[1]): l.split()[0] for l in open(os.path.join(stage, "stress-binaries.sha256"))}
QS = [262111, 524287, 1048507, 2097091, 4194271]
SCR = "/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/val_blr"
TAR = "replay/replay-runs-20261115.tar.gz"

runs = cmpj["runs"]
main_runs = [r for r in runs if r["role"] != "neg"]
neg_runs = [r for r in runs if r["role"] == "neg"]
aud = [r for r in runs if r["arm"] == "AUD"]
all_verified = all(r.get("verified") for r in main_runs)
assert all_verified and cmpj["total_mismatches_non_negative_roles"] == 0, "replay not verified"
assert all(r["negative_control"]["detected"] for r in neg_runs)
aud_cells = sorted({(r["q"], r["seed"]) for r in aud if r["block"] == 4 and r["seed"] <= 6})
oracle_tot = {k: sum(r["oracle"][k] for r in aud) for k in ("id1_used", "id1_bad", "id1_direct_fallback", "id2_used", "id2_bad", "id2_direct", "id2_reuse_after_a_narrowing")}
recon = [r for r in aud if "reconciliation" in r]
assert all(r["reconciliation"]["exact"] for r in recon)
narrow = sorted({("none" if not n[2] else ("last iteration only" if all(x == n[0] - 1 for x in n[2]) else "other")) for r in aud for n in r["oracle"]["narrowing"]})

snapshot = {
    "repository": "/home/user/crypto (branch claude/f4-f5-algorithm-speed-bvp7qd)",
    "head": "c5bf40cee6f291a301f7384dd941751c8622af28",
    "data_commit": "c5bf40cee6 (ic_gw_bl_reuse_20261115: results, report, benchmark record)",
    "protocol_freeze_commit": "5c9dc91c48 (PROTOCOL.md, analyze.py, make-units.sh, run-local.sh, units.tsv, src/ added before data; committed 04:32:34Z, run.log start 04:32:47Z)",
    "freeze_integrity": "git diff 5c9dc91c48..HEAD is empty for PROTOCOL.md, src/, units.tsv, make-units.sh, run-local.sh, analyze.py and for ../ic_gw_fasthash_20261114/src; BLR src/ differs from ../ic_gw_fasthash_20261114/src only in la2.rs (ic2.rs, rho2.rs, solver2.rs, summ2.rs, v3.rs byte-identical); the shared includes research/quartic_n4_20261019/{common,ghs,ic,solver}.rs and src/cryptanalysis/plane_quartic_pic.rs are clean in the work tree and are bound by the bit-identical REF rebuild",
    "dirty_paths_under_study_before_validation": [],
    "remote_main_at_nonce_time": "34a4f6f2904396635bb935e1c02220ffa86661f9 (git ls-remote; not fetched)",
}
binaries = {
    "REF": {"sha256": bins["ref"], "recorded": "00355034924e06661aed665f433063524c2e3964839c3ca671d54d64aaaa19a9",
            "matches_recorded": bins["ref"] == "00355034924e06661aed665f433063524c2e3964839c3ca671d54d64aaaa19a9",
            "build": f"cd research/ic_gw_fasthash_20261114/src && rustc --edition 2021 -O v3.rs --cfg combo -o {SCR}/bin/ref"},
    "BLR": {"sha256": bins["blr"], "recorded": "13959249d43bdc5582dfda9427ede06334f0c880b2d3d4ffbe03928ea9c003b8",
            "matches_recorded": bins["blr"] == "13959249d43bdc5582dfda9427ede06334f0c880b2d3d4ffbe03928ea9c003b8",
            "build": f"cd research/ic_gw_bl_reuse_20261115/src && rustc --edition 2021 -O v3.rs --cfg combo -o {SCR}/bin/blr",
            "note": "also rebuilt from a scratch copy of the tree (same relative include layout): same sha256"},
    "AUDIT": {"sha256": bins["audit"], "build": "BLR tree with la2.rs replaced by scripts/make_audit_la2.py output; rustc --edition 2021 -O v3.rs --cfg combo",
              "what": "oracle build: wherever BLR reuses a value it also computes the direct gram it replaced, with LMUL/LADD saved before and restored after, compares exactly, and logs the skipped grams' counted cost and the recurrence's counted cost (stderr BLAUDIT line per block_lanczos call); counted output must equal BLR's"},
    "MUT_C2": {"sha256": bins["mut_c2"], "build": "AUDIT tree with --cfg mut_c2 (recurrence for V_i^T V0 drops the c2^T (V_{i-2}^T V0) term)"},
    "MUT_PM": {"sha256": bins["mut_pm"], "build": "AUDIT tree with --cfg mut_pm (reused MV_{i-1}^T MV_i replaced by zeros)"},
    "stress_harness": {k: v for k, v in sbins.items()},
    "rustc": "rustc 1.97.0 (2d8144b78 2026-07-07)",
}
claim = ("BLR = two block-Lanczos reuses in src/la2.rs block_lanczos (MV_{i-1}^T MV_i from the columns sel_used of the same iteration's "
         "V_full^T M V_full; V_i^T V_0 for i >= 2 from the three-term recurrence when iteration 0 did not narrow). Every counted field "
         "identical to REF (ic_gw_fasthash COMBO) except DL linear_algebra.lmul/ladd; DL cold M-eq BLR/REF 0.81-0.82 (w_add 0), "
         "0.86-0.87 (0.5), 0.88-0.89 (1) at every k = 18..22 and seed 1-6; logs unchanged; k = 22 IC/negation-map-rho 0.00222 / 0.00207 / "
         "0.00170 at w = 0 / 0.1 / 1 against the frozen band of research/quartic_dl1_cd_sqrt_20261113/combo/validation/VALIDATION.json. "
         "[pcc:unverified], L0 local.")
indep = ("Validator subagent with its own context, nonce, sample, runner, comparator, oracle build, stress harness and recompute script; "
         "the producer's analyze.py and plot.py were not read or imported (analyze.py only hashed). It runs in the same Claude Code "
         "session and container as the producer, so independence is of reasoning and code, not of host or organization; no keyring signature.")
identities = {
    "setup": "M = B^T D B (symmetric; mblk). Iteration i: V_full (k_full columns) -> optional narrowing to V_i = V_full[:, sel] with "
             "W_i = V_i^T M V_i invertible; c1_i = W_i^{-1} (MV_i^T MV_i); c2_i = W_{i-1}^{-1} (MV_{i-1}^T MV_i); "
             "V_{i+1,full} = MV_i - V_i c1_i - V_{i-1} c2_i (all with the narrowed V_{i-1}, MV_{i-1}, W_{i-1}^{-1} stored in vp). Arithmetic is exact in F_ell.",
    "P_and_Q_unconditional": "P(i+1): V_i^T M V_{i+1,full} = MV_i^T MV_i - W_i W_i^{-1} MV_i^T MV_i - (V_i^T M V_{i-1}) c2_i = 0 given P(i). "
             "Q(i+1): V_{i-1}^T M V_{i+1,full} = MV_{i-1}^T MV_i - (V_{i-1}^T M V_i) c1_i - W_{i-1} W_{i-1}^{-1} pm_i = 0 given P(i) and pm_i exact. "
             "Both hold for any column selection of V_{i+1,full} and are independent of narrowing at any iteration.",
    "identity_1": "pm_BLR = rows 0..k_{i-1} of V_full^T M V_full at columns sel_used = V_full^T M V_i = (MV_{i-1} - V_{i-1} c1 - V_{i-2} c2)^T M V_i "
                  "= MV_{i-1}^T MV_i - c1^T P(i) - c2^T Q(i) = MV_{i-1}^T MV_i exactly, in every path (narrowing anywhere, iteration 1 without V_{i-2}). "
                  "Row p of V_full corresponds to column p of MV_{i-1}; W_full is symmetric so the row/column orientation is immaterial. The fallback "
                  "(vpr.k != k_full) is unreachable because V_full of iteration i always has exactly k_{i-1} = vpr.k columns (harmless dead branch).",
    "identity_2": "V_{i,full}^T V_0 = (MV_{i-1})^T V_0 - c1^T (V_{i-1}^T V_0) - c2^T (V_{i-2}^T V_0); the code drops the first term, i.e. it needs "
                  "V_{i-1}^T M V_0 = 0. For i = 2 that is P(1) and for i = 3 it is Q(2) (unconditional; narrowing at iteration 0 is excluded because "
                  "v0 keeps all b columns). For i >= 4 it is long-range M-orthogonality, which by V_a^T M V_b = V_{a+1,full}^T M V_{b-1} (b - a >= 3) "
                  "holds only while no earlier iteration j >= 1 dropped columns U_j with U_j^T M V_{later} != 0. The guard checks only narrowing "
                  "at iteration 0, so identity 2 is NOT exact in general after a mid-run narrowing (see discrepancies D1). Indices, transposes "
                  "(c1[p*k_full+r] = c1[p, r] summed over p), the sign (ls2(0, s)), the history (h_vt0/h_k = iterations i-2, i-1; last_c1/last_c2 = c1_{i-1}, c2_{i-1}) "
                  "and the row selection sel_used are correct. Narrowing at the current iteration itself is exact (full recurrence, then rows sel_used).",
    "counting": "The recurrence uses lm2/la2/ls2 (counted as lmul/ladd by the same global counters as every other mod-ell operation); identity 1 "
                "is copies only (uncounted, as copies are in REF). No counted work moves out of block_lanczos: REF - BLR equals the skipped direct "
                "grams minus the recurrence exactly in every audited run (lmul and ladd), and in all 17,920 stress calls where identity 2 was exact.",
}
cert = {
    "schema": "replay-certificate/v1", "study": STUDY, "claim_under_test": claim,
    "status": "verified", "independent": True, "independence_note": indep, "scalars_verified": True,
    "validated_snapshot": snapshot, "binaries": binaries,
    "nonce": {"value": nonce, "derivation": f"sha256 of the UTF-8 string '{nonce_str}' = (origin main HEAD from `git ls-remote origin refs/heads/main`) + ' ' + (`date -u +%Y-%m-%dT%H:%M:%S.%NZ`), drawn after the data commit c5bf40cee6 and after scripts/select.py was written, before any unit was selected or run",
              "remote_main_head": nonce_str.split()[0], "utc_time": nonce_str.split()[1]},
    "selection_rule": "scripts/select.py (written before the nonce; its first version had a tuple-unpacking bug that raised before any output, fixed without changing the rule). Universe q in {262111, 524287, 1048507, 2097091, 4194271} x seed 1..12; rank by sha256('<nonce>:<q>:<seed>'); first 6, extended until >= 2 seeds in 7..12, >= 2 with q >= 2097091, >= 2 seeds in 1..6 (no extension needed). Block-width units: the first four ranked pairs with q <= 1048507 get block 8, 2, 1, 3. Then the AUDIT build on the 26 producer cells not in the sample (all 30 producer cells audited).",
    "sample": plan["sample"], "block_units": plan["block_units"],
    "commands": {"block 4 (producer form)": "<BIN> ic <q> <seed> D 3.0 1.7", "other block widths": "<BIN> ic <q> <seed> D 3.0 1.7 4.0 <block>",
                 "runner": "scripts/replay.sh (xargs -P 3, counted work only)"},
    "comparison_rule": "counted fields = every key of every ic record except those ending in wall/cpu, load1, s.controls*; key lists must be equal in order; record arms/attempts must be equal. BLR and AUDIT vs replayed REF: all counted fields equal except DL linear_algebra.lmul/ladd, plus s.bl_iters and s.bl_narrowed equal. AUDIT vs BLR (replayed and committed): every counted field equal. Replayed REF/BLR at block 4, seeds 1-6 vs the committed records: every counted field equal. Replayed REF seeds 1-3 vs committed fasthash p1_COMBO and p2_COMBO: every counted field equal.",
    "fields_compared": cmpj["fields_compared_non_negative_roles"],
    "coverage": {"replayed_REF_BLR_pairs": len([r for r in runs if r["arm"] == "BLR"]),
                 "block4_pairs": len(plan["sample"]), "pairs_with_seed_outside_1_6": sum(1 for u in plan["sample"] if u["seed"] > 6),
                 "pairs_with_q_ge_2097091": sum(1 for u in plan["sample"] if u["q"] >= 2097091),
                 "block_width_pairs": [u["block"] for u in plan["block_units"]],
                 "producer_cells_audited_by_oracle": len(aud_cells), "audit_runs_total": len(aud)},
    "runs": [{k: v for k, v in r.items()} for r in runs],
    "total_mismatches": cmpj["total_mismatches_non_negative_roles"], "mismatches": [],
    "all_exit_codes_zero": all(r["exit_code"] == 0 for r in runs),
    "all_ok_and_k_found_equals_k": all(r["k_found_equals_k_and_ok"] for r in main_runs),
    "oracle_audit": {"totals": oracle_tot, "narrowing_pattern": narrow, "reconciliation_runs": len(recon), "reconciliation_all_exact": True,
                     "meaning": "In every audited run (all 30 producer cells, the 6 sampled pairs incl. seeds 9 and 10, block widths 1, 2, 3, 8) both reused quantities equal the direct grams bit for bit at every iteration; narrowing occurs only in the final iteration, so no identity-2 reuse ever followed a narrowing."},
    "identity_proofs": identities,
    "stress_test": {"what": "scripts/bltest.rs.in drives la2::block_lanczos directly (REF la2, BLR la2, AUDIT la2) on 20,000 random sparse matrices, ell in {3, 5, 7, 11, 13, 101, 65537, 4501899029752451}, n 4..63, block 1/2/3/4/8, seed 20261115",
                    "summary": st["summary"], "by_class": st["by_class"],
                    "examples": st["first_divergent_examples"][:4]},
    "negative_controls": [{"unit": r["unit"], "arm": r["arm"], **r["negative_control"]} for r in neg_runs],
    "negative_controls_note": "MUT_PM (c2 built from a zero MV_{i-1}^T MV_i) loses M-orthogonality and every block_lanczos call returns early (singular narrowed W via sinv()? or the n + 10 iteration cap) before the oracle line is printed, so its oracle counts are 0 and s.bl_iters is 0; it is detected by the comparator (linear algebra failed, 6 DL attempts vs 1). MUT_C2 runs to the normal iteration count with identical s.bl_iters at attempt 1 and is detected both by the oracle (id2_bad > 0) and by the comparator, i.e. an inexact V^T V0 that leaves the iteration count unchanged still changes counted fields.",
    "ratios_record": cmpj["replayed_ratios"],
    "raw_replay_files": {"tarball": TAR, "sha256": sha(os.path.join(V, TAR)), "inner_sums": "replay/SHA256SUMS.inner"},
    "notes": ["The identity-2 defect (validator-report discrepancies D1) is outside the measured scope: it needs a narrowing before the last two iterations, which no audited real run has (ell >= 4.5e15 there); it was found only at ell <= 101 in the stress harness.",
              "Units ran up to 3 at a time on the 4-vCPU session container (L0); only counted fields are compared, CPU/wall are not."],
}
cert_p = os.path.join(V, "replay-certificate.json")
json.dump(cert, open(cert_p, "w"), indent=1); open(cert_p, "a").write("\n")

# ---------------------------------------------------------------- validator report
H = []
def h(name, claimed, recomputed, readme=None):
    d = {"name": name, "claimed": claimed, "recomputed": recomputed, "rel_tol": 1e-9}
    if readme is not None:
        d["readme"] = readme
    d["agrees"] = abs(claimed - recomputed) <= 1e-9 * max(abs(claimed), abs(recomputed), 1e-300)
    H.append(d)
for w in ("0.0", "0.5", "1.0"):
    for q in QS:
        k = f"DL/{q}/{w}/0.0"
        h(f"DL cold M-eq BLR/REF geomean over seeds 1-6, q={q}, w_add={w}, w_h=0", cl["ratios"][k]["geomean"], rc["ratios"][k]["geomean"])
        h(f"DL cold M-eq BLR/REF max over seeds 1-6, q={q}, w_add={w}, w_h=0", cl["ratios"][k]["max"], rc["ratios"][k]["max"])
    cmin = min(x for q in QS for x in cl["ratios"][f"DL/{q}/{w}/0.0"]["per_seed"]); cmax = max(x for q in QS for x in cl["ratios"][f"DL/{q}/{w}/0.0"]["per_seed"])
    rmin = min(x for q in QS for x in rc["ratios"][f"DL/{q}/{w}/0.0"]["per_seed"]); rmax = max(x for q in QS for x in rc["ratios"][f"DL/{q}/{w}/0.0"]["per_seed"])
    rng = {"0.0": "0.81-0.82", "0.5": "0.86-0.87", "1.0": "0.88-0.89"}[w]
    h(f"DL ratio min over the 30 (q, seed) cells, w_add={w}", cmin, rmin, f"task card {rng}")
    h(f"DL ratio max over the 30 (q, seed) cells, w_add={w}", cmax, rmax, f"task card {rng}")
for wh in ("0.1", "1.0"):
    k = f"DL/4194271/0.0/{wh}"
    h(f"DL ratio geomean k=22, w_add=0, hash+cmp at {wh} GAE", cl["ratios"][k]["geomean"], rc["ratios"][k]["geomean"], {"0.1": "0.839", "1.0": "0.929"}[wh])
h("max DL ratio over every cell, w_add and w_h (all 135 DL cells' seeds)", max(x for k, v in cl["ratios"].items() if k.startswith("DL/") for x in v["per_seed"]),
  max(x for k, v in rc["ratios"].items() if k.startswith("DL/") for x in v["per_seed"]))
for q in QS:
    h(f"DL linear_algebra.lmul BLR/REF geomean, q={q}", cl["la_lmul_ratio"][str(q)]["geomean"], rc["la_lmul_ratio"][str(q)]["geomean"])
g = cl["gap_k22"]
for w in ("0", "0.1", "1"):
    h(f"k=22 IC REF mean cold M-eq (seeds 1-3, w_add 0, hash+cmp at {w}*gae)", g["ic_mean_meq"][f"REF/{w}"], rc["gap_k22"][w]["ic_ref"])
    h(f"k=22 IC BLR mean cold M-eq (seeds 1-3, w_add 0, hash+cmp at {w}*gae)", g["ic_mean_meq"][f"BLR/{w}"], rc["gap_k22"][w]["ic_blr"])
    h(f"k=22 rho/IC BLR at w={w}", g["rho_over_ic"][f"BLR/{w}"], rc["gap_k22"][w]["rho_over_ic_blr"])
    h(f"k=22 IC/rho BLR at w={w}", g["ic_over_rho"][f"BLR/{w}"], rc["gap_k22"][w]["ic_over_rho_blr"], {"0": "0.00222", "0.1": "0.00207", "1": "0.00170"}[w])
    h(f"k=22 frozen band ic reproduced by REF at w={w} (frozen value vs REF recompute)", rc["gap_k22"][w]["ic_frozen"], rc["gap_k22"][w]["ic_ref"])
assert all(x["agrees"] for x in H)

raw = [
    ("../runs-blreuse-20261115.tar.gz", "producer raw JSONL of the 60 units + binary.sha256, host.txt, run.log, SHA256SUMS (123/123 inner OK)"),
    ("../units.tsv", "frozen unit list (5c9dc91c48)"),
    ("../PROTOCOL.md", "frozen protocol (5c9dc91c48)"),
    ("../analysis.json", "producer analysis output (claimed values, read by key)"),
    ("../src/la2.rs", "BLR change set (the only file differing from ../../ic_gw_fasthash_20261114/src)"),
    ("../../ic_gw_fasthash_20261114/src/la2.rs", "REF la2.rs"),
    ("../../ic_gw_fasthash_20261114/runs-fasthash-20261114.tar.gz", "committed fasthash COMBO records (G4 and replay REF comparison)"),
    ("../../quartic_n4v2_20261020/results/summary.json", "frozen r_inv, r_lmul, gae per size"),
    ("../../quartic_dl1_cd_sqrt_20261113/combo/validation/VALIDATION.json", "frozen negation-map rho band (gae_band)"),
    (TAR, "validator replay JSONL (34 + 26 runs, stderr incl. BLAUDIT oracle lines), plan, nonce, run log, stress-harness outputs"),
    ("recompute.json", "validator recompute (scripts/recompute.py)"),
    ("replay-compare.json", "validator replay comparison (scripts/compare.py)"),
    ("stress-summary.json", "validator block_lanczos stress comparison (scripts/stress_compare.py)"),
]
raw_data = [{"path": p, "repo_path": os.path.normpath(os.path.join(STUDY, "validation", p)), "sha256": sha(os.path.join(V, p)), "role": role} for p, role in raw]
cert_sha = sha(cert_p)
disc = [
    {"id": "D1", "severity": "substantive for the mathematical claim; no effect on any measured number",
     "what": "Identity 2 is not an exact identity of this implementation. Its derivation needs V_{i-1}^T M V_0 = 0, which holds unconditionally only for i <= 3; for i >= 4 it needs long-range M-orthogonality, which a narrowing at an iteration j >= 1 that drops columns can destroy for later iterations. The guard (!narrowed0) excludes only narrowing at iteration 0. PROTOCOL.md H_blr ('as V_{i-1}^T M V_0 = 0') and README ('direct grams otherwise') state it as exact.",
     "evidence": f"stress harness, 20,000 block_lanczos calls: {st['by_class'].get('narrowed_mid', {}).get('trials')} calls narrowed before their final iteration (all at ell <= 101; none at ell = 65537 or 4.5e15); in {st['summary']['calls_with_id2_bad']} of them identity 2 was inexact at one or more iterations ({st['summary']['id2_bad']} inexact uses), and in {st['by_class'].get('narrowed_mid', {}).get('trials', 0) - st['by_class'].get('narrowed_mid', {}).get('ref_blr_same', 0)} the kernel vector BLR returned differs from REF's (examples in the replay certificate; all 24 in replay/.../stress/divergent-trials.jsonl). Zero inexact uses in the calls without narrowing, with narrowing at iteration 0 (fallback), or with narrowing only in the final iteration or the terminating check.",
     "scope_effect": "None on this study: in all 30 producer cells and all replayed configurations the oracle found 0 inexact uses of either identity; narrowing occurs only in the final iteration, so no reuse follows a narrowing (id2_reuse_after_a_narrowing = 0). A guard that disables identity 2 in every iteration after any narrowing would give the same counts on every measured cell.",
     "required_correction": "Before BLR is cited as exact or reused outside k = 18..22 of this family (other ell, block widths, matrices that narrow mid-run): disable the V_i^T V_0 recurrence after any narrowing (not only at iteration 0), and state identity 2 as conditional in README/PROTOCOL successors. Identity 1 needs no change."},
    {"id": "D2", "severity": "minor", "what": "The identity-1 fallback condition vpr.k == k_full is always true (V_full has k_{i-1} columns by construction), so gram(mvpr, mv) is dead code; 'when the dimensions match' in PROTOCOL.md is vacuous. Identity 1 is exact in every path (proof in the replay certificate; 142,892 stress uses and every real-run use, 0 mismatches)."},
    {"id": "D3", "severity": "minor (disclosed by the producer)", "what": "The prediction window (0.80-0.84 / 0.87-0.90) was written after a pilot on seed 1 at k = 18, 20, 22, so it is not independent of 3 of the 30 cells; the linear-algebra lmul prediction (0.75-0.80) missed (0.729-0.738), as README reports."},
]
report = {
    "schema": "validator-report/v1", "study": STUDY, "status": "pass", "independent": True, "independence_note": indep,
    "validated_snapshot": snapshot,
    "replay_certificate": {"path": "replay-certificate.json", "sha256": cert_sha},
    "raw_data_path_base": f"paths in raw_data[].path are relative to {STUDY}/validation/ (the directory of this report); repo_path is repository-relative",
    "raw_data": raw_data,
    "headline": H,
    "decisions": {"SUPPORTED_recomputed": rc["decision"]["SUPPORTED"], "SUPPORTED_claimed": cl["supported"],
                  "holds_all_w_h": rc["decision"]["holds_all_wh"], "D_DL1_exactly_1": rc["decision"]["D_DL1_exactly_1"],
                  "ref_reproduces_frozen_band_1e9": rc["ref_reproduces_band_1e9"],
                  "gates": {g2: rc["gates"][g2]["pass"] for g2 in ("G1", "G2", "G3", "G4")},
                  "gate_counts": {"G1_fields": rc["gates"]["G1"]["fields"], "G1_records": rc["gates"]["G1"]["records"], "G4_files": rc["gates"]["G4"]["files"], "G4_records": rc["gates"]["G4"]["records"]},
                  "analysis_json_cells_equal": f"{rc['vs_claimed']['ratio_cells_compared']} ratio cells + per-seed values, max relative difference {rc['vs_claimed']['ratio_max_rel_diff']}; LA ratios max relative difference {rc['vs_claimed']['la_max_rel_diff']}"},
    "checks": {
        "artifacts": "runs tarball sha256 e24e4050... = README; 123/123 inner SHA256SUMS OK; 60 units, 120 files + 3; binary.sha256 inside equals the recorded REF/BLR hashes; fasthash tarball d9d301a4... with 0 inner failures",
        "build": "REF and BLR rebuilt with rustc 1.97.0 to the recorded sha256 (bit-identical)",
        "freeze": snapshot["freeze_integrity"],
        "timing_of_freeze": "protocol commit 2026-11-15T04:32:34Z precedes run.log start 04:32:47Z",
        "gates_recomputed": "G1 (30 pairs, 90 records, 8,580 counted fields, 0 mismatches, LA differs in all 30), G2, G3 (all ok, k_found == k, rc 0, one attempt each), G4 (30 fasthash COMBO files, 90 records, 0 mismatches)",
        "replay": f"{len(main_runs)} runs, {cmpj['fields_compared_non_negative_roles']} counted-field comparisons, 0 mismatches; seeds 9, 10 outside 1-6; q = 2097091 and 4194271; block widths 1, 2, 3, 8",
        "identities": "proved in the replay certificate (identity_proofs); oracle: both exact at every iteration of every real run; identity 2 inexact after mid-run narrowing (D1)",
        "counting": identities["counting"],
        "real_work_removed_not_relabelled": "the reuse path performs no gram (copies only, or a k x b x 2k recurrence); diagnostic DL linear_algebra.cpu BLR/REF from the committed records is below 1 in 30/30 cells (0.71-0.96; L0, concurrent, not a claim)",
    },
    "discrepancies": disc,
    "null_object_control": "The claim is a deterministic counted-work ratio, not a statistical signal. Nulls of the same shape: (a) REF replay = committed REF = fasthash COMBO bit for bit (A/A on counted fields, ratio exactly 1); (b) the untouched D and DL1 arms give BLR/REF exactly 1.000 at every cell and w; (c) negative controls: MUT_C2 and MUT_PM, each on 2 configurations, are flagged by the same comparator (LA fails, 6 DL attempts), so G1-style equality has power against an inexact reuse. Mechanism check in place of a decay parameter: the saving should shrink as the block width falls (the removed products are n k^2 and n k b per iteration vs the sparse matvec) and does: DL ratio at w_add 0 = 0.909, 0.861, 0.833, 0.815 at b = 1, 2, 3, 4 (q = 1048507) and 0.774 at b = 8 (q = 524287); LA lmul ratio 0.770, 0.745, 0.737, 0.732, 0.728 toward the dense-only limit 5/7.",
    "validation_ladder_checks": {
        "step1_assumptions_isolated": "the claim rests on M-orthogonality (identity 1: distance 1 and 2; identity 2: to V_0 at every distance). Measured separately per iteration by the oracle build on all 30 cells, and stressed at small ell where narrowing is frequent; failure mode hunted: an inexact reuse that leaves counts equal or changes the solution. Found for identity 2 after mid-run narrowing only (D1).",
        "step2_measured_ratio_on_full_pipeline": "the ratio is measured on the full pipeline against REF at every size claimed (k = 18..22, 30 cells), re-measured on new seeds (9, 10) and other block widths; predicted negative cases (D, DL1 = 1) checked. No extrapolation beyond k = 22 is claimed.",
        "step3_cheats": "not applicable (no real-scale partial run)",
        "step4_reproducibility_pointer": "both binaries rebuilt from the committed sources to the recorded hashes and replayed; scripts and raw replay data committed under validation/",
    },
    "heuristic_validation_checks": "not applicable: no heuristic distribution is validated (deterministic counted operations).",
    "cost_model_checks": {
        "unit": "cold F_q-multiplication equivalents (M-eq): mul + r_inv inv + r_lmul lmul + w_add (add + ladd) + w_h gae (hash + cmp), frozen r_inv, r_lmul, gae per size from research/quartic_n4v2_20261020/results/summary.json (DL, kind ic_size)",
        "optimistic_assumptions": "r_lmul (2.01-2.18) and r_inv are time ratios frozen from an earlier grid, not re-measured; additions free in the primary cell and in the rho band (the band is at w_add 0); hash/cmp priced at 0, 0.1, 1 GAE (section 5a)",
        "memory": "not reported by the study; BLR keeps W_full (k^2), two V^T V0 blocks (2 k b) and c1, c2 (2 k^2) extra words per call, negligible next to the n x k blocks; no time-memory trade-off",
        "success_probability": "every ic record ok at attempt 1 in all 60 producer units and all replays; the M-eq sums over all attempt records, so a retry would be charged",
    },
    "limitations": [
        "L0: session container (4 vCPU), units up to 3 concurrent; only counted fields are claimed and compared. No EKS route, no native L2/replay/A/A admission.",
        "Scope: GHS-weak family E0/F_{q^3}, driver ic <q> <seed> D 3.0 1.7, k = 18..22, seeds 1-6 (producer) plus seeds 9 and 10 and block widths 1, 2, 3, 8 (replay). Identity 2 is exact on these instances (oracle) but not in general (D1).",
        "The rho side is the frozen negation-map band of research/quartic_dl1_cd_sqrt_20261113/combo/validation/VALIDATION.json; this validation re-measured no rho. The band prices additions at 0; at w_add = 1 the IC side is larger (ratios 0.88-0.89).",
        "Validated snapshot c5bf40cee6. Remote main was at 34a4f6f290 at nonce time and was not fetched.",
        "Independence: subagent of the producer's session on the same container; own scripts and builds; analyze.py and plot.py not read.",
        "A passed report means the receipt is admissible evidence for the counted-cost numbers at k = 18..22 in this family. It does not support an ECDLP claim, demonstrate an asymptotic speedup, or authorize promotion; the leaderboard row additionally needs VALIDATION.json with the matched rho reference.",
    ],
    "method": "scripts/: select.py (nonce sample), make_plan.py, replay.sh, make_audit_la2.py (oracle and mutant builds), compare.py (replay comparison and reconciliation), bltest.rs.in + stress_compare.py (block_lanczos stress), recompute.py (headline from the raw tarball), make_reports.py (this file and the certificate).",
    "validation_report": {
        "id": "VAL-20261115-b1e7a3", "task_id": "none named by the task card (main-session validation-gate request)",
        "run_ids": ["research/ic_gw_bl_reuse_20261115 runs-blreuse-20261115 (60 units, c5bf40cee6)"],
        "artifact_checks": ["tarball sha256 + 123/123 inner checksums", "60/60 units rc 0", "binaries rebuilt bit-identically", "freeze unchanged since 5c9dc91c48"],
        "metric_recomputations": ["405 ratio cells and per-seed values equal analysis.json (max rel diff 0)", "decision SUPPORTED", "k = 22 band numbers equal"],
        "control_checks": ["REF = committed REF = fasthash COMBO (A/A, counted)", "D, DL1 exactly 1", "2 mutants x 2 configurations detected", "oracle 0 inexact uses in real runs", "block-width dose-response as the mechanism predicts"],
        "heuristic_validation_checks": [], "cost_model_checks": ["unit declared", "optimistic constants flagged", "memory not reported (negligible change)", "success 1 at attempt 1"],
        "proof_architecture_checks": ["identity 1 proved exact in all paths", "identity 2 exact only absent mid-run narrowing (D1)"],
        "verdict": "passed", "limitations": ["see limitations and discrepancies D1-D3"],
        "artifact_paths": [f"{STUDY}/validation/replay-certificate.json", f"{STUDY}/validation/validator-report.json", f"{STUDY}/validation/{TAR}"],
    },
    "review_attestation": {
        "review_plan": "none attached to the task card; no joints assigned, so this report verdicts receipt admissibility",
        "joints_owned": [],
        "sources_read": ["crypto-autoresearcher/docs/agent-runtime-core.md", "crypto-autoresearcher/agents/validator.md",
                         "crypto-autoresearcher/AGENTS.md (Review architecture; Curve identity and measured bounds)",
                         "crypto-autoresearcher/docs/inventor-protocol.md sections 3 and 6", "crypto/docs/VALIDATION_GATE.md",
                         f"{STUDY}/PROTOCOL.md", f"{STUDY}/README.md", f"{STUDY}/tables.md", f"{STUDY}/analysis.json (by key)",
                         f"{STUDY}/make-units.sh", f"{STUDY}/run-local.sh", f"{STUDY}/src/la2.rs (diff and block Lanczos section)",
                         f"{STUDY}/src/ic2.rs (la2/ls2/lm2/linv)", f"{STUDY}/src/solver2.rs (linear-algebra dispatch)", f"{STUDY}/src/v3.rs (ic_cmd, argument parsing)",
                         "research/quartic_n4_20261019/common.rs (counters)", "research/ic_gw_fasthash_20261114/validation/ (format only)",
                         "research/quartic_dl1_cd_sqrt_20261113/combo/analyze.py (model)", "research/quartic_dl1_cd_sqrt_20261113/combo/validation/VALIDATION.json",
                         "research/quartic_n4v2_20261020/results/summary.json (DL ic_size rows)", "scripts/validation_gate.py (check functions)"],
        "not_read": [f"{STUDY}/analyze.py", f"{STUDY}/plot.py"],
        "read_sibling_reports": False, "verdict": "passed",
    },
}
rep_p = os.path.join(V, "validator-report.json")
json.dump(report, open(rep_p, "w"), indent=1); open(rep_p, "a").write("\n")

# benchmarks copies
B = os.path.join(REPO, "benchmarks/20261115-ic-gw-bl-reuse")
open(os.path.join(B, "replay-certificate.json"), "wb").write(open(cert_p, "rb").read())
rb = json.loads(json.dumps(report))
rb["copy_of"] = {"path": f"{STUDY}/validation/validator-report.json", "sha256": sha(rep_p),
                 "note": "same content; raw_data[].path rebased to benchmarks/20261115-ic-gw-bl-reuse/, replay_certificate points to the byte-identical copy in this directory"}
for it in rb["raw_data"]:
    it["path"] = os.path.relpath(os.path.join(REPO, it["repo_path"]), B)
rb["raw_data_path_base"] = "paths in raw_data[].path are relative to benchmarks/20261115-ic-gw-bl-reuse/ (the directory of this copy); repo_path is repository-relative"
json.dump(rb, open(os.path.join(B, "validator-report.json"), "w"), indent=1); open(os.path.join(B, "validator-report.json"), "a").write("\n")
print("ok", len(H), "headline items")
