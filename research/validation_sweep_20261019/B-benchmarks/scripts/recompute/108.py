#!/usr/bin/env python3
# SWB-RECOMPUTE-14 row 108: 20261013-ecbench-frontier. Reads committed files only; appends one JSON object.
import json, os, re, subprocess, glob

ROOT = '/home/user/crypto'
OUT = os.path.join(ROOT, 'research/validation_sweep_20261019/B-benchmarks/recompute/SWB-RECOMPUTE-14.jsonl')
F = 'research/ecbench_frontier_20261013'

def ex(p):
    return os.path.exists(os.path.join(ROOT, p))

fd = json.load(open(os.path.join(ROOT, F + '/frontier-data.json')))
leader = fd['canonical_koblitz_medium']['ops_leader']
gaps = {g['label'].replace('\n', ' '): g for g in fd['cold_ops_gap']}
onl = {o['label']: o for o in fd['online_boundary']}
n83 = onl['n83 dense K1798']
n61 = onl['n61 compact orbit']
compact17 = gaps['Compact FB K17']

def audit(name):
    a = json.load(open(os.path.join(ROOT, F + '/audits/' + name)))
    reps = a.get('replays', [])
    ident = sum(1 for r in reps if r.get('detail') == 'identical' and r.get('reproduced'))
    return {'ok': a['ok'], 'records': a['records'], 'verified': a['verified_records'], 'replays': len(reps),
            'reproduced': sum(1 for r in reps if r.get('reproduced')), 'identical': ident,
            'problem_count': len(a['problems']), 'detail_counts': {}}

fb = audit('factor-bases-k17.json')
f4 = audit('f4f5f6-k13.json')
rf = audit('rootfind-n13-replay-all-failed.json')
rf_details = {}
for r in json.load(open(os.path.join(ROOT, F + '/audits/rootfind-n13-replay-all-failed.json'))).get('replays', []):
    rf_details[str(r.get('detail'))] = rf_details.get(str(r.get('detail')), 0) + 1

ops_over_floor = leader['ops_over_floor']
ci = leader['ops_ci95']
n83_ratio_computed = n83['rho_online_seconds'] / n83['ic_online_seconds']
n83_total_computed = n83['setup_seconds'] + n83['ic_online_seconds']

def grep_lines(path, pattern):
    out = []
    p = os.path.join(ROOT, path)
    if not os.path.exists(p):
        return out
    with open(p) as f:
        for i, line in enumerate(f, 1):
            if re.search(pattern, line):
                out.append('%s:%d: %s' % (path, i, line.rstrip('\n')))
    return out

docs = [F + '/README.md', F + '/TESTS.md', F + '/HOST.txt', F + '/METHODS.txt']
offrepo_refs, aa_lines, isolation_lines = [], [], []
store_names = set()
for p in docs:
    offrepo_refs += grep_lines(p, r'drive\.google|/Volumes|SSD|/Users|s3://|gs://|store:|Modal|not committed|scratch')
    aa_lines += grep_lines(p, r'A/A|null-control|null control|negative control|positive control')
    isolation_lines += grep_lines(p, r'isolat|shared|idle|reserved|Xeon|vCPU|EPYC|L2|L1|L0')
    with open(os.path.join(ROOT, p)) as f:
        store_names |= set(re.findall(r'store:([A-Za-z0-9_.-]+)', f.read()))
store_index = open(os.path.join(ROOT, 'store/index.json')).read() if ex('store/index.json') else ''
store_check = {n: (n in store_index) for n in sorted(store_names)}

def git_add(path):
    r = subprocess.run(['git', '-C', ROOT, 'log', '--diff-filter=A', '--format=%H %cI', '--', path], capture_output=True, text=True)
    return r.stdout.strip() or 'empty'

cited = {
    'compact_n17': gaps['Compact FB K17']['source'],
    'n61': n61['source'],
    'n83': n83['source'],
}
cited_exists = {k: (v, ex(v.split(':')[0])) for k, v in cited.items()}

