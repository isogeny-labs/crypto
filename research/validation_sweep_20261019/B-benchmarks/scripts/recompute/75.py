import csv, json, math, os, re, subprocess
import numpy as np
REPO = '/home/user/crypto'
ROW = 75
STUDY = 'research/ic_next10_20261014/07-cubic-ghs-genus3'
RUN = os.path.join(REPO, STUDY, 'results/run-20261014')
SELF = 'scripts/recompute/75.py'
rows = list(csv.DictReader(open(os.path.join(RUN, 'solves.csv'))))
for r in rows:
    r['bits'] = int(r['bits']); r['q'] = int(r['q']); r['curve'] = int(r['curve'])
    r['cost'] = float(r['cost']); r['steps'] = float(r['steps']); r['ell'] = int(r['ell'])
    r['correct'] = r['correct'] == 'True'

def fit(arm, lo, hi):
    pts = [r for r in rows if r['arm'] == arm and lo <= r['bits'] <= hi]
    x = np.log([p['q'] for p in pts]); y = np.log([p['cost'] for p in pts])
    A = np.vstack([x, np.ones_like(x)]).T
    (e, a), *_ = np.linalg.lstsq(A, y, rcond=None)
    return e, len(pts)

# rho model per size, as analyze.py: pstep = mean(cost/steps) over rho rows; mean sqrt(ell) over curves
sizes = sorted({r['bits'] for r in rows})
mt = []
for b in sizes:
    rh = [r for r in rows if r['bits'] == b and r['arm'] == 'rho']
    ic = [r for r in rows if r['bits'] == b and r['arm'] == 'ic2lp']
    if not rh or not ic: continue
    ells = {r['curve']: r['ell'] for r in rh}
    pstep = np.mean([r['cost'] / r['steps'] for r in rh])
    msq = np.mean([math.sqrt(e) for e in ells.values()])
    mt.append((rh[0]['q'], pstep * math.sqrt(math.pi / 2) * msq))
em = np.polyfit(np.log([q for q, _ in mt]), np.log([m for _, m in mt]), 1)[0]

e_all, n_all = fit('ic2lp', 8, 15)
e_b, n_b = fit('basic', 8, 13)
e_1115, n_1115 = fit('ic2lp', 11, 15)
ic_ok = sum(1 for r in rows if r['arm'] == 'ic2lp' and r['correct'])
ic_n = sum(1 for r in rows if r['arm'] == 'ic2lp')
b_ok = sum(1 for r in rows if r['arm'] == 'basic' and r['correct'])
b_n = sum(1 for r in rows if r['arm'] == 'basic')
max_bits = max(r['bits'] for r in rows if r['arm'] == 'ic2lp')
fl = [os.path.join(STUDY, 'results/run-20261014/solves.csv')]

def st(c, v, dec):
    return 'match' if abs(v - c) / abs(c) <= 0.01 or round(v, dec) == round(c, dec) else 'mismatch'

claims = [
    {'claimed': 'q^1.650 [1.639, 1.660] (IC-basic, primary unit)', 'computed': f'{e_b:.3f}',
     'definition': f'OLS of ln cost on ln q over all basic solves bits 8-13 (n={n_b}); CI not recomputed (bootstrap over curves, Python rng seed 20261014 in analyze.py but not replayed here)',
     'files': fl, 'status': 'partial'},
    {'claimed': 'q^1.300 [1.292, 1.308] (IC-2LP, all sizes)', 'computed': f'{e_all:.3f}',
     'definition': f'OLS of ln cost on ln q over all ic2lp solves bits 8-15 (n={n_all}); CI not recomputed',
     'files': fl, 'status': 'partial'},
    {'claimed': 'q^1.377 over 2^11-2^15 (IC-2LP)', 'computed': f'{e_1115:.3f}',
     'definition': f'OLS of ln cost on ln q over ic2lp solves bits 11-15 (n={n_1115})',
     'files': fl, 'status': st(1.377, e_1115, 3)},
    {'claimed': '24/24 IC solves verified', 'computed': f'{ic_ok}/{ic_n}',
     'definition': 'count of correct==True over arm ic2lp rows of solves.csv', 'files': fl,
     'status': 'match' if (ic_ok, ic_n) == (24, 24) else 'mismatch'},
    {'claimed': '18/18 full-factor-base control solves verified', 'computed': f'{b_ok}/{b_n}',
     'definition': 'count of correct==True over arm basic rows of solves.csv', 'files': fl,
     'status': 'match' if (b_ok, b_n) == (18, 18) else 'mismatch'},
    {'claimed': 'transfer reaches q=2^15', 'computed': f'max bits {max_bits} (ic2lp)',
     'definition': 'maximum bits field over ic2lp rows of solves.csv', 'files': fl,
     'status': 'match' if max_bits == 15 else 'mismatch'},
    {'claimed': 'rho model q^1.53', 'computed': f'{em:.3f}',
     'definition': 'analyze.py rule: per size pstep=mean(cost/steps) over rho rows, rho_model=pstep*sqrt(pi/2)*mean sqrt(ell) over curves, OLS of ln model on ln q over sizes with ic2lp present',
     'files': fl + [os.path.join(STUDY, 'analyze.py')], 'status': st(1.53, em, 2)},
]

def grep_lines(rel, pat):
    out = []
    with open(os.path.join(REPO, STUDY, rel)) as fh:
        for i, line in enumerate(fh, 1):
            if re.search(pat, line):
                out.append(f'{STUDY}/{rel}:{i}: {line.rstrip()}')
    return out

aa = grep_lines('README.md', r'control|[Nn]ull') + grep_lines('PROTOCOL.md', r'[Nn]ull control|control')
iso = grep_lines('README.md', r'single-threaded|GB cap|shared|isolat') + grep_lines('PROTOCOL.md', r'shared machine|isolat')

def git_date(path):
    out = subprocess.run(['git', '-C', REPO, 'log', '--diff-filter=A', '--format=%H %cI', '--', path], capture_output=True, text=True).stdout.strip()
    return out if out else 'EMPTY'

out = {'row_no': ROW, 'slug': '20261014-ic-next10-07-cubic-ghs-genus3', 'claims': claims,
       'identical': {'claimed': False, 'n_compared': None, 'n_equal': None, 'n_differ': None, 'files': [], 'status': 'no_claim'},
       'offrepo_refs': [], 'aa_lines': aa, 'isolation_lines': iso,
       'protocol_git_date': git_date(os.path.join(STUDY, 'PROTOCOL.md')),
       'first_raw_git_date': git_date(os.path.join(STUDY, 'results/pilot_kappa_k2.jsonl')),
       'script': SELF,
       'notes': ('Exponents recomputed from solves.csv (run-20261014) with OLS as in analyze.py fit(); bootstrap CIs not recomputed. '
                 'Off-repo grep (drive.google.com|/Volumes|SSD|/Users|s3://|gs://|store:|Modal|not committed|scratch) over study .md/.json/.txt files: no hits. '
                 'Rho model exponent uses the analyze.py rule. The rho measured exponent and the crossover values are not claimed in the headline and were not computed. '
                 'The q^1.650 figure is computed on the basic arm over bits 8-13 as in summary.json.')}
print(json.dumps(out, ensure_ascii=False))
