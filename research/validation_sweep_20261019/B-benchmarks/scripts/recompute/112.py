#!/usr/bin/env python3
# SWB-RECOMPUTE-14 row 112: 20261012-pdp-root-finder. Reads committed files only; appends one JSON object.
import json, os, re, statistics, subprocess

ROOT = '/home/user/crypto'
OUT = os.path.join(ROOT, 'research/validation_sweep_20261019/B-benchmarks/recompute/SWB-RECOMPUTE-14.jsonl')
S = 'research/pdp_root_finder_20261012'

def jl(path):
    p = os.path.join(ROOT, path)
    if not os.path.exists(p) or os.path.getsize(p) == 0:
        return []
    with open(p) as f:
        return [json.loads(l) for l in f if l.strip()]

def has_contended(o):
    if isinstance(o, dict):
        for k, v in o.items():
            if 'contend' in k.lower() and v:
                return True
            if has_contended(v):
                return True
    elif isinstance(o, list):
        return any(has_contended(x) for x in o)
    return False

iso = jl(S + '/raw/isolated.jsonl')
contended_labels = set(r['label'] for r in iso if has_contended(r))
n_iso = len(iso)
n_contended = sum(1 for r in iso if has_contended(r))
uncontended_iso_labels = set(r['label'] for r in iso if not has_contended(r))

bin_time = jl(S + '/raw/bin_time.jsonl')
prime_time = jl(S + '/raw/prime_time.jsonl')
bin_count = jl(S + '/raw/bin_count.jsonl')
prime_l3 = jl(S + '/raw/prime_ladder_m3.jsonl')
bin_v = jl(S + '/raw/bin_verify.jsonl') + jl(S + '/raw/bin_verify_fp.jsonl')

def uncontended(rows):
    return [r for r in rows if r.get('label') not in contended_labels]

def is_ell_row(r, arm, l):
    # ℓ-rung rows of the README table: label roundN-<arm>-l<ℓ> (ℓ=10 baseline also pools roundN-aa<k>-baseline-l10);
    # the (23, 8) cell carries the label suffix n23l<ℓ> and is not part of these rungs
    lab = r.get('label', '')
    pat = r'round\d+-(aa\d-)?%s-l%d' % (re.escape(arm), l)
    return r['arm'] == arm and r['l'] == l and re.fullmatch(pat, lab) is not None

def med_ns(rows, arm, l=None):
    v = [r['ns_per_target'] for r in rows if is_ell_row(r, arm, l) and r.get('ns_per_target') is not None]
    return (statistics.median(v) if v else None), len(v)

bt = uncontended(bin_time)
rf_mitm = {l: med_ns(bt, 'rf-mitm', l) for l in (8, 12, 14)}
ps0 = {l: med_ns(bt, 'baseline', l) for l in (8, 12, 14)}
ps2 = {l: med_ns(bt, 'group-scalar', l) for l in (8, 12, 14)}

ratio_ps0 = {l: ps0[l][0] / rf_mitm[l][0] for l in (8, 12, 14)}
ratio_ps2 = {l: ps2[l][0] / rf_mitm[l][0] for l in (8, 12, 14)}

# counted work per target for rf-mitm: mul = 5|F| - 3, sqr = 2|F|, lookups = 2|F|, |F| = base_points
mitm_counts = []
for r in bin_count:
    if r['arm'] == 'rf-mitm':
        F = r['base_points']
        pt = r['per_target']
        mitm_counts.append((r['l'], F, pt['mul'], pt['sqr'], pt['lookups'], pt['mul'] == 5 * F - 3, pt['sqr'] == 2 * F, pt['lookups'] == 2 * F))
mitm_formula_ok = all(x[5] and x[6] and x[7] for x in mitm_counts) if mitm_counts else None

# bits per rung: OLS slope of log2(mul per target) vs l for rf-mitm (computed for the record; rung set not fully stated)
xs, ys = [], []
for r in bin_count:
    if r['arm'] == 'rf-mitm':
        xs.append(r['l']); ys.append(__import__('math').log2(r['per_target']['mul']))
slope = None
if len(set(xs)) >= 2:
    mx, my = statistics.mean(xs), statistics.mean(ys)
    sxx = sum((x - mx) ** 2 for x in xs)
    slope = sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sxx

# prime m=3 at t=6, mitm3 per target multiplications
prime_rows_all = jl(S + '/raw/prime_rows.jsonl')
p197 = [a['mul'] for r in prime_rows_all if r.get('m') == 3 and r.get('t') == 6 for a in r.get('arms', []) if a['arm'] == 'mitm3']
p197_ladder = [r['mul_per_target'] for r in prime_l3 if r['arm'] == 'mitm3' and r.get('m') == 3 and r.get('t') == 6]
pt13 = uncontended(prime_time)
mitm3_t13 = [r['ns_per_target'] for r in pt13 if r['arm'] == 'mitm3' and r.get('t') == 13 and r.get('m') == 3 and r.get('ns_per_target') is not None]

