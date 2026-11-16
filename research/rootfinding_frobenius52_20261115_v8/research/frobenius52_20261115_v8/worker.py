"""Pure-Frobenius U coordinates versus matched cached83-k component."""
from pathlib import Path
import sys,json,time,gzip,hashlib,resource,itertools,statistics
from rootfinder.binaryfield import BinaryField
from research.quartic_cell_20261114_v1 import worker as old
from research.regular_block_20261115_v2 import worker as rb
from research.regular_ore_20261115_v3 import worker as ore
from research.cubic_cached_20261115_v4 import worker as cw
P=Path(__file__).resolve().parent;ROOT=P.parents[1];START=time.monotonic();CHECKS=0

def check(v,why):
 global CHECKS
 CHECKS+=1
 assert CHECKS<=200000 and time.monotonic()-START<80,'check/time cap'
 assert v,why

def sqrt(f,v):
 for _ in range(f.degree-1):v=f.square(v)
 return v

def family(f,c,W,method,sqrtbasis):
 check(c['betaU']==0,'pure Frobenius identity')
 branch='k83';k0=0;dirs=[1<<i for i in range(f.degree)]
 if method=='U52'and c['alphaU']:
  branch='U_coordinates';eta=f.inv(sqrt(f,c['alphaU']));k0=f.mul(eta,sqrt(f,c['gammaU']));dirs=[f.mul(eta,v)for v in sqrtbasis]
 elif method=='U52'and old.residual(c['gammaU'],W[2]):return dict(branch='constant_U_outside',consistent=False,k_particular=None,k_basis=[],rows=[],coordinate_family=None,k0=0,k_directions=[])
 elif method=='U52':branch='constant_U_inside_fallback83'
 base=ore.qfast(f,c,k0);cols=[[x^v for x,v in zip(ore.qfast(f,c,k0^d),base)]for d in dirs]
 if branch=='U_coordinates':
  def pack(Q):return old.residual(Q[1],W[1])
  positions=[i for i in range(f.degree)if i not in W[1]]
 else:
  def pack(Q):return old.residual(Q[1],W[1])|(old.residual(Q[0],W[2])<<f.degree)
  positions=list(range(2*f.degree))
 rhs=pack(base);packed=[pack(v)for v in cols];rows=[(sum(((v>>j)&1)<<i for i,v in enumerate(packed)),(rhs>>j)&1)for j in positions];sol=old.solve(rows,len(dirs))
 def lift(bits):return old.xor(d for i,d in enumerate(dirs)if bits>>i&1)
 return dict(branch=branch,consistent=sol['consistent'],k_particular=(k0^lift(sol['particular']))if sol['consistent']else None,k_basis=[lift(v)for v in sol['basis']],rows=rows,coordinate_family=sol,k0=k0,k_directions=dirs)

def same_family(a,b):
 check(a['consistent']==b['consistent'],'family consistency')
 if not a['consistent']:return
 A=old.basis(a['k_basis']);B=old.basis(b['k_basis']);check(len(A)==len(a['k_basis'])and len(B)==len(b['k_basis'])and len(A)==len(B),'complete independent mapped kernels')
 check(old.residual(a['k_particular']^b['k_particular'],A)==0,'mapped particulars same coset');check(all(old.residual(v,A)==0 for v in b['k_basis'])and all(old.residual(v,B)==0 for v in a['k_basis']),'mutual kernel span')

def states(sol):
 if not sol['consistent']:return []
 return [sol['k_particular']^old.xor(v for i,v in enumerate(sol['k_basis'])if t>>i&1)for t in range(1<<len(sol['k_basis']))]

