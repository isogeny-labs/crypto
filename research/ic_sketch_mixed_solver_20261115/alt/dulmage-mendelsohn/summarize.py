import json,sys,statistics as st
from collections import defaultdict
g=defaultdict(list)
for l in open(sys.argv[1]):
    r=json.loads(l); g[(r['fb'],r['m'])].append(r)
def mean(v): return sum(v)/len(v)
print("fb m n | rows_pin(IG) oracle slack(max|.|) | circ_rows circ_cols (min..max) | IG ops  ECH ops  | DM probes blockIG blockECH totIG totECH | MP rows MP ops | fineA cols rows")
allok=True
for k in sorted(g):
    R=g[k]; d=[r['dm'] for r in R]
    ok=all(x['circuit_equals_coarse_dm_block'] and x['circuit_minimal'] and x['no_pin_without_any_row'] and x['block_pin_correct_ig'] and x['block_pin_correct_echelon'] and x['slack_vs_incremental_gauss']==0 and x['slack_vs_echelon']==0 for x in d)
    ok=ok and all(r[a]['correct'] and r[a]['solved'] for r in R for a in ('incremental-gauss','incremental-gauss-echelon','mixed-peel'))
    allok&=ok
    cr=[x['circuit_rows'] for x in d]
    print(k[0],k[1],len(R),'|',round(mean([r['incremental-gauss']['rows_read'] for r in R]),1),round(mean([x['oracle_rows'] for x in d]),1),max(abs(x['slack_vs_incremental_gauss']) for x in d),'|',round(mean(cr),1),min(cr),max(cr),'|',
      round(mean([r['incremental-gauss']['row_ops'] for r in R])),round(mean([r['incremental-gauss-echelon']['row_ops'] for r in R])),'|',
      round(mean([x['matching_probes'] for x in d])),round(mean([x['block_row_ops_ig'] for x in d])),round(mean([x['block_row_ops_echelon'] for x in d])),round(mean([x['dm_total_ig'] for x in d])),round(mean([x['dm_total_echelon'] for x in d])),'|',
      round(mean([r['mixed-peel']['rows_read'] for r in R]),1),round(mean([r['mixed-peel']['row_ops'] for r in R])),'|',
      round(mean([x['fineA_target_block_cols'] for x in d]),1),round(mean([x['fineA_target_block_rows'] for x in d]),1),'allchecks',ok)
print("ALL CHECKS",allok)
# per-seed win counts, DM total vs IG, ECH
w=defaultdict(lambda:[0,0,0,0,0])
for k in g:
  for r in g[k]:
    d=r['dm']; w[k][0]+=1
    w[k][1]+= d['dm_total_ig']<r['incremental-gauss']['row_ops']
    w[k][2]+= d['dm_total_echelon']<r['incremental-gauss-echelon']['row_ops']
    w[k][3]+= d['dm_total_echelon']<r['mixed-peel']['row_ops']
    w[k][4]+= d['block_row_ops_echelon']<r['incremental-gauss-echelon']['row_ops']
print("per-seed wins (n, DMtotIG<IG, DMtotECH<ECH, DMtotECH<MP, blockECH<ECH):")
for k in sorted(w): print(k,w[k])
