import json, statistics, pathlib, re, subprocess, sys
REPO = pathlib.Path('/home/user/crypto')
WB = REPO / 'research/validation_sweep_20261019/B-benchmarks'
OUT = WB / 'recompute/SWB-RECOMPUTE-11.jsonl'
ROW = 81
SLUG = '20261015-ic-linear-algebra-cado'
STUDY = REPO / 'research/la_engines_20261014'
RAW = STUDY / 'raw'
SCRIPT = WB / 'scripts/recompute/81.py'
PROTOCOL = STUDY / 'PROTOCOL.md'
FIRST_RAW = RAW / 'cado_logs/cado-c131072-w3-s1-t2x2-m4n4.bwc.log'

def load(p):
    p = pathlib.Path(p)
    if not p.exists():
        return None
    return [json.loads(l) for l in p.read_text().splitlines() if l.strip()]

grid = load(RAW / 'grid.jsonl'); a4 = load(RAW / 'grid_amend4.jsonl'); a5 = load(RAW / 'grid_amend5.jsonl')
e2e = load(RAW / 'e2e.jsonl'); contended = load(RAW / 'contended_runs.jsonl'); reruns = load(RAW / 'rerun_contended.jsonl')
# same replacement rule as scripts/plots.py (applied to grid only, as plots.py does)
for c, rr in zip(contended, reruns):
    if rr.get('failed'):
        rr = dict(rr, cols=c['cols'], seed=c['seed'], weight=c['weight'])
    assert (c['pass'], c['arm'], c['cols'], c['seed']) == (rr['pass'], rr['arm'], rr['cols'], rr['seed'])
    for i, r in enumerate(grid):
        if r.get('utc') == c['utc'] and r.get('arm') == c['arm'] and r.get('cols') == c['cols'] and r.get('seed') == c['seed'] and r.get('pass') == c['pass']:
            grid[i] = dict(rr, weight=rr.get('weight', c['weight']), replaced_utc=c['utc'])
seen = set()
for r in grid:
    if r.get('pass') == 'main' and r.get('arm') == 'core-bw' and r.get('seed') == 1:
        if r['cols'] in seen:
            r['arm'] = 'core-bw-aa'
        seen.add(r['cols'])

def per_seed(rows, arm, pass_=None):
    d = {}
    for r in rows:
        if r.get('arm') != arm or r.get('failed') or r.get('verified') is not True:
            continue
        if r.get('weight', 3) != 3 or (pass_ and r.get('pass') != pass_):
            continue
        d[(r['cols'], r['seed'])] = r['solve_s']
    return d

def med_ratio(num, den, D):
    xs = [num[k] / den[k] for k in num if k[0] == D and k in den]
    return (statistics.median(xs), min(xs), max(xs), len(xs)) if xs else None

def med_arm(d, D):
    xs = [v for k, v in d.items() if k[0] == D]
    return (statistics.median(xs), len(xs)) if xs else None

def ndp(s):
    return len(s.split('.')[1]) if '.' in s else 0

def ok(cl, s, c):
    return round(c, ndp(s)) == cl or abs(c - cl) / abs(cl) <= 0.01

def status(pairs):
    if not pairs or any(c is None for _, _, c in pairs):
        return 'cannot_define'
    oks = [ok(cl, s, c) for s, cl, c in pairs]
    return 'match' if all(oks) else ('partial' if any(oks) else 'mismatch')

claims = []
def claim(claimed, computed_txt, definition, files, pairs):
    claims.append({'claimed': claimed, 'computed': computed_txt, 'definition': definition,
                   'files': [str(f.relative_to(REPO)) for f in files], 'status': status(pairs)})

GRID_F = RAW / 'grid.jsonl'; A5_F = RAW / 'grid_amend5.jsonl'; E2E_F = RAW / 'e2e.jsonl'
README = STUDY / 'README.md'; REPORT = STUDY / 'REPORT.typ'

# 1. 1.86x: cado-bwc / core-bw, same core, main pass, 4 threads, D = 131072 (per-seed ratio, median over seeds)
cado = per_seed(grid, 'cado-bwc', 'main'); core = per_seed(grid, 'core-bw', 'main')
m = med_ratio(cado, core, 131072)
claim('1.86x (cado bwc over ours, D=131,072)', 'median per-seed ratio %s; min %s max %s; n=%s' % (
      None if m is None else round(m[0], 4), None if m is None else round(m[1], 4), None if m is None else round(m[2], 4), None if m is None else m[3]),
      'per-seed cado-bwc solve_s / core-bw solve_s (grid.jsonl, pass main, after amendment-3 replacement), median over seeds (plots.py engine_ratio)',
      [GRID_F], [('1.86', 1.86, m[0] if m else None)])

