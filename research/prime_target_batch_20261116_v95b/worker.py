import json,sys,time,subprocess,os,signal,select,statistics,hashlib
from .common import P,finish
from .kernel import solve
from research.twist_auxiliary_20261116_v86.algebra import Field,s3
START=time.monotonic();out={'tiny':[],'runs':[],'qualified_speedup':None,'qualified_isolation':False};error=None
try:
 d=json.loads((P/'DATA.json').read_text())
 for c in d['tiny']:
  f=Field(c['p'],[0,1]);want=[[(x,y)for x in range(c['M']+1)for y in range(c['M']+1)if s3(x,y,u,c['A'],c['B'],f)==0]for u in c['targets']];assert solve(c,'direct')['roots']==solve(c,'cached')['roots']==want;out['tiny'].append({'p':c['p'],'M':c['M'],'targets':len(c['targets']),'root_count':sum(map(len,want))})
 expected={}
 for rep in range(3):
  for i,c in enumerate(d['cases']):
   for method in(['direct','cached']if rep%2==0 else['cached','direct']):
    assert time.monotonic()-START<65;name='cold_'+str(i)+'_'+str(rep)+'_'+method;cmd=[sys.executable,'-m','research.prime_target_batch_20261116_v95b.child',str(P/'DATA.json'),str(i),method];t=time.monotonic_ns()
    with(P/(name+'.stdout')).open('xb')as stdout,(P/(name+'.stderr')).open('xb')as stderr:
     proc=subprocess.Popen(cmd,stdout=stdout,stderr=stderr,start_new_session=True);fd=os.pidfd_open(proc.pid)
     try:
      ready=bool(select.select([fd],[],[],d['child_seconds'])[0])
      if not ready:os.killpg(proc.pid,signal.SIGKILL)
      _,status,usage=os.wait4(proc.pid,0);proc.returncode=os.waitstatus_to_exitcode(status)
     finally:os.close(fd)
    r={'id':c['id'],'rep':rep,'method':method,'command':cmd,'wall_seconds':(time.monotonic_ns()-t)/1e9,'peak_rss_kib':usage.ru_maxrss,'user_seconds':usage.ru_utime,'system_seconds':usage.ru_stime,'timed_out':not ready,'exit_code':proc.returncode}
    with(P/(name+'_observer.json')).open('x')as w:json.dump(r,w)
    out['runs'].append(r)
    if not ready or proc.returncode!=0:continue
    v=json.loads((P/(name+'.stdout')).read_text());r['parsed']=v
    if not v['complete']:continue
    assert len(v['cpu_affinity'])==1;expected.setdefault(c['id'],v['roots']);assert expected[c['id']]==v['roots'];r['root_sha256']=hashlib.sha256(json.dumps(v['roots']).encode()).hexdigest()
 out['complete']=len(out['runs'])==len(d['cases'])*6 and all(r.get('parsed',{}).get('complete',False)for r in out['runs']);out['medians']={c['id']:{m:statistics.median(r['wall_seconds']for r in out['runs']if r['id']==c['id']and r['method']==m)for m in['direct','cached']}for c in d['cases']}
 if not out['complete']:error='One or more registered arms incomplete'
except Exception as e:error=repr(e)
finish(out,error,START,'Cold shared-target interval recovery with full cache construction, strong direct baseline and exact all-root output.','Test crossover across target counts and whole-chain end-to-end integration; internal specialized gains never qualify GB release.')
