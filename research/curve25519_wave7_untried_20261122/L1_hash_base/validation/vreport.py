#!/usr/bin/env python3
"""Assemble validator-report.json for wave-7 L1 from the validator's own result files."""
import json, sys
L1 = '/home/user/crypto/research/curve25519_wave7_untried_20261122/L1_hash_base'
V = json.load(open('vdata.json')); H = json.load(open('vdata_holdout.json')); F = json.load(open('vfacts.json'))
C = json.load(open('vcompare.json')); R16 = json.load(open('vreplay_k16.json'))
SP = {k: json.load(open(f'vspot2_k{k}.json')) for k in (18, 20, 22, 24)}


def rnd(x, d=4):
    if isinstance(x, float):
        return round(x, d)
    if isinstance(x, list):
        return [rnd(y, d) for y in x]
    if isinstance(x, dict):
        return {k: rnd(v, d) for k, v in x.items()}
    return x


pooled = {k: dict(mc=rnd(v['mc'], 5), mc_ci_validator_bootstrap=rnd(v['mc_ci']), exact=rnd(v['exact'], 5),
                  exact_ci_validator_bootstrap=rnd(v['exact_ci'])) for k, v in V['pooled'].items()}
per_cell = []
for c in V['cells']:
    per_cell.append(dict(k=c['k'], N=c['N'],
                         ratios={nm: dict(mc=rnd(r['mc'], 5), ci=rnd(r['mc_ci']), exact=rnd(r['exact'], 5), half_width=rnd(r['half_width']))
                                 for nm, r in c['ratios'].items()},
                         pct_bits_lo0=rnd(c['pct_bits_lo0'], 2), pct_hash_lit=rnd(c['pct_hash_lit'], 2),
                         ceiling_rate_2N2_over_rm1=rnd(c['ceiling_rate'], 6),
                         proj_over_ceiling=rnd(c['proj_over_ceiling']), rand_over_ceiling=rnd(c['unproj_over_ceiling']['rand']),
                         max_base_over_own_ceiling_all_arms=rnd(max(c['max_base_over_ceiling'].values())),
                         plant_design_meanNp2_over_N2=rnd(c['design'], 5)))
replay16 = {}
for N, c in R16['cells'].items():
    replay16[N] = dict(bases_rebuilt=c['checked_bases'], arms_per_base=9, exact_mismatches=c['exact_mismatch'], N_mismatches=c['N_mismatch'],
                       t0_exact_mismatches=c['t0_mismatch'], theta_b0_validator=c['theta_b0_mine'], theta_b0_recorded=c['theta_b0_recorded'],
                       hash_set_size_at_theta=c['hash_V_size_at_theta'], hash_windows_disjoint=c['window_disjoint'],
                       hash_proj_bases_borrowing_from_next_window=c['hash_proj_overlap_bases'],
                       own_target_hits_verified=c['verified_hits'], own_target_verify_failures=c['verify_fail'],
                       replay_ratios_own_targets=rnd(c['replay_ratios']),
                       mc_rates={a: dict(validator=rnd(v['mine_rate'], 6), recorded_512=rnd(v['recorded_rate_all512'], 6),
                                         z_vs_recorded_512=rnd(v['z_vs_recorded_all512'], 2), z_vs_recorded_same_bases=rnd(v['z_vs_recorded_same_bases'], 2),
                                         z_vs_own_exact=rnd(v['z_mine_vs_own_exact'], 2)) for a, v in c['mc_compare'].items()})
spot = {}
for k, R in SP.items():
    for N, c in R['cells'].items():
        spot[f'k{k}_N{N}'] = dict(base_indices=f"bits_lo0 and b = 0..{c['checked_bases'] - 2}", arms_per_base=9,
                                  exact_mismatches=c['exact_mismatch'], N_mismatches=c['N_mismatch'], t0_exact_mismatches=c['t0_mismatch'],
                                  theta_b0_equal=c['theta_b0_mine'] == c['theta_b0_recorded'], theta_b0=c['theta_b0_mine'],
                                  hash_set_size_at_theta=c['hash_V_size_at_theta'], hash_windows_disjoint_all_512=c['window_disjoint'],
                                  hash_proj_bases_borrowing_from_next_window=c['hash_proj_overlap_bases'])
pred = {}
for k, v in V['predictions'].items():
    pred[k] = dict(holds=v['holds'], **{kk: rnd(vv) for kk, vv in v.items() if kk != 'holds'})
