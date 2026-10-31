import json, statistics, pathlib, re, subprocess, math
from collections import Counter
REPO = pathlib.Path('/home/user/crypto')
WB = REPO / 'research/validation_sweep_20261019/B-benchmarks'
OUT = WB / 'recompute/SWB-RECOMPUTE-11.jsonl'
ROW = 88
SLUG = '20261015-ic-m4-wide-chain-vs-mitm'
STUDY = REPO / 'research/ic_m3_algebraic_vs_mitm_20261014'
M4 = STUDY / 'runs/m4'
W4 = STUDY / 'runs/m4-wide'
SCRIPT = WB / 'scripts/recompute/88.py'
PROTOCOL = STUDY / 'AMENDMENT_1.md'
FIRST_RAW = STUDY / 'runs/m3/truth-m3-n13-d5.json'
NS = [11, 13, 17, 19, 23, 29, 31]
D = {11: 3, 13: 4, 17: 5, 19: 5, 23: 6, 29: 8, 31: 8}
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

def ols(xs, ys):
    mx, my = statistics.fmean(xs), statistics.fmean(ys)
    sxx = sum((x - mx) ** 2 for x in xs)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sxx

floor = {}; wide = {}; pvals = {}
for n in NS:
    t = json.loads((M4 / ('truth-m4-n%d-d%d.json' % (n, D[n]))).read_text())
    p = t['decomposable'] / t['pool']; pvals[n] = p
    floor[n] = statistics.fmean(r['ns'] for r in t['rows']) / p   # (p c_yes + (1-p) c_no) / p over the pool
    recs = [json.loads(l) for l in (W4 / ('arm-wide-chain-m4-n%d-d%d.jsonl' % (n, D[n]))).read_text().splitlines() if l.strip()]
    fin = [r for r in recs if not r['censored']]
    cy = statistics.fmean(r['wall_ns'] for r in recs if r['truth_yes']); cn = statistics.fmean(r['wall_ns'] for r in recs if not r['truth_yes'])
    wide[n] = dict(all=(p * cy + (1 - p) * cn) / p, cens=sum(1 for r in recs if r['censored']), n=len(recs))

L2 = lambda v: math.log2(v)
meas = [n for n in NS if n <= 23]
wide_slope = ols(meas, [L2(wide[n]['all']) for n in meas])
floor_slope = ols(meas, [L2(floor[n]) for n in meas])
all_slope = ols(NS, [L2(wide[n]['all']) for n in NS])
claim('wide-chain slope 0.805 [0.717, 0.872] log2 ns/relation per unit n (n = 11-23)', 'OLS slope of log2 C_wide on n over n=11,13,17,19,23 = %.4f (C = mean wall_ns / p)' % wide_slope,
      'OLS of log2(mean wall_ns over the 12 targets / p) on n over measured cells; bootstrap interval not recomputed (2,000 replicates, seed not stated)',
      [W4], [('0.805', 0.805, wide_slope)], interval_undefined=True)
claim('floor slope 0.333 on the same cells', 'OLS slope of log2 C_floor on n over the same cells = %.4f without the table-build term (not determinable from text read)' % floor_slope,
      'OLS of log2(mean ns over pool / p) on n; README floor includes a table-build term whose spread rule is not fully stated', [M4], [('0.333', 0.333, floor_slope)], status='cannot_define')
claim('Delta [0.433, 0.504]: wide-chain minus floor slope', 'slope difference = %.4f (floor part without table-build term; interval not recomputed)' % (wide_slope - floor_slope),
      'wide slope minus floor slope on the same measured cells', [W4, M4], [], status='cannot_define')
r11 = wide[11]['all'] / floor[11]; r23 = wide[23]['all'] / floor[23]
claim('wide-chain / floor 5.5e2 at n = 11', 'ratio of C_wide to C_floor at n=11 = %.1f (floor without table-build term)' % r11, 'C_wide / C_floor, n=11',
      [W4, M4], [], status='cannot_define')
claim('wide-chain / floor 3.1e4 at n = 23', 'ratio of C_wide to C_floor at n=23 = %.1f (floor without table-build term)' % r23, 'C_wide / C_floor, n=23',
      [W4, M4], [], status='cannot_define')
