import json,sys,time,os,resource,subprocess,select,signal
from pathlib import Path
START=time.monotonic()
from .kernel import preprocess,lift
from .verifier import verify
from research.source_guards_20261116_v93.kernel import polynomial
from research.symmetric_pair_20261116_v93b.kernel import solve
from research.twist_auxiliary_20261116_v86.algebra import Field,s3
P=Path(sys.argv[1]).parent;data=json.loads(Path(sys.argv[1]).read_text());raw=data['cold_cases'][int(sys.argv[2])];c={k:raw[k]for k in['n','mod','d','u','B','fiber_cap_bits']};method=sys.argv[3];name=sys.argv[4]
v={'complete':False,'exact_root_count':None,'roots':[],'native_engine_called':False};error=None
try:
 if method=='symmetric':v.update(solve(c))
 else:
  p=polynomial(c['n'],c['mod'],c['d'],0,c['u'],0,0,0,c['B']);eq=[sorted(m for m,a in p.items()if a>>j&1)for j in range(c['n'])];r=preprocess(eq,2*c['d']);assert verify(eq,2*c['d'],r);v['certificate']=r
  if r['unsat']:roots=[]
  elif not r['polys']:
   assert r['n']<=data['output_cap_bits'];roots=[lift(r,z)for z in range(1<<r['n'])]
  else:
   q=P/(name+'_neutral.json');neutral={'schema':'neutral_boolean_polynomial_system_v1','instance_id':raw['id'],'field':{'kind':'prime','p':2},'domain':[0,1],'n_vars':r['n'],'equation_count':len(r['polys']),'equations':[[[j for j in range(r['n'])if m>>j&1]for m in row]for row in r['polys']]}
   with q.open('x')as w:json.dump(neutral,w,separators=(',',':'))
   cmd=[data['baseline_binary'],'--algorithm',method,'--input',str(q),'--max-degree','6','--node-budget','200000','--max-solutions','100000'];env=dict(os.environ);env['RAYON_NUM_THREADS']='1';t=time.monotonic_ns()
   with(P/(name+'_native.stdout')).open('xb')as stdout,(P/(name+'_native.stderr')).open('xb')as stderr:
    proc=subprocess.Popen(cmd,stdout=stdout,stderr=stderr,env=env,start_new_session=True);fd=os.pidfd_open(proc.pid)
    try:
     ready=bool(select.select([fd],[],[],data['native_timeout'])[0])
     if not ready:os.killpg(proc.pid,signal.SIGKILL)
     _,status,usage=os.wait4(proc.pid,0);proc.returncode=os.waitstatus_to_exitcode(status)
    finally:os.close(fd)
   obs={'command':cmd,'wall_seconds':(time.monotonic_ns()-t)/1e9,'timed_out':not ready,'exit_code':proc.returncode,'peak_rss_kib':usage.ru_maxrss,'user_seconds':usage.ru_utime,'system_seconds':usage.ru_stime}
   # Persist timeout/failure streams and observer BEFORE any assertions or parsing.
   with(P/(name+'_native_observer.json')).open('x')as w:json.dump(obs,w)
   v.update({'native_engine_called':True,'native_observer':obs});assert ready and proc.returncode==0
   z=json.loads((P/(name+'_native.stdout')).read_text());assert z['complete']and z['verified']and z['algorithm']==method and z['source_revision']==data['baseline_revision'];roots=[lift(r,int(x,16))for x in z['roots_hex']]
  mask=(1<<c['d'])-1;v.update({'complete':True,'roots':sorted((z&mask,z>>c['d'])for z in roots),'exact_root_count':len(roots)})
 f=Field(2,c['mod']);assert len(v['roots'])==len(set(map(tuple,v['roots'])))
 for x,y in v['roots']:assert 0<=x<1<<c['d']and 0<=y<1<<c['d']and s3(x,y,c['u'],0,c['B'],f)==0
except Exception as e:error=repr(e);v.update({'complete':False,'exact_root_count':None})
v.update({'error':error,'cpu_affinity':sorted(os.sched_getaffinity(0)),'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss});text=json.dumps(v,separators=(',',':'))
if time.monotonic()-START>=7:v.update({'complete':False,'exact_root_count':None,'error':'reporting deadline'})
print(json.dumps(v,separators=(',',':')))
