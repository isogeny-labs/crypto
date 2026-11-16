"""Paid full-batch cubic support caching and exact three-coordinate square."""
from pathlib import Path
import sys,json,time,gzip,hashlib,itertools,resource,statistics
from rootfinder.binaryfield import BinaryField,divmod_poly,powmod_poly
from research.quartic_cell_20261114_v1 import worker as old
from research.regular_block_20261115_v2 import worker as rb
from research.regular_ore_20261115_v3 import worker as ore
P=Path(__file__).resolve().parent;ROOT=P.parents[1];START=time.monotonic();CHECKS=0;ARMS=['generic_per_k','generic_cached','explicit_cached'];MAXRAW=32*1024*1024

def check(v,why):
 global CHECKS
 CHECKS+=1
 assert CHECKS<=200000 and time.monotonic()-START<80,'predicate/time cap'
 assert v,why

def lambdas(f,vs):
 coeff=[1];values=[]
 for v in vs:
  p=v;lam=0
  for c in coeff:lam^=f.mul(c,p);p=f.square(p)
  check(lam!=0,'independent source basis');values.append(lam);nxt=[0]*(len(coeff)+1)
  for j,c in enumerate(coeff):nxt[j]^=f.mul(lam,c);nxt[j+1]^=f.square(c)
  coeff=nxt
 return values

def squarer(f,Q):
 u,t,b,_=Q;bu=f.mul(b,u);bt_u=f.mul(b,t)^u;b2_t=f.square(b)^t
 def square(p):
  p=list(p)+[0]*(3-len(p));p0,p1,p2=[f.square(x)for x in p]
  return [p0^f.mul(bu,p2),f.mul(bt_u,p2),p1^f.mul(b2_t,p2)]
 return square,[bu,bt_u,b2_t]

def certificate(f,Q,vs,method,cache):
 if method=='generic_per_k':return dict(ore.certificate(f,Q,vs),multipliers=None)
 if method=='generic_cached':
  R=(0,1)
  for lam in cache:R=divmod_poly(old.padd(powmod_poly(R,2,Q,f),[f.mul(lam,x)for x in R]),Q,f)[1]
  R4=powmod_poly(R,4,Q,f);return dict(L_remainder=list(R),fourth_power_residue=list(R4),supported=not R4,multipliers=None)
 square,mult=squarer(f,Q);R=[0,1,0]
 for lam in cache:R=[x^f.mul(lam,y)for x,y in zip(square(R),R)]
 R4=square(square(R))
 def trim(p):
  p=list(p)
  while p and p[-1]==0:p.pop()
  return p
 return dict(L_remainder=trim(R),fourth_power_residue=trim(R4),supported=not any(R4),multipliers=mult)

def corecert(c):return {k:c[k]for k in ['L_remainder','fourth_power_residue','supported']}

