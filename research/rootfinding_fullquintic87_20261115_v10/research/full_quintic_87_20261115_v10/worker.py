"""Exact e2/e3 coordinate ordering for every fixed binary norm k."""
from pathlib import Path
import sys,json,time,gzip,hashlib,resource,itertools,statistics
from rootfinder.binaryfield import BinaryField,gcd_poly
from research.quartic_cell_20261114_v1 import worker as old
from research.regular_ore_20261115_v3 import worker as ore
from research.full_quintic_block_20261115_v9 import worker as previous
P=Path(__file__).resolve().parent;ROOT=P.parents[1];START=time.monotonic();CHECKS=0

def check(v,why):
 global CHECKS
 CHECKS+=1
 assert CHECKS<=200000 and time.monotonic()-START<80,'predicate/time cap'
 assert v,why

def compiler87(f,A,z,y,h,k,W):
 if not h:return dict(branch='unsupported_h0',consistent=None)
 C=old.formula(f,A,z,y,h,k,0,0)
 if old.residual(C[4],W[0]):return dict(branch='e1_outside',consistent=False,particular=None,basis=[])
 wb=[W[1][i]for i in sorted(W[1])];ih=f.inv(h);ih2=f.square(ih);mask=sum(previous.trace(f,f.mul(v,ih2))<<i for i,v in enumerate(wb));rhs=previous.trace(f,f.mul(C[3],ih2));ts=old.solve([(mask,rhs)],len(wb))
 if not ts['consistent']:return dict(branch='trace_empty',consistent=False,particular=None,basis=[],rho=ts['rank'],trace_row=[mask,rhs])
 def vfield(bits):return old.xor(v for i,v in enumerate(wb)if bits>>i&1)
 r0=f.mul(h,previous.asroot(f,f.mul(vfield(ts['particular'])^C[3],ih2)));rd=[f.mul(h,previous.asroot(f,f.mul(vfield(v),ih2)))for v in ts['basis']]+[h]
 w0=f.mul(ih,C[2]^f.mul(z,f.square(r0))^f.mul(k,r0));n=f.degree;raw0=r0|((w0^f.mul(z,r0))<<n);directions=[]
 for dr in rd:
  dw=f.mul(ih,f.mul(z,f.square(dr))^f.mul(k,dr));directions.append(dr|((dw^f.mul(z,dr))<<n))
 for i in sorted(W[2]):directions.append(f.mul(ih,W[2][i])<<n)
 check(len(old.basis(directions))==len(directions),'independent r/e3 coordinate bijection')
 constant=old.formula(f,A,z,y,h,k,r0,w0^f.mul(z,r0));cols=[]
 for raw in directions:
  dr=raw&((1<<n)-1);dw=(raw>>n)^f.mul(z,dr);cols.append([f.mul(z,f.square(dw)),f.square(dw)^f.mul(k,dw),0,0,0,0])
 def pack(H):return old.residual(H[1],W[3])|(old.residual(H[0],W[4])<<n)
 positions=[j for j in range(n)if j not in W[3]]+[n+j for j in range(n)if j not in W[4]];rhsbits=pack(constant);packed=[pack(v)for v in cols];rows=[(sum(((v>>j)&1)<<i for i,v in enumerate(packed)),(rhsbits>>j)&1)for j in positions];sol=old.solve(rows,len(directions))
 def lift(bits):return old.xor(v for i,v in enumerate(directions)if bits>>i&1)
 return dict(branch='compiler87',consistent=sol['consistent'],particular=(raw0^lift(sol['particular']))if sol['consistent']else None,basis=[lift(v)for v in sol['basis']],rho=ts['rank'],trace_row=[mask,rhs],trace_particular=ts['particular'],trace_basis=ts['basis'],constants=C,r0=r0,w0=w0,directions=directions,coordinate_particular=sol['particular'],coordinate_basis=sol['basis'],variables=len(directions),rank=sol['rank'],rows=rows)

