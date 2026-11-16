"""Exact odd-XOR obstructions to universal constant residual functionals."""
from pathlib import Path
import sys,json,time,gzip,hashlib,resource,itertools
from rootfinder.binaryfield import BinaryField,divmod_poly,powmod_poly
from research.quartic_cell_20261114_v1 import worker as old
from research.cubic_cached_20261115_v4 import worker as cw
P=Path(__file__).resolve().parent;ROOT=P.parents[1];START=time.monotonic();CHECKS=0;RAWMAX=4*1024*1024;GZMAX=1024*1024

def check(v,why):
 global CHECKS
 CHECKS+=1
 assert CHECKS<=200000 and time.monotonic()-START<80,'predicate/time cap'
 assert v,why

def packed(values,n):return sum(v<<(i*n)for i,v in enumerate(values))
def trim(values):
 values=list(values)
 while values and values[-1]==0:values.pop()
 return values

def support(f,H,lambdas):
 # Successive exact monic reductions for X5..X8, then Frobenius acts
 # on five coordinates with only the X6 and X8 reductions nontrivial.
 powers=[list(H[:5])]
 for _ in range(3):
  previous=powers[-1];lead=previous[4]
  powers.append([(previous[j-1]if j else 0)^f.mul(lead,H[j])for j in range(5)])
 p6,p8=powers[1],powers[3]
 def square(r):
  a=[f.square(x)for x in r];out=[a[0],0,a[1],0,a[2]]
  for j in range(5):out[j]^=f.mul(a[3],p6[j])^f.mul(a[4],p8[j])
  return out
 R=[0,1,0,0,0]
 for lam in lambdas:R=[x^f.mul(lam,y)for x,y in zip(square(R),R)]
 R8=square(square(square(R)))
 return R,R8

def witness(values,width):
 piv={};point_combos={}
 for index,value in enumerate(values):
  row=value|(1<<width);combo=1<<index
  while row:
   p=row.bit_length()-1
   if p not in piv:piv[p]=row;point_combos[p]=combo;break
   row^=piv[p];combo^=point_combos[p]
 target=1<<width;combo=0
 while target:
  p=target.bit_length()-1
  if p not in piv:return dict(found=False,rank=len(piv),selected=[],pivots=[[i,v]for i,v in sorted(piv.items())])
  target^=piv[p];combo^=point_combos[p]
 selected=[i for i in range(len(values))if combo>>i&1]
 result=dict(found=True,rank=len(piv),selected=selected,pivots=[[i,v]for i,v in sorted(piv.items())]);verify_witness(values,result);return result

def verify_witness(values,proof):
 selected=proof['selected'];check(proof['found']and len(selected)%2==1,'odd witness cardinality');check(len(set(selected))==len(selected)and all(isinstance(i,int)and 0<=i<len(values)for i in selected),'witness distinct valid indices');check(old.xor(values[i]for i in selected)==0,'exact odd XOR zero replay')

def anf_feasible(values,width,variables):
 # Independent full-cube Mobius ANFs, then brute force every functional.
 coefficients=list(values)
 for bit in range(variables):
  for mask in range(len(values)):
   if mask>>bit&1:coefficients[mask]^=coefficients[mask^(1<<bit)]
 feasible=[]
 for functional in range(1<<width):
  if all(((functional&c).bit_count()%2)==(mask==0)for mask,c in enumerate(coefficients)):feasible.append(functional)
 return feasible,coefficients

