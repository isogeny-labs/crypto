import json, pathlib, re, subprocess, csv
from collections import Counter
REPO = pathlib.Path('/home/user/crypto')
WB = REPO / 'research/validation_sweep_20261019/B-benchmarks'
OUT = WB / 'recompute/SWB-RECOMPUTE-11.jsonl'
ROW = 87
SLUG = '20261014-ic-program-paper'
PAPER = REPO / 'papers/ecdlp-index-calculus-program'
DATA = PAPER / 'figures/data.json'
LADDER = PAPER / 'figures/ladder.tsv'
ABS = PAPER / 'abstract.txt'
SCRIPT = WB / 'scripts/recompute/87.py'
PROTOCOL = PAPER / 'README.md'
FIRST_RAW = PAPER / 'abstract.txt'
d = json.loads(DATA.read_text())
lad = list(csv.DictReader(open(LADDER), delimiter='\t'))
claims = []
def ok(cl, s, c):
    dd = len(s.split('.')[1]) if '.' in s else 0
    return round(c, dd) == cl or abs(c - cl) / abs(cl) <= 0.01
def claim(text, computed, definition, files, pairs, status=None):
    if status is None:
        if not pairs or any(c is None for _, _, c in pairs):
            status = 'cannot_define'
        else:
            oks = [ok(cl, s, c) for s, cl, c in pairs]
            status = 'partial' if all(oks) else ('mismatch' if not any(oks) else 'partial')
    claims.append({'claimed': text, 'computed': computed, 'definition': definition,
                   'files': [str(f.relative_to(REPO)) for f in files], 'status': status})

ro = d['rootfinding_oracle']['rows']
for r in ro: print('ROW rootfind', r)
eng = d['engine']['rows']
for r in eng: print('ROW engine', r)
cg = d['cold_gap']['rows']
for r in cg: print('ROW coldgap', r)
for r in d['koblitz_online']['rows']: print('ROW online', r)
print('ROW gaudry', {k: d['gaudry_quartic'][k] for k in ('C4_S4', 'C4_D4')})
print('ROW pkm', d['pkm'])
print('ROW k13', d['k13_rootfind_progression']['rows'], d['k13_rootfind_progression']['references'])
for r in lad: print('ROW ladder', r['family'], '|', r['change'], '|', r['level'], r['ratio'], r['verdict'])

# 1. root finding binary vs matrix F4 (per decomposition)
m4 = [r['ratio'] for r in ro if 'matrix-F4' in r['label']]
claim('root finding 383-1,535x (binary vs matrix F4) per decomposition', 'matrix-F4 rows in rootfinding_oracle: %s' % m4,
      'figures/data.json rootfinding_oracle rows with label containing matrix-F4; min and max', [DATA], [('383', 383, min(m4) if m4 else None), ('1535', 1535, max(m4) if m4 else None)])
bu = [r['ratio'] for r in ro if 'Buchberger' in r['label']]
claim('3,021-5,802x (NIST vs Buchberger) per decomposition', 'Buchberger rows: %s' % bu,
      'figures/data.json rootfinding_oracle rows with label containing Buchberger; min and max', [DATA], [('3021', 3021, min(bu) if bu else None), ('5802', 5802, max(bu) if bu else None)])
ms = [r['ratio'] for r in ro if 'msolve' in r['label']]
claim('50-105x end to end vs msolve', 'msolve rows: %s' % ms,
      'figures/data.json rootfinding_oracle rows with label containing msolve; min and max', [DATA], [('50', 50, min(ms) if ms else None), ('105', 105, max(ms) if ms else None)])

# 4. best complete Koblitz n = 13 IC S: 38.82 -> 22.97 (last complete row of the k13 progression)
k13 = d['k13_rootfind_progression']['rows']
first_S = k13[0]['S']; best_complete = min(r['S'] for r in k13 if r['complete'])
claim('best complete Koblitz n=13 IC S 38.82 -> 22.97', 'first row S = %s; minimum complete-row S = %s; rows: %s' % (first_S, best_complete, [(r['step'], r['S'], r['complete']) for r in k13]),
      'figures/data.json k13_rootfind_progression rows', [DATA], [('38.82', 38.82, first_S), ('22.97', 22.97, best_complete)])
rho_strong = [x['S'] for x in d['k13_rootfind_progression']['references'] if 'strong' in x['label']][0]
claim('1.55x strong rho (n=13)', 'best complete S / strong rho S = %.4f (strong rho S = %s)' % (best_complete / rho_strong, rho_strong),
      'best complete k13 S divided by the strong signed-Frobenius rho S in references', [DATA], [('1.55', 1.55, best_complete / rho_strong)])

# 5. D4 cuts C4 47.5-48.4x: one recorded ratio C4_S4/C4_D4 from gaudry_quartic
g = d['gaudry_quartic']
q = g['C4_S4'] / g['C4_D4']
claim('D4 cuts C4 47.5-48.4x (k = 4)', 'C4_S4 / C4_D4 = %.4f (single recorded pair; no interval recorded)' % q,
      'figures/data.json gaudry_quartic C4_S4 divided by C4_D4', [DATA], [('47.5', 47.5, q), ('48.4', 48.4, q)], status='partial')

# 6. admitted n = 61 online 13.94x [9.81, 19.13] vs rho without a table
on61 = [r['ratio'] for r in d['koblitz_online']['rows'] if r['n'] == 61]
adm = [r for r in d['koblitz_online']['rows'] if r['n'] == 61 and str(r.get('status', '')).startswith('admitted')]
adm = adm[0] if adm else None
claim('admitted n=61 online 13.94x [9.81, 19.13] vs rho without a table', 'koblitz_online n=61 ratio values: %s; cold_ratio_n61_ic_over_rho = %s' % (on61, d['koblitz_online'].get('cold_ratio_n61_ic_over_rho')),
      'figures/data.json koblitz_online rows with n == 61 (rho online wall / IC online wall)', [DATA],
      [('13.94', 13.94, adm['ratio'] if adm else None), ('9.81', 9.81, adm['lo'] if adm else None), ('19.13', 19.13, adm['hi'] if adm else None)], status='partial')

