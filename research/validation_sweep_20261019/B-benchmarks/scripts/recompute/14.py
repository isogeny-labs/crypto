#!/usr/bin/env python3
# Row 14 recompute (20261017-generic-dlp-memory-parallelism). Reads committed files only.
import json, re, subprocess, statistics, collections

R = '/home/user/crypto'
SCRIPT = 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/14.py'
D = 'research/generic_dlp_memory_parallelism_20261017/'
RUN = D + 'reports/run-20261017T0201Z/'
RAW = RUN + 'raw/'
PROTO = D + 'PROTOCOL.md'
README = D + 'README.md'


def gd(path):
    out = subprocess.run(['git', '-C', R, 'log', '--diff-filter=A', '--format=%H %cI', '--', path],
                         capture_output=True, text=True).stdout.strip()
    return out.replace('\n', ';')


def line(path, n):
    with open(R + '/' + path, encoding='utf-8') as f:
        for i, t in enumerate(f, 1):
            if i == n:
                return f'{path}:{n}: {t.rstrip()}'
    return f'{path}:{n}: <missing>'


def jl(name):
    return [json.loads(l) for l in open(R + '/' + RAW + name, encoding='utf-8')]


bsgs1 = jl('bsgs1.jsonl')
bsgs3 = jl('bsgs3.jsonl')
rho1 = jl('rho1.jsonl')
rho3 = jl('rho3.jsonl')
opc = jl('opcount.jsonl')
pari = jl('pari.jsonl')
iso = jl('isolation.jsonl')

contended = {r['label']: bool(r['run']['contended']) for r in iso}
n_nonpari = [r for r in iso if not r['label'].startswith('pari-')]
n_contended_nonpari = sum(1 for r in n_nonpari if r['run']['contended'])

RE_B = re.compile(r'^bsgs(\d)-(\d+)-(chain|paired)-(A2|A|B)-r(\d+)$')
RE_R = re.compile(r'^rho(\d)-(\d+)-(A2|A|B)-r(\d+)$')


def index_bsgs(rows):
    idx = {}
    for r in rows:
        m = RE_B.match(r['label'])
        t, bits, kern, arm, rnd = int(m.group(1)), int(m.group(2)), m.group(3), m.group(4), int(m.group(5))
        idx[(t, kern, bits, arm, rnd)] = r
    return idx


def index_rho(rows):
    idx = {}
    for r in rows:
        m = RE_R.match(r['label'])
        t, bits, arm, rnd = int(m.group(1)), int(m.group(2)), m.group(3), int(m.group(4))
        idx[(t, bits, arm, rnd)] = r
    return idx


bi = {}
bi.update(index_bsgs(bsgs1))
bi.update(index_bsgs(bsgs3))
ri = {}
ri.update(index_rho(rho1))
ri.update(index_rho(rho3))


def ratio_stats(idx, keyfix, metric, numer_arm='A', denom_arm='B'):
    """keyfix(key) -> (threads, config..., rounds). Returns per-config dict."""
    configs = sorted({k[:-2] for k in idx})  # (threads, kern?, bits)
    out = {}
    for cfg in configs:
        rounds = sorted({k[-1] for k in idx if k[:-2] == cfg and k[-2] == numer_arm})
        pa, pb, a_vals, b_vals = [], [], [], []
        for rnd in rounds:
            ka = cfg + (numer_arm, rnd)
            kb = cfg + (denom_arm, rnd)
            ra, rb = idx.get(ka), idx.get(kb)
            if ra is None or rb is None:
                continue
            la = ra['label']
            lb = rb['label']
            if contended.get(la) or contended.get(lb):
                continue
            a = float(ra[metric]); b = float(rb[metric])
            a_vals.append(a); b_vals.append(b); pa.append(a / b)
        if not a_vals:
            continue
        out[cfg] = {
            'n_rounds': len(a_vals),
            'median_A': statistics.median(a_vals),
            'median_B': statistics.median(b_vals),
            'ratio_of_medians_AB': statistics.median(a_vals) / statistics.median(b_vals),
            'per_round_min': min(pa), 'per_round_max': max(pa),
        }
    return out


bs_ab = ratio_stats(bi, None, 'total_s')           # keys (threads, kern, bits)
rh_ab = ratio_stats(ri, None, 'solve_s')           # keys (threads, bits)
rh_su = ratio_stats(ri, None, 'setup_s')           # start-up time, keys (threads, bits)

chain1 = {k[2]: v for k, v in bs_ab.items() if k[0] == 1 and k[1] == 'chain'}
paired1 = {k[2]: v for k, v in bs_ab.items() if k[0] == 1 and k[1] == 'paired'}
rho1_ab = {k[1]: v for k, v in rh_ab.items() if k[0] == 1}
rho1_su = {k[1]: v for k, v in rh_su.items() if k[0] == 1}


def rng(values):
    return (min(values), max(values))


def close(a, b, tol=0.01):
    return abs(a - b) / abs(b) <= tol


