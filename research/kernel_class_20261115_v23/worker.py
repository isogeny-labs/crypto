"""Exact low-degree kernel-direction class using linear pencils and duals."""
from pathlib import Path
import gzip,hashlib,json,resource,sys,time
from research.kernel_incidence_20261115_v20 import worker as inc
b=inc.b;P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()
def xor(values):
 v=0
 for x in values:v^=x
 return v
def assemble(tables,u):
 return [(xor(table[j][0]for i,table in enumerate(tables)if u>>i&1),xor(table[j][1]for i,table in enumerate(tables)if u>>i&1))for j in range(len(tables[0]))]
def duals(tables,n):
 m=len(tables[0]);constraints=[]
 for table in tables:
  for j in range(n):constraints.append(sum(((row[0]>>j)&1)<<i for i,row in enumerate(table)))
 reduced=list(b.norm.basis(constraints).values());sol=b.norm.solve([(v,0)for v in reduced],m);out=[]
 for lam in sol['basis']:
  syndrome=sum((xor(rhs for j,(mask,rhs)in enumerate(table)if lam>>j&1))<<i for i,table in enumerate(tables))
  b.check(all(xor(mask for j,(mask,rhs)in enumerate(table)if lam>>j&1)==0 for table in tables),'common dual annihilates every pencil coefficient')
  out.append(dict(multiplier=lam,syndrome=syndrome))
 return dict(dimension=len(out),syndrome_rank=len(b.norm.basis(x['syndrome']for x in out)),vectors=out)
def controls():
 pencils=checks=0
 for code in range(4096):
  values=[(code>>(3*i))&7 for i in range(4)];tables=[[(v&3,v>>2)for v in values[:2]],[(v&3,v>>2)for v in values[2:]]];du=duals(tables,2)
  for u in[1,2,3]:
   rows=assemble(tables,u);sol=b.norm.solve(rows,2);expected=[x for x in range(4)if all((m&x).bit_count()%2==r for m,r in rows)]
   found=[]
   if sol['consistent']:
    for t in range(1<<sol['nullity']):found.append(sol['particular']^xor(v for i,v in enumerate(sol['basis'])if t>>i&1))
   b.check(sorted(found)==expected,'complete tiny bilinear pencil oracle')
   common=any((v['syndrome']&u).bit_count()%2 for v in du['vectors']);b.check(not common or not expected,'dual class rejection sound on exhaustive tiny oracle');checks+=1
  pencils+=1
 return dict(all_two_by_two_two_parameter_pencils=pencils,complete_nonzero_direction_sets=checks,all_inner_solution_sets_equal=True,common_dual_rejections_sound=True)
def actual():
 D=json.loads((R/'research/advance/instances/n83-d18-m5-T001-offset0.json').read_text());f=b.BinaryField(83,int(D['extension_field']['modulus_hex'],16));A=int(D['curve']['a_hex'],16);z=int(D['target']['x_hex'],16);y=int(D['target']['y_hex'],16)^z;h=0x4208336d0d441df79c400
 old=json.loads(gzip.decompress((R/'research/full_quintic_block_20261115_v9/raw.json.gz').read_bytes()))['out']['profile']['proofs']['baseline166'];gold=json.loads(gzip.decompress((R/'research/joint_cubic_20261115_v15/raw.json.gz').read_bytes()))['out']['actual']['family_certificate']
 families=[('pre-Gold73',old['particular'],old['basis']),('Gold69',gold['raw_particular'],gold['raw_basis'])];out=[];summary={}
 probes={1<<i for i in range(9)}|{((j*73)^0x15b)&511 for j in range(24)};probes.discard(0)
 for name,raw0,B in families:
  tables=[inc.intersection(f,A,z,y,h,1<<i,18,raw0,B)['rows']for i in range(9)];du=duals(tables,len(B));records=[];ranks={};bydegree={};common_count=0;consistent=0;previous=0;rows=[(0,0)]*45;direct=0
  for step in range(1,512):
   u=step^(step>>1);flip=u^previous;bit=flip.bit_length()-1;rows=[(m^tables[bit][j][0],r^tables[bit][j][1])for j,(m,r)in enumerate(rows)];previous=u
   if u in probes:
    literal=inc.intersection(f,A,z,y,h,u,18,raw0,B)['rows'];b.check(rows==literal,'direct field arithmetic versus exact Gray pencil');direct+=1
   sol=b.norm.solve(rows,len(B));common=[v for v in du['vectors']if (v['syndrome']&u).bit_count()%2]
   b.check(not common or not sol['consistent'],'common dual versus fullsolve')
   if common:common_count+=1
   record=dict(u=u,rank=sol['rank'],consistent=sol['consistent'])
   if sol['consistent']:
    consistent+=1;record['particular']=sol['particular'];b.check(all((m&sol['particular']).bit_count()%2==r for m,r in rows),'positive incidence witness')
   else:record['contradiction']=sol['contradiction']
   records.append(record);ranks[str(sol['rank'])]=ranks.get(str(sol['rank']),0)+1;degree=str(u.bit_length()-1);bucket=bydegree.setdefault(degree,dict(consistent=0,inconsistent=0));bucket['consistent'if sol['consistent']else'inconsistent']+=1
  b.check(len({r['u']for r in records})==511,'complete all nonzero low9directions')
  summary[name]=dict(directions=511,consistent=consistent,inconsistent=511-consistent,all_low_degree_nonzero_excluded=consistent==0,common_dual_dimension=du['dimension'],common_dual_syndrome_rank=du['syndrome_rank'],common_dual_exclusions=common_count,rank_histogram=ranks,degree_histogram=bydegree,direct_field_probes=direct)
  out.append(dict(family=name,parameter_bits=len(B),basis_tables=tables,common_dual=du,records=records))
 return dict(summary=summary,families=out,scope='All511nonzero low9bit kernel directions, fixedh/basek0spanfamilies; no full35bitdirection/outer/globalrootcompletion')
def main():
 out={};error=None
 try:out['controls']=controls();out['actual']=actual()
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode();assert len(raw)<512*1024
 with(P/'raw.json').open('xb')as f:f.write(raw)
 ev=dict(error=error,controls=out.get('controls'),actual=out.get('actual',{}).get('summary'),worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_bytes=len(raw),raw_sha256=hashlib.sha256(raw).hexdigest(),complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False,external_reviewer=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Complete low9kernel-direction class and common-dual test with Gray pencils, not generic low-degree inference from unit samples.',evidence=ev,next_actions=['If excludedclass holds, derive guarded low/highkblock consequences and seek an algebraic common certificate; ifconstantdualsfail, retain all per-direction contradictions and test a changed bounded u-dependent multiplier class. Otherdirections/outercharts/sourceclosure remainopen.'])
 text=json.dumps(result,separators=(',',':'));assert len(text.encode())<8192 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
