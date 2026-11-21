from pathlib import Path
import json,time,random,hashlib
from .common import P,finish
from .kernel import expand,evaluate,ExpansionLimit
from research.correction_basis_checks_20261117_v128b.kernel import compile_all
START=time.monotonic();out={'variants':{},'actual_value_checks':0,'original_complete':False};error=None
try:
 d=json.loads((P/'DATA.json').read_bytes());rng=random.Random(d['seed']);cfg=json.loads(Path(d['config']).read_bytes());chart=json.loads(Path(d['cover']).read_bytes())['records'][1]['chart'];G,oo,rr,meta=compile_all(cfg,chart,8);raw=json.dumps({'nodes':G.nodes,'outputs':oo,'roots':rr},separators=(',',':')).encode();prior=json.loads(Path(d['graph_receipt']).read_bytes());assert prior['error']is None;assert hashlib.sha256(raw).hexdigest()==prior['out']['variants']['8']['graph_sha256'];out['source_graph_sha256']=hashlib.sha256(raw).hexdigest();out['source_variables']=len(G.nodes)-2;out['input_bits']=G.n;wires=oo[-1]['wires'][0];samples=[rng.randrange(1<<G.n)for _ in range(128)];controls=[G.evaluate(x)for x in samples]
 for count in d['counts']:
  t=time.monotonic()
  try:
   polys,stats=expand(G,wires[:count],**d['caps'])
   for x,values in zip(samples,controls):assert [evaluate(poly,x)for poly in polys]==[values[w]for w in wires[:count]];out['actual_value_checks']+=count
   artifact={'source_graph_sha256':out['source_graph_sha256'],'input_bits':G.n,'wires':wires[:count],'polynomials_hex':[[hex(m)for m in sorted(poly)]for poly in polys],'stats':stats};b=json.dumps(artifact,separators=(',',':')).encode()
   with(P/('anf_'+str(count)+'.json')).open('xb')as f:f.write(b)
   out['variants'][str(count)]={'status':'complete','seconds':time.monotonic()-t,'degrees':[max((m.bit_count()for m in poly),default=0)for poly in polys],'term_counts':[len(poly)for poly in polys],'stats':stats,'artifact_sha256':hashlib.sha256(b).hexdigest()}
  except ExpansionLimit as e:out['variants'][str(count)]={'status':'unknown_cap','reason':str(e),'seconds':time.monotonic()-t,'stats':e.stats}
 out['total_seconds']=time.monotonic()-START
except Exception as e:error=repr(e)
finish(out,error,START,'Fixed1/2/4/8exact correction-cone ANF diagnostics with strictcaps and independent actual value controls.','If capped,localize explosion and change representation or condition a declared cell;neverinferfinaldegreefromatransientcap or retryidenticalwork.')
