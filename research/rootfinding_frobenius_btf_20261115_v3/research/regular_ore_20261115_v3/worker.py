"""Two-term Frobenius formulation with complete candidate rejection receipts."""
from pathlib import Path
import sys,json,time,gzip,hashlib,itertools,resource,statistics
from rootfinder.binaryfield import BinaryField,divmod_poly,powmod_poly
from research.quartic_cell_20261114_v1 import worker as old
from research.regular_block_20261115_v2 import worker as rb
P=Path(__file__).resolve().parent;ROOT=P.parents[1];START=time.monotonic();CHECKS=0

def check(value,why):
 global CHECKS
 CHECKS+=1
 assert CHECKS<=200000 and time.monotonic()-START<80,'predicate/time budget'
 assert value,why

def scalars(f,A,z,y,h,a,ya):
 c=a^z;ia=f.inv(a);ic2=f.inv(f.square(c));a2=f.square(a);z2=f.square(z)
 s0=f.mul(1^h,a2);s1=f.mul(ya,ia)^a
 r0=f.mul(f.mul(f.mul(h,a),ya)^f.mul(a2,a)^f.mul(z2,z)^f.mul(s0,c)^f.mul(f.mul(h,z),y),ic2)
 r1=f.mul(ya^f.mul(s1,c)^y,ic2);b=f.square(h)^h^z
 at=f.square(r1);bt=1^f.mul(h,r1);au=1^r1^f.mul(z,at);bu=r0^f.mul(h,s1)^f.mul(z,bt)
 gt=f.square(r0)^f.mul(h,r0)^f.mul(f.square(h),A)^f.mul(z,b)^a2
 gu=f.mul(h,s0)^f.mul(z,gt^a2)^f.mul(a2,b)
 delta=f.mul(au,bt)^f.mul(at,bu)
 check(at!=0 or au!=0,'nonzero square coefficient')
 return dict(alphaT=at,betaT=bt,gammaT=gt,alphaU=au,betaU=bu,gammaU=gu,Delta=delta,b=b,r0=r0,r1=r1,s0=s0,s1=s1)

def qfast(f,c,k):
 k2=f.square(k)
 return [f.mul(c['alphaU'],k2)^f.mul(c['betaU'],k)^c['gammaU'],f.mul(c['alphaT'],k2)^f.mul(c['betaT'],k)^c['gammaT'],c['b'],1]

def delta_check(f,c,k,Q):
 lhs=f.mul(c['alphaU'],Q[1]^c['gammaT'])^f.mul(c['alphaT'],Q[0]^c['gammaU'])
 check(lhs==f.mul(c['Delta'],k),'common-square compatibility identity')
 if c['Delta']:check(f.mul(lhs,f.inv(c['Delta']))==k,'Delta nonzero recovers same k')
 else:check(lhs==0,'Delta zero compatibility retained')

def rows_for(f,vs,base,cols):
 W=rb.spans(f,vs)
 def pack(Q):return old.residual(Q[1],W[1])|(old.residual(Q[0],W[2])<<f.degree)
 rhs=pack(base);packed=[pack(c)for c in cols]
 rows=[(sum(((v>>j)&1)<<i for i,v in enumerate(packed)),(rhs>>j)&1)for j in range(2*f.degree)]
 return rows,W

def formulation(f,A,B,z,y,h,a,ya,vs,method):
 coeff=None
 if method=='literal':
  base=rb.quotient(f,A,B,z,y,h,a,ya,0)
  cols=[[u^v for u,v in zip(rb.quotient(f,A,B,z,y,h,a,ya,1<<i),base)]for i in range(f.degree)]
 else:
  coeff=scalars(f,A,z,y,h,a,ya);base=qfast(f,coeff,0)
  cols=[[u^v for u,v in zip(qfast(f,coeff,1<<i),base)]for i in range(f.degree)]
 rows,W=rows_for(f,vs,base,cols)
 return base,cols,rows,W,coeff

def certificate(f,Q,vs):
 R=(0,1);coeff=[1]
 for v in vs:
  p=v;lam=0
  for c in coeff:lam^=f.mul(c,p);p=f.square(p)
  check(lam!=0,'source basis independence')
  R=divmod_poly(old.padd(powmod_poly(R,2,Q,f),[f.mul(lam,x)for x in R]),Q,f)[1]
  new=[0]*(len(coeff)+1)
  for i,c in enumerate(coeff):new[i]^=f.mul(lam,c);new[i+1]^=f.square(c)
  coeff=new
 R4=powmod_poly(R,4,Q,f)
 return dict(L_remainder=list(R),fourth_power_residue=list(R4),supported=not R4)

def points(f,A,B):return rb.points(f,A,B)

