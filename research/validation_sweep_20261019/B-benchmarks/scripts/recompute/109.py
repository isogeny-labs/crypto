#!/usr/bin/env python3
# SWB-RECOMPUTE-14 row 109: 20261013-nist-gpu-rho-ladder. Reads committed files only; appends one JSON object.
import csv, json, os, re, subprocess

ROOT = '/home/user/crypto'
OUT = os.path.join(ROOT, 'research/validation_sweep_20261019/B-benchmarks/recompute/SWB-RECOMPUTE-14.jsonl')
B = 'benchmarks/20261013-nist-gpu-rho-ladder'
G = 'gpu/nist'

def read_rows(path):
    with open(os.path.join(ROOT, path)) as f:
        lines = [l for l in f if not l.startswith('#') and l.strip()]
    return list(csv.DictReader(lines))

rows = read_rows(B + '/data.csv')
def pick(rnd, build, thr, w, maxreg):
    out = [r for r in rows if r['round'] == rnd and r['build'] == build and r['threads'] == str(thr) and r['W'] == str(w) and r['maxreg'] == str(maxreg)]
    assert len(out) == 1, (rnd, build, thr, w, maxreg, len(out))
    return float(out[0]['P256_billion_it_s']), float(out[0]['P384_billion_it_s'])

first_portable = pick('first', 'portable', 128, 8, 0)
first_ptx = pick('first', 'ptx', 128, 8, 0)
w16_256 = pick('1', 'ptx', 256, 16, 0)
w32_256 = pick('2', 'ptx', 256, 32, 0)
caps = {mr: pick('2', 'ptx', 256, 32, mr) for mr in (96, 128, 160)}

ptx_ratio = (first_ptx[0] / first_portable[0], first_ptx[1] / first_portable[1])
cum_ratio = (w32_256[0] / first_portable[0], w32_256[1] / first_portable[1])

# DP counts from logs
dp_lines = []
for p in (G + '/sweep3.log', G + '/cell-shared-256-64.log'):
    with open(os.path.join(ROOT, p)) as f:
        for i, line in enumerate(f, 1):
            m = re.search(r'\((\d+) DPs, dp_bits=(\d+)\)', line)
            if m:
                dp_lines.append((p, i, int(m.group(1)), int(m.group(2))))
dp_bits = sorted(set(x[3] for x in dp_lines))
obs = [x[2] for x in dp_lines]

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

docs = [G + '/RESULTS.md', G + '/README.md', 'paper/ecc2k130/sec-nist.tex', B + '/REPORT.tex']
offrepo_refs, aa_lines, isolation_lines = [], [], []
for p in docs:
    offrepo_refs += grep_lines(p, r'drive\.google|/Volumes|SSD|/Users|s3://|gs://|store:|Modal volume|modal volume|modal\.com')
    aa_lines += grep_lines(p, r'A/A|null-control|null control|negative control|positive control')
    isolation_lines += grep_lines(p, r'isolat|receipt|reserved|idle|shared host|Xeon|vCPU')

def git_add(path):
    r = subprocess.run(['git', '-C', ROOT, 'log', '--diff-filter=A', '--format=%H %cI', '--', path], capture_output=True, text=True)
    return r.stdout.strip() or 'empty'

