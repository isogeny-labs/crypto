"""Exact shared-parameter Boolean ANFs and bounded quintic-cell refutations."""
from pathlib import Path
import sys,json,time,gzip,hashlib,resource,itertools,collections
from rootfinder.binaryfield import BinaryField,divmod_poly,powmod_poly
from research.quartic_cell_20261114_v1 import worker as old
from research.cubic_cached_20261115_v4 import worker as cacheworker
P=Path(__file__).resolve().parent;ROOT=P.parents[1];START=time.monotonic();CHECKS=0;RAWMAX=4*1024*1024
class Cap(Exception):pass
class Budget:
 def __init__(self,total=1000000,node=100000,coefficient=4096,active=65536):
  self.total=0;self.node=0;self.total_limit=total;self.node_limit=node;self.coefficient_limit=coefficient;self.active_limit=active;self.max_terms=0;self.max_active=0;self.max_degree=0;self.stage='start';self.allocated=0;self.proof_storage=0;self.max_allocated=0
 def reserve(self,n):
  if time.monotonic()-START>=78:raise Cap('work deadline')
  if self.total+n>self.total_limit:raise Cap('global product budget')
  if self.node+n>self.node_limit:raise Cap('node product budget')
  self.total+=n;self.node+=n
 def storage(self,n):
  if self.allocated+self.proof_storage+n>self.active_limit:raise Cap('conservative active allocation cap')
  self.allocated+=n;self.max_allocated=max(self.max_allocated,self.allocated+self.proof_storage)
 def inspect(self,p):
  if len(p)>self.coefficient_limit:raise Cap('coefficient monomial cap')
  self.max_terms=max(self.max_terms,len(p))
 def live(self,polys):
  n=sum(len(p)for p in polys);self.max_active=max(self.max_active,n)
  if n>self.active_limit:raise Cap('active monomial cap')
 def stats(self):return dict(products=self.total,node_products=self.node,max_terms=self.max_terms,max_active=self.max_active,max_degree=self.max_degree,stage=self.stage,max_allocated_terms=self.max_allocated)

def check(v,why):
 global CHECKS
 CHECKS+=1
 assert CHECKS<=200000 and time.monotonic()-START<80,'check/time cap'
 assert v,why

def add(a,b,budget):
 budget.storage(len(a)+len(b));c=dict(a)
 for m,v in b.items():
  value=c.get(m,0)^v
  if value:
   if m not in c and len(c)>=budget.coefficient_limit:raise Cap('coefficient monomial preflight')
   c[m]=value;budget.max_degree=max(budget.max_degree,m.bit_count())
  else:c.pop(m,None)
  budget.inspect(c)
 return c

def scale(a,v,f,budget):
 budget.reserve(len(a));budget.storage(len(a));out={m:f.mul(c,v)for m,c in a.items()if c and v};budget.inspect(out);return out

def multiply(a,b,f,budget):
 # Refuse a conservative work upperbound BEFORE materializing pair products.
 budget.reserve(len(a)*len(b));budget.storage(len(a)*len(b));out={};iterations=0
 for m,x in a.items():
  for n,y in b.items():
   iterations+=1
   if iterations%256==0 and time.monotonic()-START>=78:raise Cap('work deadline')
   key=m|n;v=out.get(key,0)^f.mul(x,y)
   if v:
    if key not in out and len(out)>=budget.coefficient_limit:raise Cap('coefficient monomial preflight')
    out[key]=v;budget.max_degree=max(budget.max_degree,key.bit_count())
   else:out.pop(key,None)
   budget.inspect(out)
 return out

def frobenius(a,f,budget):
 budget.storage(len(a));out={m:f.square(v)for m,v in a.items()};budget.inspect(out);return out

def remainder_powers(H,f,budget):
 degree=len(H)-1;budget.storage(1);powers=[[{0:1}]+[{}for _ in range(degree-1)]]
 for exponent in range(1,2*degree-1):
  budget.stage='power_X_%d'%exponent;previous=powers[-1];lead=previous[-1];budget.storage(sum(len(v)for v in previous[:-1]));p=[{}]+[dict(v)for v in previous[:-1]]
  for j in range(degree):p[j]=add(p[j],multiply(lead,H[j],f,budget),budget)
  powers.append(p);budget.live(H+[x for row in powers for x in row])
 return powers

def squaremod(R,powers,f,budget,H):
 out=[{}for _ in R]
 for i,p in enumerate(R):
  squared=frobenius(p,f,budget)
  for j,c in enumerate(powers[2*i]):out[j]=add(out[j],multiply(squared,c,f,budget),budget)
 budget.live(H+R+out+[x for row in powers for x in row]);return out

