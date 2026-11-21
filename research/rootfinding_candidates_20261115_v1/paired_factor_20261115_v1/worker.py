"""Exact conditional 2+2+1 norm stratum; coordinator executes frozen commands."""
from pathlib import Path
import sys,json,time,gzip,hashlib,itertools,resource
from rootfinder.binaryfield import BinaryField
from research.quartic_cell_20261114_v1 import worker as old

P=Path(__file__).resolve().parent;ROOT=P.parents[1];START=time.monotonic();CHECKS=0
def check(v,why):
 global CHECKS
 CHECKS+=1
 assert CHECKS<=200000 and time.monotonic()-START<80,'predicate/time cap'
 assert v,why
def emit_family(family,cap=4):
 if not family['consistent']:return []
 check(1<<family['nullity']<=cap,'proved finite candidate cap')
 return sorted(family['particular']^old.xor(v for i,v in enumerate(family['basis'])if mask>>i&1)for mask in range(1<<family['nullity']))
def sqrt(f,a):
 for _ in range(f.degree-1):a=f.square(a)
 return a
def as_roots(f,c):
 cols=[f.square(1<<i)^(1<<i)for i in range(f.degree)]
 rows=[(sum(((v>>j)&1)<<i for i,v in enumerate(cols)),(c>>j)&1)for j in range(f.degree)]
 family=old.solve(rows,f.degree);roots=emit_family(family,2)
 check(all(f.square(v)^v==c for v in roots),'complete quadratic source roots')
 return roots
def qroots(f,sigma,tau):
 if sigma==0:return [sqrt(f,tau)]
 inv=f.inv(sigma);eta=f.mul(tau,f.square(inv))
 return sorted(f.mul(sigma,u)for u in as_roots(f,eta))
def source_tuples(f,d,H):
 b=H[4];sigma=sqrt(f,H[3]);tau=sqrt(f,H[1])
 check(H[2]==f.mul(b,H[3]) and H[0]==f.mul(b,H[1]),'paired coefficient identities')
 roots=qroots(f,sigma,tau)
 if b>=1<<d or not roots or any(a>=1<<d for a in roots):return []
 if sigma==0:values=[b]+roots*4
 else:
  check(len(roots)==2 and roots[0]!=roots[1],'squarefree quadratic pair')
  values=[b,roots[0],roots[0],roots[1],roots[1]]
 return [list(v)for v in sorted(set(itertools.permutations(values)))]
def relation(H,b,f):return H[2]^f.mul(b,H[3]),H[0]^f.mul(b,H[1])
def paired(f,A,B,z,y,h,k,d):
 base,columns=old.map_for(f,A,z,y,h,k);b=base[4]
 check(all(v[4]==0 for v in columns),'fixed first symmetric coefficient')
 if b>=1<<d:
  return dict(h=h,k=k,b=b,admitted=False,supported_norm_parameters=[],pair_family=None,
              reason='first source coefficient outside V',complete_conditional_norm_set=True)
 check(z>=1<<d and z!=0 and B!=0 and h not in [0,1],'paired degree bound guards')
 def pack(H):
  a,c=relation(H,b,f);return a|(c<<f.degree)
 rows=old.linear_rows(base,columns,pack,2*f.degree);family=old.solve(rows,2*f.degree)
 # The symbolic degree-four nonzero leading coefficient bounds every fiber.
 check(not family['consistent']or family['nullity']<=2,'at most four norm functions')
 candidates=emit_family(family);supported=[];replayed=[]
 for t in candidates:
  r=t&(f.order-1);s=t>>f.degree;H=old.literal(f,A,B,z,y,h,k,r,s)
  check(H==old.value(base,columns,t) and relation(H,b,f)==(0,0),'literal affine and paired replay')
  tuples=source_tuples(f,d,H)
  row=dict(r=r,s=s,H=H,ordered_source_tuples=tuples)
  replayed.append(row)
  if tuples:supported.append(row)
 return dict(h=h,k=k,b=b,admitted=True,pair_family=family,rows=rows,candidate_count=len(candidates),
             candidates=replayed,supported_norm_parameters=supported,complete_conditional_norm_set=True)
def oracle_sources(f,d,H):
 # Independent full-field quadratic evaluation. No inverse or linearized solver.
 q=f.order;b=H[4]
 if H[2]!=f.mul(b,H[3])or H[0]!=f.mul(b,H[1]):return []
 a=sqrt(f,H[3]);c=sqrt(f,H[1]);roots=[x for x in range(q)if f.square(x)^f.mul(a,x)^c==0]
 if b>=1<<d or not roots or any(x>=1<<d for x in roots):return []
 values=[b]+roots*4 if a==0 else[b,roots[0],roots[0],roots[1],roots[1]]
 return [list(v)for v in sorted(set(itertools.permutations(values)))]