claims = [
    {'claimed': 'first working build 1.075 / 0.210 billion iterations/s (P-256 / P-384)',
     'computed': 'data.csv first,portable,128,8,0 = %s / %s' % first_portable,
     'definition': 'P256 and P384 columns of the first portable row (control build, threads 128, W 8, maxreg 0); RESULTS.md values are copied into data.csv',
     'files': [B + '/data.csv'],
     'status': 'match' if (first_portable == (1.075, 0.210)) else 'mismatch'},
    {'claimed': 'PTX carry chains 1.19× / 1.38×',
     'computed': 'ptx / portable at first run, threads 128, W 8: P-256 %.4f ; P-384 %.4f' % ptx_ratio,
     'definition': 'ratio of the first ptx row to the first portable row, per curve',
     'files': [B + '/data.csv'],
     'status': 'match' if (round(ptx_ratio[0], 2) == 1.19 and round(ptx_ratio[1], 2) == 1.38) else 'mismatch'},
    {'claimed': 'W=16, 256 thr 1.340 / 0.438',
     'computed': 'data.csv round 1 ptx 256 threads, W 16, maxreg 0 = %s / %s' % w16_256,
     'definition': 'value of the matching row',
     'files': [B + '/data.csv'],
     'status': 'match' if (round(w16_256[0], 3) == 1.340 and round(w16_256[1], 3) == 0.438) else 'mismatch'},
    {'claimed': 'W=32 with by-reference inversion 1.632 / 0.548 (round 2, PR 1778)',
     'computed': 'data.csv round 2 ptx 256 threads, W 32, maxreg 0 = %s / %s' % w32_256,
     'definition': 'value of the matching row; PR 1778 is cited in the CSV header, not checked',
     'files': [B + '/data.csv'],
     'status': 'match' if (round(w32_256[0], 3) == 1.632 and round(w32_256[1], 3) == 0.548) else 'mismatch'},
    {'claimed': 'cumulative 1.52× / 2.61× from the first working measurement',
     'computed': 'round-2 W=32 cell / first portable: P-256 %.4f ; P-384 %.4f' % cum_ratio,
     'definition': 'ratio of the W=32 cell (round 2, ptx, 256 threads, maxreg 0) to the first portable row, per curve',
     'files': [B + '/data.csv'],
     'status': 'match' if (round(cum_ratio[0], 2) == 1.52 and round(cum_ratio[1], 2) == 2.61) else 'mismatch'},
    {'claimed': 'register caps 96/128/160 all lose',
     'computed': 'caps vs unrestricted round-2 W=32 256 thr: %s ; unrestricted = %s' % ({str(k): v for k, v in caps.items()}, w32_256),
     'definition': 'each capped cell compared with the unrestricted cell of the same shape; loses when both curves are lower',
     'files': [B + '/data.csv'],
     'status': 'match' if all(caps[k][0] < w32_256[0] and caps[k][1] < w32_256[1] for k in caps) else 'mismatch'},
    {'claimed': 'DP counts match the 2^-24 rate in every cell',
     'computed': 'observed DP counts across %d logged cells (dp_bits values %s): min %d, max %d ; no expected count per cell is recorded in the raw logs; RESULTS.md gives expected 12,096 for the best cell only, with no stated step count' % (len(dp_lines), dp_bits, min(obs), max(obs)),
     'definition': 'expected DP count needs the per-cell step count and the 2^-24 rule written out; neither is in the raw logs',
     'files': [G + '/sweep3.log', G + '/cell-shared-256-64.log', G + '/RESULTS.md'],
     'status': 'cannot_define'},
    {'claimed': 'engineering, S flat',
     'computed': 'S is not recorded in any raw file of this row (no counted-cost field in data.csv or the logs)',
     'definition': 'the counted-cost S of the GPU walk is not defined in the committed raw files',
     'files': [B + '/data.csv', G + '/sweep3.log'],
     'status': 'cannot_define'},
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
    'row_no': 109,
    'slug': '20261013-nist-gpu-rho-ladder',
    'claims': claims,
    'identical': identical,
    'offrepo_refs': offrepo_refs,
    'aa_lines': aa_lines,
    'isolation_lines': isolation_lines,
    'protocol_git_date': 'none: no PROTOCOL file in %s, %s or %s' % (G, B, 'paper/ecc2k130'),
    'first_raw_git_date': 'data.csv: %s ; sweep3.log: %s' % (git_add(B + '/data.csv'), git_add(G + '/sweep3.log')),
    'script': 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/109.py',
    'notes': 'data.csv is a recorded copy of RESULTS.md values (its own header says so); the claims were checked against data.csv, not against a device run. Logs: %d DP lines in sweep3.log and cell-shared-256-64.log. Study text has no isolation or A/A statement (matches in isolation_lines are unrelated words). Round-3 log lines (W=64, shared table) are not in data.csv and are not used by any headline claim here.' % len(dp_lines),
}

os.makedirs(os.path.dirname(OUT), exist_ok=True)
with open(OUT, 'a') as f:
    f.write(json.dumps(row, ensure_ascii=False) + '\n')
print('row 109 written; claims:', len(claims))
for c in claims:
    print(c['status'], '|', c['claimed'], '|', c['computed'])
print('offrepo_refs:', len(offrepo_refs), 'aa_lines:', len(aa_lines), 'isolation_lines:', len(isolation_lines))
print('dp lines:', len(dp_lines), 'min', min(obs), 'max', max(obs))
