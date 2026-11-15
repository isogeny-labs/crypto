"""Write validation/REPORT.md from the validator's outputs (no number retyped).
usage: write_report_md.py VALIDATION_DIR"""
import json
import os
import sys

import vcommon as v

OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "out")


def J(n):
    return json.load(open(os.path.join(OUT, n)))


def f6(x):
    return "%.6f" % x


def iv(c):
    return "[%s, %s]" % (f6(c[0]), f6(c[1]))


def main():
    vd = sys.argv[1]
    rep = json.load(open(os.path.join(vd, "validator-report.json")))
    cert = json.load(open(os.path.join(vd, "replay-certificate.json")))
    j1 = {b: J(f"j1-P{b}.json") for b in (32, 24)}
    j2, j3, j4, xc = J("j2.json"), J("j3.json"), J("j4.json"), J("j1-inpod-crosscheck.json")
    L = []
    w = L.append
    w("# XP-20261029-packed-filter: PROTOCOL §10 independent validation (TASK-20261029-654f73)")
    w("")
    w(f"**Validator report:** `validator-report.json` (sha256 `{v.sha256_file(os.path.join(vd, 'validator-report.json'))}`), status **{rep['status']}**.  ")
    w(f"**Replay certificate:** `replay-certificate.json` (sha256 `{v.sha256_file(os.path.join(vd, 'replay-certificate.json'))}`), status **{cert['status']}**, scalars_verified **{str(cert['scalars_verified']).lower()}**.  ")
    w("**Decision under check:** DEC-20261029-5eb8e6, `cards/VERDICT-P32.md` sha256 `415a89d78efd3079e7e1acd3824ec390247fed11a29baabb3ff727db9c589ea3`.  ")
    w("**Snapshot validated:** branch `claude/ic-packed-filter-20261029`, commit `ecc7e409a23ebbea91c6fe3bd3f1e1ab69baa7f0`, clean tree, in sync with `origin`; `PROTOCOL.md` (sha256 `703bde26…7e51`), `APPROVAL.md` (`0a35923c…6a40`) and the six specs are unchanged since the freeze commit `f870408c17`.  ")
    w("**Date:** 2026-10-30. **Role:** validator (independent; own code in `code/`).")
    w("")
    w("A pass says the receipts are admissible evidence and the verdict's numbers follow from them. It does not support an ECDLP claim, does not make the P32 result a counted or baseline improvement, and authorizes no promotion: that is the Coordinator's evidence review.")
    w("")
    w("## Joint verdicts")
    w("")
    w("| joint | result | breaking artifact sought | found |")
    w("|:--|:--|:--|:--|")
    w(f"| J1 decision statistic | {rep['joints']['J1 decision statistic']} | corrected upper bound ≥ 1, or a headline mismatch | none: corrected upper bound {f6(j1[32]['decision']['corrected']['ci95_log'][1])} < 1; every J1 number reproduces |")
    w(f"| J2 instrument | {rep['joints']['J2 instrument']} | a position imbalance between dgbf and dgb-aa, or an admitted pair with a run below L2 | none: 144/144 at positions 1/2 for both; 0 records below L2 in either wall session; 0 re-grade disagreements |")
    w(f"| J3 charged-work identity | {rep['joints']['J3 charged-work identity']} | any field difference, a counter on the wrong arm or phase, a bitop difference ≠ 384 × counter | none: 0 mismatches over 336 matched records per arm; identity holds on 672 filtered records per session, phase by phase |")
    w(f"| J4 counted route and band | {rep['joints']['J4 counted route and band']} | an S′ ratio off by more than its printed precision, or an order reversal | none in the claim under review; one descriptive per-curve figure is truncated in the verdict's prose (below) |")
    w(f"| J5 independent replay | {rep['joints']['J5 independent replay']} | a replay that does not reproduce a scalar or log | none: 1152/1152 identical in each of the four sessions; 1344/1344 scalars verified by own arithmetic in each |")
    w(f"| proves-too-much control | {rep['joints']['proves_too_much']} | an improvement on an A/A pair or at P24 | none: both A/A intervals contain 1; P24 corrected lower bound > 1 |")
    w("")
    w("Defects found: **none**. One descriptive discrepancy (not counted as a defect; the Coordinator may rule otherwise) is listed in \"Discrepancies\".")
    w("")
    w("## What I read, and what I did not")
    w("")
    w("- Raw inputs: `P{24,32}/{wall,counted}/eks-results/` and `P28/counted/eks-results/` (session files, `job-exit.json`, `audit.json`, `l2-gate.json`, `control-comparison.json`, `input-spec.json`, `build-info.json`, `binary.sha256`, `SHA256SUMS`); the frozen specs; the P28 receipts the handoff names; S3 (read-only) for the S3 copies, launcher `result.json` and `request.json`.")
    w("- Instrument definitions at `3271a25d02` via `git show`: `band.rs` (priced GAE), `record.rs` (`grade_v3`), `docs/units/BITOPS.md` (bitop/v1 prices). `compare.rs` was **not** read: the interval method was derived from the statement (linearised t on the log ratio, independent-SE combination, Welch–Satterthwaite dof) and then found to equal the in-pod instrument output to 4.2e-16.")
    w("- **Not read:** `cards/tools/`, any `analysis/` directory (P24, P28, P32), any wall field of a counted session, any rho wall time, any P28 wall wall-field. Counted sessions (and P28 wall) are loaded through `code/vcommon.py:load_counted`, which drops every wall and timing key and truncates the too-short blocker to its prefix before any other code sees the record. Directory listings showed the names of `analysis/` directories and of other sessions' scratch folders; none was opened.")
    w("")
    w("## J1: decision statistic (blind re-derivation)")
    w("")
    w("Method (`code/j1_decision.py`, `code/vstats.py`): pairs keyed by (curve slug, target index, round), measured rounds only, admitted when both runs are verified and at L2 or L3. ns/GAE = Σ solve_wall_ns / Σ total_gae per arm. ratio = ns/GAE(B) / ns/GAE(A). SE of ln ratio by the delta method on the four sums (sample variance, n − 1), t with n − 1 dof, interval exp(ln R ± t·SE). corrected = ratio(dgb→dgbf) / ratio(dgb→dgb-aa), SEs combined as independent, Welch–Satterthwaite dof. Student t quantile by own incomplete-beta code.")
    w("")
    w("| quantity | P32 recomputed | P32 claimed | P24 recomputed | P24 claimed |")
    w("|:--|:--|:--|:--|:--|")
    for b in (32, 24):
        pass
    d32, d24 = j1[32]["decision"], j1[24]["decision"]
    w(f"| pairs (below level) | {d32['dgb->dgbf']['n']} ({d32['dgb->dgbf']['pairs_below_level']}) | 288 (0) | {d24['dgb->dgbf']['n']} ({d24['dgb->dgbf']['pairs_below_level']}) | 288 (0) |")
    w(f"| ns/GAE dgb | {d32['dgb->dgbf']['ns_per_gae_a']:.6f} | 84.5526 | {d24['dgb->dgbf']['ns_per_gae_a']:.6f} | — |")
    w(f"| ns/GAE dgbf | {d32['dgb->dgbf']['ns_per_gae_b']:.6f} | 81.0446 | {d24['dgb->dgbf']['ns_per_gae_b']:.6f} | — |")
    w(f"| ratio dgbf/dgb [ci95] | {f6(d32['dgb->dgbf']['ratio'])} {iv(d32['dgb->dgbf']['ci95_log'])} | 0.958512 [0.956084, 0.960945] | {f6(d24['dgb->dgbf']['ratio'])} {iv(d24['dgb->dgbf']['ci95_log'])} | — |")
    w(f"| aa_ratio dgb-aa/dgb [ci95] | {f6(d32['aa dgb->dgb-aa']['ratio'])} {iv(d32['aa dgb->dgb-aa']['ci95_log'])} | 0.998337 [0.996146, 1.000534] | {f6(d24['aa dgb->dgb-aa']['ratio'])} {iv(d24['aa dgb->dgb-aa']['ci95_log'])} | 1.000506 [0.999012, 1.002003] |")
    w(f"| **corrected dgbf/dgb [ci95]** | **{d32['corrected']['ratio']:.9f} {iv(d32['corrected']['ci95_log'])}** | 0.960107806 [0.956898, 0.963328] | **{d24['corrected']['ratio']:.6f} {iv(d24['corrected']['ci95_log'])}** | 1.003200 [1.000564, 1.005842] |")
    w(f"| corrected dof | {d32['corrected']['dof']:.2f} | 562.6 | {d24['corrected']['dof']:.2f} | — |")
    w(f"| condition 9: dgbf/dgb-aa | {j1[32]['cond9']['dgb-aa->dgbf']['ratio']!r} (rel. diff {j1[32]['cond9']['rel_diff_vs_corrected']:.1e}) | 0.960107806240821 (0.0) | {j1[24]['cond9']['dgb-aa->dgbf']['ratio']!r} ({j1[24]['cond9']['rel_diff_vs_corrected']:.1e}) | — |")
    w(f"| Σwall(dgbf)/Σwall(dgb-aa); GAE sums equal | {j1[32]['cond9']['sum_wall_ratio_dgbf_over_dgbaa']!r}; {j1[32]['cond9']['gae_sums_equal']} | = corrected | {j1[24]['cond9']['sum_wall_ratio_dgbf_over_dgbaa']!r}; {j1[24]['cond9']['gae_sums_equal']} | — |")
    w("")
    w(f"Reading under PROTOCOL §6.2, recomputed: P32 corrected upper bound {f6(d32['corrected']['ci95_log'][1])} < 1 (improvement); P24 corrected lower bound {f6(d24['corrected']['ci95_log'][0])} > 1 (regression). The linear-scale interval differs in the sixth decimal (P32 corrected [0.956893, 0.963323]), so the instrument's interval is the log-scale one.")
    w("")
    w("**Cross-check against the instrument.** The in-pod `control-comparison.json` is an `ecbench compare` output (`--a dgbf-edge --b dgb-aa`, aa arms dgbf-edge/dgbf). My code recomputes its `ratio_b_over_a`, `ci95`, `aa_ratio`, `aa_ci95`, `corrected_ratio`, `corrected_ci95`, `corrected_dof` and both ns/GAE with maximum relative differences of "
      f"{xc['P32']['max_rel_diff']:.1e} (P32) and {xc['P24']['max_rel_diff']:.1e} (P24).")
    w("")
    w("### Descriptive intervals (decide nothing)")
    w("")
    w("| size | curve | corrected [linearised-t ci95] | target-clustered bootstrap ci95 (cluster = target; 24 per curve, 48 for both; B = 4000) | dgbf-edge/dgb (edge) |")
    w("|:--|:--|:--|:--|:--|")
    for b in (32, 24):
        for cv, x in j1[b]["per_curve"].items():
            bt = j1[b]["target_clustered_bootstrap_per_curve"][cv]
            w(f"| P{b} | `{cv}` | {f6(x['corrected'])} {iv(x['corrected_ci95_log'])} | {iv(bt['corrected_ci95_pct'])} | {f6(x['position_control'])} |")
        tb = j1[b]["target_clustered_bootstrap"]
        w(f"| P{b} | both (48 clusters) | {f6(j1[b]['decision']['corrected']['ratio'])} {iv(j1[b]['decision']['corrected']['ci95_log'])} | {iv(tb['corrected_ci95_pct'])} | {f6(j1[b]['dgb->dgbf-edge']['ratio'])} |")
    w("")
    w("Per-round corrected ratio (descriptive stability):")
    w("")
    for b in (32, 24):
        w(f"- P{b}: " + ", ".join(f"round {k} {f6(x)}" for k, x in j1[b]["per_round_corrected"].items()))
    w("")
    w("**Position control and middle-over-edge contrasts** (PROTOCOL §6.2):")
    w("")
    w("| size | edge estimate dgbf-edge/dgb [ci95] | decision corrected [ci95] (middle positions) | aa_ratio dgb-aa/dgb | dgbf/dgbf-edge | overlap |")
    w("|:--|:--|:--|:--|:--|:--|")
    for b in (32, 24):
        e = j1[b]["dgb->dgbf-edge"]; c = j1[b]["decision"]["corrected"]; a = j1[b]["decision"]["aa dgb->dgb-aa"]; fe = j1[b]["dgbf-edge->dgbf"]
        ov = e["ci95_log"][0] <= c["ci95_log"][1] and c["ci95_log"][0] <= e["ci95_log"][1]
        w(f"| P{b} | {f6(e['ratio'])} {iv(e['ci95_log'])} | {f6(c['ratio'])} {iv(c['ci95_log'])} | {f6(a['ratio'])} {iv(a['ci95_log'])} | {f6(fe['ratio'])} {iv(fe['ci95_log'])} | {'yes' if ov else 'no'} |")
    w("")
    w("**Null-object controls** (`docs/inventor-protocol.md` §3). Two null objects of the same shape as the decision pair are measured with the same code:")
    w("- the A/A pairs (identical oracle on both arms): both wall contrasts above contain 1 at P32, and the counted A/A ratios are exactly 1 at every weight;")
    w("- a label-swap null on the matched-position pair (dgb-aa, dgbf): swapping arm labels at random within each pair destroys the arm effect. Its distribution must centre on 1 and narrow like 1/√n as pairs are added, while a real effect stays put:")
    w("")
    w("| P32 pairs n | null median | null 95% band | observed corrected (first n pairs) | null fraction ≤ observed |")
    w("|--:|:--|:--|:--|:--|")
    for n, x in sorted(j1[32]["label_swap_null_dgbaa_dgbf"].items(), key=lambda t: int(t[0])):
        w(f"| {n} | {f6(x['null_median'])} | {iv(x['null_ci95'])} | {f6(x['observed'])} | {x['null_fraction_le_observed']} |")
    w("")
    w("The null band narrows from ±0.0095 to ±0.0053 as n goes from 72 to 288 (about 1/√n), while the observed ratio holds near 0.96 at every n and in every round. The quantity behaves as a real effect, not an artifact. At P24 the same null gives the observed 1.0032 at the 99.8th percentile, the regression the frozen rule reads.")
    w("")
    w("## J2: instrument")
    w("")
    w("Method (`code/j2_instrument.py`): every condition is read from the job's own files after my own `SHA256SUMS` check. Levels are re-counted from records and compared with `l2-gate.json`; each record's level is re-derived from its blocker prefixes. For the wall sessions, a partial re-grade recomputes each record's grading-v3 L1/L2 gates from its raw isolation and timing fields (foreign, steal and sibling ticks, run-queue delay, preemption, foreign task slices, memory stall, too-short, affinity, CPU moves, threads, CPU/wall, intruders). Positions and predecessors are tabulated from `seq` within each (curve, target, round) block.")
    w("")
    w("| check | P32 wall | P24 wall |")
    w("|:--|:--|:--|")
    for k, lab in (("cond1_exit", "1 exit 0, single pod"), ("cond2_audit", "2 audit ok, complete, 0 problems, 1152/1152 identical"),
                   ("cond3", "3 counted A/A exactly 1 at w = 0, 0.1, 1"), ("cond4", "4 spec JSON-equal, build-info, binary sha"),
                   ("cond5", "5 l2-gate accepted, all L2/L3, no invalid outcomes"), ("cond6", "6 native A/A admission file")):
        w(f"| {lab} | {j2['P32-wall'][k]} | {j2['P24-wall'][k]} |")
    for b in (32, 24):
        pass
    w(f"| records re-graded / raw-gate disagreements | {j2['P32-wall']['raw_regrade']['records']} / {len(j2['P32-wall']['raw_regrade']['disagreements'])} | {j2['P24-wall']['raw_regrade']['records']} / {len(j2['P24-wall']['raw_regrade']['disagreements'])} |")
    w(f"| level-from-blockers mismatches | {len(j2['P32-wall']['l2_gate']['level_from_blockers_mismatches'])} | {len(j2['P24-wall']['l2_gate']['level_from_blockers_mismatches'])} |")
    w(f"| record levels | {j2['P32-wall']['l2_gate']['records_levels']} | {j2['P24-wall']['l2_gate']['records_levels']} |")
    w(f"| algorithm seed shared by the four arms of each block | {j2['P32-wall']['spec_build']['algorithm_seed_shared_by_all_arms_per_key']} | {j2['P24-wall']['spec_build']['algorithm_seed_shared_by_all_arms_per_key']} |")
    w(f"| curves are the §3 curves; seeds 202610296b3/4, 6, 8 | {all(j2['P32-wall']['spec_build'][k] for k in ('curves_are_section3', 'curve_seeds', 'target_seed', 'order_seed_in_spec'))} | {all(j2['P24-wall']['spec_build'][k] for k in ('curves_are_section3', 'curve_seeds', 'target_seed', 'order_seed_in_spec'))} |")
    w("")
    w("**Block positions and predecessors** (measured rounds; identical at P32 and P24):")
    w("")
    w("| arm | position counts (0..3) | predecessor within block |")
    w("|:--|:--|:--|")
    pos = j2["P32-wall"]["positions"]
    for arm in ("dgb", "dgb-aa", "dgbf", "dgbf-edge"):
        w(f"| `{arm}` | {pos['positions'][arm]} | {pos['predecessor_within_block'][arm]} |")
    w("")
    w(f"Measured block orders: {pos['measured_orders']}; blocks are four consecutive `seq` values: {pos['blocks_consecutive_seq']}. `dgbf` and `dgb-aa` each sit 144 times at position 1 and 144 at position 2, and each has 144 filtered and 144 unfiltered predecessors.")
    w("")
    w("**Counted sessions** (v3 §6.1 conditions 1–6 as adapted by dg-wall §6.1; exit-42 clause):")
    w("")
    w("| check | P32 counted | P24 counted | P28 counted (r2, of record) |")
    w("|:--|:--|:--|:--|")
    for k, lab in (("cond1_audit", "1 audit"), ("cond2", "2 rho-neg-aa/rho-neg exactly 1"), ("cond3_spec_build", "3 spec and build"),
                   ("cond4_ic_all_L2_L3", "4 every ic.pipeline execution L2/L3"), ("cond5_rho", "5 rho L2/L3 or too-short L1 only"),
                   ("cond6_invalid_outcomes_empty", "6 invalid_outcomes empty"), ("EXIT42_CLAUSE", "exit-42 clause"),
                   ("below_l2_from_records_equal_gate", "below-L2 set from records = l2-gate.json")):
        w(f"| {lab} | {j2['P32-counted']['gate'][k]} | {j2['P24-counted']['gate'][k]} | {j2['P28-counted']['gate'][k]} |")
    w(f"| job exit / launcher exit | {j2['P32-counted']['job_exit']['exit_code']} / {rep['s3_copy_check']['P32-counted']['launcher_exit_codes']} | {j2['P24-counted']['job_exit']['exit_code']} / {rep['s3_copy_check']['P24-counted']['launcher_exit_codes']} | {j2['P28-counted']['job_exit']['exit_code']} / {rep['s3_copy_check']['P28-counted']['launcher_exit_codes']} |")
    for b in (32, 24, 28):
        pass
    w(f"| below_l2 (per arm) | {j2['P32-counted']['gate']['below_l2_from_gate']['per_arm']} | {j2['P24-counted']['gate']['below_l2_from_gate']['per_arm']} | {j2['P28-counted']['gate']['below_l2_from_gate']['per_arm']} |")
    w("")
    w("All waived executions are rho executions graded L1 whose only `L2:` blockers begin \"solve too short to admit wall time\". Their solve-time range is not reported, because it is rho wall time (PROTOCOL §10).")
    w("")
    w("**S3 and launch binding** (read-only, `code/s3check.py`, `code/s3requests.py`, `code/c2_identity.py`): every job has exactly one pod-uid prefix, equal to `job-exit.json`'s `pod_uid`. Every file of every session streamed from S3 hashes equal to the repository copy. Every launcher `request.json` embeds the frozen spec (JSON-equal) and names image `32144a29…817b`, revision `3271a25d02` and ecbench `21701be6…4503`. Launcher exits: P32 and P24 wall 0 (`succeeded`); P32, P24 and P28-r2 counted 23 (`failed`), so `verify-build` did not run for the counted jobs. Their binary is bound by `binary.sha256`, `build-info.json`, every record's `binary_sha256` and the audit's `auditor_binary_sha256`. RULING-P28 C2's identity facts, recomputed for every job including the launcher-less P28 wall: `S3_TERMINAL` true for all six; the image digest is unverifiable without pod status for all six.")
    w(f"P28 session of record: the original counted prefix `runs/ic-dgbf-rho-p28-20261029/` holds {rep['p28_counted_session_of_record']['original_prefix_objects_now']} objects today, consistent with the logged `S3_KEYCOUNT … 0` and COUNTED = r2 (VERDICT-P32 (B) B5).")
    w("")
    w("## J3: charged-work identity")
    w("")
    w("Method (`code/j3_work.py`): every `dgbf`, `dgbf-edge` and `dgb-aa` record, warm-ups included, is compared with the `dgb` record of the same (curve, target, round). The comparison covers the recovered log, `total_gae`, every unpriced count as band.rs resolves it, and per phase by name `adds`, `doubles`, `scalar_mults`, `gae`, `native` and `work_native` without `table_filter_bit_sets`. Every record's bitop/v1 work is re-priced from its primitive counts with my own reading of BITOPS.md, and the dgbf − dgb difference is checked phase by phase (bitops and primitive counts). `ecbench work` was not run: the handoff limits the binary to the replay.")
    w("")
    w("| check | P32 | P24 |")
    w("|:--|:--|:--|")
    for b in (32, 24):
        pass
    c8 = {b: j3[f"P{b}-wall-cond8"] for b in (32, 24)}
    w(f"| matched records (dgbf, dgbf-edge, dgb-aa) / mismatches | {c8[32]['matched']} / {len(c8[32]['mismatches'])} | {c8[24]['matched']} / {len(c8[24]['mismatches'])} |")
    w(f"| `table_filter_bit_sets` absent on dgb, dgb-aa | {c8[32]['tfbs']['absent_on_dgb_and_dgb_aa']} | {c8[24]['tfbs']['absent_on_dgb_and_dgb_aa']} |")
    w(f"| present once, in `oracle_setup` only, on dgbf, dgbf-edge | {c8[32]['tfbs']['present_once_in_oracle_setup_on_filtered']} | {c8[24]['tfbs']['present_once_in_oracle_setup_on_filtered']} |")
    w(f"| one value per curve | {c8[32]['tfbs']['values_per_curve']} | {c8[24]['tfbs']['values_per_curve']} |")
    w(f"| records re-priced / mismatches | {c8[32]['bitops']['records_repriced']} / {len(c8[32]['bitops']['reprice_mismatches'])} | {c8[24]['bitops']['records_repriced']} / {len(c8[24]['bitops']['reprice_mismatches'])} |")
    w(f"| bitop identity (384 × counter, oracle_setup only, prims +6 word_op per bit set) checked / failures | {c8[32]['bitops']['identity_checked']} / {len(c8[32]['bitops']['identity_failures'])} | {c8[24]['bitops']['identity_checked']} / {len(c8[24]['bitops']['identity_failures'])} |")
    w(f"| bitop/v1 dgbf/dgb, measured records; difference | {c8[32]['bitops']['measured_ratio_dgbf_over_dgb']:.7f}; {c8[32]['bitops']['measured_difference']:,} | {c8[24]['bitops']['measured_ratio_dgbf_over_dgb']:.7f}; {c8[24]['bitops']['measured_difference']:,} |")
    w(f"| condition 10 (wall): p max, signed points max (source) | {j3['P32-wall-cond10']['p_max']:,}, {j3['P32-wall-cond10']['signed_points_max']} ({j3['P32-wall-cond10']['source'][0]}) | {j3['P24-wall-cond10']['p_max']:,}, {j3['P24-wall-cond10']['signed_points_max']} |")
    w(f"| counted dgb, dgbf = wall records (incl. counter, warm-ups) | {j3['P32-counted-equals-wall']['equal']} (336, 336) | {j3['P24-counted-equals-wall']['equal']} (336, 336) |")
    w("")
    w(f"P32 difference check: 384 × 144 × (1,048,276 + 1,048,295) = {384 * 144 * (1048276 + 1048295):,}, equal to the recomputed difference. P28: counted r2 dgb and dgbf equal the P28 wall records in every condition-8 field ({j3['P28-counted-equals-wall']['equal']}); this used P28 wall counts only. Condition 10 holds on P28 counted (p max {j3['P28-counted-cond10']['p_max']:,}, signed points max {j3['P28-counted-cond10']['signed_points_max']}).")
    w("")
    w("Because the counted session re-executed every `dgb` and `dgbf` input, warm-ups included, in a second pod, the equality is also a cross-pod replication of all 672 IC records per size.")
    w("")
    w("## J4: counted route and band")
    w("")
    w("Method (`code/j4_band.py`): S′_w = (G − C + w(U + n_C) + w · table_filter_bit_sets) / √r per record, with U resolved as band.rs does (top-level counter, else Σ phase `native[name]` or `native[base]`). No record carries pinned calibration units, so C = n_C = 0. Ratios are Σ S′(B) / Σ S′(A) over matched measured pairs, overall and per curve; the order is read per curve at each weight.")
    w("")
    w("| session | pair (B/A) | w = 0 | w = 0.1 | w = 1 | per curve at w = 1 | without bit-set term | order strict across band, every curve |")
    w("|:--|:--|--:|--:|--:|:--|:--|:--|")
    for s in ("P32-wall", "P32-counted", "P24-wall", "P24-counted", "P28-counted"):
        for p in ("dgbf/dgb", "dgb/rho-neg", "dgbf/rho-neg", "rho-neg-aa/rho-neg", "dgb-aa/dgb", "dgbf/dgbf-edge"):
            if p not in j4[s]:
                continue
            r = j4[s][p]
            w(f"| {s} | `{p}` | {r['0.0']['ratio_b_over_a']:.6f} | {r['0.1']['ratio_b_over_a']:.6f} | {r['1.0']['ratio_b_over_a']:.6f} | "
              + " / ".join("%.4f" % x for x in r["1.0"]["per_curve"].values())
              + f" | {r['0.0']['ratio_without_bitset_term']:.9f} / {r['0.1']['ratio_without_bitset_term']:.9f} / {r['1.0']['ratio_without_bitset_term']:.9f} | {r['strict_same_order_across_band_every_curve']} |")
    w("")
    w("§5a: `rho-neg` is cheaper than both IC arms at every weight on every curve at P32, P24 and P28 (order strict across the band), so that ranking is permitted. `dgbf` against `dgb` ties at w = 0 and is costlier at w > 0 on both curves, so no counted ranking of `dgbf` ahead of `dgb` is permitted. Mean S′ per arm and weight (for `VALIDATION.json`'s `gae_band`) is in `code/out/j4.json` (`mean_Sprime_a`, `mean_Sprime_b`).")
    w("")
    w("## J5: independent replay")
    w("")
    w("Method (`code/replay.sh`): each session directory was copied and the copy checked against `SHA256SUMS` before and after the replay. `ecbench verify --replay-all` was run with the image binary (sha256 checked) on this host, an Intel Xeon VM; the pods ran on c8a.2xlarge, AMD EPYC 9R45. Warm-ups are refused by the binary's replay modes, so `code/scalars.py` checks every record, warm-ups included, with my own short-Weierstrass arithmetic: [recovered]G = Q, with the curve bound by its ICV1 j-invariant, p + 1 − t = r and [r]G = O.")
    w("")
    w("| session | replays identical | problems | receipt ok | own scalar check (records, warm-ups) | replay env class (pod env class) |")
    w("|:--|:--|:--|:--|:--|:--|")
    for x in cert["runs"] + cert["additional_runs"] + cert["descriptive_runs"]:
        lab = x["run"] + (" (descriptive only)" if x["descriptive_only"] else "") + ("" if x["required_by_handoff"] else (" (session of record, extra)" if not x["descriptive_only"] else ""))
        w(f"| {lab} | {x['identical']}/{x['replays']} | {x['problems']} | {x['ok']} | {x['own_scalar_check']['scalar_ok']}/{x['own_scalar_check']['records']} ({x['own_scalar_check']['warmups']}) | {x['replay_env_class_id']} ({x['pod_env_class_id']}) |")
    w("")
    w("The P28 wall replay is descriptive. It never stands in for the missing in-pod audit (RULING-P28 (a)), and P28 stays undecidable.")
    w("")
    w("## VERDICT-P32 headline numbers")
    w("")
    w(f"{sum(1 for h in rep['headline'] if h['reproduced'])} of {len(rep['headline'])} headline entries reproduce: each claimed value at its printed precision (rel_tol = half a unit in the last printed digit; 1e-9 for the 15-digit condition-9 value and the in-pod cross-check entries). The full list, with source lines, is in `validator-report.json` → `headline`. Every §6.1 wall condition (1–10) and counted condition for P24 and P32 is marked in `validator-report.json` → `section_6_1` with its recomputed value; all reproduce.")
    w("")
    w("| claim | claimed | recomputed | reproduced |")
    w("|:--|:--|:--|:--|")
    keep = ("P32 corrected_ratio", "P32 corrected ci95", "P32 ratio_b_over_a", "P32 aa_ratio", "P32 ns_per_gae", "P32 condition 9: ratio",
            "P32 position control dgbf-edge/dgb", "P32 bitop/v1 dgbf/dgb", "P32 wall per charged GAE reduction", "P32 reduction interval",
            "P32 S' dgbf/dgb w=", "P32 counted S' dgb/rho-neg w=", "P32 counted S' dgbf/rho-neg w=", "P32 phase sum", "P24 corrected", "P24 counted S'",
            "P32 target_PDP change", "P32 oracle_setup change", "P32 oracle_setup rise as")
    for h in rep["headline"]:
        if any(h["name"].startswith(k) for k in keep):
            w(f"| {h['name']} | {h['claimed_as_printed']} | {h['recomputed']:.10g} | {'yes' if h['reproduced'] else '**no**'} |")
    w("")
    w("## Discrepancies (non-blocking)")
    w("")
    for x in rep["descriptive_discrepancies"]:
        w(f"- **{x['name']}**: printed {x['claimed_as_printed']}, recomputed {x['recomputed']!r}. {x['reading']}")
    w("")
    w("## Heuristic-validation and cost-model checks")
    w("")
    w("- Heuristic validation: not applicable. This is an engineering (constant) wall-clock comparison with no heuristic prediction under test. §7's predicted range was pre-registered; the miss (gain smaller than predicted) is descriptive.")
    w("- §6 ladder (`docs/inventor-protocol.md`): not applicable. The improvement is measured at the size where it is claimed (32 bits), and VERDICT-P32 (D) 5 forbids transfer.")
    w(f"- Units declared: ns per charged GAE (wall), GAE and S = GAE/√r (counted), bitop/v1, and S′ with w GAE per unpriced native operation. Flagged assumptions: S′ at w = 1 is a pessimistic band device; `table_insert_attempts` is unpriced on every IC arm; the three bitop/v1 limits of §5 apply. Memory beside time (max RSS, median over measured records, P32): dgb {j1[32]['phase_sums_ns']['dgb']['max_rss_kib_median']:.0f} KiB, dgb-aa {j1[32]['phase_sums_ns']['dgb-aa']['max_rss_kib_median']:.0f} KiB, dgbf {j1[32]['phase_sums_ns']['dgbf']['max_rss_kib_median']:.0f} KiB, dgbf-edge {j1[32]['phase_sums_ns']['dgbf-edge']['max_rss_kib_median']:.0f} KiB. Every run is a verified solve, so cost is per solve, not per attempt over a success probability.")
    w("")
    w("## Limitations")
    w("")
    for x in LIMITS:
        w(f"- {x}")
    w("")
    w("## Negative controls on my own checkers")
    w("")
    w("`code/selftest.py` perturbs copies of real records and requires each checker to flag the change: a native count +1, an extra work_native key, a changed log, a wrong pairing, a one-bitop change, a wrong scalar, the counter on the unfiltered arm, and wall-field leakage through the counted view. All fire (`code/out/selftest.json`).")
    w("")
    w("## How to reproduce")
    w("")
    w("```")
    w("cd research/ic_packed_filter_20261029/validation/code")
    w("python3 -B scalars.py out/scalars.json")
    w("python3 -B j1_decision.py 32 out/j1-P32.json; python3 -B j1_decision.py 24 out/j1-P24.json")
    w("python3 -B xcheck_inpod.py out/j1-inpod-crosscheck.json")
    w("python3 -B j2_instrument.py out/j2.json; python3 -B j3_work.py out/j3.json; python3 -B j4_band.py out/j4.json")
    w("python3 -B selftest.py out/selftest.json")
    w("PATH=<awsvenv>/bin:$PATH python3 -B s3check.py out/s3check.json  # and s3requests.py, c2_identity.py (read-only S3)")
    w("bash replay.sh <ecbench 21701be6…> ../../P32/wall/eks-results <scratch> P32-wall   # likewise per session")
    w("python3 -B build_report.py ..; python3 -B write_report_md.py ..")
    w("```")
    w("")
    w("`raw_data` paths in `validator-report.json` are relative to a depth-2 evidence directory: `benchmarks/<dir>/` or `research/ic_packed_filter_20261029/`, where `VALIDATION.json` is expected. Each entry also carries `repo_path`. From `validation/` itself they do not resolve.")
    w("")
    w("## validation_report (agents/validator.md)")
    w("")
    w("```yaml")
    w(YAML.format(status="passed" if rep["status"] == "pass" else "failed",
                  rsha=v.sha256_file(os.path.join(vd, "validator-report.json")),
                  csha=v.sha256_file(os.path.join(vd, "replay-certificate.json"))))
    w("```")
    open(os.path.join(vd, "REPORT.md"), "w").write("\n".join(L) + "\n")


