"""Strict raw-panel parser; no cryptographic execution or timing inference."""
from pathlib import Path
import json,hashlib,argparse
D=Path(__file__).resolve().parent
ARMS=('baseline','candidate','aa')
def unique(pairs):
 d={}
 for k,v in pairs:
  if k in d:raise ValueError('duplicate JSON key')
  d[k]=v
 return d
def expected():
 return [list(map(int,l.split('\t')))for l in (D.parent/'feature-native-monomial-recovery/recovery.tsv').read_text().splitlines()[1:]]
def counts(c,fixed):
 assert type(c)is list and len(c)==14
 for s,row in enumerate(c):
  assert type(row)is list and len(row)==12
  for k,ops in enumerate(row):
   assert type(ops)is list and len(ops)==17
   for o,v in enumerate(ops):
    assert type(v)is int and 0<=v<2**64
    if s==0 or (fixed and s not in (1,2,3)) or (not fixed and s in (1,2,3)):assert v==0
    if 7<=o<=12 and k!=10:assert v==0
    if o>=13 and (s!=2 or k!=4):assert v==0
 return c
def analyze(rows):
 assert len(rows)==65
 assert rows[0]=={'schema':'monomial-matched/v1','targets':20,'shape':[14,12,17]}
 assert all(type(rows[-1].get(k))is bool for k in ('complete','aa_equal','forward_checked'))
 assert rows[-1]=={'complete':True,'queries':60,'aa_equal':True,'forward_checked':True}
 fixed={}
 for row,arm in zip(rows[1:4],ARMS):
  assert set(row)=={'kind','arm','label','counts'} and (row['kind'],row['arm'],row['label'])==('fixed',arm,-1)
  assert type(row['label'])is int
  fixed[arm]=counts(row['counts'],True)
 assert fixed['baseline']==fixed['aa']
 result=[]
 for i,f in enumerate(expected()):
  group={}
  for row,arm in zip(rows[4+3*i:7+3*i],ARMS):
   assert set(row)=={'kind','arm','label','point','result','counts'}
   assert (row['kind'],row['arm'],row['label'])==('query',arm,f[0])
   assert type(row['label'])is int
   assert row['point']==f[1:3] and row['result']==f[3:]
   assert all(type(v)is int for v in row['point']+row['result'])
   group[arm]=counts(row['counts'],False)
  assert group['baseline']==group['aa']
  per={}
  for arm in ARMS:
   # Each dimension remains a distinct helper/event category, not weighted cost.
   per[arm]=[sum(group[arm][s][k][o]+fixed[arm][s][k][o]for s in range(14)for k in range(12))for o in range(17)]
  result.append({'label':f[0],'single_target_events':per,'candidate_minus_baseline':[c-b for c,b in zip(per['candidate'],per['baseline'])]})
 return {'schema':'monomial-matched-analysis/v1','pairs':result,'aa_equal':True,'timing_claim':False,'primary_scope':'per-target helper/event dimensions, full fixed cost charged to each query; not machine cost'}
def main():
 p=argparse.ArgumentParser();p.add_argument('raw',type=Path);p.add_argument('--producer-receipt',type=Path,required=True);p.add_argument('--protocol-sha256',required=True);p.add_argument('--out',type=Path,required=True);a=p.parse_args()
 raw=a.raw.read_bytes();receipt=json.loads(a.producer_receipt.read_text(),object_pairs_hook=unique)
 assert receipt['exit_code']==0 and type(receipt['exit_code'])is int
 assert receipt['raw_sha256']==hashlib.sha256(raw).hexdigest() and receipt['protocol_sha256']==a.protocol_sha256
 assert len(a.protocol_sha256)==64 and all(c in '0123456789abcdef'for c in a.protocol_sha256)
 rows=[json.loads(l,object_pairs_hook=unique)for l in raw.splitlines()];out=analyze(rows);out['raw_sha256']=receipt['raw_sha256'];out['protocol_sha256']=a.protocol_sha256
 with a.out.open('x')as f:json.dump(out,f,indent=2);f.write('\n')
if __name__=='__main__':main()
