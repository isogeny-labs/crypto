#!/usr/bin/env python3
"""Writes checks.json: the validator's per-check findings for
research/koblitz_ksum_fold_study_20261115 (numbers read from own-*.json).
Usage: mkchecks.py VALIDATOR_DIR"""
import json, pathlib, sys

V = pathlib.Path(sys.argv[1])
own = json.loads((V / "own-study.json").read_text())
fr = json.loads((V / "own-fresh.json").read_text())
cf = json.loads((V / "own-confirm.json").read_text())
g1 = json.loads((V / "own-n29char.json").read_text())
sc = json.loads((V / "own-scalarcheck.json").read_text())
W = ["0", "0.1", "1"]


def f3(x):
    return f"{x:.3f}"


def rci(d):
    lo, hi = d["ci95_linearised_t"]
    return f"{d['ratio']:.3f} [{lo:.3f}, {hi:.3f}]"


def band(b, arm):
    return "/".join(f3(x) for x in b[arm]["mean_S"])


def row(src, key):
    return ", ".join(rci(src["rows"][w][key]) for w in W)


n23 = own["n23"]; n29 = own["n29"]
scal_total = sum(v["checked"] for k, v in sc["sessions"].items() if not k.startswith(("F", "G")))
scal_ok = sum(v["ok"] for k, v in sc["sessions"].items() if not k.startswith(("F", "G")))
scal_fresh = sum(v["ok"] for k, v in sc["sessions"].items() if k.startswith(("F", "G")))
per_rep_upper_max = max(x["ci95_linearised_t"][1] for w in W for x in n23["rows"][w]["kf3_over_strictest_per_replicate"].values())
fr_ps_upper_max = max(x["ci95_linearised_t"][1] for w in W for x in fr["rows"][w]["kf3_over_registered_per_session"].values())
cf_ps_upper_max = max(x["ci95_linearised_t"][1] for w in W for x in cf["rows"][w]["kf3_over_lazy-c4_per_session"].values())

