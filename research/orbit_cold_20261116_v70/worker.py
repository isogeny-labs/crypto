from pathlib import Path
import json,hashlib,os,resource,statistics,sys,time
from research.quadratic_baseline_pilot_20261115_v63.worker import invoke
P=Path(__file__).resolve().parent;START=time.monotonic()
def main():
 import research.quadratic_baseline_pilot_20261115_v63.worker as proc
 proc.P=P;out={'runs':[],'qualified_speedup':None};error=None
 try:
  data=json.loads((P/'DATA.json').read_text());
  from .field_first import solve as fullfield
  from research.orbit_roots_20261116_v70.solver import solve as domain
  for path in data['calibration_paths']:
   inst=json.loads(Path(path).read_text());assert fullfield(inst)['roots']==domain(inst,'gcd')['roots']
  times={m:[]for m in ['gcd','field_first','gray']};expected=None;env=os.environ.copy();env.update(RAYON_NUM_THREADS='1',OMP_NUM_THREADS='1',OPENBLAS_NUM_THREADS='1')
  for rep,order in enumerate(data['order']):
   for method in order:
    assert time.monotonic()-START<63,'no remaining childbudget';name=method+'_'+str(rep);r=invoke([sys.executable,'-m','research.orbit_cold_20261116_v70.child',data['input'],method],env,name);v=json.loads(r['stdout']);assert r['exit_code']==0 and not r['outer_timeout']and v['complete'];roots=v['roots']
    if expected is None:expected=roots
    assert roots==expected and any(t==0 for t,u in roots);r['parsed']=v;payload=json.dumps(r,sort_keys=True,separators=(',',':')).encode()
    with(P/(name+'.json')).open('xb')as f:f.write(payload)
    times[method].append(r['wall_seconds']);out['runs'].append({'method':method,'rep':rep,'wall_seconds':r['wall_seconds'],'peak_rss_kib':r['wait4_peak_rss_kib'],'root_count':v['exact_root_count'],'raw_sha256':hashlib.sha256(payload).hexdigest()})
  out['medians']={m:statistics.median(ts)for m,ts in times.items()};out['repeat_span_fraction']={m:(max(ts)-min(ts))/statistics.median(ts)for m,ts in times.items()};out['ratios_vs_gcd']={m:out['medians'][m]/out['medians']['gcd']for m in ['field_first','gray']};out['roots']=expected;out['scope']='One exposedGF83conditional orbit;not independent-source solver,genuineGBor20hard qualification'
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps({'out':out,'error':error},separators=(',',':')).encode();assert time.monotonic()-START<78
 with(P/'raw.json').open('xb')as f:f.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if error is None else'inconclusive','summary':'Four cold exact conditional orbit runs per domainGCD,field-firstGCDandstrongANF/zetascan,includingstartup,setup,allauxiliariesandserialization.','evidence':{'out':out,'error':error,'worker_seconds':time.monotonic()-START,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False},'next_actions':['Report allcomparators;do not promotean enumerationratio over a fasterunivariatebaseline as50xgeneralrootfinding.']};text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
