import json, math, os, subprocess
REPO = '/home/user/crypto'
ROW = 74
STUDY = 'research/ic_next10_20261014/09-equal-advice'
D = os.path.join(REPO, STUDY, 'data')
SELF = 'scripts/recompute/74.py'

def is_prime(n):
    if n < 2: return False
    for p in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]:
        if n % p == 0: return n == p
    d, s = n - 1, 0
    while d % 2 == 0: d //= 2; s += 1
    for a in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]:
        x = pow(a, d, n)
        if x in (1, n - 1): continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1: break
        else: return False
    return True

cen = [json.loads(l) for l in open(os.path.join(D, 'census_n61_K600.jsonl'))]
summ = [r for r in cen if r.get('kind') == 'free_relation_census'][0]
rels = [r for r in cen if r.get('kind') == 'free_relation']
r_mod = summ['r']
prime = is_prime(r_mod)
cols = set()
rows = []
for rel in rels:
    row = {}
    for idx, c in rel['support']:
        cols.add(idx); row[idx] = (row.get(idx, 0) + c) % r_mod
    rows.append({k: v for k, v in row.items() if v})

def rank_mod(rows, p):
    piv = {}
    rk = 0
    for row in rows:
        row = dict(row)
        while row:
            col = min(row)
            if col in piv:
                prow = piv[col]; f = row[col]
                for k, v in prow.items():
                    nv = (row.get(k, 0) - f * v) % p
                    if nv: row[k] = nv
                    else: row.pop(k, None)
            else:
                inv = pow(row[col], -1, p)
                piv[col] = {k: v * inv % p for k, v in row.items()}
                rk += 1
                break
    return rk

rank = rank_mod(rows, r_mod) if prime else None

ic_files = {'41': 'ic_n41_K100.jsonl', '53': 'ic_n53_K200.jsonl', '61a': 'ic_n61_K256.jsonl', '61b': 'ic_n61_K360.jsonl'}
bl_files = {'41': 'bl_n41.jsonl', '53': 'bl_n53.jsonl', '61a': 'bl_n61_64th.jsonl', '61b': 'bl_n61_64th.jsonl'}
ratios = {}
for key in ic_files:
    ic = [json.loads(l)['online_ms'] for l in open(os.path.join(D, ic_files[key])) if json.loads(l).get('kind') == 'compact_orbit_dlp_target']
    bl = [json.loads(l) for l in open(os.path.join(D, bl_files[key]))]
    bl = [r['online_ms'] for r in bl if r.get('kind') == 'bl_online_target' and r.get('prefix_level') == 0]
    ratios[key] = (len(ic), len(bl), sum(ic) / len(ic) / (sum(bl) / len(bl)))

claims = []
fl = [os.path.join(STUDY, 'data/census_n61_K600.jsonl')]
def cl(claimed, computed, definition, files, status):
    claims.append({'claimed': claimed, 'computed': computed, 'definition': definition, 'files': files, 'status': status})

cl('49 independent free relations at K=600 (rank mod r)', str(rank) if rank is not None else 'undefined',
   f'rank over GF(r) (r={r_mod}, prime={prime}) of the {len(rows)} verified non-trivial relation rows; row = support pairs [index, coefficient] summed mod r; columns touched in file = {len(cols)} (summary says {summ["columns_touched"]})',
   fl, 'match' if rank == 49 else 'mismatch')
cl('147 verified non-trivial collisions at K=600', str(len(rels)),
   'count of kind=free_relation lines in census_n61_K600.jsonl (summary line verified_nontrivial=%d)' % summ['verified_nontrivial'],
   fl, 'match' if len(rels) == 147 else 'mismatch')
cl('8.2 % of the 599 needed', f'{100 * 49 / 599:.2f}',
   '49 / 599 * 100 with 599 as stated; 599 appears only in README.md:18 and no definition was found in the study',
   [os.path.join(STUDY, 'README.md')] + fl, 'partial')
cl('x1.09 improvement over collecting every relation', 'cannot_define',
   'no formula for the x1.09 factor found in the study files read (see notes)', [os.path.join(STUDY, 'README.md')], 'cannot_define')
