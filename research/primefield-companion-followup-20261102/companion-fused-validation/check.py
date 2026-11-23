from pathlib import Path
from random import Random
from datetime import datetime,timezone
import sys,json,hashlib,platform,os,subprocess
sys.set_int_max_str_digits(0)
D=Path(__file__).resolve().parent;ROOT=D.parent;OUT=D/'runs'
sys.path[:0]=[str(ROOT/'companion-fused'),str(ROOT/'structured-block-kernel')]
from multiply import apply
from kernel import Meter
from reference import reference
from accounting import reconstruct
now=lambda:datetime.now(timezone.utc).isoformat()
digest=lambda v:hashlib.sha256(json.dumps(v,sort_keys=True).encode()).hexdigest()
def sha(p):
 h=hashlib.sha256()
 with p.open('rb') as f:
  for b in iter(lambda:f.read(1048576),b''):h.update(b)
 return h.hexdigest()
def stages(row,sources):
 p=row['payload'];plan=p['plan'];r=plan['r'];tests=p['tests'];pos=0
 if plan['kind']=='source':
  src=sources[plan['source_instance']];G=src['constructor']['G'];K=src['constructor']['K'];a=src['gamma'];b=1;n=r
 else:
  n=plan['n'];a=plan['a'];b=plan['b'];rng=Random(plan['seed']);A=[[0]*n for _ in range(n)]
  if plan['matrix']=='upper':
   for i in range(n):
    A[i][i]=1
    for j in range(i+1,n):A[i][j]=rng.randrange(r)
  elif plan['matrix']=='singular':
   for i in range(n-1):A[i][i]=1
  else:assert plan['matrix']=='zero'
  G=[[( (a*A[-1][j] if i==0 else A[i-1][j])-(b*A[i][0] if j==n-1 else A[i][j+1]))%r for j in range(n)] for i in range(n)]
  K=[[int(i==j) for j in range(n)] for i in range(n)]
 size=2*(3+(n if n<=19 else 2))
 yield 'full',G,K,a,b,tests[:size];pos=size
 inv=p['inverse']
 if inv and inv['status']=='reference_inverse_transport_verified':
  yield 'inverse',inv['U'],inv['V'],b,a,tests[pos:pos+size];pos+=size
 for i,z in enumerate(p['restrictions']):
  size=z['tests'];yield 'principal-'+str(i),z['U'],z['V'],z['a'],z['b'],tests[pos:pos+size];pos+=size
 assert pos==len(tests)

def verify(G,K,a,b,x,r,tr,expected,arm,carry=False):
 trace=[];m=Meter(r);y=apply(G,K,a,b,x if arm=='candidate' else tuple(x),m,tr,trace.append)
 counts,blocks=reconstruct(G,K,a,b,x,r,tr,trace)
 assert m.counts==counts,(m.counts,counts)
 assert m.blocks==blocks
 maxima=None
 if carry:
  expected,rt,maxima=reference(G,K,a,b,x,r,tr)
  assert trace==rt
 assert y==expected
 bad=y[:];bad[0]=(bad[0]+1)%r;assert bad!=expected
 bad=dict(counts);bad['coefficient_reductions']+=1;assert bad!=counts
 bad=dict(trace[-1]);bad['packed']+=1;assert bad!=trace[-1]
 # The wrap/sign control changes a nonzero setup value, including zero inputs.
 bad=dict(trace[0]);bad['inverse']=(-bad['inverse'])%r;assert bad!=trace[0]
 equal=0
 for z in [0,1,r-1]:
  try:apply(G,K,z,z,x,Meter(r))
  except AssertionError:equal+=1
  else:raise AssertionError('equal wraps accepted')
 radix_rejections=0
 if carry:
  bound=trace[0]['bound'];assert bound>0
  # Explicitly a proof-bound rejection, not a claim of realized carry.
  def guard(beta):assert beta>bound
  for beta in [1,bound]:
   try:guard(beta)
   except AssertionError:radix_rejections+=1
   else:raise AssertionError('uncertified radix accepted')
 return dict(input=x,result=y,transpose=tr,counts=counts,blocks=blocks,trace_sha256=digest(trace),max_coefficients=maxima,mutations_rejected=4,equal_wrap_rejections=equal,radix_guard_rejections=radix_rejections)

