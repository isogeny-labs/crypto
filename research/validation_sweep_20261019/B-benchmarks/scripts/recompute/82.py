import json, statistics, pathlib, re, subprocess, math
from collections import Counter
REPO = pathlib.Path('/home/user/crypto')
WB = REPO / 'research/validation_sweep_20261019/B-benchmarks'
OUT = WB / 'recompute/SWB-RECOMPUTE-11.jsonl'
ROW = 82
SLUG = '20261014-frobenius-orbit-ic-vs-rho'
STUDY = REPO / 'research/ecbench_frobenius_orbit_vs_rho_20261014'
SCRIPT = WB / 'scripts/recompute/82.py'
PROTOCOL = STUDY / 'PROTOCOL.md'
FIRST_RAW = STUDY / 'sessions/a0-n13/records.jsonl'
SESS = STUDY / 'sessions'

def recs(session):
    p = SESS / session / 'records.jsonl'
    return [json.loads(l) for l in p.read_text().splitlines() if l.strip()]

def ratio_pair(session, A, B):
    """B/A over pairs matched by (workload_id, round), non-warmup records, total_gae."""
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
    rom = statistics.fmean(b[k] for k in keys) / statistics.fmean(a[k] for k in keys)
    mpr = statistics.fmean(b[k] / a[k] for k in keys)
    return len(keys), rom, mpr

def ndp(s):
    return len(s.split('.')[1]) if '.' in s else 0

def ok(cl, s, c):
    if cl == 0:
        return c == 0
    return round(c, ndp(s)) == cl or abs(c - cl) / abs(cl) <= 0.01

claims = []
def claim(claimed, computed, definition, files, pairs, interval_undefined=False):
    if not pairs or any(c is None for _, _, c in pairs):
        st = 'cannot_define'
    else:
        oks = [ok(cl, s, c) for s, cl, c in pairs]
        if all(oks):
            st = 'partial' if interval_undefined else 'match'
        else:
            st = 'mismatch' if not any(oks) else 'partial'
    claims.append({'claimed': claimed, 'computed': computed, 'definition': definition,
                   'files': [str(f.relative_to(REPO)) for f in files], 'status': st})

def arms_of(sess):
    names = set(r['arm'] for r in recs(sess))
    orb = sorted(x for x in names if re.match(r'ic-orbit-c\d+-renew$', x))
    neg = sorted(x for x in names if re.match(r'ic-negfold-d\d+-renew$', x))
    return (orb[0] if orb else None), (neg[0] if neg else None)

def cmp_claim(text, sess, A, B, claimed_val, claimed_str, label_iv):
    orb, neg = arms_of(sess)
    role = {'rho-frob': 'rho-frob', 'rho-strong': 'rho-strong', 'orbit': orb, 'negfold': neg}
    A, B = role[A], role[B]
    fname = SESS / sess / 'comparisons' / ('%s__%s.json' % (A, B))
    res = ratio_pair(sess, A, B) if A and B else None
    if res is None:
        comp = 'no matched pairs'; val = None
    else:
        n, rom, mpr = res
        comp = 'ratio of means B/A = %.4f over n_pairs=%d; mean of per-pair B/A = %.4f (study comparison file present: %s)' % (rom, n, mpr, fname.exists())
        val = rom
    claim(text, comp, 'paired B/A by (workload_id, round), non-warmup, cost.total_gae; A=%s B=%s session=%s; interval %s not reproducible (cluster bootstrap seed and resample draw not stated)' % (A, B, sess, label_iv),
          [fname] if fname.exists() else [SESS / sess / 'records.jsonl'], [(claimed_str, claimed_val, val)], interval_undefined=True)

cmp_claim('orbit / rho-frob 2.96 [2.55, 3.24] (n=13)', 'a0-n13', 'rho-frob', 'orbit', 2.96, '2.96', 'of 2.55-3.24')
cmp_claim('orbit / rho-frob 18.2 [15.7, 21.6] (n=41)', 'a0-n41', 'rho-frob', 'orbit', 18.2, '18.2', 'of 15.7-21.6')
cmp_claim('orbit / rho-strong 0.41 at n=13', 'a0-n13', 'rho-strong', 'orbit', 0.41, '0.41', 'not printed')
cmp_claim('orbit / rho-strong 15.3 at n=41', 'a0-n41', 'rho-strong', 'orbit', 15.3, '15.3', 'not printed')
cmp_claim('orbit / negfold 0.51 at n=13', 'a0-n13', 'negfold', 'orbit', 0.51, '0.51', 'not printed')
cmp_claim('orbit / negfold 0.065 at n=23', 'a0-n23', 'negfold', 'orbit', 0.065, '0.065', 'not printed')

