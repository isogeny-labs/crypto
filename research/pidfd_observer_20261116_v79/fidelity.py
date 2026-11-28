from pathlib import Path
import json,time,subprocess,statistics
from .common import P,finish
from .supervisor import invoke
START=time.monotonic()
def main():
 out={'runs':[],'controls':[],'qualified_speedup':None};error=None
 try:
  d=json.loads((P/'DATA.json').read_text());b=subprocess.run(['g++','-O2','-std=c++17',str(P/'probe.cpp'),'-o',str(P/'probe')],capture_output=True,text=True,timeout=12);out['build']={'returncode':b.returncode,'stdout':b.stdout,'stderr':b.stderr};assert b.returncode==0,b.stderr;tails={m:[]for m in ['legacy','pidfd']}
  for rep in range(d['fidelity_reps']):
   for delay in d['fidelity_delays_ms']:
    for mode in (['legacy','pidfd']if rep%2==0 else['pidfd','legacy']):
     label='fidelity_'+mode+'_'+str(delay)+'_'+str(rep);r=invoke([str(P/'probe'),str(delay),'0','normal'],label,1,mode);assert r['exit_code']==0 and not r['outer_timeout'];v=json.loads(r['stdout']);tail=(r['parent_end_ns']-v['exitstamp_ns'])/1e9;assert tail>=0;r['exit_tail_seconds']=tail;tails[mode].append(tail)
     with(P/(label+'.json')).open('x')as f:json.dump(r,f,separators=(',',':'))
     out['runs'].append({'mode':mode,'delay_ms':delay,'rep':rep,'wall_seconds':r['wall_seconds'],'exit_tail_seconds':tail,'wait4_peak_rss_kib':r['wait4_peak_rss_kib']})
  for mode in ['legacy','pidfd']:
   for name,delay,rc,kind,deadline in [('nonzero',0,7,'normal',1),('malformed',0,0,'bad',1),('timeout',300,0,'partial',.05),('memory',50,0,'alloc',1)]:
    label='control_'+mode+'_'+name;r=invoke([str(P/'probe'),str(delay),str(rc),kind],label,deadline,mode)
    if name=='timeout':assert r['outer_timeout']and r['exit_code']!=0 and r['stdout'].startswith('{partial')
    elif name=='nonzero':assert r['exit_code']==7 and not r['outer_timeout']
    elif name=='malformed':
     try:json.loads(r['stdout']);raise AssertionError('malformed output accepted')
     except json.JSONDecodeError:pass
    else:assert r['exit_code']==0 and json.loads(r['stdout'])['child_peak_rss_kib']>=8192
    with(P/(label+'.json')).open('x')as f:json.dump(r,f,separators=(',',':'))
    out['controls'].append({'mode':mode,'name':name,'exit_code':r['exit_code'],'outer_timeout':r['outer_timeout'],'wait4_peak_rss_kib':r['wait4_peak_rss_kib'],'post_exec_sample_peak_kib':r['post_exec_sample_peak_kib']})
  out['tail_medians']={m:statistics.median(v)for m,v in tails.items()};out['pidfd_tail_below_1ms']=out['tail_medians']['pidfd']<.001
 except Exception as e:error=type(e).__name__+': '+str(e)
 finish(out,error,START,'Event-driven completion fidelity versus fixed polling, including failures/deadlines and memory.','Run identical-reference A/A only after fidelity controls pass; no root-solver speed ratio follows.')
if __name__=='__main__':main()
