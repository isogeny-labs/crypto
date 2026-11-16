import json, hashlib, os, secrets, subprocess
REPO = "/home/user/crypto"
STUDY = "research/ic_tableb_binary_ksum_20261115"
VAL = STUDY + "/validator"
BENCH = "benchmarks/20261115-tableb-binary-ksum"
S = "/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/val_binksum"
own = json.load(open(f"{REPO}/{VAL}/own-numbers.json"))
def sha(p): return hashlib.sha256(open(p, "rb").read()).hexdigest()
head = subprocess.check_output(["git", "-C", REPO, "rev-parse", "HEAD"]).decode().strip()
rows = own["rows"]
def cand(b, c, w): return rows[b]["candidates"][c]["by_weight"][w]
fresh = own["fresh_sessions"]
fresh_summary = {}
for s in ("F24", "F27"):
    sm = own["sessions"][s]
    f = fresh[s]
    fresh_summary[s] = {
        "dir": f"{VAL}/fresh/{s}", "session_id": sm["session_id"], "target_seed": sm["target_seed"],
        "measurement_seed": sm["measurement_seed"], "targets_per_curve": sm["targets_per_curve"], "rounds": sm["rounds"],
        "arms": f["arms"], "records": sm["records"], "status_counts": sm["status_counts"],
        "verify": "ecbench verify --replay-all --exit-code: rc 0, 144 records, 144 replays reproduced, 0 problems",
        "binary_sha256": sm["session_binary_sha256"],
        "ksum5_over_cheapest_incumbent": {w: {"incumbent": f[w]["cheapest_incumbent"], "ratio": f[w]["ksum5"]["ratio"],
                                              "delta_ci": f[w]["ksum5"]["delta_ci"], "pair_boot_ci": f[w]["ksum5"]["pair_boot_ci"]}
                                          for w in ("0", "0.1", "1")},
        "ksum4_over_cheapest_incumbent": {w: f[w]["ksum4"]["ratio"] for w in ("0", "0.1", "1")},
        "mean_S": {w: f[w]["mean_S"] for w in ("0", "0.1", "1")},
    }