def controls():
 cells=[];states=0;repeated=0
 for n,mod,vs,A in [(2,7,[2],0),(3,11,[1,2],1),(4,19,[1,2,8],0)]:
  f=BinaryField(n,mod);W=previous.spans(f,vs);pts=[(x,y)for x in range(f.order)for y in range(f.order)if f.square(y)^f.mul(x,y)==f.mul(f.square(x),x)^f.mul(A,f.square(x))^1];z,y=next(p for p in pts if old.residual(p[0],W[0]))
  for h,k in itertools.product(range(1,f.order),[0,1,f.order-1]):
   baseline=previous.baseline(f,A,z,y,h,k,W);new=compiler87(f,A,z,y,h,k,W);previous.equal(baseline,new);accepted=0;kernel=old.basis(new.get('basis',[]))
   for r,s in itertools.product(range(f.order),repeat=2):
    H=old.literal(f,A,1,z,y,h,k,r,s);check(H==old.formula(f,A,z,y,h,k,r,s),'independent original literal norm');w=s^f.mul(z,r);C=old.formula(f,A,z,y,h,k,0,0);check(H[3]==C[3]^f.square(r)^f.mul(h,r)and H[2]==C[2]^f.mul(z,f.square(r))^f.mul(k,r)^f.mul(h,w)and H[1]==C[1]^f.square(w)^f.mul(k,w)and H[0]==C[0]^f.mul(z,f.square(w)),'all fixed-k normalform identities')
    admitted=all(old.residual(H[5-j],W[j-1])==0 for j in range(1,6));raw=r|(s<<n);member=bool(new['consistent'])and old.residual(raw^new['particular'],kernel)==0;check(member==admitted,'complete all-coefficient r/s set including e5');accepted+=admitted;states+=1
    if len(gcd_poly(H,[H[1],0,H[3],0,1],f))>1:repeated+=1
   cells.append(dict(n=n,h=h,k=k,branch=new['branch'],rho=new.get('rho'),count=accepted))
  check(compiler87(f,A,z,y,0,0,W)['branch']=='unsupported_h0','h0 unsupported retained')
 # Reuse exact synthetic AS branch fixtures, distinguish from originalnorm cases.
 f=BinaryField(3,11);synthetic=[]
 for allowed,C in [([0],0),([0],1),([0,1],0),([0,1],1)]:
  wanted=[r for r in range(f.order)if f.square(r)^r^C in allowed];got=[]
  for v in allowed:
   if previous.trace(f,v^C)==0:
    root=previous.asroot(f,v^C);got.extend([root,root^1])
  check(sorted(got)==wanted,'synthetic complete AS rho preimage');synthetic.append(dict(allowed=allowed,C=C,solutions=wanted))
 return dict(cells=cells,all_r_s_states=states,repeated_H_observations=repeated,synthetic_AS_cases=synthetic,nonzero_k_exercised=True)

def arm(method,rep):
 start=time.monotonic();f,A,B,z,y,h,_,_,vs=ore.fixture();W=previous.spans(f,vs);setup=time.monotonic()-start;begin=time.monotonic()
 if method=='baseline166':result=previous.baseline(f,A,z,y,h,0,W)
 elif method=='frozen104':result=previous.block(f,A,z,y,h,0,W)
 else:result=compiler87(f,A,z,y,h,0,W)
 compiletime=time.monotonic()-begin;check(result['consistent']and len(result['basis'])==73,'actual complete family73')
 if method=='new87':check(result['rho']==1 and result['variables']==87 and len(result['rows'])==14 and result['rank']==14,'actual87/14 consequence measured')
 begin=time.monotonic();zero=old.literal(f,A,B,z,y,h,0,0,0)
 for index,raw in enumerate([result['particular']]+result['basis']):
  r=raw&((1<<83)-1);s=raw>>83;H=old.literal(f,A,B,z,y,h,0,r,s)
  if index:H=old.padd(H,zero)
  check(all(old.residual(H[5-j],W[j-1])==0 for j in range(1,6)),'paid mapped literal family replay')
 verifytime=time.monotonic()-begin;begin=time.monotonic();json.dumps(result,separators=(',',':'));serialization=time.monotonic()-begin
 return dict(method=method,rep=rep,result=result,setup_seconds=setup,compile_seconds=compiletime,verify_seconds=verifytime,serialization_seconds=serialization,total_seconds=time.monotonic()-start)

