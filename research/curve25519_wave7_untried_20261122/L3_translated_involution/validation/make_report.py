#!/usr/bin/env python3
import json, hashlib, os
S = '/tmp/claude-0/-home-user/89ca8f69-b480-5417-bbfb-508279d3c6a2/scratchpad/val'
L3 = '/home/user/crypto/research/curve25519_wave7_untried_20261122/L3_translated_involution'
desk = json.load(open(f'{S}/desk_val.json'))
ver = json.load(open(f'{S}/verdicts_val.json'))
ver10 = json.load(open(f'{S}/verdicts_val_b10k.json'))
cen = json.load(open(f'{S}/census_val.json'))
cen_r = json.load(open(f'{S}/census_val_huff5_rerun.json'))
ex1 = json.load(open(f'{S}/exact_replay.json'))
ex2 = json.load(open(f'{S}/exact_replay_odd.json'))
study = json.load(open(f'{L3}/analysis/summary.json'))['verdicts']
sha = lambda p: hashlib.sha256(open(p, 'rb').read()).hexdigest()
V = ver['verdicts']

STUDY_NAME = {'CAL1': 'CAL1_exact_frame_search_fixed_frames', 'CAL2': 'CAL2_null_matches_frame_search_mobius_mean',
              'CAL3': 'CAL3_model_matches_nulls', 'C1': 'C1_translated_frames_pointwise_identity',
              'C2': 'C2_controls_match_null', 'AA': 'AA_split_half', 'PC1': 'PC1_planted_selfpair_removal_detected',
              'PC2': 'PC2_selection_gain_detected', 'P1': 'P1_nohalf_below_null', 'P2': 'P2_Ep_xframes_above_E_xframes',
              'P3': 'P3_JI_degree4_halves_coverage', 'P4a': 'P4a_same_class_same_distribution', 'P4b': 'P4b_no_class_effect',
              'P5': 'P5_full_set_matches_shifted_null', 'P6': 'P6_no_fatter_selection_tail', 'P7': 'P7_Ep_selected_vs_E_selected'}
checks = []


def chk(name, result, detail):
    checks.append(dict(name=name, result=result, detail=detail))


chk('artifact_binding_frozen_commit', 'pass',
    'Validated the L3 subtree at 1e42afc88d65368595a9503f63cb5c61f1ce1899 (branch claude/curve25519-index-calculus-i674j4). '
    'Repository HEAD moved to 5e7db008a9 during validation (other leads only); `git diff --quiet 1e42afc88d -- L3_translated_involution` '
    'is clean, so the tracked L3 files and the working tree are byte-identical to 1e42afc88d. Freeze commit 0e3c0fbda0 (03:23:35Z) '
    'precedes the launch line in run.log (03:23:39Z, "FREEZE.sha256 verified") and the first npz (03:24); script mtimes 03:12-03:18.')
chk('frozen_files_byte_identical', 'pass',
    'FREEZE.sha256 (sha256 9314599...448a) lists 9 files; all 9 hash-match in the working tree and at 0e3c0fbda0, e783d12a5b and '
    '1e42afc88d (git show | sha256sum). PROTOCOL.md is not listed in FREEZE.sha256 but is byte-identical at 0e3c0fbda0 and '
    '1e42afc88d (sha256 1f155c5b...57f4). The short hashes quoted in PROTOCOL.md match FREEZE.sha256.')
chk('data_hashes_and_json_mirrors', 'pass',
    'data/SHA256SUMS: 51/51 OK (45 npz, 3 meta.json, 3 desk json; paths relative to scripts/). data_json/SHA256SUMS: 45/45 OK. '
    'Validator reader: every JSON mirror carries the npz sha256 as source_sha256 and equals the npz in all '
    f"{ver['npz_vs_json']['arrays']} arrays ({ver['npz_vs_json']['mismatches']} mismatches), matching data_json/CHECK.txt.")
