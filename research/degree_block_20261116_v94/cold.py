import json,sys,time,os,subprocess,select,signal,statistics,hashlib
from .common import P,finish
START=time.monotonic();out={'runs':[],'qualified_speedup':None,'qualified_isolation':False};error=None
try:
 data=json.loads((P/'DATA.json').read_text());expected={}
 for rep in range(3):
  for i,c in enumerate(data['cold_cases']):
   for method in(data['methods']if rep%2==0 else data['methods'][::-1]):
    assert time.monotonic()-START<65;name='cold_'+str(i)+'_'+str(rep)+'_'+method;cmd=[sys.executable,'-m','research.degree_block_20261116_v94.child',str(P/'DATA.json'),str(i),method,name];t=time.monotonic_ns()
    with(P/(name+'.stdout')).open('xb')as stdout,(P/(name+'.stderr')).open('xb')as stderr:
     proc=subprocess.Popen(cmd,stdout=stdout,stderr=stderr,start_new_session=True);fd=os.pidfd_open(proc.pid)
     try:
      ready=bool(select.select([fd],[],[],data['seconds_per_child'])[0])
      if not ready:os.killpg(proc.pid,signal.SIGKILL)
      _,status,usage=os.wait4(proc.pid,0);proc.returncode=os.waitstatus_to_exitcode(status)
     finally:os.close(fd)
    obs={'id':c['id'],'method':method,'rep':rep,'command':cmd,'wall_seconds':(time.monotonic_ns()-t)/1e9,'peak_rss_kib':usage.ru_maxrss,'user_seconds':usage.ru_utime,'system_seconds':usage.ru_stime,'timed_out':not ready,'exit_code':proc.returncode}
    # Record before validation so every failed child remains inspectable.
    with(P/(name+'_observer.json')).open('x')as w:json.dump(obs,w)
    if not ready or proc.returncode!=0:obs['failure']='process failure';out['runs'].append(obs);continue
    v=json.loads((P/(name+'.stdout')).read_text());obs['parsed']=v;out['runs'].append(obs)
    if not v['complete']:continue
    assert len(v['cpu_affinity'])==1;expected.setdefault(c['id'],v['roots']);assert v['roots']==expected[c['id']]
 out['complete']=all(r.get('parsed',{}).get('complete',False)for r in out['runs'])
 out['medians']={c['id']:{m:statistics.median(r['wall_seconds']for r in out['runs']if r['id']==c['id']and r['method']==m)for m in data['methods']}for c in data['cold_cases']}
 if not out['complete']:error='One or more arms incomplete; no speed ratio accepted'
except Exception as e:error=repr(e)
finish(out,error,START,'Changed degree-block preprocessing plus unchanged native F4/F5 when a residual remains; original complete roots.','Use certified degree-block rank forecasts in the full chain; these conditional cases may fail the hardness floor.')
