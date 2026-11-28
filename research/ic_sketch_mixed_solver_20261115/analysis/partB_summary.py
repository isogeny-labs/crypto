"""Part B: per curve and arm, outcomes, mean S (w = 0), rows read, LA row_ops, and A3 fallbacks.
Rows read and S come from the ecbench records; band and paired intervals come from `ecbench band/compare` (analysis/partB_band_*.json, comparisons/)."""
import json, statistics as S, collections
out = {}
for t in ['p24', 'p28']:
    R = [json.loads(l) for l in open(f'sessions/partB-{t}/records.jsonl')]
    M = [r for r in R if not r['warmup']]
    g = collections.defaultdict(list)
    for r in M: g[r['arm']].append(r)
    cur = {}
    for a, L in sorted(g.items()):
        st = collections.Counter(r['outcome']['status'] for r in L)
        ok = [r for r in L if r['outcome']['status'] == 'verified']
        cur[a] = {
            'runs': len(L), 'status': dict(st),
            'S_mean_w0': S.mean(r['cost']['s'] for r in ok) if ok else None,
            'rows_mean': S.mean(r['counters'].get('matrix_rows', 0) for r in ok) if ok else None,
            'la_row_ops_mean': S.mean(r['counters'].get('matrix_work', 0) for r in ok) if ok else None,
            'fallback_runs': sum(1 for r in ok if 'mixed_fallbacks' in json.dumps(r)),
        }
    rho = cur.get('rho', {}).get('S_mean_w0')
    for a in cur:
        if cur[a]['S_mean_w0'] and rho: cur[a]['S_over_rho_w0'] = cur[a]['S_mean_w0'] / rho
    out[t] = cur
    print(t)
    for a, v in cur.items():
        print(f"  {a:28s} {v['status']}  S={v['S_mean_w0'] and round(v['S_mean_w0'],2)}  /rho={v.get('S_over_rho_w0') and round(v['S_over_rho_w0'],2)}  rows={v['rows_mean'] and round(v['rows_mean'],1)}  la_row_ops={v['la_row_ops_mean'] and round(v['la_row_ops_mean'])}  fallbacks={v['fallback_runs']}")
json.dump(out, open('analysis/partB_summary.json', 'w'), indent=1)
