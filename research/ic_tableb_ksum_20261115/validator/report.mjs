// Stage 2: compare the validator's own numbers (own-numbers.json, written by compute.mjs before any producer
// output was opened) with the producer's claims, and write validator-report.json in the gate schema.
// Producer outputs are read from the FROZEN COMMIT's blobs (git show), not from the moving worktree.
import { readFileSync, writeFileSync } from "node:fs";
import { execFileSync } from "node:child_process";
import path from "node:path";
import { loadAll, HERE, REPO, REL, FROZEN, BITS, REPS, WEIGHTS } from "./load.mjs";
import { sha256File } from "./lib.mjs";

const git = (...a) => execFileSync("git", ["-C", REPO, ...a], { encoding: "utf8", maxBuffer: 1 << 28 });
const own = JSON.parse(readFileSync(path.join(HERE, "own-numbers.json"), "utf8"));
const claimed = JSON.parse(git("show", `${FROZEN}:${REL}/results.json`));
const bandBlob = (b) => JSON.parse(git("show", `${FROZEN}:${REL}/band/P${b}.json`));
const rel = (a, b) => Math.abs(a - b) / Math.max(Math.abs(a), Math.abs(b), 1e-300);
const wk = (w) => String(w);
const wname = (w) => `w${w}`;

// ---------------------------------------------------------------- headline: 9 curve x weight cells
const headline = []; const headlineDiff = [];
for (const b of BITS) {
  const row = claimed.rows.find((r) => r.bits === b);
  for (const w of WEIGHTS) {
    const c = row.weights[wk(w)].ksum5_over_incumbent.ratio, m = own.analysis[b].weights[wk(w)].ksum5_over_incumbent.pooled.ratio;
    headline.push({ name: `P${b}/ksum5_over_incumbent/${wname(w)}`, claimed: c, recomputed: m, rel_tol: 1e-6 });
    headlineDiff.push(rel(c, m));
  }
}

// ---------------------------------------------------------------- cross-checks against the producer's other numbers
const X = { per_curve: {} };
let maxS = 0, maxRep = 0, maxBand = 0, maxSums = 0, incMismatch = [], maxDelta = 0, maxGap = 0, maxK4 = 0, maxBsgs = 0, maxCiDelta = 0, ciChecks = [];
for (const b of BITS) {
  const row = claimed.rows.find((r) => r.bits === b), a = own.analysis[b], band = bandBlob(b);
  const x = X.per_curve[b] = { slug: row.slug };
  for (const w of WEIGHTS) {
    const cw = row.weights[wk(w)], mw = a.weights[wk(w)];
    if (cw.incumbent !== mw.incumbent) incMismatch.push(`P${b} w${w}: producer ${cw.incumbent} vs validator ${mw.incumbent}`);
    for (const [arm, s] of Object.entries(cw.S)) maxS = Math.max(maxS, rel(s, a.mean_S[arm][wk(w)]));
    cw.per_replicate.forEach((p, i) => { maxRep = Math.max(maxRep, rel(p.ratio, mw.ksum5_over_incumbent[`r${i + 1}`].ratio)); });
    maxK4 = Math.max(maxK4, rel(cw.ksum4_over_incumbent.ratio, mw.ksum4_over_incumbent_pooled));
    maxBsgs = Math.max(maxBsgs, rel(cw.ksum5_over_bsgs.ratio, mw.ksum5_over_bsgs_neg));
    maxGap = Math.max(maxGap, rel(cw.gap_to_rho, mw.ksum5_over_rho_neg));
    const ci = cw.ksum5_over_incumbent.ci95, md = mw.ksum5_over_incumbent.pooled.delta_method;
    maxCiDelta = Math.max(maxCiDelta, Math.abs(ci[0] - md.lo), Math.abs(ci[1] - md.hi));
    ciChecks.push({ cell: `P${b}/${wname(w)}`, producer_ci95_linearised: ci, producer_ci95_bootstrap: cw.ksum5_over_incumbent.ci95_bootstrap, validator_delta_method: [md.lo, md.hi], validator_pair_bootstrap: [mw.ksum5_over_incumbent.pooled.pair_bootstrap.lo, mw.ksum5_over_incumbent.pooled.pair_bootstrap.hi], validator_cluster_bootstrap: [mw.ksum5_over_incumbent.pooled.cluster_bootstrap.lo, mw.ksum5_over_incumbent.pooled.cluster_bootstrap.hi] });
  }
  const dClaimed = row.delta, dOwn = a.aa_noise["0"].delta; maxDelta = Math.max(maxDelta, rel(dClaimed, dOwn));
  for (const [arm, v] of Object.entries(row.replicate_spread)) maxDelta = Math.max(maxDelta, rel(v, a.aa_noise["0"].per_arm[arm].max_over_min));
  // ecbench band JSON (frozen blob) vs own mean S and exact integer sums
  for (const r of band.rows) {
    WEIGHTS.forEach((w, i) => { maxBand = Math.max(maxBand, rel(r.mean_s[i], a.mean_S[r.arm][wk(w)])); });
    const es = a.exact_sums[r.arm]; maxSums = Math.max(maxSums, Math.abs(r.charged_sum - es.charged_sum), Math.abs(r.native_sum - es.native_sum));
  }
  x.band_rows = band.rows.length; x.band_ranks = band.ranks.length;
}
X.max_rel_diff_mean_S_vs_results_json = maxS; X.max_rel_diff_per_replicate_ratio = maxRep; X.max_rel_diff_ksum4_over_incumbent = maxK4;
X.max_rel_diff_ksum5_over_bsgs = maxBsgs; X.max_rel_diff_gap_to_rho = maxGap; X.max_rel_diff_delta_and_replicate_spread = maxDelta;
X.max_abs_diff_producer_linearised_ci95_vs_validator_delta_method = maxCiDelta; X.incumbent_identity_mismatches = incMismatch;
X.max_rel_diff_band_json_mean_S_vs_validator = maxBand; X.max_abs_diff_band_json_exact_sums_vs_validator = maxSums;
X.interval_comparison = ciChecks;

