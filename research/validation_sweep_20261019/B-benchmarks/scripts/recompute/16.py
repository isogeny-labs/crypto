#!/usr/bin/env python3
# Row 16 recompute (20261017-isogeny-volcano-cost). Reads committed files only.
import gzip, json, glob, subprocess

R = '/home/user/crypto'
SCRIPT = 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/16.py'
D = 'research/isogeny-volcano-cost_20261017/'
PROTO = D + 'PROTOCOL.md'
README = D + 'README.md'
FIRST_RAW = D + 'sessions/l2-ramified/spec/records.jsonl.gz'


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


files = sorted(glob.glob(R + '/' + D + 'sessions/*/spec*/records.jsonl.gz'))
nonwarm_class = nonwarm_ctrl = total = 0
slugs_class = set()
for f in files:
    rel = f.split('/sessions/')[1]
    is_ctrl = rel.startswith('j0/') or rel.startswith('j1728/')
    for l in gzip.open(f, 'rt'):
        r = json.loads(l)
        total += 1
        if not is_ctrl:
            slugs_class.add(r.get('workload', {}).get('curve', {}).get('slug'))
        if not r.get('warmup'):
            if is_ctrl:
                nonwarm_ctrl += 1
            else:
                nonwarm_class += 1

audits = sorted(glob.glob(R + '/' + D + 'audits/*.audit.json'))
a_rec = a_ver = a_rep = a_rep_ok = 0
for a in audits:
    d = json.load(open(a))
    a_rec += d.get('records', 0)
    a_ver += d.get('verified_records', 0)
    reps = d.get('replays') or []
    a_rep += len(reps)
    a_rep_ok += sum(1 for x in reps if x.get('reproduced') is True)

claims = [
    {'claimed': '46,912 measured runs',
     'computed': f'non-warmup records: class sessions={nonwarm_class} + control sessions j0/j1728={nonwarm_ctrl} = {nonwarm_class + nonwarm_ctrl}; README line 19 states 43,200 class runs',
     'definition': 'records with warmup==false across all sessions/*/spec*/records.jsonl.gz files (warmup flag in ecbench.record/v1)',
     'files': [D + 'sessions/*/spec*/records.jsonl.gz', README],
     'status': 'match' if nonwarm_class + nonwarm_ctrl == 46912 else 'mismatch'},
    {'claimed': 'all verified',
     'computed': f'audit verified_records={a_ver} of records={a_rec} across {len(audits)} audit files',
     'definition': 'sum of verified_records over sum of records in audits/*.audit.json',
     'files': [D + 'audits/*.audit.json'],
     'status': 'match' if a_ver == a_rec else 'mismatch'},
    {'claimed': '408/408 replays exact',
     'computed': f'replays reproduced={a_rep_ok} of replays={a_rep}',
     'definition': 'count of replays[].reproduced==true over replays[] in audits/*.audit.json',
     'files': [D + 'audits/*.audit.json'],
     'status': 'match' if (a_rep_ok, a_rep) == (408, 408) else 'mismatch'},
    {'claimed': '800 workload curves',
     'computed': f'distinct workload.curve.slug in class sessions (non-j0/j1728)={len(slugs_class)}',
     'definition': 'count of distinct workload.curve.slug in sessions/*/spec*/records.jsonl.gz excluding j0 and j1728',
     'files': [D + 'sessions/*/spec*/records.jsonl.gz'],
     'status': 'match' if len(slugs_class) == 800 else 'mismatch'},
    {'claimed': 'BSGS identical on every curve and model for every paired target (12/12 targets)',
     'computed': 'not computed: the committed records have no field that defines the isomorphic-model group or the paired target key the study uses',
     'definition': 'identity of cost and counters across models needs a grouping key that the study does not state in the committed files',
     'files': [D + 'sessions/*/spec-bsgs/records.jsonl.gz'], 'status': 'cannot_define'},
    {'claimed': 'rho floor/crater 1.022 [0.983, 1.062] (90%)',
     'computed': 'not computed: estimator is v1.1 class-stratified mean of logs with bootstrap 2,000 and permutations 10,000 from volcano-stats; seed and resampling order are not in the committed records and the tool is not run here',
     'definition': 'point estimate and 90% bootstrap interval; requires running the native volcano-stats tool',
     'files': [D + 'README.md', 'src/bin/volcano_stats.rs'], 'status': 'cannot_define'},
    {'claimed': 'IC yield floor/crater 0.980 [0.957, 1.004]',
     'computed': 'not computed: same estimator and tool limits as the rho ratio',
     'definition': 'v1.1 class-stratified IC relation yield ratio with bootstrap interval',
     'files': [D + 'README.md', 'src/bin/volcano_stats.rs'], 'status': 'cannot_define'},
    {'claimed': 'between isomorphic models τ = 4.7% (p = 0.0005) > between same-level curves τ = 2.2%',
     'computed': 'not computed: τ statistic and its permutation null (10,000 permutations, seed not committed) are produced by volcano-stats',
     'definition': 'tau between isomorphic models vs between same-level curves with permutation p; requires running the native tool',
     'files': [D + 'README.md', 'src/bin/volcano_stats.rs'], 'status': 'cannot_define'},
    {'claimed': 'IC base+1 1.061 (pred. 1.063)',
     'computed': 'not computed: control ratio uses matched 3,200-workload bootstrap in volcano-stats',
     'definition': 'planted IC control ratio with bootstrap interval; requires running the native tool',
     'files': [D + 'README.md'], 'status': 'cannot_define'},
    {'claimed': 'Rho below IC on all 800 curves at every §5a weight (IC ≈ 30–60× rho)',
     'computed': 'not computed in this run: needs the §5a pricing band weights (band/ directory) applied to per-curve costs',
     'definition': 'per-curve ratio of IC cost to rho cost at each §5a weight in {0, 0.1, 1} GAE per unpriced operation',
     'files': [D + 'band/'], 'status': 'cannot_define'},
]

out = {
    'row_no': 16,
    'slug': '20261017-isogeny-volcano-cost',
    'claims': claims,
    'identical': {'claimed': True, 'n_compared': None, 'n_equal': None, 'n_differ': None,
                  'files': [D + 'sessions/*/spec-bsgs/records.jsonl.gz'], 'status': 'cannot_define'},
    'offrepo_refs': [],
    'aa_lines': [line(README, 27), line(README, 75), line(README, 104), line(README, 109)],
    'isolation_lines': [line(README, 156), line(README, 188)],
    'protocol_git_date': gd(PROTO),
    'first_raw_git_date': gd(FIRST_RAW),
    'script': SCRIPT,
    'notes': (f'Records files: {len(files)} sessions/*/spec*/records.jsonl.gz (total records {total}); audits: {len(audits)} files. '
              f'README line 19 says 43,200 class runs (46,912 with controls); committed non-warmup class records = {nonwarm_class}. '
              f'Controls non-warmup = {nonwarm_ctrl}. The first raw file used is sessions/l2-ramified/spec/records.jsonl.gz (first listed candidate). '
              'The study has no PDF or grep-able off-repo reference. The claim count uses all spec-family record files, not only the six spec/ files. '
              'Protocol path used: ' + PROTO + '.'),
}
print(json.dumps(out, ensure_ascii=False))
