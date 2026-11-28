"""Part A summary: medians per (|F|, m, arm) and the P1-P4 scores. Reads partA/stream.jsonl."""
import json, statistics as S, collections, sys
src = sys.argv[1] if len(sys.argv) > 1 else 'partA/stream.jsonl'
R = [json.loads(l) for l in open(src)]
g = collections.defaultdict(list); by = collections.defaultdict(dict)
for r in R:
    g[(r['fb'], r['m'], r['arm'])].append(r); by[(r['fb'], r['m'], r['seed'])][r['arm']] = r
arms = sorted({r['arm'] for r in R}); cells = sorted({(r['fb'], r['m']) for r in R})
out = {'records': len(R), 'incorrect': sum(not r['correct'] for r in R), 'cells': []}
for fb, m in cells:
    row = {'fb': fb, 'm': m, 'arms': {}}
    for a in arms:
        L = g[(fb, m, a)]
        row['arms'][a] = {'rows': S.median(x['rows_read'] for x in L), 'row_ops': S.median(x['work'] for x in L),
                          'n': len(L), 'rule': L[0]['rule']}
    out['cells'].append(row)
streams = list(by.values())
def frac(pred): return sum(1 for v in streams if pred(v)) / len(streams)
fw, fwm = 'filter+wiedemann', 'filter+wiedemann+match'
out['P1_rows_le_all'] = all(v[fwm]['rows_read'] <= v[fw]['rows_read'] for v in streams)
out['P1_rows_lt_frac'] = frac(lambda v: v[fwm]['rows_read'] < v[fw]['rows_read'])
out['P1_rowops_lower_median_fb>=64'] = {f"{c['fb']}/{c['m']}": c['arms'][fwm]['row_ops'] < c['arms'][fw]['row_ops'] for c in out['cells'] if c['fb'] >= 64}
out['P2_mixed_rows_equal_frac'] = frac(lambda v: v['mixed-peel']['rows_read'] == v['incremental-gauss-full-rank']['rows_read'])
out['P3_mixed_lt_fw'] = {f"{c['fb']}/{c['m']}": c['arms']['mixed-peel']['row_ops'] < c['arms'][fw]['row_ops'] for c in out['cells']}
out['P3_mixed_lt_igfr_fb>=128'] = {f"{c['fb']}/{c['m']}": c['arms']['mixed-peel']['row_ops'] < c['arms']['incremental-gauss-full-rank']['row_ops'] for c in out['cells'] if c['fb'] >= 128}
out['P4_duplicates_total'] = sum(r['counters'].get('mixed_duplicates', 0) for r in R)
out['P4_bloom_overhead_all'] = all(v['mixed-peel+bloom']['work'] > v['mixed-peel']['work'] for v in streams)
json.dump(out, open('analysis/partA_summary.json', 'w'), indent=1)
for c in out['cells']:
    print(f"|F|={c['fb']} m={c['m']}")
    for a in arms:
        x = c['arms'][a]; print(f"   {a:28s} {x['rule']:13s} rows {x['rows']:7.1f} row_ops {x['row_ops']:11.1f}")
print({k: v for k, v in out.items() if k != 'cells'})
