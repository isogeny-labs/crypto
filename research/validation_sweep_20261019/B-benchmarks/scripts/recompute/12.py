#!/usr/bin/env python3
# Row 12 recompute (20261017-volcano-ic-release-stopped). Reads committed files only.
import json, subprocess, collections, csv

R = '/home/user/crypto'
SCRIPT = 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/12.py'
X = 'research/full_volcano_graphwide_ic_20261017/execution/'
STUDY = X + 'README.md'
PROTO = X + 'PROTOCOL.md'
REL = X + 'release-70-summary-v2/summary.json'
INV = X + 'recovery-20261017T1450Z/inventory.json'


def gd(path):
    out = subprocess.run(['git', '-C', R, 'log', '--diff-filter=A', '--format=%H %cI', '--', path],
                         capture_output=True, text=True).stdout.strip()
    return out.replace('\n', ';')


def line(path, n):
    with open(R + '/' + path, encoding='utf-8') as f:
        for i, t in enumerate(f, 1):
            if i == n:
                return f'{path}:{n}: {t.rstrip()}'
    return f'{path}:{n}: <missing>'


s = json.load(open(R + '/' + REL))
camp = s['campaigns']
cov = [x for c in camp for x in c['coverage']]
audited_vertices = sum(x['vertices'] for x in cov if x['state'] == 'audited')
failed_vertices = sum(x['vertices'] for x in cov if x['state'] == 'failed')
strata = set()
for c in camp:
    for x in c['stratum_coverage']:
        if x['state'] == 'audited':
            strata.add((x['graph'], x['conductor']))
ot = s['outcome_totals']

claims = [
    {'claimed': '70 audited vertices', 'computed': f'sum of coverage vertices with state audited={audited_vertices}',
     'definition': 'campaigns[].coverage[] vertices where state=="audited" in release-70-summary-v2/summary.json',
     'files': [REL], 'status': 'match' if audited_vertices == 70 else 'mismatch'},
    {'claimed': 'all 51 strata', 'computed': f'distinct audited (graph, conductor) in stratum_coverage={len(strata)}; campaigns[].strata_audited={[c["strata_audited"] for c in camp]}',
     'definition': 'distinct (graph, conductor) with state=="audited" in campaigns[].stratum_coverage[]',
     'files': [REL], 'status': 'match' if len(strata) == 51 else 'mismatch'},
    {'claimed': '1,120 accepted attempts', 'computed': f'measured_records={s["measured_records"]}',
     'definition': 'measured_records in release-70-summary-v2/summary.json',
     'files': [REL], 'status': 'match' if s['measured_records'] == 1120 else 'mismatch'},
    {'claimed': '810 verified, 280 exhausted, 30 timeouts',
     'computed': f'outcome_totals verified={ot["verified"]} exhausted={ot["exhausted"]} timeout={ot["timeout"]}',
     'definition': 'outcome_totals in release-70-summary-v2/summary.json',
     'files': [REL], 'status': 'match' if (ot['verified'], ot['exhausted'], ot['timeout']) == (810, 280, 30) else 'mismatch'},
    {'claimed': 'one failed capsule', 'computed': f'sum of coverage vertices with state failed={failed_vertices}',
     'definition': 'campaigns[].coverage[] vertices where state=="failed" (the 16-record capsule size is not checked by this script)',
     'files': [REL], 'status': 'match' if failed_vertices == 1 else 'mismatch'},
    {'claimed': '42,746 relations', 'computed': f'relations_found={s["relations_found"]}',
     'definition': 'relations_found in release-70-summary-v2/summary.json',
     'files': [REL], 'status': 'match' if s['relations_found'] == 42746 else 'mismatch'},
    {'claimed': '1,383,570 actual calls', 'computed': f'decomposition_calls={s["decomposition_calls"]}',
     'definition': 'decomposition_calls in release-70-summary-v2/summary.json (headline "actual calls")',
     'files': [REL], 'status': 'match' if s['decomposition_calls'] == 1383570 else 'mismatch'},
]

out = {
    'row_no': 12,
    'slug': '20261017-volcano-ic-release-stopped',
    'claims': claims,
    'identical': {'claimed': False, 'n_compared': None, 'n_equal': None, 'n_differ': None,
                  'files': [], 'status': 'not_claimed'},
    'offrepo_refs': [
        line(STUDY, 12), line(STUDY, 34), line(STUDY, 162), line(STUDY, 202), line(STUDY, 220)],
    'aa_lines': [line(STUDY, 39), line(STUDY, 40), line(STUDY, 49), line(STUDY, 53)],
    'isolation_lines': [line(STUDY, 69), line(STUDY, 82), line(STUDY, 189)],
    'protocol_git_date': gd(PROTO),
    'first_raw_git_date': gd(INV),
    'script': SCRIPT,
    'notes': ('Protocol path used: ' + PROTO + ' (the row does not name one). First raw file used: ' + INV +
              ' (first listed candidate); aggregates read from release-70-summary-v2/summary.json, the first candidate after inventory. '
              'The A/A line 49 refers to the optimized campaign in the same README. Off-repo refs are the Drive links in the same README.'),
}
print(json.dumps(out, ensure_ascii=False))