hold = dict(description='bases 0..299 of both k = 16 cells dropped (these rows equal the pre-freeze smoke rows); other cells unchanged',
            predictions_hold={k: v['holds'] for k, v in H['predictions'].items()},
            P1_cells_ci_contain_1=H['predictions']['P1']['cells_ci_contain_1'],
            pooled={k: dict(mc=rnd(v['mc']), ci=rnd(v['mc_ci'])) for k, v in H['pooled'].items()})
cost = [dict(k=c['k'], N=c['N'], table_group_ops_N2=c['G'], hash_evals_p=c['p'], hash_evals_per_column=round(c['per_col'], 1),
             hash_over_rand_w0=rnd(c['ratio_w0.0'], 4), hash_over_rand_w0_1=rnd(c['ratio_w0.1'], 3), hash_over_rand_w1=rnd(c['ratio_w1.0'], 2),
             with_shared_2N_native_priced_w0_1=rnd(c['ratio_w0.1_with_shared_native'], 3), with_shared_2N_native_priced_w1=rnd(c['ratio_w1.0_with_shared_native'], 2),
             cost_per_relation_ratio_measured_rates={'w0': rnd(c['cost_per_rel_ratio_w0.0']), 'w0.1': rnd(c['cost_per_rel_ratio_w0.1'], 3), 'w1': rnd(c['cost_per_rel_ratio_w1.0'], 2)},
             instrument_hashed_valid_u_only=c['valid_hashed']) for c in V['cost']]