build = {
    "command": "CARGO_TARGET_DIR=<validator scratch>/target cargo build --release --locked --bin ecbench (cwd /home/user/crypto, HEAD 3c323f1226, clean tree)",
    "source_identity": "git diff 2476332d22 HEAD -- src Cargo.lock: empty; git diff --name-only 2476332d22 HEAD touches only research/ic_tableb_binary_ksum_20261115/ and benchmarks/ (no build input changed)",
    "toolchain": "cargo 1.97.0 (c980f4866 2026-06-30), rustc 1.97.0 (2d8144b78 2026-07-07)",
    "wall": "12m18s, exit 0",
    "validator_binary_sha256": "9f63678b7df865190b92644aeae2de7b22027d2f91d954da368364f351385761",
    "recorded_binary_sha256": "bc96a44a3342d24569e7f2bbe7e24e55400eda47d955960e766ae66df3e08f8d",
    "byte_identical": False,
    "explanation": ("The two ELF files differ only in symbol-table text and the linker build-id: (1) 352 local symbols carry an "
                    "LLVM promoted-local suffix `.llvm.15188867642286278132` in the recorded binary and `.llvm.8458451346301811158` "
                    "in the validator's (a module hash that depends on the build directory; the validator used a separate "
                    "CARGO_TARGET_DIR), which also explains the 352-byte size difference (95489904 vs 95489552 bytes; one digit "
                    "shorter per occurrence); no other string differs; (2) after `strip`, both files are 86539696 bytes and differ in "
                    "exactly 20 bytes at offsets 0x32c-0x33f, the GNU build-id descriptor of .note.gnu.build-id (a hash over the "
                    "unstripped output). The loaded sections .text, .rodata, .data and .data.rel.ro are byte-identical. The validator "
                    "build is therefore the same program; its replays reproduced every recorded count."),
}
checks = {
    "binding": {
        "validated_snapshot": head, "results_commit": "3ef16a20a034c128c8ebcceed3592e6c1b67cbad",
        "registration_commit": "77bc2141984f3279d854e51f444d8dc1ac0f23ee",
        "study_files_identical_between_results_commit_and_snapshot": True,
        "tree_clean_at_start": True, "snapshot_pushed": "origin/claude/f4-f5-algorithm-speed-bvp7qd == HEAD",
        "working_tree_only_artifacts_used": False,
    },
    "identity_and_registration": {
        "recorded_binary_sha256": "bc96a44a3342d24569e7f2bbe7e24e55400eda47d955960e766ae66df3e08f8d",
        "every_record_and_session_carries_it": all(own["sessions"][s]["all_records_bound_to_binary"] and own["sessions"][s]["session_binary_sha256"] == "bc96a44a3342d24569e7f2bbe7e24e55400eda47d955960e766ae66df3e08f8d" for s in own["sessions"] if s.startswith("B")),
        "records_sha256_in_session_json_matches_gunzipped_records": all(own["sessions"][s]["records_sha256_matches"] for s in own["sessions"]),
        "session_spec_json_equals_registered_specs": all(own["sessions"][s]["spec_json_equals_registered_spec"] for s in own["sessions"] if s.startswith("B")),
        "spec_curves_match_round5_ledger": all(own["sessions"][s]["spec_curve_matches_ledger"] for s in own["sessions"]),
        "sessions_git_commit": sorted({own["sessions"][s]["git_commit"] for s in own["sessions"] if s.startswith("B")}),
        "sessions_git_dirty": sorted({str(own["sessions"][s]["git_dirty"]) for s in own["sessions"] if s.startswith("B")}),
        "REGISTRATION_sha256_entries_match_files_now_and_at_77bc214198": True,
        "registration_commit_time": "2026-11-15T17:28:52Z", "registered_at_txt": "2026-11-15T17:28:51Z",
        "earliest_session_start": "2026-11-15T17:29:05.452Z", "latest_session_start": "2026-11-15T17:33:09.261Z",
        "all_sessions_started_after_registration_commit": True,
        "caveat": "commit and session timestamps are producer-controlled metadata; registration order is consistent but not externally timestamped",
        "pilots_disclosed": "three sizing pilots before registration (pilot/), different seeds; sizes chosen by minimum pilot S (winner's curse disclosed)",
    },
    "dispatch_change_review": {
        "commit": "d87eb39548 (one match arm in solve_ic_binary, src/cryptanalysis/ecbench/methods.rs: \"ksum\" => KSumOracle::new(ms))",
        "generic_over_group": "KSumOracle implements DecompositionOracle<G> for every G: CountedGroup (ic_framework/plugins.rs); KSumTable (ic_boundary.rs) uses only CountedGroup::add/neg/key/is_identity",
        "every_group_op_charged": "BinaryGroup::add increments ops.adds, ::double ops.doubles (ic_boundary.rs, impl CountedGroup for BinaryGroup); the table build adds go to build_ops and are merged into the oracle_setup phase; each probe's k-1 confirming adds go to the relation ops; probes are counted as lookups (calibrated at w = 0, repriced at w by the band), inserts as table_insert_attempts behind counted:table_insert_attempts (priced at w)",
        "build_cost_measured_equals_combinatorial_count": "oracle_setup adds = sum_{d=2..k} C(A,d) 2^(d-1) and insert attempts = C(A,k) 2^(k-1) on every one of the 4320 measured ksum4/ksum5 records (A = abscissae; e.g. n27 ksum5 A = 12: 17644 adds, 12672 inserts); 0 mismatches",
        "negation_fold_valid_in_char2": "table key is key(P) >> 1 with FastPoint::pack = 2(x+1) + [y > x XOR y]; -P = (x, x XOR y) (FastCurve::neg), so the sign bit flips and key >> 1 = x + 1 is shared by +-P exactly; a probe hit is confirmed by recomputing the k-sum and comparing with target or neg(target), and the base's neg_index is built from the same neg (FactorBase::from_column_map with neg_key_of = inst.fast.neg(p).pack())",
        "soundness": "a key match that is neither +target nor -target is rejected (no relation), so a wrong key could only lose completeness, never produce a false relation",
        "verification_independent_of_oracle": "the runner checks [k]G == target in its own (parent) process with inst.mul_generator_exact (runner.rs, audit_verification), separate from the solving child; the validator additionally re-checked every recovered log with its own Python GF(2^n) arithmetic (certificates below)",
        "verdict": "sound for binary curves",
    },
    "build": build,
    "replay": {
        "sessions_replayed": 15, "nonce_designated_sample": ["B27-r1", "B21-r2", "B21-r3"],
        "records": 25920, "verified_records": 25920, "replays_reproduced": 19440, "replays": 19440, "problems": 0,
        "all_exit_codes_zero": True, "detail": "every replay 'identical'",
    },
    "fresh_sessions": fresh_summary,
    "certificates": own["certificates"],
    "pairing": {s: own["sessions"][s]["pairing"] for s in own["sessions"]},
    "pricing_audit": {
        "formula": "priced_gae(w) = G - C + w (U + n_C); S = priced_gae / sqrt(r); C, n_C = calibrated native (lookups, target_guard_probes, row_ops, ...) priced by docs/ic/calibration.json char2 row of each curve; U = cost.unpriced counts (rho: canonicalisations_uncharged) + table_insert_attempts where the counted: marker is present",
        "anomalies_by_row": {b: {k: v for k, v in a.items() if not k.endswith(":runs")} for b, a in own["pricing_audit"].items()},
        "checks": "every IC record: G == sum(adds + doubles) + C and G - C integral; every rho record: G == sum of phase gae; every record: recorded s == G / sqrt(r); no nonzero native counter in PRICED_NATIVE left unpriced by a missing pinned unit; no unpriced counter without a count",
        "matches_producer_band_files": "mean S per arm and weight equal band/B*.json mean_s to ~1e-15 (checked after own numbers were written)",
    },
    "ksum_build_audit": own["ksum_build_audit"],
    "recompute": {b: {"mean_S_pooled": rows[b]["mean_S_pooled"], "delta": rows[b]["delta"],
                      "replicate_spread_w0": rows[b]["replicate_spread_w0"],
                      "inc_tb_over_strictest_w0": rows[b]["inc_tb_over_strictest_w0"],
                      "aa_rho_neg_vs_rho_neg_aa_pairs_differing": rows[b]["aa_rho_neg_vs_rho_neg_aa_pairs_differing"]} for b in rows},
    "l1": {b: {c: {"holds_protocol_reading": rows[b]["candidates"][c]["L1_holds_protocol_reading"],
                   "holds_pair_level_reading": rows[b]["candidates"][c]["L1_holds"],
                   "max_upper_pooled_protocol_reading": rows[b]["candidates"][c]["max_upper_pooled_protocol_reading"],
                   "max_upper_any_replicate_protocol_reading": rows[b]["candidates"][c]["max_upper_any_replicate_protocol_reading"],
                   "by_weight": {w: {"incumbent": cand(b, c, w)["strictest_incumbent_pooled"], "ratio": cand(b, c, w)["ratio"],
                                     "delta_ci_target_level": cand(b, c, w)["delta_ci_target_level"],
                                     "cluster_boot_ci": cand(b, c, w)["cluster_boot_ci"],
                                     "per_replicate": [{"incumbent": p["incumbent"], "ratio": p["ratio"], "upper": p["upper_protocol_reading"]} for p in cand(b, c, w)["per_replicate"]],
                                     "gap_to_rho_neg": cand(b, c, w)["gap_to_rho_neg"], "over_bsgs_neg": cand(b, c, w)["over_bsgs_neg"]}
                                 for w in ("0", "0.1", "1")}}
               for c in ("ksum5", "ksum4")} for b in rows},
    "intervals": {"resamples": 4000, "seed": 20261002, "protocol_reading": "upper = max(linearised-t on (session, target) means over rounds, target-cluster bootstrap stratified by session); percentile, type-7 quantiles, Python random.Random", "pair_level_reading": "max(linearised-t on (session, target, round) pairs, pair bootstrap stratified by session)", "note": "the producer's ecbench compare upper ends (max per row and candidate) are 0.4-1.4% wider than the validator's protocol reading; every L1 decision is identical under both readings"},
    "band_n27_primary": {w: {"ic_ksum5_mean_S": rows["27"]["band"][w]["ksum5"], "rho_neg_mean_S": rows["27"]["band"][w]["rho_neg"],
                             "ic_total_gae_mean": rows["27"]["band"][w]["ksum5_total_gae_mean"], "rho_total_gae_mean": rows["27"]["band"][w]["rho_neg_total_gae_mean"],
                             "ic_over_rho": rows["27"]["band"][w]["ksum5_over_rho_neg"]} for w in ("0", "0.1", "1")},
    "band_all_rows": {b: rows[b]["band"] for b in rows},
    "rho_reference": "rho-neg (rho.negation) runs in every session on the same curve, targets and algorithm seeds as every IC arm, in the same binary; random-binary curves (not Koblitz) have A = 2 (automorphisms_available 2 in every record), so negation is the strongest symmetry available; rho-neg is cheaper than ksum5 at every weight on every row (order holds across the band)",
    "null_controls": {
        "neighbouring_incumbents": {b: rows[b]["null_control_neighbouring_incumbents"] for b in rows},
        "reading": "two incumbents that differ only in base size give ratios 1.000-1.11 with intervals of about +-1.5%; where they are tied (n24 w=0: inc-n80/inc-n67 1.0014 [0.987, 1.016]; n27 w=1: inc-n150/inc-n178 1.0001 [0.989, 1.011]) the interval covers 1, so the interval machinery does not manufacture the 32-54% effects",
        "aa_arm": "rho-neg-aa shares rho-neg's seeds, so the A/A arm is identical by construction (0 differing pairs) and carries no noise information; the protocol's noise control is the replicate spread delta (max/min of per-session mean S at w = 0 over inc-tb, rho-neg, ksum5, ksum4)",
        "trend": "the ksum5 ratio falls monotonically with field size (1.14, 1.04, 0.68, 0.64, 0.46 for n15..n27 at w = 0), as a k-sum table against a pair table is expected to (heuristic r^(5/9) vs r^(2/3) balanced cost, ratio ~ r^(-1/9): 0.68 at n21 predicts ~0.48 at n27; observed 0.46); this is a consistency check, not a claim",
    },
    "heuristic_and_cost_model": {
        "unit": "ecbench.gae (counted group additions and doublings; S = GAE/sqrt(r)); unpriced native work priced at w in {0, 0.1, 1} GAE per operation (AGENTS.md 5a)",
        "optimistic_assumptions": "one affine group operation = 1 GAE for every arm (field inversions not separately priced); IC walk bookkeeping counters (walk_mix_hashes, walk_modr_adds, under counted:walk_bookkeeping) and rho's cycle_window_word_ops are outside the band's U for all arms; pricing the IC walk counters would charge the incumbents more than the candidates (about 4x more trials per run on n27), so leaving them out does not favour the candidate; the IC-vs-rho gap is computed under the band's definition of U and would move if these counters were priced",
        "memory": "max RSS per run about 17-18 MB for every IC arm on n27 (ksum5 median 17200 KiB, inc-n150 17576 KiB, rho-neg 14896 KiB); ksum5 table 12672 entries vs inc-n150 22500 on n27; memory is not binding and not part of the claim",
        "success_probability": "every one of the 25920 study records (19440 measured, 6480 warm-up) and 288 fresh records completed and verified (no censoring, no timeouts); expected cost is the mean over runs, nothing extrapolated",
        "scale": "n = 15..27 (log2 r 14.0-24.4) only; no transfer to larger fields is claimed (PROTOCOL.md 'Not claimed'); the claimed improvement is measured on the exact rows, so inventor-protocol section 6 extrapolation steps do not arise",
        "incumbent_tuning": "tuned incumbents bracket the pilot optimum: n21 interior (62 of 43/62/87) at every w; n27 interior at w = 0, 0.1 (150) and at the upper edge at w = 1 (178 by 0.003 S over 150); n24 interior at w = 0 (67), upper edge (80) at w = 0.1, 1. Fresh validator sessions added one size beyond each edge: n27 inc-n210 is worse than inc-n150 at every w; n24 inc-n94 is 1.5% cheaper than inc-n80 at w = 1 (25.55 vs 25.94, 24 targets), which would move the n24 w = 1 ratio from about 0.605 to about 0.62; immaterial to L1",
    },
    "discrepancies": [
        "Claim summary wording: '95% upper ends <= 0.70 / 0.66 / 0.47, all weights and replicates' holds for the pooled upper ends at every weight (producer 0.699/0.657/0.470; validator 0.695/0.652/0.468) but not for the per-replicate upper ends (producer 0.724/0.685/0.484; validator 0.718/0.678/0.478). results.md reports both correctly in separate columns; the L1 rule (every upper < 1) holds either way.",
        "Recorded binary sha256 not reproduced byte-for-byte (9f63678b... vs bc96a44a...): explained in checks.build (symbol-table .llvm suffixes and build-id only; code and data sections identical).",
        "Replay host: the validator replay ran in the same container class as the producer's run (not a second host class, unlike the prime twin); ecbench assigned a different hardware class id (ECBHW1h3956e07fafaa vs the producer's verify ECBHW1h0832b230440b) but no second host is claimed.",
        "Fresh sessions record git_commit null (the validator's ecbench binary lives outside the repository); the spec, binary sha256 and seeds are recorded instead.",
    ],
}
validation_report = {
    "id": "VAL-20261115-" + secrets.token_hex(3),
    "task_id": "TASK-unassigned (validation-gate request from the producing session, 2026-11-15)",
    "run_ids": sorted(own["sessions"][s]["session_id"] for s in own["sessions"] if s.startswith("B")),
    "verdict": "passed",
    "verdict_scope": "the 15 sessions are admissible counted-cost evidence and the headline recomputes; this is not an ECDLP claim, a speedup over rho, or a promotion",
    "limitations": [
        "validator is a subagent launched from the producing session (same Claude-Session id as the study commits): independent context, code, build, nonce and seeds, not an independent operator; no keyring signature",
        "same host/container class as the producer; L0 isolation; wall time not claimed or checked",
        "registration timing rests on producer-controlled git and session timestamps",
        "incumbent sizes and candidate sizes were tuned on disclosed pilots (winner's curse a few percent for both sides)",
        "ecbench replay is a re-execution of the producer's source, not an independent implementation; only pricing, statistics and scalar certificates were re-implemented",
        "rows n15-n27 only; Table B's own S values come from another harness and are not compared",
    ],
}
ctx = {
    "repo": REPO, "study_rel": STUDY, "validator_rel": VAL, "bench_rel": BENCH,
    "own_numbers": f"{REPO}/{VAL}/own-numbers.json", "replay_dir": f"{S}/replay/out",
    "nonce_file": f"{REPO}/{VAL}/nonce.txt", "build": build, "sample": ["B27-r1", "B21-r2", "B21-r3"],
    "fresh_summary": fresh_summary,
    "validator_code": [f"{REPO}/{VAL}/recompute.py", f"{REPO}/{VAL}/mkreports.py", f"{REPO}/{VAL}/own-numbers.json"],
    "integrity_ok": True,
    "validated_commit": head,
    "method": ("Independent recomputation in Python (stdlib) from the 15 committed records.jsonl.gz, by validator/recompute.py, written "
               "without reading analysis/*.mjs, summary.json or the producer's interval code (results.md, README.md and PROTOCOL.md were "
               "read before; results.json was opened only after own-numbers.json was first written; afterwards, seeing that compare pairs (session, target) "
               "means, the validator added a target-level linearised t as the protocol reading and a null control; no ratio, pricing or pairing code changed). Pricing re-implemented from the "
               "documented band formula priced_gae(w) = G - C + w (U + n_C) (AGENTS.md 5a; band.rs) with the pinned char2 calibration rows; "
               "every recovered log re-checked by own GF(2^n) arithmetic; strictest incumbent chosen on the same data (pooled and per "
               "replicate); intervals by linearised t and stratified bootstraps (4000 resamples, seed 20261002). Replay: ecbench rebuilt by "
               "the validator from the recorded source, verify --replay-all on all 15 sessions (nonce-designated sample B27-r1, B21-r2, "
               "B21-r3), and two fresh sessions (n24, n27; 24 new targets, 1 round, nonce-derived seeds) with rho-neg, the strictest "
               "incumbents plus one size beyond the bracket, ksum4 and ksum5 at the frozen sizes. No commit was made."),
    "checks": checks, "validation_report": validation_report,
}
json.dump(ctx, open(f"{S}/work/ctx.json", "w"), indent=1)
print("ok", validation_report["id"])