cens_txt = ', '.join('n=%d censored %d of %d' % (n, wide[n]['cens'], wide[n]['n']) for n in (29, 31))
claim('n = 29 and n = 31 wide-chain / floor >= 1.2e5 and >= 1.3e5 (censored lower bounds)',
      'censoring: %s; the censored-value rule for the bound is not stated in README or AMENDMENT_1, so the bound is not recomputed' % cens_txt,
      'lower bound from censored targets; the study text does not state the value used for a censored target', [W4, M4], [], status='cannot_define')
claim('lower-bound fit slope 0.724 (all seven cells)', 'OLS slope of log2 C_wide on n over all seven cells = %.4f' % all_slope,
      'OLS of log2 C_wide on n, n=11..31 (censored cells included as recorded)', [W4], [('0.724', 0.724, all_slope)], interval_undefined=True)

# exactness: finished verdicts vs the floor's truth
allrec = []
for n in NS:
    allrec += [json.loads(l) for l in (W4 / ('arm-wide-chain-m4-n%d-d%d.jsonl' % (n, D[n]))).read_text().splitlines() if l.strip()]
fin = [r for r in allrec if not r['censored']]
yes = sum(1 for r in fin if r['verdict'] == 'yes'); no = sum(1 for r in fin if r['verdict'] == 'no')
agree = sum(1 for r in fin if r['agrees'] is True)
claim('70/70 finished verdicts exact', 'finished verdicts = %d (yes %d, no %d); agreeing with truth = %d; records total %d' % (len(fin), yes, no, agree, len(allrec)),
      'count of non-censored wide-chain records and agrees == true', [W4], [('70', 70, len(fin)), ('70a', 70, agree)])

identical = {'claimed': True, 'n_compared': len(fin), 'n_equal': agree, 'n_differ': len(fin) - agree, 'files': [str(W4.relative_to(REPO))],
             'status': 'match' if (len(fin) == 70 and agree == 70) else 'mismatch'}

OFF = re.compile(r'drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:[A-Za-z0-9_.\-]+|Modal volume|not committed|scratch', re.I)
AA = re.compile(r'A/A|A-A|null[- ]control|null control', re.I)
ISO = re.compile(r'isolat|\bL[0-3]\b|NUMA|shared|reservation', re.I)
store_index = REPO / 'store/index.json'
store_txt = store_index.read_text() if store_index.exists() else None
offrepo, aa_l, iso = [], [], []
files = sorted(p for p in STUDY.rglob('*') if p.is_file() and p.suffix in ('.md', '.typ', '.txt', '.tex', '.log') and 'raw' not in p.relative_to(STUDY).parts and 'figures' not in p.relative_to(STUDY).parts and p.stat().st_size < 1_000_000)
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
notes = ('The card lists runs/m3/truth-m3-*.json as raw candidates, but the wide-chain (m = 4) records are in runs/m4-wide and the floor truth is in runs/m4 (truth-m4-*.json); those were used. '
         'The study text states C = (p c_yes + (1-p) c_no)/p; this equals mean(ns)/p, applied here. Floor and wide-chain are both divided by the same p (decomposable/pool). '
         'Bootstrap intervals and the censored lower bounds are not recomputed. Protocol (AMENDMENT_1.md) git add date: %s. First raw git add date (card candidate): %s.') % (pd or 'EMPTY', fd or 'EMPTY')
row = {'row_no': ROW, 'slug': SLUG, 'claims': claims, 'identical': identical, 'offrepo_refs': offrepo,
       'aa_lines': aa_l, 'isolation_lines': iso, 'protocol_git_date': pd, 'first_raw_git_date': fd,
       'script': str(SCRIPT.relative_to(REPO)), 'notes': notes}
lines = [l for l in OUT.read_text().splitlines() if l.strip()] if OUT.exists() else []
lines = [l for l in lines if json.loads(l)['row_no'] != ROW] + [json.dumps(row)]
OUT.write_text('\n'.join(lines) + '\n')
for c in claims: print(c['status'], '|', c['claimed'][:60], '|', c['computed'][:170])
print('floor log2', {n: round(L2(floor[n]), 3) for n in NS}, 'wide log2', {n: round(L2(wide[n]['all']), 3) for n in NS}, 'p', {n: round(pvals[n], 4) for n in NS})
print('STATUS', Counter(c['status'] for c in claims), 'offrepo', len(offrepo), 'aa', len(aa_l), 'iso', len(iso))
