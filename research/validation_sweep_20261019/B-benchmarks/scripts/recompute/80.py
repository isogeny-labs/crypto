import json, math, os, re, subprocess
REPO = '/home/user/crypto'
ROW = 80
STUDY = 'research/joux_vitse_n1_20261015'
D = os.path.join(REPO, STUDY)
SELF = 'scripts/recompute/80.py'
S = json.load(open(os.path.join(D, 'summary.json')))
fl_sum = [os.path.join(STUDY, 'summary.json')]
claims = []
def add(claimed, computed, definition, files, status):
    claims.append({'claimed': claimed, 'computed': computed, 'definition': definition, 'files': files, 'status': status})

rungs = S['rungs']
ct = [r['c_trial_mean'] for r in rungs]
add('trial 5.9-9.2e9 F_p mult. (flat)', f'{min(ct)/1e9:.2f}-{max(ct)/1e9:.2f}e9',
    'min and max of rungs[].c_trial_mean in summary.json (aggregate from data/ladder not recomputed)', fl_sum + [os.path.join(STUDY, 'data/ladder')],
    'partial' if (round(min(ct)/1e9, 1), round(max(ct)/1e9, 1)) == (5.9, 9.2) else 'mismatch')
xs = [math.log(r['p']) for r in rungs]; ys = [math.log(c) for c in ct]
n = len(xs); mx = sum(xs)/n; my = sum(ys)/n
sxx = sum((x-mx)**2 for x in xs); b = sum((x-mx)*(y-my) for x, y in zip(xs, ys))/sxx
res = [y - (my + b*(x-mx)) for x, y in zip(xs, ys)]
se = math.sqrt(sum(e*e for e in res)/(n-2)/sxx)
add('C_trial exponent p^-0.12', f'{b:.3f} (se {se:.3f})',
    'OLS of ln c_trial_mean on ln p over the 12 rungs in summary.json', fl_sum, 'match' if round(b, 2) == -0.12 else 'mismatch')
add('C_trial exponent uncertainty +-0.05', f'+-{se:.3f}',
    'standard error of the slope from the same OLS (summary.json fit_c_trial_vs_p.slope_se = %.4f)' % S['fit_c_trial_vs_p']['slope_se'], fl_sum,
    'match' if round(se, 2) == 0.05 else 'mismatch')
r241 = [r for r in rungs if r['p'] == 241][0]
add('rate x 24p 0.96 at p=241 (trials per relation ~ 25p)', f'{r241["rate_times_24p"]:.4f}',
    'rate_times_24p of the p=241 rung in summary.json', fl_sum, 'match' if round(r241['rate_times_24p'], 2) == 0.96 else 'mismatch')
dl = [d for d in S['dlp'] if d.get('ic')]
rat = {}
for d in dl:
    rat[f"{d['file']}"] = (d['ic']['online_muls'] / d['rho_fp_muls'], d['ratio_online_ic_over_rho'])
for claimed, key, dec in [('IC/rho 4.5e7 (p=11, s1)', 'p11_s1.jsonl', 4), ('IC/rho 1.8e8 (p=11, s2)', 'p11_s2.jsonl', 8), ('IC/rho 6.8e7 (p=31, s1)', 'p31_s1.jsonl', 8)]:
    calc, stored = rat[key]
    target = {'p11_s1.jsonl': 4.5e7, 'p11_s2.jsonl': 1.8e8, 'p31_s1.jsonl': 6.8e7}[key]
    add(claimed, f'{calc:.4g} (stored {stored:.4g}; compared at printed precision)',
        'online_muls / rho_fp_muls from the summary dlp entry; stored ratio_online_ic_over_rho shown for comparison', [os.path.join(STUDY, 'data/dlp', key)] + fl_sum,
        'match' if round(calc / 10 ** math.floor(math.log10(target)), 1) == round(target / 10 ** math.floor(math.log10(target)), 1) and abs(calc - stored) / stored < 1e-6 else 'mismatch')
ver = sum(1 for d in dl if d['ic'].get('verified') is True and d.get('rho_verified') is True)
add('all DLP answers verified (3 complete runs)', f'{ver}/{len(dl)} complete runs with ic.verified and rho_verified', 'count over summary.json dlp entries with ic', fl_sum,
    'match' if ver == 3 and len(dl) == 3 else 'mismatch')
