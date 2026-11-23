from pathlib import Path
from itertools import product
import collections,hashlib,json,resource,sys,time
from rootfinder.binaryfield import BinaryField,binary_field_roots
from .solver import Lift
P=Path(__file__).resolve().parent;START=time.monotonic()
def literal(a,b,mod):
 v=0
 while b:
  if b&1:v^=a
  b>>=1;a<<=1
 while v.bit_length()>=mod.bit_length():v^=mod<<(v.bit_length()-mod.bit_length())
 return v
def eq(a,b,c,B,mod):
 mul=lambda x,y:literal(x,y,mod);t=mul(a,b)^mul(a,c)^mul(b,c)
 return mul(t,t)^mul(mul(a,b),c)^B

def main():
 out={'tiny':[],'actual':[],'complete_conditional':False};error=None
 try:
  data=json.loads((P/'DATA.json').read_text())
  for n,mod,B,domain in data['tiny']:
   f=BinaryField(n,mod);L=Lift(f,B);q=f.order
   table={(a,b):tuple(c for c in range(q)if eq(a,b,c,B,mod)==0)for a in range(q)for b in range(q)}
   for (a,b),roots in table.items():assert L.roots(a,b)==roots,(n,B,a,b)
   oracle={t:[]for t in range(q)};got={t:[]for t in range(q)}
   for xs in product(domain,repeat=5):
    a,b,c,d,e=xs
    for u in table[a,b]:
     for v in table[u,c]:
      for w in table[v,d]:
       for target in table[w,e]:oracle[target].append(xs+(u,v,w))
    for target in range(q):got[target].extend(L.ordered(xs,target))
   for target in range(q):assert sorted(got[target])==sorted(oracle[target]),(n,B,target)
   bag_checks=0
   for bag in sorted(set(tuple(sorted(xs))for xs in product(domain,repeat=5))):
    counts=sorted(collections.Counter(bag).items())
    for target in range(q):
     expected=sorted(v for v in oracle[target]if tuple(sorted(v[:5]))==bag)
     assert L.bag(counts,target)==expected;bag_checks+=1
   record={'n':n,'modulus':mod,'B':B,'domain':domain,'local_pairs':q*q,'ordered_sources':len(domain)**5,'targets':q,'complete_original_tuples':sum(map(len,oracle.values())),'tuple_sha256':hashlib.sha256(json.dumps(oracle,sort_keys=True).encode()).hexdigest(),'bag_target_comparisons':bag_checks,'branches':L.branches};out['tiny'].append(record)
   assert time.monotonic()-START<65
  witness=json.loads(Path(data['witness_validation']).read_text())['out']['cases']
  for index,case in enumerate(data['actual']):
   f=BinaryField(case['n'],case['modulus']);L=Lift(f,case['B']);values=L.bag(case['roots'],case['target']);assert values
   for v in values:
    a,b,c,d,e,u,vv,w=v
    assert not any(eq(x,y,z,case['B'],case['modulus'])for x,y,z in [(a,b,u),(u,c,vv),(vv,d,w),(w,e,case['target'])])
   W=witness[index];assert tuple(W['original_field_variables'])in values
   checks=[]
   pairs=sorted(L.cache)
   chosen=list(dict.fromkeys(pairs[:4]+[p for p in pairs if p[0]==p[1]][:2]+[p for p in pairs if not p[0]][:2]+pairs[-4:]))
   for a,b in chosen:
    ab=f.mul(a,b);poly=[f.square(ab)^case['B'],ab,f.square(a^b)];oracle=binary_field_roots(poly,f,seed=55,max_trials=64)
    assert oracle['status']=='complete'and tuple(oracle['roots'])==L.roots(a,b)
    checks.append({'a':a,'b':b,'roots':oracle['roots'],'restricted_degree':oracle['restricted_degree']})
   body={'name':case['name'],'contract':'every finite original chain root for supplied source multiset','complete':True,'tuples':values,'packed_bits_hex':[hex(sum(x<<(case['d']*i)for i,x in enumerate(v[:5]))+sum(x<<(5*case['d']+case['n']*i)for i,x in enumerate(v[5:])))for v in values],'independent_polynomial_checks':checks,'branches':L.branches,'cached_pairs':len(L.cache)}
   raw=json.dumps(body,sort_keys=True,separators=(',',':')).encode();(P/(case['name']+'.json')).write_bytes(raw)
   out['actual'].append({'name':case['name'],'original_tuples':len(values),'source_orders':len(set(v[:5]for v in values)),'cached_pairs':len(L.cache),'branches':L.branches,'raw_sha256':hashlib.sha256(raw).hexdigest()});assert time.monotonic()-START<72
  out['complete_conditional']=True
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 trial=json.dumps({'out':out,'error':error},separators=(',',':'))
 if time.monotonic()-START>=78:error='late serialized result';out['complete_conditional']=False
 if error:
  out['complete_conditional']=False
  for row in out['actual']:
   row['verified_tuple_lower_bound']=row.pop('original_tuples',0);row['original_tuples']=None
 raw=json.dumps({'out':out,'error':error},sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as file:file.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if not error else'inconclusive','summary':'Exact finite original S3-chain reconstruction conditional on source bags, with independent small all-roots oracles.','evidence':{'error':error,'tiny':out['tiny'],'actual':out['actual'],'complete_conditional':out['complete_conditional'],'worker_seconds':time.monotonic()-START,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'complete_global_root_search':False,'qualified_speedup':None,'release_gate_claim':False},'next_actions':['Integrate exact original-chain reconstruction in timed conditional pipeline; separately prove global norm chart coverage including exceptional intermediate branches.']}
 text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