def profile():
 methods=['baseline166','frozen104','new87'];proofs={};records=[]
 for rep in range(3):
  order=methods[rep:]+methods[:rep];pair={}
  for method in order:
   data=arm(method,rep);pair[method]=data['result'];proofs.setdefault(method,data['result']);records.append({k:v for k,v in data.items()if k!='result'})
  previous.equal(pair['baseline166'],pair['frozen104']);previous.equal(pair['baseline166'],pair['new87'])
 med={m:statistics.median(r['total_seconds']for r in records if r['method']==m)for m in methods}
 return dict(proofs=proofs,records=records,median_total_seconds=med,internal_total_ratios=dict(baseline166_over_new87=med['baseline166']/med['new87'],frozen104_over_new87=med['frozen104']/med['new87']),complete_equivalent_relaxation=True,surviving_parameter_bits=73,timing='3rotatingSAMEPROCESSreps; paidparse/setup/conversion/compile/verify/output; notcold/genuineGB',original_global_complete=False)

def compact(out):
 c=out.get('controls',{});return dict(controls={k:v for k,v in c.items()if k!='cells'},profile={k:v for k,v in out.get('profile',{}).items()if k!='proofs'})
def demote(out):out.get('profile',{}).update(complete_equivalent_relaxation=False,internal_total_ratios=None,surviving_parameter_bits=None)
def main():
 ident=sys.argv[1];out={};status='passed';error=None
 try:out['controls']=controls();out['profile']=profile()
 except Exception as exc:status='inconclusive';error=type(exc).__name__+': '+str(exc)
 encoded=json.dumps(dict(out=out,error=error,predicates=CHECKS),sort_keys=True,separators=(',',':')).encode();blob=gzip.compress(encoded,mtime=0)
 if len(encoded)>512*1024 or len(blob)>40*1024:status='inconclusive';error='raw512KiB/gzip40KiB cap';demote(out);encoded=json.dumps(dict(error=error,proofs_omitted=True,complete=False)).encode();blob=gzip.compress(encoded,mtime=0)
 with(P/'raw.json.gz').open('xb')as stream:stream.write(blob)
 evidence=dict(details=compact(out),error=error,predicates=CHECKS,raw_path='research/full_quintic_87_20261115_v10/raw.json.gz',raw_sha256=hashlib.sha256(blob).hexdigest(),raw_bytes=len(blob),raw_uncompressed_bytes=len(encoded),peak_process_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,complete_conditional_compiler=status=='passed'and out.get('profile',{}).get('complete_equivalent_relaxation',False),complete_global_root_search=False,original_root_count_exact=None,qualified_speedup=None,release_gate_claim=False,novelty_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=ident,status=status,summary='Exact87-coordinate any-fixed-k full-quintic compiler versus strong166 and frozen104; unchanged73-bit relaxation, no root search.',evidence=evidence,next_actions=['Review exact mapped coset/kernel equality and paid total costs; use measured failure/gain to design a changed shared-k compiler workload with all setup paid, while full support and original-root obligations remain open.'])
 text=json.dumps(result,separators=(',',':'))
 if time.monotonic()-START>=84:result['status']='inconclusive';evidence['complete_conditional_compiler']=False;evidence['error']='late serialized report';demote(out);evidence['details']=compact(out);text=json.dumps(result)
 assert len(text.encode())<=24576 and time.monotonic()-START<88,'compact final deadline';print(text)
if __name__=='__main__':main()