checks = {
    "check_1_code_audit_71d56d2af0": {
        "result": "pass",
        "scope": "FrobeniusKSumTable / FrobeniusKSumOracle (src/cryptanalysis/ic_framework/plugins.rs 6874-7231 at 71d56d2af0, unchanged to HEAD), the ksum-frobenius dispatch (ecbench/methods.rs solve_ic_binary), compared line by line with FrobeniusPairTable::build_inner / build_layout / probe / resolve_member / work_counters (ic_boundary.rs 12244-12620) and FrobeniusMitmOracle::setup_native / setup_work_native (plugins.rs 3527-3735); band rule from ecbench/band.rs and PRICED_NATIVE (methods.rs 6120).",
        "build": "Every partial and completed sum is a counted g.add into build_ops, merged into the oracle_setup phase (968 adds at n23, 1568 at n29). One canonicalisation and one insert attempt per completed sum (insert attempts -> work_native.table_insert_attempts behind counted:table_insert_attempts, priced by the band: 924 at n23). Orbit pass: f canonicalisations counted as canonicalisations (calibration-priced), its f orbit-index inserts not counted; the lazy table does the mirror image (f inserts in table_insert_attempts, f orbit canonicalisations in work_native.orbit_canonicalisations, unpriced by the band): the same number of priced operations per base point. Orbit-member index: f(n-1) Frobenius maps and f(n-1) lookups charged at set-up (1012 each at n23); the lazy table charges t maps and 1 lookup per summand at a hit instead. FrobeniusCanon::new (normal basis) is work_native only on both oracles (canon_build_word_ops 9854, canon_build_field_sqrs 46 at n23, identical counts on both arms): unpriced by the band on both, symmetric. Uncounted and negligible: the f log f sort of 46 points, is_identity tests, the neg-index exclusion test (the pair table has the same uncounted tests).",
        "probe": "1 canonicalisation + 1 lookup per trial (counted in OracleCounters, calibration-priced, band-priced at w), k-1 = 2 counted additions on a key hit, a mismatch counted (frobfold_mismatches). FrobeniusPairTable::probe: 1 canonicalisation + 1 lookup, 1 addition, and for the lazy table the Frobenius maps and lookup of each summand at a hit. Priced by the same rule.",
        "fold_soundness": "t = (t_u + n - t_r) mod n as FrobeniusPairTable::probe; summands are sigma^t of the stored indices via member_of; a hit is returned only after the k-1 additions give target (summands) or -target (negated summands), so a returned decomposition always sums to the target and no relation is credited unverified. Build completeness: the first summand is the least orbit's representative, others strictly later in orbit-sorted order, so every distinct-abscissa signed 3-sum orbit is reachable (the unit test asserts no decomposition of the unfolded table is lost; not re-run here).",
        "end_to_end": f"Every candidate record: n23 {n23['candidate_records']['verified']}/{n23['candidate_records']['all']} and n29 {n29['candidate_records']['verified']}/{n29['candidate_records']['all']} measured records verified, frobfold_mismatches 0, exactly 1 relation per run, table entries 229 (n23) / 337 (n29) constant; recovered logs re-checked by the validator's own GF(2^n) arithmetic.",
        "column_coefficients": "With columns=1,generator_orbit=1 the only column is G's own signed orbit, coefficients +-lambda^t from compact-orbit-scan (closure checked: the orbit returns to G with coefficient 1) and known log h mod r (known_column_logs). One relation pins log Q; the mapping is exercised end to end by every verified run. Multi-column use (columns >= 2) is exercised only by the commit's unit test, which was not re-run.",
        "hidden_precomputation": "None found. The table is rebuilt in prepare() for every run (charged per run, no amortisation over targets). Shared, uncharged instance data (curve, lambda, normal basis) is common to every arm.",
        "pricing_identity_with_incumbent": "Both arms are plain ic.pipeline records on the same curve with the same pinned calibration units (ns_per_canon, ns_per_frobenius, ns_per_lookup, ns_per_row_op, ...); the band takes C out and charges w (U + n_C) on both; counted table inserts priced on both. Unpriced counters per arm with counts: unpriced-n23.json (none uncounted).",
        "caveat": "The oracle audit is clean, but the registered comparison changes the factor base as well as the oracle (generator_orbit=1 on the candidate only); see check 2.",
    },
    "check_2_incumbent_fairness": {
        "result": "fail",
        "premise": "PROTOCOL.md section 1 names the lazy orbit-member pair table on compact-orbit-scan with stride-renew as 'the strongest existing IC arm'. research/ecbench_generator_orbit_20261022 (frozen 2026-10-22, before this study; VALIDATION.json valid under scripts/validation_gate.py evidence; numbers [pcc:unverified]) measured the orbit-layout pair table with generator_orbit=1 and targets=stride-generator 1.26-3.20x cheaper than that lazy baseline on 7 Koblitz curves, including this study's K1 n29 curve (icv1-f2m29-tm40309-30c52b96). The study neither cites nor runs it.",
        "tuning_asymmetry": "Candidate configuration searched over m {3,4} x columns {1,2} x generator_orbit {0,1} x targets {walk, stride-renew} (PROTOCOL.md section 5); incumbent over columns only, with generator_orbit=0 and stride-renew fixed. The generator-orbit base the candidate uses was never offered to the incumbent.",
        "measured_F1_F3": f"Validator fresh sessions F1-F3 (K1 n23, nonce seeds, 3 x 48 targets x 3 rounds): band mean S (w=0/0.1/1) kf3-c1g-sr {band(fr['band_pooled'], 'kf3-c1g-sr')}, lazy-c4 {band(fr['band_pooled'], 'lazy-c4')}, gen-layout-c3 {band(fr['band_pooled'], 'gen-layout-c3')}, gen-layout-c4 {band(fr['band_pooled'], 'gen-layout-c4')}, gen-layout-c5 {band(fr['band_pooled'], 'gen-layout-c5')}, gen-layout-c4-sr {band(fr['band_pooled'], 'gen-layout-c4-sr')}, gen-lazy-c4-sr {band(fr['band_pooled'], 'gen-lazy-c4-sr')}, layout-c4-sr {band(fr['band_pooled'], 'layout-c4-sr')}. Cheapest existing IC arm at every w: gen-layout-c4. kf3 / gen-layout-c4: {row(fr, 'kf3_over_gen-layout-c4')}.",
        "confirmation_F4_F6_predeclared": f"Pre-declared before running (confirm-predeclared.txt), no reselection, new nonce seeds: kf3 / gen-layout-c4 = {row(cf, 'kf3_over_gen-layout-c4')} at w = 0 / 0.1 / 1: null at w = 0, regression at w = 0.1 and 1. gen-layout-c4 / lazy-c4 = {row(cf, 'gen-layout-c4_over_lazy-c4')}.",
        "confound": f"Same lazy oracle as the registered incumbent, with the candidate's generator-orbit base family and the candidate's stride-renew targets (gen-lazy-c4-sr): kf3 / gen-lazy-c4-sr = {row(fr, 'kf3_over_gen-lazy-c4-sr')} (F1-F3) and {row(cf, 'kf3_over_gen-lazy-c4-sr')} (F4-F6). The oracle swap is a null within about 4 %; the registered 0.78-0.83x is the generator-orbit factor base (one known column, no lift/projection, one relation), not the folded k-sum.",
        "winners_curse": "None in the registered statistic: candidate and incumbent columns were sized on disjoint pilot seeds and fresh nonce seeds reproduce it (check 5). Choosing the strictest of lazy-c3/c4 on study data favours the incumbent, as the protocol says. The validator selected gen-layout-c4 on F1-F3 and confirmed on F4-F6 to keep its own selection out of the test.",
        "answer": "lazy-c4 is not the strongest existing IC configuration on K1 n23, not even among lazy-oracle configurations (gen-lazy-c4-sr is cheaper); the incumbent was not tuned on the axes the candidate was; the claim 'measured improvement over the strongest existing IC arm on this curve' does not hold.",
    },
    "check_3_replays": {
        "result": "pass",
        "detail": f"ecbench verify --replay-all on all six sessions: ok, 2112 records each, 1584 replays each reproduced (9504 in total), 0 problems, every integrity check true; the n23 *_full error records replay as the same error. Binary: target/release/ecbench sha256 6c60b4c9...4d77 at validation = registered-at.txt (committed in the freeze commit 9fa38942e4 before any session) = binary_sha256 of all six session.json. records.jsonl sha256 = sessions.sha256 (6/6); session spec.json = frozen specs/ (canonical JSON, 6/6); REGISTRATION.sha256 OK (9/9). Independent scalar check (scalarcheck.py, own GF(2^n), polynomials 0x800021 / 0x20000005 found from G): {scal_ok}/{scal_total} verified study records' logs correct (48 distinct targets per session), {scal_fresh} fresh-session records correct.",
    },
    "check_4_independent_recompute": {
        "result": "pass",
        "detail": f"Every claimed point estimate in results.json (mean S per arm and w, H1 pooled and per replicate, R vs each incumbent, A/A, H2, the H1 walk variant, every H3 gap to rho-strong, the n29 H4 recorded-S ratios) recomputed from the raw records and agrees within 1e-9 (validator-report.json headline). n23 R pooled {', '.join(f3(n23['rows'][w]['kf3_over_strictest_pooled']['ratio']) for w in W)}; kf3 / rho-strong {', '.join(f3(n23['rows'][w]['kf3-c1g-sr_over_rho-strong']['ratio']) for w in W)}. Intervals: the producer's are ecbench compare's two-stage cluster bootstrap (seed 20261002), reproduced exactly by running ecbench compare (pooled and per replicate); the validator's own linearised-t intervals (identical to ecbench compare's primary ci95) are n23 pooled {row(own['n23'], 'kf3_over_strictest_pooled')}, per-replicate upper ends at most {per_rep_upper_max:.3f}. H1(a) and (b) hold under both interval readings; H1(c) is vacuous (A/A exactly 1.000).",
    },
    "check_5_fresh_sessions": {
        "result": "pass",
        "detail": f"Registered statistic on nonce-derived seeds (nonce.txt, drawn 2026-11-15T22:41:48Z, after the results commit): kf3-c1g-sr / lazy-c4 = {row(fr, 'kf3_over_registered_strictest')} (F1-F3; lazy-c4 strictest of the registered pair at every w, per-session upper end at most {fr_ps_upper_max:.3f}) and {row(cf, 'kf3_over_lazy-c4')} (F4-F6, per-session upper at most {cf_ps_upper_max:.3f}). kf3 / rho-strong = {row(fr, 'kf3_over_rho-strong')} (F1-F3), {row(cf, 'kf3_over_rho-strong')} (F4-F6). Band means reproduce: lazy-c3 {band(fr['band_pooled'], 'lazy-c3')}, lazy-c4 {band(fr['band_pooled'], 'lazy-c4')}, kf3 {band(fr['band_pooled'], 'kf3-c1g-sr')}, rho-strong {band(fr['band_pooled'], 'rho-strong')}. Every fresh session: all records verified, verify --replay-all ok, 0 problems.",
    },
    "check_6_n29_regression": {
        "result": "characterised: genuine and structural; no cost-model defect found in either arm",
        "mechanism": "The folded 3-sum table over G's 58-point signed orbit has 337 entries covering about 337 x 58 = 19.5k of r = 42,457 points (46 %). Building it costs 1568 additions (1751 GAE, 8.5 sqrt r) and the run then needs 1.9-2.2 trials. The table size is fixed by 2n and m (there is no size parameter), so at log2 r = 15.4 it overshoots; at n23 the same construction (229 entries, 0.25 % coverage, 968 additions = 0.47 sqrt r) is balanced against about 410 trials. The protocol's own hit model (entries x 2n / r) predicts 398 trials at n23 (measured 405-418) and 2.2 at n29 (measured 1.9-2.2).",
        "incumbent_pricing": "lazy-c1: 26 pair entries, 55.6 trials; the factor base (65 GAE) includes the cofactor-12646 projections charged as group operations; every unpriced counter carries a count and band means are finite at every w. No defect found.",
        "lazy_vs_rho_strong": f"lazy-c1 / rho-strong = 0.344 at w = 0 is reproduced (G1: {rci(g1['rows']['0']['lazy-c1_over_rho-strong'])}), but it is a reference artefact: rho-strong's 32-lane set-up is 738 of its about 943 GAE at r = 42,457. In G1 rho.signed_frobenius_v5 / rho-strong = {row(g1, 'rho-frob-v5_over_rho-strong')} and lazy-c1 / rho-frob-v5 = {row(g1, 'lazy-c1_over_rho-frob-v5')}.",
        "against_strongest_ic": f"G1 band mean S: kf3 {band(g1['band'], 'kf3-c1g-sr')}, lazy-c1 {band(g1['band'], 'lazy-c1')}, gen-layout-c1 {band(g1['band'], 'gen-layout-c1')}, gen-lazy-c1-sr {band(g1['band'], 'gen-lazy-c1-sr')}; kf3 / gen-layout-c1 = {row(g1, 'kf3-c1g-sr_over_gen-layout-c1')}.",
        "consequence_for_n23": "No pricing defect in the incumbent; the n23 comparison is not invalidated by pricing, only by the choice of incumbent (check 2).",
    },
    "check_7_protocol_adherence": {
        "result": "pass with findings (no deviation moves a registered count)",
        "disclosed_and_verified": [
            "L0 local route with --cpus none --allow-busy instead of isolab-eks: counts are deterministic and every replay reproduces; wall times not used.",
            "*_full arms error on n23 (full_prices.json has no row for icv1-f2m23-tm5197-1f85e9e1): 576 error records per session, replays reproduce the same error; the primary tests use no *_full arm; H4 exists for n29 only and recomputes.",
            "run.sh, analyze.py, make_report.py and figures written after the freeze: the registered quantities recompute from raw records without them.",
            "deterministic A/A (rho-strong-aa = rho-strong exactly): criterion H1(c) adds nothing.",
            "verify has no --exit-code flag (it exits non-zero on a problem by default).",
            "first run.sh launch killed before any session finished: only the README's statement; no partial output is in the record to check.",
            "order alternate with 3 measured rounds is unbalanced: counts cannot depend on order (replays identical).",
        ],
        "undisclosed": [
            "H0 requires lower_bound = false, which plain methods with unpriced counters cannot meet; the README reinterprets it. Every unpriced counter carries a count, so the band is well defined.",
            "Sessions record git_dirty = true (n23-r1..r3, n29-r3) and the head moved during the run (9fa38942e4 -> b7666dcd8f -> c9557817b3, commits by a concurrent session); the binary sha256 is constant across all six, so no effect on counts.",
            "records.jsonl.gz (1.6-1.7 MB each) committed to Git directly, above the artifact policy's 64 KiB data-file threshold (AGENTS.md section 13).",
            "The protocol's premise about the strongest IC arm and its rho reference were not checked against existing in-repo results (checks 2 and rho_reference).",
        ],
    },
    "rho_reference": {
        "result": "fail for the 'distance to rho' as stated",
        "protocol_named": "rho.signed_frobenius_strong (signed Frobenius, A = 2n = 46, the strongest symmetry; same binary, same unit, same targets and seeds): matches the protocol.",
        "not_strongest_implementation": f"In the same binary on the same targets (F1-F3 band mean S w=0/0.1/1): rho-strong {band(fr['band_pooled'], 'rho-strong')}, rho.signed_frobenius_v5 {band(fr['band_pooled'], 'rho-frob-v5')}, _v4 {band(fr['band_pooled'], 'rho-frob-v4')}, _v3 {band(fr['band_pooled'], 'rho-frob-v3')}, _strong_v2 {band(fr['band_pooled'], 'rho-strong-v2')}, plain _signed_frobenius {band(fr['band_pooled'], 'rho-frob')}, _strong_escape {band(fr['band_pooled'], 'rho-strong-escape')}. rho-frob-v5 / rho-strong = {row(cf, 'rho-frob-v5_over_rho-strong')} (F4-F6). rho-frob-v5 runs at 1.2x the automorphism floor (0.185); rho-strong at 4.8x, its 32-lane set-up 1056 GAE = 0.52 sqrt r. rho-frob-v5 records count steps, start additions, set-up, verification, canonicalisations and DP table operations (accounting v2).",
        "gap_against_cheapest_rho": f"kf3-c1g-sr / rho.signed_frobenius_v5 = {row(fr, 'kf3_over_rho-frob-v5')} (F1-F3), {row(cf, 'kf3_over_rho-frob-v5')} (F4-F6). The claimed gap 0.883 / 1.105 / 2.262 and 'unresolved inside the band' against rho are artefacts of rho-strong's fixed set-up at log2 r = 22 (PROTOCOL.md section 8 says the gap is 'partly a set-up artefact'; measured, it is a factor of about 4).",
        "gate_consequence": "Under the gate's descriptor the kind (negation_frobenius) is the strongest symmetry, but a row stating the gap to rho would understate the distance by about 4x. Not admitted (check 2 fails regardless).",
    },
    "heuristic_and_cost_model": {
        "class": "engineering constant at two sizes, not a heuristic-validation experiment; no extrapolation is claimed, so the inventor-protocol section 6 ladder does not apply (the measured n23 constant is the claim; the predicted negative case, n29, is reported).",
        "prediction": "PROTOCOL.md section 2: about F^3/(6 x 2n) entries and hit probability proportional to entries / r. F = 46: 353 entries predicted, 229 measured (the distinct-abscissa rule and key coincidences); with measured entries the hit model gives 398 expected trials (measured 405-418) at n23 and 2.2 at n29 (measured 1.9-2.2).",
        "unit": "ecbench.gae, S = GAE / sqrt r, every unpriced native operation priced at w = 0, 0.1, 1 (AGENTS.md 5a), table inserts priced; unpriced counters listed per arm (unpriced-n23.json).",
        "memory": "Not reported beside cost in the README (minor). From the records: 229 k-sum entries vs 356 pair entries (lazy-c4) at n23; max_rss_kib is in every record.",
        "expected_cost": "Band means are empirical totals over runs to a verified log, so per-trial cost x 1/p is measured, not assumed.",
        "null_control": "The A/A arm is deterministic (exactly 1) and tests nothing beyond determinism. The relevant null, giving the incumbent the generator-orbit base, removes the effect (check 2, confound); the report does not address it.",
    },
}