def controls():
 records=[];degenerate=0;normpositive=0
 for n,mod,vs,A in [(2,7,[2],0),(3,11,[1,2],1),(4,19,[1,2,8],0)]:
  f=BinaryField(n,mod);W=rb.spans(f,vs);roots=[sqrt(f,W[2][i])for i in sorted(W[2])];V=[old.xor(v for i,v in enumerate(vs)if t>>i&1)for t in range(1<<len(vs))]
  for alpha,gamma in itertools.product(range(f.order),repeat=2):
   c=dict(alphaT=0,betaT=0,gammaT=0,alphaU=alpha,betaU=0,gammaU=gamma,b=0);a=family(f,c,W,'k83',roots);b=family(f,c,W,'U52',roots);same_family(a,b);expected={k for k in range(f.order)if old.residual(f.mul(alpha,f.square(k))^gamma,W[2])==0};check(set(states(b))==expected,'all alpha/gamma/k independent Frobenius image oracle');degenerate+=alpha==0
  pts=rb.points(f,A,1);targets=[p for p in pts if p[0]not in V];check(targets,'tiny target admission');z,y=targets[0];sources=[p for p in pts if p[0]in V and p[0]!=0][:2];hs=[h for h in range(f.order)if f.square(h)^h^z in V][:2];positive=0
  products={tuple(old.pmul(old.pmul([t[0],1],[t[1],1],f),[t[2],1],f)):t for t in itertools.combinations_with_replacement(V,3)}
  for a,ya in sources:
   for h in hs:
    c=ore.scalars(f,A,z,y,h,a,ya);check(c['betaU']==0,'regular constant coefficient has no linear k');fa=family(f,c,W,'k83',roots);fb=family(f,c,W,'U52',roots);same_family(fa,fb)
    for k in range(f.order):
     Q=rb.quotient(f,A,1,z,y,h,a,ya,k);check(Q==ore.qfast(f,c,k),'literal norm quotient oracle');supported=tuple(Q)in products;check(rb.supported(f,Q,vs)==supported,'independent complete source product oracle')
     if supported and f.mul(h,a)^k:
      rec=rb.recover(f,Q,a,vs);expected=[list(t)for t in sorted(set(itertools.permutations(list(products[tuple(Q)])+[a,a])))];check(rec['status']=='complete'and rec['sources']==expected,'all multiplicities/source orderings');positive+=1
    check(set(states(fb))=={k for k in range(f.order)if old.residual(ore.qfast(f,c,k)[1],W[1])==0 and old.residual(ore.qfast(f,c,k)[0],W[2])==0},'complete tiny norm family')
  records.append(dict(n=n,regular_cells=len(sources)*len(hs),positive=positive));normpositive+=positive
 check(normpositive>0,'positive regular norm control')
 return dict(fields=records,synthetic_alpha_zero_cases=degenerate,positive_norm_cases=normpositive)

def trace(f,x):
 value=0
 for _ in range(f.degree):value^=x;x=f.square(x)
 return value

