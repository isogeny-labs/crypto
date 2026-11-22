"""Assemble validator-report.json for wave-7 L2 from the validator's own outputs."""
import json, hashlib, os

V = '/tmp/claude-0/-home-user/89ca8f69-b480-5417-bbfb-508279d3c6a2/scratchpad/val_L2'
L = '/home/user/crypto/research/curve25519_wave7_untried_20261122/L2_highdeg_coords'
ld = lambda f: json.load(open(f'{V}/{f}'))
desk = ld('desk_16_18_20_x-ye-xe-v-q-xeye-jac6-uv.json')
cen = ld('census_check.json')
mv = ld('mac_verdicts.json')
mc = ld('mac_check_16_18_20_x4-v4-x5-v5.json')
rp = ld('replay_xAA-v-xe-xeye_4-5-6-7_K2000.json')
ec = ld('e_check.json')
pc1 = ld('pc1_own.json')
ex = ld('extra_checks.json')
VF = cen['verdicts_full']
r4 = lambda x: round(x, 4)

desk_table = {}
for f in ['x', 'ye', 'xe', 'v', 'q', 'xeye', 'jac6', 'uv']:
    row = {}
    for k in ('16', '18', '20'):
        R = desk[k][f]
        row[k] = dict(deg=R['degree_function_field'], max_fibre=R['max_fibre'], points_per_value=round(R['points_per_value'], 5),
                      H_logs_over_r=R['H_translations_plus'], symmetries=R['symmetries'], split_fibre=R['split_fibre'],
                      S_pervar=R['S_own']['pervar'], S_total=R['S_own']['total'], S_terms_producer_recount=R['S_producer_check']['producer_terms_recount'],
                      kernel_dims=R['kernels'], fresh_zeros_vanish=R['S_own']['fresh_zeros_vanish'],
                      random_nonvanish=R['S_own']['random_nonvanish'], proportional_to_producer=R['S_producer_check']['proportional'])
    desk_table[f] = row
curves = {k: dict(facts=desk[k]['_curve']['facts'], validator_generator_uv=desk[k]['_curve']['generator_uv'],
                  u_of_rational_4_torsion=desk[k]['_curve']['u_of_rational_4torsion']) for k in ('16', '18', '20')}

cells = {}
for key, c in cen['cells'].items():
    cells[key] = dict(R_col=r4(c['R_col']), R_col_ci=[r4(x) for x in c['R_col_ci']], R_col_paired_ci=[r4(x) for x in c['R_col_paired_ci']],
                      R_val=None if c['R_val'] is None else r4(c['R_val']), mean_cov=c['mean_cov'], mean_xm=c['mean_xm'],
                      sel_gain_f=r4(c['sel_gain_f']), sel_gain_xm=r4(c['sel_gain_xm']))

replay = {k: dict(K=v['K'], rows_identical=v['rows_identical'], R_col=r4(v['my_R_col'])) for k, v in rp.items() if '/' in k}
mac_own = [dict(k=r['k'], f=r['f'], T=r['T'], kind=r['kind'], target_log=r['target_log'], e=r['e'], terms=r['terms'], roots=r['roots'],
                M1_d_mult=r.get('M1_d_mult'), D_ref=r['D_ref'], D_root=r['D_root'], muladds=r['hist'][-1]['muladds'],
                ranks=[h['rank'] for h in r['hist']], producer_muladds=r['producer']['muladds'],
                all_match=all(v for kk, v in r['match'].items() if kk != 'muladds_ratio') and r['match']['muladds_ratio'] == 1.0)
           for r in mc]

checks = []
def chk(name, result, detail):
    checks.append(dict(name=name, result=result, detail=detail))

chk('commit_binding', 'pass',
    'Validated HEAD 77b1e5ed91f5e46b1ab24d1dd7a83499a9eeb034 on branch claude/curve25519-index-calculus-i674j4; L2 directory clean (git status empty). '
    'Freeze b0a6957edb (protocol + code), run + frozen analysis a936c01c55, write-up 77b1e5ed91. run.log line 1: launch 2026-11-22T03:26:37Z at b0a6957edb; '
    'data mtimes 03:38:19 (sumpoly.json) to 04:21:48 (engine_k20_T5.json), all after the freeze commit (03:26:33).')