claims = [
    {'claimed': 'cold one-target frontier remains signed-Frobenius rho (medium-tier 1.089x floor [0.927, 1.261])',
     'computed': 'frontier-data ops_leader %s ops_over_floor %.6f -> %.3f ; ci95 [%.6f, %.6f] -> [%.3f, %.3f]' % (
         leader['method'], ops_over_floor, round(ops_over_floor, 3), ci[0], ci[1], round(ci[0], 3), round(ci[1], 3)),
     'definition': 'copied value of ops_over_floor and ops_ci95 in frontier-data.json (canonical_koblitz_medium.ops_leader), printed to three decimals; the synthesis is not re-derived from the bound record (docs/bounds/frontier.json not in study dir)',
     'files': [F + '/frontier-data.json'],
     'status': 'match' if (round(ops_over_floor, 3) == 1.089 and round(ci[0], 3) == 0.927 and round(ci[1], 3) == 1.261) else 'mismatch'},
    {'claimed': 'compact orbit is the best measured factor-base arm but 9.985x matched rho at n17',
     'computed': 'frontier-data cold_ops_gap Compact FB K17 ratio_to_matched_rho = %s (reference %s, lower_bound %s); source %s exists in repo: %s' % (
         compact17['ratio_to_matched_rho'], compact17['reference'], compact17['lower_bound'], compact17['source'], cited_exists['compact_n17'][1]),
     'definition': 'copied value; the cited table research/ecbench_factor_bases_20261013/table-koblitz17.md is the source, and the matched rho S is not in this study dir',
     'files': [F + '/frontier-data.json', compact17['source'].split(':')[0]],
     'status': 'partial' if cited_exists['compact_n17'][1] else 'no_raw_in_repo'},
    {'claimed': 'admitted retained-setup online frontier is n61 rho/IC 13.9x [9.8, 19.1]',
     'computed': 'frontier-data online_boundary n61 compact orbit online_rho_over_ic %s ci95 [%s, %s]; source %s exists in repo: %s' % (
         n61['online_rho_over_ic'], n61['online_ci95'][0], n61['online_ci95'][1], n61['source'], cited_exists['n61'][1]),
     'definition': 'copied value from frontier-data.json; the online runs themselves are not in this study dir',
     'files': [F + '/frontier-data.json', n61['source'].split(':')[0]],
     'status': 'partial' if cited_exists['n61'][1] else 'no_raw_in_repo'},
    {'claimed': 'N83 is exploratory at 18.528x',
     'computed': 'rho_online_seconds / ic_online_seconds = %.6f / %.9f = %.6f (frontier-data online_boundary N83 online_rho_over_ic %s); source %s exists in repo: %s' % (
         n83['rho_online_seconds'], n83['ic_online_seconds'], n83_ratio_computed, n83['online_rho_over_ic'], n83['source'], cited_exists['n83'][1]),
     'definition': 'ratio of the two component online times recorded in frontier-data.json (rho over IC)',
     'files': [F + '/frontier-data.json', n83['source'].split(':')[0]],
     'status': 'match' if round(n83_ratio_computed, 3) == 18.528 else 'mismatch'},
    {'claimed': 'its complete IC process is 339.068 s (reported separately from rho online)',
     'computed': 'complete_process_seconds recorded = %s ; setup_seconds + ic_online_seconds = %s + %s = %.6f' % (
         n83['complete_process_seconds'], n83['setup_seconds'], n83['ic_online_seconds'], n83_total_computed),
     'definition': 'the recorded complete_process_seconds, compared with setup_seconds + ic_online_seconds from the same entry',
     'files': [F + '/frontier-data.json'],
     'status': 'match' if round(n83_total_computed, 3) == 339.067 and abs(n83_total_computed - 339.068) / 339.068 <= 0.01 else 'mismatch'},
    {'claimed': 'factor-base 8/8 replays identical (n17)',
     'computed': 'audits/factor-bases-k17.json: records %d valid %d ; replays %d reproduced %d identical %d' % (fb['records'], fb['verified'], fb['replays'], fb['reproduced'], fb['identical']),
     'definition': 'replay records with reproduced true and detail identical',
     'files': [F + '/audits/factor-bases-k17.json'],
     'status': 'match' if (fb['replays'] == 8 and fb['identical'] == 8) else 'mismatch'},
    {'claimed': 'representative F4/F5/F6 7/7 replays identical',
     'computed': 'audits/f4f5f6-k13.json: records %d valid %d ; replays %d reproduced %d identical %d' % (f4['records'], f4['verified'], f4['replays'], f4['reproduced'], f4['identical']),
     'definition': 'as above',
     'files': [F + '/audits/f4f5f6-k13.json'],
     'status': 'match' if (f4['replays'] == 7 and f4['identical'] == 7) else 'mismatch'},
    {'claimed': 'root-finder session 120/168 replays with 48 inherited-F4 solver-counter mismatches retained',
     'computed': 'audits/rootfind-n13-replay-all-failed.json: session ok=%s records %d ; replays %d reproduced %d ; detail counts %s ; problems %d' % (
         rf['ok'], rf['records'], rf['replays'], rf['reproduced'], rf_details, rf['problem_count']),
     'definition': 'reproduced count out of replay count, and count of replays whose detail is not identical (audit wording: "differs in phase counts")',
     'files': [F + '/audits/rootfind-n13-replay-all-failed.json'],
     'status': 'match' if (rf['replays'] == 168 and rf['reproduced'] == 120 and rf_details.get('differs in phase counts') == 48) else 'mismatch'},
]

