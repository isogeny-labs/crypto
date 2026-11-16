from pathlib import Path
import itertools,json,hashlib,resource,sys,time,collections
from rootfinder.binaryfield import BinaryField
from research.root_conditioned_fibers_20261115_v45 import solver as s
from research.certified_support_20261115_v44 import solver as prior
from research.k0_reciprocal_cells_20261115_v38.solver import coefficients
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()

def controls():
 systems=states=positive=zero=0;profiles=[]
 for n,mod in [(3,11),(4,19)]:
  f=BinaryField(n,mod);cc=next(c for c in range(f.order)if (f.square(c)^c)>=4);z=f.square(cc)^cc;y=0
  for c in [cc,cc^1]:
   ctx=dict(status='ready',z=z,y=y,c=c,d=f.mul(z,c));hs=[h for h in range(f.order)if f.square(h)^h^z<4]
   widths=tuple(min(n,j+1)for j in [2,3,4]);C=s.b.Context(f,z,ctx['d'],hs,[0],widths);C.prepare_blocks()
   supported={tuple(coefficients(f,rs)):rs for rs in itertools.combinations_with_replacement(range(4),5)}
   for h in hs:
    fiber=C.block(h,0);assert fiber is not None;base,cols=s.model(f,ctx,h,fiber);mask=f.order-1
    explicit=[]
    for u,v in itertools.product(range(f.order),repeat=2):
     H=s.c.norm_coefficients(f,ctx,h,u,v)
     if all(H[3-i]<1<<widths[i]for i in range(3)):explicit.append(u|(v<<n))
    generated=[fiber['particular']^s.b.apply(fiber['basis'],t)for t in range(1<<len(cols))];assert sorted(explicit)==sorted(generated);states+=len(generated)
    pieces=set()
    for x in range(4):
     sol=s.condition(f,base,cols,x);got=s.parameters(sol);truth=[]
     for t,uv in enumerate(generated):
      H=s.c.norm_coefficients(f,ctx,h,uv&mask,uv>>n)
      if s.evaluate(f,H,x)==0:truth.append(t)
     assert sorted(got)==truth;systems+=1;zero+=x==0
     for t in got:
      uv=generated[t];H=tuple(s.c.norm_coefficients(f,ctx,h,uv&mask,uv>>n))
      if H in supported and x==min(supported[H]):pieces.add(t)
    want={t for t,uv in enumerate(generated)if tuple(s.c.norm_coefficients(f,ctx,h,uv&mask,uv>>n))in supported};assert pieces==want;positive+=len(want)
   profiles.append(dict(n=n,c=c,h_count=len(hs)))
 return dict(complete_conditioned_cells=systems,affine_states=states,positive_supported_parameters=positive,zero_root_cells=zero,profiles=profiles)

def actual():
 rule=json.loads((P/'DATA.json').read_text());f=BinaryField(83,prior.MOD);ctx=s.c.compile_context(f,0,prior.Z,prior.QY^prior.Z);cover=json.loads((R/rule['centroid_receipt']).read_text())['out']['actual']['domain'];hs=[cover['particular']]
 for i in range(1,rule['h_count']):hs.append(hs[-1]^cover['basis'][(i&-i).bit_length()-1])
 C=s.b.Context(f,prior.Z,ctx['d'],hs,[0],(35,52,69));C.prepare_blocks();records=[];rank=collections.Counter();compatible=0
 for h in hs:
  fiber=C.block(h,0);assert len(fiber['basis'])==73;base,cols=s.model(f,ctx,h,fiber)
  for x in rule['roots']:
   values=[s.evaluate(f,H,x)for H in cols];r=s.b.solve(s.b.rows_from(values,0,83),73)['rank'];sol=s.condition(f,base,cols,x);rank[r]+=1;compatible+=sol is not None
   records.append(dict(h=h,x=x,rank=r,consistent=sol is not None,nullity=73-r,particular=None if sol is None else sol['particular']))
   assert time.monotonic()-START<70
 return dict(records=records,rank_histogram=dict(rank),compatible_cells=compatible,cells=len(records),input_dimension=73,scope='Sampled root-conditioned k0 norm cells only; global h/x census and original chain remain open')

def main():
 out={};error=None
 try:out['controls']=controls();out['actual']=actual()
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 trial=json.dumps(dict(out=out,error=error),separators=(',',':'))
 if time.monotonic()-START>=78:error='late serialized result'
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as f:f.write(raw)
 ev=dict(error=error,controls=out.get('controls'),actual={k:v for k,v in out.get('actual',{}).items()if k!='records'},worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),qualified_speedup=None,complete_global_root_search=False,release_gate_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Exact whole-cell root conditioning with independent complete small-fiber oracle.',evidence=ev,next_actions=['If ranks high, freeze a bounded full-x native conditional search for one h; if low, exploit the residual kernel rather than assume injectivity.'])
 text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