rows = S['model_extrapolation']['rows']
def cross():
    for a, b2 in zip(rows, rows[1:]):
        if a['ic_over_rho'] >= 1 >= b2['ic_over_rho']:
            fa, fb = math.log(a['ic_over_rho']), math.log(b2['ic_over_rho'])
            return a['log2_q'] + (b2['log2_q'] - a['log2_q']) * fa / (fa - fb), a['log2_group'] + (b2['log2_group'] - a['log2_group']) * fa / (fa - fb)
lq, lg = cross()
add('extrapolated crossover q ~ 2^58.2', f'2^{lq:.2f}', 'log-linear interpolation of ic_over_rho = 1 over model_extrapolation.rows in summary.json', fl_sum,
    'match' if round(lq, 1) == 58.2 else 'mismatch')
add('group ~ 2^291 at the crossover', f'2^{lg:.1f}', 'log2_group interpolated at the same crossover', fl_sum, 'match' if round(lg) == 291 else 'mismatch')
rr = {r['log2_q']: r['ic_over_rho'] for r in rows}
add('about 1e7 times rho at q=2^10 (model)', f'{rr[10.0]:.3g}', 'ic_over_rho at log2_q=10 in model_extrapolation.rows', fl_sum, 'match' if round(math.log10(rr[10.0])) == 7 else 'mismatch')
add('2e6 times rho at q=2^16 (model)', f'{rr[16.0]:.3g}', 'ic_over_rho at log2_q=16 in model_extrapolation.rows', fl_sum, 'match' if abs(rr[16.0] - 2e6) / 2e6 <= 0.15 else 'mismatch')
J = json.load(open(os.path.join(D, 'data/jv_quintic_rerun/weierstrass_p271_s1.json')))[0]
cs = J['c_second']['mean']
add('jv_quintic C" = 1.104e10', f'{cs:.5g}', 'mean of c_second over the jv_quintic_rerun record (data/jv_quintic_rerun/weierstrass_p271_s1.json)',
    [os.path.join(STUDY, 'data/jv_quintic_rerun/weierstrass_p271_s1.json')], 'match' if round(cs / 1e10, 3) == 1.104 else 'mismatch')
add('scoreboard C" was 3.0e10', 'no_raw', 'the 3.0e10 scoreboard value is not in this study directory', [], 'no_raw_in_repo')
add('checks failed 0', str(S['checks_failed']), 'checks_failed in summary.json', fl_sum, 'match' if S['checks_failed'] == 0 else 'mismatch')

def grep_lines(rel, pat):
    out = []
    with open(os.path.join(D, rel)) as fh:
        for i, line in enumerate(fh, 1):
            if re.search(pat, line):
                out.append(f'{STUDY}/{rel}:{i}: {line.rstrip()}')
    return out
aa = grep_lines('README.md', r'[Cc]ontrol|[Nn]ull|[Cc]heck')
iso = grep_lines('README.md', r'[Ii]solation|isolat|[Hh]ost')
offr = []
for root, _, files in os.walk(D):
    if 'raw' in root: continue
    for f in files:
        if f.endswith(('.md', '.typ', '.json')) and not f.endswith('.pdf'):
            p = os.path.join(root, f)
            for i, line in enumerate(open(p, errors='replace'), 1):
                if re.search(r'drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:|Modal|not committed|scratch|/tmp', line):
                    offr.append(f'{os.path.relpath(p, REPO)}:{i}: {line.rstrip()[:200]}')
def git_date(path):
    out = subprocess.run(['git', '-C', REPO, 'log', '--diff-filter=A', '--format=%H %cI', '--', path], capture_output=True, text=True).stdout.strip()
    return out if out else 'EMPTY'
out = {'row_no': ROW, 'slug': '20261015-joux-vitse-k5-n1', 'claims': claims,
       'identical': {'claimed': False, 'n_compared': None, 'n_equal': None, 'n_differ': None, 'files': [], 'status': 'no_claim'},
       'offrepo_refs': offr[:20], 'aa_lines': aa, 'isolation_lines': iso,
       'protocol_git_date': git_date(os.path.join(STUDY, 'PROTOCOL.md')),
       'first_raw_git_date': git_date(os.path.join(STUDY, 'data/edwards/cost_p271_s1.jsonl')),
       'script': SELF,
       'notes': ('Rung trial costs, exponent and trials-per-relation are the summary.json aggregates (the ladder raw records were not re-aggregated). '
                 'DLP ratios and the crossover are recomputed from summary.json components. '
                 'Off-repo grep over study .md/.typ/.json (excluding raw): ' + ('hits listed in offrepo_refs' if offr else 'no hits') + '. '
                 'Isolation: README states isolation none (host statement).')}
print(json.dumps(out, ensure_ascii=False))
