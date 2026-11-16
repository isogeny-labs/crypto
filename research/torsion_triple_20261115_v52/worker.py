from pathlib import Path
import hashlib,itertools,json,resource,sys,time
from rootfinder.binaryfield import BinaryField
from research.norm_block_reuse_20261115_v36 import backend as b
P=Path(__file__).resolve().parent;START=time.monotonic()

def literal_square(v,mod):
 out=0
 while v:
  low=v&-v;i=low.bit_length()-1;out^=1<<(2*i);v^=low
 while out.bit_length()>=mod.bit_length():out^=mod<<(out.bit_length()-mod.bit_length())
 return out

def kernel(f,d):
 Dwidth=3*(d-1)+1;cols=[f.square(1<<i)^(1<<i)for i in range(Dwidth)]+[f.square(1<<i)for i in range(d)]
 independent=[literal_square(1<<i,f.modulus)^(1<<i)for i in range(Dwidth)]+[literal_square(1<<i,f.modulus)for i in range(d)];assert cols==independent
 sol=b.solve(b.rows_from(cols,0,f.degree),len(cols));piv={}
 for v in independent:
  while v:
   p=(v&-v).bit_length()-1
   if p in piv:v^=piv[p]
   else:piv[p]=v;break
 assert len(piv)==sol['rank'];return Dwidth,sol,cols

def S3(f,a,bb,c):return f.square(f.mul(a,bb)^f.mul(a,c)^f.mul(bb,c))^f.mul(f.mul(a,bb),c)^1

def controls():
 states=triples=zeroO=zeroP0=0;profiles=[]
 for n,mod,d in [(7,131,2),(11,2053,3)]:
  f=BinaryField(n,mod);w,sol,cols=kernel(f,d);assert len(sol['basis'])<=12;models={b.apply(sol['basis'],v)for v in range(1<<len(sol['basis']))};truth={D|(S<<w)for D in range(1<<w)for S in range(1<<d)if f.square(D)^D^f.square(S)==0};assert models==truth;states+=(1<<w)*(1<<d);counts=[0,0]
  for a,bb,c in itertools.product(range(1,1<<d),repeat=3):
   D=f.mul(f.mul(a,bb),c);S=a^bb^c;direct=S3(f,a,bb,f.inv(c))==0;linear=f.square(D)^D^f.square(S)==0;assert direct==linear and linear==((D|(S<<w))in models);triples+=1;counts[1]+=direct;counts[0]+=S3(f,a,bb,c)==0
  assert counts[0]==0;zeroO+=counts[0];zeroP0+=counts[1];profiles.append(dict(n=n,modulus=mod,d=d,kernel_dimension=len(sol['basis']),triples_O=counts[0],triples_P0=counts[1]))
 return dict(literal_product_sum_states=states,complete_nonzero_triples=triples,profiles=profiles)

def actual():
 f=BinaryField(83,0x800000000200000000007);d=18;w,sol,cols=kernel(f,d);expected=[(1<<(2*i))|(((1<<(2*i))^(1<<i))<<w)for i in range(9)];assert all(b.apply(cols,v)==0 for v in expected)
 equal=len(sol['basis'])==9;forbidden=0
 if equal:
  for tag in [b.apply(expected,v)for v in range(512)]:
   D=tag&((1<<w)-1);S=tag>>w
   # Nonzero product: deg(sum)<=maxdeg<=sumdeg, equality forces two factors1.
   impossible=(D==0)or(D==1 and S!=1)or(D>1 and D.bit_length()==S.bit_length()and D!=S)
   assert impossible;forbidden+=1
 z=0x622425e96ba633969263b;inv=f.inv(z);assert f.mul(z,inv)==1
 return dict(n=83,modulus=f.modulus,d=d,D_width=w,S_width=d,matrix_rank=sol['rank'],kernel_dimension=len(sol['basis']),kernel_basis=sol['basis'],expected_low_square_basis=expected,kernel_matches_low_square_prediction=equal,forbidden_product_sum_states=forbidden,nonzero_triple_P0_excluded=equal,nonzero_triple_P0_exact_count=0 if equal else None,S3_O_polynomial_degree_bound=68,field_degree=83,target_z=z,target_plus_P0_x=inv,target_plus_P0_outside_domain=inv>=1<<18,scope='Exact torsion-triple algebraic exclusion only; full norm/source-chain integration remains open')

def main():
 out={};error=None
 try:out['controls']=controls();out['actual']=actual()
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 trial=json.dumps(dict(out=out,error=error),separators=(',',':'))
 if time.monotonic()-START>=18:error='late serialized output';out.get('actual',{})['nonzero_triple_P0_excluded']=False;out.get('actual',{})['nonzero_triple_P0_exact_count']=None
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as f:f.write(raw)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Independent rank/product-sum certificate for actual binary torsion triples.',evidence=dict(error=error,controls=out.get('controls'),actual=out.get('actual'),worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),qualified_speedup=None,complete_global_root_search=False,release_gate_claim=False),next_actions=['If kernel prediction holds, review ghost-partition norm-coverage proof and independent tiny group oracles; otherwise retain/factor every extra product state under a fresh bound.'])
 text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<24;print(text)
if __name__=='__main__':main()