n_ex = sum(v['exact_match'] for v in list(ex1.values()) + list(ex2.values()))
n_fr = sum(v['frames'] for v in list(ex1.values()) + list(ex2.values()))
chk('data_produced_by_the_protocol_instrument_frame_exact_replay', 'pass',
    f'With the protocol seed rule (sha256 "wave7-L3|k=|t=|arm=", random.Random, nu then T draws) and the validator\'s own bases '
    f'and coverage counting (pointwise slope tables for h_T, not line-cubic solving), {n_ex}/{n_fr} recorded k = 16 frames match '
    'covered_D, covered_full and n_points exactly: Ep.huff_x, E.x, Ep.x, E.iota_odd, Ep.iota_c01 at t = 4..8, 2000 frames each.')
chk('model_predictions_equal_desk_closed_forms', 'pass',
    'model_predictions.json (frozen, sha256 5daf9fb7...f9d0) equals the DESK section 5 closed forms recomputed by the validator '
    f"(sat(E[N(N-1)]/4 + w E[N]) with w = 1/2 on E, 1 on E', 0 for no-half; JI sat(E[N4(N4-1)]/2 + w4 E[N4])): max relative "
    f"difference {ver['model_json_vs_own_closed_forms_max_rel_diff']} over 225 arm cells. The PROTOCOL.md table matches the JSON "
    'except an unnoted k-dependence of ji_s at t = 7 (erratum E8).')
dc = {c['name']: c for c in desk['checks']}
chk('desk_curve_facts_k16_k18_k20', 'pass' if all(dc[n]['ok'] for n in dc if n.startswith('k')) else 'fail',
    'Own code, all three k: p and r prime (trial division), #E = 8r by character sum, a point of order 8r (E cyclic), A^2-4 '
    "non-square, own Velu (Washington 12.16) for kernel (0,0) gives y^2 = x^3 + A x^2 - 4x - 4A, shift X = x + A gives "
    "X(X-(A+2))(X-(A-2)), #E' = #E, E'[2] rational with v2(#E') = 3 so E'(F_p) = Z/2 x Z/4r, generator of order 4r found, "
    'phi on-curve and homomorphic (400 pairs), phi injective E[r] -> E\'[r] (all 8208 points at k = 16; 1500 distinct at k = 18, 20). '
    f"Twist order 65204 at k = 16. {sum(dc[n]['ok'] for n in dc if n.startswith('k'))}/{sum(1 for n in dc if n.startswith('k'))} pass.")
chk('desk_lemma3_E_which_T_have_a_rational_half', 'pass',
    'k = 16 exhaustive over all 65671 affine points, with 2E(F_p) built by doubling every point (no logs): P in 2E <=> log even <=> '
    'x_P square (0 mismatches, |2E| = 32836 incl. O). T0 = (0,0) has exactly 2 rational halves, both with x = 1 at k = 16 '
    '(A+2 square), so y/x = h_T0 is a translated frame.')
chk("desk_lemma3_Ep_which_T_have_a_rational_half_and_Kummer_map", 'pass',
    "k = 16 exhaustive: 2E'(F_p) by doubling = class (0,0) = {X - e_i all squares} (0 mismatches, |2E'| = 16418). Exactly one "
    "2-torsion point lies in 2E': e0 = A+2 = 1060. Explicit F_{p^2} halving of 400 sampled T: every T has a half over F_{p^2}; "
    f"pi(S) - S gives the bijection {desk['kummer_own_labels']} (validator's own generator; (0,1) is canonical = class of the "
    "2-torsion outside 2E'; the (1,0)/(1,1) labels depend on the generator choice and here agree with DESK).")
chk('desk_halves_of_odd_T_on_E_defined_over_Fp4_not_Fp2', 'pass',
    'For 147 odd-log T at k = 16 the halving quartic (x^2-1)^2 - 4 x_T x (x^2+Ax+1) has no root in F_p or F_{p^2} '
    '(gcd with x^p - x and x^{p^2} - x), hence is irreducible, and f(x) = x^3+Ax^2+x is a square in F_p[x]/(H) = F_{p^4}, '
    'so the halves are defined over F_{p^4} and not over F_{p^2}. All 153 even T have a rational half. The brief correction holds.')