def controls():
 cases=[]
 for values,width in [([0,1,2,3],2),([1,1,1,1],1),([1,2,3,0],2),([1,2,3,1,2,3,0,0],2)]:
  variables=(len(values)-1).bit_length();feasible,coefficients=anf_feasible(values,width,variables);proof=witness(values,width);check(proof['found']==(not feasible),'full cube ANF/dual equivalence');cases.append(dict(values=values,width=width,feasible_functionals=feasible,ANF=coefficients,proof=proof))
 proof=witness([1,2,3],2);check(proof['found'],'nonzero odd triple obstruction');verify_witness([1,2,3],proof)
 for bad in [dict(proof,selected=[]),dict(proof,selected=[0]),dict(proof,selected=[0,1,9])]:
  try:verify_witness([1,2,3],bad)
  except AssertionError:pass
  else:raise AssertionError('corrupt witness admitted')
 check(not witness([1,1],1)['found']and witness([1,1,0],1)['found'],'misleading proper sample is not universal identity')
 arithmetic=[]
 for n,mod,vs in [(2,7,[1]),(3,11,[1,2]),(4,19,[1,2,8])]:
  f=BinaryField(n,mod);lams=cw.lambdas(f,vs);V=[old.xor(v for i,v in enumerate(vs)if mask>>i&1)for mask in range(1<<len(vs))];L=[1]
  for v in V:L=old.pmul(L,[v,1],f)
  for seed in range(16):
   H=[(seed*(j+1)+j*j)%f.order for j in range(5)]+[1];R,R8=support(f,H,lams);direct=divmod_poly(L,H,f)[1];expected=powmod_poly(direct,8,H,f);check(R==list(direct)+[0]*(5-len(direct))and R8==list(expected)+[0]*(5-len(expected)),'independent finite-field quintic remainder oracle');arithmetic.append(dict(n=n,H=H,R=R,R8=R8))
 # Preserve original norm arithmetic checks, separately from arbitrary H.
 f=BinaryField(2,7);lams=cw.lambdas(f,[1])
 for r,s in itertools.product(range(4),repeat=2):
  H=old.literal(f,0,1,0,1,0,0,r,s);check(H==old.formula(f,0,0,1,0,0,r,s),'tiny literal norm formula oracle');R,R8=support(f,H,lams);check(trim(R8)==list(powmod_poly(divmod_poly([0,1,1],H,f)[1],8,H,f)),'tiny original norm support oracle')
 return dict(cases=cases,arithmetic=arithmetic,corrupt_witness_rejected=True,misleading_fit_rejected=True)

def setup():
 data=json.loads((ROOT/'research/advance/instances/n83-d18-m5-T001-offset0.json').read_text());spec=data['extension_field'];f=BinaryField(spec['degree'],int(spec['modulus_hex'],16));vs=[int(v,16)for v in data['source_basis_hex']];A=int(data['curve']['a_hex'],16);B=int(data['curve']['b_hex'],16);z=int(data['target']['x_hex'],16);y=int(data['target']['y_hex'],16)^z;h=0x4208336d0d441df79c400;k=0
 check(f.degree==83 and f.modulus==0x800000000200000000007 and vs==[1<<i for i in range(18)]and int(data.get('source_offset_hex','0'),16)==0,'actual modulus/domain');check(B!=0 and f.square(y)^f.mul(z,y)==f.mul(f.square(z),z)^f.mul(A,f.square(z))^B,'actual curve target')
 base,columns=old.map_for(f,A,z,y,h,k);W,rows,family=old.relaxation(f,18,base,columns);check(family['consistent']and family['rank']==93 and family['nullity']==73,'exact actual relaxed73bit family')
 rb=old.value(base,columns,family['particular']);rc=[[a^b for a,b in zip(old.value(base,columns,v),base)]for v in family['basis']]
 return f,A,B,z,y,h,k,vs,cw.lambdas(f,vs),dict(rows=rows,family=family,H_base=rb,H_columns=rc,span_dimensions=[len(w)for w in W])