def fmt(x):
    return f'{x:.3f}'


# PARI/GP elllog (seconds), median per bits
pari_bsgs = collections.defaultdict(list)
for r in pari:
    if r['role'] == 'bsgs' and not contended.get(r['label'], False):
        pari_bsgs[r['bits']].append(r['elllog_ms'] / 1000.0)
pari_med = {b: statistics.median(v) for b, v in pari_bsgs.items()}

cand_pari, base_pari = [], []
for kern in ('chain', 'paired'):
    for bits, pm in sorted(pari_med.items()):
        ks = [k for k in bs_ab if k[0] == 1 and k[1] == kern and k[2] == bits]
        if not ks:
            continue
        v = bs_ab[ks[0]]
        cand_pari.append(v['median_B'] / pm)
        base_pari.append(v['median_A'] / pm)

chain_vals = [chain1[b]['ratio_of_medians_AB'] for b in (44, 48, 52) if b in chain1]
paired_vals = [paired1[b]['ratio_of_medians_AB'] for b in (44, 48, 52) if b in paired1]
rho_bits = [32, 36, 40, 44, 48, 52]
su_vals = [rho1_su[b]['ratio_of_medians_AB'] for b in rho_bits if b in rho1_su]

claims = []


def add(claimed, computed_str, definition, files, ok):
    claims.append({'claimed': claimed, 'computed': computed_str, 'definition': definition,
                   'files': files, 'status': 'match' if ok else 'mismatch'})


lo, hi = rng(chain_vals)
add('BSGS chain total_s baseline/candidate 1.30–1.44× at 44–52 bits (1 thread), low end',
    f'min ratio_of_medians_AB over 44,48,52 bits={fmt(lo)}', 'median(A total_s)/median(B total_s) over rounds, contended runs excluded, bsgs1 chain',
    [RUN + 'raw/bsgs1.jsonl', RUN + 'raw/isolation.jsonl'], close(lo, 1.30))
add('BSGS chain total_s baseline/candidate 1.30–1.44× at 44–52 bits (1 thread), high end',
    f'max ratio_of_medians_AB over 44,48,52 bits={fmt(hi)}', 'as above', [RUN + 'raw/bsgs1.jsonl'], close(hi, 1.44))
lo, hi = rng(paired_vals)
add('BSGS paired total_s baseline/candidate 1.74–1.98× at 44–52 bits (1 thread), low end',
    f'min ratio_of_medians_AB over 44,48,52 bits={fmt(lo)}', 'median(A total_s)/median(B total_s), bsgs1 paired kernel',
    [RUN + 'raw/bsgs1.jsonl'], close(lo, 1.74))
add('BSGS paired total_s baseline/candidate 1.74–1.98× at 44–52 bits (1 thread), high end',
    f'max ratio_of_medians_AB over 44,48,52 bits={fmt(hi)}', 'as above', [RUN + 'raw/bsgs1.jsonl'], close(hi, 1.98))
for bits, claimed in ((32, 5.84), (36, 3.75), (40, 1.82), (48, 1.14)):
    v = rho1_ab.get(bits, {}).get('ratio_of_medians_AB')
    add(f'rho solve_s {claimed}× at {bits} bits (1 thread)',
        f'median(A solve_s)/median(B solve_s)={fmt(v) if v is not None else None}',
        'rho1 solve_s, contended runs excluded, median over rounds', [RUN + 'raw/rho1.jsonl'],
        v is not None and close(v, claimed))
lo, hi = rng(su_vals)
add('start-up alone 6.8–16.1×',
    f'min={fmt(lo)} max={fmt(hi)} of median(A setup_s)/median(B setup_s) over rho1 bits 32–52',
    'rho1 setup_s (start-up time field), ratio of medians, per bits; range over bits',
    [RUN + 'raw/rho1.jsonl'], close(lo, 6.8) and close(hi, 16.1))
clo, chi = rng(cand_pari)
add('vs PARI/GP elllog candidate/PARI 0.020–0.167',
    f'min={clo:.4f} max={chi:.4f} of median(B total_s)/median(PARI elllog_ms/1000) over chain,paired x bits 40–52',
    'per kernel and bits: median candidate total_s over rounds divided by median PARI elllog seconds over PARI rounds',
    [RUN + 'raw/pari.jsonl', RUN + 'raw/bsgs1.jsonl'], close(clo, 0.020, 0.01) and close(chi, 0.167, 0.01))
blo, bhi = rng(base_pari)
add('vs PARI/GP elllog baseline/PARI 0.022–0.974',
    f'min={blo:.4f} max={bhi:.4f} of median(A total_s)/median(PARI elllog seconds) over chain,paired x bits 40–52',
    'as above with baseline arm A', [RUN + 'raw/pari.jsonl', RUN + 'raw/bsgs1.jsonl'],
    close(blo, 0.022, 0.01) and close(bhi, 0.974, 0.01))
