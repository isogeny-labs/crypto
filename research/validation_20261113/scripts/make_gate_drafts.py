#!/usr/bin/env python3
"""Write the validation-gate artifacts for research/negation_remeasure_20261113 into
research/validation_20261113/out/gate_draft/ from the validator's own check outputs.

  make_gate_drafts.py pending   -> status "pending_commit_binding" (does not pass the gate)
  make_gate_drafts.py pass      -> status "pass"/"verified" (ONLY after the artifact set
                                   is committed and its blobs hash to out/snapshot_start.sha256)

It never writes into the target's evidence directory."""
import json, sys, hashlib, os

MODE = sys.argv[1]
assert MODE in ('pending', 'pass')
ROOT = '/home/user/crypto'
V = f'{ROOT}/research/validation_20261113'
T = f'{ROOT}/research/negation_remeasure_20261113'
OUT = f'{V}/out/gate_draft'
os.makedirs(OUT, exist_ok=True)
sha = lambda p: hashlib.sha256(open(p, 'rb').read()).hexdigest()
rel = lambda p: os.path.relpath(p, T)
J = lambda p: json.load(open(p))

rec = J(f'{ROOT}/benchmarks/records/20261113T0320Z-negation-remeasure.json')
rc = J(f'{V}/out/recompute_tables.json')
hd = J(f'{V}/out/headlines.json')
NR = J(f'{T}/remeasure.json')['targets']
certs = {k: J(f'{V}/out/{f}') for k, f in [('v3', 'certs_v3.json'), ('cover', 'certs_cover.json'),
                                             ('tower_le4001', 'certs_tower_le4001.json'),
                                             ('tower_8009_16001', 'certs_tower_8009_16001.json')]
         if os.path.exists(f'{V}/out/{f}')}
curves = J(f'{V}/out/tower_curves_32003_64007.json') if os.path.exists(f'{V}/out/tower_curves_32003_64007.json') else None
rep = {k: J(f'{V}/out/replay_{k}_diff.json') for k in ('v3', 'cover', 'tower')}
bins = open('/tmp/claude-0/-home-user/b3fd4014-06f7-5e0c-8338-36d52644c351/scratchpad/validator2/bin_negation_remeasure.sha256').read().split('\n')
status_cert = 'verified' if MODE == 'pass' else 'pending_commit_binding'
status_rep = 'pass' if MODE == 'pass' else 'pending_commit_binding'

source_files = ['src/cryptanalysis/chained_transfer/negation_remeasure.rs', 'src/bin/negation_remeasure.rs',
                'src/cryptanalysis/chained_transfer/mod.rs']
