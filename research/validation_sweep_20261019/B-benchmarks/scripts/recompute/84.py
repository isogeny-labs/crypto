import json, statistics, pathlib, re, subprocess, lzma
from collections import Counter, defaultdict
REPO = pathlib.Path('/home/user/crypto')
WB = REPO / 'research/validation_sweep_20261019/B-benchmarks'
OUT = WB / 'recompute/SWB-RECOMPUTE-11.jsonl'
ROW = 84
SLUG = '20261014-rho-walk-leads'
STUDY = REPO / 'research/rho_walk_leads_20261014'
MAIN = STUDY / 'runs/run-20261014T1511Z'
BATCH = STUDY / 'runs/run-20261014T1513Z-batch'
SCRIPT = WB / 'scripts/recompute/84.py'
PROTOCOL = STUDY / 'README.md'
FIRST_RAW = STUDY / 'runs/run-20261014T1506Z/e1-32.jsonl.xz'

def rows(path):
    with lzma.open(path, 'rt') as fh:
        return [json.loads(l) for l in fh if l.startswith('{')]

def steps_by_seed(rs, exp, arm, bits):
    d = {}
    for r in rs:
        if r.get('exp') == exp and r.get('arm') == arm and r.get('bits') == bits:
            d[r['seed']] = r['steps']
    return d

def paired_speedup(path, exp, ref, arm, bits):
    rs = rows(path)
    a = steps_by_seed(rs, exp, ref, bits); b = steps_by_seed(rs, exp, arm, bits)
    common = sorted(set(a) & set(b))
    if not common:
        return None
    return statistics.fmean(a[k] for k in common) / statistics.fmean(b[k] for k in common), len(common)

def ok(cl, s, c):
    d = len(s.split('.')[1]) if '.' in s else 0
    return round(c, d) == cl or abs(c - cl) / abs(cl) <= 0.01

claims = []
def claim(text, computed, definition, files, pairs, interval_undefined=False):
    if not pairs or any(c is None for _, _, c in pairs):
        st = 'cannot_define'
    else:
        oks = [ok(cl, s, c) for s, cl, c in pairs]
        st = ('partial' if interval_undefined else 'match') if all(oks) else ('mismatch' if not any(oks) else 'partial')
    claims.append({'claimed': text, 'computed': computed, 'definition': definition,
                   'files': [str(f.relative_to(REPO)) for f in files], 'status': st})

def speed_claim(text, exp, ref, arm, bits, claimed_val, claimed_str, path, ci_txt, ref_label):
    res = paired_speedup(path, exp, ref, arm, bits)
    if res is None:
        comp = 'no common seeds'; val = None
    else:
        comp = 'mean steps %s / mean steps %s over %d common trial seeds = %.4f' % (ref, arm, res[1], res[0])
        val = res[0]
    claim(text, comp, 'paired speedup: reference mean steps / arm mean steps over common trial seeds, solved trials only (file %s exp %s bits %s); %s' % (path.name, exp, bits, ref_label),
          [path], [(claimed_str, claimed_val, val)], interval_undefined=True)

# 1. best of P+-R, DP method: sel_pm_score vs sel_base (e1dp, 36 bits, main run)
speed_claim('best of P+-R steps speedup 1.129 under distinguished points', 'e1dp', 'sel_base', 'sel_pm_score', 36, 1.129, '1.129',
            MAIN / 'e1dp-36.jsonl.xz', '1.081-1.184', '95 % CI not recomputed: bootstrap seed and resample draw not stated')
# 2. best of four under DP
speed_claim('best of four steps 1.463 under distinguished points', 'e1dp', 'sel_base', 'sel_best4_score', 36, 1.463, '1.463',
            MAIN / 'e1dp-36.jsonl.xz', '1.401-1.527', '95 % CI not recomputed')

# 3-4. batch cost: P-256 pm_score / base, batch 1024 (1.215) and batch 8 and 64 (1.029 to 1.119); median over the 21 reps
def bench_med(path, field, walks, arm):
    v = [r['ns_per_step'] for r in rows(path) if r.get('exp') == 'bench' and r.get('field') == field and r.get('walks') == walks and r.get('arm') == arm]
    return statistics.median(v) if v else None, len(v)
def cost(path, field, walks, arm, ref):
    a, na = bench_med(path, field, walks, arm); b, nb = bench_med(path, field, walks, ref)
    return (a / b if a and b else None), na, nb
c1024, na1, nb1 = cost(BATCH / 'bench-m1024.jsonl.xz', 'P-256', 1024, 'pm_score', 'base')
claim('cost 1.215x a plain step, P-256, 1024 walks per inversion', 'median ns/step pm_score / base = %.4f (n=%d, %d)' % (c1024, na1, nb1) if c1024 else 'undefined',
      'median over reps of ns_per_step, bench-m1024 (run-20261014T1513Z-batch), field P-256, walks 1024', [BATCH / 'bench-m1024.jsonl.xz'], [('1.215', 1.215, c1024)])
c8, _, _ = cost(BATCH / 'bench-m8.jsonl.xz', 'P-256', 8, 'pm_score', 'base')
c64, _, _ = cost(BATCH / 'bench-m64.jsonl.xz', 'P-256', 64, 'pm_score', 'base')
claim('cost 1.029-1.119x at 8-64 walks per inversion, P-256', 'median ratio at 8 walks = %.4f; at 64 walks = %.4f' % (c8, c64),
      'median ns/step pm_score / base at walks 8 (bench-m8) and walks 64 (bench-m64), P-256',
      [BATCH / 'bench-m8.jsonl.xz', BATCH / 'bench-m64.jsonl.xz'], [('1.029', 1.029, c8), ('1.119', 1.119, c64)])