chk('frozen_files_byte_identical', 'pass',
    'git diff b0a6957edb..HEAD is empty for PROTOCOL.md, scripts/*, smoke/run_smoke.sh, smoke/SMOKE.md; git diff a936c01c55..HEAD empty for data/, analysis/, run.log; '
    'blob ids of PROTOCOL.md and all 8 scripts equal the freeze tree. Diffs 56e5973465 -> b0a6957edb match the four instrument fixes listed in PROTOCOL section 0 exactly '
    '(PC3 int64 overflow; allne/bool casts and per-curve Macaulay files in analyse.py; per-curve macaulay runs in run_all.sh; root-mode D_root with lin_target); '
    'the PROTOCOL diff over the same span adds only section 0 text, the D_root wording, twist orders and section 9; no prediction or threshold changed.')
chk('artifact_hashes', 'pass',
    'sha256sum -c data/SHA256SUMS: 131/131 OK (120 npz + census_meta, 6 engine, 3 macaulay, sumpoly JSON); smoke/SHA256SUMS 10/10 OK; data_json/SHA256SUMS 130/130 OK. '
    'Validator loader: all 120 data and 10 smoke JSON mirrors equal their npz arrays exactly (dtype, shape, values). Every npz seed field equals the PROTOCOL seed rule '
    'int(sha256("wave7-L2|k|arm|t")[:12],16).')
chk('analysis_reproducible', 'pass',
    'The frozen scripts/analyse.py re-run on data/ into scratch (PYTHONDONTWRITEBYTECODE=1) reproduces analysis/summary.json, analysis/summary.md and the run.log body byte-identically.')
chk('engine_calibration_rerun', 'pass',
    'Binary /home/user/bin-archive/curve25519_family-5abc70ba sha256 5abc70ba47b1...3746 equals data/engine_binary.sha256. Re-running engine_calib.sh commands for k in {16,18,20}, T in {4,5} '
    'gives JSON identical to data/engine_k*_T*.json except wall_s: solving_degree_max 6 everywhere, poly_terms_per_specialisation 121 (T=4) and 256 (T=5).')
chk('curve_facts', 'pass',
    'Own point count over all u (Atkin square roots, Weierstrass group law, generator = smallest-u point of order 8r, different from the producer\'s): #E = 65672, 263096, 1049624 = 8r with r prime; '
    'twist orders 65204, 261172, 1047524 = 4 x prime; p = 5 mod 8; A^2-4 non-square; A+2 square only at k=16; rational 4-torsion has u = 1 at k=16 and u = -1 at k=18, 20.')
chk('desk_degrees', 'pass',
    'Degree of f: E -> P^1 from the function field (polynomial gcd mod p; even f = g(u): 2 deg g; odd f = h(u) v: deg_u(h^2 w)): x 2, ye 2, xe 2, v 3, q 3, xeye 4, jac6 6, uv 5 on all three curves, as PROTOCOL section 2.')
chk('desk_fibres_points_per_value_P3', 'pass',
    'Exact fibre tables over every point: max rational fibre = deg f except xeye = 2 at k=18, 20 (4 at k=16). Points per value 1.00355-1.00358 (k=16), 1.00366-1.00367 (k=18), 1.00100 (k=20); P3 band [0.99, 1.01] holds.')
chk('desk_symmetries_and_H', 'pass',
    'Signed symmetries P -> eps P + Q, Q in E[8], checked on every point: the sets equal the PROTOCOL section 2 table (plus (1,2,-),(1,6,-),(-1,2,+),(-1,6,+) for xeye at k=16) on all curves. '
    'Translation parts of the + stabiliser: xe {O,T0}; xeye <Q1> (order 4) at k=16 and {O,T0} at k=18, 20; trivial for the rest.')
chk('summation_polynomials_own_interpolation', 'pass',
    'Own interpolation in the elementary-symmetric basis e1^i e2^j e3^k (box i+j+k <= D, parity classes for odd f) from own sampled zeros: kernel 0 at D-1 and exactly 1 at D. '
    'Per-variable / total degree: x 2/4, ye 2/6, xe 4/12, v 9/18, q 9/21, xeye 4/12, jac6 18/54, uv 25/50 on every curve; 1000/1000 fresh zeros vanish and 1000/1000 random triples do not; '
    'proportional to the producer\'s recorded polynomial; recounted producer terms 13, 9, 11, 315, 429, 11, 6495, 5894. Split-fibre counts (200 pairs) max = per-variable degree; xeye has no fully rational fibre at k=18, 20. '
    'PC3: own S^x proportional to the explicit Weierstrass S3 at k=16, 18, 20.')
