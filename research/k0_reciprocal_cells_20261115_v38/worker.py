"""Nonlinear whole source-cell filter with independent repeated/zero controls."""
from pathlib import Path
import collections,hashlib,itertools,json,math,resource,sys,time
from rootfinder.binaryfield import BinaryField
from research.inversion_free_oracles_20261115_v35 import worker as old
from research.k0_reciprocal_cells_20261115_v38 import solver as s
P=Path(__file__).resolve().parent;START=time.monotonic()
def weight(row):return math.factorial(5)//math.prod(math.factorial(x)for x in collections.Counter(row).values())
def controls():
 profiles=[];bags=states=positive=zero_patterns=0
 for n,m in[(2,7),(3,11),(4,19),(5,37)]:
  f=old.Small(n,m);f.inv=lambda x:next(y for y in range(1,f.order)if f.mul(x,y)==1);D=tuple(range(1<<min(n,3)));nonzero=D[1:]
  allbags=list(itertools.combinations_with_replacement(D,5));bagcoeff=[(row,s.coefficients(f,row))for row in allbags]
  for z in range(1,f.order):
   expected={row for row,H in bagcoeff if H[0]==f.mul(z,H[1])};want={row for row in expected if 0 not in row}
   got=s.solve(f,[nonzero]*5,z);assert got['complete'];gotbags={tuple(sorted(row))for row in got['tuples']};assert gotbags==want;assert len(got['tuples'])==sum(weight(row)for row in want)
   zcases={row for row in expected if 0 in row};assert all(row.count(0)>=2 for row in zcases)
   assert zcases=={row for row in allbags if row.count(0)>=2}
   # Independent literal symmetric sums and reciprocal sum check for each multiset.
   for row,H in bagcoeff:
    product=1
    for x in row:product=f.mul(product,x)
    e4=0
    for j in range(5):
     v=1
     for i,x in enumerate(row):
      if i!=j:v=f.mul(v,x)
     e4^=v
    assert H[0]==product and H[1]==e4;literal=product==f.mul(z,e4)
    if 0 not in row:
     invsum=0
     for x in row:invsum^=f.inv(x)
     assert literal==(invsum==f.inv(z))
    bags+=1;states+=weight(row)
   positive+=got['exact_count'];zero_patterns+=sum(weight(row)for row in zcases)
   profiles.append(dict(n=n,z=z,domain_size=len(D),nonzero_ordered_count=got['exact_count'],zero_ordered_count=sum(weight(row)for row in zcases)))
 return dict(multiset_checks=bags,ordered_assignments_represented=states,nonzero_positive_ordered=positive,zero_positive_ordered=zero_patterns,profiles=profiles)
def actual():
 config=json.loads((P/'DATA.json').read_text());f=BinaryField(83,config['modulus']);z=config['z'];out=[];totalwork=0;covered=0
 for case in config['cases']:
  domains=[[offset^j for j in range(128)]for offset in case['offsets']];result=s.solve(f,domains,z,cap=4096,work_cap=2200000);assert result['complete'];totalwork+=result['work'];covered+=128**5
  out.append(dict(case_id=case['id'],offsets=case['offsets'],complete=True,exact_reciprocal_count=result['exact_count'],tuples=result['tuples'],work=result['work'],source_assignments=128**5))
  assert time.monotonic()-START<78
 return dict(modulus=f.modulus,z=z,cells=out,disjoint_cells=True,source_assignments_covered=covered,cell_cover_bits=55,total_partition_cells=1<<55,unvisited_partition_cells=(1<<55)-len(out),totalwork=totalwork,scope='Exact auxiliary nonzero-source e5=z e4 cell filter; no original curve/chain roots or global completion')
def main():
 out={};error=None
 try:out['controls']=controls();out['actual']=actual()
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode();assert len(raw)<131072
 with(P/'raw.json').open('xb')as f:f.write(raw)
 aa=out.get('actual',{});ev=dict(error=error,controls=out.get('controls'),actual_cells=len(aa.get('cells',[])),source_assignments_covered=aa.get('source_assignments_covered'),totalwork=aa.get('totalwork'),worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Exact nonlinear reciprocal cell join; repeated/zero support controls and8actual disjoint sourcecells.',evidence=ev,next_actions=['Quantify full55bitoutercover and reusable cell exclusion certificates; do not extrapolate sampledcell rejection to globalallroots or turn allsolutions into firstrelation. Preserve zero>=2cubic branch.'])
 text=json.dumps(result,separators=(',',':'));assert len(text)<16384 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