def controls():
 records=[];total=0;positive=0
 for n,mod,vs in [(2,7,[2]),(3,11,[1,2]),(4,19,[1,2,8])]:
  f=BinaryField(n,mod);V=[old.xor(v for i,v in enumerate(vs)if mask>>i&1)for mask in range(1<<len(vs))];cache=lambdas(f,vs);products={tuple(old.pmul(old.pmul([r[0],1],[r[1],1],f),[r[2],1],f)):r for r in itertools.combinations_with_replacement(V,3)};tests=[]
  # Every monic cubic over GF4, declared256 cubics in GF8/GF16, plus every
  # domain-supported cubic guarantees positive repeated/zero-root fixtures.
  coefficient_states=itertools.product(range(f.order),repeat=3)
  for state in itertools.islice(coefficient_states,64 if n==2 else 256):tests.append([*state,1])
  tests.extend([list(Q)for Q in products]);seen=set();localpos=0;repeated=0
  for Q in tests:
   if tuple(Q)in seen:continue
   seen.add(tuple(Q));total+=1;square,_=squarer(f,Q)
   for p in [[0,0,0],[0,1,0],[0,0,1],[1,1,1],Q[:3]]:
    expected=list(powmod_poly(p,2,Q,f));got=square(p)
    check(got==expected+[0]*(3-len(expected)),'independent generic polynomial square')
   certs=[certificate(f,Q,vs,method,None if method=='generic_per_k'else cache)for method in ARMS]
   check(all(corecert(c)==corecert(certs[0])for c in certs),'all candidate remainder/fourth-power identities')
   check(certs[0]['supported']==(tuple(Q)in products),'full independent source-product support oracle')
   recovery=None
   if certs[0]['supported']:
    positive+=1;localpos+=1;roots=products[tuple(Q)];repeated+=len(set(roots))<3;recovery=rb.recover(f,Q,V[0],vs);wanted=[list(p)for p in sorted(set(itertools.permutations(list(roots)+[V[0],V[0]])))];check(recovery['status']=='complete'and recovery['sources']==wanted,'complete multiplicity/source orderings')
   records.append(dict(n=n,Q=Q,certificate=corecert(certs[0]),recovery=recovery))
  check(localpos>0 and repeated>0,'positive repeated-root coverage every field')
 return dict(records=records,cubics=total,positive_supported=positive,field_degrees=[2,3,4],original_global_complete=False)

def shared_fixture():
 # ore.fixture parses and validates the FULL unchanged actual fixture once.
 f,A,B,z,y,h,a,ya,vs=ore.fixture();W=rb.spans(f,vs);check([len(w)for w in W]==[18,35,52],'actual product spans')
 n=f.degree;rhs=z>>18;cols=[(f.square(1<<i)^(1<<i))>>18 for i in range(n)]
 rows=[(sum(((v>>j)&1)<<i for i,v in enumerate(cols)),(rhs>>j)&1)for j in range(n-18)];family=old.solve(rows,n);check(family['consistent']and family['nullity']==18,'complete admitted h family')
 hs=[family['particular']^old.xor(v for j,v in enumerate(family['basis'])if i>>j&1)for i in range(32)]
 cells=[dict(index=0,label='exposed_anchor',h=h,a=a,ya=ya)]
 for i,hh in enumerate(hs):
  for yy in [0,1]:cells.append(dict(index=len(cells),label='training_%02d_y%d'%(i,yy),h=hh,a=1,ya=yy))
 check(len(cells)==65 and len(set((c['h'],c['a'],c['ya'])for c in cells))>=64,'65 indexed/atleast64distinct cells')
 return f,A,B,z,y,vs,W,cells,dict(rows=rows,family=family,h_values=hs,distinct_cells=len(set((c['h'],c['a'],c['ya'])for c in cells)))

def make_family(f,A,z,y,vs,W,cell):
 c=ore.scalars(f,A,z,y,cell['h'],cell['a'],cell['ya']);base=ore.qfast(f,c,0);cols=[[x^v for x,v in zip(ore.qfast(f,c,1<<i),base)]for i in range(f.degree)]
 def pack(Q):return old.residual(Q[1],W[1])|(old.residual(Q[0],W[2])<<f.degree)
 rhs=pack(base);packed=[pack(v)for v in cols];rows=[(sum(((v>>j)&1)<<i for i,v in enumerate(packed)),(rhs>>j)&1)for j in range(2*f.degree)];sol=old.solve(rows,f.degree)
 return dict(cell=cell,coefficients=c,base=base,columns=cols,rows=rows,family=sol)

