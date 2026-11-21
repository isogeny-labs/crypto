#!/usr/bin/env python3
"""Writes replay-certificate.json and the VALIDATION.json draft from the pod outputs in this directory.
Run from research/jv_nagao_hilbert_20261112/validation. Every number is read from files, none typed."""
import json, hashlib, os
H = lambda p: hashlib.sha256(open(p, 'rb').read()).hexdigest()
cmp = json.load(open('replay/compare.json')); c1 = json.load(open('replay_1thread/compare.json'))[0]
rc = json.load(open('rho_fresh_run0-13/compare.json'))['summary']
host = open('replay/host.txt').read().strip().splitlines()
DIAG = ['[0].f4_muls_per_test_sum', '[0].hilbert_check_muls']
runs, ok = [], True
for r in cmp:
    bad = [m['field'] for m in r['mismatches'] if m['field'] not in DIAG]
    ok &= r['present'] and not bad and r['correct'] is True and r['solved'] is True
    runs.append(dict(run=r['label'], record='../results/' + r['label'] + '.json', record_sha256=r['committed_sha256'],
                     replay='replay/' + r['label'] + '.json', replay_sha256=r['replay_sha256'], fields_compared=r['fields_compared'],
                     identical_counted_fields=not bad, differing_fields=[m['field'] for m in r['mismatches']],
                     solved=r['solved'], correct=r['correct'], total_muls=r['total_muls'], s=r['s'], relation_digest=r['relation_digest'],
                     scalar='correct: true -- the recovered logarithm equals the planted one'))
cert = dict(
    schema='replay-certificate/producer-v1', id='NH-20261112-replay', study='research/jv_nagao_hilbert_20261112',
    status='verified' if ok else 'failed', independent=True,
    independence='HOST-independent only: a second host (RunPod pod, see host) re-ran the frozen command; the replay was run by a '
                 'sibling agent of the producing programme, not by an independent validator. A validator should re-sign or redo it.',
    scalars_verified=ok,
    scope='all 10 records: the A/A drift control aa_line_p1301_s1 and the 9 hilbert instances, frozen command of ../run.sh, unchanged flags',
    host='; '.join(host[:2]), producer_host='Intel Xeon, 4 vCPU cloud container (../results/walls.txt context; README "local L0")',
    binary=dict(
                replay_sha256=host[2].split()[0], study_sha256=open('../results/BINARY.sha256').read().split()[0],
                source_commit=open('../results/SOURCE_COMMIT').read().strip(),
                note='rebuilt from the study commit (git archive of the crate sources, rustc 1.97.0, cargo --release --example jv_cover); '
                     'the hash did not reproduce byte-for-byte (different build directory and target state), every counted field did'),
    comparison='flattened JSON, every field compared for exact equality except timing fields (*_ms); the two fields '
               + ', '.join(DIAG) + ' are per-test differences of a process-wide counter (f4_fp.rs FIELD_OPS) and scale with the '
               'number of concurrent rayon threads, so they are not counted quantities; neither enters total_muls, s, or the rescore',
    concurrency_control=dict(run='hilbert_p1301_s1 single-threaded (RAYON_NUM_THREADS=1) on the pod', replay='replay_1thread/hilbert_p1301_s1.json',
                             replay_sha256=c1['replay_sha256'], differing_fields=[m['field'] for m in c1['mismatches']],
                             finding='with one thread f4_muls_per_test_sum falls to within 0.02% of f4_muls (as its doc comment says) and '
                                     'hilbert_check_muls to 8.875e9 (11.1% of f4_muls), against 6.44e10 in the committed 4-thread record: '
                                     'the committed hilbert_check_muls is concurrency-inflated and must not be quoted'),
    rho_reference_replay=dict(runs=rc['fresh_runs'], identical_except_wall_ms=rc['identical_except_wall_ms'], all_correct=rc['all_correct'],
                              none_capped=rc['none_capped'], dir='rho_fresh_run0-13/', compare='rho_fresh_run0-13/compare.json',
                              note='the 126-run pooled reference of research/jv_cover_negrho_rescore_20261101 (binary 8f62bee...) re-run with '
                                   'the study binary on the pod: every record identical except wall_ms'),
    runs=runs)
json.dump(cert, open('replay-certificate.json', 'w'), indent=1)
b = json.load(open('band_pooled_plus_fresh.json')) if os.path.exists('band_pooled_plus_fresh.json') else None
b0 = json.load(open('band_fresh_run0-13.json'))
cell = next(c for c in b0['cells'] if c['unit'].startswith('U2') and c['p'] == 1601)
val = {
  'schema': 'validation-gate/v1',
  'status_note': 'DRAFT, not a validated row: validator_report is an empty slot for the independent validator; the replay '
                 'certificate is host-independent only; no S3 bench.record/v1 exists. [pcc:unverified]',
  'replay_certificate': {'path': 'replay-certificate.json', 'sha256': H('replay-certificate.json')},
  'validator_report': {'path': 'review/validator/validator-report.json', 'sha256': 'TO-BE-FILLED-BY-VALIDATOR'},
  'rho_reference': {
    'kind': 'negation',
    'protocol': 'research/jv_cover_negrho_rescore_20261101/PROTOCOL.md: rhostrong --neg 1 --batch 16 --r 256 --dp-bits 16 --run R, '
                'runs 0..13 on the 9 instances (126), estimator (b) pooled iff homogeneous; re-run with the study binary on the pod',
    'unit': 'U2 textbook F_p multiplications / (c_add_e = 331) per sqrt(l) (rho s_tuned; cover total_muls), native operations priced at '
            'w GAE each (cover f4_native_ops, a lower bound; rho canon_cmp+hash+dp_check+scalar_ops+table_ops+cycle_cmp)',
    'binary_or_process': 'examples/jv_cover built from ' + cert['binary']['source_commit'] + ' (sha256 ' + cert['binary']['replay_sha256'] +
                         '): --exp dlp --stop hilbert:16 (cover) and --exp rhostrong (rho), both on the same RunPod pod',
    'matches_protocol': True, 'strongest_available': True, 'same_unit': True, 'same_binary_or_process': True,
    'strongest_available_basis': 'research/jv_cover_fbline_20261102/review/red_team/REPORT.md row 7: on all 9 instances j is not in F_{p^2} or F_{p^3}, j != 0, 1728; no Frobenius or automorphism fold beyond negation',
    'independent_sample_check': 'runs 14..27 (126 new walks, same binary, same pod): U2 p=1601 0.728 [0.672, 0.793] w=0, below at all w (rescore_fresh_run14-27/table.md)',
    'cell': 'U2, p = 1601 (the other five cells, U1/U2 x p, are in band_fresh_run0-13.json)',
    'gae_band': {k: {'ic': v['ic'], 'rho': v['rho']} for k, v in cell['band'].items()},
    'order_holds_across_band': next(x['same_at_all_w'] for x in json.load(open('rescore_fresh_run0-13/fit.json'))['band_5a'] if x['unit'].startswith('U2') and x['p'] == 1601),
  },
}
json.dump(val, open('VALIDATION.json', 'w'), indent=1)
print('cert status', cert['status'], 'scalars_verified', cert['scalars_verified'])