checks = [
 dict(name='binding: validated commit and tree state', result='pass',
      detail='Validated the L1 files as of 190daab37583c88cca0b8bea595e081140a12af9 (last commit touching L1_hash_base; HEAD moved to 1e42afc88d during validation through an L3-only commit; `git diff 190daab375 HEAD -- L1_hash_base` empty). `git status` for L1_hash_base clean; the only untracked file is the ignored out/cell_k24_N579.npz, bound by out/SHA256SUMS. Freeze commit 91f2f2bcde is an ancestor of HEAD.'),
 dict(name='(7) frozen files byte-identical between 91f2f2bcde, the run and the write-up', result='pass',
      detail='sha256 of l1lib.py e1c09a5b..., run_l1.py 202f8123..., analyse_l1.py 2b402f39..., PROTOCOL.md 4ab666fb... identical at 91f2f2bcde, e783d12a5b, 190daab375 and in the worktree; `sha256sum -c FREEZE.sha256` OK for all four. No commit between 91f2f2bcde and 190daab375 modifies them (git diff-tree per commit). Freeze commit 03:10:00Z precedes run.log launch 03:10:04Z ("commit 91f2f2bcde; FREEZE.sha256 verified") and the first data commit 376b26f92a (03:10:22Z). The pre-freeze commit 9a45d456d4 already held byte-identical analyse_l1.py, run_l1.py and l1lib.py.'),
 dict(name='data integrity: npz, JSON mirrors, in-history blobs, run.log', result='pass',
      detail='10/10 out/*.npz match out/SHA256SUMS; 10/10 out_json/*.json match out_json/SHA256SUMS; every array of every JSON mirror equals the npz exactly (dtype, shape, values; validator code, not export_json.py). The 9 npz committed during the run (376b26f92a..969a857783) hash in Git history to the same values as on disk and in SHA256SUMS, so no cell was altered after its first commit. The 10 JSON meta lines in run.log equal the npz meta exactly; smoke=false in all.'),
 dict(name='(1) curve facts by an independent method', result='pass',
      detail='#E from the Euler-criterion character sum (vectorised square-and-multiply, not the producer\'s squares table) and, independently, from an x-only Montgomery-ladder point of exact order 8r plus Hasse uniqueness (4 sqrt p < r); twist order 2p+2-#E with a twist point of order divisible by r\'; r and r\' prime by deterministic Miller-Rabin. All five: #E = 8r with r = 8209, 32887, 131203, 524731, 2098009; twist 4r\' with r\' = 16301, 65293, 261881, 1047689, 4192553; E(F_p) cyclic (point of order 8r); p = 5 mod 8; A^2-4 non-square; r > 2^(k-3); valid columns 4r-4 = 32832, 131544, 524808, 2098920, 8392032 equal run.log; N = 18, 36, 36, 73, 72, 145, 145, 290, 290, 579. Agrees with curve_facts.json and PROTOCOL section 3.'),
 dict(name='structure of the raw counts', result='pass',
      detail='Every cell: 512 rows per arm, 1 for bits_lo0; N_actual = N (plant arms in {floor(fN), floor(fN)+1}); m = m_u = ceil(800r/N^2) for unprojected, m_p = ceil(100(r-1)/N^2) for projected arms; T0 columns only on rand and >= the blind columns; exact counts even (negation-symmetric); verified_ok equals counted hits (+ T0-only hits) and verified_fail = 0 everywhere; instrument_check_failures = 0.'),
 dict(name='(2) every pooled and per-cell ratio recomputed', result='pass',
      detail='All 10 comparisons x 10 cells and the 10 pooled values, MC and exact: point estimates identical to analysis/summary.json (max abs difference 5.6e-17). Validator bootstrap (4000 resamples, own seed, independent per arm, paired for T0) gives CI endpoints within 0.13 % (relative) of the producer\'s; ratio-estimator delta-method intervals agree. Borderline cell CIs keep their side of 1 (k18 N36 hash/rand lower 1.0001 vs 1.0004; k20 N72 hash/hash2 lower 0.9998 vs 0.9999; k24 N290 hash/hash2 upper 0.9987 vs 0.9991).'),
 dict(name='(2) prediction verdicts P1-P8 recomputed', result='pass',
      detail='All ten hold under validator code with the same counts: P1 9/10 cell CIs contain 1; P1p 10/10 half-widths <= 0.03; P2 both pooled in band; P2b percentiles identical; P3 0 ceiling violations, projected 0.8563-0.9663 of ceiling, rand_proj/rand exact 6.867-7.700 and MC 6.908-7.713, hash_proj/rand_proj pooled 1.0006; P4 exact 1.956-2.004, 10/10 exclude 1; P5 design 1.03087/1.05058, pooled 1.0299/1.0486, 10/10 and 10/10 exclude 1; P6 9/10 and 9/10; P7 0 failures, worst z -2.54 (k24 N290 plant3); P8 holds by construction.'),
 dict(name='(3) exact covered-point ratios and the ceiling 2N^2/(r-1)', result='pass',
      detail='Ceiling re-derived: signed pair sums with i <= j number 4 C(N,2) + 2N = 2N^2, targets uniform on E[r]\\{O}, so P(hit) <= 2N^2/(r-1). Exact coverage checked against it for every base of every arm (including rand2, bits_lo0, plant arms at their own N\', projected arms): 0 violations; the largest projected base reaches exactly 1.0 of its ceiling (k16 N18, no collision, allowed by <=), the largest unprojected base 0.256. Cross-check: unprojected E[#sums in E[r]] = N(N+1)/4 from the coset distribution (1/8, 1/8, 1/4, 1/4, 1/4 for {0},{4},{+-1},{+-2},{+-3}); recorded rand means are 0.980-0.997 of it (birthday loss about N(N+1)/(8(r-1))); projected arms sit 0.0002-0.008 above the random-collision model (1-e^-x)/x, x = 2N^2/(r-1), the excess shrinking with N as expected from sums that share an index (they cannot coincide in one orientation).'),
 dict(name='(4) independent end-to-end replay, k = 16, both sizes', result='pass',
      detail='Own Euler mask, Tonelli-Shanks square roots, affine arithmetic, own generator (u = 2) and own discrete-log table; every signed pair sum computed by explicit addition; bases rebuilt from the PROTOCOL rules (seed rule, hash windows, bits offset, rand choice, plant N\', projection). All 512 bases of all 9 arms plus bits_lo0 in both cells (9,218 bases): exact coverage, N_actual and T0 coverage equal the recorded columns exactly (0 mismatches). MC with own targets (own seed, own generator, m_u/m_p per base): 2,073,673 hits, each re-verified by explicit arithmetic, 0 failures; rates agree with the recorded rates (|z| <= 1.30 against the 512-base record, <= 1.47 against the same bases, <= 2.10 against own exact).'),
 dict(name='(4b) exact-coverage spot check in the other eight cells', result='pass',
      detail='Coset method (no log table): t_i from [r]P_i matched against [t]Q, Q = [r]G, only coset-cancelling signed sums added explicitly. bits_lo0 and bases 0..31 (k = 18, 20, 22) or 0..15 (k = 24) for all 9 arms: exact coverage, N_actual and T0 coverage equal the recorded columns exactly in all eight cells (2,024 bases, 0 mismatches).'),
 dict(name='(5) hash base construction rule', result='pass',
      detail='Key bytes from the l1lib.py source text (PROTOCOL section 4 points to HASH_KEYS); H(u) = SHA-256(key || u as 4 bytes LE)[:8] big-endian over the valid u (E[8] excluded). Validator theta_b0 equals run.log in all 10 cells; |{u valid: H(u) <= theta_b0}| = N exactly and |{H(u) < H_(N+1)}| = N (no 64-bit ties for either key on any curve). Windows [bN, (b+1)N) for b < 512 are pairwise disjoint for hash and hash2, and the exact-coverage matches above confirm these windows produced the data. hash_proj borrows from window b+1 when projected images collide (erratum E7).'),
 dict(name='(6) cost table at w = 0, 0.1, 1 and ranking language', result='pass_with_errata',
      detail='All 60 table entries (N^2, p, p/N, ratios at w = 0, 0.1, 1) recomputed and equal to README after rounding; ranges p/N 1,818-57,852, w = 0.1 5.92-21.23, w = 1 50.19-203.27. The order does not flip inside the band (tie at w = 0, hash costlier at 0.1 and 1), and README claims no strict ranking, consistent with section 5a. Shared native work is excluded rather than priced (erratum E5); pricing it changes no order.'),
 dict(name='(7) k = 24 member choice disclosed', result='pass',
      detail='PROTOCOL section 3 and 13 and README Disclosures state p = 16777141, A = 7842 (brief\'s faithful member, twist 4*4192553) instead of the frame-search smooth-family member p = 16777213, A = 318; matches BRIEF.md.'),
 dict(name='(7) PROTOCOL section 9 hash-evaluation range discrepancy', result='pass',
      detail='Disclosed in README: section 9 says "3,600 to 29,000" per column; computed p/N is 1,818 to 57,852 (the quoted range matches only the c = 0.2 cells at k = 16-22). The frozen text was not edited (hash unchanged). The adjacent design-ratio range is also slightly off and not disclosed (erratum E8).'),
 dict(name='(8) README numbers against data and recomputation', result='pass_with_errata',
      detail='260 automated comparisons (pooled table, both per-cell tables, cost table) pass; prose numbers (1.0013, [0.9978, 1.0048], 1.0026, 1.0299 [1.0263, 1.0337] vs design 1.0309, 1.0486 vs 1.0506, 1.9767 [1.9733, 1.9803], 0.9984, 1.0018, 1.0006, 0.856-0.966, 7.25x, P1p half-widths, P2b percentiles, P4 1.956-2.004, worst z -2.54, hit totals 87,601-112,645 per arm and cell, "about 100,000") all supported. Errata: commit label (E1), undisclosed smoke overlap (E2), inverted citation (E3), unlabelled exact range (E4), artifact disclosure (E6), hash_proj disjointness (E7). REPORT.typ carries the same numbers.'),
 dict(name='(8) [pcc:unverified] on result sentences', result='pass_with_note',
      detail='`python3 tools/pcc/pcc.py lint-prose README.md`: 0 findings. Every result paragraph or bullet ends with the tag, but the tag is per item rather than per sentence and tables are untagged (erratum E9, note).'),
 dict(name='(8) derived vs measured labelled; distance to rho imported only; no relabelling', result='pass',
      detail='README "Measured versus derived" separates measured hit rates and exact counts from the derived ceiling, design ratios and cost table. Distance to rho is only imported: 7.8x rho at k = 20 and 2.6x (w = 0) to 1.4e5x (w = 0.1) for the algebraic arms are in curve25519_frame_search_20261103/README.md lines 107-110; the T0 range 1.944-2.016 is in curve25519_t0_symmetry_20261102/README.md line 42. The class line says it is not a leaderboard row and changes no order; no rho or BSGS arm is labelled index calculus; no improvement is claimed.'),
 dict(name='preregistration hygiene: smoke versus data', result='finding',
      detail='The smoke B = 300 rows (generated before the freeze) are byte-identical to bases 0-299 of both k = 16 data cells (erratum E2). The thresholds printed by the smoke-time analysis equal the frozen ones, and all verdicts hold on the held-out rows.'),
 dict(name='controls and reference arms', result='pass',
      detail='Positive controls present and detected: T0 key (about 2x, exact 1.956-2.004) and planted 3 % / 5 % (recovered 1.0299 / 1.0486 against designs 1.0309 / 1.0506). A/A arms rand2 and hash2 inside +-1.5 %. Reference population rand and the cheaper interval base bits_rand both measured; projected ceiling arm present. No cross-method comparison is made, so no rho arm or external fastest implementation is required (distance to rho imported only).'),
 dict(name='certificate discipline', result='pass',
      detail='No discrete-log solve, relation for linear algebra or closure is claimed. Each counted hit carries a real-arithmetic verification (verified_ok = hits, verified_fail = 0). The validator independently re-verified 2,073,673 own-target hits at k = 16 with its own arithmetic.'),
 dict(name='section 5a canonical band (lesson L-546d37ca7750)', result='not_applicable',
      detail='No ecbench session and no rho or BSGS arm; the only band is the per-hash-evaluation price, recomputed directly above.'),
 dict(name='artifact policy', result='pass',
      detail='The tracked npz are 27-30 KB, under the 64 KiB data-suffix threshold; JSON mirrors are 140-160 KB, under 1 MiB, and .json is not data-suffixed; PNG 24 KB. No policy violation; disclosure inaccuracy recorded as E6.'),
]