# 2. 86.57 s -> 8.35 s (legacy-bw -> cado-bw, amendment 5, D = 262144): medians over seeds
leg5 = per_seed(a5, 'legacy-bw'); cb5 = per_seed(a5, 'cado-bw')
ml = med_arm(leg5, 262144); mc = med_arm(cb5, 262144)
claim('86.57 s -> 8.35 s (D=262,144, pipeline before -> after)',
      'legacy-bw median %s (n=%s); cado-bw median %s (n=%s)' % (
          None if ml is None else round(ml[0], 4), None if ml is None else ml[1],
          None if mc is None else round(mc[0], 4), None if mc is None else mc[1]),
      'median over seeds of per-seed solve_s, amendment-5 grid (plots.py before_after), +-1 coefficients',
      [A5_F], [('86.57', 86.57, ml[0] if ml else None), ('8.35', 8.35, mc[0] if mc else None)])

# 3. 10.40x [9.99, 11.06]: legacy-bw / cado-bw per-seed ratio, D = 262144
r10 = med_ratio(leg5, cb5, 262144)
claim('10.40x [9.99, 11.06] (D=262,144)',
      'median %s, min %s, max %s, n=%s' % tuple(None if r10 is None else (round(v, 4) if isinstance(v, float) else v) for v in r10) if r10 else 'undefined',
      'per-seed legacy-bw solve_s / cado-bw solve_s, amendment-5 grid, median and per-seed min and max',
      [A5_F], [('10.40', 10.40, r10[0] if r10 else None), ('9.99', 9.99, r10[1] if r10 else None), ('11.06', 11.06, r10[2] if r10 else None)])

# 4. 7.83 s vs 14.73 s: cado-bw-pos (ours, +1) vs cado-nfs, D = 262144 medians
cbp5 = per_seed(a5, 'cado-bw-pos'); cnfs5 = per_seed(a5, 'cado-nfs')
mp = med_arm(cbp5, 262144); mn = med_arm(cnfs5, 262144)
claim('7.83 s (ours) vs 14.73 s (CADO-NFS), D=262,144',
      'cado-bw-pos median %s (n=%s); cado-nfs median %s (n=%s)' % (
          None if mp is None else round(mp[0], 4), None if mp is None else mp[1],
          None if mn is None else round(mn[0], 4), None if mn is None else mn[1]),
      'median over seeds of per-seed solve_s, amendment-5 grid, +1 coefficients (arms cado-bw-pos, cado-nfs)',
      [A5_F], [('7.83', 7.83, mp[0] if mp else None), ('14.73', 14.73, mn[0] if mn else None)])

# 5. 1.93x [1.74, 1.96]: cado-nfs / cado-bw-pos per-seed ratio, D = 262144
r193 = med_ratio(cnfs5, cbp5, 262144)
claim('1.93x [1.74, 1.96] (D=262,144)',
      'median %s, min %s, max %s, n=%s' % (tuple(round(v, 4) if isinstance(v, float) else v for v in r193)) if r193 else 'undefined',
      'per-seed cado-nfs solve_s / cado-bw-pos solve_s, amendment-5 grid',
      [A5_F], [('1.93', 1.93, r193[0] if r193 else None), ('1.74', 1.74, r193[1] if r193 else None), ('1.96', 1.96, r193[2] if r193 else None)])

# 6. 0.40x at 1,024: legacy-bw / cado-bw per-seed ratio, D = 1024, amendment 5 (amendment 4 reported in notes)
r040 = med_ratio(leg5, cb5, 1024)
a4l = per_seed(a4, 'legacy-bw'); a4c = per_seed(a4, 'cado-bw')
r040_a4 = med_ratio(a4l, a4c, 1024)
claim('0.40x at D=1,024 (old filter over new filter)',
      'median %s, min %s, max %s, n=%s' % (tuple(round(v, 4) if isinstance(v, float) else v for v in r040)) if r040 else 'undefined',
      'per-seed legacy-bw solve_s / cado-bw solve_s at D=1024, amendment-5 grid',
      [A5_F], [('0.40', 0.40, r040[0] if r040 else None)])

# 7. 0.11-1.9 percent of real ic logs runs: linear_algebra_seconds / elapsed_seconds over e2e.jsonl
fr = [100.0 * r['linear_algebra_seconds'] / r['elapsed_seconds'] for r in e2e if r.get('elapsed_seconds')]
claim('0.11-1.9% of real ic logs runs',
      'min %s%%, max %s%%, n=%d' % (round(min(fr), 4), round(max(fr), 4), len(fr)) if fr else 'undefined',
      'linear_algebra_seconds / elapsed_seconds x 100 over all e2e.jsonl records',
      [E2E_F], [('0.11', 0.11, min(fr) if fr else None), ('1.9', 1.9, max(fr) if fr else None)])

# 8. 19.1 s IncrementalGauss at 8,192 columns: median over seeds of arm igauss, pass main
ig = [r for r in grid if r.get('arm') == 'igauss' and r.get('pass') == 'main' and r.get('cols') == 8192 and r.get('verified') is True and r.get('weight', 3) == 3]
igv = [r['solve_s'] for r in ig]
claim('19.1 s IncrementalGauss at 8,192 columns',
      'median %s over n=%d (values %s)' % (round(statistics.median(igv), 4), len(igv), [round(v, 3) for v in igv]) if igv else 'no igauss record at 8192 in grid.jsonl',
      'median solve_s of igauss records at cols=8192, pass main, verified',
      [GRID_F], [('19.1', 19.1, statistics.median(igv) if igv else None)])

