import json, statistics, pathlib, re, subprocess
from collections import Counter
REPO = pathlib.Path('/home/user/crypto')
WB = REPO / 'research/validation_sweep_20261019/B-benchmarks'
OUT = WB / 'recompute/SWB-RECOMPUTE-11.jsonl'
ROW = 85
SLUG = '20261014-fb-hyperparam-sweep'
STUDY = REPO / 'research/fb_hyperparam_sweep_20261014'
SWEEP = STUDY / 'data/sweep.json'
SCRIPT = WB / 'scripts/recompute/85.py'
PROTOCOL = STUDY / 'PROTOCOL.md'
FIRST_RAW = STUDY / 'data/console.log'

d = json.loads(SWEEP.read_text())
cells = d['binary'] + [x for x in d.get('prime', []) if False]
binary = d['binary']
an = d['analysis']
claims = []
def ok(cl, s, c):
    dd = len(s.split('.')[1]) if '.' in s else 0
    return round(c, dd) == cl or abs(c - cl) / abs(cl) <= 0.01
def claim(text, computed, definition, files, pairs, status=None, interval_undefined=False):
    if status is None:
        if not pairs or any(c is None for _, _, c in pairs):
            status = 'cannot_define'
        else:
            oks = [ok(cl, s, c) for s, cl, c in pairs]
            status = ('partial' if interval_undefined else 'match') if all(oks) else ('mismatch' if not any(oks) else 'partial')
    claims.append({'claimed': text, 'computed': computed, 'definition': definition,
                   'files': [str(f.relative_to(REPO)) for f in files], 'status': status})

n_cells = len(binary); n_ok = sum(1 for c in binary if c)
ctrl_ok = an['cells_controls_ok']
claim('208 cells pass every control', '%d cells in sweep.json binary list; analysis.cells_controls_ok = %d' % (n_cells, ctrl_ok),
      'count of binary-arm cells; cells_controls_ok from analysis block', [SWEEP], [('208', 208, n_cells)])

planted = sum(c['positive_control']['planted'] for c in binary)
found_rf = sum(c['positive_control']['found_root_find'] for c in binary)
claim('13,312/13,312 planted decompositions found by root finding', 'found_root_find %d of planted %d across %d cells' % (found_rf, planted, n_cells),
      'sum over cells of positive_control.found_root_find and positive_control.planted', [SWEEP], [('13312', 13312, found_rf), ('13312p', 13312, planted)])

tg = sum(c['crosscheck']['targets'] for c in binary); dis = sum(c['crosscheck']['disagreements'] for c in binary)
claim('53,248/53,248 oracle agreements', '%d agreements of %d targets (disagreements %d)' % (tg - dis, tg, dis),
      'sum over cells of crosscheck.targets minus crosscheck.disagreements', [SWEEP], [('53248', 53248, tg - dis)])

cal_ref = sum(c['degrees']['calibration_decomp_refuted'] for c in binary); cal_t = sum(c['degrees']['calibration_decomp_targets'] for c in binary)
claim('0/832 calibration refutations', '%d refuted of %d decomposable calibration targets' % (cal_ref, cal_t),
      'sum over cells of degrees.calibration_decomp_refuted and degrees.calibration_decomp_targets', [SWEEP], [('0', 0, cal_ref), ('832', 832, cal_t)])

cens = sum(c['degrees']['dstar_censored'] for c in binary)
claim('0 censored D* (none censored)', '%d censored D* values across %d cells' % (cens, n_cells),
      'sum over cells of degrees.dstar_censored', [SWEEP], [('0', 0, cens)])

# H1: measured/heuristic yield ratio, cells excluding subfield family; median over those cells
rat = [c['yield']['rate'] / c['yield']['heuristic_V2_over_2n'] for c in binary if c['family'] != 'subfield' and c['yield']['heuristic_V2_over_2n']]
med = statistics.median(rat)
claim('yield approx 0.5x the |V|^2/2^n heuristic (median)', 'median measured/heuristic over %d non-subfield cells = %.4f' % (len(rat), med),
      'median over binary cells with family != subfield of yield.rate / yield.heuristic_V2_over_2n', [SWEEP], [('0.5', 0.5, med)])

