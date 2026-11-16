#!/usr/bin/env python3
# SWB-RECOMPUTE-14 row 107: 20261013-ic-rootfind-ecbench. Reads committed files only; appends one JSON object.
import json, os, re, statistics, collections

ROOT = '/home/user/crypto'
OUT = os.path.join(ROOT, 'research/validation_sweep_20261019/B-benchmarks/recompute/SWB-RECOMPUTE-14.jsonl')
P = 'research/pdp_root_finder_ecbench_20261013'
K = 'research/pkm_tower_ecbench_20261013'
BIN_SIZES = [13, 19, 23, 31, 37]
PKM_SIZES = [20, 22, 24, 26, 28]

def recs(path):
    with open(os.path.join(ROOT, path)) as f:
        return [json.loads(l) for l in f if l.strip()]

def sess(study, n, prefix):
    return recs('%s/sessions/%s%d/records.jsonl' % (study, prefix, n))

def paired_ratio(rs, arm_b, arm_a):
    """sum S_B / sum S_A over (workload_id, round) pairs of measured (non-warmup) records."""
    a = {}
    b = {}
    for r in rs:
        if r['warmup']:
            continue
        key = (r['workload']['workload_id'], r['round'])
        if r['arm'] == arm_a:
            a[key] = r['cost']['s']
        if r['arm'] == arm_b:
            b[key] = r['cost']['s']
    keys = sorted(set(a) & set(b))
    sb = sum(b[k] for k in keys)
    sa = sum(a[k] for k in keys)
    return sb / sa, len(keys)

def median_wall(rs, arm):
    v = [r['online']['wall_ns'] for r in rs if (not r['warmup']) and r['arm'] == arm and r.get('online') and r['online'].get('wall_ns') is not None]
    return (statistics.median(v) if v else None), len(v)

def levels(rs, arm):
    c = collections.Counter(r['isolation']['level'] for r in rs if (not r['warmup']) and r['arm'] == arm)
    return dict(c)

bin_rs = {n: sess(P, n, 'n') for n in BIN_SIZES}
pkm_rs = {n: sess(K, n, 'p') for n in PKM_SIZES}

# S ratios, binary
ratio_rootfind_rho = {}
for n in BIN_SIZES:
    ratio_rootfind_rho[n] = paired_ratio(bin_rs[n], 'ic-rootfind', 'rho-strong')
# wall ratios F4 over rootfind (ratio of medians), n13, n19, n23
wall = {}
for n in (13, 19, 23):
    f4, nf4 = median_wall(bin_rs[n], 'ic-f4')
    rf, nrf = median_wall(bin_rs[n], 'ic-rootfind')
    wall[n] = {'ic-f4_median_wall_ns': f4, 'n_f4': nf4, 'ic-rootfind_median_wall_ns': rf, 'n_rootfind': nrf,
               'ratio_f4_over_rootfind': (f4 / rf) if (f4 and rf) else None,
               'levels_f4': levels(bin_rs[n], 'ic-f4'), 'levels_rootfind': levels(bin_rs[n], 'ic-rootfind')}
# PKM S ratios rootfind / rho-neg
pkm_ratio = {n: paired_ratio(pkm_rs[n], 'ic-rootfind', 'rho-neg') for n in PKM_SIZES}

# verification counts
all_bin = []
for n in BIN_SIZES:
    all_bin += bin_rs[n]
n_total = len(all_bin)
n_warm = sum(1 for r in all_bin if r['warmup'])
n_meas = n_total - n_warm
n_verified = sum(1 for r in all_bin if r['outcome']['status'] == 'verified')
all_pkm = []
for n in PKM_SIZES:
    all_pkm += pkm_rs[n]
n_pkm_total = len(all_pkm)
n_pkm_verified = sum(1 for r in all_pkm if r['outcome']['status'] == 'verified')

# audits
def audit(study, tag, n):
    d = json.load(open(os.path.join(ROOT, '%s/audits/%s%d.audit.json' % (study, tag, n))))
    reps = d.get('replays', [])
    return d['ok'], d['session_status'], len(reps), sum(1 for x in reps if x.get('reproduced')), sum(1 for x in reps if x.get('detail') == 'identical'), d.get('records'), d.get('verified_records')
bin_audit = {n: audit(P, 'n', n) for n in BIN_SIZES}
pkm_audit = {n: audit(K, 'p', n) for n in PKM_SIZES}
bin_all_ok = all(v[0] for v in bin_audit.values())
pkm_all_ok = all(v[0] for v in pkm_audit.values())

# doc lines
def grep_lines(path, pattern):
    out = []
    with open(os.path.join(ROOT, path)) as f:
        for i, line in enumerate(f, 1):
            if re.search(pattern, line):
                out.append('%s:%d: %s' % (path, i, line.rstrip('\n')))
    return out

