import sys, os, json, glob, statistics, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _common import *

BASE = 'research/f5_gf2_defaults_20261017/'
RUNS = {'run1': BASE + 'reports/run-20261017T0331Z/', 'run2': BASE + 'reports/run-20261017T0341Z/'}
OUT = p('research/validation_sweep_20261019/B-benchmarks/recompute/rows/row21.json')
DEC = ("hit", "indices", "exhausted", "reductions", "splits", "propagations", "infeasible", "word_xors")
med = statistics.median


def parse(run_dir):
    f5 = collections.defaultdict(list)           # (form, th, case, arm) -> wall_ms per round
    f5key = collections.defaultdict(set)
    f4 = collections.defaultdict(list)           # (case, arm)
    f4key = collections.defaultdict(set)
    inhkey = collections.defaultdict(set)
    rounds = set()
    for path in sorted(glob.glob(p(run_dir) + 'raw/*.jsonl')):
        lines = [json.loads(l) for l in open(path) if l.strip()]
        m = lines[0]
        rounds.add(m['round'])
        for d in lines[1:]:
            if m['tool'] == 'f5':
                k = (m['form'], m['threads'], d['case'])
                f5[k + (m['arm'],)].append(d['wall_ms'])
                f5key[k].add((d['rows_fp'], d['row_space_fp'], d['output_terms'], d['reduce_word_ops'], d['rank']))
            elif m['tool'] == 'f4':
                f4[(d['case'], m['arm'])].append(d['wall_ms'])
                f4key[d['case']].add((d['basis_fp'], d['word_xors']))
            elif m['tool'] == 'inh' and 'target' in d:
                inhkey[(m['form'], d['target'])].add(tuple(json.dumps(d[x]) for x in DEC))
    return f5, f5key, f4, f4key, inhkey, rounds


P = {r: parse(d) for r, d in RUNS.items()}
ratio = {}
for r, (f5, _, f4, _, _, _) in P.items():
    for (form, th, case, arm), v in f5.items():
        if arm == 'on':
            continue
        on = f5[(form, th, case, 'on')]
        ratio[(r, 'f5', form, th, case, arm)] = med(v) / med(on)
    for (case, arm), v in f4.items():
        if arm != 'on':
            ratio[(r, 'f4', case, arm)] = med(v) / med(f4[(case, 'on')])

claims = []
# 1. direct pack, run 1, every F5 cell (nopack arm / on)
pk = [ratio[k] for k in ratio if k[0] == 'run1' and k[1] == 'f5' and k[5] == 'nopack']
lo_pk, hi_pk = min(pk), max(pk)
claims.append({'claimed': 'direct pack 1.006–2.173× every F5 cell (run 1)',
               'computed': f"nopack arm/on over {len(pk)} F5 cells: min={lo_pk:.4f} (f5_n12_m12_d4 form0 t4 in study table.tsv) max={hi_pk:.4f} (f5_n20_m20_d3 form2 t4)",
               'definition': 'median over rounds of arm wall_ms / median over rounds of on wall_ms, arm=nopack, run-20261017T0331Z raw, all F5 cells',
               'files': [RUNS['run1'] + 'raw/'],
               'status': 'match' if round(lo_pk, 3) == 1.006 and round(hi_pk, 3) == 2.173 else 'mismatch'})

# 2. direct unpack off, n24 d4 Selective 4 threads (form2, th4), run 2
v = ratio[('run2', 'f5', '2', 4, 'f5_n24_m24_d4', 'nounpack')]
claims.append({'claimed': 'direct unpack 0.986× on f5_n24_m24_d4 SelectiveEchelon 4 threads',
               'computed': f"nounpack arm/on = {v:.4f} (form2 = SelectiveEchelon)",
               'definition': 'median arm/on, run-20261017T0341Z, form 2, threads 4, case f5_n24_m24_d4, arm nounpack',
               'files': [RUNS['run2'] + 'raw/'],
               'status': 'match' if round(v, 3) == 0.986 else 'mismatch'})