chk('P4_total_degree', 'pass_with_erratum',
    'Verdict reproduced (failed). By the frozen text (x = 4; others <= 2 x per-variable) the failures are ye 6 (bound 4), xe 12 (8), q 21 (18), xeye 12 (8), jac6 54 (36); x 4, v 18, uv 50 are within. '
    'analyse.py uses TOT_MAX ye = 6, so ye passed there; the README lists ye as within (erratum E2).')
chk('translation_collapse_mechanism', 'pass',
    'Every xe and xeye base class set tested (200 random 64-value frames per curve) is closed under +h for h in H (k=16: xe {T0}, xeye {Q1,T0,-Q1}; k=18, 20: {T0}). '
    'Coverage per column: xe 0.508-0.568, xeye 0.507-0.534 (k=18, 20, 1/|H| = 1/2), xeye 0.274-0.311 (k=16, 1/|H| = 1/4).')
chk('census_recompute_from_raw', 'pass',
    'Own recomputation of all 120 cells from the npz rows: R_col, R_val, means and selection gains equal summary.json exactly (max abs diff 0); own bootstrap intervals (B=2000, independent arms) '
    'differ from the producer\'s by at most 0.0036. Paired bootstrap reported beside each cell.')
chk('census_independent_replay_k16', 'pass',
    'Own point table, own base construction and own signed-pair coverage counter, sharing only the seed rule and the L, L\', L\'\' draw order: arms xAA (x-frame null), v, xe, xeye at k=16, t=4..7, '
    'K=2000 each: 32000/32000 rows identical in all 10 columns (values, points, cols, covered, J, xm covered/J, xv cols/covered/J), including the failing P1x cell (xeye t=7, R_col 0.3110).')
chk('PC1_replay', 'pass', 'Own code at k=20: x in [0,32) covers 42, [0,64) 162, L=976503 t=5 198, L=908172 t=6 536, equal to the published frame-search counts.')
chk('controls_PC1b_PC2_NC1_NC2_NC3', 'pass',
    f'PC1b sel gain at k=20 xAA t=4, 5: {VF["PC1b"]} (>= 1.5). PC2 R_col {r4(VF["PC2_range"][0])}-{r4(VF["PC2_range"][1])} (band [1.6, 2.4]). NC1 {VF["NC1_inside"]} in [0.95, 1.05]. '
    f'NC2 {r4(VF["NC2_range"][0])}-{r4(VF["NC2_range"][1])} (band [0.9, 1.1]). NC3 ye {r4(VF["NC3_range"][0])}-{r4(VF["NC3_range"][1])}; ye per-variable degree 2; P5e holds.')
chk('P1', 'pass', f'{VF["P1_inside"]} qualifying cells (mean xm >= 50; excluded {VF["P1_excluded_cells"]}); qualifying R_col range {r4(VF["P1_range_qualifying"][0])}-{r4(VF["P1_range_qualifying"][1])}; '
    f'max interval lower bound {r4(VF["P1_max_ci_lo"])} (<= 1.15).')
fc = cen['P1x_failing_cell']
chk('P1x', 'pass (verdict reproduced: failed)',
    f'{VF["P1x_inside"]}: the only cell outside its band is xeye k=16 t=7, R_col {r4(fc["R_col"])} (mean cov {fc["mean_cov"]} / mean xm {fc["mean_xm"]}), own 95% CI [{r4(fc["ci"][0])}, {r4(fc["ci"][1])}], '
    f'paired CI [{r4(fc["paired_ci"][0])}, {r4(fc["paired_ci"][1])}], bound 1.2/4 = 0.300; producer CI [{r4(fc["producer"]["ci"][0])}, {r4(fc["producer"]["ci"][1])}]. Blind cells alone: all inside. '
    'PROTOCOL applies no xm filter to P1x; analyse.py does (mean xm >= 50) but every xe/xeye cell has mean xm >= 70, so no effect.')
chk('P2', 'pass', f'{VF["P2_inside"]}; R_val v {r4(VF["R_val_range_v"][0])}-{r4(VF["R_val_range_v"][1])}, q {r4(VF["R_val_range_q"][0])}-{r4(VF["R_val_range_q"][1])}, '
    f'uv {r4(VF["R_val_range_uv"][0])}-{r4(VF["R_val_range_uv"][1])}, jac6 {r4(VF["R_val_range_jac6"][0])}-{r4(VF["R_val_range_jac6"][1])}, xe {r4(VF["R_val_range_xe"][0])}-{r4(VF["R_val_range_xe"][1])}, '
    f'xeye k=18,20 {r4(VF["R_val_range_xeye_(18, 20)"][0])}-{r4(VF["R_val_range_xeye_(18, 20)"][1])}, xeye k=16 {r4(VF["R_val_range_xeye_(16,)"][0])}-{r4(VF["R_val_range_xeye_(16,)"][1])}.')
