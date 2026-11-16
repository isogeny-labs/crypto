#!/usr/bin/env python3
# SWB-RECOMPUTE-14 row 106: 20261013-gaudry-quartic-solve-speed. Reads committed files only; appends one JSON object.
import json, os, re, statistics

ROOT = '/home/user/crypto'
OUT = os.path.join(ROOT, 'research/validation_sweep_20261019/B-benchmarks/recompute/SWB-RECOMPUTE-14.jsonl')
S = 'research/gaudry_quartic_speed_20261013'
R = 'research/gaudry_quartic_replay_20261013'

def rd(p):
    with open(os.path.join(ROOT, p)) as f:
        return [json.loads(l) for l in f if l.strip()]

def speed(arm, rnd, d):
    return rd('%s/raw/%s_r%d_d%d.jsonl' % (S, arm, rnd, d))

med_ms = {}
records_per = {}
for arm in ('before', 'after'):
    for d in (4, 5, 6):
        recs = []
        for rnd in (1, 2, 3):
            recs += speed(arm, rnd, d)
        records_per[(arm, d)] = len(recs)
        med_ms[(arm, d)] = statistics.median([r['baseline']['total_ms'] for r in recs])

ratio = {d: med_ms[('before', d)] / med_ms[('after', d)] for d in (4, 5, 6)}
s5 = rd(R + '/raw/s5_p269.jsonl')
s5_ms = [r['baseline']['total_ms'] for r in s5]
s5_muls = [r['baseline']['total_muls'] for r in s5]

# identical check: before vs after, same (round, degree, system), all count fields of baseline (not *_ms)
def counts(rec):
    b = rec['baseline']
    out = {k: v for k, v in b.items() if not k.endswith('_ms') and k not in ('outcome', 'replay')}
    out['baseline_solutions'] = rec.get('baseline_solutions')
    return out

n_pairs = 0
n_equal = 0
diff_detail = []
for rnd in (1, 2, 3):
    for d in (4, 5, 6):
        bef = speed('before', rnd, d)
        aft = speed('after', rnd, d)
        assert len(bef) == len(aft)
        for a, b in zip(bef, aft):
            n_pairs += 1
            ca, cb = counts(a), counts(b)
            if ca == cb:
                n_equal += 1
            else:
                diff_detail.append('r%d d%d sys%s' % (rnd, d, a.get('system')))

solutions_agree_speed = []
for arm in ('before', 'after'):
    for rnd in (1, 2, 3):
        for d in (4, 5, 6):
            solutions_agree_speed += [r['solutions_agree'] for r in speed(arm, rnd, d)]

def f1(x):
    return '%.1f' % x

computed = {
    'median_total_ms_before': {d: med_ms[('before', d)] for d in (4, 5, 6)},
    'median_total_ms_after': {d: med_ms[('after', d)] for d in (4, 5, 6)},
    'ratio_median_before_over_after': ratio,
    'records_per_arm_degree': {'%s_d%d' % k: v for k, v in records_per.items()},
    's5_baseline_total_ms': s5_ms,
    's5_baseline_total_muls': s5_muls,
    'identical_pairs': [n_pairs, n_equal],
    'identical_diff_detail': diff_detail,
    'solutions_agree_speed_records': [len(solutions_agree_speed), sum(1 for x in solutions_agree_speed if x)],
}
for k, v in computed.items():
    print(k, '=', json.dumps(v))

def ok_ratio(c, claimed):
    return 'match' if f1(c) == claimed else 'mismatch'

