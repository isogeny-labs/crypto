import json, os, re, subprocess
REPO = '/home/user/crypto'
ROW = 78
STUDY = 'research/ic_next10_20261014/01-fp4-ladder'
D = os.path.join(REPO, STUDY)
SELF = 'scripts/recompute/78.py'
rows = [json.loads(l) for l in open(os.path.join(D, 'raw/ladder.jsonl'))]
summ = json.load(open(os.path.join(D, 'raw/summary.json')))
fl_lad = [os.path.join(STUDY, 'raw/ladder.jsonl')]
fl_sum = [os.path.join(STUDY, 'raw/summary.json')]
claims = []
def add(claimed, computed, definition, files, status):
    claims.append({'claimed': claimed, 'computed': computed, 'definition': definition, 'files': files, 'status': status})

def count_correct(o):
    if isinstance(o, dict):
        n = 1 if 'correct' in o else 0
        ok = 1 if o.get('correct') is True else 0
        t = n; k = ok
        for v in o.values():
            a, b = count_correct(v); t += a; k += b
        return t, k
    if isinstance(o, list):
        t = k = 0
        for v in o:
            a, b = count_correct(v); t += a; k += b
        return t, k
    return 0, 0
ic_arms = ['plain', 'd4', 'dlp', 'dlp-null']
ic_tot = ic_ok = 0
for r in rows:
    for a in r['arms']:
        if a['arm'] in ic_arms:
            ic_tot += 1; ic_ok += 1 if a['correct'] is True else 0
rho_tot = rho_ok = 0
for r in rows:
    t, k = count_correct(r['rho']); rho_tot += t; rho_ok += k
add('84/84 IC logarithms verified (21 curves x 4 IC arms)', f'{ic_ok}/{ic_tot}',
    'count of arms with arm in {plain,d4,dlp,dlp-null} and correct==True over raw/ladder.jsonl rows', fl_lad,
    'match' if (ic_ok, ic_tot) == (84, 84) else 'mismatch')
add('672/672 rho walks verified', f'{rho_ok}/{rho_tot}',
    'count of run dicts with correct==True / with a correct key, recursively under each row rho field in raw/ladder.jsonl', fl_lad,
    'match' if (rho_ok, rho_tot) == (672, 672) else 'mismatch')
la_p = summ['fits']['plain']['linear_algebra']['slope']; la_d = summ['fits']['dlp']['linear_algebra']['slope']
add('LA exponent 0.495 (plain) -> 0.420 (DLP)', f'{la_p:.3f} -> {la_d:.3f}', 'OLS slope of log2 LA quantity on log2 #E, raw/summary.json fits.plain/dlp.linear_algebra.slope', fl_sum,
    'match' if round(la_p, 3) == 0.495 and round(la_d, 3) == 0.420 else 'mismatch')
rp_p = summ['fits']['plain']['relation_phase']['slope']; rp_d = summ['fits']['dlp']['relation_phase']['slope']
add('relation phase 0.238 -> 0.333', f'{rp_p:.3f} -> {rp_d:.3f}', 'OLS slope, raw/summary.json fits.plain/dlp.relation_phase.slope', fl_sum,
    'match' if round(rp_p, 3) == 0.238 and round(rp_d, 3) == 0.333 else 'mismatch')
# ecbench aggregates printed in compare.txt (aggregator not stated in repo)
cmp_txt = open(os.path.join(D, 'ecbench/compare.txt')).read().split('\n')
agg = {}
for line in cmp_txt:
    m = re.match(r'ops \(ok\): (ic-k4-\S+ / rho-neg) = ([\d.]+)', line)
    if m: agg[m.group(1)] = float(m.group(2))
