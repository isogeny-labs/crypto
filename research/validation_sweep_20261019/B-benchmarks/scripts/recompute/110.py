#!/usr/bin/env python3
# SWB-RECOMPUTE-14 row 110: 20261013-n83-dense-root. Reads committed files only; appends one JSON object.
import json, os, re, statistics, subprocess
from datetime import datetime

ROOT = '/home/user/crypto'
OUT = os.path.join(ROOT, 'research/validation_sweep_20261019/B-benchmarks/recompute/SWB-RECOMPUTE-14.jsonl')
N = 'research/n83_dense_root_20261013'

def jl(path):
    with open(os.path.join(ROOT, path)) as f:
        return [json.loads(l) for l in f if l.strip()]

def one(path):
    rs = jl(path)
    assert len(rs) == 1, (path, len(rs))
    return rs[0]

k1798 = jl(N + '/runs/panel/k1798_dense/targets_result.jsonl')
k1500 = jl(N + '/runs/panel/k1500_dense/targets_result.jsonl')
k1271d = jl(N + '/runs/panel/k1271_dense/targets_result.jsonl')
k1271r = jl(N + '/runs/panel/k1271_regular/targets_result.jsonl')
primary_k1798 = one(N + '/runs/primary/k1798_dense/targets_result.jsonl')
primary_k1798_stdout = one(N + '/runs/primary/k1798_dense/stdout.jsonl')
panel_k1798_stdout = one(N + '/runs/panel/k1798_dense/stdout.jsonl')
panel_k1500_stdout = one(N + '/runs/panel/k1500_dense/stdout.jsonl')
rho = json.loads([l for l in open(os.path.join(ROOT, N + '/runs/rho_primary/stdout.log')) if l.startswith('{')][0])
analysis = json.load(open(os.path.join(ROOT, N + '/analysis_panel.json')))

def key(r):
    return r['fixture_index']

by1798 = {key(r): r for r in k1798}
by1500 = {key(r): r for r in k1500}
common = sorted(set(by1798) & set(by1500))
mean1798 = statistics.mean(r['probes'] for r in k1798)
mean1500 = statistics.mean(r['probes'] for r in k1500)
paired = [by1500[i]['probes'] - by1798[i]['probes'] for i in common]
paired_mean = statistics.mean(paired)
same_target = sum(1 for i in common if by1500[i]['target'] == by1798[i]['target'])

ratio = rho['online_ms'] / primary_k1798['target_ms']

# timeouts: duration of the two K1500 regular runs (panel and primary)
def dur(path):
    s = open(os.path.join(ROOT, path)).read().strip()
    return s
def secs(d):
    a = open(os.path.join(ROOT, N + '/runs/' + d + '/started_utc.txt')).read().strip()
    b = open(os.path.join(ROOT, N + '/runs/' + d + '/finished_utc.txt')).read().strip()
    fa = datetime.strptime(a, '%Y-%m-%dT%H:%M:%SZ')
    fb = datetime.strptime(b, '%Y-%m-%dT%H:%M:%SZ')
    return (fb - fa).total_seconds()
exit_codes = {}
for d in ('panel/k1500_regular', 'primary/k1500_regular'):
    exit_codes[d] = open(os.path.join(ROOT, N + '/runs/' + d + '/exit_status.txt')).read().strip()
timeout_secs = {d: secs(d) for d in ('panel/k1500_regular', 'primary/k1500_regular')}

# completed panel rows replayed
arms = analysis['arms']
panel_pass_rows = sum(a['targets'] for a in arms if a.get('status') == 'PASS_INDEPENDENT_REPLAY')
panel_arm_status = {a['arm']: a.get('status') for a in arms}
panel_rows_raw = sum(len(x) for x in (k1798, k1500, k1271d, k1271r))

# bootstrap statement lines in study text
def grep_lines(path, pattern):
    out = []
    p = os.path.join(ROOT, path)
    if not os.path.exists(p):
        return out
    with open(p) as f:
        for i, line in enumerate(f, 1):
            if re.search(pattern, line):
                out.append('%s:%d: %s' % (path, i, line.rstrip('\n')))
    return out

boot_lines = grep_lines(N + '/REPORT.md', r'bootstrap|resample|seed')
pc = analysis.get('probe_comparisons')

