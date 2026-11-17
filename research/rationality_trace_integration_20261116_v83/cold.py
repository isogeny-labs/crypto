import json,time,sys,statistics
from .common import P,finish
from .supervisor import invoke
START=time.monotonic();out={'runs':[],'all_equal':True,'qualified_speedup':None,'scope':'full support+label bundles; standalone support has a narrower output contract, never a matched speed ratio'};error=None
try:
 d=json.loads((P/'DATA.json').read_text());expected={}
 for rep in range(d['cold_repetitions']):
  methods=d['methods'][rep:]+d['methods'][:rep]
  for method in methods:
   assert time.monotonic()-START<65;name='cold_'+str(rep)+'_'+method;r=invoke([sys.executable,'-m','research.rationality_trace_integration_20261116_v83.child',str(P/'DATA.json'),method],name,d['child_seconds']);assert r['exit_code']==0 and not r['outer_timeout'];v=json.loads(r['stdout']);assert v['complete']and len(v['cpu_affinity'])==1 and len(v['rows'])==36
   key='support'if method=='support'else'labels';expected.setdefault(key,v['rows']);assert expected[key]==v['rows'];r['parsed']=v
   with(P/(name+'.json')).open('x')as f:json.dump(r,f,separators=(',',':'))
   out['runs'].append({'rep':rep,'method':method,'wall_seconds':r['wall_seconds'],'bundle_seconds':v['bundle_seconds'],'user_seconds':r['user_seconds'],'system_seconds':r['system_seconds'],'wait4_peak_rss_kib':r['wait4_peak_rss_kib'],'post_exec_sample_peak_kib':r['post_exec_sample_peak_kib'],'child_peak_rss_kib':v['child_peak_rss_kib']})
 assert[{k:v for k,v in row.items()if k in ['name','support','scope']}for row in expected['labels']]==expected['support']
 out['medians']={method:statistics.median(r['wall_seconds']for r in out['runs']if r['method']==method)for method in d['methods']};out['spans']={method:(max(ts)-min(ts))/statistics.median(ts)for method in d['methods']for ts in [[r['wall_seconds']for r in out['runs']if r['method']==method]]};out['new_main_rejections']=sum(r['support']and r['rationality_reject']for r in expected['labels']if r['name'].startswith(('relaxed_','fresh_')));out['complete']=True;out['qualified_isolation']=False
except Exception as e:error=repr(e)
finish(out,error,START,'Three cold repetitions of full reference/shared bundles with every support and label output verified, plus standalone support costs.','If extra pruning stays zero, do not promote the label filter; test a new odd-characteristic norm or whole-cell mechanism rather than another noisy calibration.')
