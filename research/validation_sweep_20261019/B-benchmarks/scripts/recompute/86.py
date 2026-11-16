import json, statistics, pathlib, re, subprocess, math
from collections import Counter
REPO = pathlib.Path('/home/user/crypto')
WB = REPO / 'research/validation_sweep_20261019/B-benchmarks'
OUT = WB / 'recompute/SWB-RECOMPUTE-11.jsonl'
ROW = 86
SLUG = '20261014-rootfind-stride-targets'
STUDY = REPO / 'research/ecbench_rootfind_speedup_20261014'
SESS = STUDY / 'sessions'
SCRIPT = WB / 'scripts/recompute/86.py'
PROTOCOL = STUDY / 'PROTOCOL.md'
FIRST_RAW = STUDY / 'logs/run-amendment1.log'
claims = []

def recs(session):
    return [json.loads(l) for l in (SESS / session / 'records.jsonl').read_text().splitlines() if l.strip()]

def ratio_pair(session, A, B):
    rs = recs(session)
    a, b = {}, {}
    for r in rs:
        if r.get('warmup'):
            continue
        key = (r['workload']['workload_id'], r['round'])
        if r['arm'] == A:
            a[key] = r['cost']['total_gae']
        elif r['arm'] == B:
            b[key] = r['cost']['total_gae']
    keys = sorted(k for k in a if k in b)
    if not keys:
        return None
    return len(keys), statistics.fmean(b[k] for k in keys) / statistics.fmean(a[k] for k in keys), statistics.fmean(b[k] / a[k] for k in keys)

def ok(cl, s, c):
    dd = len(s.split('.')[1]) if '.' in s else 0
    return round(c, dd) == cl or abs(c - cl) / abs(cl) <= 0.01

def claim(text, computed, definition, files, pairs, interval_undefined=False, status=None):
    if status is None:
        if not pairs or any(c is None for _, _, c in pairs):
            status = 'cannot_define'
        else:
            oks = [ok(cl, s, c) for s, cl, c in pairs]
            status = ('partial' if interval_undefined else 'match') if all(oks) else ('mismatch' if not any(oks) else 'partial')
    claims.append({'claimed': text, 'computed': computed, 'definition': definition,
                   'files': [str(f.relative_to(REPO)) for f in files], 'status': status})

def ratio_claim(text, sess, A, B, val, sval):
    res = ratio_pair(sess, A, B)
    comp = 'no matched pairs' if res is None else 'ratio of means B/A = %.4f over n_pairs=%d (B=%s, A=%s); mean of per-pair B/A = %.4f' % (res[1], res[0], B, A, res[2])
    cf = SESS / sess / 'comparisons' / ('%s__%s.json' % (A, B))
    claim(text, comp, 'paired B/A by (workload_id, round), non-warmup, cost.total_gae, session %s; interval not reproduced (cluster bootstrap seed not stated)' % sess,
          [cf if cf.exists() else SESS / sess / 'records.jsonl'], [(sval, val, None if res is None else res[1])], interval_undefined=True)

ratio_claim('renew / walk 0.531 [0.506, 0.557] (n = 13, d = 5)', 'claim-renew-n13', 'ic-negfold-m2-d5-walk', 'ic-negfold-m2-d5-renew', 0.531, '0.531')
ratio_claim('renew / walk 0.835 [0.741, 0.943] (n = 19, d = 6)', 'claim-renew-n19', 'ic-negfold-m2-d6-walk', 'ic-negfold-m2-d6-renew', 0.835, '0.835')
ratio_claim('d = 4 renew / rho-strong 0.770 [0.720, 0.824] (n = 13)', 'claim-renew-n13', 'rho-strong', 'ic-negfold-m2-d4-renew', 0.770, '0.770')
ratio_claim('index calculus / rho-strong 4.7x at n = 19 (d6 renew)', 'claim-renew-n19', 'rho-strong', 'ic-negfold-m2-d6-renew', 4.7, '4.7')

# rho-frob S = mean(GAE / sqrt(r)) over non-warm-up records in claim-renew-n13
rf = [r for r in recs('claim-renew-n13') if r['arm'] == 'rho-frob' and not r.get('warmup')]
rr = rf[0]['workload']['curve']['r']
S_rf = statistics.fmean(r['cost']['total_gae'] / math.sqrt(r['workload']['curve']['r']) for r in rf)
ic_arms = sorted(set(r['arm'] for r in recs('claim-renew-n13') if r['arm'].startswith('ic-')))
S_ic = {a: statistics.fmean(r['cost']['total_gae'] / math.sqrt(r['workload']['curve']['r']) for r in recs('claim-renew-n13') if r['arm'] == a and not r.get('warmup')) for a in ic_arms}
claim('rho-frob S about 1.9 on the n = 13 targets', 'mean S = mean(total_gae / sqrt(r)) = %.4f over n=%d records (r = %s)' % (S_rf, len(rf), rr),
      'mean of cost.total_gae / sqrt(workload.curve.r) over non-warm-up rho-frob records in claim-renew-n13', [SESS / 'claim-renew-n13/records.jsonl'], [('1.9', 1.9, S_rf)])