add('26/438 runs contended, left out',
    f'contended among non-PARI isolation runs={n_contended_nonpari} of {len(n_nonpari)}',
    'run.contended == true in isolation.jsonl over non-PARI labels', [RUN + 'raw/isolation.jsonl'],
    n_contended_nonpari == 26 and len(n_nonpari) == 438)

merged = []
i = 0
while i < len(claims):
    c = claims[i]
    if c['claimed'].endswith(', low end'):
        h = claims[i + 1]
        assert h['claimed'].endswith(', high end')
        merged.append({'claimed': c['claimed'][:-len(', low end')],
                       'computed': c['computed'] + ' ; ' + h['computed'],
                       'definition': c['definition'],
                       'files': c['files'],
                       'status': 'match' if (c['status'] == 'match' and h['status'] == 'match') else 'mismatch'})
        i += 2
    else:
        merged.append(c)
        i += 1
claims = merged

for c in claims:
    if c['claimed'].startswith('vs PARI'):
        c['status'] = 'cannot_define'
        c['definition'] += ('; PROTOCOL states ratio of medians only for A/B and A/A2 and does not state how PARI rounds are paired or ranged, so this range is cannot_define')

# identical check: A vs B pairs, all runs (contention does not affect identity)
pinned_bsgs1 = ['answers_digest', 'table_entries', 'baby_steps', 'giant_steps', 'candidates', 'false_candidates']
pinned_bsgs_t = ['answers_digest']
pinned_rho1 = ['log', 'verified', 'total_steps', 'distinguished_points']
pinned_rho_t = ['log', 'verified']
n_cmp = n_eq = n_diff = 0
for (t, kern, bits, arm, rnd), ra in bi.items():
    if arm != 'A':
        continue
    rb = bi.get((t, kern, bits, 'B', rnd))
    if rb is None:
        continue
    fields = pinned_bsgs1 if t == 1 else pinned_bsgs_t
    n_cmp += 1
    if all(ra[f] == rb[f] for f in fields):
        n_eq += 1
    else:
        n_diff += 1
for (t, bits, arm, rnd), ra in ri.items():
    if arm != 'A':
        continue
    rb = ri.get((t, bits, 'B', rnd))
    if rb is None:
        continue
    fields = pinned_rho1 if t == 1 else pinned_rho_t
    n_cmp += 1
    if all(ra[f] == rb[f] for f in fields):
        n_eq += 1
    else:
        n_diff += 1
# opcount pairs: key on (bits, phase, kernel, pair_k, chains, len, visited, occurrence)
def opkeys(rows, arm):
    seen = collections.Counter()
    out = {}
    for r in rows:
        if not r['label'].endswith('-' + arm):
            continue
        base = (r['phase'], r['kernel'], r['pair_k'], r['chains'], r['len'], r['visited'])
        seen[base] += 1
        out[base + (seen[base],)] = r
    return out
oa = opkeys(opc, 'A')
ob = opkeys(opc, 'B')
o_cmp = o_eq = o_diff = 0
for k, ra in oa.items():
    rb = ob.get(k)
    if rb is None:
        continue
    o_cmp += 1
    if all(ra[f] == rb[f] for f in ('mul', 'sqr', 'inv', 'add')):
        o_eq += 1
    else:
        o_diff += 1

identical = {
    'claimed': True,
    'n_compared': n_cmp + o_cmp,
    'n_equal': n_eq + o_eq,
    'n_differ': n_diff + o_diff,
    'files': [RUN + 'raw/bsgs1.jsonl', RUN + 'raw/bsgs3.jsonl', RUN + 'raw/rho1.jsonl', RUN + 'raw/rho3.jsonl', RUN + 'raw/opcount.jsonl'],
    'status': 'match' if (n_diff + o_diff) == 0 else 'mismatch',
}

out = {
    'row_no': 14,
    'slug': '20261017-generic-dlp-memory-parallelism',
    'claims': claims,
    'identical': identical,
    'offrepo_refs': [],
    'aa_lines': [line(README, 71), line(README, 81), line(README, 111), line(README, 150)],
    'isolation_lines': [line(README, 19), line(README, 59), line(README, 76), line(README, 189)],
    'protocol_git_date': gd(PROTO),
    'first_raw_git_date': gd(RAW + 'bsgs1.jsonl'),
    'script': SCRIPT,
    'notes': ('identical units: A vs B record pairs (bsgs1 and rho1 at 1 thread, all pinned fields; bsgs3 and rho3 at 3 threads, answer fields only, per PROTOCOL pinned-outputs rule); '
              f'opcount pairs compared on mul, sqr, inv, add; n_compared = {n_cmp} run pairs + {o_cmp} opcount pairs. '
              'median = statistics.median (mean of two middle values when even). A2 control arm not in any claim. '
              'The "start-up" claim uses the setup_s field. The PARI ranges use PARI rounds per bits. '
              f'Contended runs (run.contended true) excluded from timing: {n_contended_nonpari} non-PARI runs. '
              'The study PDFs are not text-searchable here, so PDF text was not checked.'),
}
print(json.dumps(out, ensure_ascii=False))
