#!/usr/bin/env python3
# Row 99 recompute: committed raw records, replay audits and the fixed-prefix receipt only. Prints one JSON object.
import glob, json, os, random, re, subprocess
ROOT = '/home/user/crypto'
os.chdir(ROOT)
ROW = 99
SLUG = '20261014-rootfinding-lanes-coverage'
STUDY = 'research/ecbench_rootfinding_lanes_20261013'
PROTO = STUDY + '/PROTOCOL.md'
FIRST_RAW = 'benchmarks/20261014-rootfinding-lanes-coverage/copies.json'
FP = STUDY + '/fixed-prefix-fresh-result.json'

def jl(p):
    with open(p) as f:
        return [json.loads(x) for x in f if x.strip()]

def sig_eq(claimed, sig, computed):
    rounded = float(f"{computed:.{sig}g}")
    rel = abs(computed - claimed) / abs(claimed)
    return 'match' if (rounded == claimed or rel <= 0.01) else 'mismatch'

def gitdate(path):
    out = subprocess.run(['git', 'log', '--diff-filter=A', '--format=%H %cI', '--', path],
                         capture_output=True, text=True, cwd=ROOT).stdout.strip()
    return out.splitlines()[-1] if out else ''

def scan(pat, cap=30):
    hits = []; n = 0
    for dp, dn, fn in os.walk(STUDY):
        for f in sorted(fn):
            if f.endswith(('.md', '.typ', '.txt', '.tex')):
                p = os.path.join(dp, f)
                with open(p, encoding='utf-8', errors='replace') as fh:
                    for i, line in enumerate(fh, 1):
                        if re.search(pat, line, re.I):
                            n += 1
                            if len(hits) < cap:
                                hits.append(f"{p}:{i}: {line.strip()}")
    return hits, n

claims = []; notes = []
# 1. sealed executions across every session records file
ses_files = sorted(glob.glob(STUDY + '/sessions/*/records.jsonl'))
tot = ver = 0
for f in ses_files:
    recs = jl(f)
    tot += len(recs); ver += sum(1 for r in recs if r['outcome']['status'] == 'verified')
notes.append(f"session records files: {len(ses_files)}; records {tot}; verified {ver}")
claims.append(dict(claimed='1,088/1,088 verified sealed ecbench executions', computed=f"{ver}/{tot}",
                   definition='count of outcome.status == verified over all sessions/*/records.jsonl (warmups included)',
                   files=ses_files, status='match' if (ver == tot == 1088) else 'mismatch'))

# 2. replays identical across every replay-all audit
rep_files = sorted(glob.glob(STUDY + '/*replay*-all.json'))
rtot = rok = 0
per = []
for f in rep_files:
    d = json.load(open(f))
    reps = d.get('replays', [])
    ok = sum(1 for x in reps if x.get('reproduced') and x.get('detail') == 'identical')
    rtot += len(reps); rok += ok
    per.append(f"{os.path.basename(f)}:{ok}/{len(reps)}")
notes.append('replay files: ' + ', '.join(per))
claims.append(dict(claimed='816/816 identical measured replays', computed=f"{rok}/{rtot}",
                   definition='count of replays[] with reproduced true and detail identical over *replay*-all.json',
                   files=rep_files, status='match' if (rok == rtot == 816) else 'mismatch'))

# 3. exact serial/batch lane pairs (lanes-local session)
L = jl(STUDY + '/sessions/lanes-local-20261013/records.jsonl')
L = [r for r in L if not r['warmup']]
pairs = 0; eq = 0
for k in (4, 8, 16):
    ser = {(r['workload']['workload_id'], r['round']): r for r in L if r['arm'] == f'b48-serial-{k}'}
    bat = {(r['workload']['workload_id'], r['round']): r for r in L if r['arm'] == f'b48-batch-{k}'}
    for key in set(ser) & set(bat):
        pairs += 1
        a, b = ser[key], bat[key]
        if a['outcome']['recovered'] == b['outcome']['recovered'] and a['cost']['total_gae'] == b['cost']['total_gae'] and a['counters'] == b['counters']:
            eq += 1
claims.append(dict(claimed='72/72 exact lane pairs at equal counted GAE', computed=f"{eq}/{pairs}",
                   definition='serial-k vs batch-k pairs (k=4,8,16), measured, matched on workload_id and round; equal recovered, cost.total_gae and counters',
                   files=[STUDY + '/sessions/lanes-local-20261013/records.jsonl'],
                   status='match' if (eq == pairs == 72) else 'mismatch'))

# 4. B56/B48 ratio, retune holdout (ratio of mean cost.s over measured runs)
H = [r for r in jl(STUDY + '/sessions/retune-holdout-local-20261013/records.jsonl') if not r['warmup']]
def mean_s(arm, extra=None):
    xs = [r['cost']['s'] for r in H if r['arm'] == arm]
    return sum(xs) / len(xs), len(xs)
m56, n56 = mean_s('b56-batch-8'); m48, n48 = mean_s('b48-batch-8')
ratio = m56 / m48
HF = STUDY + '/sessions/retune-holdout-local-20261013/records.jsonl'
claims.append(dict(claimed='B56/B48 0.979', computed=f"{ratio:.6f} (mean cost.s b56 {m56:.6f} n={n56} / b48 {m48:.6f} n={n48})",
                   definition='ratio of arithmetic means of cost.s, b56-batch-8 over b48-batch-8, measured runs of retune-holdout',
                   files=[HF], status=sig_eq(0.979, 3, ratio)))
