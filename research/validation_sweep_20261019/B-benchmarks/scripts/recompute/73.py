import json, math, glob, os, re, subprocess
REPO = '/home/user/crypto'
ROW = 73
STUDY = 'research/ic_improve_20261014/04-genus3-2lp'
RUN = os.path.join(REPO, STUDY, 'results/run-20261014')
XC = os.path.join(REPO, STUDY, 'results/xcheck')
I7 = os.path.join(REPO, 'research/ic_next10_20261014/07-cubic-ghs-genus3/results/run-20261014/solves.csv')
SELF = 'scripts/recompute/73.py'

recs = {}
for f in sorted(glob.glob(os.path.join(RUN, 'b*_c*_*.jsonl'))):
    m = re.search(r'b(\d+)_c(\d+)_', os.path.basename(f))
    for line in open(f):
        d = json.loads(line)
        for r in d['results']:
            if r['arm'] not in ('basic', 'ic2lp', 'ic1lp', 'aa'):
                continue
            key = (r['arm'], d['bits'], d['curve'])
            assert key not in recs, key
            recs[key] = {'q': d['q'], 'cost': r['cost'], 'la': r['la_counters'][2],
                         'correct': r['correct'], 'k_ok': r['k'] == r['k_true']}

def pts(arm, lo, hi, field='cost'):
    xs, ys = [], []
    for (a, b, c), v in recs.items():
        if a == arm and lo <= b <= hi:
            xs.append(math.log(v['q'])); ys.append(math.log(v[field]))
    return xs, ys

def ols(xs, ys):
    n = len(xs); mx = sum(xs) / n; my = sum(ys) / n
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)

def gm(arm, b, field='cost'):
    v = [recs[(arm, b, c)][field] for c in range(3)]
    return math.exp(sum(math.log(x) for x in v) / len(v))

def lnq_mean(arm, b):
    return sum(math.log(recs[(arm, b, c)]['q']) for c in range(3)) / 3

def ok(claimed, value, dec):
    return abs(value - claimed) / abs(claimed) <= 0.01 or round(value, dec) == round(claimed, dec)

claims = []
run_files = ['results/run-20261014/b*_c*_all.jsonl', 'results/run-20261014/b1[6-9]_c*_basic.jsonl',
             'results/run-20261014/b1[6-9]_c*_lp.jsonl', 'results/run-20261014/b*_c*_ic1lp.jsonl',
             'results/run-20261014/b*_c*_ic2lp.jsonl']

def add(claimed_str, value, claimed_num, dec, definition, status_override=None):
    st = status_override or ('match' if ok(claimed_num, value, dec) else 'mismatch')
    claims.append({'claimed': claimed_str, 'computed': f'{value:.{dec}f}', 'definition': definition,
                   'files': [os.path.join(STUDY, f) for f in run_files], 'status': st})

s_2lp = ols(*pts('ic2lp', 10, 18)); s_b = ols(*pts('basic', 10, 18)); s_1 = ols(*pts('ic1lp', 10, 18))
add('q^1.385 [1.379, 1.391] (IC-2LP, 2^10-2^18)', s_2lp, 1.385, 3,
    'OLS slope of ln(cost) on ln(q) over all solves 2^10-2^18 (3 curves each); CI not recomputed (Rust bootstrap, seed not in repo)', 'partial')
add('q^1.910 [1.904, 1.913] (IC-basic, 2^10-2^18)', s_b, 1.910, 3,
    'OLS slope, IC-basic, all solves 2^10-2^18; CI not recomputed', 'partial')
add('Delta -0.525 [-0.529, -0.521] (2LP minus basic exponent)', s_2lp - s_b, -0.525, 3,
    'slope(ic2lp) - slope(basic) over 2^10-2^18; CI not recomputed', 'partial')
r10 = gm('ic2lp', 10) / gm('basic', 10)
add('paired ratio 0.620 at 2^10 (IC-2LP/IC-basic)', r10, 0.620, 3,
    'geometric mean of 3 curves IC-2LP cost / geometric mean of 3 curves IC-basic cost, bits=10')
r18 = gm('ic2lp', 18) / gm('basic', 18)
add('paired ratio 0.036 at 2^18 (IC-2LP/IC-basic)', r18, 0.036, 3,
    'as above, bits=18')
add('q^1.446 (tuned IC-1LP, 2^10-2^18)', s_1, 1.446, 3,
    'OLS slope, IC-1LP, all solves 2^10-2^18; CI not recomputed', 'partial')
loc = (math.log(gm('ic2lp', 20)) - math.log(gm('ic2lp', 19))) / (lnq_mean('ic2lp', 20) - lnq_mean('ic2lp', 19))
add('IC-2LP local exponent 1.46 at 2^20 (interval 2^19-2^20)', loc, 1.46, 2,
    'd ln(geomean cost) / d ln(q) between bits 19 and 20 (geomean over 3 curves; mean ln q per size)')