def symbolic(H,vs,f,budget):
 degree=len(H)-1;budget.storage(1);powers=remainder_powers(H,f,budget);R=[{}, {0:1}]+[{}for _ in range(degree-2)];lams=cacheworker.lambdas(f,vs)
 for j,lam in enumerate(lams):
  budget.stage='subspace_%d'%j;sq=squaremod(R,powers,f,budget,H);R=[add(x,scale(y,lam,f,budget),budget)for x,y in zip(sq,R)]
 for j in range(3):budget.stage='eighth_power_%d'%j;R=squaremod(R,powers,f,budget,H)
 return R

def evaluate(p,bits):return old.xor(v for m,v in p.items()if m&bits==m)
def packpolys(polys):return [[[m,v]for m,v in sorted(p.items())]for p in polys]
def unpackpolys(polys):return [dict(p)for p in polys]

def constant_certificate(residual,f,budget=None):
 monomials=sorted({0}|{m for p in residual for m in p});indices={m:i for i,m in enumerate(monomials)};piv={};rows=[]
 for j,p in enumerate(residual):
  for bit in range(f.degree):
   row=sum(1<<indices[m]for m,v in p.items()if v>>bit&1);rows.append(row);combo=1<<(j*f.degree+bit)
   while row:
    pivot=row.bit_length()-1
    if pivot not in piv:piv[pivot]=(row,combo);break
    row^=piv[pivot][0];combo^=piv[pivot][1]
 target=1;combo=0
 while target:
  pivot=target.bit_length()-1
  if pivot not in piv:return dict(found=False,rank=len(piv),monomials=len(monomials))
  target^=piv[pivot][0];combo^=piv[pivot][1]
 selected=[i for i in range(len(rows))if combo>>i&1]
 if budget is not None:budget.storage(sum(len(p)for p in residual))
 cert=dict(found=True,rank=len(piv),monomials=len(monomials),selected_coordinates=selected,residual=packpolys(residual));verify_certificate(cert,f);return cert

