#!/usr/bin/env python3
# SWB-RECOMPUTE-14 row 111: 20261012-rootfinding-decomposition. Reads committed files only; appends one JSON object.
import json, os, re, statistics, subprocess

ROOT = '/home/user/crypto'
OUT = os.path.join(ROOT, 'research/validation_sweep_20261019/B-benchmarks/recompute/SWB-RECOMPUTE-14.jsonl')
R = 'research/ecbench_rootfinding_assessment_20261012'

def jl(path):
    with open(os.path.join(ROOT, path)) as f:
        return [json.loads(l) for l in f if l.strip()]

def pair_ratio(rs, num_arm, den_arm):
    num, den = {}, {}
    for r in rs:
        if r['warmup']:
            continue
        key = (r['workload']['workload_id'], r['round'])
        if r['arm'] == num_arm:
            num[key] = r['cost']['s']
        if r['arm'] == den_arm:
            den[key] = r['cost']['s']
    keys = sorted(set(num) & set(den))
    return sum(num[k] for k in keys) / sum(den[k] for k in keys), len(keys), len(num), len(den)

def arm_mean_s(rs, arm):
    v = [r['cost']['s'] for r in rs if (not r['warmup']) and r['arm'] == arm]
    return statistics.mean(v), len(v)

def arm_lower_bound(rs, arm):
    return sorted(set(bool(r['cost']['lower_bound']) for r in rs if (not r['warmup']) and r['arm'] == arm))

def arm_unpriced(rs, arm):
    s = set()
    for r in rs:
        if (not r['warmup']) and r['arm'] == arm:
            s.update(r['cost'].get('unpriced', []))
    return sorted(s)

def verified(rs, arm):
    v = [r for r in rs if (not r['warmup']) and r['arm'] == arm]
    return sum(1 for r in v if r['outcome']['status'] == 'verified'), len(v)

def levels(rs):
    return sorted(set(r['isolation']['level'] for r in rs))

prime = jl(R + '/sessions/prime20/records.jsonl')
kob = jl(R + '/sessions/koblitz17/records.jsonl')

q_prime, pairs_prime, n_pt_p, n_st_p = pair_ratio(prime, 'ic-pair-table', 'ic-subtract')
q_kob, pairs_kob, n_pt_k, n_st_k = pair_ratio(kob, 'ic-pair-table', 'ic-subtract')
rho_mean_prime = arm_mean_s(prime, 'rho-neg')
rho_mean_kob = arm_mean_s(kob, 'rho-strong')
ic_mean_prime = {a: arm_mean_s(prime, a) for a in ('ic-pair-table', 'ic-subtract')}
ic_mean_kob = {a: arm_mean_s(kob, a) for a in ('ic-pair-table', 'ic-subtract')}
ver = {'prime': {a: verified(prime, a) for a in ('ic-pair-table', 'ic-subtract', 'rho-neg', 'rho-neg-aa')},
       'koblitz': {a: verified(kob, a) for a in ('ic-pair-table', 'ic-subtract', 'rho-strong', 'rho-strong-aa')}}

rho_smaller_prime = rho_mean_prime[0] < min(v[0] for v in ic_mean_prime.values())
rho_smaller_kob = rho_mean_kob[0] < min(v[0] for v in ic_mean_kob.values())
lb_prime = {a: arm_lower_bound(prime, a) for a in ('ic-pair-table', 'ic-subtract')}
lb_kob = {a: arm_lower_bound(kob, a) for a in ('ic-pair-table', 'ic-subtract')}
unpriced_prime = {a: arm_unpriced(prime, a) for a in ('ic-pair-table', 'ic-subtract')}
unpriced_kob = {a: arm_unpriced(kob, a) for a in ('ic-pair-table', 'ic-subtract')}
lv = {'prime': levels(prime), 'koblitz': levels(kob)}

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

docs = [R + '/REPORT.md', R + '/PROTOCOL.md', R + '/VALIDATION.md']
offrepo_refs, aa_lines, isolation_lines = [], [], []
for p in docs:
    offrepo_refs += grep_lines(p, r'drive\.google|/Volumes|SSD|/Users|s3://|gs://|store:|Modal volume|not committed|scratch')
    aa_lines += grep_lines(p, r'A/A|null-control|null control|negative control|positive control')
    isolation_lines += grep_lines(p, r'isolat|receipt|shared|L0|macOS|logical CPUs')

def git_add(path):
    r = subprocess.run(['git', '-C', ROOT, 'log', '--diff-filter=A', '--format=%H %cI', '--', path], capture_output=True, text=True)
    return r.stdout.strip() or 'empty'