chk('lead_positive_census', 'pass (not met)', f'No arm reaches R_col >= 1.5; maximum over test arms {r4(VF["max_R_col_test_arms"])}.')
chk('macaulay_own_closure', 'pass',
    'Own Boolean reduction (explicit S3 for x; own e-basis S^v for v), own Moebius transform and own degree-D closure (MutantXL rule, documented counting convention) on 3 common + 1 covered recorded targets per (k, f, T), '
    'k in {16,18,20}, f in {x, v}, T in {4, 5}: 48/48 records equal in e, terms, roots, M1 multiplier degree, D_ref, D_root, rank per D and counted mul-adds (exact). '
    'x: e 4, D_ref 6 at n=8 and n=10, mul-adds 8.914e7 (T=4) and 3.960e9 (T=5); v: e = n, D_ref = n, 1.094e8 and 8.407e9.')
chk('macaulay_all_records_independent', 'pass',
    f'For all {len(ec)} recorded targets (8 f x T=4,5,6 x 13 targets x 3 curves, none skipped), own S^f gives the recorded e, terms, Boolean roots, M1 degree, census coverage of the target and base coverage of [0,2^t) exactly. '
    f'Own true decompositions verified with the validator group law: covered <=> at least one decomposition, on every record. The producer\'s true_decomp is exactly 2x the validator\'s on '
    f'{sum(1 for x in ec if not x["true_match"])} records, all with a root value 0 (erratum E9).')
chk('P5a', 'pass (verdict reproduced: failed)', f'Mismatches only xe and xeye at T=4, e = 6 instead of 8, on all three curves ({mv["P5a_mismatch_count"]} records); every other (f, T) holds. analyse.py truncates its mismatch list to 20 (all k=16) but the README statement "on all three curves" is correct.')
chk('P5b_P5c_P5d_P5e_P5f', 'pass',
    f'P5b dense D_ref = n on all refutable targets. P5c x D_ref = 6 on 30/30 at T=4 and 30/30 at T=5. P5d v/x and q/x median mul-adds 1.2271 (T=4) and 2.1231 (T=5) on every curve. '
    f'P5e median D_ref ye = x = 6. P5f xe, xeye 9 vs x 6 at T=5.')
chk('P5g', 'pass (verdict reproduced: failed)', f'Pooled {mv["P5g_pooled"]}; per arm {mv["P5g_per_arm"]}: xe and xeye need d_mult = n-2 on every refutable target, all other arms n.')
chk('P6_cost_per_relation', 'pass', 'v/x and q/x cost-per-relation ratios at T=5: ' + ', '.join(f'{k} {r4(v)}' for k, v in mv['P6_ratios'].items()) + ' (all >= 1.0). Base coverage recomputed independently for every cell. '
    'The base coverage counts are single draws as small as 10 points (k=18 q, T=5), as the README states.')
chk('lead_positive_M2', 'pass (not met)', f'Lowest T=5 cost-per-relation ratio to x over all f: {r4(mv["min_T5_ratio_any_f"][0])} ({mv["min_T5_ratio_any_f"][1]}); over the test arms the lowest is 0.8412 (jac6 k=20); threshold 0.67 on >= 2 curves not met.')
chk('consistency_C1_C2_rank', 'pass', 'census-system agreement on all records; rank V_n = 2^n - |Z| wherever D = n was reached; S_x terms 121/256/484; engine solving_degree_max 6 = median D_ref(x) 6 at every (k, T in {4,5}).')
chk('verdict_rule_application', 'pass_with_erratum',
    'Instrument-validity gates all pass (PC1, PC2, PC3, NC1, NC2, consistency, S^f verification). Positive rule not met (census and proxy). Negative rule needs P1, P1x and P6: P1x failed, so no negative. '
    'The outcome is "no positive; not negative; mixed, reported as it falls". The README label "no verdict" reuses the section 7 wording reserved for "instrument invalid" (erratum E6).')