errata = [
 dict(id='E1', severity='minor',
      text='README header "Run commit: `e783d12a5b`" mislabels the run. run.log line 1 records the launch from frozen commit 91f2f2bcde at 2026-11-22T03:10:04Z. Nine cell npz were committed progressively (376b26f92a to 969a857783) and cell_k24_N579.npz was never committed; e783d12a5b added analysis/ and the final run.log, and the write-up is 190daab375. Neither README nor REPORT.typ names 91f2f2bcde.',
      suggested_fix='Replace with: "Protocol and code frozen at 91f2f2bcde (FREEZE.sha256); run launched from it at 03:10:04Z; outputs and frozen analysis committed at e783d12a5b; write-up 190daab375." Put it in the README Errata section or a sibling ERRATA.md (this report pins README sha256 231f143d...).'),
 dict(id='E2', severity='moderate',
      text='"Frozen before any data" is not literally true for the two k = 16 cells. The seed rule takes neither B nor a smoke/run role as input. As a result, smoke/out_pipeline/cell_k16_N18.npz and cell_k16_N36.npz (B = 300, smoke = true, generated about 03:07 and committed in 9a45d456d4 at 03:09:10Z, before the freeze commit 91f2f2bcde at 03:10:00Z) are byte-identical in every arm to bases 0-299 of the 512 in out/cell_k16_N18.npz and out/cell_k16_N36.npz. smoke/out/cell_k16_N18.npz (B = 40) is likewise identical to bases 0-39. PROTOCOL section 7 quotes results from these rows (CV, plant3/rand 1.034, plant5/rand 1.044). README does not disclose this. Mitigation checked by the validator: analyse_l1.py, run_l1.py and l1lib.py at 9a45d456d4 equal the frozen files. The smoke-time RESULTS.md prints the same thresholds. The only pre-freeze design change was HITS_PER_BASE 50 to 200 (2d683358c2 to 9a45d456d4). All P1-P8 verdicts still hold with bases 0-299 of both k = 16 cells dropped (hash/rand pooled 1.0026 [0.9982, 1.0071]); P1 then has exactly the threshold, 8 of 10 cell CIs containing 1.',
      suggested_fix='Disclose the overlap and the held-out result in an erratum (README Errata section or ERRATA.md). In later protocols, add a "smoke" role to the seed parts so smoke draws can never coincide with data draws.'),
 dict(id='E3', severity='minor',
      text='Citation direction and quantity are wrong. README Mechanism (and frozen PROTOCOL section 2) say lead_sweep_20261110/LEAD-EXO-coordinate-revealing-oracles found "coordinate/random 0.998 at 2^20 but with per-size ratios 0.97 to 1.75". The source RESULT.md line 13 reports "Negative control randfb/coord at largest size = 0.998 ... Per size: 0.97, 1.04, 1.75, 1.38, 1.00", and that lead\'s PROTOCOL.md defines it as a ratio of counted additions (cost to collect relations), not of relation yield. Coordinate/random would be 1.002 at 2^20 (0.57 to 1.03 per size). The point being made (that study was not powered for 3 %) is unaffected.',
      suggested_fix='"found a random-base/coordinate-base addition-count ratio of 0.998 at 2^20 (per size 0.97 to 1.75, 9 to 101 relations)".'),
 dict(id='E4', severity='minor',
      text='The P3 line "rand_proj/rand is 6.87 to 7.70 in every cell" quotes the exact-coverage ratio (validator: 6.867 to 7.700). The per-cell table above lists the MC ratio (6.908 to 7.713) under the same name with no label. Both ranges lie inside the frozen [6.0, 8.4].',
      suggested_fix='"rand_proj/rand exact 6.87 to 7.70 (MC 6.91 to 7.71) in every cell".'),
 dict(id='E5', severity='minor',
      text='The cost table prices the hash evaluations at w but excludes the shared native work as "matched across arms": about 2N Legendre or square-root tests per base on every arm, plus the T0 extra inversions. Section 5a asks that every arm\'s unpriced counter be listed with its count and priced at the band. Pricing 2N native operations at w on both arms lowers hash/rand at w = 1 from 202.97 to 182.77 (k16 N18), from 51.49 to 48.83 (k16 N36) and from 203.26 to 192.62 (k18 N36), and by under 3 % elsewhere; at w = 0.1 the change is under 1.2 %. The order (tie at w = 0, hash costlier at 0.1 and 1) and the "no strict ranking" statement are unchanged. P8 holds by construction (N^2 + w p >= N^2): it is a derived statement, not a measurement.',
      suggested_fix='Add the per-base native count (2N, both arms) to the cost table and price it in the band, or call the listed ratios upper bounds. Label P8 "derived (holds by construction)".'),
 dict(id='E6', severity='minor',
      text='README says the binary out/*.npz are mirrored as out_json/*.json, and the wave .gitignore says data-suffixed outputs stay on the run host. In fact 9 of the 10 npz are tracked in Git at 190daab375. They were committed during the run (376b26f92a to 969a857783), before the ignore rule of 048030bee7 existed. Only out/cell_k24_N579.npz is untracked and ignored; it is bound by out/SHA256SUMS (cd4f30e1...) and its JSON mirror (7720c5a3...). The tracked blobs match SHA256SUMS. At 27-30 KB they are under the 64 KiB threshold, so no policy is broken.',
      suggested_fix='State which npz files are in Git and that cell_k24_N579.npz exists only on the run host, bound by its sha256 and its lossless JSON mirror.'),
 dict(id='E7', severity='minor',
      text='"Hash replicates are disjoint rank windows" holds for hash and hash2 but not strictly for hash_proj. Base b draws rank positions [bN, bN + floor(1.3N) + 10) and keeps the first N distinct [8]P images, so when images collide it borrows positions from window b+1. Validator replay counts: 24 of 512 bases (k16 N18), 69 of 512 (k16 N36), and 0 to 6 of the first 16 or 32 bases in the other cells. The effect on hash_proj/rand_proj is negligible.',
      suggested_fix='Qualify the disclosure: hash and hash2 windows are disjoint; hash_proj windows overlap the next window when a projected image collides.'),
 dict(id='E8', severity='minor',
      text='The disclosed PROTOCOL section 9 discrepancy covers only "3,600 to 29,000" hash evaluations per column. The same paragraph\'s design ratios ("w = 0.1: 6.0 to 21.2; w = 1: 50 to 203") also differ from the computed 5.92 to 21.23 and 50.19 to 203.27, and this is not disclosed.',
      suggested_fix='Extend the existing disclosure to this range. Leave the frozen text unedited.'),
 dict(id='E9', severity='note',
      text='[pcc:unverified] appears once per paragraph or bullet rather than after every result sentence: 15 items hold 2 to 4 result sentences under one trailing tag, and the tables carry none. `pcc lint-prose` reports 0 findings.',
      suggested_fix='Add a file-level line stating that every number and table in README.md is [pcc:unverified], or tag each result sentence.'),
]

