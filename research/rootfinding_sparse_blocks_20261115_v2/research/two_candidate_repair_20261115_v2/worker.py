"""Complete conditional4+1 norm candidate solver; coordinator executes frozen code."""
from pathlib import Path
import sys,json,time,gzip,hashlib,itertools,resource,statistics
from rootfinder.binaryfield import BinaryField
from research.quartic_cell_20261114_v1 import worker as old
P=Path(__file__).resolve().parent;ROOT=P.parents[1];START=time.monotonic();CHECKS=0

def check(v,why):
 global CHECKS
 CHECKS+=1
 assert CHECKS<=200000 and time.monotonic()-START<80,'predicate/time budget'
 assert v,why

def span(basis):
 out=[0]
 for v in basis:out += [x^v for x in out]
 return sorted(out)

def trace(f,x):
 v=0
 for _ in range(f.degree):v^=x;x=f.square(x)
 return v

def quadratic(f,rhs):
 if f.degree%2:
  if trace(f,rhs):return []
  v=0;x=rhs
  for _ in range((f.degree+1)//2):v^=x;x=f.square(f.square(x))
  check(f.square(v)^v==rhs,'odd half trace equation')
  return sorted([v,v^1])
 cols=[f.square(1<<i)^(1<<i)for i in range(f.degree)]
 rows=[(sum(((c>>j)&1)<<i for i,c in enumerate(cols)),(rhs>>j)&1)for j in range(f.degree)]
 sol=old.solve(rows,f.degree)
 if not sol['consistent']:return []
 check(sol['nullity']==1,'even quadratic complete kernel')
 return sorted([sol['particular'],sol['particular']^sol['basis'][0]])

def product_roots(f,roots):
 H=[1]
 for root in roots:H=old.pmul(H,[root,1],f)
 return H

def support(f,H,V):
 b=H[4]
 if b not in V or H[3] or H[2] or H[0]!=f.mul(b,H[1]):return None
 a=H[1]
 for _ in range((f.degree-2)%f.degree):a=f.square(a)
 if a not in V:return None
 roots=[b,a,a,a,a]
 check(product_roots(f,roots)==H,'literal source product')
 return sorted(set(itertools.permutations(roots)))

def direct(f,A,B,z,y,h,k,V):
 if not B or z in V or not z:return dict(status='unsupported',candidates=[],parameter_pairs=[])
 b=f.square(h)^h^z
 if b not in V:return dict(status='outside_cell',candidates=[],parameter_pairs=[])
 check(h not in [0,1],'admitted h nonzero/nonone')
 invh=f.inv(h);K=k^f.mul(f.square(h),A)^f.mul(z,b)
 candidates=[];pairs=[]
 for u in quadratic(f,f.mul(K,f.square(invh))):
  r=f.mul(h,u);s=f.mul(f.mul(k,r)^f.square(k),invh)
  H=old.literal(f,A,B,z,y,h,k,r,s)
  check(H==old.formula(f,A,z,y,h,k,r,s),'direct literal coefficient replay')
  sources=support(f,H,V)
  candidates.append(dict(r=r,s=s,H=H,supported=sources is not None,sources=[]if sources is None else[list(t)for t in sources]))
  if sources is not None:pairs.append([r,s])
 check(len(candidates)<=2,'at most two candidates')
 return dict(status='complete',b=b,candidates=candidates,parameter_pairs=sorted(pairs),complete_conditional_norm=True)

def gaussian(f,A,B,z,y,h,k,basis):
 base,cols=old.map_for(f,A,z,y,h,k);b=base[4]
 branch=old.stratum(f,len(basis),base,cols,b) if basis==[1<<i for i in range(len(basis))] else None
 if branch is None:
  C=old.basis([f.square(f.square(x))for x in basis])
  def pack(H):return H[3]|(H[2]<<f.degree)|((H[0]^f.mul(b,H[1]))<<(2*f.degree))|(old.residual(H[1],C)<<(3*f.degree))
  rows=old.linear_rows(base,cols,pack,4*f.degree);sol=old.solve(rows,2*f.degree)
 else:sol=branch['result']
 if not sol['consistent']:return []
 check(sol['nullity']<=1,'Gaussian conditional quartic dimension')
 states=[sol['particular']]
 for v in sol['basis']:states += [x^v for x in states]
 return sorted([[x&(f.order-1),x>>f.degree]for x in states])

def controls():
 records=[];states=0;quadratic_checks=0;positive=0;uniform=0
 for n,mod,bases in [(2,0b111,[[2]]),(3,0b1011,[[1],[1,2]]),(4,0b10011,[[1],[1,2]])]:
  f=BinaryField(n,mod);q=f.order
  for rhs in range(q):
   wanted=[u for u in range(q)if f.square(u)^u==rhs]
   check(quadratic(f,rhs)==wanted,'complete quadratic independent exhaustive oracle');quadratic_checks+=1
  for basis in bases:
   V=span(basis);A=1 if n==3 else 0;B=1
   source_index={}
   for roots in itertools.product(V,repeat=5):
    H=tuple(product_roots(f,roots));source_index.setdefault(H,[]).append(roots)
   target=next((z,y)for z in range(1,q)if z not in V for y in range(q)if f.square(y)^f.mul(z,y)==f.mul(f.square(z),z)^f.mul(A,f.square(z))^B)
   z,y=target
   for h in range(q):
    if f.square(h)^h^z not in V:continue
    for k in range(q):
     wanted=[]
     for r,s in itertools.product(range(q),repeat=2):
      H=old.literal(f,A,B,z,y,h,k,r,s)
      shape=not H[3] and not H[2] and H[0]==f.mul(H[4],H[1])
      if shape and tuple(H) in source_index:wanted.append([r,s])
      states+=1
     got=direct(f,A,B,z,y,h,k,V);check(got['status']=='complete'and got['parameter_pairs']==sorted(wanted),'complete direct norm parameter set')
     for item in got['candidates']:
      if item['supported']:check(item['sources']==[list(t)for t in sorted(source_index[tuple(item['H'])])],'independent complete original source-order set')
     check(gaussian(f,A,B,z,y,h,k,basis)==sorted(wanted),'independent Gaussian parameter set')
     if k==0:check(not wanted,'uniform kzero contradiction');uniform+=1
     positive+=bool(wanted)
     records.append(dict(n=n,basis=basis,target=[z,y],h=h,k=k,wanted=wanted,direct=got))
  bad=direct(f,0,1,0,1,0,0,[0,1]);check(bad['status']=='unsupported','pole/domain guard unknown not UNSAT')
 return dict(records=records,raw_parameter_states=states,quadratic_rhs_checks=quadratic_checks,positive_cells=positive,uniform_kzero_cells=uniform,changed_assumption='GF8 usesA1/B1 becauseGF8A0B1hasnopointoutsideprefixdomain; originalfailedvariantpreserved')

def actual():
 data=json.load(open(ROOT/'research/advance/instances/n83-d18-m5-T001-offset0.json'));spec=data['extension_field'];f=BinaryField(spec['degree'],int(spec['modulus_hex'],16));A=int(data['curve']['a_hex'],16);B=int(data['curve']['b_hex'],16);z=int(data['target']['x_hex'],16);y=int(data['target']['y_hex'],16)^z;basis=[int(v,16)for v in data['source_basis_hex']]
 check(basis==[1<<i for i in range(18)]and int(data.get('source_offset_hex','0'),16)==0,'actual source representation')
 check(f.square(y)^f.mul(z,y)==f.mul(f.square(z),z)^f.mul(A,f.square(z))^B,'actual curve target')
 return f,A,B,z,y,basis

def profile():
 class Prefix:
  def __contains__(self,v):return 0<=v<(1<<18)
 Vset=Prefix();h=0x4208336d0d441df79c400
 timings=[];results=[]
 for rep,order in enumerate([['gaussian','direct'],['direct','gaussian'],['gaussian','direct']]):
  pairs={}
  for method in order:
   t=time.monotonic();f,A,B,z,y,basis=actual();rows=[];debug=[]
   for k in [0,1]:
    out=direct(f,A,B,z,y,h,k,Vset)if method=='direct'else gaussian(f,A,B,z,y,h,k,basis)
    parameter_pairs=out['parameter_pairs']if method=='direct'else out;solutions=[]
    for r,s in parameter_pairs:
     H=old.literal(f,A,B,z,y,h,k,r,s);sources=support(f,H,Vset);check(sources is not None,'common supported result replay')
     solutions.append(dict(r=r,s=s,H=H,sources=[list(v)for v in sources]))
    rows.append(dict(h=h,k=k,solutions=solutions,complete_conditional_norm=True));debug.append(out)
   serialized=json.dumps(rows,sort_keys=True);elapsed=time.monotonic()-t
   pairs[method]=rows
   timings.append(dict(rep=rep,method=method,wall_seconds=elapsed,serialized_bytes=len(serialized.encode()),rows=rows,debug=debug))
  check(pairs['direct']==pairs['gaussian'],'actual complete method parameter sets')
  results.append(pairs['direct'])
 med={m:statistics.median(r['wall_seconds']for r in timings if r['method']==m)for m in ['gaussian','direct']}
 return dict(results=results,timings=timings,medians=med,diagnostic_component_ratio=med['gaussian']/med['direct'],timing_scope='each arm reads actual fixture, constructs and validates field/target, solves both fixed cells, independently replays all supported output and serializes IDENTICAL complete solution payload; debug retained outside paid serialization. Same Python wrapper and process-local irreducibility cache shared; not qualified cold full pipelines',uniform_kzero_guards_verified=z not in Vset and B!=0,original_auxiliary_roots_recovered=False)

def main():
 ident,mode=sys.argv[1:];assert mode in ['controls','profile'];status='passed';error=None;out={}
 try:out=controls()if mode=='controls'else profile()
 except (AssertionError,ValueError,RuntimeError,ZeroDivisionError,KeyError,StopIteration)as exc:status='inconclusive';error=str(exc)
 raw=dict(mode=mode,out=out,error=error,predicates=CHECKS,legacy_checks=old.CHECKS)
 blob=gzip.compress(json.dumps(raw,sort_keys=True,separators=(',',':')).encode(),mtime=0)
 with(P/('raw-'+mode+'.json.gz')).open('xb')as stream:stream.write(blob)
 evidence=dict(details=out,error=error,predicates=CHECKS,legacy_predicates=old.CHECKS,raw_sha256=hashlib.sha256(blob).hexdigest(),raw_bytes=len(blob),peak_process_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,complete_conditional_norm=status=='passed',complete_global_root_search=False,original_root_count_exact=None,original_auxiliary_roots_recovered=False,qualified_speedup=None,release_gate_claim=False,novelty_claim=False,known_affine_rank_one_overlap=True,new_source_coverage_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=ident,status=status,summary='Complete conditional 4+1 norm candidates by quadratic inversion; no original-global-root or genuine baseline gain claim.',evidence=evidence,next_actions=['Broaden to paired repeated factors only after an independent completeness derivation; preserve uniformkzero, even-field quadratic and original ordinate/auxiliary obligations.'])
 text=json.dumps(result,separators=(',',':'))
 if time.monotonic()-START>=84:
  result['status']='inconclusive';evidence['complete_conditional_norm']=False;evidence['error']='late serialized report'
  if 'diagnostic_component_ratio'in out:out['diagnostic_component_ratio']=None
  def demote(value):
   if isinstance(value,dict):
    if 'complete_conditional_norm'in value:value['complete_conditional_norm']=False
    if 'solutions'in value or'parameter_pairs'in value:value['verified_arrays_are_lower_bounds']=True
    for child in value.values():demote(child)
   elif isinstance(value,list):
    for child in value:demote(child)
  demote(out)
  text=json.dumps(result,separators=(',',':'))
 assert time.monotonic()-START<88,'full report deadline';print(text)
if __name__=='__main__':main()