chk('desk_every_degree2_function_is_iota_T_invariant', 'pass',
    '18 degree-2 F_p functions covering every polar-divisor type (2O: Moebius of x; O+Q: h_Q incl. Q = T0; P1+P2 rational, '
    'conjugate pair and double point via (y - l(x))/q(x)), each composed with a random Moebius map: exhaustive over all points, every '
    'fibre has <= 2 points, every 2-point fibre has the same group sum T, T equals the polar-divisor sum (rational), and the 1-point '
    'fibres are exactly the rational halves of T. Since L(D) = <1, f_D>, this covers every degree-2 function up to Moebius '
    '(the general statement is the standard Riemann-Roch argument in DESK section 1, which is correct).')
chk('desk_theorem1_translated_xframe', 'pass',
    '6 random T = 2S at k = 16: a Moebius mu fitted on 3 points satisfies mu o h_T = M o x o tau_{-S} on every point; '
    'B_f(V) = S + B_x(M^{-1}V) as sets; B_f - S negation-closed; D_f = B_x +- B_x and the signed set of B_f = '
    'Sigma_x u (Sigma_x + T) u (Sigma_x - T) as sets of group elements; P_i + P_j = T + (Q_i + Q_j) by curve arithmetic. '
    'Also 900 E.iota_even control frames (t = 4, 5, 6) with the validator\'s own per-frame check (fit on curve points, verify on ALL '
    'points, census equal to the x-frame census on M^{-1}(V)): 900/900.')
chk('desk_lemma2_B_eq_T_minus_B', 'pass',
    '6 odd-T bases at k = 16: B = T - B and all signed pair sums = D u (D + T) u (D - T) exactly.')
chk('desk_theorem4_no_within_fibre_relations', 'pass',
    'k = 16: for 40 odd T and every point a, 2a - T is never in E[r] (0 hits); 300 curve-arithmetic spot checks agree; even T give '
    "16415-16416 hits per T. On E', classes (0,1), (1,0), (1,1) give 0 hits, class (0,0) gives hits. Self-paired fraction of "
    "x-frame points: 0.25 on E (2 of 8 cosets), 0.4999 on E' (4 of 8).")
chk('desk_huff_coordinates_T_class', 'pass',
    "Own Huff model (e0 = 1060, ea = 0, eb = 1056): a x (y^2-1) = b y (x^2-1) holds on every point. Huff x: degree 2 (fibre "
    "histogram [0, 1, 32834]), constant fibre sum T = (0,0) = Wa; Huff y: T = (1056,0) = Wb. Both are 2-torsion, outside 2E', class (0,1).")
chk('desk_jacobi_intersection_degree4_and_pullbacks', 'pass',
    "alpha s^2 + c^2 = 1 and beta s^2 + d^2 = 1 on every point. Fibre histograms show degree 4 (16416-16417 values with 4 points, "
    "<= 4 exceptional values). Invariance: c under {-1, tau_eb, iota_eb}, d under {-1, tau_ea, iota_ea}, s under {tau_(0,0), iota_ea, "
    "iota_eb} and not under -1. Own Velu isogenies: c = Moebius o x o phi_<(A-2,0)> and d = Moebius o x o phi_<(A+2,0)> on every "
    "point; s = g o phihat with phihat of kernel (0,0) (codomain order n, j equal to E's), g = Moebius o x o tau_{-S} with S rational "
    "of order 4, 2S = T_g = (2116, 0). The s identification, labelled derived in the README, now also has a pointwise check at k = 16.")