def verify_certificate(cert,f):
 check(cert.get('found')is True,'certificate type');residual=unpackpolys(cert['residual']);out={}
 for coordinate in cert['selected_coordinates']:
  check(isinstance(coordinate,int)and 0<=coordinate<len(residual)*f.degree,'certificate index bound');poly=residual[coordinate//f.degree];bit=coordinate%f.degree
  for m,c in poly.items():
   if c>>bit&1:
    if m in out:out.pop(m)
    else:out[m]=1
 check(out=={0:1},'full constant1 cancellation replay')

def controls():
 records=[];truth_states=0
 for n,mod,vs in [(2,7,[1]),(3,11,[1,2]),(4,19,[1,2,4,8])]:
  f=BinaryField(n,mod);V=[old.xor(v for i,v in enumerate(vs)if mask>>i&1)for mask in range(1<<len(vs))];L=[1]
  for x in V:L=old.pmul(L,[x,1],f)
  products={tuple(old.pmul(old.pmul(old.pmul(old.pmul([a,1],[b,1],f),[c,1],f),[d,1],f),[e,1],f))for a,b,c,d,e in itertools.combinations_with_replacement(V,5)}
  constant=[1]
  for x in ([0,0,1,1,1]if n<4 else[0,1,2,4,8]):constant=old.pmul(constant,[x,1],f)
  cases=[]
  cases.append(([{0:c}if c else{}for c in constant],0,'positive_repeated_or_distinct'))
  for seed in range(3):
   H=[]
   for j in range(5):
    p={0:(3*j+seed)%f.order}
    for i in range(3):
     c=(j*(i+1)+seed+i+1)%f.order
     if c:p[1<<i]=c
    if not p[0]:p.pop(0)
    H.append(p)
   H.append({0:1});cases.append((H,3,'affine_family'))
  for H,width,label in cases:
   budget=Budget();residual=symbolic(H,vs,f,budget);accepted=[]
   for bits in range(1<<width):
    specialized=[evaluate(p,bits)for p in H];expected=powmod_poly(divmod_poly(L,specialized,f)[1],8,specialized,f);got=[evaluate(p,bits)for p in residual]
    check(got==list(expected)+[0]*(5-len(expected)),'complete independent symbolic/truth remainder oracle');support=not any(got);check(support==(tuple(specialized)in products),'independent quintic source-product oracle')
    if support:accepted.append(bits)
    truth_states+=1
   cert=constant_certificate(residual,f)
   if cert['found']:check(not accepted,'refuted tiny cell contains no supported point')
   records.append(dict(n=n,label=label,width=width,H=packpolys(H),residual=packpolys(residual),accepted=accepted,certificate=cert,budget=budget.stats()))
 # Certificate acceptance and corruption are tested independently of search.
 f=BinaryField(2,7);nb,nc=old.map_for(f,0,0,1,0,0);NH=[({0:v}if v else{})for v in nb]
 for i,col in enumerate(nc):
  for j,c in enumerate(col):
   if c:NH[j][1<<i]=c
 nr=symbolic(NH,[1],f,Budget())
 for bits in range(16):
  direct=old.literal(f,0,1,0,1,0,0,bits&3,bits>>2);check(direct==old.value(nb,nc,bits),'tiny literal norm affine map')
  expected=powmod_poly(divmod_poly([0,1,1],direct,f)[1],8,direct,f);check([evaluate(p,bits)for p in nr]==list(expected)+[0]*(5-len(expected)),'independent tiny original norm support oracle')
 cert=constant_certificate([{0:1}],f);check(cert['found'],'constant contradiction fixture');verify_certificate(cert,f)
 bad=dict(cert,selected_coordinates=[])
 try:verify_certificate(bad,f)
 except AssertionError:pass
 else:raise AssertionError('corrupt certificate admitted')
 # The two shared bits of u yield a genuinely nonlinear residual coefficient.
 values=[f.mul(f.square(u),u)^u for u in range(4)];check(values[2]^values[3]!=values[0]^values[1],'nonlinear support residual counterexample')
 cap=Budget(total=0)
 try:multiply({0:1},{0:1},f,cap)
 except Cap:pass
 else:raise AssertionError('product cap not enforced')
 cover_check([dict(depth=1,prefix=0),dict(depth=1,prefix=1)])
 try:cover_check([dict(depth=1,prefix=0)])
 except AssertionError:pass
 else:raise AssertionError('corrupt prefix cover admitted')
 return dict(records=records,truth_states=truth_states,corrupt_certificate_rejected=True,nonlinear_values=values,product_cap_control=True)

def actual_family():
 data=json.loads((ROOT/'research/advance/instances/n83-d18-m5-T001-offset0.json').read_text());spec=data['extension_field'];f=BinaryField(spec['degree'],int(spec['modulus_hex'],16));vs=[int(v,16)for v in data['source_basis_hex']];A=int(data['curve']['a_hex'],16);B=int(data['curve']['b_hex'],16);z=int(data['target']['x_hex'],16);y=int(data['target']['y_hex'],16)^z;h=0x4208336d0d441df79c400;k=0
 check(f.degree==83 and f.modulus==0x800000000200000000007 and vs==[1<<i for i in range(18)]and int(data.get('source_offset_hex','0'),16)==0,'actual field/domain identity')
 check(B!=0 and f.square(y)^f.mul(z,y)==f.mul(f.square(z),z)^f.mul(A,f.square(z))^B,'actual target curve')
 base,columns=old.map_for(f,A,z,y,h,k);W,rows,family=old.relaxation(f,18,base,columns);check(family['consistent']and family['rank']==93 and family['nullity']==73,'actual fixedcell73family reproduced')
 rb=old.value(base,columns,family['particular']);rc=[[x^v for x,v in zip(old.value(base,columns,t),base)]for t in family['basis']]
 for t in [family['particular']]+family['basis']:
  r=t&((1<<83)-1);s=t>>83;check(old.value(base,columns,t)==old.literal(f,A,B,z,y,h,k,r,s),'actual constant/kernel literal norm replay')
 proof=dict(h=h,k=k,rows=rows,family=family,H_base=rb,H_columns=rc,product_span_dimensions=[len(v)for v in W]);return f,vs,rb,rc,proof

def restricted(base,cols,depth,prefix):
 out=[]
 for j,b in enumerate(base):
  c=b^old.xor(cols[i][j]for i in range(depth)if prefix>>i&1);p={0:c}if c else{}
  for i in range(depth,len(cols)):
   if cols[i][j]:p[1<<i]=cols[i][j]
  out.append(p)
 return out

def cover_check(leaves):
 check(sum(1<<(73-r['depth'])for r in leaves)==1<<73,'exact73bit cover mass')
 for i,a in enumerate(leaves):
  for b in leaves[i+1:]:
   d=min(a['depth'],b['depth']);check((a['prefix']&((1<<d)-1))!=(b['prefix']&((1<<d)-1)),'prefix-free disjoint leaf cover')

def actual():
 f,vs,base,cols,proof=actual_family();budget=Budget();queue=collections.deque([(0,0)]);leaves=[];nodes=[];proofbytes=0;stopped=None
 while queue:
  if len(nodes)>=511 or budget.total>=budget.total_limit or time.monotonic()-START>=76:stopped='global traversal cap';break
  depth,prefix=queue.popleft();budget.node=0;budget.allocated=0;budget.stage='node_start';status='unknown';cert=None;reason=None
  try:
   budget.storage(6*(74-depth));H=restricted(base,cols,depth,prefix);budget.live(H);residual=symbolic(H,vs,f,budget);cert=constant_certificate(residual,f,budget)
   if cert['found']:
    size=len(json.dumps(cert,separators=(',',':')).encode())
    if proofbytes+size>2*1024*1024:reason='certificate evidence cap';cert=None
    else:
     retained=sum(len(p)for p in residual)
     if budget.allocated+budget.proof_storage+retained>budget.active_limit:raise Cap('certificate retention allocation cap')
     proofbytes+=size;status='rejected';budget.proof_storage+=retained
   else:reason='constant1 absent from exact residual span'
  except Cap as exc:
   cert=None;reason=str(exc)
   if reason in ['global product budget','work deadline']:stopped=reason
  except Exception as exc:
   reason=type(exc).__name__+': '+str(exc);stopped='unexpected node failure'
  record=dict(depth=depth,prefix=prefix,status=status,reason=reason,budget=budget.stats(),symbolic_residual_rank=cert['rank']if cert else None,certificate=cert if cert and cert['found']else None);nodes.append(record)
  if status=='rejected':leaves.append(record)
  elif depth<8 and stopped is None:
   # Atomic replacement of this parent by BOTH children preserves its mass.
   queue.extend([(depth+1,prefix),(depth+1,prefix|(1<<depth))])
  else:leaves.append(record)
  if stopped:break
 for depth,prefix in queue:leaves.append(dict(depth=depth,prefix=prefix,status='unknown',reason=stopped or'unvisited frontier'))
 cover_check(leaves);rejected=sum(1<<(73-r['depth'])for r in leaves if r['status']=='rejected');remaining=(1<<73)-rejected
 return dict(field_degree=83,shared_parameter_bits=73,proof=proof,nodes=nodes,frontier=leaves,processed_nodes=len(nodes),rejected_leaves=sum(r['status']=='rejected'for r in leaves),rejected_parameter_mass=rejected,unresolved_parameter_mass=remaining,cover_mass=1<<73,whole_fixed_norm_cell_rejected=remaining==0,original_global_complete=False,budget=budget.stats(),stopped=stopped,enumerated_2_to_73=False)

def main():
 ident=sys.argv[1];out={};status='passed';error=None
 try:out['controls']=controls();out['actual']=actual()
 except Exception as exc:
  status='inconclusive';error=type(exc).__name__+': '+str(exc)
  if 'actual'not in out:out['actual']=dict(frontier=[dict(depth=0,prefix=0,status='unknown',reason=error)],cover_mass=1<<73,parameterization_verified=False,unresolved_parameter_mass=1<<73,whole_fixed_norm_cell_rejected=False)
 raw=dict(out=out,predicates=CHECKS,error=error);encoded=json.dumps(raw,sort_keys=True,separators=(',',':')).encode()
 if len(encoded)>RAWMAX:
  status='inconclusive';error='4MiB raw evidence cap';raw=dict(error=error,predicates=CHECKS,complete=False,oversize_bytes=len(encoded));encoded=json.dumps(raw).encode()
 blob=gzip.compress(encoded,mtime=0)
 with(P/'raw.json.gz').open('xb')as s:s.write(blob)
 actualout=out.get('actual',{});summary={k:v for k,v in actualout.items()if k not in ['proof','nodes','frontier']};
 if status!='passed':
  summary['whole_fixed_norm_cell_rejected']=False;summary['rejected_parameter_mass']=None;summary['unresolved_parameter_mass']=None;summary['cover_mass']=None
 summary['control_truth_states']=out.get('controls',{}).get('truth_states');summary['frontier_depth_counts']={str(d):sum(r['depth']==d for r in actualout.get('frontier',[]))for d in range(9)}
 evidence=dict(details=summary,error=error,predicates=CHECKS,raw_sha256=hashlib.sha256(blob).hexdigest(),raw_bytes=len(blob),raw_uncompressed_bytes=len(encoded),peak_process_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,exact_prefix_cover_completed=status=='passed'and bool(actualout),complete_global_root_search=False,original_root_count_exact=None,qualified_speedup=None,release_gate_claim=False,novelty_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=ident,status=status,summary='Bounded exact Boolean-ANF support certificates on actual shared73bit norm cell; capped leaves remain UNKNOWN.',evidence=evidence,next_actions=['If cap/no-refutation occurs, use recorded first growth stage and shared coefficients to devise a different cell invariant or restricted multiplication closure; do not retry identical symbolic traversal. Preserve rank4/all-distinct and global-original obligations.'])
 text=json.dumps(result,separators=(',',':'))
 if time.monotonic()-START>=84:result['status']='inconclusive';evidence['exact_prefix_cover_completed']=False;evidence['error']='late serialized report';summary['whole_fixed_norm_cell_rejected']=False;summary['rejected_parameter_mass']=None;summary['unresolved_parameter_mass']=None;summary['cover_mass']=None;text=json.dumps(result)
 assert len(text.encode())<=32768 and time.monotonic()-START<88,'compact final deadline';print(text)
if __name__=='__main__':main()