claims = [
    {'claimed': 'median whole solve 1.9× faster at d = 4',
     'computed': 'before %.2f ms / after %.2f ms = %.4f (n=6 per arm, 3 rounds x 2 systems)' % (med_ms[('before', 4)], med_ms[('after', 4)], ratio[4]),
     'definition': 'median of baseline.total_ms (the arm binary full solve) over the 6 records per arm and degree; ratio = before / after',
     'files': ['%s/raw/before_r{1,2,3}_d4.jsonl' % S, '%s/raw/after_r{1,2,3}_d4.jsonl' % S],
     'status': ok_ratio(ratio[4], '1.9')},
    {'claimed': '4.1× at d = 5',
     'computed': 'before %.2f ms / after %.2f ms = %.4f' % (med_ms[('before', 5)], med_ms[('after', 5)], ratio[5]),
     'definition': 'as above, d = 5',
     'files': ['%s/raw/before_r{1,2,3}_d5.jsonl' % S, '%s/raw/after_r{1,2,3}_d5.jsonl' % S],
     'status': ok_ratio(ratio[5], '4.1')},
    {'claimed': '6.4× at d = 6',
     'computed': 'before %.2f ms / after %.2f ms = %.4f' % (med_ms[('before', 6)], med_ms[('after', 6)], ratio[6]),
     'definition': 'as above, d = 6',
     'files': ['%s/raw/before_r{1,2,3}_d6.jsonl' % S, '%s/raw/after_r{1,2,3}_d6.jsonl' % S],
     'status': ok_ratio(ratio[6], '6.4')},
    {'claimed': '37.4 s → 5.9 s at d = 6',
     'computed': 'before median %.3f s -> %.3f ; after median %.3f s -> %.3f' % (med_ms[('before', 6)] / 1000, round(med_ms[('before', 6)] / 1000, 1), med_ms[('after', 6)] / 1000, round(med_ms[('after', 6)] / 1000, 1)),
     'definition': 'median baseline.total_ms over 6 records per arm at d = 6, shown to 0.1 s',
     'files': ['%s/raw/before_r{1,2,3}_d6.jsonl' % S, '%s/raw/after_r{1,2,3}_d6.jsonl' % S],
     'status': 'match' if (f1(med_ms[('before', 6)] / 1000) == '37.4' and f1(med_ms[('after', 6)] / 1000) == '5.9') else 'mismatch'},
    {'claimed': 'S5 164–203 s a solve',
     'computed': 'S5 d=8 baseline total_ms min %.1f s, max %.1f s (3 systems)' % (min(s5_ms) / 1000, max(s5_ms) / 1000),
     'definition': 'baseline.total_ms of the three s5 records (after-commit binary, full solve); min and max shown to 1 s',
     'files': [R + '/raw/s5_p269.jsonl'],
     'status': 'match' if (round(min(s5_ms) / 1000) == 164 and round(max(s5_ms) / 1000) == 203) else 'mismatch'},
    {'claimed': 'S5 about 30 min before',
     'computed': 'no before-commit S5 timing in the repo raw files; S5 raw holds the after binary only (s5_p269.jsonl). speed README: before "not timed here"',
     'definition': 'the before-commit S5 wall time is not in any committed raw file',
     'files': [S + '/README.md', R + '/raw/s5_p269.jsonl'],
     'status': 'no_raw_in_repo'},
    {'claimed': 'C₄ = 1.2269·10¹² unchanged (system 1)',
     'computed': 'system-1 baseline total_muls = %d = %.4e' % (s5_muls[1], s5_muls[1]),
     'definition': 'baseline.total_muls of s5 record with system 1',
     'files': [R + '/raw/s5_p269.jsonl'],
     'status': 'match' if ('%.4e' % s5_muls[1]) == '1.2269e+12' else 'mismatch'},
    {'claimed': '12 solves per arm and degree',
     'computed': '%d records per arm and degree (3 rounds x 2 systems); each record is one full solve and one replay (replay not counted as a full solve)' % records_per[('before', 4)],
     'definition': 'record count per arm/degree in raw/<arm>_r<round>_d<d>.jsonl; the 12 count is stated as "12 solves per arm and degree" with the replay counted as a solve',
     'files': [S + '/run.sh', S + '/README.md'],
     'status': 'partial'},
    {'claimed': 'every count and solution identical (MEASURED: all 36 solves per arm agree in total_muls)',
     'computed': 'before-vs-after pairs %d, equal on all count fields %d, differ %d (%s)' % (n_pairs, n_equal, n_pairs - n_equal, diff_detail or 'none'),
     'definition': 'pairs of same (round, degree, system) records; count fields = all baseline fields except *_ms, outcome and replay, plus baseline_solutions',
     'files': ['%s/raw/before_r{1,2,3}_d{4,5,6}.jsonl' % S, '%s/raw/after_r{1,2,3}_d{4,5,6}.jsonl' % S],
     'status': 'match' if n_equal == n_pairs else 'mismatch'},
]

identical = {
    'claimed': True,
    'n_compared': n_pairs,
    'n_equal': n_equal,
    'n_differ': n_pairs - n_equal,
    'files': ['%s/raw/before_r{1,2,3}_d{4,5,6}.jsonl' % S, '%s/raw/after_r{1,2,3}_d{4,5,6}.jsonl' % S],
    'status': 'match' if n_equal == n_pairs else 'mismatch',
}

def grep_lines(path, pattern):
    out = []
    with open(os.path.join(ROOT, path)) as f:
        for i, line in enumerate(f, 1):
            if re.search(pattern, line):
                out.append('%s:%d: %s' % (path, i, line.rstrip('\n')))
    return out

docs = [S + '/README.md', S + '/REPORT.typ', S + '/run.sh', R + '/README.md']
offrepo_refs, aa_lines, isolation_lines = [], [], []
for p in docs:
    offrepo_refs += grep_lines(p, r'drive\.google|/Volumes|SSD|/Users|s3://|gs://|store:|Modal|not committed|scratch')
    aa_lines += grep_lines(p, r'A/A|null-control|null control|negative control|positive control')
    isolation_lines += grep_lines(p, r'isolat|shared|idle|reserved|Xeon|vCPU|nothing else')

row = {
    'row_no': 106,
    'slug': '20261013-gaudry-quartic-solve-speed',
    'claims': claims,
    'identical': identical,
    'offrepo_refs': offrepo_refs,
    'aa_lines': aa_lines,
    'isolation_lines': isolation_lines,
    'protocol_git_date': 'none: no PROTOCOL file in research/gaudry_quartic_speed_20261013 or research/gaudry_quartic_replay_20261013; nearest committed spec %s/run.sh: ca00c151764f7c3221b22a43a138887f171e051d 2026-10-17T10:19:32-07:00' % S,
    'first_raw_git_date': 'ca00c151764f7c3221b22a43a138887f171e051d 2026-10-17T10:19:32-07:00 (git log --diff-filter=A -- %s/raw/generic_d3-5.jsonl, first raw file in RAW CANDIDATES order)' % R,
    'script': 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/106.py',
    'notes': 'Speed raw files hold 2 records each (system 0 and 1), 3 rounds per arm per degree: 6 records per arm and degree. binaries.txt lists 2 SHA-256 prefixes (before 7c28d317b2121cf5, after aeefa7cbe9f46902). S5 wall times are in the replay raw file and its log (after binary only). Checked the SHA-256 of the run.sh-named binaries: not done (no binary run). Speed records also contain replay blocks, not used for timing.',
}

os.makedirs(os.path.dirname(OUT), exist_ok=True)
with open(OUT, 'a') as f:
    f.write(json.dumps(row, ensure_ascii=False) + '\n')
print('row 106 written; claims:', len(claims))
for c in claims:
    print(c['status'], '|', c['claimed'], '|', c['computed'])
print('identical:', json.dumps(identical))
print('offrepo_refs:', len(offrepo_refs), 'aa_lines:', len(aa_lines), 'isolation_lines:', len(isolation_lines))
