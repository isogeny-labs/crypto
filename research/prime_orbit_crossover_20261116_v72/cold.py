from pathlib import Path
import os,json,time,sys,subprocess,signal,statistics,resource
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
  data=json.loads((P/'DATA.json').read_text());expected={};times={}
  for rep in range(data['repetitions']):
   for field in data['fields']:
    for bits in data['cold_bits']:
     methods=data['methods'][rep:]+data['methods'][:rep]
     for m in methods:
      assert time.monotonic()-START<65,'no remaining supervised budget'
      label=field+'_'+str(bits)+'_'+m+'_'+str(rep);r=invoke([sys.executable,'-m','research.prime_orbit_crossover_20261116_v72.child',str(P/(field+'_'+str(bits)+'.json')),m],label)
      assert r['exit_code']==0 and not r['outer_timeout'],label
      v=json.loads(r['stdout']);assert v['complete'] and v['exact_root_count']==len(v['roots']);key=(field,bits)
      if key not in expected:expected[key]=v['roots']
      assert v['roots']==expected[key] and len(v['cpu_affinity'])==1
      r['parsed']=v
      with(P/(label+'.json')).open('x')as f:json.dump(r,f,separators=(',',':'))
      times.setdefault((field,bits,m),[]).append(r['wall_seconds']);out['rows'].append({'field':field,'source_bits':bits,'method':m,'rep':rep,'wall_seconds':r['wall_seconds'],'wait4_peak_rss_kib':r['wait4_peak_rss_kib'],'post_exec_sample_peak_kib':r['post_exec_sample_peak_kib'],'child_peak_rss_kib':v['child_peak_rss_kib'],'root_count':len(v['roots']),'raw':label+'.json'})
  for field in data['fields']:
   for bits in data['cold_bits']:
    median={m:statistics.median(times[field,bits,m])for m in data['methods']};spread={m:(max(times[field,bits,m])-min(times[field,bits,m]))/median[m]for m in data['methods']}
    out['summaries'].append({'field':field,'source_bits':bits,'medians':median,'span_fraction':spread,'scan_over_factor':median['difference']/median['factor'],'portfolio_over_fastest':median['portfolio']/min(median['factor'],median['difference']),'roots':expected[field,bits]})
  out['complete']=True;out['scope']='Exposed fixed-offset conditional orbit; genuine GB, hard-instance and qualified host-isolation gates absent'
 except Exception as e:error=type(e).__name__+': '+str(e)
 finish(out,error,START,'Repeated cold five-field interval comparisons with complete roots, final serialization and memory.','Freeze new independent-source pruning test; do not promote specialized scan ratios as genuine GB speedups.')
if __name__=='__main__':main()