sens = cen['sensitivity']
chk('blindness_seen_prefreeze_rows', 'fail (disclosure); verdicts unaffected',
    'The pre-freeze K=50 census at k=16, t=5 used the data seed: the first 50 rows of data/census_k16_{v,xeye,even,rand,xAA}_t5.npz give mean covered / mean xm 262.8/255.4, 77.92/278.96, 507.56/259.36, 260.28/256.28, 272.6/266.8, '
    'exactly the PROTOCOL section 0 table. PROTOCOL section 0 says the cell is "re-drawn anyway, with a different seed" (false); the README seed-overlap list omits this overlap; the promised with/without-that-cell verdicts are not reported (erratum E1). '
    f'Recomputed: without the k=16 t=5 cells P1 {sens["without_seen_cell_k16t5"]["P1_inside"]}, P1x {sens["without_seen_cell_k16t5"]["P1x_inside"]} (same failing cell), P2 {sens["without_seen_cell_k16t5"]["P2_inside"]}, '
    f'NC1 {sens["without_seen_cell_k16t5"]["NC1_inside"]}, PC2 and NC2 hold.')
chk('blindness_smoke_overlaps', 'pass',
    'Verified: the 20 smoke census rows per arm at k=16, t=4 equal the first 20 data rows for all 10 arms (same seeds); smoke Macaulay target 47248 is a data target with identical closure histories for x, q, xeye at T=4, 5; '
    'covered xeye T=4 target 17664 coincides (other smoke covered targets do not); the k=16 smoke interpolations of x, ye, xe, v, q, xeye equal the data coefficients. '
    f'Dropping the smoke rows: P1 {sens["drop_smoke20_k16t4"]["P1_inside"]}, P1x {sens["drop_smoke20_k16t4"]["P1x_inside"]}, P2 {sens["drop_smoke20_k16t4"]["P2_inside"]}; dropping smoke rows and the 50 pre-freeze rows: '
    f'P1 {sens["drop_both"]["P1_inside"]}, P1x {sens["drop_both"]["P1x_inside"]}, P2 {sens["drop_both"]["P2_inside"]}; every verdict unchanged.')
chk('blindness_timeline', 'note (erratum E7)',
    'The analysis-script predictions (HSIZE, SYM_PRED, per-variable degrees, TOT_MAX) are in c47218b9d5 (03:16:23), before any committed smoke output (first in 56e5973465, 03:20:10), which supports the blind xe claim. '
    'PROTOCOL.md first appears in 56e5973465 together with the first smoke outputs, so "frozen before the smoke test" cannot be verified from git; the verifiable freeze b0a6957edb follows the smoke. '
    'The P5d threshold changed in the draft analysis from >= 3 (T=5) to >= 1.0 (T=4, 5) in fcd9e9186a (03:18:24); the measured 2.12 passes the frozen threshold and would have failed the draft one; not disclosed.')
chk('readme_numbers', 'pass_with_errata',
    'Every README number was checked against the data and the recomputation. Wrong or misleading: ye listed as within the P4 total-degree bound (E2); "dense functions cost 1.01-12.3x x per relation" (E3); '
    '"0.989-1.007 in 45/45 qualifying cells" (E4); selection gain "1.4-6 in every arm" (E5); outcome label (E6); npz exclusion reason (E11). REPORT.typ/REPORT.pdf repeat the same sentences. All other values hold, e.g. the desk table, census ranges, P-table outcomes, proxy table, P6 table, C2.')
chk('pcc_tags_and_labels', 'pass_with_erratum',
    'Result sentences carry [pcc:unverified] except a few listed in E8. Measured, counted and derived quantities are labelled (section "Derived against measured"). Distance to rho is imported from curve25519_frame_search_20261103 README lines 105-111 '
    '(547-936 group operations, 18x to 5.9e6x, generic baseline 7.8x), quoted correctly. No rho or BSGS work is relabelled as index calculus.')
chk('reporting_contract_and_scope', 'pass',
    'Mechanism measurement, explicitly not a leaderboard or benchmark row, no speed claim; no m=83 or ECC2K-130 claim; no cross-method ranking is measured, so no section 5a band is required (the README imports distances only). '
    'No discrete-log solve or closure is claimed; the decompositions behind covered targets were re-verified with independent group arithmetic.')
chk('proxy_vs_engine_scale', 'note (erratum E10)', 'For x the M2 proxy counts 22.0x (T=4) and 31.1x (T=5) the engine F4 mul-adds per target (8.91e7 vs 4.05e6; 3.96e9 vs 1.27e8). The README says the proxy is not F4\'s count but does not give the factor.')
chk('file_policy', 'pass_with_erratum', 'data/*.log <= 22.5 KB, sumpoly.json 442 KB, macaulay JSON 134 KB, JSON mirrors <= 93 KB, REPORT.pdf 190 KB. The npz files are git-ignored "under the 64 KB data-file policy", but every L2 npz is <= 31.3 KB (2.43 MB total); mirrors verified lossless, so no evidence is lost (E11).')