def actual():
 start=time.monotonic();f,A,B,z,y,h,k,vs,lams,proof=setup();setup_seconds=time.monotonic()-start;table=[];lookup={};cells=[];selection=[(0,0)]+[(d,p)for d in [1,2]for p in range(1<<d)];stopped=None
 for depth,prefix in selection:
  values=[];indices=[];error=None
  for index in range(512):
   if time.monotonic()-START>=73:stopped='evaluation deadline';break
   t=prefix|(index<<depth)
   try:
    if t not in lookup:
     raw=proof['family']['particular']^old.xor(v for bit,v in enumerate(proof['family']['basis'])if t>>bit&1);r=raw&((1<<83)-1);s=raw>>83;H=old.value(proof['H_base'],proof['H_columns'],t);check(H==old.literal(f,A,B,z,y,h,k,r,s),'paid original norm point replay');R,R8=support(f,H,lams);lookup[t]=len(table);table.append([t,raw,H,R,R8])
    pi=lookup[t];indices.append(pi);values.append(packed(table[pi][4],83))
   except Exception as exc:stopped=type(exc).__name__+': '+str(exc);error=stopped;break
  obstruction=witness(values,415)if values else dict(found=False,rank=0,selected=[],pivots=[])
  if obstruction['found']:
   verify_witness(values,obstruction);selected_table=[indices[i]for i in obstruction['selected']];class_status='constant_functional_class_impossible'
  else:selected_table=[];class_status='unknown'
  cells.append(dict(depth=depth,prefix=prefix,point_indices=indices,selected_table_indices=selected_table,proof=obstruction,class_status=class_status,all512_points_evaluated=len(indices)==512,error=error))
  if stopped:break
 for depth,prefix in selection[len(cells):]:cells.append(dict(depth=depth,prefix=prefix,class_status='unknown',point_indices=[],selected_table_indices=[],all512_points_evaluated=False,error=stopped or'unvisited'))
 return dict(field_degree=83,shared_parameter_bits=73,h=h,k=k,proof=proof,lambdas=lams,point_schema=['shared_t','raw_rs166','Hascending','LV_remainder5','eighth_power_residue5'],points=table,cells=cells,unique_evaluated_points=len(table),point_occurrences=sum(len(c['point_indices'])for c in cells),all_declared_points_evaluated=all(c['all512_points_evaluated']for c in cells),setup_seconds=setup_seconds,total_seconds=time.monotonic()-start,overlapping_diagnostics=True,disjoint_cover=False,only_nine_free_bits_per_cell=True,stopped=stopped,root_cell_class_obstructed=cells[0]['class_status']=='constant_functional_class_impossible',supported_point_observations=sum(not any(p[4])for p in table),cell_unsat_claim=False,original_global_complete=False)

def compact(out):
 actualout=out.get('actual',{});summary={k:v for k,v in actualout.items()if k not in ['proof','lambdas','points','cells','point_schema']};summary['cells']=[dict(depth=c['depth'],prefix=c['prefix'],class_status=c['class_status'],points=len(c['point_indices']),all512_points_evaluated=c['all512_points_evaluated'],rank=c.get('proof',{}).get('rank'),witness_size=len(c['selected_table_indices']))for c in actualout.get('cells',[])];summary['small_control_cases']=len(out.get('controls',{}).get('cases',[]));return summary

def clear_obstructions(out):
 actualout=out.get('actual',{});actualout['root_cell_class_obstructed']=None
 for cell in actualout.get('cells',[]):cell['class_status']='unknown';cell['selected_table_indices']=[]

def main():
 ident=sys.argv[1];out={};status='passed';error=None
 try:out['controls']=controls();out['actual']=actual()
 except Exception as exc:status='inconclusive';error=type(exc).__name__+': '+str(exc)
 raw=dict(out=out,predicates=CHECKS,error=error);encoded=json.dumps(raw,sort_keys=True,separators=(',',':')).encode();blob=gzip.compress(encoded,mtime=0)
 if len(encoded)>RAWMAX or len(blob)>GZMAX:
  status='inconclusive';error='raw4MiB/compressed1MiB cap';clear_obstructions(out);raw=dict(error=error,complete=False,raw_record_omitted=True,uncompressed_bytes=len(encoded),compressed_bytes=len(blob));encoded=json.dumps(raw).encode();blob=gzip.compress(encoded,mtime=0)
 with(P/'raw.json.gz').open('xb')as stream:stream.write(blob)
 evidence=dict(details=compact(out),error=error,predicates=CHECKS,raw_path=str((P/'raw.json.gz').relative_to(ROOT)),raw_sha256=hashlib.sha256(blob).hexdigest(),raw_bytes=len(blob),raw_uncompressed_bytes=len(encoded),peak_process_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,class_diagnostic_only=True,cell_unsat_claim=False,complete_global_root_search=False,original_root_count_exact=None,qualified_speedup=None,release_gate_claim=False,novelty_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=ident,status=status,summary='Exact odd-XOR witnesses refute only universal constant-functional support certificates on overlapping fixed norm cells; no cell/root UNSAT.',evidence=evidence,next_actions=['If the class is obstructed, design a genuinely different restricted polynomial-multiplier or whole-cell invariant with exact proof and cost; if no witness, sampled functional fits still require a universal identity proof before pruning.'])
 text=json.dumps(result,separators=(',',':'))
 if time.monotonic()-START>=84:
  result['status']='inconclusive';evidence['error']='late serialized report';clear_obstructions(out);evidence['details']=compact(out);text=json.dumps(result)
 assert len(text.encode())<=32768 and time.monotonic()-START<88,'compact final report deadline';print(text)
if __name__=='__main__':main()
