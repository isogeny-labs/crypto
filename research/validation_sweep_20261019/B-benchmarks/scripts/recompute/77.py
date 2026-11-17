import csv, json, os, re, subprocess
REPO = '/home/user/crypto'
ROW = 77
STUDY = 'research/ic_next10_20261014/03-pkm-degree'
D = os.path.join(REPO, STUDY)
SELF = 'scripts/recompute/77.py'
res = list(csv.DictReader(open(os.path.join(D, 'results.csv'))))
f4 = {r['x_r']: r for r in csv.DictReader(open(os.path.join(D, 'f4_committed.csv')))}
xl = list(csv.DictReader(open(os.path.join(D, 'xl.csv'))))
fl_res = [os.path.join(STUDY, 'results.csv')]

def tab(path):
    pat = re.compile(r'^\s*(\d+)\s+(\d+)\s+(\d+)\s+(\d+ x \d+)\s+([\d.]+)%\s+(\d+) new\s+(\d+) zero')
    out = []
    for line in open(path):
        m = pat.match(line)
        if m: out.append(m.groups())
    return out

fin = [r for r in res if r['exit'] == '0']
match16 = sum(1 for r in fin if r['x_r'] in f4 and r['D_msolve'] == f4[r['x_r']]['D_f4'])
claims = []
def add(claimed, computed, definition, files, status):
    claims.append({'claimed': claimed, 'computed': computed, 'definition': definition, 'files': files, 'status': status})

t5a = tab(os.path.join(D, 'runs/pkm_kummer_p2013265921_m4_t5_x362838726.log'))
t5b = tab(os.path.join(D, 'runs/pkm_kummer_p2013265921_m4_t5_x843615913.log'))
deg8_new = [int(r[5]) for r in t5a if r[0] == '8' and int(r[5]) > 0]
add('three productive degree-8 rounds at m=4 N=20', f'{len(deg8_new)} rounds, new = {deg8_new}',
    'rows of the -v 2 round table with deg=8 and new>0 in runs/...m4_t5_x362838726.log', [os.path.join(STUDY, 'runs/pkm_kummer_p2013265921_m4_t5_x362838726.log')],
    'match' if len(deg8_new) == 3 and deg8_new == [1241, 2507, 3578] else 'mismatch')
vm = int([r for r in res if r['x_r'] == '362838726'][0]['VmPeak_kB'])
add('msolve m=4 N=20 stays within 5 GB', f'VmPeak {vm*1024} bytes (cap 5e9 per README)',
    'VmPeak_kB * 1024 from results.csv (x_r 362838726) against 5e9 bytes', fl_res,
    'match' if vm * 1024 < 5e9 else 'mismatch')
add('f4_fp_tower needs >14 GB at N=20', 'cannot_define',
    'no f4_fp_tower memory figure for N=20 found in the study files read', [os.path.join(STUDY, 'README.md')], 'cannot_define')
add('matched f4_fp_tower on all 16 finished systems', f'{match16}/{len(fin)}',
    'count of exit==0 rows in results.csv whose D_msolve equals D_f4 of the same x_r in f4_committed.csv', fl_res + [os.path.join(STUDY, 'f4_committed.csv')],
    'match' if match16 == 16 and len(fin) == 16 else 'mismatch')
d20 = [r for r in res if r['m'] == '4' and r['N'] == '20']
add('m=4 plateau at 7 breaks (D >= 8) at N=20', f'D_last_productive={",".join(r["D_last_productive"] for r in d20)}',
    'D_last_productive for m=4 N=20 rows in results.csv', fl_res,
    'match' if all(int(r['D_last_productive']) >= 8 for r in d20) else 'mismatch')
m3 = [r for r in res if r['m'] == '3' and int(r['N']) >= 15]
add('m=3 D >= 7 undecided (N=15..24)', f'{len(m3)} runs, verdict={sorted({r["verdict"] for r in m3})}, D_last_productive={sorted({r["D_last_productive"] for r in m3})}',
    'results.csv rows with m=3 and N>=15', fl_res,
    'match' if all(r['verdict'] == 'none' and r['D_last_productive'] == '7' for r in m3) else 'mismatch')
xl2 = [r for r in xl if r['m'] == '2']
xl_ok = sum(1 for r in xl2 if int(r['D_XL']) == int(r['N']) + 4)
add('XL degree = N + 4 exactly for m=2', f'{xl_ok}/{len(xl2)} rows with D_XL = N + 4',
    'xl.csv rows with m=2 (N=12..26, both methods)', [os.path.join(STUDY, 'xl.csv')],
    'match' if xl_ok == len(xl2) else 'mismatch')

# identical: the two m=4 N=20 round tables
n_cmp = min(len(t5a), len(t5b))
eq = sum(1 for a, b in zip(t5a, t5b) if a[:4] + a[5:7] == b[:4] + b[5:7])
ident = {'claimed': True, 'n_compared': n_cmp, 'n_equal': eq, 'n_differ': n_cmp - eq,
         'files': [os.path.join(STUDY, 'runs/pkm_kummer_p2013265921_m4_t5_x362838726.log'), os.path.join(STUDY, 'runs/pkm_kummer_p2013265921_m4_t5_x843615913.log')],
         'status': 'match' if eq == n_cmp and len(t5a) == len(t5b) else 'mismatch'}
note_ident = f'identity compares round rows (deg, sel, pairs, matrix, density, new, zero; timings excluded): rows {len(t5a)} vs {len(t5b)}. These are two targets, not two arms.'

def grep_lines(rel, pat):
    out = []
    with open(os.path.join(D, rel)) as fh:
        for i, line in enumerate(fh, 1):
            if re.search(pat, line):
                out.append(f'{STUDY}/{rel}:{i}: {line.rstrip()}')
    return out
aa = grep_lines('README.md', r'[Cc]ontrol|[Nn]ull')
iso = grep_lines('README.md', r'load of|nice|prlimit|single-thread|isolat|shared') + grep_lines('PROTOCOL.md', r'load|nice|prlimit|isolat|shared')
def git_date(path):
    out = subprocess.run(['git', '-C', REPO, 'log', '--diff-filter=A', '--format=%H %cI', '--', path], capture_output=True, text=True).stdout.strip()
    return out if out else 'EMPTY'
out = {'row_no': ROW, 'slug': '20261014-ic-next10-03-pkm-degree', 'claims': claims, 'identical': ident,
       'offrepo_refs': [], 'aa_lines': aa, 'isolation_lines': iso,
       'protocol_git_date': git_date(os.path.join(STUDY, 'PROTOCOL.md')),
       'first_raw_git_date': git_date(os.path.join(STUDY, 'logs/convert.jsonl')),
       'script': SELF,
       'notes': ('N = m * t in results.csv (checked: N=20 at m=4 is t=5). Off-repo: no hits in study .md/.txt/.json. ' + note_ident +
                 ' First raw = logs/convert.jsonl (first RAW CANDIDATES entry). Bootstrap or CI not claimed for this row.')}
print(json.dumps(out, ensure_ascii=False))
