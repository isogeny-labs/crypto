#!/usr/bin/env python3
# Row 11 recompute (20261017-volcano-repaired-qa). Reads committed files only.
import json, subprocess, collections

R = '/home/user/crypto'
SCRIPT = 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/11.py'
X = 'research/full_volcano_graphwide_ic_20261017/execution/'
STUDY = X + 'WATCHDOG_REPAIR_PROTOCOL.md'
QA = X + 'repaired-qa-summary-v2/summary.json'
PM = X + 'watchdog-repair/pilot-match.json'
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


qa = json.load(open(R + '/' + QA))
pm = json.load(open(R + '/' + PM))
ot = qa['outcome_totals']
cls = collections.Counter(c['classification'] for c in pm['checks'])
completed = cls.get('completed_count_comparison', 0)
censored_checks = cls.get('censored_not_count_comparable', 0)

claims = [
    {'claimed': '60 attempts', 'computed': f'measured_records={qa["measured_records"]}',
     'definition': 'measured_records in repaired-qa-summary-v2/summary.json (not in the card candidate list; the only file with the 60-attempt aggregate)',
     'files': [QA], 'status': 'match' if qa['measured_records'] == 60 else 'mismatch'},
    {'claimed': '40 verified, 18 exhausted, 2 timeout',
     'computed': f'outcome_totals verified={ot["verified"]} exhausted={ot["exhausted"]} timeout={ot["timeout"]}',
     'definition': 'outcome_totals in repaired-qa-summary-v2/summary.json',
     'files': [QA], 'status': 'match' if (ot['verified'], ot['exhausted'], ot['timeout']) == (40, 18, 2) else 'mismatch'},
    {'claimed': 'Exact baseline completed pairs 58',
     'computed': f'checks with classification completed_count_comparison={completed} of checks={len(pm["checks"])}',
     'definition': 'count of checks[].classification == "completed_count_comparison" in watchdog-repair/pilot-match.json',
     'files': [PM], 'status': 'match' if completed == 58 else 'mismatch'},
    {'claimed': 'censored pairs 2',
     'computed': f'censored_pairs_not_compared={pm["censored_pairs_not_compared"]}; checks classified censored_not_count_comparable={censored_checks}',
     'definition': 'censored_pairs_not_compared field and checks[].classification == "censored_not_count_comparable" in watchdog-repair/pilot-match.json',
     'files': [PM], 'status': 'match' if pm['censored_pairs_not_compared'] == 2 else 'mismatch'},
]

out = {
    'row_no': 11,
    'slug': '20261017-volcano-repaired-qa',
    'claims': claims,
    'identical': {'claimed': False, 'n_compared': None, 'n_equal': None, 'n_differ': None,
                  'files': [], 'status': 'not_claimed'},
    'offrepo_refs': [],
    'aa_lines': [line(STUDY, 21), line(STUDY, 23)],
    'isolation_lines': [],
    'protocol_git_date': gd(STUDY),
    'first_raw_git_date': gd(INV),
    'script': SCRIPT,
    'notes': ('Study file is WATCHDOG_REPAIR_PROTOCOL.md (protocol text only; lines 21 and 23 are protocol text, not a result). '
              'Claimed raw candidates were listed as the release/survey summaries; the 60-attempt and 58/2 pair aggregates were read from '
              'repaired-qa-summary-v2 and watchdog-repair/pilot-match.json. First raw file used: recovery-20261017T1450Z/inventory.json (first listed candidate).'),
}
print(json.dumps(out, ensure_ascii=False))
