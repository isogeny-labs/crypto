from pathlib import Path
from itertools import combinations_with_replacement
import json,time,hashlib,random
from .kernel import mul,divrem,polynomial,infinity_ok,code,echelon,solve
from .common import P,finish
from research.image_cells_20261115_v67.algebra import BField,roots_poly,supported
START=time.monotonic()
def main():
 out={'tables':[],'records':[],'positive_controls':0,'local_oracles':0,'qualified_speedup':None};error=None
 try:
  d=json.loads((P/'DATA.json').read_text());M=d['modulus'];rng=random.Random(d['seed']);tables=[]
  for s,m in d['residue_fields']:
   f=BField(s,m);q=1<<s
   assert all(any(f.mul(a,b)==1 for b in range(1,q))for a in range(1,q))
   allowed={sum(v<<(s*i)for i,v in enumerate(roots_poly(xs,f)[:-1]))for xs in combinations_with_replacement(range(q),5)}
   assert len(allowed)==__import__('math').comb(q+4,5)
   test=list(range(1<<(5*s)))if s<=2 else[rng.randrange(1<<(5*s))for _ in range(64)]
   for v in test:
    H=[v>>(s*i)&(q-1)for i in range(5)]+[1];assert(v in allowed)==supported(H,range(q),f)['supported'];out['local_oracles']+=1
   tables.append((s,m,allowed));out['tables'].append({'degree':s,'modulus':m,'allowed_count':len(allowed),'allowed_sha256':hashlib.sha256(json.dumps(sorted(allowed)).encode()).hexdigest()})
  prior=json.load(open(d['records_input']))['out']['actual_records'];records=[r for r in prior if(r['name'].startswith(('relaxed_','fresh_'))and int(r['name'].split('_')[-1])<16)or r['scope']=='constructed point fixture'];assert len(records)==36
  for i in range(d['positive_cases']):
   xs=[rng.randrange(1<<d['dimension'])for _ in range(5)]
   if i%4==0:xs=[xs[0]]*5
   if i%4==1:xs[0]=0
   exact=polynomial(xs);H=[divrem(a,M)[1]for a in exact];lift=divrem(exact[0]^H[0],M);assert lift[1]==0 and lift[0]<8 and all(exact[j]==H[j]for j in range(1,6));records.append({'name':'positive_'+str(i),'H':H,'support_truth':True,'true_lift':lift[0]})
  for r in records:
   stages=[0]*(2+len(tables));survivors=[]
   for lift in range(8):
    c=list(r['H']);c[0]^=mul(lift,M)
    if not infinity_ok(c,d['dimension']-1):continue
    stages[0]+=1
    for j,(s,m,allowed)in enumerate(tables):
     if code(c,s,m)not in allowed:break
     stages[j+1]+=1
    else:survivors.append(lift)
   if r['support_truth']:assert survivors
   if 'true_lift'in r:assert r['true_lift']in survivors;out['positive_controls']+=1
   out['records'].append({'name':r['name'],'supported_truth':r['support_truth'],'stage_surviving_lifts':stages[:-1],'final_lifts':survivors})
  rows=[[int(x,16)for x in s.split()]for s in Path(d['matrix']).read_text().splitlines()[1:]];base=rows[0];cols=rows[1:];n=len(cols);assert n==73 and len(echelon([sum(x<<(83*i)for i,x in enumerate(v))for v in cols]))==73;flags=[(3,34),(2,51),(1,68)];constraints=[]
  for i,k in flags:constraints.append((sum(((v[i]>>k)&1)<<j for j,v in enumerate(cols)),(base[i]>>k)&1))
  branches=[]
  for pattern in [(0,0,0),(1,1,0),(0,0,1)]:
   free=solve([(a,b^t)for(a,b),t in zip(constraints,pattern)],n);branches.append({'top_degree_count':{(0,0,0):1,(1,1,0):3,(0,0,1):5}[pattern],'flags':pattern,'free_H_bits':free,'H_count':0 if free is None else 1<<free})
  out['leading_branches']=branches;out['original_H_count']=1<<n;out['leading_surviving_H_count']=sum(b['H_count']for b in branches);out['leading_surviving_lift_pairs']=sum(b['H_count']*(4 if b['top_degree_count']==5 else 2)for b in branches)
  joint=[0]*(n+3);offset=0;profile=[]
  for s,m,allowed in tables:
   for j,v in enumerate(cols):joint[j]^=code(v,s,m)<<offset
   for j in range(3):joint[n+j]^=divrem(mul(1<<j,M),m)[1]<<offset
   offset+=5*s;profile.append({'residue_modulus':m,'ambient_bits':offset,'linear_projection_rank':len(echelon(joint)),'syndrome_bits':offset-len(echelon(joint))})
  out['joint_projection']=profile;out['complete']=True
 except Exception as e:error=repr(e)
 finish(out,error,START,'Necessary integer-slope/residue splitting sieve with globally coupled8lifts,positive oracles and actual73parameter linear profiles.','Use an exact table-XOR solver or certified variable elimination; standard birthday filters do not certify all roots. Keep residue constraints necessary until full polynomial factor/support verification.')
if __name__=='__main__':main()