def controls():
 records=[];totalstates=0;positives=0;delta0=0;r1zero=0;perfield=[]
 for n,mod,vs,A in [(2,7,[2],0),(3,11,[1,2],1),(4,19,[1,2,8],0)]:
  f=BinaryField(n,mod);V=[old.xor(v for i,v in enumerate(vs)if t>>i&1)for t in range(1<<len(vs))];pts=points(f,A,1);targets=[t for t in pts if t[0]not in V];sources=[p for p in pts if p[0]in V and p[0]!=0];npositive=0;ncells=0
  cells=[(z,y,a,ya,h)for z,y in targets for a,ya in sources for h in range(f.order)if f.square(h)^h^z in V]
  check(len(cells)*f.order+totalstates<=100000,'preflight small-state cap')
  for z,y,a,ya,h in cells:
   c=scalars(f,A,z,y,h,a,ya);delta0+=c['Delta']==0;r1zero+=c['r1']==0
   base,cols,rows,W,_=formulation(f,A,1,z,y,h,a,ya,vs,'fast');sol=old.solve(rows,n);wanted=[];cellpositive=0
   # Independent product table built once for this source domain.
   products={tuple(old.pmul(old.pmul([t[0],1],[t[1],1],f),[t[2],1],f)):t for t in itertools.combinations_with_replacement(V,3)}
   for k in range(f.order):
    totalstates+=1;Q=rb.quotient(f,A,1,z,y,h,a,ya,k);check(Q==qfast(f,c,k),'literal division versus six scalar coefficients');delta_check(f,c,k,Q)
    if old.residual(Q[1],W[1])==0 and old.residual(Q[0],W[2])==0:wanted.append(k)
    cert=certificate(f,Q,vs);check(cert['supported']==(tuple(Q)in products),'independent full tiny source-product oracle')
    if cert['supported']and f.mul(h,a)^k:
     rec=rb.recover(f,Q,a,vs);expected=[list(t)for t in sorted(set(itertools.permutations(list(products[tuple(Q)])+[a,a])))];check(rec['status']=='complete' and rec['sources']==expected,'all original source orderings in scoped norm component');positives+=1;npositive+=1;cellpositive+=1
   check(set(rb.family_states(sol))==set(wanted),'complete tiny relaxed affine family')
   records.append(dict(n=n,z=z,y=y,a=a,ya=ya,h=h,Delta=c['Delta'],r1=c['r1'],positive_regular_parameters=cellpositive,rank=sol['rank'],nullity=sol['nullity'],consistent=sol['consistent']));ncells+=1
  check(ncells>0,'nonvacuous every declared field')
  perfield.append(dict(n=n,cells=ncells,positive_regular_norm_parameters=npositive,targets=len(targets),source_lifts=len(sources)))
 check(positives>0,'positive norm controls required')
 return dict(records=records,per_field=perfield,exhausted_k_states=totalstates,positive_regular_norm_parameters=positives,Delta_zero_cells=delta0,r1_zero_cells=r1zero,Delta_zero_coverage='observed'if delta0 else'gap',r1_zero_coverage='observed'if r1zero else'gap',GF16_positive_coverage='observed'if perfield[-1]['positive_regular_norm_parameters']else'gap',global_original_root_complete=False)

def fixture():
 data=json.loads((ROOT/'research/advance/instances/n83-d18-m5-T001-offset0.json').read_text());spec=data['extension_field'];f=BinaryField(spec['degree'],int(spec['modulus_hex'],16));A=int(data['curve']['a_hex'],16);B=int(data['curve']['b_hex'],16);z=int(data['target']['x_hex'],16);y=int(data['target']['y_hex'],16)^z;h=0x4208336d0d441df79c400;a=1;ya=0;vs=[1<<i for i in range(18)]
 check(f.degree==83 and [int(v,16)for v in data['source_basis_hex']]==vs and int(data.get('source_offset_hex','0'),16)==0,'actual field/source fixture')
 check(B!=0 and a!=z and z>=1<<18 and (f.square(h)^h^z)<1<<18,'actual regular guards/admitted h')
 check(f.square(y)^f.mul(z,y)==f.mul(f.square(z),z)^f.mul(A,f.square(z))^B and f.square(ya)^f.mul(a,ya)==f.mul(f.square(a),a)^f.mul(A,f.square(a))^B,'actual target/source rationality')
 return f,A,B,z,y,h,a,ya,vs

