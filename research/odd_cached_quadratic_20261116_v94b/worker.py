import time,json,hashlib,resource,os
from .common import P,finish
from .kernel import solve,Arithmetic,coefficients
from research.twist_auxiliary_20261116_v86.algebra import s3
START=time.monotonic();out={'tiny':[],'nist':[],'qualified_speedup':None};error=None
try:
 d=json.loads((P/'DATA.json').read_text())
 for spec in d['tiny']:
  f=Arithmetic(spec['p'],spec['mod']);a=solve(spec,'cached');b=solve(spec,'direct')
  expected=[(x,y,z)for x in spec['domain']for y in spec['domain']for z in range(f.q)if s3(x,y,z,spec['A'],spec['B'],f)==0];assert a['roots']==b['roots']==expected
  # Independent displayed discriminant identity at every source pair.
  for x in spec['domain']:
   for y in spec['domain']:
    aa,bb,cc=coefficients(x,y,spec['A'],spec['B'],f);fx=f.add(f.mul(f.sq(x),x),f.add(f.mul(spec['A'],x),spec['B']));fy=f.add(f.mul(f.sq(y),y),f.add(f.mul(spec['A'],y),spec['B']));assert f.sub(f.sq(bb),f.mul(4%f.p,f.mul(aa,cc)))==f.mul(16%f.p,f.mul(fx,fy))
  out['tiny'].append({'id':spec['id'],'root_count':len(expected),'cached':a['stats'],'direct':b['stats']})
 for spec in d['nist']:
  row={'id':spec['id'],'runs':[]};expected=None;f=Arithmetic(spec['p'],spec['mod'])
  for rep in range(3):
   for method in(['direct','cached']if rep%2==0 else['cached','direct']):
    start=time.monotonic();v=solve(spec,method)
    for x,y,z in v['roots']:assert s3(x,y,z,spec['A'],spec['B'],f)==0
    encoded=json.dumps(v['roots'],separators=(',',':'));digest=hashlib.sha256(encoded.encode()).hexdigest();assert expected in[None,digest];expected=digest
    row['runs'].append({'rep':rep,'method':method,'seconds':time.monotonic()-start,'root_sha256':digest,'root_count':len(v['roots']),'stats':v['stats'],'process_high_water_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss})
  # Complete explicit root set retained once; matching digests for every timed repetition.
  row['roots']=v['roots'];out['nist'].append(row)
 out['cpu_affinity']=sorted(os.sched_getaffinity(0));assert len(out['cpu_affinity'])==1
except Exception as e:error=repr(e)
finish(out,error,START,'Odd prime/extension exact pair-to-third-root backend with cached square classes and batched inversions.','Freeze cold-process odd-backend comparison next; internal repeated timings are not a GB speed ratio or hardness proof.')