# 3. table reuse, matrix F4 n16 m24, run 2
f4k = sorted(k for k in ratio if k[0] == 'run2' and k[1] == 'f4' and 'n16' in k[2] and 'm24' in k[2])
vals = {k[2] + '/' + k[3]: round(ratio[k], 4) for k in f4k}
v3 = [ratio[k] for k in f4k if k[3] == 'noreuse']
claims.append({'claimed': 'table reuse 0.955× matrix F4 n16 m24',
               'computed': f"noreuse arm/on for f4 n16 m24 cases: {vals}",
               'definition': 'median arm/on, run-20261017T0341Z, tool f4, case containing n16 and m24, arm noreuse',
               'files': [RUNS['run2'] + 'raw/'],
               'status': 'match' if v3 and any(round(x, 3) == 0.955 for x in v3) else 'mismatch'})

# 4. all three off vs on on the largest F5 cases (run 2). largest = the two cases with the largest median on wall_ms (pre-declared)
f5r2 = P['run2'][0]
on_med = {case: med(f5r2[(f, t, case, 'on')]) for (f, t, case, a) in f5r2 if a == 'on'}
top2 = sorted(set(on_med), key=lambda c: -on_med[c])[:2]
off_vals = [ratio[('run2', 'f5', f, t, c, 'off')] for f in ['0', '2'] for t in [1, 4] for c in top2]
claims.append({'claimed': 'all three vs previous defaults 1.11–1.27× on the largest F5 cases (run 2)',
               'computed': f"largest cases by median on_ms = {top2} ({[round(on_med[c],2) for c in top2]} ms); off arm/on over forms 0,2 x threads 1,4: min={min(off_vals):.4f} max={max(off_vals):.4f}",
               'definition': 'off arm / on median ratio over the two largest-on_ms F5 cases, all form/thread cells, run-20261017T0341Z',
               'files': [RUNS['run2'] + 'raw/'],
               'status': 'match' if round(min(off_vals), 2) == 1.11 and round(max(off_vals), 2) == 1.27 else 'mismatch'})

# 5. rounds per run
claims.append({'claimed': '3 then 15 rounds', 'computed': f"run1 rounds={len(P['run1'][5])}, run2 rounds={len(P['run2'][5])}",
               'definition': 'distinct round values in the first line (meta) of raw/*.jsonl per run',
               'files': [RUNS['run1'] + 'raw/', RUNS['run2'] + 'raw/'],
               'status': 'match' if len(P['run1'][5]) == 3 and len(P['run2'][5]) == 15 else 'mismatch'})

# identical claim: exactness keys over both runs
n_cmp = n_eq = n_diff = 0
for r in RUNS:
    f5, f5key, f4, f4key, inhkey, _ = P[r]
    for dct in (f5key, f4key, inhkey):
        for key, s in dct.items():
            n_cmp += 1
            if len(s) == 1:
                n_eq += 1
            else:
                n_diff += 1
ident = {'claimed': True, 'n_compared': n_cmp, 'n_equal': n_eq, 'n_differ': n_diff,
         'files': [RUNS['run1'] + 'raw/', RUNS['run2'] + 'raw/'],
         'status': 'match' if n_diff == 0 and n_cmp > 0 else 'mismatch'}

study = [BASE + 'README.md', BASE + 'REPORT.typ', BASE + 'scripts/DECISION_RULE.md']
study_off = grep_lines(study, OFFREPO_RX)
study_aa = grep_lines(study, AA_RX)
study_iso = grep_lines(study, ISO_RX)
proto = BASE + 'scripts/DECISION_RULE.md'
first_raw = RUNS['run1'] + 'summary.txt'

row = {
    'row_no': 21, 'slug': '20261017-f5-gf2-defaults',
    'claims': claims, 'identical': ident,
    'offrepo_refs': study_off, 'aa_lines': study_aa, 'isolation_lines': study_iso,
    'protocol_git_date': git_added(proto), 'first_raw_git_date': git_added(first_raw),
    'script': 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/21.py',
    'notes': ("protocol file = scripts/DECISION_RULE.md (written before run 2). first raw file = run1 summary.txt as the first listed candidate in the card is run1 summary.txt. "
              "Ratios recomputed from raw/*.jsonl using the study rule in summarize.py: medians over rounds of wall_ms, arm/on. "
              "Exactness keys: f5 (rows_fp,row_space_fp,output_terms,reduce_word_ops,rank) per (form,threads,case); f4 (basis_fp,word_xors) per case; inherited DEC tuple per (form,target). Both runs combined. "
              "Form 0 = Reduced, form 2 = SelectiveEchelon per REPORT.typ. Largest F5 cases = two with the largest median on_ms in run 2 (declared before computing)."),
}
write_row(21, row, OUT)
