#!/usr/bin/env python3
# Row 15 recompute (20261017-generic-dlp-correctness). Reads committed files only.
import json, subprocess, statistics

R = '/home/user/crypto'
SCRIPT = 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/15.py'
D = 'research/generic_dlp_correctness_20261017/'
RAW = D + 'raw/'
PROTO = D + 'PROTOCOL.md'
README = D + 'README.md'
BEFORE = RAW + 'counts_before.jsonl'
AFTER = RAW + 'counts_after.jsonl'


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


def jl(p):
    return [json.loads(l) for l in open(R + '/' + p, encoding='utf-8')]


b = jl(BEFORE)
a = jl(AFTER)
bc = [r for r in b if r['kind'] == 'collab']
ac = [r for r in a if r['kind'] == 'collab']
bp = {(r['curve'], r['target'], r['bound']): r for r in b if r['kind'] == 'ph'}
ap = {(r['curve'], r['target'], r['bound']): r for r in a if r['kind'] == 'ph'}

def cpair(curve, neg):
    x = [r for r in bc if r['curve'] == curve and r['negation_map'] == neg]
    y = [r for r in ac if r['curve'] == curve and r['negation_map'] == neg]
    assert len(x) == 1 and len(y) == 1
    return x[0], y[0]

claims = []
def add(claimed, computed, definition, files, ok):
    claims.append({'claimed': claimed, 'computed': computed, 'definition': definition,
                   'files': files, 'status': 'match' if ok else 'mismatch'})

for curve, claimed_before, claimed_after in (('demo-small', 50, 1), ('demo-mid', 106, 0), ('demo-32', 197, 15)):
    x, y = cpair(curve, True)
    add(f'collab dead trails with the negation map {claimed_before} → {claimed_after} ({curve})',
        f'dead_trails before={x["dead_trails"]} after={y["dead_trails"]}',
        'dead_trails field of raw/counts_before.jsonl and raw/counts_after.jsonl, kind=collab, negation_map=true',
        [BEFORE, AFTER], x['dead_trails'] == claimed_before and y['dead_trails'] == claimed_after)

dps_b = sum(r['dps_verified'] for r in bc); dps_bt = sum(r['dps'] for r in bc)
dps_a = sum(r['dps_verified'] for r in ac); dps_at = sum(r['dps'] for r in ac)
add('every DP record verifies',
    f'dps_verified/dps before={dps_b}/{dps_bt} after={dps_a}/{dps_at} (all rows)',
    'sum of dps_verified over sum of dps, collab rows, both labels',
    [BEFORE, AFTER], dps_b == dps_bt and dps_a == dps_at)

ph_b = sum(r['total_steps'] for r in bp.values())
ph_a = sum(r['total_steps'] for r in ap.values())
add('Pohlig–Hellman total_steps 284,364 → 10,682',
    f'sum total_steps before={ph_b} after={ph_a}',
    'sum of total_steps over the 48 kind=ph rows in each file',
    [BEFORE, AFTER], ph_b == 284364 and ph_a == 10682)

add('26.62× total_steps ratio', f'{ph_b / ph_a:.4f}', 'sum before / sum after', [BEFORE, AFTER],
    abs(ph_b / ph_a - 26.62) / 26.62 <= 0.01)

per = []
for k in bp:
    per.append(bp[k]['total_steps'] / ap[k]['total_steps'])
add('1.85–46.53× per solve',
    f'min={min(per):.4f} max={max(per):.4f} of per-(curve,target,bound) total_steps before/after (n={len(per)})',
    'ratio per kind=ph row pair keyed by (curve,target,bound), min and max',
    [BEFORE, AFTER], abs(min(per) - 1.85) / 1.85 <= 0.01 and abs(max(per) - 46.53) / 46.53 <= 0.01)

correct_b = sum(1 for r in bp.values() if r['correct'])
correct_a = sum(1 for r in ap.values() if r['correct'])
rec_eq = sum(1 for k in bp if bp[k]['recovered'] == ap[k]['recovered'])
add('all 48 answers correct and identical',
    f'correct before={correct_b}/{len(bp)} after={correct_a}/{len(ap)}; recovered identical={rec_eq}/{len(bp)}',
    'correct flag and recovered field of kind=ph rows, pairs keyed by (curve,target,bound)',
    [BEFORE, AFTER], correct_b == 48 and correct_a == 48 and rec_eq == 48)

# identical: collab no-negation arm pairs (5 counter fields) and the 48 PH answer pairs
n_cmp = n_eq = n_diff = 0
for curve in ('demo-small', 'demo-mid', 'demo-32'):
    x, y = cpair(curve, False)
    n_cmp += 1
    if all(x[f] == y[f] for f in ('dead_trails', 'dead_steps', 'dp_steps', 'dps', 'dps_verified')):
        n_eq += 1
    else:
        n_diff += 1
for k in bp:
    n_cmp += 1
    if bp[k]['recovered'] == ap[k]['recovered']:
        n_eq += 1
    else:
        n_diff += 1

identical = {
    'claimed': True, 'n_compared': n_cmp, 'n_equal': n_eq, 'n_differ': n_diff,
    'files': [BEFORE, AFTER],
    'status': 'match' if n_diff == 0 else 'mismatch',
}

out = {
    'row_no': 15,
    'slug': '20261017-generic-dlp-correctness',
    'claims': claims,
    'identical': identical,
    'offrepo_refs': [],
    'aa_lines': [],
    'isolation_lines': [],
    'protocol_git_date': gd(PROTO),
    'first_raw_git_date': gd(BEFORE),
    'script': SCRIPT,
    'notes': ('identical units: 3 no-negation collab pairs (fields dead_trails, dead_steps, dp_steps, dps, dps_verified) '
              f'and {len(bp)} PH recovered-answer pairs; n_compared={n_cmp}. '
              'The study states no timing or isolation line; isolation_lines is empty. '
              'Protocol path used: ' + PROTO + '. First raw file used: ' + BEFORE + ' (first listed candidate).'),
}
print(json.dumps(out, ensure_ascii=False))