def batch(method,rep):
 start=time.monotonic();f,A,B,z,y,vs,W,cells,hproof=shared_fixture();setup=time.monotonic()-start
 begin=time.monotonic();cache=None if method=='generic_per_k'else lambdas(f,vs);cachetime=time.monotonic()-begin
 begin=time.monotonic();families=[make_family(f,A,z,y,vs,W,c)for c in cells];formulation=time.monotonic()-begin
 eligible=0;complete=True;reason=None
 for rec in families:
  sol=rec['family']
  if sol['consistent']:
   if sol['nullity']>12:complete=False;reason='cell nullity cap'
   else:eligible+=1<<sol['nullity']
 if eligible>8192:complete=False;reason='full batch8192 candidate cap'
 check(eligible<=65*4096,'bounded candidate preflight arithmetic')
 begin=time.monotonic();tuplecount=0;supported=0;regular=0;outputs=[]
 for rec in families:
  cell=rec['cell'];entry=dict(cell=cell,family=rec['family'],coefficients=rec['coefficients'],base=rec['base'],columns=rec['columns'],rows=rec['rows'],candidates=[],complete_regular_norm_fiber=False,regular_parameters=None,supported_parameters=None)
  if complete:
   localregular=0;localpositive=0;localcomplete=True
   for k in sorted(rb.family_states(rec['family'])):
    Q=ore.qfast(f,rec['coefficients'],k);check(Q==rb.quotient(f,A,B,z,y,cell['h'],cell['a'],cell['ya'],k),'paid every-candidate literal norm verification');ore.delta_check(f,rec['coefficients'],k,Q);cert=certificate(f,Q,vs,method,cache);excluded=k==f.mul(cell['h'],cell['a']);recovery=None
    if not excluded:
     localregular+=1
     if cert['supported']:
      localpositive+=1
      if tuplecount+120>100000:complete=False;reason='global source tuple cap preflight';localcomplete=False
      else:
       recovery=rb.recover(f,Q,cell['a'],vs);localcomplete &=recovery['status']=='complete';tuplecount+=len(recovery['sources'])
    entry['candidates'].append(dict(k=k,Q=Q,excluded_common_factor=excluded,recovery=recovery,**cert))
    if not complete:break
   entry['regular_parameters']=localregular if localcomplete else None;entry['supported_parameters']=localpositive if localcomplete else None;entry['complete_regular_norm_fiber']=localcomplete;regular+=localregular;supported+=localpositive
   if not localcomplete:complete=False;reason=reason or'factorization cap'
  outputs.append(entry)
 extraction=time.monotonic()-begin
 if not complete:
  for rec in outputs:rec['complete_regular_norm_fiber']=False;rec['regular_parameters']=None;rec['supported_parameters']=None
 out=dict(method=method,rep=rep,h_proof=hproof,lambda_cache=cache,cells=outputs,indexed_cells=65,distinct_cells=hproof['distinct_cells'],eligible_candidates_preflight=eligible,global_regular_parameters=regular if complete else None,global_supported_parameters=supported if complete else None,global_source_tuple_count=tuplecount if complete else None,complete_batch=complete,unresolved_reason=reason,setup_seconds=setup,lambda_cache_seconds=cachetime,formulation_solve_seconds=formulation,extract_verify_seconds=extraction)
 begin=time.monotonic();encoded=json.dumps(out,separators=(',',':')).encode();check(len(encoded)<=MAXRAW,'per-arm raw output size cap');out['serialization_seconds']=time.monotonic()-begin;out['total_seconds']=time.monotonic()-start
 return out

def comparable(c):return {k:v for k,v in c.items()if k!='multipliers'}