identical = {
    'claimed': True,
    'n_compared': fb['replays'] + f4['replays'],
    'n_equal': fb['identical'] + f4['identical'],
    'n_differ': (fb['replays'] + f4['replays']) - (fb['identical'] + f4['identical']),
    'files': [F + '/audits/factor-bases-k17.json', F + '/audits/f4f5f6-k13.json'],
    'status': 'match' if (fb['identical'] + f4['identical']) == (fb['replays'] + f4['replays']) else 'mismatch',
}

row = {
    'row_no': 108,
    'slug': '20261013-ecbench-frontier',
    'claims': claims,
    'identical': identical,
    'offrepo_refs': offrepo_refs,
    'aa_lines': aa_lines,
    'isolation_lines': isolation_lines,
    'protocol_git_date': 'none: no PROTOCOL file in %s' % F,
    'first_raw_git_date': 'audits/factor-bases-k17.json: %s ; frontier-data.json: %s' % (git_add(F + '/audits/factor-bases-k17.json'), git_add(F + '/frontier-data.json')),
    'script': 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/108.py',
    'notes': 'Study dir has no raw per-run records; the headline ratios are copied from frontier-data.json and cite other study directories. Store names found in the study text: %s; store/index.json lists them: %s. The identical block counts only replay-vs-original comparisons from the two audits that the headline states as identical; the root-finder replay (168) is reported in its own claim. Rootfind replay audit detail strings: %s. Cited sources present in repo: %s.' % (
        sorted(store_names), store_check, rf_details, {k: v[1] for k, v in cited_exists.items()}),
}

os.makedirs(os.path.dirname(OUT), exist_ok=True)
with open(OUT, 'a') as f:
    f.write(json.dumps(row, ensure_ascii=False) + '\n')
print('row 108 written; claims:', len(claims))
for c in claims:
    print(c['status'], '|', c['claimed'], '|', c['computed'])
print('identical:', json.dumps(identical))
print('offrepo_refs:', len(offrepo_refs), 'aa_lines:', len(aa_lines), 'isolation_lines:', len(isolation_lines), 'store_names:', store_check)
