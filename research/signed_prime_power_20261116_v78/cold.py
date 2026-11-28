from pathlib import Path
import os,json,time,sys,subprocess,signal,statistics
from .common import P,finish
START=time.monotonic()
def invoke(command,label):
 begin=time.monotonic();ready=P/(label+'.ready');post=[];timeout=False
 with(P/(label+'.stdout')).open('xb')as sout,(P/(label+'.stderr')).open('xb')as serr:
  child=subprocess.Popen(command+[str(ready)],stdout=sout,stderr=serr,start_new_session=True)
  while True:
   pid,status,usage=os.wait4(child.pid,os.WNOHANG)
   if pid:break
   if ready.exists():
    try:
     for line in Path('/proc/'+str(child.pid)+'/status').read_text().splitlines():
      if line.startswith('VmRSS:'):post.append(int(line.split()[1]));break
    except FileNotFoundError:pass
   if time.monotonic()-begin>=8:
    timeout=True;os.killpg(child.pid,signal.SIGKILL);pid,status,usage=os.wait4(child.pid,0);break
   time.sleep(.005)
  child.returncode=os.waitstatus_to_exitcode(status)
 return {'command':command,'wall_seconds':time.monotonic()-begin,'wait4_peak_rss_kib':usage.ru_maxrss,'post_exec_sample_peak_kib':max(post,default=None),'post_exec_samples':len(post),'user_seconds':usage.ru_utime,'system_seconds':usage.ru_stime,'exit_code':child.returncode,'outer_timeout':timeout,'stdout':(P/(label+'.stdout')).read_text(),'stderr':(P/(label+'.stderr')).read_text()}
def main():
 out={'rows':[],'summaries':[],'qualified_speedup':None};error=None
 try:
  d=json.loads((P/'DATA.json').read_text());expected={};times={}
  for rep in range(d['cold_repetitions']):
   for fi,field in enumerate(d['fields']):
    methods=d['methods'];j=(rep+fi)%2;order=methods[j:]+methods[:j]
    for m in order:
     assert time.monotonic()-START<65,'no remaining child budget';label=field+'_'+m+'_'+str(rep);path=d['input_by_field'][field];r=invoke([sys.executable,'-m','research.signed_prime_power_20261116_v78.child',path,m],label)
     assert r['exit_code']==0 and not r['outer_timeout'],label;v=json.loads(r['stdout']);assert v['complete']and v['exact_root_count']==sum(map(len,v['roots']))and len(v['cpu_affinity'])==1
     check=(v['roots'],v['candidate_shifts'],v['orbit_polynomial'])
     if field not in expected:expected[field]=check
     assert check==expected[field];r['parsed']=v
     with(P/(label+'.json')).open('x')as f:json.dump(r,f,separators=(',',':'))
     times.setdefault((field,m),[]).append(r['wall_seconds']);out['rows'].append({'field':field,'method':m,'rep':rep,'wall_seconds':r['wall_seconds'],'wait4_peak_rss_kib':r['wait4_peak_rss_kib'],'post_exec_sample_peak_kib':r['post_exec_sample_peak_kib'],'child_peak_rss_kib':v['child_peak_rss_kib'],'root_count':v['exact_root_count'],'operation_counts':v['operation_counts'],'raw':label+'.json'})
  for field in d['fields']:
   med={m:statistics.median(times[field,m])for m in d['methods']};span={m:(max(times[field,m])-min(times[field,m]))/med[m]for m in d['methods']};out['summaries'].append({'field':field,'medians':med,'span_fraction':span,'generic_over_adaptive':med['generic']/med['adaptive'],'root_count':sum(map(len,expected[field][0])),'instances_per_process':d['batch_size'],'roots':expected[field][0]})
  out['complete']=True;out['scope']='Frozen exposed conditional orbit; not full independent-source or genuine GB comparison'
 except Exception as e:error=type(e).__name__+': '+str(e)
 finish(out,error,START,'Six-field cold-batch comparison of generic and guarded sparse-power exact conditional backends.','Retain all fields and timing losses; benchmark strong native baselines and full original search before any gate claim.')
if __name__=='__main__':main()