claims.append(dict(claimed='B56/B48 interval 0.919–1.038', computed='not computed',
                   definition='target-cluster interval: resampling seed and count are not stated in the committed records; the comparison file is a derived study product',
                   files=[HF], status='cannot_define'))

# 5./6./8./9./10. fixed-prefix receipt (fresh targets, 8 targets, 4096 and 8192 probes)
R = json.load(open(FP))['result']
T = R['targets']
def arm(t, s):
    return next(v for v in t['arms'] if v['stride'] == s)
def ms(t, s, l):
    return next(v for v in arm(t, s)['milestones'] if v['probes'] == l)
def mean_rank(s, l):
    return sum(ms(t, s, l)['independent_rank'] for t in T) / len(T)
def rpg(s, l):
    rank = sum(ms(t, s, l)['independent_rank'] for t in T)
    cost = sum(ms(t, s, l)['cold_gae_lower_bound'] for t in T)
    return (rank / cost)
d_point = mean_rank(23, 4096) - mean_rank(1, 4096)
r_point = rpg(23, 4096) / rpg(1, 4096)
# replicate the study's paired target-cluster bootstrap (analyze_fixed_prefix.py); seed as recorded in fixed-prefix-fresh.json
SEED = json.load(open(STUDY + '/fixed-prefix-fresh.json'))['bootstrap_seed']
rng = random.Random(SEED)
store = {}
for s in (7, 23):
    for l in (4096, 8192):
        diffs = []; ratios = []
        for _ in range(20000):
            draw = rng.choices(T, k=len(T))
            base = [ms(t, 1, l) for t in draw]
            cand = [ms(t, s, l) for t in draw]
            br = sum(v['independent_rank'] for v in base); cr = sum(v['independent_rank'] for v in cand)
            diffs.append((cr - br) / len(draw))
            bc = sum(v['cold_gae_lower_bound'] for v in base); cc = sum(v['cold_gae_lower_bound'] for v in cand)
            ratios.append((cr / cc) / (br / bc))
        store[(s, l)] = (diffs, ratios)
def interval(vals):
    o = sorted(vals)
    return [o[499], o[19499]]
di = interval(store[(23, 4096)][0]); ri = interval(store[(23, 4096)][1])
claims.append(dict(claimed='2.750 rows per target (stride 23 minus stride 1, 4,096 probes)', computed=f"{d_point:.6f}",
                   definition='mean over 8 targets of independent_rank, stride 23 minus stride 1, milestone probes 4096, fixed-prefix-fresh-result.json',
                   files=[FP], status=sig_eq(2.750, 4, d_point)))
claims.append(dict(claimed='interval 1.000–4.625 (rows per target)', computed=f"[{di[0]:.6f}, {di[1]:.6f}] (paired bootstrap, 20000 resamples, seed {SEED})",
                   definition='analyze_fixed_prefix.py percentile rule: sorted index 499 and 19499; rng random.Random(seed) consumed in study loop order',
                   files=[FP, STUDY + '/fixed-prefix-fresh.json'],
                   status=('match' if abs(di[0] - 1.000) <= 0.0005 and abs(di[1] - 4.625) <= 0.0005 else 'mismatch')))
claims.append(dict(claimed='1.095 rank per lower-bound cold GAE ratio', computed=f"{r_point:.6f}",
                   definition='(stride 23 rank / cold GAE lower bound) over (stride 1 rank / cold GAE lower bound), 8 targets, 4,096 probes',
                   files=[FP], status=sig_eq(1.095, 4, r_point)))
claims.append(dict(claimed='interval 1.032–1.176 (rank-per-GAE ratio)', computed=f"[{ri[0]:.6f}, {ri[1]:.6f}] (paired bootstrap, seed {SEED})",
                   definition='same bootstrap as above, ratio statistic',
                   files=[FP, STUDY + '/fixed-prefix-fresh.json'],
                   status=('match' if abs(ri[0] - 1.032) <= 0.0005 and abs(ri[1] - 1.176) <= 0.0005 else 'mismatch')))
claims.append(dict(claimed='7.463 times rho cold GAE (batch-8, REPORT)', computed='not computed in this recompute',
                   definition='listed in REPORT, not in the headline; not reached within the 10-claim cap', files=[HF], status='cannot_define'))

off, off_n = scan(r'drive\.google\.com|/Volumes|SSD|/Users|s3://|gs://|store:[A-Za-z0-9_\-]+|Modal volume|not committed|scratch')
aa, aa_n = scan(r'A/A|A-A|null control|null-control|aa control|rho-neg-aa|rho-aa')
iso, iso_n = scan(r'isolat|reserved core|\bL[012]\b|NUMA|pinned|shared, L0|wall L0')
notes.append(f"off-repo hits total {off_n}; AA hits total {aa_n}; isolation hits total {iso_n} (each shown up to 30); "
             f"store/index.json present: {os.path.exists(ROOT + '/store/index.json')}")
out = {
    'row_no': ROW, 'slug': SLUG, 'claims': claims,
    'identical': {'claimed': True, 'n_compared': rtot, 'n_equal': rok, 'n_differ': rtot - rok,
                  'files': rep_files, 'status': 'match' if rtot and rok == rtot else 'mismatch'},
    'offrepo_refs': off, 'aa_lines': aa, 'isolation_lines': iso,
    'protocol_git_date': gitdate(PROTO), 'first_raw_git_date': gitdate(FIRST_RAW),
    'script': 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/99.py',
    'notes': ' | '.join(notes),
}
print(json.dumps(out, ensure_ascii=False))