defects = [
    {"id": "D1", "severity": "major (invalidates the improvement claim)", "what": "The incumbent is not the strongest existing IC arm on K1 n23, and the comparison is confounded: the candidate has generator_orbit=1, the incumbent does not. The existing gen-layout arm (research/ecbench_generator_orbit_20261022) is not beaten (pre-declared confirmation: null at w = 0, regression at w = 0.1 and 1), and the lazy oracle on the candidate's base matches the candidate (null at every w)."},
    {"id": "D2", "severity": "major for the stated distance to rho", "what": "rho.signed_frobenius_strong is about 4x the cost of rho.signed_frobenius_v5 (and 2.8-3.5x rho.signed_frobenius_strong_v2) in the same binary on K1 n23; the candidate is 3.5 / 4.2 / 7.4x the cheapest signed-Frobenius rho, not 0.88 / 1.10 / 2.26."},
    {"id": "D3", "severity": "minor", "what": "H0's lower_bound = false is unattainable for plain methods; the deterministic A/A makes H1(c) vacuous."},
    {"id": "D4", "severity": "minor (hygiene)", "what": "Raw records committed to Git above the artifact threshold; sessions ran with a dirty tree and a moving head (binary constant)."},
    {"id": "D5", "severity": "minor", "what": "No memory column beside the cost tables."},
]

