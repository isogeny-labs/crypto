from pathlib import Path
import itertools,json,hashlib,resource,sys,time,collections
from rootfinder.binaryfield import BinaryField
from research.three_point_line_20261115_v46 import solver as s
from research.certified_support_20261115_v44 import solver as old
from research.k0_reciprocal_cells_20261115_v38.solver import coefficients
P=Path(__file__).resolve().parent;START=time.monotonic()

def controls():
 states=profiles=positive=checks=0;dimension=collections.Counter()
 for n,mod,lifts in [(3,11,8),(4,19,1)]:
  f=BinaryField(n,mod);q=f.order;table=[[f.mul(a,b)for b in range(q)]for a in range(q)]
  found=None
  for A,B in itertools.product(range(q),range(1,q)):
   ys={x:[y for y in range(q)if f.square(y)^table[x][y]==table[f.square(x)][x]^table[A][f.square(x)]^B]for x in range(q)}
   if all(ys[x]for x in [1,2,3])and any(ys[z]for z in range(4,q)):
    z=next(z for z in range(4,q)if ys[z]);found=A,B,z,ys;break
  assert found;A,B,z,ys=found;target=(z,ys[z][0]);supported={tuple(coefficients(f,rs))for rs in itertools.combinations_with_replacement(range(4),5)}
  for choices in list(itertools.product(range(2),repeat=3))[:lifts]:
   points=[target]+[(x,ys[x][c])for x,c in zip([1,2,3],choices)];line=s.interpolate(f,points);assert line and line['rank']==4
   generated={tuple(s.vector(f,line,t))for t in range(q)};literal=set();rows=[]
   for x,y in points[1:]:rows.append(([f.square(x)^f.square(z),x^z,table[x][y]^table[z][target[1]],y^target[1]],table[f.square(x)][x]^table[f.square(z)][z]))
   for r,ss,h,k in itertools.product(range(q),repeat=4):
    states+=1
    if all(s.xor(table[a][b]for a,b in zip(row,[r,ss,h,k]))==rhs for row,rhs in rows):
     t=table[f.square(z)][z]^table[r][f.square(z)]^table[ss][z]^table[h][table[z][target[1]]]^table[k][target[1]];literal.add((r,ss,t,h,k))
   assert generated==literal;compiled=s.compile_line(f,A,B,z,line,2);sol=compiled['solution'];got=set()if sol is None else{sol['particular']^s.linear.apply(sol['basis'],i)for i in range(1<<len(sol['basis']))};want=set()
   for t in range(q):
    H=s.norm(f,A,B,z,s.vector(f,line,t));model=[a^s.xor(c[j]for i,c in enumerate(compiled['columns'])if t>>i&1)for j,a in enumerate(compiled['base'])];assert tuple(model)==H
    if s.pack(f,H,2)[0]==0:want.add(t)
    positive+=H in supported;checks+=1
   assert got==want;profiles+=1;dimension[len(sol['basis'])if sol else -1]+=1
 return dict(literal_coefficient_states=states,point_contexts=profiles,norm_line_checks=checks,positive_supported_norms=positive,residual_dimensions=dict(dimension))

def actual():
 rule=json.loads((P/'DATA.json').read_text());f=BinaryField(83,old.MOD);target=(old.Z,old.QY^old.Z);source=[]
 for x in range(1,rule['x_scan_limit']):
  inv=f.inv(x);a=x^f.square(inv);trace=0;v=a
  for _ in range(83):trace^=v;v=f.square(v)
  if trace:continue
  ht=0;v=a
  for _ in range(42):ht^=v;v=f.square(f.square(v))
  y=f.mul(x,ht);assert f.square(y)^f.mul(x,y)==f.mul(f.square(x),x)^1;source.append((x,y))
  if len(source)==6:break
 assert len(source)==6;records=[]
 for triple in list(itertools.combinations(source,3))[:2]:
  for choices in itertools.product(range(2),repeat=3):
   points=[target]+[(x,y^(x if c else 0))for (x,y),c in zip(triple,choices)];line=s.interpolate(f,points);assert line and line['rank']==4;compiled=s.compile_line(f,0,1,old.Z,line,18);sol=compiled['solution'];row=dict(points=points,rank=None if sol is None else sol['rank'],consistent=sol is not None,residual_dimension=None if sol is None else len(sol['basis']),supported=[],complete=True)
   if sol is not None:
    if len(sol['basis'])>8:row['complete']=False
    else:
     lam=old.compile_basis(f,[1<<i for i in range(18)])[0]
     for i in range(1<<len(sol['basis'])):
      t=sol['particular']^s.linear.apply(sol['basis'],i);V=s.vector(f,line,t);H=s.norm(f,0,1,old.Z,V)
      if old.generic(f,H,lam):row['supported'].append(dict(parameter=t,coefficients=V,H=H))
   records.append(row);assert time.monotonic()-START<65
 return dict(source_points=source,records=records,complete_cells=sum(r['complete']for r in records),empty_linear_cells=sum(not r['consistent']for r in records),scope='16point-triple norm cells, arbitrary h/k; no global triple/sourcechain closure')

def main():
 out={};error=None
 try:out['controls']=controls();out['actual']=actual()
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 trial=json.dumps(dict(out=out,error=error),separators=(',',':'))
 if time.monotonic()-START>=78:error='late serialized output'
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as f:f.write(raw)
 ev=dict(error=error,controls=out.get('controls'),actual={k:v for k,v in out.get('actual',{}).items()if k!='records'},worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),qualified_speedup=None,complete_global_root_search=False,release_gate_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Three-point interpolation compiles arbitrary h/k binary norms into one exact F2-linear parameter fiber.',evidence=ev,next_actions=['Investigate exact pair/triple cell batching and repeated/ghost Hermite branches; derive separate odd quadratic-parameter solver and measure all enumeration costs.'])
 text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