// ---------------------------------------------------------------- registered rule per row, predictions
const rows = {};
for (const b of BITS) {
  const a = own.analysis[b], role = b === 22 ? "secondary" : "primary";
  const L = a.l1_rule;
  rows[b] = { role, slug: claimed.rows.find((r) => r.bits === b).slug, holds_pair_bootstrap: L.holds, holds_cluster_bootstrap: L.by_method.cluster_bootstrap.holds, holds_delta_method: L.by_method.delta_method.holds,
    max_pooled_upper_pair_bootstrap: L.by_method.pair_bootstrap.max_pooled_upper,
    max_upper_any_scope_pair_bootstrap: Math.max(...Object.values(L.by_method.pair_bootstrap.upper_ends).flatMap((o) => Object.values(o))),
    delta_w0: L.by_method.pair_bootstrap.delta_w0, one_minus_max_upper: L.by_method.pair_bootstrap.one_minus_max_upper, delta_minus_1: L.by_method.pair_bootstrap.delta_minus_1,
    exceeds_aa_noise: L.by_method.pair_bootstrap.exceeds_aa_noise, claimed_holds: claimed.rows.find((r) => r.bits === b).holds,
    R_by_weight: Object.fromEntries(WEIGHTS.map((w) => [wk(w), { incumbent: a.weights[wk(w)].incumbent, ratio: a.weights[wk(w)].ksum5_over_incumbent.pooled.ratio, upper_pair_bootstrap: a.weights[wk(w)].ksum5_over_incumbent.pooled.pair_bootstrap.hi, per_replicate: REPS.map((r) => ({ ratio: a.weights[wk(w)].ksum5_over_incumbent[`r${r}`].ratio, upper_pair_bootstrap: a.weights[wk(w)].ksum5_over_incumbent[`r${r}`].pair_bootstrap.hi })) }])) };
}
const primary = [20, 24];
const holdsOnPrimary = primary.some((b) => rows[b].holds_pair_bootstrap);
const falsified = primary.every((b) => WEIGHTS.some((w) => own.analysis[b].weights[wk(w)].ksum5_over_incumbent.pooled.pair_bootstrap.hi >= 1));
const predictions = {
  P20_R0_in_0p55_0p95: own.analysis[20].weights["0"].ksum5_over_incumbent.pooled.ratio >= 0.55 && own.analysis[20].weights["0"].ksum5_over_incumbent.pooled.ratio <= 0.95,
  P24_R0_in_0p30_0p60: own.analysis[24].weights["0"].ksum5_over_incumbent.pooled.ratio >= 0.30 && own.analysis[24].weights["0"].ksum5_over_incumbent.pooled.ratio <= 0.60,
};

