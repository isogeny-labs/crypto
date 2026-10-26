import json, statistics, pathlib, re, subprocess, csv
from collections import Counter
REPO = pathlib.Path('/home/user/crypto')
WB = REPO / 'research/validation_sweep_20261019/B-benchmarks'
OUT = WB / 'recompute/SWB-RECOMPUTE-11.jsonl'
ROW = 83
SLUG = '20261014-ecc2k130-signed-sigma'
STUDY = REPO / 'ecc2k130'
BP = STUDY / 'benchmarks/best-of-two'
SCRIPT = WB / 'scripts/recompute/83.py'
PROTOCOL = STUDY / 'BEST-OF-TWO.md'
FIRST_RAW = BP / 'emulation-v1.log'

claims = []
def st_of(pairs, interval=False):
    if not pairs or any(c is None for _, _, c in pairs):
        return 'cannot_define'
    def ok(cl, s, c):
        d = len(s.split('.')[1]) if '.' in s else 0
        return round(c, d) == cl or abs(c - cl) / abs(cl) <= 0.01
    oks = [ok(cl, s, c) for s, cl, c in pairs]
    return 'match' if all(oks) else ('partial' if any(oks) else 'mismatch')
def claim(text, computed, definition, files, pairs, status=None):
    claims.append({'claimed': text, 'computed': computed, 'definition': definition,
                   'files': [str(f.relative_to(REPO)) for f in files], 'status': status or st_of(pairs)})

# GPU run 2 (SIGMA_SIGN=2) from samples.tsv and planted.tsv
R2 = BP / 'gpu-run2/results'
samp = list(csv.DictReader(open(R2 / 'samples.tsv'), delimiter='\t'))
ab = {}
for r in samp:
    if r['phase'] == 'ab':
        ab.setdefault(r['pair'], {})[r['variant']] = float(r['rateMps'])
pr = [ab[p]['candidate'] / ab[p]['control'] for p in sorted(ab)]
ratio_med = statistics.median(pr)
ctrl = statistics.median(ab[p]['control'] for p in ab)
cand = statistics.median(ab[p]['candidate'] for p in ab)
claim('A/B median rate ratio 0.9965 (run 2, SIGMA_SIGN=2)',
      'median over 5 pairs of candidate/control rate = %.5f (per-pair: %s)' % (ratio_med, [round(x, 5) for x in pr]),
      'median over pairs of rateMps(candidate)/rateMps(control), phase ab, gpu-run2 samples.tsv',
      [R2 / 'samples.tsv'], [('0.9965', 0.9965, ratio_med)])
claim('rate 14.731 -> 14.678 B it/s (run 2)', 'control median %.4f B it/s, candidate median %.4f B it/s' % (ctrl / 1000, cand / 1000),
      'median over 5 pairs of rateMps / 1000, phase ab, gpu-run2 samples.tsv',
      [R2 / 'samples.tsv'], [('14.731', 14.731, ctrl / 1000), ('14.678', 14.678, cand / 1000)])
aa = {}
for r in samp:
    if r['phase'] == 'aa':
        aa.setdefault(r['pair'], {})[r['variant']] = float(r['rateMps'])
drift = [abs(aa[p]['control_b'] / aa[p]['control_a'] - 1) for p in sorted(aa)]
claim('A/A drift 0.10% (run 2)', 'max over 5 A/A pairs of |control_b/control_a - 1| = %.5f%%' % (100 * max(drift)),
      'max over pairs of |rateMps(control_b)/rateMps(control_a) - 1|, phase aa, gpu-run2 samples.tsv',
      [R2 / 'samples.tsv'], [('0.10', 0.10, 100 * max(drift))])

pl = list(csv.DictReader(open(R2 / 'planted.tsv'), delimiter='\t'))
cand_rows = [r for r in pl if r['variant'] == 'candidate']
solved = sum(1 for r in cand_rows if r['solved'] == '1')
claim('64/64 planted GF(2^41) logs (SIGMA_SIGN=2)', '%d of %d candidate rows solved (planted.tsv, gpu-run2)' % (solved, len(cand_rows)),
      'count of planted.tsv rows with variant == candidate and solved == 1', [R2 / 'planted.tsv'],
      [('64', 64, solved)])

# Replays: the 300/300 claim appears as a PASS line of integration-candidate.txt; per-report replay records are not committed
hits = sorted(str(f.relative_to(REPO)) for f in R2.iterdir() if f.is_file() and '300/300' in f.read_text(errors='replace'))
has = hits
claim('300/300 device reports replay', 'PASS line containing "300/300" found in: %s; no per-report replay records are committed (uncommitted-artifacts.sha256 lists hashes only)' % hits,
      'text search for the PASS line; the DP corpora are not in the repo', [R2 / 'integration-candidate.txt'], [], status='no_raw_in_repo')

# Iterations: device reference walk, sign 2 vs sigma (c ratio), n = 41, native (v1) and ecc2k130 hashed (v2)
def jl(f):
    return [json.loads(l) for l in (BP / f).read_text().splitlines() if l.strip().startswith('{')]