docs = [P + '/README.md', P + '/REPORT.typ', P + '/PROTOCOL.md', K + '/README.md', K + '/PROTOCOL.md']
offrepo_refs, aa_lines, isolation_lines = [], [], []
for p in docs:
    offrepo_refs += grep_lines(p, r'drive\.google|/Volumes|SSD|/Users|s3://|gs://|store:|Modal|not committed|scratch')
    aa_lines += grep_lines(p, r'A/A|null-control|null control|negative control|positive control')
    isolation_lines += grep_lines(p, r'isolat|shared|idle|reserved|Xeon|vCPU|L2|L1')

summary = {
    'binary_S_ratio_rootfind_over_rho_strong': {str(n): list(ratio_rootfind_rho[n]) for n in BIN_SIZES},
    'pkm_S_ratio_rootfind_over_rho_neg': {str(n): list(pkm_ratio[n]) for n in PKM_SIZES},
    'wall_ratio_f4_over_rootfind': {str(n): wall[n] for n in wall},
    'runs_bin_total_warm_meas': [n_total, n_warm, n_meas],
    'runs_bin_verified': n_verified,
    'runs_pkm_total_verified': [n_pkm_total, n_pkm_verified],
    'bin_audit': {str(n): bin_audit[n] for n in BIN_SIZES},
    'pkm_audit': {str(n): pkm_audit[n] for n in PKM_SIZES},
    'bin_audits_all_ok': bin_all_ok,
    'pkm_audits_all_ok': pkm_all_ok,
}
print(json.dumps(summary, indent=1))

def r1(x):
    return '%.1f' % x

def r0(x):
    return '%d' % round(x)

gap = [ratio_rootfind_rho[n][0] for n in BIN_SIZES]
gap_monotone = all(gap[i] < gap[i + 1] for i in range(len(gap) - 1))