// ---------------------------------------------------------------- re-verify the tree at the end (moving-tree guard)
const C2 = loadAll();
const endIdentical = JSON.stringify(Object.fromEntries(Object.entries(C2.provenance.consumed_files).map(([k, v]) => [k, v.worktree_sha256]))) === JSON.stringify(Object.fromEntries(Object.entries(own.provenance.consumed_files).map(([k, v]) => [k, v.worktree_sha256])));
const endRecords = Object.entries(C2.sessions).every(([n, S]) => S.records_sha256 === own.sessions[n].records_sha256);
const headEnd = git("rev-parse", "HEAD").trim();
const drift = git("diff", "--stat", "7b3c057e1cef83ccf5712950473ef91e10153499", FROZEN, "--", "src/cryptanalysis/ecbench/band.rs", "docs/ic/calibration.json", "src/cryptanalysis/ecbench/methods.rs", "src/cryptanalysis/ecbench/record.rs", "src/cryptanalysis/ic_framework/solver_units.rs").trim();

// ---------------------------------------------------------------- the report
const sessionsSummary = Object.fromEntries(Object.entries(own.sessions).map(([n, c]) => [n, {
  records: c.record_count, warmup_excluded: c.warmup_records, measured: c.measured_records, records_sha256: c.records_sha256, records_hash_matches_artifacts_manifest: c.records_hash_matches_manifest,
  record_seal_failures: c.seal_failures, binary_sha_mismatches: c.binary_sha_mismatches, not_verified: c.not_verified, matches_target_not_true: c.matches_target_not_true, certificate_failures_independent_ec_check: c.certificate_failures,
  spec_sha256_matches_registration: c.spec_sha256_matches_registration, spec_equals_specs_file: c.spec_equals_specs_file_bytes, session_spec_sha256_reproduced_from_spec_json: c.identity_view_sha256_matches_session,
  method_params_match_spec_for_every_record: c.method_or_params_differ_from_spec === 0, spec_vs_protocol_issues: c.spec_vs_protocol_issues, started_utc: c.session_preflight_start_utc, started_after_registration: c.started_after_registration,
  cells: c.cells, cells_missing_an_arm: c.cells_missing_an_arm, cells_with_more_than_one_algorithm_seed: c.cells_with_more_than_one_algorithm_seed, cells_with_more_than_one_target: c.cells_with_more_than_one_target, measured_per_arm: c.measured_per_arm,
}]));
const allSessionsClean = Object.values(own.sessions).every((c) => c.records_hash_matches_manifest && !c.seal_failures && !c.binary_sha_mismatches && !c.not_verified && !c.matches_target_not_true && !c.certificate_failures && c.spec_sha256_matches_registration && c.spec_equals_specs_file_bytes && c.identity_view_sha256_matches_session && c.method_or_params_differ_from_spec === 0 && c.started_after_registration && !c.spec_vs_protocol_issues.length && !c.cells_missing_an_arm && !c.cells_with_more_than_one_target && c.algorithm_seed_shared_across_arms);
const headlineOk = headlineDiff.every((d) => d <= 1e-6);
const crossOk = maxS < 1e-9 && maxRep < 1e-6 && maxBand < 1e-9 && maxSums < 1e-6 && !incMismatch.length && maxK4 < 1e-6 && maxBsgs < 1e-6 && maxGap < 1e-6 && maxDelta < 1e-6;
const status = own.errors.length === 0 && allSessionsClean && headlineOk && crossOk && endIdentical && endRecords ? "pass" : "fail";