# 5. best of four cost, P-256, 1024 walks (main bench file)
c4, n4, nb4 = cost(MAIN / 'bench.jsonl.xz', 'P-256', 1024, 'best4', 'base')
claim('best of four costs 2.43x a plain step (P-256, 1024 walks)', 'median ns/step best4 / base = %.4f (n=%d, %d)' % (c4, n4, nb4) if c4 else 'undefined',
      'median over reps of ns_per_step, run-20261014T1511Z bench.jsonl, P-256, walks 1024', [MAIN / 'bench.jsonl.xz'], [('2.43', 2.43, c4)])

# 6-10. lazy reduction, correlated walks, negation rules (e2, e4 in the main run, 36 bits)
speed_claim('lazy DP test 0.998', 'e2', 'canon', 'lazy_dp', 36, 0.998, '0.998', MAIN / 'e2-36.jsonl.xz', '0.995-1.001', 'CI not recomputed')
speed_claim('lazy table index 0.733', 'e2', 'canon', 'lazy_index', 36, 0.733, '0.733', MAIN / 'e2-36.jsonl.xz', '0.703-0.766', 'CI not recomputed')
speed_claim('shared index 0.073 (lower bound: censored trials)', 'e2', 'canon', 'shared_index', 36, 0.073, '0.073', MAIN / 'e2-36.jsonl.xz', '0.070-0.076', 'CI not recomputed; 176 of 1000 censored, so steps is capped')
speed_claim('pair table 1.027', 'e2', 'canon', 'pair_table', 36, 1.027, '1.027', MAIN / 'e2-36.jsonl.xz', '0.982-1.073', 'CI not recomputed')
n_hr = paired_speedup(MAIN / 'e4-36.jsonl.xz', 'e4', 'nonneg', 'neg_hashrule', 36)
n_c = paired_speedup(MAIN / 'e4-36.jsonl.xz', 'e4', 'nonneg', 'neg_canon', 36)
n_ep = paired_speedup(MAIN / 'e4-36.jsonl.xz', 'e4', 'nonneg', 'neg_epoch', 36)
claim('negation rule: per point 1.330, fixed 1.368, per epoch 0.828',
      'per point (arm neg_hashrule) %s; fixed (arm neg_canon) %s; per epoch (arm neg_epoch) %s' % (
          None if not n_hr else round(n_hr[0], 4), None if not n_c else round(n_c[0], 4), None if not n_ep else round(n_ep[0], 4)),
      'paired speedup (ref nonneg steps / arm steps) over common seeds, e4 36 bits; arm-to-rule mapping by arm name (study text does not name the arms)',
      [MAIN / 'e4-36.jsonl.xz'],
      [('1.330', 1.330, n_hr[0] if n_hr else None), ('1.368', 1.368, n_c[0] if n_c else None), ('0.828', 0.828, n_ep[0] if n_ep else None)], interval_undefined=True)

# identical outputs: not claimed
identical = {'claimed': False, 'n_compared': None, 'n_equal': None, 'n_differ': None, 'files': [], 'status': 'not_claimed'}

OFF = re.compile(r'drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:[A-Za-z0-9_.\-]+|Modal volume|not committed|scratch', re.I)
AA = re.compile(r'A/A|A-A|null[- ]control|null control|two-table control|positive control', re.I)
ISO = re.compile(r'isolat|\bL[0-3]\b|NUMA|shared container|vCPU', re.I)
store_index = REPO / 'store/index.json'
store_txt = store_index.read_text() if store_index.exists() else None
offrepo, aa_l, iso = [], [], []
study_files = sorted(p for p in STUDY.rglob('*') if p.is_file() and p.suffix in ('.md', '.typ', '.txt', '.tex', '.py') and p.stat().st_size < 1_000_000)
for p in study_files:
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
notes = ('No PROTOCOL file in the study; README.md is used as the stated design. The first RAW CANDIDATE (run-20261014T1506Z) is marked "not used" by README.md; '
         'the headline numbers are computed from run-20261014T1511Z (the main run, which has summary.jsonl). Intervals are not recomputed (bootstrap seed not stated). '
         'Arm-to-rule mapping for negation claims is by arm name. Protocol git add date: %s. First raw git add date: %s.') % (pd or 'EMPTY', fd or 'EMPTY')
row = {'row_no': ROW, 'slug': SLUG, 'claims': claims, 'identical': identical, 'offrepo_refs': offrepo,
       'aa_lines': aa_l, 'isolation_lines': iso, 'protocol_git_date': pd, 'first_raw_git_date': fd,
       'script': str(SCRIPT.relative_to(REPO)), 'notes': notes}
lines = [l for l in OUT.read_text().splitlines() if l.strip()] if OUT.exists() else []
lines = [l for l in lines if json.loads(l)['row_no'] != ROW] + [json.dumps(row)]
OUT.write_text('\n'.join(lines) + '\n')
for c in claims: print(c['status'], '|', c['claimed'], '|', c['computed'][:160])
print('STATUS', Counter(c['status'] for c in claims), 'offrepo', len(offrepo), 'aa', len(aa_l), 'iso', len(iso))