s_la = ols(*pts('ic2lp', 10, 20, field='la'))
add('linear algebra q^1.52 (IC-2LP, 2^10-2^20)', s_la, 1.52, 2,
    'OLS slope of ln(la_counters[2]) on ln(q), IC-2LP, bits 10-20; la_counters[2] taken as the linear-algebra cost (matches item 7 la_cost field)')

# identical claim: port (xcheck) vs item 7 Python (solves.csv)
import csv
i7 = {}
with open(I7) as fh:
    for row in csv.DictReader(fh):
        i7[(row['arm'], int(row['bits']), int(row['curve']))] = row
xc = {}
for f in sorted(glob.glob(os.path.join(XC, 'ic_b*_c*.jsonl'))):
    m = re.search(r'ic_b(\d+)_c(\d+)', os.path.basename(f))
    for line in open(f):
        d = json.loads(line)
        for r in d['results']:
            if r['arm'] in ('basic', 'ic2lp'):
                xc[(r['arm'], d['bits'], d['curve'])] = (r, d)
keys = sorted(set(xc) & set(i7))
fields = [('cost', lambda r, d: r['cost'], 'cost'), ('cost_cl9', lambda r, d: r['cost_cl9'], 'cost_cl9'),
          ('steps', lambda r, d: r['steps'], 'steps'), ('nfb', lambda r, d: r['nfb'], 'nfb'),
          ('rows', lambda r, d: r['rows'], 'rows'), ('restarts', lambda r, d: r['restarts'], 'restarts'),
          ('full', lambda r, d: r['full'], 'full'), ('lp1', lambda r, d: r['lp1'], 'lp1'),
          ('lp2', lambda r, d: r['lp2'], 'lp2'), ('cycles', lambda r, d: r['cycles'], 'cycles'),
          ('la_cost', lambda r, d: r['la_counters'][2], 'la_cost')]
n_eq = n_diff = 0
for k in keys:
    r, d = xc[k]; row = i7[k]
    same = all(int(float(row[cf])) == int(fn(r, d)) for _, fn, cf in fields)
    if same: n_eq += 1
    else: n_diff += 1
xc_files = sorted(glob.glob(os.path.join(XC, 'ic_b*_c*.jsonl')))
ident = {'claimed': True, 'n_compared': len(keys), 'n_equal': n_eq, 'n_differ': n_diff,
         'files': [os.path.join(STUDY, 'results/xcheck', os.path.basename(f)) for f in xc_files] +
                  ['research/ic_next10_20261014/07-cubic-ghs-genus3/results/run-20261014/solves.csv'],
         'status': 'partial'}

def git_date(path):
    out = subprocess.run(['git', '-C', REPO, 'log', '--diff-filter=A', '--format=%H %cI', '--', path],
                         capture_output=True, text=True).stdout.strip()
    return out if out else 'EMPTY'

protocol = os.path.join(STUDY, 'PROTOCOL.md')
first_raw = os.path.join(STUDY, 'results/run-20261014/b15_c0_all.jsonl')

def grep_lines(path, needles):
    out = []
    with open(os.path.join(REPO, path)) as fh:
        for i, line in enumerate(fh, 1):
            if any(n in line for n in needles):
                out.append(f'{path}:{i}: {line.rstrip()}')
    return out

aa = grep_lines(os.path.join(STUDY, 'README.md'), ['**A/A** (IC-2LP on an independent'])
iso = grep_lines(os.path.join(STUDY, 'README.md'), ['shared machine', 'shared, loaded host']) + \
      grep_lines(os.path.join(STUDY, 'PROTOCOL.md'), ['shared machine'])

out = {'row_no': ROW, 'slug': '20261014-ic-improve-04-genus3-2lp', 'claims': claims,
       'identical': ident, 'offrepo_refs': [], 'aa_lines': aa, 'isolation_lines': iso,
       'protocol_git_date': git_date(protocol), 'first_raw_git_date': git_date(first_raw),
       'script': SELF,
       'notes': ('Off-repo grep (drive.google.com|/Volumes|SSD|/Users|s3://|gs://|store:|Modal|not committed|scratch) over study .md/.json files: no hits. '
                 'Identical block compares xcheck port records with item 7 solves.csv on cost, cost_cl9, steps, nfb, rows, restarts, full, lp1, lp2, cycles, la_cost; '
                 'k and the five counters are not in solves.csv, so they were not compared. '
                 'CIs for exponents not recomputed (bootstrap RNG not in repo). '
                 'first_raw = first RAW CANDIDATES entry as listed on the card. '
                 'q taken from record field q (not 2^bits).')}
print(json.dumps(out, ensure_ascii=False))