def profile():
 records=[];comparisons=[]
 for rep in range(3):
  order=ARMS[rep:]+ARMS[:rep];pair={}
  for method in order:pair[method]=batch(method,rep);records.append(pair[method])
  reference=pair[ARMS[0]]
  for method in ARMS[1:]:
   other=pair[method];check(reference['h_proof']==other['h_proof'],'same full h selection proof');check(reference['eligible_candidates_preflight']==other['eligible_candidates_preflight'],'same candidate preflight')
   for a,b in zip(reference['cells'],other['cells']):
    check(a['cell']==b['cell']and a['family']==b['family']and a['rows']==b['rows']and a['base']==b['base']and a['columns']==b['columns'],'all indexed exact affine families equal')
    check([comparable(c)for c in a['candidates']]==[comparable(c)for c in b['candidates']],'all indexed candidate/support/order receipts equal')
  comparisons.append(dict(rep=rep,order=order,all_recorded_families_and_candidates_equal=True))
 complete=all(r['complete_batch']for r in records);med={m:statistics.median(r['total_seconds']for r in records if r['method']==m)for m in ARMS}
 ratios=None if not complete else dict(generic_per_k_over_explicit=med[ARMS[0]]/med[ARMS[2]],generic_cached_over_explicit=med[ARMS[1]]/med[ARMS[2]])
 return dict(records=records,comparisons=comparisons,complete_batch_comparison=complete,median_total_seconds=med,internal_full_batch_ratios=ratios,timing_contract='3rotating SAME-PROCESS reps, shared fixture parsed once per full65cell arm; notcold/A-A/qualified isolation; workerwall pays outerreport/gzip/crosscomparison',original_global_complete=False)

def demote(out):
 out['complete_batch_comparison']=False;out['internal_full_batch_ratios']=None
 for r in out.get('records',[]):
  if 'complete_batch'in r:
   r['complete_batch']=False;r['global_regular_parameters']=None;r['global_supported_parameters']=None;r['global_source_tuple_count']=None
   for c in r['cells']:c['complete_regular_norm_fiber']=False;c['regular_parameters']=None;c['supported_parameters']=None

def compact(out,mode):
 if mode=='controls':return {k:v for k,v in out.items()if k!='records'}
 return {**{k:v for k,v in out.items()if k!='records'},'records':[{k:v for k,v in r.items()if k not in ['cells','h_proof','lambda_cache']}for r in out.get('records',[])]}

def main():
 ident,mode=sys.argv[1:];assert mode in ['controls','profile'];out={};status='passed';error=None
 try:out=controls()if mode=='controls'else profile()
 except Exception as exc:status='inconclusive';error=type(exc).__name__+': '+str(exc)
 if mode=='profile'and not out.get('complete_batch_comparison',False):status='inconclusive'
 raw=dict(mode=mode,out=out,predicates=CHECKS,error=error);encoded=json.dumps(raw,separators=(',',':'),sort_keys=True).encode()
 if len(encoded)>MAXRAW:
  status='inconclusive';error='global32MiB raw record cap';demote(out);raw=dict(mode=mode,predicates=CHECKS,error=error,oversize_bytes=len(encoded),records_retained=False,completion=False);encoded=json.dumps(raw).encode()
 blob=gzip.compress(encoded,mtime=0)
 with(P/('raw-'+mode+'.json.gz')).open('xb')as stream:stream.write(blob)
 evidence=dict(details=compact(out,mode),predicates=CHECKS,error=error,raw_sha256=hashlib.sha256(blob).hexdigest(),raw_bytes=len(blob),raw_uncompressed_bytes=len(encoded),peak_process_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,complete_conditional_component=status=='passed',complete_global_root_search=False,original_root_count_exact=None,qualified_speedup=None,release_gate_claim=False,novelty_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=ident,status=status,summary='Three matched full-batch cubic support arms with shared parser/cache accounting and exact bounded certificates; conditional norm scope only.',evidence=evidence,next_actions=['Independently replay all distinct raw k/Q/R/R4 records and source orderings; if total gain remains below target identify parser/elimination versus support costs before a changed outer-chart pruning experiment.'])
 text=json.dumps(result,separators=(',',':'));assert len(text.encode())<=256*1024,'compact result256KiB cap'
 if time.monotonic()-START>=84:
  status='inconclusive';result['status']=status;evidence['complete_conditional_component']=False;evidence['error']='late serialized report';demote(out);evidence['details']=compact(out,mode);text=json.dumps(result)
 assert time.monotonic()-START<88,'final report deadline';print(text)
if __name__=='__main__':main()
