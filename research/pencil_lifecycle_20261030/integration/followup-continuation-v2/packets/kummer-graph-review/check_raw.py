from pathlib import Path
import json,collections,hashlib,math
D=Path(__file__).parent;S=Path('/var/tmp/pencil-kummer-graph-integration-20261031');x=json.loads((S/'run.stdout').read_text())
def solve(es,n,r):
 rows=[]
 for ns,ss,a,b in es:
  z=[0]*(n+1)
  for i,s in zip(ns,ss):
   if i:z[i-1]=(z[i-1]+s)%r
  z[n-1]=-b%r;z[n]=a;rows.append(z)
 rank=0
 for c in range(n):
  piv=next((i for i in range(rank,len(rows)) if rows[i][c]),None)
  if piv is None:continue
  rows[rank],rows[piv]=rows[piv],rows[rank]
  scale=pow(rows[rank][c],-1,r);rows[rank]=[t*scale%r for t in rows[rank]]
  for i in range(len(rows)):
   if i!=rank:
    v=rows[i][c];rows[i]=[(a-v*b)%r for a,b in zip(rows[i],rows[rank])]
  rank+=1
 answers=[]
 for row in rows:
  if not any(row[:-1]):assert row[-1]==0
  if not any(row[:n-1]) and row[n-1]:answers.append(row[n]*pow(row[n-1],-1,r)%r)
 assert len(set(answers))<=1
 return (answers[0] if answers else None),rank
summary={'models':len(x['cases']),'arms':{},'controls':[],'equations_equal':True}
for arm in ['eager','candidate']:
 stats=collections.Counter();cycles=terminal=prefixes=0;primes=[]
 for case in x['cases']:
  v=case[arm];r=v['r'];assert r>2 and all(r%d for d in range(2,math.isqrt(r)+1));primes.append(r);es=[]
  for row in v['rows']:
   g=row['graph'];stats[g['status']]+=1;cycles+=g.get('cycles_delta',0);terminal+=g.get('answer') is not None
   if g['status']=='inserted':
    e=g['equation'];es.append((e['nodes'],e['signs'],e['a'],e['b']));answer,rank=solve(es,g['nodes'],r);assert answer==g['answer']==g['dense_answer'];assert g['factor_sum_verified'];assert rank==len(es);prefixes+=1
 summary['arms'][arm]={'states':sum(stats.values()),'statuses':dict(stats),'cycles':cycles,'terminals':terminal,'independent_dense_prefixes':prefixes,'prime_moduli_checked':len(primes)}
for case in x['cases']:
 def equations(arm):return [z['graph']['equation'] for z in case[arm]['rows'] if z['graph']['status']=='inserted']
 assert equations('eager')==equations('candidate')
for c in x['controls']:
 es=[]
 for row in c['rows']:
  u,su,v,sv,a,b=row['equation'];es.append(([u,v],[su,sv],a,b));answer,rank=solve(es,4,101);assert answer==row['answer']==row['dense_answer']
 assert answer==7;assert len(es)==c['stop_prefix']
 summary['controls'].append({'name':c['name'],'stop_prefix':len(es),'terminal':answer,'independent_rank':rank})
summary['raw_sha256']=hashlib.sha256((S/'run.stdout').read_bytes()).hexdigest();(D/'independent-counts.json').write_text(json.dumps(summary,indent=2));print(json.dumps(summary,indent=2))