# exactness digests: verdict and root digests against the corpus, both bin_verify files
dig_cmp = 0
dig_eq = 0
mismatch_total = 0
for r in bin_v:
    for a, b in (('digest_verdicts', 'digest_verdicts_corpus'), ('digest_roots', 'digest_roots_corpus')):
        dig_cmp += 1
        if r[a] == r[b]:
            dig_eq += 1
    mismatch_total += r.get('mismatches', 0) or 0

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

docs = [S + '/README.md', S + '/PROTOCOL.md', S + '/ANALYSIS.md']
offrepo_refs, aa_lines, isolation_lines = [], [], []
for p in docs:
    offrepo_refs += grep_lines(p, r'drive\.google|/Volumes|SSD|/Users|s3://|gs://|store:|Modal volume|not committed|scratch')
    aa_lines += grep_lines(p, r'A/A')
    isolation_lines += grep_lines(p, r'isolat|reserved|shared host|CPU 3|contended')

def git_add(path):
    r = subprocess.run(['git', '-C', ROOT, 'log', '--diff-filter=A', '--format=%H %cI', '--', path], capture_output=True, text=True)
    return r.stdout.strip() or 'empty'

def f1(x):
    return '%.1f' % x

claims = [
    {'claimed': 'rf-mitm 3.1 µs per refuted target at ℓ = 8 and 482 µs at ℓ = 14',
     'computed': 'median ns_per_target over uncontended rows: ℓ=8 %.1f ns (n=%d) ; ℓ=14 %.1f ns (n=%d)' % (rf_mitm[8][0], rf_mitm[8][1], rf_mitm[14][0], rf_mitm[14][1]),
     'definition': 'median ns_per_target of bin_time rows with arm rf-mitm at the stated ℓ, excluding labels flagged contended in isolated.jsonl',
     'files': [S + '/raw/bin_time.jsonl', S + '/raw/isolated.jsonl'],
     'status': 'match' if (f1(rf_mitm[8][0] / 1000) == '3.1' and round(rf_mitm[14][0] / 1000) == 482) else 'mismatch'},
    {'claimed': 'PS-0 baseline 9.3 ms → 55.5 s; 3,007× → 115,160× against rf-mitm',
     'computed': 'baseline median ℓ=8 %.1f ns (n=%d), ℓ=14 %.1f ns (n=%d); baseline ÷ rf-mitm: ℓ=8 %.1f, ℓ=14 %.1f' % (ps0[8][0], ps0[8][1], ps0[14][0], ps0[14][1], ratio_ps0[8], ratio_ps0[14]),
     'definition': 'ratio of medians, PS-0 baseline arm over rf-mitm arm, same ℓ',
     'files': [S + '/raw/bin_time.jsonl'],
     'status': 'match' if (round(ratio_ps0[8]) == 3007 and round(ratio_ps0[14]) == 115160) else 'mismatch'},
    {'claimed': 'PS-2 group-scalar 276 µs → 757 ms; 89× → 1,572× against rf-mitm',
     'computed': 'group-scalar median ℓ=8 %.1f ns (n=%d), ℓ=14 %.1f ns (n=%d); ÷ rf-mitm: ℓ=8 %.1f, ℓ=14 %.1f' % (ps2[8][0], ps2[8][1], ps2[14][0], ps2[14][1], ratio_ps2[8], ratio_ps2[14]),
     'definition': 'ratio of medians, group-scalar arm over rf-mitm arm, same ℓ',
     'files': [S + '/raw/bin_time.jsonl'],
     'status': 'match' if (round(ratio_ps2[8]) == 89 and round(ratio_ps2[14]) == 1572) else 'mismatch'},
    {'claimed': '5|F| − 3 multiplications, 2|F| squarings and 2|F| lookups a target (rf-mitm)',
     'computed': 'bin_count rf-mitm rows (l, |F|, mul, sqr, lookups, checks): %s ; all three formulas hold on every row: %s' % (mitm_counts, mitm_formula_ok),
     'definition': '|F| taken as base_points of the bin_count row; formula checks on per_target mul, sqr and lookups',
     'files': [S + '/raw/bin_count.jsonl'],
     'status': 'match' if mitm_formula_ok else 'mismatch'},
    {'claimed': '1.000 ± 0.027 bits per rung of ℓ (binary rf-mitm counted work)',
     'computed': 'OLS slope of log2(mul per target) on ℓ over %d bin_count rf-mitm rows = %s ; the study rung set and its s.e. rule are not stated in the committed text, so the ± 0.027 is not reproduced' % (len(xs), ('%.4f' % slope) if slope is not None else 'undefined'),
     'definition': 'slope of log2 counted multiplications against ℓ; rung set and s.e. formula not stated',
     'files': [S + '/raw/bin_count.jsonl', S + '/README.md'],
     'status': 'cannot_define'},
    {'claimed': '197 multiplications per target for mitm3 at N = 18 (m = 3, t = 6)',
     'computed': 'prime_rows mitm3 mul over %d rows at m=3 t=6: %s (median %s) ; the single prime_ladder_m3 counts-pass row at the same cell gives %s' % (len(p197), sorted(set(p197)), statistics.median(p197) if p197 else None, p197_ladder),
     'definition': 'mul field of the mitm3 arm entries of prime_rows.jsonl rows at m=3 t=6; the ladder file row is a different pass (label counts-pass) and is reported as a fact only',
     'files': [S + '/raw/prime_rows.jsonl', S + '/raw/prime_ladder_m3.jsonl'],
     'status': 'match' if (p197 and statistics.median(p197) == 197) else 'mismatch'},
    {'claimed': 'mitm3 306.5 µs per full scan at t = 13',
     'computed': 'median ns_per_target of uncontended prime_time rows, arm mitm3, m=3, t=13: %s µs over %d rows' % ((f1(statistics.median(mitm3_t13) / 1000) if mitm3_t13 else 'none'), len(mitm3_t13)),
     'definition': 'median ns_per_target over uncontended prime_time rows, converted to µs',
     'files': [S + '/raw/prime_time.jsonl', S + '/raw/isolated.jsonl'],
     'status': 'match' if (mitm3_t13 and f1(statistics.median(mitm3_t13) / 1000) == '306.5') else ('mismatch' if mitm3_t13 else 'no_raw_in_repo')},
    {'claimed': 'exactness: all digests equal, 707/707 brute-force agreements',
     'computed': 'digest comparisons (verdicts and roots, both bin_verify files): %d compared, %d equal, mismatches field sum %d ; 707/707 brute agreements is the study count (ANALYSIS.json), not recomputed here' % (dig_cmp, dig_eq, mismatch_total),
     'definition': 'verdict and root digests against the corpus digests; brute-force agreement count taken from the study aggregate only',
     'files': [S + '/raw/bin_verify.jsonl', S + '/raw/bin_verify_fp.jsonl'],
     'status': 'partial'},
    {'claimed': '5 rounds A/A + ABAB, 7 of 177 runs contended and excluded',
     'computed': 'isolated.jsonl records %d ; flagged contended %d ; uncontended labels %d' % (n_iso, n_contended, len(uncontended_iso_labels)),
     'definition': 'count of isolated.jsonl records with a contended flag set (any nested key containing "contend" with a true value)',
     'files': [S + '/raw/isolated.jsonl'],
     'status': 'match' if (n_iso == 177 and n_contended == 7) else 'mismatch'},
]