def controls():
 summaries=[];raw_states=0;positive=0;guarded=0;support_cases=0
 for n,modulus in [(2,0b111),(3,0b1011),(4,0b10011)]:
  f=BinaryField(n,modulus);q=f.order;d=1 if n==2 else 2
  # Every scalar quadratic is compared with direct whole-field evaluation.
  for sigma,tau in itertools.product(range(q),repeat=2):
   check(qroots(f,sigma,tau)==[a for a in range(q)if f.square(a)^f.mul(sigma,a)^tau==0],'odd/even complete quadratic oracle')
  # Constructed supported paired factors retain positive, repeated and zero roots.
  for b,a,c in itertools.product(range(1<<d),repeat=3):
   H=old.pmul([b,1],old.pmul([f.mul(a,c),a^c,1],[f.mul(a,c),a^c,1],f),f)
   check(source_tuples(f,d,H)==[list(v)for v in sorted(set(itertools.permutations([b,a,a,c,c])))],'constructed complete source orderings')
   support_cases+=1
  points=[(z,y)for z in range(1<<d,q)for y in range(q)if f.square(y)^f.mul(z,y)==f.mul(f.square(z),z)^1][:2]
  for z,y in points:
   for h,k in itertools.product(range(q),[0,1]):
    result=paired(f,0,1,z,y,h,k,d);wanted=[]
    for r,s in itertools.product(range(q),repeat=2):
     H=old.literal(f,0,1,z,y,h,k,r,s);tuples=oracle_sources(f,d,H)
     if tuples:wanted.append(dict(r=r,s=s,H=H,ordered_source_tuples=tuples))
     raw_states+=1
    got=sorted(result['supported_norm_parameters'],key=lambda v:(v['r'],v['s']))
    wanted.sort(key=lambda v:(v['r'],v['s']))
    check(got==wanted,'complete tiny supported norm parameter set')
    positive+=bool(wanted);guarded+=result['admitted']
    summaries.append(dict(n=n,d=d,z=z,y=y,h=h,k=k,admitted=result['admitted'],
                          candidate_count=result.get('candidate_count'),supported_parameters=len(wanted)))
 return dict(cells=summaries,raw_states=raw_states,positive_cells=positive,admitted_cells=guarded,constructed_support_cases=support_cases)
def actual():
 path=ROOT/'research/advance/instances/n83-d18-m5-T001-offset0.json';data=json.loads(path.read_text());spec=data['extension_field'];f=BinaryField(spec['degree'],int(spec['modulus_hex'],16))
 check(f.degree==83 and data['source_basis_hex']==[hex(1<<i)for i in range(18)],'actual field/domain identity')
 A=int(data['curve']['a_hex'],16);B=int(data['curve']['b_hex'],16);z=int(data['target']['x_hex'],16);y=int(data['target']['y_hex'],16)^z;h=0x4208336d0d441df79c400
 check(f.square(y)^f.mul(z,y)==f.mul(f.square(z),z)^f.mul(A,f.square(z))^B,'actual target membership')
 base,cols=old.map_for(f,A,z,y,h,1)
 check(base==old.literal(f,A,B,z,y,h,1,0,0),'actual literal constant')
 for i in range(166):
  r=(1<<i)if i<83 else 0;s=(1<<(i-83))if i>=83 else 0
  check(old.value(base,cols,1<<i)==old.literal(f,A,B,z,y,h,1,r,s),'actual literal column')
 rows=[paired(f,A,B,z,y,h,k,18)for k in [0,1]]
 return dict(field_degree=83,modulus=f.modulus,h=h,fixed_k=[0,1],source_dimension=18,
             cells=rows,structural_affinity_proved=True,enumerated_2_to_73=False,
             original_root_count_exact=None,complete_global_root_search=False)
def demote(out):
 for row in out.get('actual',{}).get('cells',[]):
  row['complete_conditional_norm_set']=False
  row['candidate_count']=None
  row['verified_lower_bounds_only']=True
 for row in out.get('controls',{}).get('cells',[]):
  row['candidate_count']=None
  row['supported_parameters']=None
  row['verified_lower_bounds_only']=True
def main():
 ident=sys.argv[1];out={};status='passed';error=None
 try:out=dict(controls=controls(),actual=actual())
 except (AssertionError,ValueError,RuntimeError,ZeroDivisionError,KeyError,IndexError)as exc:status='inconclusive';error=str(exc)
 if status!='passed':demote(out)
 raw=dict(out=out,predicates=CHECKS,imported_predicates=old.CHECKS,error=error)
 blob=gzip.compress(json.dumps(raw,sort_keys=True,separators=(',',':')).encode(),mtime=0)
 with(P/'raw-controls.json.gz').open('xb')as stream:stream.write(blob)
 evidence=dict(details=out,predicates=CHECKS,imported_predicates=old.CHECKS,error=error,
               raw_sha256=hashlib.sha256(blob).hexdigest(),raw_bytes=len(blob),
               peak_process_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,
               complete_conditional_norm_sets=status=='passed',original_root_count_exact=None,
               complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False,novelty_claim=False,
               prior_lowrank_overlap='Every2+2+1source multiset has affine source rank at most2 after centroid translation; this is within previously covered rank-at-most2 source patterns, not new rank3 coverage.')
 result=dict(schema='scientific_result_v1',experiment_id=ident,status=status,
             summary='Exact paired repeated-factor norm sets within fixed binary cells; no original-global-root or qualified gain claim.',
             evidence=evidence,next_actions=['Inspect actual paired-factor candidate/support obstructions and classify remaining original lifts; do not call a fixed-cell reduction global coverage.'])
 text=json.dumps(result,separators=(',',':'))
 if time.monotonic()-START>=84:
  result['status']='inconclusive';evidence['complete_conditional_norm_sets']=False;evidence['error']='late serialized report';demote(out);text=json.dumps(result)
 assert time.monotonic()-START<88,'final report deadline';print(text)
if __name__=='__main__':main()