report = dict(
 schema='validator-report/v1 (L1 wave 7)',
 lead='L1 keyed-hash factor-base membership in C = 2 table-and-probe search',
 directory='research/curve25519_wave7_untried_20261122/L1_hash_base',
 validated_commit='190daab37583c88cca0b8bea595e081140a12af9',
 head_at_end_of_validation='1e42afc88d65368595a9503f63cb5c61f1ce1899 (L3-only change since 190daab375)',
 protocol_commit='91f2f2bcde (freeze, 2026-11-22T03:10:00Z)',
 run_launch='run.log: launch 2026-11-22T03:10:04Z from commit 91f2f2bcde',
 analysis_commit='e783d12a5b130e2759ce9685275cb9e1187d7ca1',
 writeup_commit='190daab37583c88cca0b8bea595e081140a12af9',
 validator_date='2026-11-22',
 verdict='pass_with_errata',
 verdict_reason=('Independent code reproduced every measured number in README.md and analysis/summary.json from the raw per-base counts. Point estimates are identical; the validator\'s own bootstrap intervals agree within Monte Carlo noise; all ten verdicts P1-P8 hold with the same counts. '
                 'The validator rebuilt each base from the protocol rules and recomputed its exact coverage with its own point arithmetic: all 512 bases of every arm in both k = 16 cells and 16 to 32 bases of every arm in the other eight cells, 0 mismatches. '
                 'An end-to-end k = 16 replay with its own targets re-verified 2.07 M hits with 0 failures and matched the recorded rates within sampling error. Curve facts were confirmed by two independent methods; the hash rule (key, theta, |V| = N) and window disjointness were confirmed in all ten cells. '
                 'The main result stands: pooled hash/rand 1.0013 [0.9978, 1.0048]. '
                 'The errata concern disclosure, labelling and accounting: undisclosed overlap of the pre-freeze smoke with 300 of 512 bases of the two k = 16 cells (verdicts hold without them), the commit label, an inverted citation, an unlabelled exact range, shared native work excluded from the band, artifact and hash_proj disclosures, and tag granularity. No erratum changes a count, a ratio or a verdict.'),
 independence=('Validator code lives in the session scratchpad: vfacts.py, vdata.py, vdata_holdout.py, vcompare.py, vreplay.py, vreport.py. It imports none of l1lib.py, run_l1.py, analyse_l1.py, export_json.py or figure.py. Reused from the producer: the protocol text (seed rule, arm definitions, estimator, thresholds), the key bytes and the curve parameters. '
               'Bases were rebuilt with numpy Generator calls following the protocol seed rule so that per-base exact counts could be compared one to one. Targets, generator, discrete-log table, square roots, point arithmetic, coverage and verification are the validator\'s own. No producer script was executed.'),
 environment=dict(python='3.13.18', numpy='2.5.3', host='4-core shared host, one core pinned with taskset (cores 2-3), OMP/OPENBLAS threads = 1',
                  scratch='/tmp/claude-0/-home-user/89ca8f69-b480-5417-bbfb-508279d3c6a2/scratchpad/val',
                  validator_script_sha256={'vfacts.py': 'd3cf0abd762aa4f0a2f9b16d3b83638f3e75c7861a1f680ae9a55958c236d9a3',
                                           'vdata.py': '1a084339910c5b4f47ecd9c5798278e24e54756ebea1d7bd732633002b9ca438',
                                           'vdata_holdout.py': '878ccbac639b1875c3fc3508372a8dae4500e00115f04426e40ffb45069c233d',
                                           'vcompare.py': '931e41a49294d79514502cd7c685d646a0b5e6331dbc0fed4b19c6501909f2ec',
                                           'vreplay.py': '89c0fc3c8f8b832e3117dd158b1dfca6d772fcc0cf3f1f95cc69f9846244f993'}),
 checks=checks,
 errata=errata,
 recomputed=dict(curve_facts=F, pooled=pooled, per_cell=per_cell, predictions=pred, holdout_k16_bases_300_511=hold,
                 replay_k16=replay16, exact_coverage_spot_checks=spot, cost=cost,
                 readme_numeric_comparisons=dict(total=len(C['readme_checks']), failed=sum(not c[3] for c in C['readme_checks'])),
                 summary_json_agreement=C['summary_vs_mine']),
 frozen_hashes={'l1lib.py': 'e1c09a5b6f9043f04895c6e69bd8bc6e7f71fe73ee2b9016275a5118d09a1bed',
                'run_l1.py': '202f81235c4dc9a4aa33438b25da297c6df83e04b1c26d35b9164ba10a646462',
                'analyse_l1.py': '2b402f394d40bdfa81783f308b1c3b0c3baccf3e9ce2765e039a9263778dfbca',
                'PROTOCOL.md': '4ab666fb8a62794bf2916d88e588b76fd8496261b4d4682aabb667726cedde25',
                'FREEZE.sha256': '35dc6c737329fd44c838053a5834c728033e6309a6a0a78a72cad39c5e5d1ed7'},
 artifact_sha256_at_validation={
  'README.md': '231f143d2c2f99994adfa50ae49e35ce7843e69c61c07d67ff8364b08de8be18',
  'REPORT.typ': 'eb7fd20ca553144464a984c83084874bc472487ce9e1e9d002c186d137b49de2',
  'REPORT.pdf': '54440a3550f10a1e9903e56d2fa90a3f15eea0b9e69758b803628855344d9597',
  'run.log': '86a85e0baa08558f811c25fb532513e6c6aaa7798fc722130c44e272064f537f',
  'analysis/RESULTS.md': '209d41ce60078ac222d9012d03bbca31cc437b85f00765f3084673ae74c97d01',
  'analysis/summary.json': '16acf0c985f85a8c2541a9a65d5ad5db3fc27b86513c433a48b78236b80f0459',
  'curve_facts.json': '1c94cb0826723ffc0fb0048523ff2c52c7a99d43380653bbc371363c99aed650',
  'export_json.py': 'bc2ccfffc95cfa224252a87a82c2f77a878347a703aa4f4878f64daa387ccc76',
  'out/SHA256SUMS': '3b04fb02e532535c2a8cac4742e90e04d9bbc061430f8413af9b2c601815bb38',
  'out_json/SHA256SUMS': 'c865f33b474cb60416043aec9768ae2edbdde9bdd5f38f72a91fe460f857b796',
  'out/*.npz and out_json/*.json': 'as listed in out/SHA256SUMS and out_json/SHA256SUMS; all 20 verified'},
 pinning_note='This report pins README.md at 231f143d...; put corrections in the README Errata section only if the gate file records the post-errata hash, otherwise use a sibling ERRATA.md (lesson L-8d86e8fc352c).',
 commands=[
  'cd /home/user/crypto && git rev-parse HEAD; git status --porcelain --ignored -- research/curve25519_wave7_untried_20261122/L1_hash_base',
  'for c in 91f2f2bcde e783d12a5b 190daab375: git show $c:<L1>/{l1lib.py,run_l1.py,analyse_l1.py,PROTOCOL.md,FREEZE.sha256} | sha256sum; (cd <L1> && sha256sum -c FREEZE.sha256)',
  'git rev-list 91f2f2bcde..190daab375 | xargs -n1 git diff-tree --no-commit-id --name-status -r -- <L1>',
  'for each out/*.npz: git show <first-add-commit>:<path> | sha256sum vs disk vs out/SHA256SUMS',
  'git diff 9a45d456d4 91f2f2bcde -- <L1>/{analyse_l1.py,run_l1.py,l1lib.py}; git diff 2d683358c2 9a45d456d4 -- <L1>/{run_l1.py,l1lib.py}',
  'taskset -c 3 python3 -I vfacts.py > vfacts.json',
  'VFACTS=vfacts.json taskset -c 3 python3 -I vdata.py <L1> vdata.json',
  'HOLDOUT=1 VFACTS=vfacts.json taskset -c 2 python3 -I vdata_holdout.py <L1> vdata_holdout.json',
  'python3 -I vcompare.py <L1>',
  'taskset -c 3 python3 -I vreplay.py <L1> full 16 512 vreplay_k16.json',
  'taskset -c 3 python3 -I vreplay.py <L1> spot {18,20,22} 32 vspot2_k{k}.json; taskset -c 3 python3 -I vreplay.py <L1> spot 24 16 vspot2_k24.json',
  'python3 -I (smoke rows vs data rows: np.array_equal(smoke[a], data[a][:B]) for every arm)',
  'python3 tools/pcc/pcc.py lint-prose <L1>/README.md',
  'python3 -I vreport.py'],
 not_applicable=['ecbench band replay (no ecbench session, no rho/BSGS arm)', 'binary-to-commit binding (pure Python; no compiled binary)',
                 'validation gate VALIDATION.json (no leaderboard or benchmarks row is proposed)'],
 edits_by_validator=['research/curve25519_wave7_untried_20261122/L1_hash_base/validator-report.json (this file only)'],
 lessons_proposed_for_main_session=[
  "python3 tools/lessons.py outcome --scope validator --task 'wave-7 L1 hash-base validation' --result success --applied L-99ee952350b2,L-26895912385b,L-1a5a6b64022c,L-8d86e8fc352c --evidence research/curve25519_wave7_untried_20261122/L1_hash_base/validator-report.json",
  "python3 tools/lessons.py test --lesson L-99ee952350b2 --outcome confirm --context 'L1 vcompare/vdata indexed summary.json and npz keys with []' --evidence research/curve25519_wave7_untried_20261122/L1_hash_base/validator-report.json",
  "python3 tools/lessons.py test --lesson L-26895912385b --outcome confirm --context 'L1 ratios recomputed from raw per-base counts; T0 paired' --evidence research/curve25519_wave7_untried_20261122/L1_hash_base/validator-report.json",
  "python3 tools/lessons.py add --scope validator --trigger 'validating a study whose smoke test uses the same seed rule as the run (seed parts without a smoke role or B)' --action 'compare the smoke outputs row by row with the first rows of the data; if identical, recompute every verdict on the held-out rows and report the pre-freeze overlap' --why 'wave-7 L1: smoke B = 300 rows were byte-identical to bases 0-299 of both k = 16 data cells and preceded the freeze' --evidence research/curve25519_wave7_untried_20261122/L1_hash_base/validator-report.json"],
)
out = f'{L1}/validator-report.json'
json.dump(report, open(out, 'w'), indent=1, default=float)
print('wrote', out)