# d_ff moves <= 1 across n' (analysis h4)
mx = max(x['first_fall_mean_range'] for x in an['h4_dff_vs_dstar_spread'])
claim('d_ff moves <= 1 across n′ at fixed class, family, n', 'max first_fall_mean_range over %d groups = %s' % (len(an['h4_dff_vs_dstar_spread']), mx),
      'max of analysis.h4_dff_vs_dstar_spread[].first_fall_mean_range', [SWEEP], [('1', 1.0, mx)], status=None)

# H2: random vs coordinate matched pairs (90 of the 118 entries are random vs coordinate)
h2 = [x for x in an['h2_family_vs_coordinate'] if x['a']['family'] == 'random' and x['b']['family'] == 'coordinate']
raise_ = sum(1 for x in h2 if x['dstar_different'] and x['dstar_diff'] > 0)
lower_ = sum(1 for x in h2 if x['dstar_different'] and x['dstar_diff'] < 0)
claim('random basis raises D* vs coordinate in 31 of 90 matched pairs', 'random-vs-coordinate pairs = %d; D* significantly higher = %d; significantly lower = %d' % (len(h2), raise_, lower_),
      'analysis.h2_family_vs_coordinate entries with a.family random, b.family coordinate; dstar_different and dstar_diff sign', [SWEEP], [('31', 31, raise_), ('90', 90, len(h2))])

h3 = an['h3_koblitz_vs_generic']
h3d = sum(1 for x in h3 if x['dstar_different'])
claim('Koblitz vs generic D* differs in 0 of 104 pairs', 'pairs = %d; dstar_different true = %d' % (len(h3), h3d),
      'analysis.h3_koblitz_vs_generic entries, dstar_different', [SWEEP], [('0', 0, h3d), ('104', 104, len(h3))])

pr = d['prime']
no_solve = sum(1 for p in pr if p.get('solved') is False)
claim('9 of 90 prime IC runs returned no solve', 'prime runs = %d; solved false = %d' % (len(pr), no_solve),
      'count of prime records with solved == false (the prime list holds rho records)', [SWEEP], [('9', 9, no_solve)])

# identical / agreement (root finding vs brute force on V x V)
identical = {'claimed': True, 'n_compared': tg, 'n_equal': tg - dis, 'n_differ': dis, 'files': [str(SWEEP.relative_to(REPO))],
             'status': 'match' if (tg == 53248 and dis == 0) else 'mismatch'}

OFF = re.compile(r'drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:[A-Za-z0-9_.\-]+|Modal volume|not committed|scratch', re.I)
AA = re.compile(r'A/A|A-A|null[- ]control|null control|positive control', re.I)
ISO = re.compile(r'isolat|\bL[0-3]\b|NUMA|shared', re.I)
store_index = REPO / 'store/index.json'
store_txt = store_index.read_text() if store_index.exists() else None
offrepo, aa_l, iso = [], [], []
files = sorted(p for p in STUDY.rglob('*') if p.is_file() and p.suffix in ('.md', '.typ', '.txt', '.tex', '.py', '.log') and 'figures' not in p.relative_to(STUDY).parts and p.stat().st_size < 1_000_000)
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
notes = ('Aggregation of H1 ratio: median over non-subfield cells (study states medians per n′ of the same ratio). "Prime" list holds rho runs; the 9 no-solve count is from the prime records. '
         'Protocol git add date: %s. First raw git add date: %s.') % (pd or 'EMPTY', fd or 'EMPTY')
row = {'row_no': ROW, 'slug': SLUG, 'claims': claims, 'identical': identical, 'offrepo_refs': offrepo,
       'aa_lines': aa_l, 'isolation_lines': iso, 'protocol_git_date': pd, 'first_raw_git_date': fd,
       'script': str(SCRIPT.relative_to(REPO)), 'notes': notes}
lines = [l for l in OUT.read_text().splitlines() if l.strip()] if OUT.exists() else []
lines = [l for l in lines if json.loads(l)['row_no'] != ROW] + [json.dumps(row)]
OUT.write_text('\n'.join(lines) + '\n')
for c in claims: print(c['status'], '|', c['claimed'], '|', c['computed'][:170])
print('STATUS', Counter(c['status'] for c in claims), 'offrepo', len(offrepo), 'aa', len(aa_l), 'iso', len(iso))