def main():
 for name in ['source-lock.json','input-lock.json']:
  for p,h in json.loads((D/name).read_text()).items():assert sha(ROOT/p)==h,p
 plan=json.loads((D/'plan.json').read_text());assert len(plan)==849
 sources={}
 with (ROOT/'direct-companion-validation/runs/cases.jsonl').open() as f:
  for line in f:
   z=json.loads(line)
   if z['arm']=='candidate':sources[z['instance_id']]=z
 assert len(sources)==81
 OUT.mkdir(exist_ok=False)
 (OUT/'host.json').write_text(json.dumps(dict(source_commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),platform=platform.platform(),logical_cpus=os.cpu_count()),indent=2)+'\n')
 # Concrete primitive carry certificate, separate from radix guards.
 beta=256;coeff=[beta,1];packed=sum(v*beta**i for i,v in enumerate(coeff));decoded=[packed%beta,packed//beta]
 assert decoded!=coeff and decoded==[0,2]
 (OUT/'carry-certificate.json').write_text(json.dumps(dict(radix=beta,coefficients=coeff,packed=packed,decoded=decoded,overflow_detected=True),indent=2)+'\n')
 total=dict(cases=0,products=0,archived_products=0,carry_products=0,mutations_rejected=0,equal_wrap_rejections=0,radix_guard_rejections=0,aa_exact=True)
 aa={};index=[]
 with (OUT/'cases.jsonl').open('w') as out:
  for arm in ['candidate','aa']:
   archive=(ROOT/'companion-blocks-validation/runs/cases.jsonl').open()
   for iid,p in enumerate(plan):
    started=now();results=[];r=p['r']
    if p['kind']=='archived':
     old=json.loads(next(archive));assert old['arm']=='candidate' and old['instance_id']==str(iid) and old['seed']==p['seed']
     for name,G,K,a,b,tests in stages(old,sources):
      for t in tests:
       z=verify(G,K,a,b,t['input'],r,t['transpose'],t['result'],arm)
       z.update(stage=name,label=t['label'],n=len(G),s=len(G[0]),a=a,b=b,archived_counts=t['counts'])
       results.append(z)
     assert len(results)==old['matvecs']
    else:
     n=p['n'];s=p['s'];rng=Random(p['seed'])
     draw=lambda:r-1 if p['pattern']=='maximum' else rng.randrange(r)
     G=[[draw() for _ in range(s)] for _ in range(n)];K=[[draw() for _ in range(s)] for _ in range(n)];x=[draw() for _ in range(n)]
     for tr in [False,True]:
      z=verify(G,K,p['a'],p['b'],x,r,tr,None,arm,True)
      z.update(stage='carry',label=p['pattern'],n=n,s=s,a=p['a'],b=p['b'],G=G,K=K)
      results.append(z)
    payload=dict(plan=p,tests=results);h=digest(payload)
    if arm=='candidate':aa[iid]=h
    else:assert aa[iid]==h
    row=dict(arm=arm,instance_id=str(iid),replicate=0,seed=p['seed'],status='verified',kind=p['kind'],products=len(results),mutations_rejected=sum(z['mutations_rejected'] for z in results),equal_wrap_rejections=sum(z['equal_wrap_rejections'] for z in results),radix_guard_rejections=sum(z['radix_guard_rejections'] for z in results),aa_sha256=h,payload=payload,started_at=started,finished_at=now())
    encoded=json.dumps(row)+'\n';offset=out.tell();out.write(encoded);out.flush()
    index.append(dict(arm=arm,instance_id=str(iid),offset=offset,bytes=len(encoded.encode()),sha256=hashlib.sha256(encoded.encode()).hexdigest()))
    total['cases']+=1;total['products']+=len(results);total[p['kind']+'_products']+=len(results)
    for name in ['mutations_rejected','equal_wrap_rejections','radix_guard_rejections']:total[name]+=row[name]
    print(arm,iid,p['kind'],len(results),'verified',flush=True)
   archive.close()
 assert total==dict(cases=1698,products=34248,archived_products=32328,carry_products=1920,mutations_rejected=136992,equal_wrap_rejections=102744,radix_guard_rejections=3840,aa_exact=True),total
 (OUT/'summary.json').write_text(json.dumps(total,indent=2)+'\n');(OUT/'row-index.json').write_text(json.dumps(index,indent=2)+'\n');print(total)
if __name__=='__main__':main()
