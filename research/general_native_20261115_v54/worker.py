from pathlib import Path
import hashlib,json,resource,subprocess,sys,time
from research.general_native_20261115_v54.solve import solve
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()

def main():
 out=dict(cases=[],builds=[]);error=None
 try:
  for source,name in [('reference.cpp','reference'),('compressed.cpp','compressed')]:
   t=time.monotonic();run=subprocess.run(['g++','-O3','-std=c++17',str(P/source),'-o',str(P/name)],capture_output=True,text=True,timeout=15);out['builds'].append(dict(source=source,seconds=time.monotonic()-t,stdout=run.stdout,stderr=run.stderr,exit_code=run.returncode));assert run.returncode==0,run.stderr
  t=time.monotonic();run=subprocess.run(['g++','-O3','-std=c++17','-shared','-fPIC',str(R/'research/native_validation_20261115_v49/validation.cpp'),'-o',str(P/'validation.so')],capture_output=True,text=True,timeout=12);out['builds'].append(dict(source='validation.cpp',seconds=time.monotonic()-t,stdout=run.stdout,stderr=run.stderr,exit_code=run.returncode));assert run.returncode==0,run.stderr
  witnesses=json.loads((R/'research/positive_binary_charts_20261115_v53/raw.json').read_text());assert witnesses['error']is None
  for witness in witnesses['out']['cases']:
   name=witness['curve']+'_'+witness['kind'];path=R/'research/positive_binary_charts_20261115_v53'/('instance_'+name+'.json');instance=json.loads(path.read_text());results={}
   for method in ['reference','compressed']:
    result=solve(instance,P/method,P/'validation.so');assert result['complete'];results[method]=result
    with(P/(name+'_'+method+'.json')).open('x')as file:json.dump(result,file,sort_keys=True,separators=(',',':'))
   assert results['reference']['candidate_pairs']==results['compressed']['candidate_pairs'];assert results['reference']['solutions']==results['compressed']['solutions']
   known=witness['norm_coefficients'];assert any(v['r']==known[0]and v['s']==known[1]and list(v['H'])==witness['H']for v in results['compressed']['solutions'])
   out['cases'].append(dict(name=name,candidates=len(results['compressed']['candidate_pairs']),solutions=results['compressed']['solutions'],reference_seconds=results['reference']['total_seconds'],compressed_seconds=results['compressed']['total_seconds'],reference_rss=results['reference'].get('native',{}).get('peak_rss_kib'),compressed_rss=results['compressed'].get('native',{}).get('peak_rss_kib')));assert time.monotonic()-START<72
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 trial=json.dumps(dict(out=out,error=error),separators=(',',':'))
 if time.monotonic()-START>=78:error='late serialized result'
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as file:file.write(raw)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Complete native general-h/k positive conditional norm charts with zero/ghost poles, matched to generic full matrix scan.',evidence=dict(error=error,cases=[{k:v for k,v in row.items()if k!='solutions'}|dict(solution_count=len(row['solutions']))for row in out['cases']],worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),qualified_speedup=None,complete_global_root_search=False,release_gate_claim=False),next_actions=['If matched, freeze fresh cold timings and add complete original S3 auxiliary reconstruction; preserve positive multiplicities and pole cases.'])
 text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
