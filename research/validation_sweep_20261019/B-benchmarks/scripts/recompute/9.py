#!/usr/bin/env python3
# Row 9 recompute (20261017-rootfinding-native-cycles). Reads committed files only.
import json, subprocess

R = '/home/user/crypto'
SCRIPT = 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/9.py'
README = 'research/rootfinding_publication_20261017_v1/README.md'
SCI = 'lab/ledger/attempts/native-full-norm-CDCL-search-n83-20261017-v1/0001/scientific_result.json'
PROC = 'lab/ledger/attempts/native-full-norm-CDCL-search-n83-20261017-v1/0001/process.json'
PREREG = 'lab/ledger/attempts/native-full-norm-CDCL-search-n83-20261017-v1/0001/preregistration.json'
V5 = 'research/autolab_cycle_20261017_v5/REPORT.json'
V7 = 'research/autolab_cycle_20261017_v7/REPORT.json'
RECEIPT = 'research/rootfinding_publication_20261017_v1/PUBLICATION_RECEIPT.json'


def load(p):
    return json.load(open(R + '/' + p))


def paths_with_value(o, val, pre=''):
    if isinstance(o, dict):
        for k, v in o.items():
            yield from paths_with_value(v, val, pre + '/' + k)
    elif isinstance(o, list):
        for i, v in enumerate(o):
            yield from paths_with_value(v, val, pre + '/' + str(i))
    elif isinstance(o, (int, float)) and not isinstance(o, bool) and o == val:
        yield pre


def line(path, n):
    with open(R + '/' + path, encoding='utf-8') as f:
        for i, t in enumerate(f, 1):
            if i == n:
                return f'{path}:{n}: {t.rstrip()}'
    return f'{path}:{n}: <missing>'


def gd(path):
    out = subprocess.run(['git', '-C', R, 'log', '--diff-filter=A', '--format=%H %cI', '--', path],
                         capture_output=True, text=True).stdout.strip()
    return out.replace('\n', ';')


sci = load(SCI)
proc = load(PROC)
prereg = load(PREREG)
v5 = load(V5)
v7 = load(V7)

wall = proc['wall_seconds']
timeout = prereg['spec']['timeout_seconds']
assert prereg['identity']['timeout_seconds'] == timeout
claims = []
claims.append({
    'claimed': '60 s hard search',
    'computed': f'process.json wall_seconds={wall:.2f}; preregistration.json spec.timeout_seconds={timeout}',
    'definition': 'wall_seconds (process.json) and spec.timeout_seconds (preregistration.json; identity.timeout_seconds equal) of the norm-primary CDCL attempt 0001',
    'files': [PROC, PREREG],
    'status': 'mismatch' if abs(wall - 60) / 60 > 0.01 else 'match',
})
vlb = sci['evidence']['verified_norm_primary_lower_bound']
claims.append({
    'claimed': '0 verified models',
    'computed': f'verified_norm_primary_lower_bound={vlb}',
    'definition': 'evidence.verified_norm_primary_lower_bound in scientific_result.json',
    'files': [SCI],
    'status': 'match' if vlb == 0 else 'mismatch',
})
nc = sci['evidence']['norm_primary_count_exact']
claims.append({
    'claimed': 'no exact count',
    'computed': f'norm_primary_count_exact={nc}',
    'definition': 'evidence.norm_primary_count_exact in scientific_result.json (None = no exact count)',
    'files': [SCI],
    'status': 'match' if nc is None else 'mismatch',
})
rec = [r for r in v5['records'] if r['id'] == 'native-odd-interval-roots-NIST-panel-20261017-v2'][0]
nrows = len(rec['evidence']['rows'])
claims.append({
    'claimed': '66 odd-field extraction controls',
    'computed': f'len(records[id=native-odd-interval-roots-NIST-panel-20261017-v2].evidence.rows)={nrows}',
    'definition': 'row count of the odd-interval-roots record in autolab_cycle_20261017_v5 REPORT.json',
    'files': [V5],
    'status': 'match' if nrows == 66 else 'mismatch',
})
nf = rec['evidence']['field_count']
claims.append({
    'claimed': '11 fields',
    'computed': f'field_count={nf}',
    'definition': 'evidence.field_count of the same record',
    'files': [V5],
    'status': 'match' if nf == 11 else 'mismatch',
})
cv = [p for p in paths_with_value(v7, 16133)]
claims.append({
    'claimed': '16,133 formal cone variables',
    'computed': f'value 16133 found at {cv}',
    'definition': 'a value equal to 16133 anywhere in autolab_cycle_20261017_v7 REPORT.json (path listed)',
    'files': [V7],
    'status': 'match' if cv else 'mismatch',
})
pr = [p for p in paths_with_value(v7, 2879)]
claims.append({
    'claimed': '2,879 parity rows',
    'computed': f'value 2879 found at {pr}',
    'definition': 'a value equal to 2879 anywhere in autolab_cycle_20261017_v7 REPORT.json (path listed)',
    'files': [V7],
    'status': 'match' if pr else 'mismatch',
})

out = {
    'row_no': 9,
    'slug': '20261017-rootfinding-native-cycles',
    'claims': claims,
    'identical': {'claimed': False, 'n_compared': None, 'n_equal': None, 'n_differ': None,
                  'files': [], 'status': 'not_claimed'},
    'offrepo_refs': [],
    'aa_lines': [line(README, 9), line(README, 15)],
    'isolation_lines': [],
    'protocol_git_date': gd(PREREG),
    'first_raw_git_date': gd(RECEIPT),
    'script': SCRIPT,
    'notes': (f'Study README line 3 says the search ran "after60seconds"; process.json wall_seconds={wall}; '
              f'preregistration spec.timeout_seconds={timeout}. The README and HEADLINE name no protocol path; '
              f'PREREG used as protocol path, RECEIPT (first listed candidate) as first raw file. '
              f'The aa_lines are the control statements; the study README has no A/A or null arm. '
              f'The README has no isolation statement.'),
}
print(json.dumps(out, ensure_ascii=False))