def ordinate(f,A,B,x):
 rhs=x^A^f.mul(B,f.inv(f.square(x)))
 if trace(f,rhs):return None
 u=0;v=rhs
 for _ in range((f.degree+1)//2):u^=v;v=f.square(f.square(v))
 check(f.square(u)^u==rhs,'halftrace ordinate');return f.mul(x,u)

def outside_order4(f,A,x,y):
 lam=x^f.mul(y,f.inv(x));x2=f.square(lam)^lam^A;y2=f.square(x)^f.mul(lam^1,x2)
 return x2!=0,[x2,y2]

def setup():
 f,A,B,z,y,h,_,_,vs=ore.fixture();W=rb.spans(f,vs);selection=[];generic=None
 for x in range(2,256):
  yy=ordinate(f,A,B,x)
  if yy is None:selection.append([x,'no_rational_lift']);continue
  outside,double=outside_order4(f,A,x,yy);selection.append([x,'outside_order4'if outside else'order_divides4',yy,double])
  if outside:generic=(x,yy);break
 check(generic is not None,'source-only generic anchor found within fixed2..255');x,yy=generic;cells=[dict(a=1,ya=0),dict(a=1,ya=1),dict(a=x,ya=yy),dict(a=x,ya=yy^x)]
 for cell in cells:check(f.square(cell['ya'])^f.mul(cell['a'],cell['ya'])==f.mul(f.square(cell['a']),cell['a'])^f.mul(A,f.square(cell['a']))^B,'source-only cell rationality')
 return f,A,B,z,y,h,vs,W,cells,selection

def arm(method,rep):
 start=time.monotonic();f,A,B,z,y,h,vs,W,cells,selection=setup();setup_time=time.monotonic()-start
 begin=time.monotonic();lams=cw.lambdas(f,vs);sqrtbasis=[sqrt(f,W[2][i])for i in sorted(W[2])]if method=='U52'else[];cache_time=time.monotonic()-begin
 begin=time.monotonic();families=[]
 for cell in cells:
  c=ore.scalars(f,A,z,y,h,cell['a'],cell['ya']);t0=f.mul(f.square(z),z)^f.mul(c['r0'],f.square(z))^f.mul(c['s0'],z)^f.mul(f.mul(h,z),y);t1=f.mul(c['r1'],f.square(z))^f.mul(c['s1'],z)^y;den=f.inv(f.mul(z,f.square(cell['a'])));check(c['alphaU']==f.mul(f.square(t1)^B,den)and c['gammaU']==f.mul(f.square(t0),den)and c['betaU']==0,'constant norm coefficient Frobenius identity');families.append(dict(cell=cell,coefficients=c,family=family(f,c,W,method,sqrtbasis)))
 formulation_time=time.monotonic()-begin;complete=True;count=0
 for rec in families:
  sol=rec['family']
  if sol['consistent']:
   if len(sol['k_basis'])>12:complete=False
   else:count+=1<<len(sol['k_basis'])
 if count>256:complete=False
 begin=time.monotonic();tuples=0
 for rec in families:
  rec['candidates']=[]
  if not complete:continue
  a=rec['cell']['a'];ya=rec['cell']['ya'];c=rec['coefficients']
  for k in sorted(states(rec['family'])):
   Q=ore.qfast(f,c,k);check(Q==rb.quotient(f,A,B,z,y,h,a,ya,k),'paid actual literal verification');cert=cw.certificate(f,Q,vs,'explicit_cached',lams);excluded=k==f.mul(h,a);recovery=None
   if cert['supported']and not excluded:
    if tuples+120>1000:complete=False;break
    recovery=rb.recover(f,Q,a,vs);tuples+=len(recovery['sources']);complete &=recovery['status']=='complete'
   rec['candidates'].append(dict(k=k,Q=Q,excluded_common_factor=excluded,recovery=recovery,**cw.corecert(cert)))
  if not complete:break
 extraction_time=time.monotonic()-begin
 out=dict(method=method,rep=rep,selection=selection,cells=families,complete_batch=complete,candidate_count=count if complete else None,source_tuple_count=tuples if complete else None,setup_seconds=setup_time,cache_seconds=cache_time,formulation_solve_seconds=formulation_time,extract_verify_seconds=extraction_time)
 begin=time.monotonic();json.dumps(out,separators=(',',':'));out['serialization_seconds']=time.monotonic()-begin;out['total_seconds']=time.monotonic()-start;return out

def profile():
 records=[];reference=None;receipts={};aliases=[];representations={}
 for rep in range(3):
  order=['k83','U52']if rep%2==0 else['U52','k83'];pair={}
  for method in order:
   data=arm(method,rep);pair[method]=data
   if reference is None:reference=data['cells']
   if method not in representations:
    representations[method]=[dict(cell=r['cell'],coefficients=r['coefficients'],family={k:v for k,v in r['family'].items()if k!='rows'} )for r in data['cells']]
    for rec in representations[method]:
     sol=rec['family']['coordinate_family']
     if sol is not None:rec['family']['coordinate_family']={k:v for k,v in sol.items()if k not in ['pivots','contradiction']}
   if data['complete_batch']:
    for a,b in zip(reference,data['cells']):same_family(a['family'],b['family']);check(a['cell']==b['cell']and a['candidates']==b['candidates'],'same exact complete candidate/output sets')
   compactcells=[]
   for i,r in enumerate(data['cells']):
    # Exact receipt/family alias to first copy; preserve every changed representation.
    payload=json.dumps(r['candidates'],sort_keys=True,separators=(',',':'));receipt_id=hashlib.sha256(payload.encode()).hexdigest();receipts.setdefault(receipt_id,r['candidates'])
    compactcells.append(dict(cell=r['cell'],branch=r['family']['branch'],consistent=r['family']['consistent'],coordinate_variables=len(r['family']['k_directions']),rows=len(r['family']['rows']),rank=(r['family']['coordinate_family']or{}).get('rank'),nullity=len(r['family']['k_basis']),receipt_alias=receipt_id))
   records.append({**{k:v for k,v in data.items()if k!='cells'},'cells':compactcells});aliases.append([rep,method,[r['receipt_alias']for r in compactcells]])
 complete=all(r['complete_batch']for r in records);med={m:statistics.median(r['total_seconds']for r in records if r['method']==m)for m in ['k83','U52']}
 return dict(representations=representations,receipts=receipts,aliases=aliases,records=records,complete_comparison=complete,median_total_seconds=med,internal_total_ratio=med['k83']/med['U52']if complete else None,timing='3alternatingSAMEPROCESSrep, parsing/fullsetup/sourceanchorselection/cache/formulation/support/verification/outputpaid; notcold/genuineGB',original_global_complete=False)

def compact(out):
 profileout=out.get('profile',{});return dict(controls=out.get('controls'),profile={k:v for k,v in profileout.items()if k not in ['representations','receipts','aliases']})

def demote(out):
 profileout=out.get('profile',{});profileout['complete_comparison']=False;profileout['internal_total_ratio']=None
 for record in profileout.get('records',[]):record['complete_batch']=False;record['candidate_count']=None;record['source_tuple_count']=None

def main():
 ident=sys.argv[1];out={};status='passed';error=None
 try:out['controls']=controls();out['profile']=profile()
 except Exception as exc:status='inconclusive';error=type(exc).__name__+': '+str(exc)
 encoded=json.dumps(dict(out=out,error=error,predicates=CHECKS),sort_keys=True,separators=(',',':')).encode();blob=gzip.compress(encoded,mtime=0)
 if len(encoded)>512*1024 or len(blob)>32*1024:
  status='inconclusive';error='512KiBraw/32KiBcompressed cap';demote(out);encoded=json.dumps(dict(error=error,complete=False,raw_omitted=True)).encode();blob=gzip.compress(encoded,mtime=0)
 with(P/'raw.json.gz').open('xb')as stream:stream.write(blob)
 evidence=dict(details=compact(out),error=error,predicates=CHECKS,raw_path='research/frobenius52_20261115_v8/raw.json.gz',raw_sha256=hashlib.sha256(blob).hexdigest(),raw_bytes=len(blob),raw_uncompressed_bytes=len(encoded),peak_process_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,complete_conditional_comparison=status=='passed'and out.get('profile',{}).get('complete_comparison',False),complete_global_root_search=False,original_root_count_exact=None,qualified_speedup=None,release_gate_claim=False,novelty_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=ident,status=status,summary='Exact pure-Frobenius52-coordinate component versus strong cached83-k baseline on matched torsion and source-only non-order4 anchors.',evidence=evidence,next_actions=['Review both coordinate family maps and all retained receipts; if total gain is setup-limited use a fresh preregistered batch with independently fixed generic anchors, while pursuing actual all-distinct rank4/global-root closure separately.'])
 text=json.dumps(result,separators=(',',':'))
 if time.monotonic()-START>=84:result['status']='inconclusive';evidence['complete_conditional_comparison']=False;evidence['error']='late serialized report';demote(out);evidence['details']=compact(out);text=json.dumps(result)
 assert len(text.encode())<=16384 and time.monotonic()-START<88,'compact final deadline';print(text)
if __name__=='__main__':main()
