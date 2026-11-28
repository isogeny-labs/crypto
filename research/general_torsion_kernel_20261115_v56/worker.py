from pathlib import Path
import hashlib,itertools,json,resource,sys,time
from research.binary_wide_20261115_v41.backend import WideBinaryField
from research.norm_block_reuse_20261115_v36 import backend as linear
from research.original_chain_lift_20261115_v55.worker import literal
P=Path(__file__).resolve().parent;START=time.monotonic()
def kernel(f,d,B):
 beta=B
 for _ in range(f.degree-1):beta=f.square(beta)
 assert literal(beta,beta,f.modulus)==B
 w=3*d-2;assert w<f.degree;cols=[f.square(1<<i)^f.mul(beta,1<<i)for i in range(w)]+[f.mul(B,f.square(1<<i))for i in range(d)]
 independent=[literal(1<<i,1<<i,f.modulus)^literal(beta,1<<i,f.modulus)for i in range(w)]+[literal(B,literal(1<<i,1<<i,f.modulus),f.modulus)for i in range(d)];assert cols==independent
 sol=linear.solve(linear.rows_from(cols,0,f.degree),len(cols));piv={}
 for v in independent:
  while v:
   p=(v&-v).bit_length()-1
   if p in piv:v^=piv[p]
   else:piv[p]=v;break
 assert len(piv)==sol['rank']
 for v in sol['basis']:assert linear.apply(cols,v)==0
 return beta,w,cols,sol

def main():
 out={'controls':[],'profiles':[]};error=None
 try:
  data=json.loads((P/'DATA.json').read_text())
  for n,mod,d,B in data['tiny']:
   f=WideBinaryField(n,mod);beta,w,cols,sol=kernel(f,d,B);models={linear.apply(sol['basis'],v)for v in range(1<<len(sol['basis']))};truth=set();mul=lambda a,b:literal(a,b,mod)
   for D,S in itertools.product(range(1<<w),range(1<<d)):
    if not(mul(D,D)^mul(beta,D)^mul(B,mul(S,S))):truth.add(D|(S<<w))
   assert models==truth;triples=0;satisfying=0
   for a,b,c in itertools.product(range(1,1<<d),repeat=3):
    D=mul(mul(a,b),c);S=a^b^c;x=mul(beta,f.inv(c));v=mul(a,b)^mul(a,x)^mul(b,x);direct=not(mul(v,v)^mul(mul(a,b),x)^B);packed=D|(S<<w);assert direct==(packed in models);triples+=1;satisfying+=direct
   out['controls'].append({'n':n,'d':d,'B':B,'DS_states':(1<<w)*(1<<d),'nonzero_triples':triples,'satisfying':satisfying,'kernel_dimension':len(sol['basis'])})
  for case in data['profiles']:
   n=case['n'];mod=sum(1<<i for i in case['modulus_exponents']);f=WideBinaryField(n,mod);d=case['d']
   for B in [1,(1<<(n-8))^0xabcdef010203]:
    start=time.monotonic();beta,w,cols,sol=kernel(f,d,B);dim=len(sol['basis']);out['profiles'].append({'n':n,'modulus':mod,'d':d,'B':B,'beta':beta,'D_width':w,'S_width':d,'matrix_rank':sol['rank'],'kernel_dimension':dim,'kernel_basis':sol['basis'],'columns_sha256':hashlib.sha256(json.dumps(cols,separators=(',',':')).encode()).hexdigest(),'zero_kernel_nonzero_triple_P0_excluded':dim==0,'nonzero_triple_P0_exact_count':0 if dim==0 else None,'seconds':time.monotonic()-start,'scope':'exact auxiliary torsion-triple necessary-condition kernel; no curve-performance or global chain coverage claim'});assert time.monotonic()-START<65
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 trial=json.dumps({'out':out,'error':error},separators=(',',':'))
 if time.monotonic()-START>=72:error='late serialized output'
 if error:
  for p in out['profiles']:p['zero_kernel_nonzero_triple_P0_excluded']=False;p['nonzero_triple_P0_exact_count']=None
 raw=json.dumps({'out':out,'error':error},sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as file:file.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if not error else'inconclusive','summary':'General-B finite binary torsion-triple kernel across six extension fields, with independent arithmetic and rank oracles.','evidence':{'error':error,'controls':out['controls'],'profiles':[{k:v for k,v in p.items()if k!='kernel_basis'}for p in out['profiles']],'worker_seconds':time.monotonic()-START,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False},'next_actions':['Use zero-kernel certificates only for the explicit field,B,domain; build independent tiny group/Frobenius oracles before global original-to-norm promotion.']};text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<80;print(text)
if __name__=='__main__':main()