for key in ('CAL1', 'CAL2', 'CAL3', 'C1', 'C2', 'AA', 'PC1', 'PC2', 'P1', 'P2', 'P3', 'P4a', 'P4b', 'P5', 'P6', 'P7'):
    v, s = V[key], study[STUDY_NAME[key]]
    extra = {a: b for a, b in v.items() if a not in ('held', 'n_ok', 'n_total')}
    same = (v['held'] == s['held']) and (int(v['n_ok']) == int(s['n_ok']) or key == 'AA')
    res = 'pass' if v['held'] and same else ('pass_with_note' if v['held'] else 'fail')
    note = ''
    if key == 'AA':
        note = (f" Validator B = 2000 stream: {v['n_ok']}/30 (adds k=16 t=6 E.x, CI 0.9563-0.9994); B = 10000: "
                f"{ver10['verdicts']['AA']['n_ok']}/30. Study: 28/30 with that cell's upper bound 1.0008. Held in every variant (>= 27). Erratum E4.")
        res = 'pass_with_note'
    chk(f'verdict_{key}_recomputed', res,
        f"Validator: {'held' if v['held'] else 'FAILED'} {v['n_ok']}/{v['n_total']}; study: {'held' if s['held'] else 'FAILED'} "
        f"{s['n_ok']}/{s['n_total']}. {json.dumps(extra, default=str)[:900]}{note}")
cells = cen['cells']
dz = [(k, c['D']['z']) for k, c in cells.items()]
chk('reduced_independent_census_replay_k16', 'pass_with_note',
    f"Validator frames (own PCG64 seeds, own Moebius draws, own bases, own counting), K = {cen['K_per_cell']} per cell, arms E.x "
    f"(with one odd-T shift), E.iota_odd, Ep.huff_x, Ep.x at t = 4..8, plus E.iota_even at t = 4..6 (300 frames): "
    f"{sum(abs(z) < 2 for _, z in dz)}/{len(dz)} covered_D cells within 2 SE of the recorded means; 48/48 frames re-counted by curve "
    "arithmetic agree. Exception: 16|5|Ep.huff_x, z = -3.89 (60.06 vs 63.62); a fresh 4000-frame rerun gives 61.47 (z = -3.18). "
    "Frame-exact replay reproduces all 2000 recorded frames of that cell, and the recorded huff_y, c01, c10, c11 cells (same "
    "distribution by Lemma 2) average 61.49, so the recorded cell is a +3.4 SE excursion of the seeded sample (look-elsewhere "
    "p about 0.15 over 225 cells), not an instrument fault. Recorded n_points means over all 225 cells against their exact expectation "
    "2^t (#finite-valued points)/(p+1): mean z -0.02, sd 0.998, 8 with |z| > 2, 0 with |z| > 3 (frame generation unbiased).")
chk('C1_exceptions_34_code2_frames', 'pass',
    f"control_ok values: code 0 = {V['C1']['code0']}, code 2 = {V['C1']['code2']}, all at t = 4: {V['C1']['code2_cells']}; no other "
    "values. Code 2 means fewer than 3 distinct x(P - S), i.e. fewer than 3 fibres; with N ~ Bin(16, 0.502) P(N <= 2) = 0.0021, so "
    "about 37.6 are expected in 18,000 t = 4 control frames; 34 is consistent (Poisson). 34/90000 = 0.038% <= 1%.")
chk('AA_28_of_30_explanation', 'pass_with_note',
    'The two excluded cells named in the README (k=20 t=4 Ep.x 1.0734, CI 1.0256-1.1212; k=20 t=6 E.x 0.9714, CI 0.9507-0.9931) are '
    'the frozen run\'s and are reproduced by the validator stream. Two (or three) misses in 30 is consistent with a 5% rate '
    '(expected 1.5). The count is stream-sensitive at the third cell (erratum E4).')
chk('readme_k20_census_table', 'pass', '70/70 k = 20 means in the README census table equal the validator\'s means rounded to 0.1.')
chk('readme_k20_ratio_table', 'pass',
    'All 16 entries: point ratios and model values equal the validator\'s to 3 decimals; CIs agree within bootstrap noise (<= 0.002).')