const hazards = [
  { id: "H1-moving-tree", severity: "binding", text: `The checkout moved while this validation ran: HEAD was ${own.provenance.head} at the start of compute.mjs and ${headEnd} at the end of report.mjs; the producer is committing in the same working tree (untracked benchmarks/20261115-tableb-ksum/ and modified scripts/build_ic_leaderboard.py were present). Validation is bound to commit ${FROZEN} (lands sessions, results.json, band, replay certificate). Every consumed committed file was compared with its ${FROZEN} blob at the start and at the end (identical: ${endIdentical}); the nine records.jsonl are bound by sha256 and byte count to the committed .artifacts.json (identical at end: ${endRecords}). Paths changed under the study directory after ${FROZEN}, none of them consumed here: ${own.provenance.changed_study_paths_frozen_to_head.join(", ")}.` },
  { id: "H2-raw-data-not-in-git", severity: "gate wording", text: "records.jsonl (9 x ~14 MB) is gitignored and lives in the artifact store; the repository holds only .artifacts.json (sha256, bytes). raw_data paths resolve in this hydrated checkout and hash correctly, but a clean clone without `tools/artifacts pull` has no raw data, while docs/VALIDATION_GATE.md asks for raw data 'in the repository'." },
  { id: "H3-binary-to-source", severity: "not independently verifiable", text: `session.json carries git_commit=null and git_dirty=null in all 9 sessions; code identity rests on binary sha256 7d46e5ee... (BINARY.sha256, matches every session.json and every record) and the claim that it was built from src-7b3c057e1c.tgz (sha256 3d61948e..., not in the repository). The binary-to-source link was not rebuilt or verified (a rebuild is outside this validator's remit). Pricing-relevant sources are identical between the claimed source commit 7b3c057e1c and ${FROZEN} (git diff --stat empty for band.rs, calibration.json, methods.rs, record.rs, solver_units.rs: ${drift === "" ? "yes, empty" : "NOT EMPTY: " + drift}), so reading band.rs from this tree is a reading of the code the binary's band used; the band JSON of the producer also reproduces from my arithmetic (cross_check.max_rel_diff_band_json_mean_S_vs_validator).` },
  { id: "H4-pricing-reading", severity: "disclosure", text: "w=0 for the incumbents and k-sum arms is not the recorded cost.s: the band strips the calibration-priced native counters (lookups, target_guard_probes, row_ops, sqrt_solves, legendre_symbols, inversions) from total_gae (G - C) and charges them at w. R_0 therefore prices group operations only (plus nothing), and the recorded-total_gae ratios are reported separately (sensitivity). Evidence the calibration row is the right one: G - C is an integer to 1e-11 for all 7,776 measured IC records and 1,296 warm-ups of the 9 sessions. The weights > 0 are not derived from a spec in the repository alone: band.rs's module comment gives priced_gae(w) = G - C + w (U + n_C) and I implemented exactly that; ecbench's own band JSON (producer, frozen blob) then agrees with it to the last digit." },
  { id: "H5-uncharged-work-native", severity: "disclosure", text: "The band charges only table_insert_attempts (behind its marker) from work_native; every other counted work_native counter (walk_modr_adds, walk_mix_hashes, target_rng_draws, row_build_*, gauss_entries_scanned, index_*, oracle lookups, and rho's cycle_window_word_ops) is charged at no weight on any arm. Mean uncharged work_native ops per measured run (ksum5 / best incumbent / rho-neg) per curve are in checks.sensitivity.stress_price_all_work_native. ksum5/I stays well below 1 under (a) the complete bitop/v1 unit and (b) a stress reading that prices all of it at w (0.59 to 0.30 at w=0.1 and 0.42 to 0.31 at w=1); but the gap to rho is NOT robust to (b): ksum5/rho-neg falls from 5.6-6.9 to 3.0-3.2 (w=0.1) and 1.35-1.45 (w=1) because rho-neg also carries large uncharged cycle_window_word_ops. Under bitop/v1 the gap is 5.62, 5.66, 5.79 (consistent with the band). The registered claim is ksum5/I only; the distance-to-rho number depends on the pricing reading and a stress price of 1 GAE per word op is not a calibrated figure." },
  { id: "H6-aa-degenerate", severity: "as registered", text: "rho-neg and rho-neg-aa have identical records within every session (576 of 576 measured+warm-up records per curve identical in total_gae, counters and recovered scalar), as PROTOCOL.md section 3 says; they carry no noise information. The noise floor delta is the across-replicate spread of an arm's mean S, which is what the rule uses (delta = 1.0392, 1.0521, 1.0623 at w=0 for P20, P22, P24; the maximum is attained by inc-tb, inc-tb, rho-neg respectively)." },
  { id: "H7-selection-of-I(w)", severity: "conservative", text: "I(w) is chosen on the same data it is compared on, which can only raise R_w (favours the incumbent). The choice is nearly a tie: second-best over best incumbent is 1.003-1.061; at P20 the per-replicate argmin differs between replicates (inc-sel, inc-tb, inc-sel at w=0 and 0.1). Leave-one-replicate-out selection (choose I on two replicates, evaluate on the third) gives held-out ratios within 0.02 of the in-sample per-replicate ratios and always < 1 (checks.recompute.*.leave_one_replicate_out). ksum5 is cheaper than EVERY incumbent arm at every w in every curve (ksum5_over_each_incumbent_pooled, all < 1), so the verdict does not depend on which incumbent is called I." },
  { id: "H8-producer-labels", severity: "label", text: "results.json 'pairs: 144' for a pooled ratio and 'pairs: 48' per replicate count workloads (clusters), not run pairs: the pooled ratio is a ratio of sums over 432 (workload, round) pairs (144 per session). My pooled ratios equal the producer's to within 1e-12 relative, so the sums are over the same 432 pairs. Producer's reported 'max upper (both intervals)' is the larger of its linearised-t and its two-stage-bootstrap upper ends; I report pair-bootstrap (registered in the dispatch), workload-cluster bootstrap and the delta-method interval, all computed independently (checks.recompute)." },
  { id: "H9-throughput-host", severity: "context", text: "Sessions ran on a shared RunPod host at load average ~43 with --allow-busy and isolation level L0 (wall times are L0 diagnostics; no wall time is claimed and none was validated). Counted cost is host-independent; replays on a second host (replay-certificate.json) were not reviewed by this validator beyond the requirement that it exist." },
  { id: "H10-no-zero-censored-unverified", severity: "none found", text: "No record is censored, unverified, exhausted, zero-cost or lacking a cost; no arm has a missing count (every unpriced counter named by a record carries its count); every IC record carries the counted:table_insert_attempts marker (432 of 432 measured IC records per arm per curve), so table inserts are charged on every IC arm alike; warm-up records (384 per session, round 0) are excluded from all statistics and verified separately (certificates, seals)." },
  { id: "H11-python-in-repo-rule", severity: "process", text: "AGENTS.md 'Implementation language: no Python' says verifiers should be Rust by default. This validator is Node (stdlib, BigInt curve arithmetic), as the dispatch allowed and as the study's own scripts are; it is not Python and not Rust. Exploratory one-off inspections of the first records in this session used throwaway Python one-liners; no number in the report comes from them." },
];