Kfull = (12 * r_mod / 61 ** 3) ** (1 / 3)
cl('full rank would need K approx 2,050', f'{Kfull:.1f}',
   'K = (12 r / n^3)^(1/3) at n=61, the formula at README.md:153-154 (r from the census summary line)',
   [os.path.join(STUDY, 'README.md')], 'match' if round(Kfull, -1) == 2050 or abs(Kfull - 2050) / 2050 <= 0.01 else 'mismatch')
for key, claimed in [('41', 317), ('53', 208), ('61a', 221), ('61b', 53)]:
    n_ic, n_bl, rr = ratios[key]
    name = {'41': 'n=41 K=100 vs BL 2.73M-DP table', '53': 'n=53 K=200 vs BL 1.25M-DP table',
            '61a': 'n=61 K=256 vs BL 357k-DP table', '61b': 'n=61 K=360 vs BL 357k-DP table'}[key]
    cl(f'{claimed}x ratio of means IC/BL online ({name})', f'{rr:.1f}',
       f'mean(IC online_ms over {n_ic} targets) / mean(BL online_ms prefix_level=0 over {n_bl} targets)',
       [os.path.join(STUDY, 'data', ic_files[key]), os.path.join(STUDY, 'data', bl_files[key])],
       'match' if abs(rr - claimed) / claimed <= 0.01 or round(rr) == claimed else 'mismatch')

def grep_lines(rel, needles):
    out = []
    with open(os.path.join(REPO, STUDY, rel)) as fh:
        for i, line in enumerate(fh, 1):
            if any(n in line for n in needles):
                out.append(f'{STUDY}/{rel}:{i}: {line.rstrip()}')
    return out

readme = os.path.join(STUDY, 'README.md')
lines = open(os.path.join(REPO, readme)).read().split('\n')
s = next(i for i, l in enumerate(lines) if 'Null (identical map required)' in l)
e = next(i for i, l in enumerate(lines) if 'control was built to show' in l)
aa = [f'{readme}:{i+1}: {lines[i]}' for i in range(s, e + 1)]
iso = grep_lines('README.md', ['nice', 'prlimit', 'load average', 'isolat', 'shared']) + grep_lines('PROTOCOL.md', ['nice', 'prlimit', 'load', 'isolat', 'shared'])

def git_date(path):
    out = subprocess.run(['git', '-C', REPO, 'log', '--diff-filter=A', '--format=%H %cI', '--', path], capture_output=True, text=True).stdout.strip()
    return out if out else 'EMPTY'

off = []
with open(os.path.join(D, 'census_binary.sha256')) as fh:
    for i, line in enumerate(fh, 1):
        if '/tmp' in line or 'scratch' in line:
            off.append(f'{STUDY}/data/census_binary.sha256:{i}: {line.rstrip()}')

out = {'row_no': ROW, 'slug': '20261014-ic-next10-09-equal-advice', 'claims': claims,
       'identical': {'claimed': False, 'n_compared': None, 'n_equal': None, 'n_differ': None, 'files': [], 'status': 'no_claim'},
       'offrepo_refs': off, 'aa_lines': aa, 'isolation_lines': iso,
       'protocol_git_date': git_date(os.path.join(STUDY, 'PROTOCOL.md')),
       'first_raw_git_date': git_date(os.path.join(STUDY, 'data/bl_n61_64th.jsonl')),
       'script': SELF,
       'notes': (f'census file: {len(rels)} free_relation lines plus one free_relation_census summary line; rank computed by Gaussian elimination mod r (r prime={prime}); '
                 f'columns touched in file = {len(cols)}, summary columns_touched = {summ["columns_touched"]}. '
                 'Off-repo: one line in data/census_binary.sha256 (the only /tmp or scratch token in the study). '
                 'No store: references in study files. BL online rows use prefix_level 0 (= full table, entries 2730602/1250307/356938). '
                 'The 599 denominator has no definition in the study files read (README.md:18 only); the x1.09 figure has no computation in the files read; K approx 2,050 is computed from the README formula. Bootstrap CIs not recomputed (10,000-resample bootstrap, seed not in repo).')}
print(json.dumps(out, ensure_ascii=False))