def arm(method,rep):
 start=time.monotonic();f,A,B,z,y,h,a,ya,vs=fixture();setup=time.monotonic()-start
 begin=time.monotonic();base,cols,rows,W,c=formulation(f,A,B,z,y,h,a,ya,vs,method);construction=time.monotonic()-begin;check([len(w)for w in W]==[18,35,52],'actual spans')
 begin=time.monotonic();sol=old.solve(rows,83);solvetime=time.monotonic()-begin
 begin=time.monotonic();candidates=[];complete=True;regular=None;supported=None
 if sol['consistent']and sol['nullity']>12:complete=False
 else:
  regular=0;supported=0
  for k in sorted(rb.family_states(sol)):
   Q=[base[j]^old.xor(col[j]for i,col in enumerate(cols)if k>>i&1)for j in range(4)]
   check(Q==rb.quotient(f,A,B,z,y,h,a,ya,k),'paid literal candidate verification')
   if c is not None:delta_check(f,c,k,Q)
   cert=certificate(f,Q,vs);excluded=k==f.mul(h,a);recovery=None
   if not excluded:
    regular+=1
    if cert['supported']:
     supported+=1;recovery=rb.recover(f,Q,a,vs);complete &=recovery['status']=='complete'
   candidates.append(dict(k=k,Q=Q,excluded_common_factor=excluded,recovery=recovery,**cert))
 extraction=time.monotonic()-begin
 out=dict(method=method,rep=rep,base=base,columns=cols,rows=rows,family=sol,coefficients=c,candidates=candidates,complete_regular_norm_fiber=complete,regular_parameters=regular,supported_parameters=supported,setup_seconds=setup,construction_seconds=construction,solve_seconds=solvetime,extract_verify_seconds=extraction)
 begin=time.monotonic();json.dumps(out,separators=(',',':'));serialization=time.monotonic()-begin;out['serialization_seconds']=serialization;out['total_seconds']=time.monotonic()-start
 return out

def profile():
 records=[];comparisons=[]
 for rep in range(3):
  order=['literal','fast']if rep%2==0 else['fast','literal'];pair={}
  for method in order:pair[method]=arm(method,rep);records.append(pair[method])
  check(pair['literal']['base']==pair['fast']['base'] and pair['literal']['columns']==pair['fast']['columns'] and pair['literal']['rows']==pair['fast']['rows'],'entire actual formulation equality')
  old.verify(pair['literal']['rows'],83,pair['fast']['family']);check(pair['literal']['family']==pair['fast']['family'],'canonical deterministic full family equality')
  check(pair['literal']['candidates']==pair['fast']['candidates'],'every bounded candidate/rejection/recovery equal across arms')
  comparisons.append(dict(rep=rep,order=order,complete_family_equal=True,every_candidate_equal=True))
 med={method:statistics.median(r['total_seconds']for r in records if r['method']==method)for method in ['literal','fast']};costs={method:statistics.median(r['construction_seconds']for r in records if r['method']==method)for method in ['literal','fast']}
 return dict(records=records,comparisons=comparisons,median_total_seconds=med,median_construction_seconds=costs,internal_component_total_ratio=med['literal']/med['fast'] if all(r['complete_regular_norm_fiber']for r in records)else None,internal_formulation_ratio=costs['literal']/costs['fast'] if all(r['complete_regular_norm_fiber']for r in records)else None,timing_contract='three alternating SAME-PROCESS reps; cached irreducibility, no cold/A-A/isolation qualifications; no genuine GB comparison',main_fixture=dict(h=0x4208336d0d441df79c400,a=1,ya=0),global_original_root_complete=False,original_root_count_exact=None)

def demote(out):
 out['internal_component_total_ratio']=None;out['internal_formulation_ratio']=None
 for r in out.get('records',[]):
  if 'complete_regular_norm_fiber'in r:r['complete_regular_norm_fiber']=False;r['regular_parameters']=None;r['supported_parameters']=None

def main():
 ident,mode=sys.argv[1:];assert mode in ['controls','profile'];out={};status='passed';error=None
 try:
  out=controls()if mode=='controls'else profile()
 except Exception as exc:status='inconclusive';error=type(exc).__name__+': '+str(exc)
 raw=dict(mode=mode,out=out,predicates=CHECKS,error=error);blob=gzip.compress(json.dumps(raw,sort_keys=True,separators=(',',':')).encode(),mtime=0)
 with(P/('raw-'+mode+'.json.gz')).open('xb')as stream:stream.write(blob)
 complete=status=='passed'and(mode=='controls'or all(r['complete_regular_norm_fiber']for r in out.get('records',[])))
 evidence=dict(details=out,predicates=CHECKS,error=error,raw_sha256=hashlib.sha256(blob).hexdigest(),raw_bytes=len(blob),peak_process_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,complete_conditional_component=complete,complete_global_root_search=False,original_root_count_exact=None,qualified_speedup=None,release_gate_claim=False,novelty_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=ident,status=status,summary='Exact two-term Frobenius formulation versus literal norm construction; full bounded rejection receipts; conditional component only.',evidence=evidence,next_actions=['Review every emitted Q/remainder/fourthpower independently; if formulation gain is swallowed by support costs, batch shared-subspace recurrence or prove a necessary image constraint before support. Preserve all unresolved global/exceptional branches.'])
 text=json.dumps(result,separators=(',',':'))
 if time.monotonic()-START>=84:
  result['status']='inconclusive';evidence['complete_conditional_component']=False;evidence['error']='late serialized report';demote(out);text=json.dumps(result)
 assert time.monotonic()-START<88,'final report deadline';print(text)
if __name__=='__main__':main()
