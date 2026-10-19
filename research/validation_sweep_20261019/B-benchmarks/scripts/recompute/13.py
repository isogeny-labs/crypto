#!/usr/bin/env python3
# Row 13 recompute (20261017-volcano-ic-campaign). Reads committed files only.
import json, subprocess

R = '/home/user/crypto'
SCRIPT = 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/13.py'
X = 'research/full_volcano_graphwide_ic_20261017/execution/'
STUDY = X + 'README.md'
PROTO = X + 'PROTOCOL.md'
SURV = X + 'survey-99-summary-v2/summary.json'
REL_PM = X + 'evidence/release-pilot-match.json'
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


import collections
s = json.load(open(R + '/' + SURV))
camp = s['campaigns']
cov = [x for c in camp for x in c['coverage']]
audited_vertices = sum(x['vertices'] for x in cov if x['state'] == 'audited')
strata = set()
for c in camp:
    for x in c['stratum_coverage']:
        if x['state'] == 'audited':
            strata.add((x['graph'], x['conductor']))
ot = s['outcome_totals']
rp = json.load(open(R + '/' + REL_PM))
cls = collections.Counter(c['classification'] for c in rp['checks'])
completed = cls.get('completed_count_comparison', 0)

claims = [
    {'claimed': '99 vertices', 'computed': f'sum of coverage vertices with state audited={audited_vertices}',
     'definition': 'campaigns[].coverage[] vertices with state=="audited" in survey-99-summary-v2/summary.json',
     'files': [SURV], 'status': 'match' if audited_vertices == 99 else 'mismatch'},
    {'claimed': 'all 51 strata', 'computed': f'distinct audited (graph, conductor) in stratum_coverage={len(strata)}',
     'definition': 'distinct (graph, conductor) with state=="audited" in campaigns[].stratum_coverage[]',
     'files': [SURV], 'status': 'match' if len(strata) == 51 else 'mismatch'},
    {'claimed': '1,584 attempts', 'computed': f'measured_records={s["measured_records"]}',
     'definition': 'measured_records in survey-99-summary-v2/summary.json',
     'files': [SURV], 'status': 'match' if s['measured_records'] == 1584 else 'mismatch'},
    {'claimed': '1,065 verified, 422 exhausted, 97 timeouts',
     'computed': f'outcome_totals verified={ot["verified"]} exhausted={ot["exhausted"]} timeout={ot["timeout"]}',
     'definition': 'outcome_totals in survey-99-summary-v2/summary.json',
     'files': [SURV], 'status': 'match' if (ot['verified'], ot['exhausted'], ot['timeout']) == (1065, 422, 97) else 'mismatch'},
    {'claimed': '57,176 relations', 'computed': f'relations_found={s["relations_found"]}',
     'definition': 'relations_found in survey-99-summary-v2/summary.json',
     'files': [SURV], 'status': 'match' if s['relations_found'] == 57176 else 'mismatch'},
    {'claimed': '2,223,204 source trials', 'computed': f'relation_trials={s["relation_trials"]}',
     'definition': 'relation_trials in survey-99-summary-v2/summary.json',
     'files': [SURV], 'status': 'match' if s['relation_trials'] == 2223204 else 'mismatch'},
    {'claimed': '2,075,145 actual oracle calls', 'computed': f'decomposition_calls={s["decomposition_calls"]}',
     'definition': 'decomposition_calls in survey-99-summary-v2/summary.json (headline "actual oracle calls")',
     'files': [SURV], 'status': 'match' if s['decomposition_calls'] == 2075145 else 'mismatch'},
    {'claimed': '56 exact completed comparisons', 'computed': f'checks classified completed_count_comparison={completed} of checks={len(rp["checks"])}',
     'definition': 'count of checks[].classification == "completed_count_comparison" in evidence/release-pilot-match.json',
     'files': [REL_PM], 'status': 'match' if completed == 56 else 'mismatch'},
    {'claimed': 'four censored', 'computed': f'censored_pairs_not_compared={rp["censored_pairs_not_compared"]}',
     'definition': 'censored_pairs_not_compared in evidence/release-pilot-match.json',
     'files': [REL_PM], 'status': 'match' if rp['censored_pairs_not_compared'] == 4 else 'mismatch'},
]

out = {
    'row_no': 13,
    'slug': '20261017-volcano-ic-campaign',
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
              ' (first listed candidate). Completed-comparison and censored counts read from evidence/release-pilot-match.json, '
              'which is not in the card candidate list. Off-repo refs are the Drive links in the same README.'),
}
print(json.dumps(out, ensure_ascii=False))
