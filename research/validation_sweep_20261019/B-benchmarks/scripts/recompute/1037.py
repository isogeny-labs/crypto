#!/usr/bin/env python3
# Row 1037 (20261005-ic-semiregular-law). Reads committed files only; prints one JSON object.
import glob, json, os, re, subprocess, statistics

ROOT = '/home/user/crypto'
os.chdir(ROOT)
ROW = 1037
SLUG = '20261005-ic-semiregular-law'
STUDY_DIR = 'research/ic_semiregular_law_20261005'
RUNS = STUDY_DIR + '/runs'
PRED = RUNS + '/registered/predictions.jsonl'
PROTO = STUDY_DIR + '/PREREGISTRATION.md'
FIRST_RAW = RUNS + '/registered/K0n61l4.jsonl'
SCRIPT = 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/1037.py'

OFF_RX = re.compile(r'drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:[A-Za-z0-9_./-]+|Modal volume|not committed|scratch')
AA_RX = re.compile(r'\bA/A\b|null[- ]control|negative control|\bnull (?:result|control|arm)|\bctrl\b', re.I)
ISO_RX = re.compile(r'isolat|isolcpus|nohz|CPU pinning|frequency control|governor|bare metal|wall times advisory', re.I)


def files_under(paths, exts=('.md', '.typ', '.txt', '.json', '.sh', '.py')):
    out = []
    for p in paths:
        if os.path.isfile(p):
            out.append(p)
            continue
        for dp, _, fn in os.walk(p):
            for f in fn:
                q = os.path.join(dp, f)
                if f.endswith(exts) and os.path.getsize(q) < 2_000_000:
                    out.append(q)
    return sorted(out)


def grep(files, rx, cap=200):
    hits = []
    for q in files:
        with open(q, encoding='utf-8', errors='replace') as h:
            for i, line in enumerate(h, 1):
                if rx.search(line):
                    hits.append(f'{q}:{i}: {line.strip()[:260]}')
                    if len(hits) >= cap:
                        return hits
    return hits


def git_added(path):
    if not path:
        return ''
    r = subprocess.run(['git', 'log', '--diff-filter=A', '--format=%H %cI', '--', path],
                       capture_output=True, text=True, cwd=ROOT)
    lines = [x for x in r.stdout.splitlines() if x.strip()]
    return lines[-1] if lines else ''