# trend slope: ecbench trend is a weighted least squares fit with ln r and ln n; the weights and the SE inflation rule are not stated in the study text
claim('trend slope 0.165 [0.117, 0.213] per ln r', 'not computed: the weighted least-squares weights for ecbench trend are not stated in PROTOCOL.md or README.md, so the slope cannot be re-derived from records',
      'WLS of ln(paired ratio) on ln r and ln n over eight comparisons (study trend/ file)', [STUDY / 'trend/trend-orbit-vs-rho-frob.json'], [], interval_undefined=True)

# verified / replay / lower-bound counts from all committed session records
allr = []
for s in sorted(p.name for p in SESS.iterdir() if (p / 'records.jsonl').exists()):
    allr += recs(s)
n_all = len(allr)
n_ver = sum(1 for r in allr if r['outcome']['status'] == 'verified')
claim('2,816 of 2,816 records verified', '%d of %d records outcome.status == verified' % (n_ver, n_all),
      'count over all sessions/*/records.jsonl including warm-up records', [SESS], [('2816', 2816, n_all), ('2816v', 2816, n_ver)])
n_lb = sum(1 for r in allr if r['cost'].get('lower_bound') is True)
claim('no lower bounds', '%d records with cost.lower_bound == true out of %d' % (n_lb, n_all),
      'count of cost.lower_bound == true over all records', [SESS], [('0', 0, n_lb)])

# replays: the eight session audits (audit-a*.json) hold per-replay entries; reaudit-* repeats a0-n13 and is not in the 2,112 count
SESS_AUDITS = sorted(f for f in (STUDY / 'audits').glob('audit-*.json'))
rp_all = []
for f in SESS_AUDITS:
    d = json.loads(f.read_text())
    rp_all += d.get('replays', [])
n_rep = len(rp_all)
n_eq = sum(1 for x in rp_all if x.get('reproduced') is True and x.get('detail') == 'identical')
reaud = json.loads((STUDY / 'audits/reaudit-a0-n13-formatted-binary.json').read_text())
claim('2,112 of 2,112 replays identical (eight session audits)', '%d replay entries, %d reproduced=true with detail identical, %d other' % (n_rep, n_eq, n_rep - n_eq),
      'count of replays[] entries over audits/audit-*.json (8 files); reproduced == true and detail == "identical"',
      SESS_AUDITS, [('2112', 2112, n_rep), ('2112e', 2112, n_eq)])
identical = {'claimed': True, 'n_compared': n_rep, 'n_equal': n_eq, 'n_differ': n_rep - n_eq,
             'files': [str(f.relative_to(REPO)) for f in SESS_AUDITS],
             'status': 'match' if (n_rep == 2112 and n_eq == n_rep) else 'mismatch'}

OFF = re.compile(r'drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:[A-Za-z0-9_.\-]+|Modal volume|not committed|scratch', re.I)
AA = re.compile(r'A/A|A-A|null[- ]control|null control|aa_', re.I)
ISO = re.compile(r'isolat|\bL[0-3]\b|NUMA', re.I)
store_index = REPO / 'store/index.json'
store_txt = store_index.read_text() if store_index.exists() else None
offrepo, aa, iso = [], [], []
study_files = sorted(p for p in STUDY.rglob('*') if p.is_file() and p.suffix in ('.md', '.typ', '.txt', '.tex') and 'raw' not in p.relative_to(STUDY).parts and p.stat().st_size < 1_000_000)
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
notes = ('Sessions and comparisons read from sessions/*/comparisons and records.jsonl. A/A arm in the study is rho-frob-aa (not computed here). '
         'reaudit-a0-n13-formatted-binary.json holds %d replay entries (not in the 2,112 count). Off-repo, A/A and isolation lines are whole lines from study .md/.typ/.txt/.tex files, not split sentences. '
         'Protocol git add date: %s. First raw git add date: %s.') % (len(reaud.get('replays', [])), pd or 'EMPTY', fd or 'EMPTY')
row = {'row_no': ROW, 'slug': SLUG, 'claims': claims, 'identical': identical, 'offrepo_refs': offrepo,
       'aa_lines': aa, 'isolation_lines': iso, 'protocol_git_date': pd, 'first_raw_git_date': fd,
       'script': str(SCRIPT.relative_to(REPO)), 'notes': notes}
lines = [l for l in OUT.read_text().splitlines() if l.strip()] if OUT.exists() else []
lines = [l for l in lines if json.loads(l)['row_no'] != ROW] + [json.dumps(row)]
OUT.write_text('\n'.join(lines) + '\n')
print(json.dumps(row, indent=1)[:9000])
print('STATUS', Counter(c['status'] for c in claims), 'records', n_all, 'verified', n_ver, 'lower_bound', n_lb)