rr = ver['readme_ranges']
chk('readme_answer_and_prediction_numbers', 'pass_with_note',
    f"Recomputed: E odd T / null at t = 4 {rr['E_odd_t4']}; E' no-half / null at t = 4 {rr['Ep_nohalf_t4']}; JI / Ep.x {rr['JI_all']}; "
    f"Ep.x/E.x t = 4 {rr['P2_t4']}, t = 5 {rr['P2_t5']}, t = 8 {rr['P2_t8']}, smallest CI lower bound over all 15 cells "
    f"{rr['P2_lo_all']}; P5 full/shift ratio {rr['P5_ratio']}; full/D {V['P5']['full_over_D_range']} with minimum at "
    f"{V['P5']['full_over_D_min_cell']}; CAL3 {V['CAL3']['range']}; C2 {V['C2']['range']}; P4a ratios {V['P4a']['ratio_range']}, "
    f"max KS {V['P4a']['max_ks']}; P4b {V['P4b']['ratio_range']}; PC2 {V['PC2']['gains']}; P6 {V['P6']['arms']}; P7 geomean "
    f"{V['P7']['geomean']} range {V['P7']['range']}. All README numbers agree except: item 4 direction (E1), PC1 bound 0.992 (E3), "
    '"1.5% at t = 8" (E7), N/4 unit (E2).')
chk('readme_pcc_tags', 'partial',
    'Most result sentences carry [pcc:unverified]; several do not (erratum E6).')
chk('readme_derived_vs_measured_labelled', 'pass',
    'A "Derived vs measured" section separates derived algebra (with numerical checks) from measured exact counts; the multidegree '
    '(2,2,2) and 3x target degree are labelled sketches, not engine-tested; no timings or F4 runs are claimed.')
chk('predicted_positive_not_overstated_engine_caveat_present', 'pass',
    "E' over E x-frames is reported as a measured coverage-only component effect (1.21-1.24x at t = 4 fading to 1.01-1.02x at t = 8; "
    'selected geomean 1.117), with four stated reasons why it needs an engine check (cost vs coverage, 1/N fade, isogeny transport '
    'charge, selection noise) and the specific check (pkm-semaev on E\' vs E, same seeds). No ledger or leaderboard use is claimed.')
chk('distance_to_rho_imported_only', 'pass',
    'The figures (generic baseline 7.8x rho at k = 20; best algebraic arm vs rho-neg 18x / 9.6e5x / 5.9e6x at w = 0 / 0.1 / 1) equal '
    'research/curve25519_frame_search_20261103/README.md lines 109-111; nothing about rho is measured or invented here.')
chk('no_relabelling', 'pass',
    'covered_full (the D +- T sets) is reported as bookkeeping at 3x target degree, never as a gain (P5); no rho/BSGS work is '
    'relabelled; "not a leaderboard row, not a solve" is stated.')
chk('section5a_band', 'not_applicable',
    'No cross-method cost comparison is measured; the README only states that a <= 1.24x coverage change cannot reorder arms 18x to '
    '5.9e6x apart across the imported band (lesson L-546d37ca7750 has nothing to recompute).')
chk('certificates', 'not_applicable',
    'No discrete-log solve, factor-base relation or closure is claimed. The exact counts were instead re-derived frame by frame '
    '(50,000 frames) with independent code.')
chk('disclosures', 'pass',
    'README discloses no Conductor task, no EKS route, no S3 resume checkpoint (token expired, run.log line 1), no bench.record and '
    'no pcc claim; the BRIEF directed a local one-core run. Runtime claim "about 20 minutes, peak RSS 253 MB" matches meta.json '
    '(census 372.6 + 382.9 + 329.2 s, desk 92 s; maxrss 253.0 MB at k = 20).')

