from pathlib import Path
import json,lzma,hashlib,time,itertools
from .binaryfield import BinaryField
from .finite_fiber import BinaryQuadraticFiber,five_source_intermediate_fiber
from research.twist_auxiliary_20261116_v86.algebra import Field,s3

def oracles():
 f=BinaryField(2,7);solver=BinaryQuadraticFiber(f);g=Field(2,[1,1,1]);checks=0
 for i in range(12):
  xs=tuple((i//(j+1))%4 for j in range(5));target=i%4;got=five_source_intermediate_fiber(f,xs,target,quadratic_solver=solver)['intermediate_chains'];expected=[]
  for u in itertools.product(range(4),repeat=3):
   links=[(xs[0],xs[1],u[0]),(u[0],xs[2],u[1]),(u[1],xs[3],u[2]),(u[2],xs[4],target)]
   if all(s3(*v,0,1,g)==0 for v in links):expected.append(u)
  assert [tuple(v)for v in got]==sorted(expected);checks+=1
 return checks

def recover(sources,P,data,start):
 raw=lzma.decompress((P/data['instance']).read_bytes());assert hashlib.sha256(raw).hexdigest()==data['instance_sha256'];d=json.loads(raw);mod=int(d['extension_field']['modulus_hex'],16);assert d['source_dimension']==18 and d['source_bits']==90 and d['n_vars']==339 and d['equation_count']==332;assert [int(v,0)for v in d['source_basis_hex']]==[1<<j for j in range(18)];f=BinaryField(83,mod);solver=BinaryQuadraticFiber(f);target=int(d['target']['x_hex'],16);B=int(d['curve']['b_hex'],16);tuples=[];count=0;complete=True
 for xs in sorted(set(itertools.permutations(sources))):
  if time.monotonic()-start>=data['recovery_deadline_seconds']:complete=False;break
  result=five_source_intermediate_fiber(f,xs,target,curve_b=B,quadratic_solver=solver)
  for chain in result['intermediate_chains']:
   coords=xs+tuple(chain);encoded=sum(value<<row['offset']for value,row in zip(coords,d['variable_layout']));assert all(0<=x<1<<18 for x in xs);timed=False
   for eq in d['equations']:
    if time.monotonic()-start>=data['recovery_deadline_seconds']:timed=True;break
    parity=0
    for monomial in eq:parity^=int(all(encoded>>bit&1 for bit in monomial))
    assert parity==0,'originalANFresidual'
   if timed:complete=False;break
   tuples.append({'field_coordinates':coords,'original_encoding_hex':hex(encoded)})
  if not complete:break
  count+=1
 assert len({v['original_encoding_hex']for v in tuples})==len(tuples)
 return {'complete_for_this_source_multiset':complete,'checked_orderings':count,'total_distinct_orderings':len(set(itertools.permutations(sources))),'verified_original_tuples':tuples,'verified_original_tuple_lower_bound':len(tuples),'original_instance_sha256':data['instance_sha256'],'whole_original_system_complete':False}
