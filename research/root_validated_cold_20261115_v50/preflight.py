from pathlib import Path
import hashlib,json,resource,sys,time
from research.root_validated_cold_20261115_v50.child import solve_chart
P=Path(__file__).resolve().parent;START=time.monotonic()

def main():
 out=dict(charts=[]);error=None
 try:
  rule=json.loads((P/'DATA.json').read_text())
  for index in rule['h_indices']:
   results={}
   for method in ['reference','compressed']:
    result=solve_chart(method,index);assert result['norm_family_complete'];results[method]=result
    with(P/('preflight_'+method+'_h'+str(index)+'.json')).open('x')as f:json.dump(result,f,sort_keys=True,separators=(',',':'))
   key=lambda r:{(x,int(t,16))for x,t in r['native']['candidates']};assert key(results['reference'])==key(results['compressed']);assert results['reference']['supported']==results['compressed']['supported']
   if index==0:assert len(key(results['reference']))==249 and not results['reference']['supported']
   out['charts'].append(dict(h_index=index,h=results['reference']['h'],candidates=len(key(results['reference'])),norms=results['reference']['supported'],candidate_sha256=hashlib.sha256(json.dumps(sorted(key(results['reference'])),separators=(',',':')).encode()).hexdigest()))
   assert time.monotonic()-START<72
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 trial=json.dumps(dict(out=out,error=error),separators=(',',':'))
 if time.monotonic()-START>=78:error='late serialized preflight'
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode()
 with(P/'preflight.json').open('xb')as f:f.write(raw)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Integrated portable native validation and support retain complete matched candidate/norm sets on4h charts.',evidence=dict(error=error,charts=out['charts'],worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),qualified_speedup=None,complete_global_root_search=False,release_gate_claim=False),next_actions=['Run the separately frozen three-cold-repetition four-chart comparison only after this preflight passes.'])
 text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
