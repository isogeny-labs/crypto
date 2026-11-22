from pathlib import Path
import json,hashlib,resource,sys,time
from rootfinder.binaryfield import BinaryField
from research.root_constant_image_20261115_v47 import solver as s
from research.certified_support_20261115_v44 import solver as prior
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()

def compare(f,ctx,h,fiber,x,ys):
 base,cols=s.old.model(f,ctx,h,fiber);reference=set(s.old.parameters(s.old.condition(f,base,cols,x)));S=s.split(f,fiber);got=set()
 for y in ys:got.update(s.old.parameters(s.point(f,ctx,h,S,x,y)))
 assert got==reference
 return len(reference)

def run():
 tiny=positive=0
 for n,mod in [(2,7),(3,11),(4,19)]:
  f=BinaryField(n,mod);c=2;z=f.square(c)^c;B=f.mul(f.square(z),z);ctx=dict(status='ready',z=z,y=0,c=c,d=f.mul(z,c));fiber=dict(particular=0,basis=[1<<i for i in range(2*n)])
  for x in range(1,f.order):
   if x==z:continue
   ys=[y for y in range(f.order)if f.square(y)^f.mul(x,y)==f.mul(f.square(x),x)^B];positive+=compare(f,ctx,1,fiber,x,ys)>0;tiny+=1
 f=BinaryField(83,prior.MOD);ctx=s.old.c.compile_context(f,0,prior.Z,prior.QY^prior.Z);raw=json.loads((R/'research/root_conditioned_native_20261115_v45/raw.json').read_text())['out'];h=raw['h'];C=s.old.b.Context(f,prior.Z,ctx['d'],[h],[0],(35,52,69));C.prepare_blocks();fiber=C.block(h,0);S=s.split(f,fiber);assert len(S['U'])==35 and len(S['kernel'])==38
 roots=sorted(set(range(1,33))|{x for x,t in raw['native']['candidates']});cells=compatible=0;counts=[]
 for x in roots:
  inv=f.inv(x);a=x^f.square(inv);ht=0;v=a
  for _ in range(42):ht^=v;v=f.square(f.square(v))
  ys=[]
  if f.square(ht)^ht==a:
   y=f.mul(x,ht);ys=[y,y^x]
  count=compare(f,ctx,h,fiber,x,ys);counts.append([x,count]);compatible+=bool(count);cells+=1;assert time.monotonic()-START<18
 deltas=[[s.reduce(S,f.mul(1<<i,u))[0]for u in S['U']]for i in range(18)];constant=[s.reduce(S,v)[0]for v in S['V']]
 for x in roots:
  for j,(u,v)in enumerate(zip(S['U'],S['V'])):assert constant[j]^s.old.b.xor(deltas[i][j]for i in range(18)if x>>i&1)==s.reduce(S,f.mul(x,u)^v)[0]
 return dict(tiny_complete_root_cells=tiny,tiny_positive_cells=positive,actual_cells=cells,actual_compatible_cells=compatible,actual_counts=counts,u_directions=35,constant_v_directions=38,quotient_dimension=45,gray_increment_checks=len(roots)*35,scope='Exact sampled point/root fibers; no timed native implementation or new global norm closure')

def main():
 out={};error=None
 try:out=run()
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 trial=json.dumps(dict(out=out,error=error),separators=(',',':'))
 if time.monotonic()-START>=24:error='late serialized output'
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as f:f.write(raw)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Constant-image root conditioning and exact projected Gray increments.',evidence=dict(error=error,summary={k:v for k,v in out.items()if k!='actual_counts'},worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),qualified_speedup=None,complete_global_root_search=False,release_gate_claim=False),next_actions=['Implement native35x45 full scan with shared point table and compare paid cold calls against73x83 source-conditioned solver; retain both lifts and exceptional cells.'])
 text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<28;print(text)
if __name__=='__main__':main()