errata = [
    dict(id='E1', severity='moderate',
         text='PROTOCOL section 0 says the seen pre-freeze K = 50 census at k = 16, t = 5 is "re-drawn anyway, with a different seed" at K = 2000. It was not: the first 50 rows of data/census_k16_{v,xeye,even,rand,xAA}_t5.npz reproduce the section 0 table exactly (262.8/255.4, 77.92/278.96, 507.56/259.36, 260.28/256.28, 272.6/266.8), so the seen frames are rows 0-49 of the data cells. The README "smoke and the data run share seeds" list omits this overlap, and the with/without-that-cell verdicts promised in section 0 are not reported. Recomputed by the validator: without those cells P1 44/44, P1x 22/23 (same failing cell), P2 70/70, NC1 11/11, PC2 and NC2 hold; dropping only the 50 rows changes no verdict.',
         suggested_fix='Leave PROTOCOL.md as frozen. Add an Errata entry to README.md (and REPORT.typ/pdf) stating the overlap, that section 0\'s "different seed" sentence is wrong, and the with/without verdicts above.'),
    dict(id='E2', severity='minor',
         text='P4 total degree: PROTOCOL bounds every f other than x by 2 x its per-variable degree, i.e. 4 for ye. ye measures 6, so ye also fails as frozen. analyse.py uses TOT_MAX ye = 6 (present since c47218b9d5, before the protocol), so it passed ye. The README P-table (and REPORT) say "x 4, v 18, ye 6 and uv 50 were within", and PROTOCOL section 0\'s smoke disclosure lists xe, q, xeye but not ye, although the smoke also showed ye total degree 6. The P4 total-degree verdict (failed) is unchanged.',
         suggested_fix='List ye 6 (bound 4) among the P4 total-degree failures and note that analyse.py\'s TOT_MAX for ye differs from the frozen text.'),
    dict(id='E3', severity='minor',
         text='README "What the data show" item 4 (and REPORT): "Per relation, the dense functions cost 1.01-12.3x x on the same [0, 2^t) base". That range covers v and q only (the P6 arms). Over the four dense functions at T = 5 the range is 0.84-12.3: uv at k = 18 is 0.88 and jac6 at k = 20 is 0.84, both below x. At T = 4 it is 0.25-9.8. The noise comes from single-base coverage counts as small as 10 points.',
         suggested_fix='Restrict the sentence to v and q (P6), or give 0.84-12.3x at T = 5 with the two sub-1 cells and the single-draw caveat.'),
    dict(id='E4', severity='minor',
         text='README item 1: "0.989-1.007, in 45/45 qualifying cells". 0.989 is the k = 18 jac6 t = 4 cell, which is not qualifying (mean xm 19.8 < 50). The 45 qualifying cells span 0.993-1.007; all 48 cells span 0.989-1.007.',
         suggested_fix='Write "0.993-1.007 in the 45 qualifying cells (0.989-1.007 over all 48)".'),
    dict(id='E5', severity='low',
         text='README census paragraph: "argmax-J frame / mean is 1.4-6 at t = 4-7 for both f-frames and x-frames" "in every arm". Over the function arms the f-frames give 1.42-6.03 and the cols-matched x-frames 1.42-7.20. Over all arms, controls included, the f-frames go down to 1.05 and the x-frames to 1.11.',
         suggested_fix='Give 1.42-6.03 (f-frames) and 1.42-7.20 (x-frames) for the function arms, and say the controls go lower.'),
    dict(id='E6', severity='low',
         text='Outcome label: the README calls the outcome "no verdict: inconclusive, no positive". PROTOCOL section 7 uses "no verdict" only for "Instrument invalid". Here every validity gate passed, the positive rule is not met, and the negative rule fails only because P1x failed. By section 7 the outcome is mixed, reported as it falls.',
         suggested_fix='Use: "Instrument valid; positive rule not met; negative rule not met (P1x failed in one post-hoc cell); mixed outcome, no positive".'),
    dict(id='E7', severity='low',
         text='Blindness timeline. (a) "Frozen before the smoke test" cannot be verified from git: PROTOCOL.md first appears in 56e5973465 together with the first smoke outputs, and the verifiable freeze b0a6957edb comes after the smoke. The analysis-script predictions do predate any committed smoke output (c47218b9d5). (b) In the draft analysis the P5d threshold was >= 3 at T = 5 (c47218b9d5). It became >= 1.0 at T = 4 and 5 in fcd9e9186a (03:18:24), before the protocol\'s first commit. The measured 2.12 passes the frozen threshold and would have failed the draft one. The same commit adds runtime instrumentation, which suggests runs were attempted. Section 0 does not mention this, and the record cannot show whether data informed the change.',
         suggested_fix='Add both points to the README Blindness section and treat P5d as not demonstrably blind.'),
    dict(id='E8', severity='low',
         text='Some result sentences lack their own [pcc:unverified] tag: "P1x failed in one of its 24 cells: x_E.y_E at k = 16, t = 7."; "x refutes at D = 6 at n = 8 and n = 10 on every target." (Answer item 4) and "x refutes at D = 6 at both n = 8 and n = 10." with "the dense functions need D = n at every n" (Power); "Every kernel had dimension 1."; "the x-frames cover 3,264 of 8,209 points there". E[r] has 8,208 non-identity points.',
         suggested_fix='Tag each sentence, and write 8,208 non-identity points of E[r].'),
    dict(id='E9', severity='info',
         text='macaulay.py\'s true_decomp double-counts lifts when a Boolean root has value 0, because the lift dictionary stores both P and -P under key 0 = -0. 14 of 933 records show exactly 2x the validator\'s independent count, all with a zero root value. Census agreement (covered <=> at least one decomposition) is unaffected.',
         suggested_fix='Note it in the README errata. No rerun is needed.'),
    dict(id='E10', severity='info',
         text='For x, the M2 mul-add proxy is 22.0x (T = 4) and 31.1x (T = 5) the engine F4 count per target (8.91e7 vs 4.05e6; 3.96e9 vs 1.27e8). The README calls the proxy "not the engine\'s F4 count" but gives no factor, and the "2.12x" dense/x ratio is a ratio between proxies.',
         suggested_fix='Add the proxy/engine factor for x beside C2 and keep the "2.12x" labelled as a proxy ratio.'),
    dict(id='E11', severity='low',
         text='README Data and .gitignore: "data/*.npz and smoke/*.npz are git-ignored (the 64 KB data-file policy ...)". Every L2 npz is at most 31.3 KB (2.43 MB in total), so the policy did not require excluding them. The committed JSON mirrors are larger (64-93 KB each, about 9 MB), and the validator confirmed they are lossless.',
         suggested_fix='Correct the stated reason, for example "kept out of Git by the wave .gitignore; lossless mirrors in data_json/", or commit the npz files.'),
]