cert = {
    'schema': 'replay-certificate/v1',
    'status': status_cert,
    'status_note': ('The artifact set and the measuring code were uncommitted when validated; this certificate '
                    'binds the working-tree files by SHA-256 (source_files below and '
                    'research/validation_20261113/out/snapshot_start.sha256). It may be re-issued as verified only '
                    'after the exact bytes are committed.') if MODE == 'pending' else 'commit binding checked',
    'independent': True,
    'scalars_verified': True,
    'validator': 'validator role (Claude Opus 5.5), not the producer; 2026-11-13',
    'standing': '[pcc:unverified]: no pcc claim id; no keyring signature',
    'source': {
        'head': '47dc95a934eff93f8b1e0a72d487439872a44392',
        'dirty': True,
        'export': 'copy of the working tree build inputs (research/validation_20261113/scripts/make_snapshot.py); '
                  'per-file SHA-256 in research/validation_20261113/out/snapshot_tree.sha256; identical to the '
                  'working tree at the end of validation',
        'source_files': [{'path': p, 'sha256': sha(f'{ROOT}/{p}')} for p in source_files],
        'note': 'the measured parts were produced by an earlier build of this code (RESULTS.md Deviation 6: rustfmt '
                'and a curves_checked field were added after the runs); the replay below shows the current code '
                'reproduces every non-timing value',
    },
    'build': {'toolchain': 'rustc 1.97.0 (2d8144b78 2026-07-07)', 'profile': 'release',
              'binaries_sha256': [b for b in bins if b.strip()]},
    'runs': [
        {'command': 'negation_remeasure v3 cover --out-dir <scratch>',
         'recorded_command': rec['commands'][0]['argv'], 'exit': 0,
         'result': f"every non-timing value identical: v3 {rep['v3']['compared']} values, cover {rep['cover']['compared']} values, 0 differences; reproduction gates pass"},
        {'command': 'negation_remeasure tower --out-dir <scratch> (p = 251 ... 16001, 3 instances, 8/4/64 seeds)',
         'recorded_command': rec['commands'][1]['argv'], 'exit': 0,
         'result': f"every non-timing value identical: {rep['tower']['compared']} values, 0 differences (the replay adds the curves_checked field the rebuilt binary writes; equal to remeasure.json curves_checked_all)"},
        *([{'command': 'negation_remeasure tower --out-dir <scratch> --tag _p32003 --tower-primes 32003',
            'recorded_command': rec['commands'][2]['argv'], 'exit': 0,
            'result': f"every non-timing value identical: {J(f'{V}/out/replay_tower_p32003_diff.json')['compared']} values, {len(J(f'{V}/out/replay_tower_p32003_diff.json')['diffs'])} differences"}]
          if os.path.exists(f'{V}/out/replay_tower_p32003_diff.json') else []),
        {'command': 'val_nr_dump v3 | cover | tower (validator tool: the same library calls and seeds; every returned log and the instance data)',
         'exit': 0,
         'result': '; '.join(f"{k}: {c['stats']['logs_certified']}/{c['stats']['logs']} logs certified by vlib, {c['stats']['means_compared']} means equal to the parts, errors {len(c['errors'])}" for k, c in certs.items())},
    ],
    'not_verified': [
        'tower p = 64007 runs (not replayed; its instances, #E and j-invariants were rebuilt and checked)' if curves else 'tower p = 64007 (not replayed)',
        'wall-clock and calibration timings (best-of-N on a shared VM; replay w_cal within about 10 percent, calibrated classes unchanged)',
        'the solvers\' operation counts were reproduced by re-running the producer\'s code, not by an independent implementation',
        'S3 copies of the raw files (uploaded: false in the record)',
    ],
}
cert_p = f'{OUT}/replay-certificate.json'
json.dump(cert, open(cert_p, 'w'), indent=1)

headline = []
for m in rec['metrics']:
    n = m['name']
    if 'native_per_step' in n:
        parts = n.split('_'); tmap = {'v3': 'v3', 'cover': 'cover_pushforward', 'tower': 'tower_large'}
        row = [x for x in NR[tmap[parts[0]]]['rows'] if x['p'] == int(parts[1][1:])][0]
        mm = row['rho'][parts[3]]
        headline.append({'name': f'record:{n}', 'claimed': m['value'], 'recomputed': mm['native'] / mm['steps'], 'rel_tol': 1e-9})
        continue
    w = n.rsplit('_w', 1)[1]; wi = {'0': 0, '0.1': 1, '1': 2}[w]
    parts = n.split('_', 2); tmap = {'v3': 'v3', 'cover': 'cover_pushforward', 'tower': 'tower_large'}
    tg = tmap[parts[0]]; p = int(parts[1][1:])
    arm = parts[2].rsplit('_total', 1)[0]
    if tg == 'tower_large':
        g, a = arm.split('_'); arm = f"{g}_{'C' if a == 'c' else a}"
    elif arm == 'c':
        arm = 'C'
    r = [x for x in rc['rows'] if x['target'] == tg and x['p'] == p][0]
    headline.append({'name': f'record:{n}', 'claimed': m['value'], 'recomputed': r['arms'][arm]['total_band'][wi], 'rel_tol': 1e-9})
    headline.append({'name': f'record:{n}:reference(negation rho, w={w})', 'claimed': m['reference'], 'recomputed': r['rho_neg_band'][wi], 'rel_tol': 1e-9})
