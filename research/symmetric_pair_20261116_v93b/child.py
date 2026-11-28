import json,sys,time,subprocess,resource,os
from pathlib import Path
START=time.monotonic()
from .kernel import solve
from research.source_guards_20261116_v93.kernel import polynomial
from research.twist_auxiliary_20261116_v86.algebra import Field,s3
p=Path(sys.argv[1]);data=json.loads(p.read_text());raw=data['cases'][int(sys.argv[2])];c={k:raw[k]for k in['n','mod','d','u','B','fiber_cap_bits']};method=sys.argv[3];name=sys.argv[4];native=None
if method=='symmetric':v=solve(c)
else:
 poly=polynomial(c['n'],c['mod'],c['d'],0,c['u'],0,0,0,c['B']);nvars=2*c['d'];eq=[[[i for i in range(nvars)if mask>>i&1]for mask,co in sorted(poly.items())if co>>j&1]for j in range(c['n'])];neutral={'schema':'neutral_boolean_polynomial_system_v1','instance_id':raw['id'],'field':{'kind':'prime','p':2},'domain':[0,1],'n_vars':nvars,'equation_count':len(eq),'equations':eq};q=p.parent/(name+'_neutral.json')
 with q.open('x')as w:json.dump(neutral,w,separators=(',',':'))
 cmd=[data['baseline_binary'],'--algorithm',method,'--input',str(q),'--max-degree','6','--node-budget','200000','--max-solutions','100000'];env=dict(os.environ);env['RAYON_NUM_THREADS']='1';r=subprocess.run(cmd,capture_output=True,text=True,timeout=data['native_timeout'],env=env);native={'command':cmd,'returncode':r.returncode,'stdout':r.stdout,'stderr':r.stderr};z=json.loads(r.stdout);assert r.returncode==0 and z['complete']and z['verified']and z['algorithm']==method and z['source_revision']==data['baseline_revision'];mask=(1<<c['d'])-1;roots=sorted((int(t,16)&mask,int(t,16)>>c['d'])for t in z['roots_hex']);v={'complete':True,'roots':roots,'exact_root_count':len(roots),'stats':z['stats']}
f=Field(2,c['mod']);assert len(v['roots'])==len(set(map(tuple,v['roots'])))
for x,y in v['roots']:assert x<1<<c['d']and y<1<<c['d']and s3(x,y,c['u'],0,c['B'],f)==0
v.update({'native':native,'cpu_affinity':sorted(os.sched_getaffinity(0)),'child_peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss});trial=json.dumps(v,separators=(',',':'))
if time.monotonic()-START>=7:v['complete']=False;v['exact_root_count']=None
print(json.dumps(v,separators=(',',':')))
