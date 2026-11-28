import json,time,sys,statistics
from .common import P,finish
from .supervisor import invoke
START=time.monotonic();out={'pairs':[],'qualified_speedup':None};error=None
try:
 d=json.loads((P/'DATA.json').read_text());expected={}
 for n in d['batch_sizes']:
  for rep in range(d['pairs_per_size']):
   values={}
   for label in (['A','B']if rep%2==0 else['B','A']):
    name='aa_'+str(n)+'_'+str(rep)+'_'+label;r=invoke([sys.executable,'-m','research.aa_amortization_20261116_v81.child',str(P/'DATA.json'),str(n),label],name,d['child_seconds']);assert r['exit_code']==0 and not r['outer_timeout'];v=json.loads(r['stdout']);assert v['complete']and len(v['cpu_affinity'])==1 and v['exact_root_count']==2*n
    check=(v['roots'],v['candidate_shifts'],v['orbit_polynomial']);expected.setdefault(n,check);assert check==expected[n];r['parsed']=v
    with(P/(name+'.json')).open('x')as f:json.dump(r,f,separators=(',',':'))
    values[label]={k:r[k]for k in ['wall_seconds','user_seconds','system_seconds','wait4_peak_rss_kib','post_exec_sample_peak_kib']};values[label]['child_peak_rss_kib']=v['child_peak_rss_kib']
   ts=[values[x]['wall_seconds']for x in ['A','B']];spread=abs(ts[0]-ts[1])/statistics.median(ts);out['pairs'].append({'batch_size':n,'rep':rep,'A':values['A'],'B':values['B'],'spread':spread,'within10percent':spread<=d['aa_threshold']})
 out['failed_pairs']=sum(not v['within10percent']for v in out['pairs']);out['all_pairs_within10percent']=out['failed_pairs']==0;out['complete']=True;out['qualified_isolation']=False
except Exception as e:error=repr(e)
finish(out,error,START,'Fixed identical-reference startup ladder; every pair and memory measurement retained.','Test semilinear Frobenius composition against sequential binary quotient squaring on repeated-root rings; batching is not a hardness certificate.')