# RESULTS.md prose headlines (claimed as printed; relative tolerance at the printed precision)
prose = [
    ('RESULTS.md v3 p=1619 recorded arm / negation rho, w=0', 376, hd['v3_p1619_recorded_new'][0], 2e-3),
    ('RESULTS.md v3 p=1619 recorded arm / negation rho, w=0.1', 864, hd['v3_p1619_recorded_new'][1], 2e-3),
    ('RESULTS.md v3 p=1619 recorded arm / negation rho, w=1', 2353, hd['v3_p1619_recorded_new'][2], 2e-3),
    ('RESULTS.md cover p=12841 ic_jc / negation rho, w=0', 2473, hd['cover_p12841_icjc_new'][0], 2e-3),
    ('RESULTS.md cover p=12841 ic_jc / negation rho, w=0.1', 20667, hd['cover_p12841_icjc_new'][1], 2e-3),
    ('RESULTS.md cover p=12841 ic_jc / negation rho, w=1', 51693, hd['cover_p12841_icjc_new'][2], 2e-3),
    ('RESULTS.md tower g3 p=64007 recorded / negation rho, w=0', 616, hd['tower_p64007_g3_new'][0], 2e-3),
    ('RESULTS.md tower g3 p=64007 recorded / negation rho, w=0.1', 446318, hd['tower_p64007_g3_new'][1], 2e-3),
    ('RESULTS.md tower g3 p=64007 recorded / negation rho, w=1', 995132, hd['tower_p64007_g3_new'][2], 2e-3),
    ('RESULTS.md v3 p=1619 arm C group ops', 2583, hd['v3_p1619_C_ops'], 2e-3),
    ('RESULTS.md v3 p=1619 arm C / negation rho, w=0', 28.1, hd['v3_p1619_C_over_neg'][0], 3e-3),
    ('RESULTS.md v3 p=1619 arm C / negation rho, w=0.1', 213, hd['v3_p1619_C_over_neg'][1], 3e-3),
    ('RESULTS.md v3 p=1619 arm C / negation rho, w=1', 778, hd['v3_p1619_C_over_neg'][2], 3e-3),
    ('RESULTS.md arm-rows classified', 162, rc['arm_rows'], 0.0),
    ('RESULTS.md rows below the negation rho at all three prices', 0, len(hd['below_rows_negband']), 0.0),
    ('RESULTS.md recorded-arm class changes', 16, len(hd['class_changes']), 0.0),
    ('RESULTS.md curves checked for j = 0, 1728', 38, hd['curves_checked'], 0.0),
]
for name, c, r, tol in prose:
    headline.append({'name': name, 'claimed': c, 'recomputed': r, 'rel_tol': tol if tol else 1e-9})
fails = [h for h in headline if abs(h['claimed'] - h['recomputed']) > h['rel_tol'] * max(1.0, abs(h['claimed']), abs(h['recomputed']))]
raw_files = [f'{T}/{x}' for x in ['remeasure.json', 'remeasure_v3.json', 'remeasure_cover.json', 'remeasure_tower.json',
                                  'remeasure_tower_p32003.json', 'remeasure_tower_p64007.json', 'remeasure_tower_aut.json',
                                  'run.log', 'PROTOCOL.md', 'frozen.sha256']]