claims = [
    {'claimed': 'whole IC solve with the root-finder 350× faster in wall than with F4 at n = 13 (admitted)',
     'computed': 'median online.wall_ns ic-f4 %s ns (n=%d) / ic-rootfind %s ns (n=%d) = %s ; levels f4 %s rootfind %s' % (
         wall[13]['ic-f4_median_wall_ns'], wall[13]['n_f4'], wall[13]['ic-rootfind_median_wall_ns'], wall[13]['n_rootfind'],
         '%.1f' % wall[13]['ratio_f4_over_rootfind'], wall[13]['levels_f4'], wall[13]['levels_rootfind']),
     'definition': 'ratio of medians of online.wall_ns over measured records, F4 over root-finder; the README admitted-median (L2 only) is the harness value and is not reproduced here',
     'files': [P + '/sessions/n13/records.jsonl'],
     'status': 'match' if r0(wall[13]['ratio_f4_over_rootfind']) == '350' else 'mismatch'},
    {'claimed': 'about 10,000× at n = 23 (descriptive)',
     'computed': 'median online.wall_ns ic-f4 %s (n=%d) / ic-rootfind %s (n=%d) = %.0f ; levels f4 %s' % (
         wall[23]['ic-f4_median_wall_ns'], wall[23]['n_f4'], wall[23]['ic-rootfind_median_wall_ns'], wall[23]['n_rootfind'],
         wall[23]['ratio_f4_over_rootfind'], wall[23]['levels_f4']),
     'definition': 'same as above at n = 23; descriptive (the F4 runs are not held at L2 per the README)',
     'files': [P + '/sessions/n23/records.jsonl'],
     'status': 'match' if (abs(wall[23]['ratio_f4_over_rootfind'] - 10000) / 10000 <= 0.05) else 'mismatch'},
    {'claimed': 'IC 2.9× the strong rho in S (n = 13, binary)',
     'computed': 'paired totals sum S ic-rootfind / sum S rho-strong = %.4f over %d pairs' % (ratio_rootfind_rho[13][0], ratio_rootfind_rho[13][1]),
     'definition': 'ecbench compare rule: sum S_B / sum S_A over (workload_id, round) pairs of measured records',
     'files': [P + '/sessions/n13/records.jsonl'],
     'status': 'match' if r1(ratio_rootfind_rho[13][0]) == '2.9' else 'mismatch'},
    {'claimed': 'IC 25,116× the strong rho in S (n = 37, binary upper end)',
     'computed': 'paired totals sum S ic-rootfind / sum S rho-strong = %.2f over %d pairs' % (ratio_rootfind_rho[37][0], ratio_rootfind_rho[37][1]),
     'definition': 'as above at n = 37',
     'files': [P + '/sessions/n37/records.jsonl'],
     'status': 'match' if r0(ratio_rootfind_rho[37][0]) == '25116' else 'mismatch'},
    {'claimed': 'gap widening (binary S ratio, n = 13, 19, 23, 31, 37)',
     'computed': 'S ratios %s ; strictly increasing: %s' % (['%.4f' % g for g in gap], gap_monotone),
     'definition': 'paired totals ratios from the five binary sessions, in size order',
     'files': [P + '/sessions/n%d/records.jsonl' % n for n in BIN_SIZES],
     'status': 'match' if gap_monotone else 'mismatch'},
    {'claimed': '29× rho (prime, lower end of 29–141×)',
     'computed': 'pkm paired totals sum S ic-rootfind / sum S rho-neg per size: %s' % ({str(n): round(pkm_ratio[n][0], 3) for n in PKM_SIZES}),
     'definition': 'the lower end is the minimum over the five prime sizes of the paired S ratio (rounded to an integer)',
     'files': [K + '/sessions/p%d/records.jsonl' % n for n in PKM_SIZES],
     'status': 'match' if r0(min(pkm_ratio[n][0] for n in PKM_SIZES)) == '29' else 'mismatch'},
    {'claimed': '141× rho (prime, upper end of 29–141×)',
     'computed': 'maximum over the five prime sizes of the paired S ratio = %.3f' % max(pkm_ratio[n][0] for n in PKM_SIZES),
     'definition': 'as above; maximum rounded to an integer',
     'files': [K + '/sessions/p%d/records.jsonl' % n for n in PKM_SIZES],
     'status': 'match' if r0(max(pkm_ratio[n][0] for n in PKM_SIZES)) == '141' else 'mismatch'},
    {'claimed': 'all runs verified',
     'computed': 'binary: %d runs (%d measured, %d warm-ups), verified %d ; prime PKM: %d runs, verified %d' % (n_total, n_meas, n_warm, n_verified, n_pkm_total, n_pkm_verified),
     'definition': 'records with outcome.status == verified over sessions n13, n19, n23, n31, n37 (interrupted n37-interrupted-01 excluded: not a headline session) and p20..p28',
     'files': [P + '/sessions/n%d/records.jsonl' % n for n in BIN_SIZES] + [K + '/sessions/p%d/records.jsonl' % n for n in PKM_SIZES],
     'status': 'match' if (n_verified == n_total and n_pkm_verified == n_pkm_total) else 'mismatch'},
    {'claimed': 'every replay audit is OK',
     'computed': 'binary audits ok=%s for n13..n37 ; prime audits ok=%s for p20..p28 ; binary replays reproduced %s' % (
         bin_all_ok, pkm_all_ok, {str(n): [bin_audit[n][3], bin_audit[n][2]] for n in BIN_SIZES}),
     'definition': 'ok field of audits/*.audit.json for the five binary sessions and five prime sessions',
     'files': [P + '/audits/n%d.audit.json' % n for n in BIN_SIZES] + [K + '/audits/p%d.audit.json' % n for n in PKM_SIZES],
     'status': 'match' if (bin_all_ok and pkm_all_ok) else 'mismatch'},
]

identical = {
    'claimed': False,
    'n_compared': None,
    'n_equal': None,
    'n_differ': None,
    'files': [],
    'status': 'not_claimed',
}

row = {
    'row_no': 107,
    'slug': '20261013-ic-rootfind-ecbench',
    'claims': claims,
    'identical': identical,
    'offrepo_refs': offrepo_refs,
    'aa_lines': aa_lines,
    'isolation_lines': isolation_lines,
    'protocol_git_date': 'ca00c151764f7c3221b22a43a138887f171e051d 2026-10-17T10:19:32-07:00 (git log --diff-filter=A -- %s/PROTOCOL.md; %s/PROTOCOL.md same commit)' % (P, K),
    'first_raw_git_date': 'ca00c151764f7c3221b22a43a138887f171e051d 2026-10-17T10:19:32-07:00 (git log --diff-filter=A -- %s/sessions/n13/records.jsonl)' % P,
    'script': 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/107.py',
    'notes': 'Binary sessions n13 (336 records), n19 (336), n23 (224), n31 (192), n37 (160) total 1248 records; the n37-interrupted-01 session (114 records) and superseded-binary-1 are not used. Prime sessions p20..p28 are in %s. Wall figures use online.wall_ns (ic arms), not the harness admitted median. Audit replay field detail == identical counted separately.' % K,
}

os.makedirs(os.path.dirname(OUT), exist_ok=True)
with open(OUT, 'a') as f:
    f.write(json.dumps(row, ensure_ascii=False) + '\n')
print('row 107 written; claims:', len(claims))
for c in claims:
    print(c['status'], '|', c['claimed'], '|', c['computed'])
print('offrepo_refs:', len(offrepo_refs), 'aa_lines:', len(aa_lines), 'isolation_lines:', len(isolation_lines))