recomputed = dict(
    curves=curves, desk_table=desk_table,
    P4_total_degree_failures_by_frozen_text=dict(ye=[6, 4], xe=[12, 8], q=[21, 18], xeye=[12, 8], jac6=[54, 36]),
    census_cells=cells, census_verdicts=VF, census_sensitivity=cen['sensitivity'], P1x_failing_cell=fc,
    census_vs_producer_max_abs_diff=cen['max_abs_diff_vs_producer_summary'],
    smoke_overlap=cen['smoke_overlap_k16_t4'], prefreeze_K50=cen['prefreeze_K50_k16_t5'],
    replay_k16=replay, mechanism=dict(k16=rp['mechanism_k16'], k18_k20={k: v for k, v in ex.items() if k.startswith('closure')}),
    PC1_own=pc1, PC3_own=dict(k16=True, k18=ex['PC3_own_k18'], k20=ex['PC3_own_k20']),
    macaulay_own_closure=mac_own,
    macaulay_verdicts=dict((k, mv[k]) for k in ('P5a_pass', 'P5a_mismatch_groups', 'P5a_mismatch_count', 'e_table', 'P5b_pass', 'P5c', 'P5d_ratios', 'P5e', 'P5f',
                                                 'P5g_pooled', 'P5g_per_arm', 'P6_ratios', 'cost_per_relation_ratio_vs_x', 'dense_T5_ratio_range', 'dense_T5_below_1',
                                                 'dense_T4_ratio_range', 'lead_positive_M2', 'C1_pass', 'consistency_all', 'rank_check_all_true', 'C2', 'readme_proxy_table')),
    all_records_recheck=dict(records=len(ec), exact_e_terms_roots_M1_covered_basecov=all(x.get('match') for x in ec),
                             true_decomp_double_counts=sum(1 for x in ec if not x['true_match'])),
)

def sha(p):
    return hashlib.sha256(open(p, 'rb').read()).hexdigest()

