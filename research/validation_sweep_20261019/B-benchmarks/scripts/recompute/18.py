import sys, os, statistics
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _common import *

BASE = 'research/eigen_seta_assessment_20261017/'
RUN = BASE + 'records/run-20261017T1623Z-pk-degree-test/'
OUT = p('research/validation_sweep_20261019/B-benchmarks/recompute/rows/row18.json')
STUDY = [BASE + 'REPORT.md', BASE + 'report.typ', RUN + 'host.txt']


def kv(line):
    d = {}
    for tok in line.rstrip('\n').split('\t'):
        if '=' in tok:
            k, v = tok.split('=', 1)
            d[k] = v
    return d


def printed(claimed, computed, dec):
    return 'match' if round(computed, dec) == claimed else 'mismatch'


recs = {}
for lvl in ['L1', 'L3', 'L5']:
    with open(p(RUN + lvl + '.tsv')) as fh:
        recs[lvl] = [l.rstrip('\n').split('\t') for l in fh if l.strip()]

claims = []
ident_rows = {}
wrong_total = 0
wrong_acc = 0
spec = {'L1': (569, 1076, 0.79, 1.28, 7), 'L3': (832, 1594, 2.72, 5.54, 3), 'L5': (1087, 2106, 6.84, 13.34, 3)}
for lvl in ['L1', 'L3', 'L5']:
    rows = recs[lvl]
    times = [kv('\t'.join(r)) for r in rows if r[0] == 'time']
    e_true = int(times[0]['pk_test_e'])
    E = int(times[0]['decaps_E'])
    tests = [float(t['pk_test_s']) for t in times]
    decs = [float(t['decaps_s']) for t in times]
    ce, cE, ct, cd, n = spec[lvl]
    claims.append({'claimed': f"chain e/E = {ce}/{cE} ({lvl})", 'computed': f"e={e_true}, E={E}",
                   'definition': f"pk_test_e and decaps_E fields of the time rows in records/run.../{lvl}.tsv",
                   'files': [RUN + lvl + '.tsv'],
                   'status': 'match' if (e_true, E) == (ce, cE) else 'mismatch'})
    mt = statistics.median(tests)
    md = statistics.median(decs)
    claims.append({'claimed': f"test median {ct} s ({lvl}, n={n})", 'computed': f"median pk_test_s={mt:.4f} over n={len(tests)} time rows (range {min(tests):.4f}-{max(tests):.4f})",
                   'definition': 'median of pk_test_s over all time rows of the level',
                   'files': [RUN + lvl + '.tsv'],
                   'status': printed(ct, mt, 2) if len(tests) == n else 'mismatch'})
    claims.append({'claimed': f"decaps median {cd} s ({lvl}, n={n})", 'computed': f"median decaps_s={md:.4f} over n={len(decs)} time rows (range {min(decs):.4f}-{max(decs):.4f})",
                   'definition': 'median of decaps_s over all time rows of the level',
                   'files': [RUN + lvl + '.tsv'],
                   'status': printed(cd, md, 2) if len(decs) == n else 'mismatch'})
    ok = all(t['decaps_correct'] == 'true' for t in times)
    wrong = [kv('\t'.join(r)) for r in rows if r[0] == 'wrong']
    wrong_total += len(wrong)
    wrong_acc += sum(1 for w in wrong if w.get('accept') == 'true')

claims.append({'claimed': '158 wrong candidates, 0 accepted', 'computed': f"wrong rows={wrong_total}, accept=true among wrong rows={wrong_acc}",
               'definition': 'count of wrong-kind rows in L1/L3/L5 TSVs and count with accept=true',
               'files': [RUN + 'L1.tsv', RUN + 'L3.tsv', RUN + 'L5.tsv'],
               'status': 'match' if wrong_total == 158 and wrong_acc == 0 else 'mismatch'})

study_off = grep_lines(STUDY, OFFREPO_RX)
study_aa = grep_lines(STUDY, AA_RX)
study_iso = grep_lines(STUDY, ISO_RX)
proto = BASE + 'REPORT.md'
first_raw = BASE + 'records/eigenx_tests_main_d4ee763.txt'

row = {
    'row_no': 18, 'slug': '20261017-eigen-seta-pk-degree-test',
    'claims': claims,
    'identical': {'claimed': False, 'n_compared': None, 'n_equal': None, 'n_differ': None, 'files': [], 'status': 'not_claimed'},
    'offrepo_refs': study_off, 'aa_lines': study_aa, 'isolation_lines': study_iso,
    'protocol_git_date': git_added(proto), 'first_raw_git_date': git_added(first_raw),
    'script': 'research/validation_sweep_20261019/B-benchmarks/scripts/recompute/18.py',
    'notes': ("no PROTOCOL file exists in the study dir; REPORT.md (Method section) is the spec-bearing file used for protocol_git_date. "
              "first raw file = first RAW CANDIDATES entry as listed (records/eigenx_tests_main_d4ee763.txt). "
              "Median test/decaps values are computed over the time rows of each level TSV. The wrong-candidate count covers all wrong-kind rows in the three TSVs."),
}
write_row(18, row, OUT)