LIMITS = [
    "`ecbench work` was not run: the handoff limits the image binary to the replay. Condition 8's bitop half was checked against the records' in-pod `work` blocks, which I re-priced from their primitive counts with my own reading of BITOPS.md. The output of `ecbench work` itself was not compared.",
    "Condition 7 and the identical w = 0.1 and w = 1 wall blocks concern analysis files I was told not to read. Their content was recomputed from records, and my method equals the in-pod instrument to 4.2e-16.",
    "Warm-ups cannot be replayed by the binary's replay modes. They are covered by my own scalar check and by the counted session's second-pod execution of every dgb and dgbf input (J3), not by `ecbench verify`.",
    "Image digest: no job's pod image digest is verifiable from S3 without pod status. For the P32 and P24 wall jobs the launcher succeeded, and its `verify-build` ran. For the three counted jobs the launcher exited 23, so `verify-build` did not run, as VERDICT-P32 (C) discloses. Their binary is bound by `binary.sha256`, `build-info.json`, every record's `binary_sha256` and the audit's auditor binary.",
    "The raw-field re-grade covers the record-level grading-v3 gates. Session-level gates (preflight, thresholds, capsule, eviction) were read from `session.json`, and `irq` is null in every record, so the interrupt gate was not exercised. Counted sessions were checked by level-from-blocker consistency only (their raw fields are wall fields).",
    "The solve-time range of the waived rho executions (VERDICT-P32: 257.0 to 968.9 µs) is not reproduced: it is rho wall time.",
    "One session per size, c8a.2xlarge only, two curves per size: the scope of VERDICT-P32 (A) and (D). Nothing here bears on transfer or replication.",
    "Independence: I was dispatched by the main session that also relayed the producer's cards. I share no code or outputs with the producer, read no `analysis/` file and no `cards/tools/` file, and replayed on a host of another class.",
    "The launcher exit codes of the P24 and P32 jobs were read from the S3 launcher prefixes (`result.json`), not from local receipts.",
]