limitations = [
    "Replays used the producer's binary, identified by sha256 against the pre-registration record; not rebuilt by the validator (about 2.1-2.4 GB free against a 3.8 GB release target directory).",
    "The unit tests of 71d56d2af0 (folded_ksum_agrees_with_the_unfolded_table, ksum_frobenius_recovers_planted_logs) were not re-run, for the same disk reason; soundness is established end to end on every run and by code reading.",
    "Same host/container class as the producer (L0); counts are deterministic, so this does not affect any compared quantity.",
    "The validator is a subagent launched from the producing session (same Claude-Session id): independence of context, code and seeds, not of operator.",
    "The generator-orbit incumbents were tuned by the validator only over columns 2-5 with stride-generator (plus c4 with stride-renew); a wider tuning could only strengthen D1.",
    "rho.signed_frobenius_v5's accounting was checked at record level (counters), not at source level.",
    "The n29 characterisation rests on one validator session (G1, 48 targets x 3 rounds) plus the study's three.",
]

summary = ("failed. The registered H1 statistic replays (6/6 sessions, 9504 replays identical), recomputes to 1e-9 and reproduces on "
           f"fresh nonce seeds (kf3-c1g-sr / lazy-c4 = {', '.join(f3(fr['rows'][w]['kf3_over_registered_strictest']['ratio']) for w in W)} "
           f"and {', '.join(f3(cf['rows'][w]['kf3_over_lazy-c4']['ratio']) for w in W)} at w = 0 / 0.1 / 1), and the oracle's accounting is sound and priced like the pair table's. "
           "But the claim it supports, a measured improvement over the strongest existing IC arm on K1 n23, fails: the existing generator-orbit "
           f"layout arm is not beaten (pre-declared confirmation kf3 / gen-layout-c4 = {row(cf, 'kf3_over_gen-layout-c4')}), and the registered lazy oracle on the candidate's "
           f"own generator-orbit base matches the candidate ({row(cf, 'kf3_over_gen-lazy-c4-sr')}): the gain over lazy-c4 is the factor base, not the folded k-sum. "
           f"The stated distance to rho uses a reference about 4x costlier than rho.signed_frobenius_v5 in the same binary (kf3 / rho-frob-v5 = {row(cf, 'kf3_over_rho-frob-v5')}). "
           "The n29 regression is genuine (an oversized fixed table at log2 r = 15.4), with no pricing defect. No benchmarks evidence directory is written.")

(V / "checks.json").write_text(json.dumps({"verdict_summary": summary, "checks": checks, "defects": defects,
                                           "limitations": limitations}, indent=1) + "\n")
print(summary)