def main():
    files = sorted(glob.glob(RUNS + '/registered*/K*.jsonl'))
    units = {}
    recs = []
    for f in files:
        d = f.split('/')[-2]
        for line in open(f, encoding='utf-8'):
            if line.strip():
                r = json.loads(line)
                r['_dir'] = d
                recs.append(r)
                units.setdefault((r['cell'], r['arm'], r['draw']), []).append(r)

    reading = {}
    for k, v in units.items():
        nz = sorted({r['refuted_at'] for r in v if r['refuted_at'] > 0})
        reading[k] = nz[0] if len(nz) == 1 else None
    resolved = [k for k in units if reading[k] is not None]

    # windows as registered in predictions.jsonl (per cell, arm, draw)
    win = {}
    for line in open(PRED, encoding='utf-8'):
        if line.strip():
            p = json.loads(line)
            win[(p['cell'], p['arm'], p['draw'])] = tuple(p['window'])

    def in_win(k):
        lo, hi = win[k]
        return lo <= reading[k] <= hi

    x4 = [k for k in units if k[1] == 'x4']
    rr = [k for k in units if k[1] == 'rr']
    x4_in = sum(1 for k in x4 if in_win(k))
    rr_below = sum(1 for k in rr if reading[k] < win[k][0])
    rr_in = sum(1 for k in rr if in_win(k))
    rr_above = sum(1 for k in rr if reading[k] > win[k][1])

    # heavy degree-10 solves
    big = [r for r in recs if r['dmax'] == 10 and r['n_cols'] == 199140]
    rows_lo = min(r['n_rows'] for r in big)
    rows_hi = max(r['n_rows'] for r in big)
    cols = sorted({r['n_cols'] for r in big})
    loc = [r['wall'] for r in big if r['_dir'] == 'registered']
    rem = [r['wall'] for r in big if r['_dir'] != 'registered']
    rem_min = min(rem) / 60 if rem else None
    rem_max = max(rem) / 60 if rem else None
    loc_min = min(loc) / 60 if loc else None
    loc_max = max(loc) / 60 if loc else None

    def within(c, x):
        return x is not None and abs(x - c) / c <= 0.01

    claims = [
        {'claimed': '68/68 units resolved',
         'computed': f'units={len(units)}; resolved={len(resolved)}',
         'definition': ('units = distinct (cell, arm, draw) over runs/registered*/K*.jsonl; '
                        'resolved = some record of the unit has refuted_at > 0'),
         'files': files,
         'status': 'match' if (len(units), len(resolved)) == (68, 68) else 'mismatch'},
        {'claimed': 'x4 meets its generic degree window on 32/32 draws',
         'computed': f'x4 units={len(x4)}; inside registered window={x4_in}',
         'definition': 'x4 units with window[0] <= refutation degree <= window[1] (window per cell, arm, draw from runs/registered/predictions.jsonl)',
         'files': files + [PRED],
         'status': 'match' if (len(x4), x4_in) == (32, 32) else 'mismatch'},
        {'claimed': 'rr falls below on 8',
         'computed': f'rr units={len(rr)}; below={rr_below}; inside={rr_in}; above={rr_above}',
         'definition': 'rr units with refutation degree < window[0]',
         'files': files + [PRED],
         'status': 'match' if rr_below == 8 else 'mismatch'},
        {'claimed': 'Heavy degree-10 solves: 239k-247k rows x 199k columns',
         'computed': f'dmax=10 records with n_cols=199140: n={len(big)}, n_rows min={rows_lo} max={rows_hi}, n_cols values={cols}',
         'definition': 'n_rows min and max over dmax=10 records with n_cols=199140 in runs/registered*/K*.jsonl',
         'files': files,
         'status': 'match' if (round(rows_lo / 1000), round(rows_hi / 1000)) == (239, 247) and cols == [199140] else 'mismatch'},
        {'claimed': '42-51 min on 4 threads',
         'computed': (f'remote-dir dmax=10 n_cols=199140 wall min={rem_min:.2f} min max={rem_max:.2f} min '
                      f'(n={len(rem)})' if rem else 'no remote records'),
         'definition': ('wall/60 over dmax=10, n_cols=199140 records in runs/registered-remote-*; the thread count is not '
                        'a field of the records and is taken from RESULTS.md ("three remote 4-core hosts")'),
         'files': [f for f in files if '/registered-remote' in f],
         'status': ('match' if within(42, rem_min) and within(51, rem_max) else
                    ('partial' if rem_min is not None and round(rem_min) == 42 and round(rem_max) == 51 else 'mismatch'))},
        {'claimed': 'about 90 min on 2 threads',
         'computed': (f'local-dir dmax=10 n_cols=199140 wall min={loc_min:.2f} min max={loc_max:.2f} min (n={len(loc)})'),
         'definition': ('wall/60 over dmax=10, n_cols=199140 records in runs/registered (local lanes, two 2-thread lanes '
                        'per RESULTS.md header)'),
         'files': [f for f in files if f.startswith(RUNS + '/registered/')],
         'status': 'partial' if loc_max is not None and within(90, loc_max) and not within(90, loc_min) else
                   ('match' if loc_min is not None and within(90, loc_min) and within(90, loc_max) else 'mismatch')},
    ]

    studyfiles = files_under([STUDY_DIR + '/RESULTS.md', STUDY_DIR + '/PREREGISTRATION.md',
                              STUDY_DIR + '/reports', STUDY_DIR + '/GOAL.md'])
    notes = ('units with no refutation or two distinct refutation values: none. '
             'the readout labels one rr draw (K1n23l5 draw 2) "triv"; its raw refuted_at is 1 at dmax 1 with rank 0, '
             'and its registered window is [0, 1] per PREREGISTRATION.md. '
             'the verdict "neither" is not recomputed (it is printed by analyze.py). '
             'protocol: PREREGISTRATION.md at the study root. '
             'first_raw = runs/registered/K0n61l4.jsonl, chosen as the first registered file by name.')
    out = {
        'row_no': 1000 + 37,
        'slug': SLUG,
        'claims': claims,
        'identical': {'claimed': False, 'n_compared': None, 'n_equal': None, 'n_differ': None,
                      'files': [], 'status': 'not_claimed_between_arms'},
        'offrepo_refs': grep(files_under([STUDY_DIR]), OFF_RX),
        'aa_lines': grep(files_under([STUDY_DIR]), AA_RX),
        'isolation_lines': grep(files_under([STUDY_DIR]), ISO_RX),
        'protocol_git_date': git_added(PROTO),
        'first_raw_git_date': git_added(FIRST_RAW),
        'script': SCRIPT,
        'notes': notes,
    }
    print(json.dumps(out, ensure_ascii=False))


if __name__ == '__main__':
    main()