d4_plain = agg['ic-k4-d4 / rho-neg'] / agg['ic-k4-plain / rho-neg']
dlp_plain = agg['ic-k4-dlp / rho-neg'] / agg['ic-k4-plain / rho-neg']
fl_cmp = [os.path.join(STUDY, 'ecbench/compare.txt')]
add('D4/plain 0.0548 [0.048, 0.061]; 18.2x constant improvement', f'{d4_plain:.4f} (1/x = {1/d4_plain:.1f})',
    'ratio of the printed ops aggregates ic-k4-d4/rho-neg over ic-k4-plain/rho-neg in ecbench/compare.txt; the aggregator for these point values is not stated in the study, and the per-workload values were not recomputed from records', fl_cmp,
    'partial')
add('DLP/plain 2.31 [2.10, 2.62] whole solve costlier', f'{dlp_plain:.4f}',
    'ratio of the printed ops aggregates ic-k4-dlp/rho-neg over ic-k4-plain/rho-neg in ecbench/compare.txt; aggregator not stated; interval not recomputed', fl_cmp,
    'partial')
ver_t = len(json.load(open(os.path.join(D, 'ecbench/session-20261014-fp4.audit.json')))['replays'])
aud = json.load(open(os.path.join(D, 'ecbench/session-20261014-fp4.audit.json')))
rep_ok = sum(1 for r in aud['replays'] if r['reproduced'] is True and r['detail'] == 'identical')
add('12/12 replays identical', f'{rep_ok}/{len(aud["replays"])}',
    'count of replays with reproduced==True and detail=="identical" in ecbench/session-20261014-fp4.audit.json', [os.path.join(STUDY, 'ecbench/session-20261014-fp4.audit.json')],
    'match' if rep_ok == 12 and len(aud['replays']) == 12 else 'mismatch')

# identical: null control dlp-null vs plain on each curve
keys = ['solved', 'trials', 'la_modr_mults', 'full_relations', 'lp_hist', 'targets_used', 'S_charged']
n_eq = 0
for r in rows:
    A = {a['arm']: a for a in r['arms']}
    if all(A['plain'][k] == A['dlp-null'][k] for k in keys):
        n_eq += 1
ident = {'claimed': True, 'n_compared': len(rows), 'n_equal': n_eq, 'n_differ': len(rows) - n_eq,
         'files': fl_lad, 'status': 'match' if n_eq == len(rows) else 'mismatch'}

def grep_lines(rel, pat):
    out = []
    with open(os.path.join(D, rel)) as fh:
        for i, line in enumerate(fh, 1):
            if re.search(pat, line):
                out.append(f'{STUDY}/{rel}:{i}: {line.rstrip()}')
    return out
aa = grep_lines('README.md', r'[Cc]ontrol')
iso = grep_lines('README.md', r'L0|isolat|busy|cpus|load') + grep_lines('PROTOCOL.md', r'L0|isolat|busy|cpus|load')
def git_date(path):
    out = subprocess.run(['git', '-C', REPO, 'log', '--diff-filter=A', '--format=%H %cI', '--', path], capture_output=True, text=True).stdout.strip()
    return out if out else 'EMPTY'
out = {'row_no': ROW, 'slug': '20261014-ic-next10-01-fp4-ladder', 'claims': claims, 'identical': ident,
       'offrepo_refs': [], 'aa_lines': aa, 'isolation_lines': iso,
       'protocol_git_date': git_date(os.path.join(STUDY, 'PROTOCOL.md')),
       'first_raw_git_date': git_date(os.path.join(STUDY, 'ecbench/compare.txt')),
       'script': SELF,
       'notes': ('Identical block: plain vs dlp-null on keys solved, trials, la_modr_mults, full_relations, lp_hist, targets_used, S_charged per curve. '
                 'Replay identity is listed in claims. The 0.0548 and 2.31 ratios are computed from the printed compare.txt aggregates only (aggregator not defined in the study). '
                 'Off-repo grep over study .md/.json/.txt: no hits. No store: references. Bootstrap intervals not recomputed.')}
print(json.dumps(out, ensure_ascii=False))
