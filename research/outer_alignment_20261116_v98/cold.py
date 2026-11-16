from .common import P,finish
import json,subprocess,sys,time,os,signal,select,statistics,hashlib
START=time.monotonic();out={'runs':[],'qualified_speedup':None,'qualified_isolation':False};error=None
try:
 d=json.loads((P/'DATA.json').read_text());expected={}
 for rep in range(3):
  for i,c in enumerate(d['cold']):
   for method in(['join','block','filtered']if rep%2==0 else['filtered','block','join']):
    assert time.monotonic()-START<65;name='cold_'+str(i)+'_'+str(rep)+'_'+method;cmd=[sys.executable,'-m','research.outer_alignment_20261116_v98.child',str(P/'DATA.json'),str(i),method];t=time.monotonic_ns()
    with(P/(name+'.stdout')).open('xb')as stdout,(P/(name+'.stderr')).open('xb')as stderr:
     proc=subprocess.Popen(cmd,stdout=stdout,stderr=stderr,start_new_session=True);fd=os.pidfd_open(proc.pid)
     try:
      ready=bool(select.select([fd],[],[],9)[0])
      if not ready:os.killpg(proc.pid,signal.SIGKILL)
      _,status,usage=os.wait4(proc.pid,0);proc.returncode=os.waitstatus_to_exitcode(status)
     finally:os.close(fd)
    r={'id':c['id'],'method':method,'rep':rep,'command':cmd,'wall_seconds':(time.monotonic_ns()-t)/1e9,'peak_rss_kib':usage.ru_maxrss,'user_seconds':usage.ru_utime,'system_seconds':usage.ru_stime,'timed_out':not ready,'exit_code':proc.returncode}
    with(P/(name+'_observer.json')).open('x')as w:json.dump(r,w)
    out['runs'].append(r)
    if not ready or proc.returncode!=0:continue
    v=json.loads((P/(name+'.stdout')).read_text());r['parsed']=v
    if not v['complete']:continue
    assert len(v['cpu_affinity'])==1;expected.setdefault(c['id'],v['roots']);assert expected[c['id']]==v['roots'];r['root_sha256']=hashlib.sha256(json.dumps(v['roots']).encode()).hexdigest()
 out['complete']=len(out['runs'])==len(d['cold'])*9 and all(r.get('parsed',{}).get('complete',False)for r in out['runs']);out['medians']={c['id']:{m:statistics.median(r['wall_seconds']for r in out['runs']if r['id']==c['id']and r['method']==m)for m in['join','block','filtered']}for c in d['cold']}
 if not out['complete']:error='Incompletearm;noacceptedratio'
except Exception as e:error=repr(e)
finish(out,error,START,'Cold complete original-chain8-coordinate outputs,join versus rebracket versus verifiedcellfilter.','Retainlosingintegration;useconditionaloracleonlywhereendpointsalreadyfixed andinvestigate actualnormresidue/search bottleneck.')
