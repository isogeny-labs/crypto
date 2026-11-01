import json, os, re, statistics, subprocess
REPO = '/home/user/crypto'
ROW = 79
STUDY = 'research/ecdlp_speedups_20261015'
D = os.path.join(REPO, STUDY)
SELF = 'scripts/recompute/79.py'
def s2(arm, i):
    return json.loads(open(os.path.join(D, f'raw/s2-{arm}-{i}.txt')).read())['timing_ms']
pb = [s2('base', i)['process_total'] for i in (1, 2, 3)]
pc = [s2('cand', i)['process_total'] for i in (1, 2, 3)]
tb = [s2('base', i)['targets_total'] for i in (1, 2, 3)]
tc = [s2('cand', i)['targets_total'] for i in (1, 2, 3)]
mpb, mpc = statistics.median(pb) / 1000, statistics.median(pc) / 1000
mtb, mtc = statistics.median(tb), statistics.median(tc)
fl_s2 = [os.path.join(STUDY, f'raw/s2-{a}-{i}.txt') for a in ('base', 'cand') for i in (1, 2, 3)]
claims = [
    {'claimed': 'orbit fast path whole-process median 60.9 s -> 37.5 s (1.63x)',
     'computed': f'{mpb:.2f} s -> {mpc:.2f} s ({mpb / mpc:.3f}x)',
     'definition': 'median over 3 reps of timing_ms.process_total (s2-*.txt) per arm; ratio of medians',
     'files': fl_s2,
     'status': 'match' if abs(mpb / mpc - 1.63) / 1.63 <= 0.01 and abs(mpc - 37.5) / 37.5 <= 0.01 and abs(mpb - 60.9) / 60.9 <= 0.01 else 'mismatch'},
    {'claimed': 'targets stage 1.95x', 'computed': f'{mtb / mtc:.3f}x ({mtb:.1f} / {mtc:.1f} ms)',
     'definition': 'median over 3 reps of timing_ms.targets_total per arm; ratio of medians',
     'files': fl_s2,
     'status': 'match' if abs(mtb / mtc - 1.95) / 1.95 <= 0.01 else 'mismatch'},
    {'claimed': 'j=0 relation stage 13.99 s -> 11.90 s (1.17x median, ~8% per trial)', 'computed': 'no_raw',
     'definition': 'j=0 relations_ms and trials per rep are not in raw/ (only o2-*.jsonl and s2-*.txt are committed); the README table is the only source', 'files': [],
     'status': 'no_raw_in_repo'},
    {'claimed': 'all runs recovered the same planted scalar and agreed with the rho control (j=0 stage)', 'computed': 'no_raw',
     'definition': 'no j=0 stage record in raw/', 'files': [], 'status': 'no_raw_in_repo'},
]
# identical: orbit fast path base vs cand, per rep and fixture
n_cmp = n_eq = 0
for i in (1, 2, 3):
    B = [json.loads(l) for l in open(os.path.join(D, f'raw/o2-base-{i}.jsonl'))]
    C = [json.loads(l) for l in open(os.path.join(D, f'raw/o2-cand-{i}.jsonl'))]
    for b, c in zip(B, C):
        n_cmp += 1
        if b['fixture_index'] == c['fixture_index'] and all(b[k] == c[k] for k in ('recovered_scalar', 'point_indices', 'x_codes')):
            n_eq += 1
ident = {'claimed': True, 'n_compared': n_cmp, 'n_equal': n_eq, 'n_differ': n_cmp - n_eq,
         'files': [os.path.join(STUDY, f'raw/o2-{a}-{i}.jsonl') for a in ('base', 'cand') for i in (1, 2, 3)],
         'status': 'match' if n_eq == n_cmp else 'mismatch'}
def grep_lines(rel, pat):
    out = []
    with open(os.path.join(D, rel)) as fh:
        for i, line in enumerate(fh, 1):
            if re.search(pat, line):
                out.append(f'{STUDY}/{rel}:{i}: {line.rstrip()}')
    return out
aa = grep_lines('README.md', r'control|null|A/A')
iso = grep_lines('README.md', r'isolat|shared|contended|other agents|host')
def git_date(path):
    out = subprocess.run(['git', '-C', REPO, 'log', '--diff-filter=A', '--format=%H %cI', '--', path], capture_output=True, text=True).stdout.strip()
    return out if out else 'EMPTY'
out = {'row_no': ROW, 'slug': '20261015-ecdlp-speedups', 'claims': claims, 'identical': ident,
       'offrepo_refs': [], 'aa_lines': aa, 'isolation_lines': iso,
       'protocol_git_date': 'NO_PROTOCOL_FILE (no PROTOCOL in study dir)',
       'first_raw_git_date': git_date(os.path.join(STUDY, 'raw/o2-base-1.jsonl')),
       'script': SELF,
       'notes': ('Process and target medians come from timing_ms in raw/s2-*.txt; the target sums in o2-*.jsonl differ from timing targets_total and were not used. '
                 'Candidate rep 3 is contended (README). Identity compares recovered_scalar, point_indices, x_codes per rep and fixture (16 fixtures x 3 reps). '
                 'Host: no isolation receipt (README). Off-repo grep over study README: no hits. No j=0 stage raw in repo.')}
print(json.dumps(out, ensure_ascii=False))