# 7. genus-3 negation-map rho ratio 1.15x at q = 2^20 (ladder), and round 3 0.62x (q = 2^22)
g3 = [r for r in lad if r['family'].startswith('genus-3')]
q20 = [r for r in g3 if '2^20' in r['change']]
q22 = [r for r in g3 if '2^22' in r['change']]
claim('genus-3 round-3 further 0.62x (q = 2^22 row)', 'ladder.tsv genus-3 rows at q=2^22: %s' % [(r['change'], r['ratio']) for r in q22],
      'figures/ladder.tsv ratio column for genus-3 rows with 2^22 in change', [LADDER], [('0.62', 0.62, float(q22[0]['ratio']) if q22 else None)])
claim('genus-3 1.15x a measured negation-map rho at q = 2^20 after round 2', 'ladder.tsv genus-3 rows at q=2^20: %s; no ladder row carries 1.15' % [(r['change'], r['ratio']) for r in q20],
      'figures/ladder.tsv rows with 2^20 in change', [LADDER], [('1.15', 1.15, None)])

# 8. every cold IC arm above matched rho: count cold_gap rows with ratio <= 1
below = [r['label'] for r in cg if r['ratio'] <= 1.0]
claim('every cold IC arm above matched rho', 'cold_gap rows = %d; rows with candidate S / rho S <= 1: %d %s' % (len(cg), len(below), below),
      'figures/data.json cold_gap rows; ratio is candidate S over matched rho S (>1 means rho cheaper)', [DATA], [('0', 0, len(below))])

# 9. genus-3 cubic GHS q^1.380 vs rho q^1.5: exponents not recorded in repo files read here
claim('genus-3 cubic GHS q^1.380 vs rho q^1.5', 'exponents 1.380 and 1.5 are not recorded in figures/data.json or ladder.tsv', 'search of the paper figure data for the exponent values', [DATA, LADDER], [], status='no_raw_in_repo')

# 10. the 4,848-execution ladder: not a headline number in this list; omitted from claims
identical = {'claimed': False, 'n_compared': None, 'n_equal': None, 'n_differ': None, 'files': [], 'status': 'not_claimed'}

OFF = re.compile(r'drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:[A-Za-z0-9_.\-]+|Modal volume|not committed|scratch', re.I)
AA = re.compile(r'A/A|A-A|null[- ]control|null control', re.I)
ISO = re.compile(r'isolat|\bL[0-3]\b|NUMA', re.I)
store_index = REPO / 'store/index.json'
store_txt = store_index.read_text() if store_index.exists() else None
offrepo, aa_l, iso = [], [], []
files = [ABS] + sorted(p for p in PAPER.rglob('*') if p.is_file() and p.suffix in ('.md', '.tex', '.txt', '.tsv', '.json', '.bib') and p != ABS and p.stat().st_size < 1_000_000)
for p in files:
    rel = str(p.relative_to(REPO))
    for n, line in enumerate(p.read_text(errors='replace').splitlines(), 1):
        if OFF.search(line):
            extra = ''
            for name in re.findall(r'store:[A-Za-z0-9_.\-]+', line):
                extra = ' [%s listed in store/index.json: %s]' % (name, 'yes' if (store_txt and name.split(':', 1)[1] in store_txt) else ('no' if store_txt else 'no store/index.json'))
            offrepo.append('%s:%d: %s%s' % (rel, n, line.strip(), extra))
        if AA.search(line):
            aa_l.append('%s:%d: %s' % (rel, n, line.strip()))
        if ISO.search(line):
            iso.append('%s:%d: %s' % (rel, n, line.strip()))

def gitdate(path):
    out = subprocess.run(['git', '-C', str(REPO), 'log', '--diff-filter=A', '--format=%H %cI', '--', str(path.relative_to(REPO))],
                         capture_output=True, text=True).stdout.strip().splitlines()
    return out[-1] if out else ''
pd = gitdate(PROTOCOL); fd = gitdate(FIRST_RAW)
notes = ('This row has no measurement; the numbers are read from figures/data.json (recorded copies, each naming its source artefact) and figures/ladder.tsv. Those copies are not recomputed from the artefacts, so matching statuses are partial. '
         'The 1.15x genus-3 figure is not in the recorded data. Protocol is the paper README; the first raw file is abstract.txt. Protocol git add date: %s. First raw git add date: %s.') % (pd or 'EMPTY', fd or 'EMPTY')
row = {'row_no': ROW, 'slug': SLUG, 'claims': claims, 'identical': identical, 'offrepo_refs': offrepo,
       'aa_lines': aa_l, 'isolation_lines': iso, 'protocol_git_date': pd, 'first_raw_git_date': fd,
       'script': str(SCRIPT.relative_to(REPO)), 'notes': notes}
lines = [l for l in OUT.read_text().splitlines() if l.strip()] if OUT.exists() else []
lines = [l for l in lines if json.loads(l)['row_no'] != ROW] + [json.dumps(row)]
OUT.write_text('\n'.join(lines) + '\n')
for c in claims: print(c['status'], '|', c['claimed'], '|', c['computed'][:170])
print('STATUS', Counter(c['status'] for c in claims), 'offrepo', len(offrepo), 'aa', len(aa_l), 'iso', len(iso))
