import json,time,sys,statistics,hashlib
from .common import P,finish
from .supervisor import invoke
START=time.monotonic()
def main():
 out={'pairs':[],'all_roots_equal':True,'qualified_speedup':None};error=None
 try:
  d=json.loads((P/'DATA.json').read_text());expected={}
  for case in d['aa_cases']:
   for rep in range(case['pairs']):
    values={}
    for label in (['A','B']if rep%2==0 else['B','A']):
     assert time.monotonic()-START<65;name='aa_'+case['name']+'_'+str(rep)+'_'+label;r=invoke([sys.executable,'-m','research.pidfd_observer_20261116_v79.child',case['input'],label],name,d['child_seconds'],'pidfd');assert r['exit_code']==0 and not r['outer_timeout'];v=json.loads(r['stdout']);assert v['complete']and v['exact_root_count']==sum(map(len,v['roots']))and len(v['cpu_affinity'])==1
     check=(v['roots'],v['candidate_shifts'],v['orbit_polynomial']);key=case['name']
     if key not in expected:expected[key]=check
     assert check==expected[key];r['parsed']=v
     with(P/(name+'.json')).open('x')as f:json.dump(r,f,separators=(',',':'))
     values[label]={'wall_seconds':r['wall_seconds'],'supervisor_seconds':r['supervisor_seconds'],'wait4_peak_rss_kib':r['wait4_peak_rss_kib'],'post_exec_sample_peak_kib':r['post_exec_sample_peak_kib'],'child_peak_rss_kib':v['child_peak_rss_kib'],'root_count':v['exact_root_count']}
    ts=[values[k]['wall_seconds']for k in ['A','B']];spread=(max(ts)-min(ts))/statistics.median(ts);out['pairs'].append({'field':case['name'],'rep':rep,'A':values['A'],'B':values['B'],'spread_fraction':spread,'within10percent':spread<=d['aa_threshold']})
  out['all_pairs_within10percent']=all(x['within10percent']for x in out['pairs']);out['failed_pairs']=sum(not x['within10percent']for x in out['pairs']);out['qualified_isolation']=False;out['scope']='identical code and frozen inputs; startup/output charged; no optimized algorithm comparison'
 except Exception as e:error=type(e).__name__+': '+str(e)
 finish(out,error,START,'Event-observed identical-reference A/A with complete roots and retained startup noise.','If A/A remains noisy, do not promote comparison claims; continue independent exact-kernel work and change isolation assumptions before another calibration.')
if __name__=='__main__':main()