docs = [N + '/REPORT.md', N + '/PROTOCOL.md', N + '/TESTS.md', N + '/CAPACITY_CEILING.md', N + '/REPORT.typ']
offrepo_refs, aa_lines, isolation_lines = [], [], []
for p in docs:
    offrepo_refs += grep_lines(p, r'drive\.google|/Volumes|SSD|/Users|s3://|gs://|store:|Modal volume|not committed|scratch')
    aa_lines += grep_lines(p, r'A/A|null-control|null control|negative control|positive control')
    isolation_lines += grep_lines(p, r'isolat|receipt|shared|M4|cores')

def git_add(path):
    r = subprocess.run(['git', '-C', ROOT, 'log', '--diff-filter=A', '--format=%H %cI', '--', path], capture_output=True, text=True)
    return r.stdout.strip() or 'empty'

def r1(x):
    return '%.1f' % x

claims = [
    {'claimed': 'K1798 dense: 298,468 actual base points',
     'computed': 'factor_base_points in runs/panel/k1798_dense/stdout.jsonl = %d ; primary k1798 = %d' % (panel_k1798_stdout['factor_base_points'], primary_k1798_stdout['factor_base_points']),
     'definition': 'factor_base_points field of the run summaries',
     'files': [N + '/runs/panel/k1798_dense/stdout.jsonl', N + '/runs/primary/k1798_dense/stdout.jsonl'],
     'status': 'match' if (panel_k1798_stdout['factor_base_points'] == 298468 and primary_k1798_stdout['factor_base_points'] == 298468) else 'mismatch'},
    {'claimed': '8 GiB allocated slots',
     'computed': 'root_table_allocated_bytes (panel k1798 summary) = %d = %.4f GiB' % (panel_k1798_stdout['root_table_allocated_bytes'], panel_k1798_stdout['root_table_allocated_bytes'] / 2 ** 30),
     'definition': 'root_table_allocated_bytes divided by 2^30',
     'files': [N + '/runs/panel/k1798_dense/stdout.jsonl'],
     'status': 'match' if panel_k1798_stdout['root_table_allocated_bytes'] == 8 * 2 ** 30 else 'mismatch'},
    {'claimed': '169,069.8 mean probes (K1798 dense)',
     'computed': 'mean probes over %d panel targets = %.1f' % (len(k1798), mean1798),
     'definition': 'arithmetic mean of the probes field over targets_result.jsonl rows of runs/panel/k1798_dense',
     'files': [N + '/runs/panel/k1798_dense/targets_result.jsonl'],
     'status': 'match' if r1(mean1798) == '169069.8' else 'mismatch'},
    {'claimed': 'versus K1500 dense 300,021.9 mean probes',
     'computed': 'mean probes over %d panel targets = %.1f' % (len(k1500), mean1500),
     'definition': 'as above, runs/panel/k1500_dense',
     'files': [N + '/runs/panel/k1500_dense/targets_result.jsonl'],
     'status': 'match' if r1(mean1500) == '300021.9' else 'mismatch'},
    {'claimed': 'paired difference 130,952.1 probes',
     'computed': 'mean of (K1500 dense - K1798 dense) probes over %d targets paired by fixture_index (same target in %d) = %.1f' % (len(common), same_target, paired_mean),
     'definition': 'paired mean difference, pairing by fixture_index',
     'files': [N + '/runs/panel/k1500_dense/targets_result.jsonl', N + '/runs/panel/k1798_dense/targets_result.jsonl'],
     'status': 'match' if r1(paired_mean) == '130952.1' else 'mismatch'},
    {'claimed': '95% bootstrap [52,057.7, 209,990.8] on the paired difference',
     'computed': 'bootstrap resampling rule (resamples, seed) not stated in the committed study files; REPORT.md bootstrap/seed lines: %s ; analysis_panel.json probe_comparisons present: %s' % (boot_lines or 'none', pc is not None),
     'definition': 'interval depends on a resampling rule and seed that the study does not state in the files, so it cannot be redefined from raw',
     'files': [N + '/analysis_panel.json', N + '/REPORT.md'],
     'status': 'cannot_define'},
    {'claimed': 'one-target online 63.324 ms',
     'computed': 'primary k1798 targets_result target_ms = %.6f ms (stdout timing target_median = %.6f ms)' % (primary_k1798['target_ms'], primary_k1798_stdout['timing_ms']['target_median']),
     'definition': 'target_ms of the single primary target row',
     'files': [N + '/runs/primary/k1798_dense/targets_result.jsonl'],
     'status': 'match' if r1(primary_k1798['target_ms']) == '63.3' and ('%.3f' % primary_k1798['target_ms']) == '63.324' else 'mismatch'},
    {'claimed': 'same-point rho 1,173.293 ms',
     'computed': 'rho_primary stdout online_ms = %.5f ms ; online_interval_ns = %d' % (rho['online_ms'], rho['online_interval_ns']),
     'definition': 'online_ms of the rho run stdout',
     'files': [N + '/runs/rho_primary/stdout.log'],
     'status': 'match' if ('%.3f' % rho['online_ms']) == '1173.293' else 'mismatch'},
    {'claimed': 'exploratory ratio 18.528',
     'computed': 'rho online_ms / K1798 primary target_ms = %.6f / %.6f = %.6f' % (rho['online_ms'], primary_k1798['target_ms'], ratio),
     'definition': 'ratio of same-point rho online time over the one-target K1798 online time',
     'files': [N + '/runs/rho_primary/stdout.log', N + '/runs/primary/k1798_dense/targets_result.jsonl'],
     'status': 'match' if ('%.3f' % ratio) == '18.528' else 'mismatch'},
    {'claimed': 'two K1500 regular 900 s timeouts retained',
     'computed': 'exit_status: %s ; duration panel %.0f s, primary %.0f s (started/finished UTC files)' % (exit_codes, timeout_secs['panel/k1500_regular'], timeout_secs['primary/k1500_regular']),
     'definition': 'count of K1500 regular runs with exit status 124, and their wall duration from the started and finished UTC files',
     'files': [N + '/runs/panel/k1500_regular/exit_status.txt', N + '/runs/primary/k1500_regular/exit_status.txt'],
     'status': 'match' if (all(v == '124' for v in exit_codes.values()) and all(abs(t - 900) <= 5 for t in timeout_secs.values())) else 'mismatch'},
    {'claimed': '256 completed panel rows independently replayed',
     'computed': 'panel arm status %s ; completed rows with PASS_INDEPENDENT_REPLAY = %d ; targets_result rows in the four completed arms = %d' % (panel_arm_status, panel_pass_rows, panel_rows_raw),
     'definition': 'sum of targets over panel arms whose analysis status is PASS_INDEPENDENT_REPLAY, cross-checked against targets_result.jsonl row counts of the same arms',
     'files': [N + '/analysis_panel.json'] + [N + '/runs/panel/%s/targets_result.jsonl' % a for a in ('k1271_regular', 'k1271_dense', 'k1500_dense', 'k1798_dense')],
     'status': 'match' if panel_pass_rows == 256 else 'mismatch'},
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
    'row_no': 110,
    'slug': '20261013-n83-dense-root',
    'claims': claims,
    'identical': identical,
    'offrepo_refs': offrepo_refs,
    'aa_lines': aa_lines,
    'isolation_lines': isolation_lines,
    'protocol_git_date': 'PROTOCOL.md: %s' % git_add(N + '/PROTOCOL.md'),
    'first_raw_git_date': 'final_fmt.log: %s' % git_add(N + '/final_fmt.log'),
    'script': 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/110.py',
    'notes': 'Panel arms k1271_regular, k1271_dense, k1500_dense, k1798_dense each 64 target rows; k1500_regular exit 124 with 0 rows. Primary run target is one public target. rho stdout reports online_ms and is the same-point rho. Bootstrap rule not stated in study files (see REPORT.md lines matched above). Study has no A/A line; isolation text is in the study REPORT.md and is recorded in isolation_lines.',
}

os.makedirs(os.path.dirname(OUT), exist_ok=True)
with open(OUT, 'a') as f:
    f.write(json.dumps(row, ensure_ascii=False) + '\n')
print('row 110 written; claims:', len(claims))
for c in claims:
    print(c['status'], '|', c['claimed'], '|', c['computed'])
print('offrepo_refs:', len(offrepo_refs), 'aa_lines:', len(aa_lines), 'isolation_lines:', len(isolation_lines))