errata = [
    dict(id='E1', severity='medium',
         text='README answer item 4 (line 43) and REPORT.typ line 65 invert a measured direction: "An x-frame with the same T-shifted '
              'targets gets 1.00-1.17x of that count." The recorded ratio is the no-half arm\'s covered_full divided by the x-null\'s '
              f"covered_shift in the same class: {rr['P5_ratio'][0]:.4f}-{rr['P5_ratio'][1]:.4f} over 60 cells (model 1.002-1.151; P5 60/60). "
              'So the no-half full set is 1.00-1.17x the x-frame set, and the x-frame gets 0.85-1.00x of the no-half count.',
         suggested_fix='Reword to: "Their full signed set is 1.00-1.17x the set an x-frame covers when it decomposes the same '
                       'T-shifted targets R, R - T and R + T (model 1.00-1.15; P5 60/60). Both need three decompositions of the same '
                       'degree, so this is accounting, not a gain. [pcc:unverified]" Apply the same change in REPORT.typ and rebuild REPORT.pdf.'),
    dict(id='E2', severity='low',
         text='Unit slip. DESK.md section 3 (line 123), copied in REPORT.typ line 617, says "E\' x-frames have about N/4 more than E '
              'x-frames" right after giving the deficits in covered points (N/2 on E, N on E\'). In covered points the model and data '
              'give about N/2: Ep.x minus E.x at t = 4 is 4.23 / 3.92 / 4.45 (k = 16 / 18 / 20) against N/2 = 4.0, and at t = 5 '
              '8.29 / 7.08 / 7.25 against 8.0. N/4 is the number of extra self-paired fibres, each covering 2 points. README line 211 '
              '("adds about N/4 within-fibre relations") is right only if a relation means an unordered fibre pair.',
         suggested_fix='DESK.md is pinned by FREEZE.sha256, so leave it byte-equal and record the correction in README "Errata" (or a '
                       'sibling ERRATA.md). Reword README line 211 and REPORT.typ lines 385 and 617 to "about N/4 more self-paired fibres, '
                       'i.e. about N/2 more covered points".'),
    dict(id='E3', severity='low',
         text='README PC1 line (171) and REPORT.typ line 335: "detected at t = 4-6 in every k (CI upper bound <= 0.992)". At t = 4-6 the '
              'largest upper bound is 0.950 (summary.json 0.9495 at k = 16, t = 6; validator 0.9496). 0.992 is the k = 16 and k = 20 t = 8 '
              'value, and the t = 8 maximum is 0.993 (k = 18). The statement is true but cites the wrong cells.',
         suggested_fix='Change to "(CI upper bound <= 0.950)".'),
    dict(id='E4', severity='low',
         text='A/A "held, 28/30" depends on the bootstrap stream. In the frozen run the k = 16, t = 6 E.x split interval is 0.9574-1.0008. '
              'With the validator\'s independent stream it is 0.9563-0.9994 at B = 2000 (27/30, exactly at the threshold) and contains '
              '1 at B = 10000 (28/30). The verdict holds in every variant.',
         suggested_fix='Add: "A third cell (k = 16, t = 6, E x-frame) has an upper bound of 1.0008; with another bootstrap stream it '
                       'falls just below 1 (27/30, still at the threshold)."'),
    dict(id='E5', severity='low',
         text='Provenance: README line 8 and REPORT.typ line 16 say "the run was committed at e783d12a5b". That commit touches only L1 files. '
              'The L3 run outputs (analysis/summary.json, VERDICTS.md, model_table.txt, data/SHA256SUMS, data/k20/meta.json and the final '
              'run.log) were added in 6d9cb1674a (03:43:40Z); data/k16 and k18 meta.json came in earlier sweep commits (3ccc863cc4, '
              'c36e9d186a). They are present in the e783d12a5b tree.',
         suggested_fix='Say "the run outputs were committed at 6d9cb1674a (and are present at e783d12a5b)".'),
    dict(id='E6', severity='low',
         text='Some result sentences lack [pcc:unverified]. In "Desk results": the curve-facts sub-bullets, Lemma 3 "Checked exactly on '
              '2000 points per curve", Huff "the curve equation was checked on every point", JI "checked on every point", "read off the '
              'fibre histograms" and the invariance-group identification. Also both bullets of "Corrections to the brief", the "Why." '
              'paragraph (the N/4 claim), and items 2 ("1.5% at t = 8") and 4 ("ranging 1.03-1.39 across cells") of the engine-check list.',
         suggested_fix='Append [pcc:unverified] to each listed sentence (README and REPORT.typ).'),
    dict(id='E7', severity='low',
         text='README line 217 and REPORT.typ line 393: "it is 1.5% at t = 8". The measured Ep.x/E.x at t = 8 is 1.0105 / 1.0163 / 1.0195 '
              '(k = 16 / 18 / 20), i.e. 1.1-2.0%. 1.5% is the k = 20 model value (model 1.012-1.015).',
         suggested_fix='Change to "1-2% at t = 8 (measured; model 1.2-1.5%)".'),
    dict(id='E8', severity='info',
         text='The PROTOCOL.md model table says k = 18 and k = 20 are within 0.01 of k = 16 "except where noted". Ep.ji_s / Ep.x at t = 7 '
              'is 0.517 at k = 16 but 0.504 and 0.501 at k = 18 and k = 20, and this is not noted. The file is frozen. Verdicts are '
              'unaffected because analyse.py reads model_predictions.json, which equals the DESK section 5 closed forms exactly.',
         suggested_fix='No change to PROTOCOL.md (frozen). Optionally list this in README "Errata".'),
    dict(id='E9', severity='info',
         text='The recorded 16|5|Ep.huff_x cell (mean 63.62) lies +3.4 SE above the pooled same-distribution estimate: recorded huff_y, '
              'c01, c10 and c11 at that cell average 61.49 over 8000 frames, and the validator\'s 5000 independent frames give 61.19. '
              'A frame-exact replay reproduces all 2000 recorded frames, so this is a sampling excursion of the seeded sample, not a '
              'fault. It is also the cell with P4a\'s narrowest margins (ratio 1.037 against the 1.05 bound; KS 0.0575 against 0.0617).',
         suggested_fix='None required. Optionally note it beside P4a.'),
]

