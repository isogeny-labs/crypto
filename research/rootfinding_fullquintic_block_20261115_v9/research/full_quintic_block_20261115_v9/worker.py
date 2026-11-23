"""Exact AS/Frobenius block compilation of full quintic norm spans."""
from pathlib import Path
import sys,json,time,gzip,hashlib,resource,itertools,statistics
from rootfinder.binaryfield import BinaryField
from research.quartic_cell_20261114_v1 import worker as old
from research.regular_ore_20261115_v3 import worker as ore
P=Path(__file__).resolve().parent;ROOT=P.parents[1];START=time.monotonic();CHECKS=0

def check(v,why):
 global CHECKS
 CHECKS+=1
 assert CHECKS<=200000 and time.monotonic()-START<80,'check/time cap'
 assert v,why

def trace(f,x):
 v=0
 for _ in range(f.degree):v^=x;x=f.square(x)
 return v

def sqrt(f,x):
 for _ in range(f.degree-1):x=f.square(x)
 return x

def asroot(f,x):
 check(trace(f,x)==0,'AS input trace zero')
 if f.degree%2:
  v=0;y=x
  for _ in range((f.degree+1)//2):v^=y;y=f.square(f.square(y))
 else:
  columns=[f.square(1<<i)^(1<<i)for i in range(f.degree)];rows=[(sum(((v>>j)&1)<<i for i,v in enumerate(columns)),(x>>j)&1)for j in range(f.degree)];sol=old.solve(rows,f.degree);check(sol['consistent']and sol['nullity']==1,'even AS exact family');v=sol['particular']
 check(f.square(v)^v==x,'AS root exact replay');return v

def spans(f,vs):
 if vs==[1<<i for i in range(len(vs))]:return old.product_spans(f,len(vs))
 out=[]
 for j in range(1,6):
  values=[]
  for tup in itertools.product(vs,repeat=j):
   x=1
   for v in tup:x=f.mul(x,v)
   values.append(x)
  out.append(old.basis(values))
 return out

def baseline(f,A,z,y,h,k,W):
 base,cols=old.map_for(f,A,z,y,h,k)
 def pack(H):return sum(old.residual(H[5-j],W[j-1])<<((j-1)*f.degree)for j in range(1,6))
 rows=old.linear_rows(base,cols,pack,5*f.degree);sol=old.solve(rows,2*f.degree)
 return dict(branch='baseline166',consistent=sol['consistent'],particular=sol['particular'],basis=sol['basis'],rank=sol['rank'],variables=2*f.degree,rows=rows,coordinate_particular=sol['particular'],coordinate_basis=sol['basis'])

def block(f,A,z,y,h,k,W):
 if not h or k:return dict(branch='unsupported',consistent=None)
 H0=old.formula(f,A,z,y,h,k,0,0);C2,C3,C4,C5=H0[3],H0[2],H0[1],H0[0]
 if old.residual(H0[4],W[0]):return dict(branch='constant_e1_outside',consistent=False,particular=None,basis=[])
 wb=[W[1][i]for i in sorted(W[1])];invh2=f.inv(f.square(h));tr=[trace(f,f.mul(v,invh2))for v in wb];rhs=trace(f,f.mul(C2,invh2));equation=(sum(v<<i for i,v in enumerate(tr)),rhs);ts=old.solve([equation],len(wb));rho=ts['rank']
 if not ts['consistent']:return dict(branch='trace_empty',consistent=False,particular=None,basis=[],trace_row=equation,rho=rho)
 def vfield(t):return old.xor(v for i,v in enumerate(wb)if t>>i&1)
 r0=f.mul(h,asroot(f,f.mul(vfield(ts['particular'])^C2,invh2)));rd=[f.mul(h,asroot(f,f.mul(vfield(v),invh2)))for v in ts['basis']]+[h]
 w0=sqrt(f,C4);wd=[sqrt(f,W[3][i])for i in sorted(W[3])];n=f.degree;raw0=r0|((w0^f.mul(z,r0))<<n);directions=[v|(f.mul(z,v)<<n)for v in rd]+[v<<n for v in wd];check(len(old.basis(directions))==len(directions),'independent full r/w coordinate map')
 constant=old.formula(f,A,z,y,h,k,r0,w0^f.mul(z,r0));cols=[]
 for raw in directions:
  dr=raw&((1<<n)-1);ds=raw>>n;dw=ds^f.mul(z,dr);de3=f.mul(z,f.square(dr))^f.mul(h,dw);de5=f.mul(z,f.square(dw));cols.append([de5,0,de3,0,0,0])
 # Tiny W5 need not be full: preserve BOTH e3 and e5 residuals.
 def pack(H):return old.residual(H[2],W[2])|(old.residual(H[0],W[4])<<n)
 positions=[j for j in range(n)if j not in W[2]]+[n+j for j in range(n)if j not in W[4]];rhsbits=pack(constant);packed=[pack(c)for c in cols];rows=[(sum(((v>>j)&1)<<i for i,v in enumerate(packed)),(rhsbits>>j)&1)for j in positions];sol=old.solve(rows,len(directions))
 def lift(bits):return old.xor(v for i,v in enumerate(directions)if bits>>i&1)
 return dict(branch='block',consistent=sol['consistent'],particular=(raw0^lift(sol['particular']))if sol['consistent']else None,basis=[lift(v)for v in sol['basis']],rho=rho,trace_row=equation,trace_particular=ts['particular'],trace_basis=ts['basis'],constants=H0,r0=r0,w0=w0,r_directions=rd,w_directions=wd,coordinate_particular=sol['particular'],coordinate_basis=sol['basis'],variables=len(directions),rank=sol['rank'],rows=rows)

def equal(a,b):
 check(a['consistent']==b['consistent'],'same affine consistency')
 if not a['consistent']:return
 A=old.basis(a['basis']);B=old.basis(b['basis']);check(len(A)==len(a['basis'])and len(B)==len(b['basis'])and len(A)==len(B),'complete independent kernel dimensions');check(old.residual(a['particular']^b['particular'],A)==0,'same affine coset');check(all(old.residual(v,A)==0 for v in b['basis'])and all(old.residual(v,B)==0 for v in a['basis']),'mutual kernel span')

def controls():
 cells=[];states=0;branches=set()
 for n,mod,vs,A in [(2,7,[2],0),(3,11,[1,2],1),(4,19,[1,2,8],0)]:
  f=BinaryField(n,mod);W=spans(f,vs);pts=[(x,y)for x in range(f.order)for y in range(f.order)if f.square(y)^f.mul(x,y)==f.mul(f.square(x),x)^f.mul(A,f.square(x))^1];z,y=next(p for p in pts if old.residual(p[0],W[0]))
  for h in range(1,f.order):
   a=baseline(f,A,z,y,h,0,W);b=block(f,A,z,y,h,0,W);equal(a,b);branches.add(b['branch']);wanted=[]
   for r,s in itertools.product(range(f.order),repeat=2):
    H=old.literal(f,A,1,z,y,h,0,r,s);check(H==old.formula(f,A,z,y,h,0,r,s),'independent literal norm formula');w=s^f.mul(z,r);C=old.formula(f,A,z,y,h,0,0,0);check(H[3]==C[3]^f.square(r)^f.mul(h,r)and H[2]==C[2]^f.mul(z,f.square(r))^f.mul(h,w)and H[1]==C[1]^f.square(w)and H[0]==C[0]^f.mul(z,f.square(w)),'complete normalform identity')
    admitted=all(old.residual(H[5-j],W[j-1])==0 for j in range(1,6));raw=r|(s<<n)
    if admitted:wanted.append(raw)
    if b['consistent']:check((old.residual(raw^b['particular'],old.basis(b['basis']))==0)==admitted,'full tiny original coefficient membership')
    else:check(not admitted,'empty original coefficient family')
    states+=1
   cells.append(dict(n=n,h=h,branch=b['branch'],rho=b.get('rho'),count=len(wanted)))
  check(block(f,A,z,y,0,0,W)['branch']=='unsupported'and block(f,A,z,y,1,1,W)['branch']=='unsupported','h0/k!=0 refusal')
 # Synthetic trace subspaces exercise rho0/RHS0,rho0/RHS1 andrho1 explicitly.
 f=BinaryField(3,11);synthetic=[]
 for mask,rhs in [(0,0),(0,1),(1,0),(1,1)]:
  sol=old.solve([(mask,rhs)],2);check(sol['consistent']==bool(mask or not rhs),'trace rank/empty branch oracle');synthetic.append(dict(row=[mask,rhs],consistent=sol['consistent'],rank=sol['rank']))
 synthetic_preimages=[]
 for allowed,C in [([0],0),([0],1),([0,1],0),([0,1],1)]:
  expected=[r for r in range(f.order)if f.square(r)^r^C in allowed];soluble=[v for v in allowed if trace(f,v^C)==0];actual=[]
  for v in soluble:
   root=asroot(f,v^C);actual.extend([root,root^1])
  check(sorted(actual)==expected,'synthetic rho branch complete AS preimages');synthetic_preimages.append(dict(allowed=allowed,C=C,solutions=expected))
 return dict(cells=cells,exhaustive_r_s_states=states,synthetic_trace_branches=synthetic,synthetic_AS_preimages=synthetic_preimages,branches=sorted(branches))

def arm(method,rep):
 start=time.monotonic();f,A,B,z,y,h,_,_,vs=ore.fixture();W=spans(f,vs);setup=time.monotonic()-start;begin=time.monotonic();result=baseline(f,A,z,y,h,0,W)if method=='baseline166'else block(f,A,z,y,h,0,W);compiletime=time.monotonic()-begin;check(result['consistent'],'actual family consistent');check(len(result['basis'])==73,'actual full family73');
 if method=='block104':check(result['rho']==1 and result['variables']==104 and len(result['rows'])==31 and result['rank']==31,'actual block consequence replayed')
 # Pay literal/spans verification for all mapped anchors in EACH arm.
 begin=time.monotonic()
 for raw in [result['particular']]+result['basis']:
  r=raw&((1<<83)-1);s=raw>>83;H=old.literal(f,A,B,z,y,h,0,r,s)
  if raw!=result['particular']:H=old.padd(H,old.literal(f,A,B,z,y,h,0,0,0))
  # Kernel directions use homogeneous differences from the zero norm.
  check(all(old.residual(H[5-j],W[j-1])==0 for j in range(1,6)),'mapped full-family literal coefficient verification')
 verifytime=time.monotonic()-begin;begin=time.monotonic();json.dumps(result,separators=(',',':'));serialization=time.monotonic()-begin
 return dict(method=method,rep=rep,result=result,setup_seconds=setup,compile_seconds=compiletime,verify_seconds=verifytime,serialization_seconds=serialization,total_seconds=time.monotonic()-start)

def profile():
 proofs={};records=[]
 for rep in range(3):
  order=['baseline166','block104']if rep%2==0 else['block104','baseline166'];pair={}
  for method in order:
   data=arm(method,rep);pair[method]=data['result'];proofs.setdefault(method,data['result']);records.append({k:v for k,v in data.items()if k!='result'})
  equal(pair['baseline166'],pair['block104'])
 med={m:statistics.median(r['total_seconds']for r in records if r['method']==m)for m in proofs}
 return dict(proofs=proofs,records=records,median_total_seconds=med,internal_total_ratio=med['baseline166']/med['block104'],complete_equivalent_relaxation=True,surviving_parameter_bits=73,original_global_complete=False)

def compact(out):return dict(controls=out.get('controls'),profile={k:v for k,v in out.get('profile',{}).items()if k!='proofs'})
def demote(out):out.get('profile',{}).update(complete_equivalent_relaxation=False,internal_total_ratio=None,surviving_parameter_bits=None)
def main():
 ident=sys.argv[1];out={};status='passed';error=None
 try:out['controls']=controls();out['profile']=profile()
 except Exception as exc:status='inconclusive';error=type(exc).__name__+': '+str(exc)
 encoded=json.dumps(dict(out=out,error=error,predicates=CHECKS),sort_keys=True,separators=(',',':')).encode();blob=gzip.compress(encoded,mtime=0)
 if len(encoded)>256*1024 or len(blob)>24*1024:status='inconclusive';error='raw256KiB/gzip24KiB cap';demote(out);encoded=json.dumps(dict(error=error,proofs_omitted=True,complete=False)).encode();blob=gzip.compress(encoded,mtime=0)
 with(P/'raw.json.gz').open('xb')as stream:stream.write(blob)
 evidence=dict(details=compact(out),error=error,predicates=CHECKS,raw_sha256=hashlib.sha256(blob).hexdigest(),raw_bytes=len(blob),raw_uncompressed_bytes=len(encoded),peak_process_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,complete_conditional_compiler=status=='passed'and out.get('profile',{}).get('complete_equivalent_relaxation',False),complete_global_root_search=False,original_root_count_exact=None,qualified_speedup=None,release_gate_claim=False,novelty_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=ident,status=status,summary='Exact full-quintic AS/Frobenius block compiler preserves the73-bit relaxation; no root search or global speed claim.',evidence=evidence,next_actions=['Review mapped full-family equivalence and paid compiler costs; use this exact full-quintic representation only as a building block for a separately proved whole-cell/rank4 support strategy.'])
 text=json.dumps(result,separators=(',',':'))
 if time.monotonic()-START>=84:result['status']='inconclusive';evidence['complete_conditional_compiler']=False;evidence['error']='late report';demote(out);evidence['details']=compact(out);text=json.dumps(result)
 assert len(text.encode())<=16384 and time.monotonic()-START<88,'compact final deadline';print(text)
if __name__=='__main__':main()
