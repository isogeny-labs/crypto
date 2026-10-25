import json, os, re, subprocess
REPO = '/home/user/crypto'
ROW = 76
STUDY = 'research/ic_next10_20261014/06-equivariant-f4-m4'
RUN = os.path.join(REPO, STUDY, 'runs')
SELF = 'scripts/recompute/76.py'
ctrl = {d['n']: d for d in (json.loads(l) for l in open(os.path.join(RUN, 'g2_full_pass_control.jsonl')))}
blk = {d['n']: d for d in (json.loads(l) for l in open(os.path.join(RUN, 'g2_full_pass_blocks.jsonl')))}
fc = 'runs/g2_full_pass_control.jsonl'; fb = 'runs/g2_full_pass_blocks.jsonl'
def cpu_ratio(n):
    c = ctrl[n]; return c['full_rref_cpu'] / (c['blocks_rref_cpu'] + c['inverse_cpu'])
def asm_ratio(n):
    c = ctrl[n]; return c['full_rref_cpu'] / (c['blocks_rref_cpu'] + c['inverse_cpu'] + c['build_s'])
cl = []
def add(claimed, computed, definition, files, status):
    cl.append({'claimed': claimed, 'computed': computed, 'definition': definition,
               'files': [os.path.join(STUDY, f) for f in files] if files and not files[0].startswith('research/') else files, 'status': status})
for n, c in [(11, 44), (13, 58), (17, 73)]:
    v = cpu_ratio(n)
    add(f'{c}x CPU at n={n} (block RREF + inverse vs full F2 RREF)', f'{v:.1f}',
        'full_rref_cpu / (blocks_rref_cpu + inverse_cpu), same process', [fc],
        'match' if abs(v - c) / c <= 0.01 or round(v) == c else 'mismatch')
rng = [asm_ratio(n) for n in (11, 13, 17)]
add('25-45x with assembly charged (n=11..17)', f'{min(rng):.1f}-{max(rng):.1f}',
    'full_rref_cpu / (blocks_rref_cpu + inverse_cpu + build_s) at n=11,13,17 (build_s = assembly wall); README table gives 18, 22, 45',
    [fc, fb], 'match' if round(min(rng)) == 25 and round(max(rng)) == 45 else 'mismatch')
tot31 = blk[31]['build_s'] + blk[31]['blocks_rref_cpu'] + blk[31]['inverse_cpu']
add('n=31 equivariant pass in 31 s', f'{tot31:.1f}',
    'build_s (assembly wall) + blocks_rref_cpu + inverse_cpu at n=31', [fb],
    'match' if round(tot31) == 31 else 'mismatch')
gib = 275191 * 645566 / 8 / 2**30
add('full F2 matrix at n=31 needs about 21 GB', f'{gib:.1f} GiB',
    'dense bit-packed size of sym_rows x sym_cols = 275191 x 645566 bits, in GiB (README table gives 20.7 GB > cap); definition assumed from "dense"',
    [fb], 'match' if round(gib) == 21 else 'mismatch')
s = open(os.path.join(RUN, 'cost_d4-output.txt')) if False else None
cost_txt = open(os.path.join(REPO, STUDY, 'cost_d4-output.txt')).read()
has = '2^+7.62' in cost_txt
add('modelled m=4 cost sits +7.6 bits above rho at w=1', '2^+7.62' if has else 'row not found',
    'row (m=4, D=4, w=1, best l=33) of cost_d4-output.txt', [os.path.join(STUDY, 'cost_d4-output.txt')],
    'match' if has else 'mismatch')
add('+21/+112 modelled n=131 price before K', 'cannot_define',
    'no +21 or +112 n=131 figure found in cost_d4-output.txt or README.md; README cites 2^+21.30 and 2^+173.13 for other rows', [os.path.join(STUDY, 'README.md'), os.path.join(STUDY, 'cost_d4-output.txt')], 'cannot_define')
pairs = []
for line in open(os.path.join(RUN, 'c2_identity.txt')):
    m = re.search(r'XL (\S+) \[[^\]]*\] F4 (\S+)', line)
    if m: pairs.append((m.group(1), m.group(2)))
def num(x): return int(x.split('+')[0])
xl = sorted({x for x, _ in pairs}, key=num)
f4 = sorted({f for _, f in pairs}, key=num)
assert len(pairs) == 48, len(pairs)
diffs = [(num(x) - num(f), '+(>=)' in x) for x, f in pairs]
dmin = min(d for d, _ in diffs); dmax = max(diffs)
xl_str = f'{xl[0]} to {"≥" if "+(>=)" in xl[-1] else ""}{num(xl[-1])}'
dmax_str = f'{"≥" if dmax[1] else ""}{dmax[0]}'
add('XL reads 5 to ≥7 and F4 is 1 to ≥3 degrees lower', f'XL {xl_str}; F4 {f4[0]}-{f4[-1]}; gap {dmin} to {dmax_str}',
    'XL and F4 degree per draw parsed from runs/c2_identity.txt (XL value vs F4 value on the same line)',
    [os.path.join(STUDY, 'runs/c2_identity.txt')],
    'match' if xl_str == '5 to ≥7' and dmin == 1 and dmax_str == '≥3' else 'mismatch')

def grep_lines(rel, pat):
    out = []
    with open(os.path.join(REPO, STUDY, rel)) as fh:
        for i, line in enumerate(fh, 1):
            if re.search(pat, line):
                out.append(f'{STUDY}/{rel}:{i}: {line.rstrip()}')
    return out
aa = grep_lines('README.md', r'[Cc]ontrol|[Nn]ull|[Ii]dentical') + grep_lines('PROTOCOL.md', r'[Cc]ontrol|[Nn]ull')
iso = grep_lines('README.md', r'3 GB|isolat|shared|single thread|single-thread|load') + grep_lines('PROTOCOL.md', r'3 GB|shared|isolat|single')
def git_date(path):
    out = subprocess.run(['git', '-C', REPO, 'log', '--diff-filter=A', '--format=%H %cI', '--', path], capture_output=True, text=True).stdout.strip()
    return out if out else 'EMPTY'
ident_ok = sum(1 for n in (7, 11, 13, 17) if ctrl[n]['rref_identical'] is True)
out = {'row_no': ROW, 'slug': '20261014-ic-next10-06-equivariant-f4-m4', 'claims': cl,
       'identical': {'claimed': True, 'n_compared': 4, 'n_equal': ident_ok, 'n_differ': 4 - ident_ok,
                     'files': [os.path.join(STUDY, fc)], 'status': 'match' if ident_ok == 4 else 'mismatch'},
       'offrepo_refs': [], 'aa_lines': aa, 'isolation_lines': iso,
       'protocol_git_date': git_date(os.path.join(STUDY, 'PROTOCOL.md')),
       'first_raw_git_date': git_date(os.path.join(STUDY, 'runs/c2_calibration.jsonl')),
       'script': SELF,
       'notes': ('Identity check uses rref_identical in g2_full_pass_control.jsonl for n=7,11,13,17 (degree-fall row counts also equal there: full_low_rows == equivariant_low_rows). '
                 'Headline "25-45x" compared to the README table ratios (18, 22, 45) and to the recomputed assembly-charged ratios. '
                 'The 21 GB figure is a dense bit-packed size computed here (assumption). No off-repo references found in study .md/.txt/.json files. '
                 'Headline "+21/+112" has no matching value in the repo. '
                 'Not checked: msolve degree counts beyond c2_identity.txt, and the headline "(4,5)/(4,6) blocked at 3 GB" (text claim).')}
print(json.dumps(out, ensure_ascii=False))
