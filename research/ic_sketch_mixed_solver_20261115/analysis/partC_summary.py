"""Part C: what a Bloom 'seen-twice' gate on large-prime partials could save, from the ic.large_prime counters."""
import json, statistics as S
R = [json.loads(l) for l in open('sessions/partC-lp/records.jsonl')]
L = [x for x in R if x.get('arm') == 'lp']; H = [x for x in R if x.get('arm') != 'lp']
c = lambda k: [x['detail']['counters'].get(k, 0) for x in L]
out = {
  'runs_lp': len(L), 'runs_rho': len(H), 'verified': sum(x['outcome'].get('status') == 'verified' for x in R),
  'S_lp_mean': S.mean(x['cost']['s'] for x in L), 'S_rho_mean': S.mean(x['cost']['s'] for x in H),
  'combination_states_mean': S.mean(c('combination_states_uncharged')),
  'lp_merge_ops_mean': S.mean(c('lp_merge_ops_uncharged')), 'lp_pivots_mean': S.mean(c('lp_pivots')),
  'trials_mean': S.mean(c('trials')),
}
out['merge_share_of_native'] = out['lp_merge_ops_mean'] / (out['combination_states_mean'] + out['lp_merge_ops_mean'])
out['note'] = ('A Bloom seen-twice gate can only skip work on partials whose large prime never recurs: '
               'bounded above by lp_merge_ops (native, unpriced at w = 0). The large-prime store holds '
               'lp_pivots entries a run, so an exact table is already O(1) memory here.')
json.dump(out, open('analysis/partC_summary.json', 'w'), indent=1); print(json.dumps(out, indent=1))
