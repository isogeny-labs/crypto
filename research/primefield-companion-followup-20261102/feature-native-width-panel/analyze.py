"""Strict deterministic panel shape/control checker. No elliptic arithmetic here."""
import json,sys
from pathlib import Path
ARMS=['old','planned','separate','aa','positive'];PHASES=['initial','doubled']
CASES=[(r,m,c) for r,m in [(281,35),(1009,63),(2017,63),(2521,315)] for c in [1,2,7,16,64]]
NAMES=['scalar_products','polynomial_products','eea_inverse_calls','pow_r_minus_2_calls','alloc_calls','zeroed_calls','realloc_calls','requested_bytes','allocation_failures','dealloc_calls']
def integer(x):return type(x)is int and 0<=x<2**64
def unique(pairs):
 d={}
 for k,v in pairs:
  if k in d:raise ValueError('duplicate JSON key')
  d[k]=v
 return d
def analyze(rows,cases=CASES):
 expected=[(i,*cases[i],ARMS[(j+i)%5],phase)for i in range(len(cases))for j in range(5)for phase in PHASES]
 if len(rows)!=len(expected):raise ValueError('row count')
 lookup={}
 for row,key in zip(rows,expected):
  if set(row)!={'case','r','m','cap','arm','phase','values','counts','overflow'}:raise ValueError('row keys')
  if any(not integer(row[k])for k in ['case','r','m','cap']):raise ValueError('identity type')
  actual=tuple(row[k]for k in ['case','r','m','cap','arm','phase'])
  if actual!=key:raise ValueError('row order or identity')
  if row['overflow'] is not False:raise ValueError('overflow')
  counts=row['counts'];v=row['values'];r=row['r'];m=row['m']*(1 if row['phase']=='initial' else 2)
  size=sum(((r-1)//2-e)//m+1 for e in range(3))
  if type(counts)is not list or len(counts)!=10 or any(not integer(x)for x in counts)or counts[8]!=0:raise ValueError('counter domain or allocation failure')
  if type(v)is not list or len(v)!=size or any(not integer(x)or x>=r for x in v):raise ValueError('coefficient domain/length')
  lookup[(row['case'],row['arm'],row['phase'])]=row
 deltas=[]
 for i in range(len(cases)):
  for phase in PHASES:
   base=lookup[i,'old',phase]
   for arm in ARMS[1:]:
    row=lookup[i,arm,phase]
    if row['values']!=base['values']:raise ValueError('coefficient mismatch')
    ds=[a-b for a,b in zip(row['counts'],base['counts'])]
    if arm=='aa' and any(ds):raise ValueError('AA mismatch')
    if arm=='positive' and ds!=[1]+[0]*9:raise ValueError('positive control')
    if arm in ['planned','separate']:deltas.append(dict(case=i,phase=phase,arm=arm,delta=dict(zip(NAMES,ds))))
 return {'status':'controls_and_shape_pass','rows':len(rows),'deltas':deltas,'scope':'Interval engine only; coefficient truth requires pinned native oracle and producer receipt.'}
if __name__=='__main__':
 rows=[json.loads(line,object_pairs_hook=unique)for line in Path(sys.argv[1]).read_text().splitlines()]
 result=analyze(rows)
 with Path(sys.argv[2]).open('x')as f:json.dump(result,f,indent=2);f.write('\n')