claims = [
    {'claimed': '24 verified measured runs per arm',
     'computed': 'prime20 verified/measured per arm %s ; koblitz17 %s' % (ver['prime'], ver['koblitz']),
     'definition': 'records with warmup false and outcome.status verified, per arm, per session',
     'files': [R + '/sessions/prime20/records.jsonl', R + '/sessions/koblitz17/records.jsonl'],
     'status': 'match' if all(v[0] == 24 and v[1] == 24 for d in ver.values() for v in d.values()) else 'mismatch'},
    {'claimed': 'pair-table/subtraction counted-S quotient 0.018861 on the prime curve',
     'computed': 'sum S ic-pair-table / sum S ic-subtract over %d paired (workload, round) records = %.6f' % (pairs_prime, q_prime),
     'definition': 'paired totals ratio, pair table over subtraction, measured records only; the study computes the same mean-S ratio (summary-prime20.json mean_s 28.7531 / 1524.5108)',
     'files': [R + '/sessions/prime20/records.jsonl'],
     'status': 'match' if ('%.6f' % q_prime) == '0.018861' else 'mismatch'},
    {'claimed': '95% cluster interval 0.018547–0.019242 (prime)',
     'computed': 'cluster bootstrap resampling rule and seed are in the study comparisons/ JSON, not reproduced here; no interval computed',
     'definition': 'interval from the study cluster bootstrap; the RNG is not stated in the committed text',
     'files': [R + '/sessions/prime20/comparisons'],
     'status': 'cannot_define'},
    {'claimed': '0.057047 on the Koblitz curve, fully priced in ecbench.gae',
     'computed': 'sum S ic-pair-table / sum S ic-subtract over %d paired records = %.6f' % (pairs_kob, q_kob),
     'definition': 'as above, koblitz17 session',
     'files': [R + '/sessions/koblitz17/records.jsonl'],
     'status': 'match' if ('%.6f' % q_kob) == '0.057047' else 'mismatch'},
    {'claimed': '(0.041159–0.092069) Koblitz interval',
     'computed': 'cluster bootstrap interval not reproduced (RNG not stated); no interval computed',
     'definition': 'interval from the study cluster bootstrap',
     'files': [R + '/sessions/koblitz17/comparisons'],
     'status': 'cannot_define'},
    {'claimed': 'matched rho has smaller counted S on both curves',
     'computed': 'prime: rho-neg mean S %.4f vs ic-pair-table %.4f, ic-subtract %.4f (smaller: %s) ; Koblitz: rho-strong mean S %.4f vs ic-pair-table %.4f, ic-subtract %.4f (smaller: %s)' % (
         rho_mean_prime[0], ic_mean_prime['ic-pair-table'][0], ic_mean_prime['ic-subtract'][0], rho_smaller_prime,
         rho_mean_kob[0], ic_mean_kob['ic-pair-table'][0], ic_mean_kob['ic-subtract'][0], rho_smaller_kob),
     'definition': 'mean cost.s over measured records per arm; matched rho arm is rho-neg on prime, rho-strong on Koblitz',
     'files': [R + '/sessions/prime20/records.jsonl', R + '/sessions/koblitz17/records.jsonl'],
     'status': 'match' if (rho_smaller_prime and rho_smaller_kob) else 'mismatch'},
    {'claimed': 'prime IC counts bounded by unpriced native work',
     'computed': 'prime IC arm lower_bound values %s ; unpriced lists %s' % (lb_prime, unpriced_prime),
     'definition': 'cost.lower_bound and cost.unpriced fields of measured prime IC records',
     'files': [R + '/sessions/prime20/records.jsonl'],
     'status': 'match' if all(v == [True] for v in lb_prime.values()) else 'mismatch'},
    {'claimed': 'Koblitz quotient fully priced in ecbench.gae',
     'computed': 'Koblitz IC arm lower_bound values %s ; unpriced lists %s' % (lb_kob, unpriced_kob),
     'definition': 'cost.lower_bound and cost.unpriced fields of measured Koblitz IC records',
     'files': [R + '/sessions/koblitz17/records.jsonl'],
     'status': 'match' if all(v == [False] for v in lb_kob.values()) and all(v == [] for v in unpriced_kob.values()) else 'mismatch'},
    {'claimed': 'HOST: macOS arm64, shared, L0 wall',
     'computed': 'isolation levels in records: prime %s ; koblitz %s' % (lv['prime'], lv['koblitz']),
     'definition': 'isolation.level field of all records in both sessions; host type is in host.json (not read further here)',
     'files': [R + '/sessions/prime20/records.jsonl', R + '/sessions/koblitz17/records.jsonl'],
     'status': 'match' if (lv['prime'] == ['L0'] and lv['koblitz'] == ['L0']) else 'mismatch'},
]

identical = {
    'claimed': False,
    'n_compared': None,
    'n_equal': None,
    'n_differ': None,
    'files': [],
    'status': 'not_claimed',
}

row = {
    'row_no': 111,
    'slug': '20261012-rootfinding-decomposition',
    'claims': claims,
    'identical': identical,
    'offrepo_refs': offrepo_refs,
    'aa_lines': aa_lines,
    'isolation_lines': isolation_lines,
    'protocol_git_date': 'PROTOCOL.md: %s' % git_add(R + '/PROTOCOL.md'),
    'first_raw_git_date': 'audit-koblitz17-all.json: %s' % git_add(R + '/audit-koblitz17-all.json'),
    'script': 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/111.py',
    'notes': 'Pairs: prime %d paired records (ic-pair-table %d, ic-subtract %d); Koblitz %d paired records (ic-pair-table %d, ic-subtract %d). Quotient definition follows the study numerator and denominator (pair table over subtraction). Cluster-bootstrap intervals not recomputed.' % (pairs_prime, n_pt_p, n_st_p, pairs_kob, n_pt_k, n_st_k),
}

os.makedirs(os.path.dirname(OUT), exist_ok=True)
with open(OUT, 'a') as f:
    f.write(json.dumps(row, ensure_ascii=False) + '\n')
print('row 111 written; claims:', len(claims))
for c in claims:
    print(c['status'], '|', c['claimed'], '|', c['computed'])
print('offrepo_refs:', len(offrepo_refs), 'aa_lines:', len(aa_lines), 'isolation_lines:', len(isolation_lines))