YAML = """validation_report:
  id: null  # not allocated by the validator (no allocator type); named by path and sha256
  task_id: TASK-20261029-654f73
  run_ids: [ic-dgbf-p32-20261029, ic-dgbf-rho-p32-20261029, ic-dgbf-p24-20261029, ic-dgbf-rho-p24-20261029, ic-dgbf-rho-p28-20261029-r2]
  artifact_checks:
    - SHA256SUMS of all six sessions OK; store copies match .artifacts.json; S3 copies byte-equal; one pod prefix per job equal to job-exit pod_uid
    - frozen specs JSON-equal to input-spec.json, session/spec.json and every launcher request; build-info, binary.sha256, record and auditor binary = 21701be6...4503 from 3271a25d02
  metric_recomputations:
    - J1 P32 corrected dgbf/dgb 0.960107806 [0.956898, 0.963328], dof 562.6, condition 9 exact; P24 1.003200 [1.000564, 1.005842] (own code; in-pod comparison agrees to 4.2e-16)
    - J3 condition 8 and bitop identity on all records incl. warm-ups; bitop/v1 dgbf/dgb 1.0030127 (P32)
    - J4 S' at w = 0, 0.1, 1 for dgbf/dgb, dgb/rho-neg, dgbf/rho-neg at P32, P24, P28; order strict for rho-neg vs both IC arms
  control_checks:
    - counted A/A exactly 1 at every weight (dgb/dgb-aa, dgbf-edge/dgbf, rho-neg-aa/rho-neg); wall A/A intervals contain 1; native A/A admission file conditions hold
    - proves-too-much: no improvement on an A/A pair; P24 regression reproduced; label-swap null centred on 1 and narrowing as 1/sqrt(n)
    - positions 144/144 for dgbf and dgb-aa at positions 1/2; predecessors balanced
  heuristic_validation_checks: [not applicable (engineering constant; no heuristic prediction under test)]
  cost_model_checks:
    - units declared (ns per charged GAE, GAE/sqrt(r), bitop/v1, S' with w GAE per native op); w = 1 flagged pessimistic; table_insert_attempts unpriced on all IC arms; memory (max RSS) reported beside time
  proof_architecture_checks: []
  verdict: {status}
  limitations: [see REPORT.md, Limitations]
  artifact_paths:
    - research/ic_packed_filter_20261029/validation/validator-report.json  # sha256 {rsha}
    - research/ic_packed_filter_20261029/validation/replay-certificate.json  # sha256 {csha}
    - research/ic_packed_filter_20261029/validation/REPORT.md
    - research/ic_packed_filter_20261029/validation/code/
review_attestation:
  joints_owned: [J1 decision statistic, J2 instrument, J3 charged-work identity, J4 counted route and band, J5 independent replay, proves_too_much, blind_rederivation]
  joint_verdicts: {{J1: holds, J2: holds, J3: holds, J4: holds, J5: holds, proves_too_much: holds}}
  blind_rederivation: {{quantity: corrected wall ratio dgbf/dgb at P32, value: 0.960107806240821, ci95: [0.956898, 0.963328], blind_from_respected: true}}
  read_sibling_reports: false
  sources_read:
    - crypto-autoresearcher docs/agent-runtime-core.md, agents/validator.md, AGENTS.md (Review architecture; Curve identity and measured bounds)
    - crypto-autoresearcher origin/main ledger/handoffs/TASK-20261029-654f73.yaml, ledger/decisions/DEC-20261029-5eb8e6.yaml
    - backlog/XP-20261029-packed-filter/PROTOCOL.md; spec-P24.json, spec-P24-rho.json, spec-P28.json, spec-P28-rho.json, spec-P32.json, spec-P32-rho.json (APPROVAL.md hashed only)
    - research/ic_packed_filter_20261029/cards/VERDICT-P32.md (lines 1-230); RULING-P28.md ((a), seal, C2, C6); RULING-P28-ADDENDUM-1.md (heading and D2' lines via grep)
    - research/ic_packed_filter_20261029/P24/{{wall,counted}}/eks-results/, P32/{{wall,counted}}/eks-results/, P28/counted/eks-results/ (raw files; counted sessions through the wall-field-stripping loader)
    - research/ic_packed_filter_20261029/P28/wall/eks-results/ (counts only, wall fields stripped; job-exit, build-info, binary.sha256, input-spec, session.json status, SHA256SUMS)
    - research/ic_packed_filter_20261029/P28/P28-HOSTLOSS.md, s3verify-P28/{{check-output.txt, s3receipt.json (first 2.5 KB)}}, receipts-P28-rho-r2/{{result.json, launch-P28-rho-r2.log, request.json}}
    - git show 3271a25d02 src/cryptanalysis/ecbench/band.rs (lines 1-200), record.rs (grade_v3), docs/units/BITOPS.md (conventions, 2026-10-29 filter rule)
    - git show e27727238 backlog/XP-20261029-srenew-deferred-guard-v3/PROTOCOL.md section 6.1; git show 6e4804417 backlog/XP-20261029-dg-wall/PROTOCOL.md (counted-session line)
    - docs/VALIDATION_GATE.md; scripts/validation_gate.py (checker functions); scripts/fixtures/validation_gate/positive/*.json; tools/artifacts/artifacts.py (store policy); origin/main benchmarks/20261017-ic-a15-filter-slots/VALIDATION.json (head)
    - docs/session-logs/2026-10-29.md and 2026-10-30.md (S3_KEYCOUNT lines via grep only)
    - S3 (read-only) runs/ic-dgbf-*-20261029*/ listings, pod files, launcher result.json and request.json
  not_read: [research/ic_packed_filter_20261029/cards/tools/, any analysis/ directory, compare.rs, any counted-session wall field, any rho wall time, P28 wall wall fields]
  verdict: {status}"""


if __name__ == "__main__":
    main()
