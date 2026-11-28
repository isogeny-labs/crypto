from pathlib import Path
import subprocess,select,os,json,time,statistics,sys,hashlib
from .common import P,finish
START=time.monotonic()
def observe(cmd,name,seconds):
 start=time.monotonic_ns();done=False;child=None;fd=None
 try:
  with(P/(name+'.stdout')).open('xb')as stdout,(P/(name+'.stderr')).open('xb')as stderr:
   child=subprocess.Popen(cmd,stdout=stdout,stderr=stderr);fd=os.pidfd_open(child.pid)
   ready=select.select([fd],[],[],seconds)[0]
   if not ready:child.kill()
   pid,status,usage=os.wait4(child.pid,0);done=True;end=time.monotonic_ns();child.returncode=os.waitstatus_to_exitcode(status)
 finally:
  if fd is not None:os.close(fd)
  if child is not None and not done:child.kill();os.wait4(child.pid,0)
 return{'wall_seconds':(end-start)/1e9,'exit_code':child.returncode,'timed_out':not ready or end-start>seconds*1e9,'peak_rss_kib':usage.ru_maxrss,'user_seconds':usage.ru_utime,'system_seconds':usage.ru_stime,'stdout':(P/(name+'.stdout')).read_text(),'stderr':(P/(name+'.stderr')).read_text()}
out={'runs':[],'qualified_speedup':None};error=None
try:
 d=json.load(open(P/'DATA.json'));expect={}
 for rep in range(d['cold_repetitions']):
  for i,case in enumerate([x for x in d['cases']if x['id']in d['cold_ids']]):
   for method in(d['methods']if rep%2==0 else d['methods'][::-1]):
    assert time.monotonic()-START<65;name='cold_'+str(i)+'_'+str(rep)+'_'+method;r=observe([sys.executable,'-m','research.symmetric_pair_20261116_v93b.child',str(P/'DATA.json'),str(d['cases'].index(case)),method,name],name,d['seconds_per_child']);assert not r['timed_out']and r['exit_code']==0;v=json.loads(r['stdout']);assert v['complete']and len(v['cpu_affinity'])==1;key=case['id'];expect.setdefault(key,v['roots']);assert expect[key]==v['roots'];r['parsed']=v
    with(P/(name+'.json')).open('x')as f:json.dump(r,f,separators=(',',':'))
    out['runs'].append({'id':key,'rep':rep,'method':method,**{k:r[k]for k in['wall_seconds','peak_rss_kib','user_seconds','system_seconds']},'child_peak_rss_kib':v['child_peak_rss_kib'],'stats':v['stats'],'exact_root_count':v['exact_root_count'],'root_sha256':hashlib.sha256(json.dumps(v['roots']).encode()).hexdigest()})
 out['medians']={case['id']:{method:statistics.median(r['wall_seconds']for r in out['runs']if r['id']==case['id']and r['method']==method)for method in d['methods']}for case in [x for x in d['cases']if x['id']in d['cold_ids']]};out['complete']=True;out['qualified_isolation']=False
except Exception as e:error=repr(e)
finish(out,error,START,'Matched exact original-source-pair roots: symmetric support versus genuine BooleanF4/F5,conditionaldiagnostic only.','Retain all sizes and setup/fiber costs;derive finite-domain matching F4/F5/F6 inputs before any release comparison. An internal gain is not the qualified gate.')