# 9. 65,858-column core at D = 131,072: matrix dimension printed by the CADO bwc logs
dims = {}
for s in (1, 2, 3):
    f = RAW / ('cado_logs/cado-c131072-w3-s%d-t2x2-m4n4.bwc.log' % s)
    mm = re.search(r'dimension (\d+)x\1', f.read_text(errors='replace')) if f.exists() else None
    dims[s] = int(mm.group(1)) if mm else None
claim('65,858-column core at D=131,072 (CADO bwc input)',
      'dimension per log: %s' % dims,
      'first "dimension NxN" line in each cado-c131072 bwc log (seeds 1-3)',
      [RAW / ('cado_logs/cado-c131072-w3-s%d-t2x2-m4n4.bwc.log' % s) for s in (1, 2, 3)],
      [('65858', 65858, dims[k]) for k in (1, 2, 3)])

# Step 5: identical-output wording. MEASURED says "identical matrices" (an input), HEADLINE says nothing about equal outputs.
identical = {'claimed': False, 'n_compared': None, 'n_equal': None, 'n_differ': None, 'files': [], 'status': 'not_claimed'}

# Step 6 and 7: verbatim line extraction from study text files (.md .typ .txt .tex), raw/ excluded
OFF = re.compile(r'drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:[A-Za-z0-9_.\-]+|Modal volume|not committed|scratch', re.I)
AA = re.compile(r'A/A|A-A|null[- ]control|null control', re.I)
ISO = re.compile(r'isolat|\bL[0-3]\b|NUMA', re.I)
store_index = REPO / 'store/index.json'
store_txt = store_index.read_text() if store_index.exists() else None
offrepo, aa, iso = [], [], []
study_files = sorted(p for p in STUDY.rglob('*') if p.is_file() and p.suffix in ('.md', '.typ', '.txt', '.tex') and 'raw' not in p.relative_to(STUDY).parts)
for p in study_files:
    rel = str(p.relative_to(REPO))
    for n, line in enumerate(p.read_text(errors='replace').splitlines(), 1):
        if OFF.search(line):
            extra = ''
            for name in re.findall(r'store:[A-Za-z0-9_.\-]+', line):
                extra = ' [%s listed in store/index.json: %s]' % (name, 'yes' if (store_txt and name.split(':', 1)[1] in store_txt) else ('no' if store_txt else 'no store/index.json'))
            offrepo.append('%s:%d: %s%s' % (rel, n, line.strip(), extra))
        if AA.search(line):
            aa.append('%s:%d: %s' % (rel, n, line.strip()))
        if ISO.search(line):
            iso.append('%s:%d: %s' % (rel, n, line.strip()))

def gitdate(path):
    out = subprocess.run(['git', '-C', str(REPO), 'log', '--diff-filter=A', '--format=%H %cI', '--', str(path.relative_to(REPO))],
                         capture_output=True, text=True).stdout.strip().splitlines()
    return out[-1] if out else ''

pd = gitdate(PROTOCOL); fd = gitdate(FIRST_RAW)
notes = ('Claims are read from the claim text; the study uses medians over 3 seeds and per-seed ratios (scripts/plots.py), applied here. '
         'Row claim 65,858 is compared with the CADO log dimension 65202. '
         'Amendment-4 0.40x at 1024 (computed from grid_amend4.jsonl): %s. '
         'Off-repo and isolation lines are whole lines matched in study .md/.typ/.txt/.tex files, not split sentences. '
         'A/A lines found in study text files: %s. '
         'Protocol git add date: %s. First raw git add date: %s. '
         'Study "identical matrices" wording refers to input matrices in MEASURED; no output-equality field claimed.'
         ) % (None if r040_a4 is None else [round(v, 4) if isinstance(v, float) else v for v in r040_a4],
              'yes' if aa else 'no', pd or 'EMPTY', fd or 'EMPTY')
row = {'row_no': ROW, 'slug': SLUG, 'claims': claims, 'identical': identical, 'offrepo_refs': offrepo,
       'aa_lines': aa, 'isolation_lines': iso, 'protocol_git_date': pd, 'first_raw_git_date': fd,
       'script': str(SCRIPT.relative_to(REPO)), 'notes': notes}
lines = [l for l in OUT.read_text().splitlines() if l.strip()] if OUT.exists() else []
lines = [l for l in lines if json.loads(l)['row_no'] != ROW] + [json.dumps(row)]
OUT.write_text('\n'.join(lines) + '\n')
from collections import Counter
print(json.dumps(row, indent=1)[:6000])
print('STATUS', Counter(c['status'] for c in claims))
