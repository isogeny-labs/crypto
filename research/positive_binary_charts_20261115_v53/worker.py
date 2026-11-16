from pathlib import Path
from types import SimpleNamespace
import collections,hashlib,json,resource,sys,time
from rootfinder.binaryfield import BinaryField
from research.positive_binary_charts_20261115_v53 import math as m
from research.general_binary_points_20261115_v52 import solver as general
from research.compiled_subspace_20261115_v31 import factor
P=Path(__file__).resolve().parent;START=time.monotonic()

def case(f,rule,kind,seed):
 A=rule['A'];B=rule['B'];gen=m.Generator(f,A,B,seed,18);beta=m.sqrt(f,B);attempts=[]
 for attempt in range(64):
  used=set();points=[]
  for _ in range(5 if kind=='rational5'else 2):
   Q=gen.point(used);used.add(Q[0]);points.append(Q)
  source=points if kind=='rational5'else[(0,beta)]+points;T=m.total(f,A,B,source)
  if T is None or T[0]<1<<18:attempts.append(dict(attempt=attempt,rejected='target outside-domain rule',points=points));continue
  if rule['require_trace_one']and m.trace(f,A^T[0])!=1:attempts.append(dict(attempt=attempt,rejected='trace-one coverage rule',points=points));continue
  break
 else:raise RuntimeError('target generation cap')
 assert m.total(f,A,B,list(reversed(source)))==T;z=T[0];yT=T[1]^z;target=(z,yT)
 if kind=='rational5':
  V=m.F6(f,[target]+points);xs=[x for x,y in points];partial=None;aux=[]
  for i,Q in enumerate(points):
   partial=m.add(f,A,B,partial,Q);assert partial is not None
   if i in [1,2,3]:aux.append(partial[0])
  point_lifts={x:y for x,y in points}
 else:
  xp=gen.ghost_x(used);r2,s2,h2=m.F4(f,[target]+source);V=[r2^xp,s2^f.mul(xp,r2),f.mul(xp,s2),h2,f.mul(xp,h2)];assert h2 and V[4]
  xs=[xp,0,xp,points[0][0],points[1][0]];aux=[f.mul(beta,f.inv(xp)),0,f.mul(beta,f.inv(points[0][0]))];point_lifts={0:beta,points[0][0]:points[0][1],points[1][0]:points[1][1]}
 H=m.polynomial.norm(f,A,B,z,V);expected=(1,)
 for x in xs:expected=m.polynomial.b._mul(expected,(x,1),f)
 assert tuple(H)==tuple(expected)
 extracted=factor.supported_roots(SimpleNamespace(p=2,field=f),H,[1<<i for i in range(18)]);assert extracted['supported']and extracted['roots']==sorted(collections.Counter(xs).items())
 r,ss,t,h,k=V;base,cols,sol=general.model(f,A,B,z,yT,h,k,18);assert sol is not None;packed=r|(ss<<83);tag=m.coordinates(sol['basis'],packed^sol['particular']);assert tag is not None;S=general.c.split(f,sol);root_checks=[]
 for x in sorted(set(xs)):
  G=f.mul(h,x)^k;y=None if not G else point_lifts[x];point=general.point(f,z,yT,h,k,S,x,y);assert point is not None;assert m.coordinates(point['basis'],tag^point['particular'])is not None;root_checks.append(dict(x=x,G_zero=G==0,nonrational=x not in point_lifts))
 values=xs+aux;res=m.residuals_literal(values,z,B,f.modulus);assert res==[0]*4
 for a,bb,c in [(xs[0],xs[1],aux[0]),(aux[0],xs[2],aux[1]),(aux[1],xs[3],aux[2]),(aux[2],xs[4],z)]:assert m.S3(f,B,a,bb,c)==0
 mutation=None
 for bit in range(83):
  bad=values[:];bad[5]^=1<<bit
  if any(m.residuals_literal(bad,z,B,f.modulus)):mutation=bit;break
 assert mutation is not None
 root_bits=sum(x<<(18*i)for i,x in enumerate(xs))+sum(x<<(90+83*i)for i,x in enumerate(aux));assert root_bits.bit_length()<=339
 return dict(curve=rule['name'],A=A,B=B,kind=kind,seed=seed,generation_rejections=attempts,source_rational_points=source,target_T=T,norm_target_Q=target,norm_coefficients=V,H=H,expected_source_x=xs,factorization=extracted,affine_dimension=len(sol['basis']),r_changing_dimension=len(S['U']),constant_s_dimension=len(S['kernel']),known_compact_parameter=tag,point_checks=root_checks,trace_A_plus_z=m.trace(f,A^z),original_field_variables=values,original_root_bits_hex=hex(root_bits),original_variable_count=339,original_bit_equations=332,original_residuals=res,rejected_mutation_bit=mutation,scope='Positive witnesses and complete factorization of constructed H; not a global multivariate all-roots solve')

def main():
 out=dict(cases=[]);error=None
 try:
  rules=json.loads((P/'DATA.json').read_text());f=BinaryField(83,0x800000000200000000007)
  for rule in rules['curves']:
   for j,kind in enumerate(['rational5','zero_ghost_pair']):
    record=case(f,rule,kind,rules['seed']+1009*j+rule['seed_offset']);out['cases'].append(record)
    instance=dict(field_degree=83,modulus=f.modulus,d=18,A=record['A'],B=record['B'],z=record['norm_target_Q'][0],y=record['norm_target_Q'][1],h=record['norm_coefficients'][3],k=record['norm_coefficients'][4],contract='all supported norm parameters in this fixedh/k chart; originalchain output requires reconstruction')
    with(P/('instance_'+rule['name']+'_'+kind+'.json')).open('x')as file:json.dump(instance,file,sort_keys=True,separators=(',',':'))
    assert time.monotonic()-START<70
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 trial=json.dumps(dict(out=out,error=error),separators=(',',':'))
 if time.monotonic()-START>=78:error='late serialized result'
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as f:f.write(raw)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Positive full-width binary norm witnesses and independently verified339bit original-chain membership.',evidence=dict(error=error,cases=[{k:r[k]for k in ['curve','kind','affine_dimension','r_changing_dimension','constant_s_dimension','trace_A_plus_z','original_variable_count','original_bit_equations']}for r in out['cases']],worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),qualified_speedup=None,complete_global_root_search=False,release_gate_claim=False),next_actions=['Port generic r/s point scan to native backend and close each positive conditional chart against complete generic-matrix oracle before timing.'])
 text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
