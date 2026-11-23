from pathlib import Path
import json,hashlib,sys,time,resource
START=time.monotonic();P=Path(__file__).resolve().parent
def mul(a,b):
 r=0
 while b:
  if b&1:r^=a
  b>>=1;a<<=1
  if a&256:a^=283
 return r

def main():
 out={'cases':[],'qualified_speedup':None};error=None
 try:
  from research.prime_orbit_roots_20261116_v71.field import Prime
  f=Prime(257);assert all(257%d for d in range(2,17))
  for binary in [False,True]:
   for k in range(1,9):
    sq=(lambda x:mul(x,x))if binary else(lambda x:x*x%257)
    frontier=[(1,)]
    for _ in range(k):
     fresh=[]
     for suffix in frontier:
      y=suffix[0]
      if binary:
       r=y
       for j in range(7):r=sq(r)
       predecessors=[r]
      else:
       r=f.sqrt(y);predecessors=[]if r is None else sorted({r,(-r)%257})
      fresh.extend((x,)+suffix for x in predecessors)
     frontier=fresh
    roots=sorted(frontier);expected=[]
    for x in range(256 if binary else 257):
     t=[x]
     for _ in range(k):t.append(sq(t[-1]))
     if t[-1]==1:expected.append(tuple(t))
    assert roots==expected and len(roots)==(1 if binary else 1<<k)
    out['cases'].append({'binary':binary,'k':k,'graph_treewidth':1,'elimination_degree':1<<k,'roots':roots,'complete':True,'exact_root_count':len(roots)})
 except Exception as e:error=type(e).__name__+': '+str(e)
 raw=json.dumps({'out':out,'error':error},separators=(',',':')).encode();assert time.monotonic()-START<75
 with(P/'raw.json').open('xb')as f:f.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if error is None else'inconclusive','summary':'Independent complete oracle for path-width-one squaring chains in odd and binary fields.','evidence':{'error':error,'cases':len(out['cases']),'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False,'worker_seconds':time.monotonic()-START},'next_actions':['Profile genuinely linearized blocks and algebraic degree on original source chains before promoting sparse structure.']};text=json.dumps(result);assert time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