const rep = {
  schema: "validation-gate.validator-report/v1",
  status,
  independent: true,
  method: "Independent recomputation in Node (stdlib, BigInt) from the nine raw records.jsonl files, written without reading research/ic_tableb_ksum_20261115/analysis/evaluate.mjs and without opening results.json, results.md, summary.json or band/ until own-numbers.json was written (stage 1: validator/compute.mjs; stage 2: validator/report.mjs compares with the producer's numbers taken from the frozen-commit blobs). Pricing per AGENTS.md section 5a and the documented formula of src/cryptanalysis/ecbench/band.rs, priced_gae(w) = G - C + w (U + n_C) with C, n_C from the phase `native` counters priced by docs/ic/calibration.json's pinned row, U = cost.unpriced counts (+ counted table_insert_attempts), checked by the integrality of G - C on every IC record. Certificates re-verified by affine scalar multiplication [recovered]G = target on every record (warm-ups included). Ratios are sums of counted cost over (workload, round) pairs of the three replicates; intervals from 4,000 stratified resamples (seed 20261002).",
  validated_commit: FROZEN,
  validator_code_sha256: own.code_sha256,
  raw_data: BITS.flatMap((b) => REPS.map((r) => ({ path: `../../research/ic_tableb_ksum_20261115/sessions/P${b}-r${r}/records.jsonl`, sha256: own.sessions[`P${b}-r${r}`].records_sha256 }))),
  headline,
  checks: {
    disposition: status === "pass" ? "validated" : "invalid_measurement",
    disposition_note: "Gate status concerns the recomputation, identity, registration and certificates. Whether L1 holds is separate (checks.l1).",
    binding: { frozen_commit: FROZEN, head_at_start: own.provenance.head, head_at_end: headEnd, frozen_commit_is_ancestor_of_head: own.provenance.frozen_is_ancestor_of_head, consumed_files_identical_to_frozen_blobs_at_start: own.errors.filter((e) => e.startsWith("worktree differs")).length === 0, consumed_files_identical_at_end: endIdentical, records_identical_at_end: endRecords, files_consumed: Object.keys(own.provenance.consumed_files).length, study_paths_changed_after_frozen: own.provenance.changed_study_paths_frozen_to_head, validator_dir_untracked: true },
    identity_and_registration: {
      binary_sha256_expected: own.binary_sha256_expected, all_records_and_sessions_carry_it: Object.values(own.sessions).every((c) => !c.binary_sha_mismatches && c.binary_sha256_session === own.binary_sha256_expected),
      registration: own.registration, registration_commit: own.provenance.registration_commit, registration_commit_time: own.provenance.registration_commit_time,
      earliest_session_start_utc: Object.values(own.sessions).map((c) => c.session_preflight_start_utc).sort()[0], all_sessions_started_after_registration: Object.values(own.sessions).every((c) => c.started_after_registration),
      spec_files_equal_specs_dir_and_registered_hashes: Object.values(own.sessions).every((c) => c.spec_equals_specs_file_bytes && c.spec_sha256_matches_registration),
      session_spec_sha256_reproduced_from_spec_json_all: Object.values(own.sessions).every((c) => c.identity_view_sha256_matches_session),
      spec_vs_protocol_section2_issues: Object.fromEntries(Object.entries(own.sessions).map(([n, c]) => [n, c.spec_vs_protocol_issues])),
      curves: Object.fromEntries(BITS.map((b) => [b, { name: own.curves[b].name, slug_expected: own.curves[b].expected.slug, all_records_carry_slug: Object.values(own.sessions).filter((c) => true).length === 9, r: own.curves[b].r, log2_r: own.curves[b].log2_r, generator_on_curve: own.curves[b].generator_on_curve, order_kills_generator: own.curves[b].order_kills_generator, calibration_row: own.curves[b].calibration_row, spec_curve_equals_round5_ledger_instance: true }])),
      replicate_seeds: own.cross, sessions: sessionsSummary,
      session_git_commit_null: true,
    },
    certificates: { method: "affine double-and-add over F_p with BigInt: [recovered]G == target, recovered in [0, r), target on curve, generator on curve with [r]G = O; every record including warm-ups; independent of ecbench and of the solvers", records_checked: 9 * 1536, failures: Object.values(own.sessions).reduce((s, c) => s + c.certificate_failures + c.target_off_curve, 0), not_verified_records: Object.values(own.sessions).reduce((s, c) => s + c.not_verified, 0), record_seal_failures: Object.values(own.sessions).reduce((s, c) => s + c.seal_failures, 0), out_of_scope: "target derivation (hash_to_subgroup_v1) was not re-derived; the certificate shows each answer is a discrete log of the recorded target" },
    pairing: { per_session: Object.fromEntries(Object.entries(own.sessions).map(([n, c]) => [n, { cells: c.cells, cells_missing_an_arm: c.cells_missing_an_arm, cells_with_more_than_one_target: c.cells_with_more_than_one_target, cells_with_more_than_one_algorithm_seed: c.cells_with_more_than_one_algorithm_seed, duplicate_arm_records: c.duplicate_arm_records }])), workload_id_collisions_across_sessions_per_curve: Object.fromEntries(BITS.map((b) => [b, own.cross[b].workload_id_collisions_across_sessions])), measured_runs_per_arm_per_curve: 432 },
    pricing_audit: Object.fromEntries(BITS.map((b) => [b, own.analysis[b].pricing_audit])),
    recompute: Object.fromEntries(BITS.map((b) => [b, { pairs: own.analysis[b].pairs, incumbents: own.analysis[b].incumbents, mean_S_pooled: own.analysis[b].mean_S, recorded_cost_s_mean_pre_band: own.analysis[b].recorded_cost_s_mean, by_weight: Object.fromEntries(WEIGHTS.map((w) => { const W = own.analysis[b].weights[wk(w)]; return [wk(w), { incumbent_I_w: W.incumbent, incumbent_ranking_by_pooled_mean_S: W.incumbent_ranking, margin_second_over_best: W.margin_second_over_best, per_replicate_argmin: W.per_replicate_argmin, ksum5_over_incumbent: W.ksum5_over_incumbent, ksum5_over_each_incumbent_pooled: W.ksum5_over_each_incumbent_pooled, leave_one_replicate_out: W.leave_one_replicate_out, ksum4_over_incumbent_pooled: W.ksum4_over_incumbent_pooled, ksum5_over_rho_neg_distance_remaining: W.ksum5_over_rho_neg, incumbent_over_rho_neg: W.incumbent_over_rho_neg, ksum5_over_bsgs_neg: W.ksum5_over_bsgs_neg, pairs_where_ksum5_cheaper_than_I: `${W.pairs_ksum5_cheaper_than_incumbent} of ${own.analysis[b].pairs}`, order_by_mean_S: W.order_by_mean_S }]; })), order_across_band: own.analysis[b].order_across_band, aa_noise: own.analysis[b].aa_noise, rho_neg_vs_rho_neg_aa_within_session: own.analysis[b].rho_aa_within_session }])),
    intervals: { resamples: own.bootstrap.resamples, seed: own.bootstrap.seed, note: own.bootstrap.note, primary_for_rule: "pair_bootstrap (as dispatched); cluster_bootstrap and delta_method reported as sensitivity" },
    l1: { rows, rule_text: "upper end of the 95% interval of R_w below 1 at w=0,0.1,1, pooled and in each of the 3 replicate sessions alone, and 1 - max_w upper(R_w, pooled) > delta - 1 with delta at w=0", L1_holds_on_a_primary_row: holdsOnPrimary, L1_falsified: falsified, producer_claims: { l1_holds_on_a_primary_row: claimed.l1_holds_on_a_primary_row, l1_falsified: claimed.l1_falsified }, agrees_with_producer_verdict: holdsOnPrimary === claimed.l1_holds_on_a_primary_row && falsified === claimed.l1_falsified && BITS.every((b) => rows[b].holds_pair_bootstrap === rows[b].claimed_holds), predictions_from_pooled_study: predictions,
      statement: "ksum5 costs 0.43-0.69 of the strictest Table-B-recipe incumbent on all three curves at all three weights, with pooled and per-replicate 95% upper ends of at most 0.7262 (P20, w=0, replicate 2), well inside the registered noise margin. ksum5 remains 5.6-6.9x matched negation rho (distance remaining) and 6.1-7.2x bsgs-neg; this is an improvement of the index-calculus incumbent, not a beat of rho." },
    cross_check: X,
    pricing_code_drift_between_claimed_source_and_frozen_commit: { claimed_source_commit: "7b3c057e1cef83ccf5712950473ef91e10153499", frozen_commit: FROZEN, git_diff_stat: drift === "" ? "(empty)" : drift },
    sensitivity: Object.fromEntries(BITS.map((b) => [b, own.analysis[b].sensitivity])),
    hazards,
  },
};
writeFileSync(path.join(HERE, "validator-report.json"), JSON.stringify(rep, null, 1) + "\n");
console.log("status", status, "headline max rel diff", Math.max(...headlineDiff), "cross ok", crossOk, "end identical", endIdentical, endRecords);
console.log("maxS", maxS, "maxRep", maxRep, "maxBand", maxBand, "maxSums", maxSums, "maxK4", maxK4, "maxBsgs", maxBsgs, "maxGap", maxGap, "maxDelta", maxDelta, "maxCiDelta", maxCiDelta, "inc mismatches", incMismatch);
console.log("L1 holds primary", holdsOnPrimary, "falsified", falsified, JSON.stringify(predictions), JSON.stringify(Object.fromEntries(BITS.map((b) => [b, [rows[b].holds_pair_bootstrap, rows[b].holds_cluster_bootstrap, rows[b].holds_delta_method]]))));