scripts = {os.path.basename(p): sha(p) for p in sorted(f'{V}/{f}' for f in os.listdir(V) if f.endswith('.py'))}
commands = [
    'git -C /home/user/crypto rev-parse HEAD; git status --short <L2>; git log -- <L2>; git diff 56e5973465 b0a6957edb -- <L2>/scripts <L2>/PROTOCOL.md; git diff --stat b0a6957edb HEAD -- <L2>/PROTOCOL.md <L2>/scripts',
    'cd <L2> && sha256sum -c data/SHA256SUMS && (cd smoke && sha256sum -c SHA256SUMS) && (cd data_json && sha256sum -c SHA256SUMS)',
    'python3 desk.py 16,18,20            # degrees, fibres, symmetries, H, split-fibre, own e-basis S^f, verification, proportionality',
    'python3 census_check.py             # all 120 cells, own bootstrap, verdicts P1 P1x P2 PC1b PC2 NC1-3, overlaps, sensitivity',
    'python3 replay.py xAA,v,xe,xeye 4,5,6,7 2000   # independent k=16 census replay, row-by-row comparison, collapse closure',
    'python3 mac_check.py 16,18,20 x:4,v:4,x:5,v:5 3  # own Boolean reduction and degree-D closure on recorded targets',
    'python3 e_check.py                  # every recorded target: e, terms, roots, M1, coverage, decompositions',
    'python3 mac_verdicts.py             # P5a-g, P6, C1, C2, consistency, README proxy table',
    'PYTHONDONTWRITEBYTECODE=1 python3 <L2>/scripts/analyse.py --data <L2>/data --out <scratch>/reanalyse; cmp with analysis/summary.*',
    '/home/user/bin-archive/curve25519_family-5abc70ba f4cost k bits T 0 10 $((2026112200+10k+T)) 900 2 full 0 30000000  (k=16,18,20; T=4,5), compared with data/engine_k*_T*.json',
]

report = dict(
    schema='validator-report/v1', lead='wave7 L2 coordinate types as decomposition variables',
    directory='research/curve25519_wave7_untried_20261122/L2_highdeg_coords',
    validated_commit='77b1e5ed91f5e46b1ab24d1dd7a83499a9eeb034', freeze_commit='b0a6957edb', run_commit='a936c01c55',
    tree_state='clean (git status empty for the L2 directory); HEAD advanced to 4a7ee6e766 during validation with no change under the L2 directory (git diff 77b1e5ed91..4a7ee6e766 empty there)',
    validator='validator (Claude Opus 5.5), one core, Python 3.13.18, numpy 2.5.3', date='2026-11-22',
    verdict='pass_with_errata',
    summary=('The data, the frozen analysis and every verdict reproduce. The validator\'s own code reproduces the census exactly (32,000 replayed rows at k = 16; all 120 cells recomputed from raw rows), '
             'the desk table (degrees, fibres, symmetries, H, summation polynomials of the same per-variable and total degree, proportional to the recorded ones), the Macaulay proxy '
             '(48 targets: exact e, D_ref, D_root, ranks and counted mul-adds; 933 records: exact e, terms, roots, M1, coverage) and the engine calibration. '
             'The frozen rule is applied correctly: there is no positive result, and the negative rule is not met because P1x failed in one post-hoc cell (xeye k = 16 t = 7, 0.311 > 0.300, interval above the bound). '
             'P4 total degree, P5a and P5g also failed as reported. Errata: one blindness statement is false (E1: the seen pre-freeze rows are the data\'s first 50 rows; verdicts unchanged), '
             'ye also fails P4 total degree by the frozen text (E2), the README states the dense cost-per-relation range for v and q only (E3), and there are smaller wording and disclosure items (E4-E11).'),
    checks=checks, errata=errata, recomputed=recomputed,
    commands=commands, validator_scripts=dict(location='session scratchpad (not durable): ' + V, sha256=scripts),
    lessons_applied=['L-99ee952350b2 (keys checked: M1 present iff roots == 0 and M2 iff T <= 5 and roots <= 8 in every record, so None == None comparisons cannot pass silently)',
                     'L-26895912385b (P1x cell ratio recomputed from unrounded means, paired interval reported beside it)',
                     'L-1a5a6b64022c (report written once, complete)',
                     'L-546d37ca7750 not applicable (no ecbench session or band computed by the study)'],
)
out = f'{L}/validator-report.json'
json.dump(report, open(out, 'w'), indent=1, ensure_ascii=False, default=str)
print('written', out, os.path.getsize(out))