rep = dict(
    schema='validator-report (wave-7 L3)', lead='L3_translated_involution', date='2026-11-22',
    verdict='pass_with_errata', disposition='validated',
    summary=('Desk theorems hold under independent checks: 60/60 checks at k = 16, curve facts at k = 16, 18, 20. All 16 verdicts were '
             'recomputed from the raw arrays with the frozen thresholds and the same outcomes; counts are identical except A/A, which '
             'depends on the bootstrap stream (27 or 28 of 30, held). 50,000 recorded frames were replayed frame-exactly with '
             'independent code. The reduced independent census agrees except one recorded cell, explained as a sampling excursion. '
             'Frozen files are byte-identical. Errata: one direction inversion in a README result sentence (E1, medium) and six '
             'low-severity wording, precision or provenance items. No measured claim or verdict changes.'),
    validated_commit='1e42afc88d65368595a9503f63cb5c61f1ce1899',
    head_at_report_time='5e7db008a97a237b4c3b8cd9c6b6bd2623b818b4 (L3 subtree identical to 1e42afc88d; working tree clean for L3)',
    freeze_commit='0e3c0fbda0', run_outputs_commit='6d9cb1674a', run_complete_commit='e783d12a5b', writeup_commit='1e42afc88d',
    artifact_sha256={p: sha(f'{L3}/{p}') for p in ('README.md', 'REPORT.typ', 'REPORT.pdf', 'PROTOCOL.md', 'DESK.md', 'FREEZE.sha256',
                                                    'model_predictions.json', 'run.log', 'analysis/summary.json',
                                                    'analysis/VERDICTS.md', 'data/SHA256SUMS', 'data_json/SHA256SUMS')},
    checks=checks, errata=errata,
    recomputed=dict(
        verdicts={k: dict(held=v['held'], n_ok=v['n_ok'], n_total=v['n_total']) for k, v in V.items()},
        verdicts_detail={k: {a: b for a, b in v.items() if a not in ('held', 'n_ok', 'n_total')} for k, v in V.items()},
        AA_B10000=ver10['verdicts']['AA'],
        readme_ranges=rr,
        P2_cells_ratio_lo_hi_model=ver['P2_cells'],
        k20_means={k: v['mean_D'] for k, v in ver['cells'].items() if k.startswith('20|')},
        model_max_rel_diff=ver['model_json_vs_own_closed_forms_max_rel_diff'],
        desk=dict(n_pass=desk['n_pass'], n_total=desk['n_total'], facts={k: v for k, v in desk.items() if k.endswith('_facts')},
                  Ep_e0=desk['Ep_e0'], kummer_own_labels=desk['kummer_own_labels']),
        reduced_census=dict(K_per_cell=cen['K_per_cell'], cells=cen['cells'], control_theorem1=cen['control_theorem1'],
                            spot_pointwise=cen['spot_pointwise'], huff5_rerun_4000=cen_r['cells']),
        frame_exact_replay={**ex1, **ex2},
        npoints_global_check='225 recorded cells: mean z -0.02, sd 0.998, |z|>2: 8, |z|>3: 0 (exact expectation 2^t * #finite-valued points/(p+1))',
        ep_minus_e_xframe_mean_difference_points={'k16': [4.23, 8.29, 15.72], 'k18': [3.92, 7.08, 15.84], 'k20': [4.45, 7.25, 14.75],
                                                  'N_over_2': [4.0, 8.0, 16.0], 't': [4, 5, 6]},
        C1_expected_code2='18000 * P(Bin(16, 0.502) <= 2) = 37.6; observed 34'),
    commands=[
        'cd /home/user/crypto && git rev-parse HEAD && git status --porcelain && git diff --quiet 1e42afc88d -- research/curve25519_wave7_untried_20261122/L3_translated_involution',
        'cd L3 && sha256sum -c FREEZE.sha256; for c in 0e3c0fbda0 e783d12a5b 1e42afc88d: git show $c:<path> | sha256sum (9 frozen files + PROTOCOL.md)',
        'cd L3/scripts && sha256sum -c ../data/SHA256SUMS; cd L3/data_json && sha256sum -c SHA256SUMS',
        'python3 desk_val.py desk_val.json            # own desk checks, k = 16 (theorems) and k = 16/18/20 (curve facts)',
        'python3 verdicts_val.py verdicts_val.json 2000   # all verdicts from raw npz + JSON-mirror equality',
        'python3 verdicts_val.py verdicts_val_b10k.json 10000',
        'python3 census_val.py census_val.json 1000   # reduced independent census, k = 16',
        'SALT=rerun2| ARMS=Ep.huff_x TS=5 CTRL=0 python3 census_val.py census_val_huff5_rerun.json 4000',
        'python3 exact_replay.py exact_replay.json; python3 exact_replay_odd.py exact_replay_odd.json   # frame-exact replays',
        'python3 npts_check.py; python3 readme_check.py'],
    validator_code=dict(location='/tmp/claude-0/-home-user/89ca8f69-b480-5417-bbfb-508279d3c6a2/scratchpad/val (session scratch; not committed)',
                        imports='stdlib + numpy + own vcore.py only (no study module imported)',
                        sha256={f: sha(f'{S}/{f}') for f in ('vcore.py', 'desk_val.py', 'verdicts_val.py', 'census_val.py',
                                                             'exact_replay.py', 'exact_replay_odd.py', 'npts_check.py',
                                                             'readme_check.py', 'make_report.py')},
                        note='exact_replay*.py mirror the protocol seed rule and draw order (and, to map recorded T labels, the '
                             "study's generator convention); bases and counts are the validator's own."),
    environment=dict(python='3.13.18', numpy='2.5.3', host='4 cores shared with other agents; one core used (nice 5)',
                     wall_s=dict(desk=16, verdicts=15, census=97, exact_replay=41, exact_replay_odd=206)),
    lessons_applied=['L-99ee952350b2 (index with []; study analyse.py and export_json.py --check also read real keys)',
                     'L-26895912385b (ratios recomputed from unrounded means; paired ratios not applicable: arms use independent seeds)',
                     'L-546d37ca7750 (not applicable: no band computed)',
                     'L-1a5a6b64022c (report written only when every check was complete)',
                     'L-8d86e8fc352c (E2: DESK.md is hash-pinned, so corrections go to README Errata or ERRATA.md)'],
)
json.dump(rep, open(f'{L3}/validator-report.json', 'w'), indent=1, ensure_ascii=False, default=str)
print('written', len(checks), 'checks', len(errata), 'errata')