ratio_min = min(S_ic.values()) / S_rf
claim('rho-frob 6x below every index-calculus arm at n = 13', 'min IC mean S / rho-frob S = %.3f; per arm S: %s' % (ratio_min, {k: round(v, 3) for k, v in S_ic.items()}),
      'ratio of the smallest IC arm mean S to rho-frob mean S, session claim-renew-n13', [SESS / 'claim-renew-n13/records.jsonl'], [('6', 6.0, ratio_min)])

# verified and replayed counts
allr = []
for s in sorted(p.name for p in SESS.iterdir() if (p / 'records.jsonl').exists()):
    allr += recs(s)
n_all = len(allr); n_ver = sum(1 for r in allr if r['outcome']['status'] == 'verified')
claim('2,064 of 2,064 records verified', '%d of %d records outcome.status == verified' % (n_ver, n_all),
      'count over sessions/*/records.jsonl including warm-up records', [SESS], [('2064', 2064, n_all), ('2064v', 2064, n_ver)])

# replays from audits/*.json (if the audit files list replays)
AUD = STUDY / 'audits'
aud_files = sorted(AUD.glob('audit-*.json')) if AUD.exists() else []
re_files = sorted(AUD.glob('reaudit-*.json')) if AUD.exists() else []
re_rp = sum(len(json.loads(f.read_text()).get('replays', [])) for f in re_files)
n_warm = sum(1 for r in allr if r.get('warmup'))
rp = []
keys = set()
for f in aud_files:
    dd = json.loads(f.read_text())
    keys |= set(dd.keys())
    rp += dd.get('replays', []) if isinstance(dd.get('replays'), list) else []
n_rp = len(rp); n_rp_eq = sum(1 for x in rp if x.get('reproduced') is True)
if n_rp:
    claim('2,064 of 2,064 records replayed', '%d replay entries in audit-*.json (measured records only; warm-up records %d not replayed); %d reproduced true; reaudit-*.json adds %d entries (not counted)' % (n_rp, n_warm, n_rp_eq, re_rp),
          'count of audits/*.json replays[] entries and reproduced == true', aud_files, [('2064', 2064, n_rp_eq)])
else:
    claim('2,064 of 2,064 records replayed', 'audits/*.json (%d files) contain no replays list; top-level keys: %s' % (len(aud_files), sorted(keys)[:30]),
          'search for per-replay entries in audits/*.json', aud_files, [], status='no_raw_in_repo')

identical = {'claimed': False, 'n_compared': None, 'n_equal': None, 'n_differ': None, 'files': [], 'status': 'not_claimed'}

OFF = re.compile(r'drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:[A-Za-z0-9_.\-]+|Modal volume|not committed|scratch', re.I)
AA = re.compile(r'A/A|A-A|null[- ]control|null control', re.I)
ISO = re.compile(r'isolat|\bL[0-3]\b|NUMA', re.I)
store_index = REPO / 'store/index.json'
store_txt = store_index.read_text() if store_index.exists() else None
offrepo, aa_l, iso = [], [], []
files = sorted(p for p in STUDY.rglob('*') if p.is_file() and p.suffix in ('.md', '.typ', '.txt', '.tex') and 'raw' not in p.relative_to(STUDY).parts and 'figures' not in p.relative_to(STUDY).parts and p.stat().st_size < 1_000_000)
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
notes = 'The first RAW CANDIDATE listed by the card (run-amendment1.log) is a log, not per-run records; per-run records are sessions/*/records.jsonl and the claim files are the comparison JSONs. Protocol git add date: %s. First raw git add date: %s.' % (pd or 'EMPTY', fd or 'EMPTY')
row = {'row_no': ROW, 'slug': SLUG, 'claims': claims, 'identical': identical, 'offrepo_refs': offrepo,
       'aa_lines': aa_l, 'isolation_lines': iso, 'protocol_git_date': pd, 'first_raw_git_date': fd,
       'script': str(SCRIPT.relative_to(REPO)), 'notes': notes}
lines = [l for l in OUT.read_text().splitlines() if l.strip()] if OUT.exists() else []
lines = [l for l in lines if json.loads(l)['row_no'] != ROW] + [json.dumps(row)]
OUT.write_text('\n'.join(lines) + '\n')
for c in claims: print(c['status'], '|', c['claimed'], '|', c['computed'][:170])
print('STATUS', Counter(c['status'] for c in claims), 'offrepo', len(offrepo), 'aa', len(aa_l), 'iso', len(iso))