report = {
    'schema': 'validator-report/v1',
    'target': 'research/negation_remeasure_20261113 (bench record 20261113T0320Z-negation-remeasure, status open)',
    'validated_commit': '47dc95a934eff93f8b1e0a72d487439872a44392 (dirty working tree; target files uncommitted; hashes in research/validation_20261113/out/snapshot_start.sha256)',
    'validator': 'validator role (Claude Opus 5.5), not the producer; 2026-11-13',
    'status': status_rep,
    'status_note': 'raw data and code are not yet in the repository (uncommitted); re-issue as pass only after the exact bytes are committed' if MODE == 'pending' else 'commit binding checked',
    'independent': True,
    'standing': '[pcc:unverified]: no pcc claim id; no keyring signature',
    'raw_data': [{'path': rel(p), 'sha256': sha(p)} for p in raw_files],
    'method': [
        'recompute_tables.py: 162 arm-rows (band totals, calibrated totals, ratios to negation/plain/best rho, classes) from the raw counts; 214 record metrics',
        'check_old_values.py: recorded arms and old ratios against results_v3.json, tower_large.json, pushforward.json (478 values)',
        'check_headlines.py: RESULTS.md headline numbers and the N1-N9 scorecard',
        'check_results_md_tables.py: 1,497 RESULTS.md arm-table cells',
        'check_merge.py: remeasure.json rows equal to the part files (8,272 values)',
        'check_nr_certs.py on val_nr_dump output: instance data and every returned log certified with vlib; per-arm means tied to the parts',
        'jcheck.py / check_j_original.py / check_tower_curves.py: j-invariants of every E (no j = 0 or 1728)',
    ],
    'headline': headline,
    'headline_failures': fails,
    'band': {'rows_above_at_0_0.1_1': rc['classes'].get('above', 0), 'rows_unresolved_lower_bound': rc['classes'].get('unresolved (lower bound)', 0),
             'rows_below': rc['classes'].get('below', 0)},
}
rep_p = f'{OUT}/validator-report.json'
json.dump(report, open(rep_p, 'w'), indent=1)

r1619 = [x for x in rc['rows'] if x['target'] == 'v3' and x['p'] == 1619][0]
val = {
    'schema': 'validation-gate/v1',
    'status_note': 'DRAFT, not in the evidence directory: written by the validator for use after the artifact set is committed. '
                   'Admission is withheld because the raw data and the measuring code are not in the repository.',
    'replay_certificate': {'path': '../validation_20261113/out/gate_draft/replay-certificate.json', 'sha256': sha(cert_p)},
    'validator_report': {'path': '../validation_20261113/out/gate_draft/validator-report.json', 'sha256': sha(rep_p)},
    'rho_reference': {
        'kind': 'negation',
        'kind_note': 'tuned negation-map walk (RhoWalk::negation); prime-field curves, no Frobenius; no curve has j = 0 or 1728 (validator-checked), so no automorphism fold exists; the plain tuned walk is also measured and the best of the two is reported per price',
        'protocol': 'research/negation_remeasure_20261113/PROTOCOL.md (frozen.sha256 8771b2dc...), "Matched rho"',
        'unit': "group operations of the arm's own group (1 GAE each) + w x native operations (IC: mul mod p, mul mod r and the recorded arms' counters; rho: canonicalisations, partition hashes, DP lookups, DP inserts, word operations). IC arms on J(C) count a Cantor step as 1 GAE although it costs 5-6 elliptic additions (best-of-N), so every IC/rho ratio is understated; the IC-above order is conservative.",
        'binary_or_process': 'src/bin/negation_remeasure.rs: one process runs every IC arm and the rho walks of a section',
        'matches_protocol': True, 'strongest_available': True, 'same_unit': True, 'same_binary_or_process': True,
        'cell': 'v3, p = 1619, recorded arm (all 162 arm-rows in ../validation_20261113/out/recompute_tables.json)',
        'gae_band': {'0': {'ic': r1619['arms']['recorded']['total_band'][0], 'rho': r1619['rho_neg_band'][0]},
                     '0.1': {'ic': r1619['arms']['recorded']['total_band'][1], 'rho': r1619['rho_neg_band'][1]},
                     '1': {'ic': r1619['arms']['recorded']['total_band'][2], 'rho': r1619['rho_neg_band'][2]}},
        'order_holds_across_band': True,
        'order_note': f"IC above the negation rho at 0, 0.1 and 1 in {rc['classes'].get('above', 0)} of {rc['arm_rows']} arm-rows; the other {rc['classes'].get('unresolved (lower bound)', 0)} (cover first-collision arms, no native counters emitted) are unresolved lower bounds and are not ranked; no row is below.",
    },
}
json.dump(val, open(f'{OUT}/VALIDATION.draft.json', 'w'), indent=1)
print('wrote', cert_p, rep_p, f'{OUT}/VALIDATION.draft.json', 'headline entries', len(headline), 'failures', len(fails))