v1 = jl('device-wc/device-wc-v1.jsonl'); v2 = jl('device-wc/device-wc-v2.jsonl')
def cval(rows, n, dist, sg):
    for r in rows:
        if r['n'] == n and r['dist'] == dist and r['sigma_sign'] == sg:
            return r['c']
    return None
it_native = cval(v1, 41, 'native', 2) / cval(v1, 41, 'native', 0)
it_ecc = cval(v2, 41, 'ecc2k130', 2) / cval(v2, 41, 'ecc2k130', 0)
claim('iterations 0.958-0.967 on the device walk at n = 41', 'c(sign2)/c(sigma): native v1 %.4f; ecc2k130 v2 %.4f' % (it_native, it_ecc),
      'device-wc-v1 native row and device-wc-v2 ecc2k130 row at n=41, sigma_sign 2 divided by sigma_sign 0 (c)',
      [BP / 'device-wc/device-wc-v1.jsonl', BP / 'device-wc/device-wc-v2.jsonl'],
      [('0.958', 0.958, it_native), ('0.967', 0.967, it_ecc)])

# Emulation n=37-59: which emulation walk is SIGMA_SIGN=2 is not named in the study files; print sigmapm/sigma ratios
emu = jl('emulation-v1.jsonl') + jl('emulation-v2.jsonl') + jl('emulation-v3.jsonl')
def ecv(walk, n, dist):
    xs = [r['c'] for r in emu if r['walk'] == walk and r['n'] == n and r['dist'] == dist and r.get('score') in ('hw', None)]
    return xs[0] if xs else None
emu_r = {}
for n in (37, 41, 53, 59):
    for d in ('native', 'ecc2k130'):
        s = ecv('sigma', n, d); p = ecv('sigmapm', n, d)
        emu_r['%s n=%d' % (d, n)] = None if (s is None or p is None) else round(p / s, 4)
claim('iterations 0.951-0.959 emulated at n = 37-59', 'sigmapm c / sigma c: %s; study does not state which emulated walk is SIGMA_SIGN=2' % json.dumps(emu_r),
      'emulation-v{1,2,3}.jsonl rows with walk sigmapm and sigma at n in {37,41,53,59}; walk-to-SIGMA_SIGN mapping not stated', [BP / 'emulation-v2.jsonl', BP / 'emulation-v3.jsonl'], [], status='cannot_define')

# Derived GPU time per solve = iterations / rate, from iteration range 0.958-0.967 and rate ratio 0.9965
lo = 0.958 / ratio_med; hi = 0.967 / ratio_med
claim('derived GPU time per solve 0.961-0.970', 'iterations(0.958, 0.967) / A-B rate ratio = %.4f to %.4f' % (lo, hi),
      'iterations ratio divided by the run-2 A/B median rate ratio (derived, both endpoints)', [R2 / 'samples.tsv'],
      [('0.961', 0.961, lo), ('0.970', 0.970, hi)])

identical = {'claimed': False, 'n_compared': None, 'n_equal': None, 'n_differ': None, 'files': [], 'status': 'not_claimed'}

OFF = re.compile(r'drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:[A-Za-z0-9_.\-]+|Modal volume|not committed|scratch|uncommitted', re.I)
AA = re.compile(r'A/A|A-A|null[- ]control|null control', re.I)
ISO = re.compile(r'isolat|\bL[0-3]\b|NUMA|reservation', re.I)
store_index = REPO / 'store/index.json'
store_txt = store_index.read_text() if store_index.exists() else None
offrepo, aa_l, iso = [], [], []
files = [STUDY / 'BEST-OF-TWO.md'] + sorted(p for p in BP.rglob('*') if p.is_file() and p.suffix in ('.md', '.typ', '.txt', '.tex', '.log') and p.stat().st_size < 1_000_000)
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
notes = ('No PROTOCOL file exists in this study; BEST-OF-TWO.md is used as the stated design. Aggregation: per-pair candidate/control medians over 5 pairs (gpu-run2). '
         'Emulated walk mapping to SIGMA_SIGN=2 is not stated, so the emulation claim is not defined. Run-1 (SIGMA_SIGN=1) numbers were not used. '
         'Off-repo, A/A and isolation lines are whole lines, including uncommitted-artifacts references. Protocol git add date: %s. First raw git add date: %s.') % (pd or 'EMPTY', fd or 'EMPTY')
row = {'row_no': ROW, 'slug': SLUG, 'claims': claims, 'identical': identical, 'offrepo_refs': offrepo,
       'aa_lines': aa_l, 'isolation_lines': iso, 'protocol_git_date': pd, 'first_raw_git_date': fd,
       'script': str(SCRIPT.relative_to(REPO)), 'notes': notes}
lines = [l for l in OUT.read_text().splitlines() if l.strip()] if OUT.exists() else []
lines = [l for l in lines if json.loads(l)['row_no'] != ROW] + [json.dumps(row)]
OUT.write_text('\n'.join(lines) + '\n')
for c in claims: print(c['status'], '|', c['claimed'], '|', c['computed'][:200])
print('STATUS', Counter(c['status'] for c in claims), 'offrepo', len(offrepo), 'aa', len(aa_l), 'iso', len(iso))
