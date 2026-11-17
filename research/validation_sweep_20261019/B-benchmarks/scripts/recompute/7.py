import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from swb_common import *  # noqa

ROW = 7
SLUG = '20261017-volcano-repaired-2935'
D = 'research/full_volcano_graphwide_ic_20261017/execution/recovery-20261017T1450Z'
README = D + '/README.md'
INV = D + '/inventory.json'
SUMM = D + '/summary.json'
RAW_CANDS = [D + '/combined-cli-site-network-failed-browser.log',
             D + '/combined-release-network-denied.log',
             D + '/inventory.json',
             D + '/resumed-launch-initial.json',
             D + '/resumed-launch-verified.json',
             D + '/validation.json']

inv = json.load(open(abspath(INV)))
summ = json.load(open(abspath(SUMM)))
caps = inv['capsules']
vertices = len(caps)
unique_ids = len({c['id'] for c in caps})
strata_sum = sum(g['strata_audited'] for g in inv['graphs'])
ot = inv['outcome_totals']
cap_succ = sum(c['successful_replays'] for c in caps)
cap_exh = sum(c['exhausted_replays'] for c in caps)
timeout_derived = inv['measured_records'] - ot['verified'] - ot['exhausted']
interrupted = inv['states'].get('interrupted')

F_inv = [INV]
F_sum = [SUMM]
claims = [
    C('2935', vertices, 'len(capsules) in inventory.json (%d unique ids); vertices_audited = %s' %
      (unique_ids, inv['vertices_audited']), F_inv),
    C('51', inv['strata_audited'], 'strata_audited in inventory.json; sum over graphs = %d' % strata_sum, F_inv),
    C('46960', inv['measured_records'], 'measured_records in inventory.json; summary.json = %s' %
      summ['measured_records'], F_inv + F_sum),
    C('30270', ot['verified'], 'outcome_totals.verified in inventory.json; summary.json verified_records = %s' %
      summ['verified_records'], F_inv + F_sum),
    C('14626', ot['exhausted'], 'outcome_totals.exhausted in inventory.json; top-level exhausted_replays = %s' %
      inv['exhausted_replays'], F_inv),
    C('2064', ot['timeout'], 'outcome_totals.timeout in inventory.json; derived as measured - verified - exhausted = %d' %
      timeout_derived, F_inv),
    C('1530557', summ['relations_found'], 'relations_found in summary.json (relation_observations %s)' %
      summ['relation_observations'], F_sum),
    C('71296000', summ['decomposition_calls'], 'decomposition_calls in summary.json', F_sum),
    C('1', interrupted, 'states.interrupted in inventory.json', F_inv),
]

notes = ('Counts come from the committed inventory.json (capsules, outcome_totals, states) and summary.json (in the same '
         'directory; not in the card list, used for relations and calls). Per-record outcomes are not in the repo: '
         'the README names the complete frozen checkpoint and raw evidence on external storage, so the outcome counts '
         'are the inventory aggregates. Per-capsule sums: successful_replays %d, exhausted_replays %d. Timeout counters '
         'are unknown per record in the README wording; the timeout count is the inventory aggregate. '
         'relation_trials %s, verified_records %s.' % (cap_succ, cap_exh, summ['relation_trials'], summ['verified_records']))

offrepo_files, off, aa, iso = scan_study([README])
store = store_refs(offrepo_files)
emit(ROW, SLUG, claims, no_identical(), off + store, aa, iso,
     git_add(README), first_raw_date([p for p in RAW_CANDS if not p.endswith('.yaml')]),
     'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/7.py', notes)