identical = {
    'claimed': True,
    'n_compared': dig_cmp,
    'n_equal': dig_eq,
    'n_differ': dig_cmp - dig_eq,
    'files': [S + '/raw/bin_verify.jsonl', S + '/raw/bin_verify_fp.jsonl'],
    'status': 'match' if dig_eq == dig_cmp else 'mismatch',
}

row = {
    'row_no': 112,
    'slug': '20261012-pdp-root-finder',
    'claims': claims,
    'identical': identical,
    'offrepo_refs': offrepo_refs,
    'aa_lines': aa_lines,
    'isolation_lines': isolation_lines,
    'protocol_git_date': 'PROTOCOL.md: %s' % git_add(S + '/PROTOCOL.md'),
    'first_raw_git_date': 'raw/bin_time.jsonl: %s' % git_add(S + '/raw/bin_time.jsonl'),
    'script': 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/112.py',
    'notes': 'Empty raw file in the study (no rows): raw/f6_ic_probe_n24_dim8.jsonl. Wall medians use rows labelled roundN-<arm>-l<ell> (the l10 baseline also pools roundN-aaK-baseline-l10), excluding the seven runs flagged contended in isolated.jsonl: round2-aa2-baseline-l10, round2-baseline-l8, round2-group-scalar-l9, round2-rf-mitm-l12, round4-baseline-l8, round4-rf-pairs-l9, round4-rf-mitm-l10. Fact: the median of all four round l8 baseline rows, including the two flagged ones, is %.4f ms; the README figure 9.3 ms (4 rounds) matches that set. Digest mismatch: raw/bin_verify_fp.jsonl, arm rf-mitm, l 8, n 23, root_sets_checked 79 of 80, digest_roots "-" against corpus digest d659bad77f1bc4f116aa334d50311d2fca57de3d92ca60ee9be64e579a46fb5e; the mismatches field is 0.' % (statistics.median([r['ns_per_target'] for r in bin_time if r['arm'] == 'baseline' and r['l'] == 8 and 're.fullmatch' and re.fullmatch(r'round\d+-baseline-l8', r['label'])]) / 1e6),
}

os.makedirs(os.path.dirname(OUT), exist_ok=True)
with open(OUT, 'a') as f:
    f.write(json.dumps(row, ensure_ascii=False) + '\n')
print('row 112 written; claims:', len(claims))
for c in claims:
    print(c['status'], '|', c['claimed'], '|', c['computed'])
print('identical:', json.dumps(identical))
print('offrepo_refs:', len(offrepo_refs), 'aa_lines:', len(aa_lines), 'isolation_lines:', len(isolation_lines))
