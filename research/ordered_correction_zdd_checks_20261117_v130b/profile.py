from pathlib import Path
import json,time,random,hashlib
from .common import P,finish
from .kernel import compile_cones,fixed_orders
from research.correction_cone_zdd_20261117_v129c.kernel import ZDDLimit
from research.correction_basis_checks_20261117_v128b.kernel import compile_all
START=time.monotonic();out={'variants':{},'orders':{},'actual_value_checks':0,'original_complete':False};error=None
try:
 d=json.loads((P/'DATA.json').read_bytes());rng=random.Random(d['seed']);cfg=json.loads(Path(d['config']).read_bytes());chart=json.loads(Path(d['cover']).read_bytes())['records'][1]['chart'];G,oo,rr,meta=compile_all(cfg,chart,8);b=json.dumps({'nodes':G.nodes,'outputs':oo,'roots':rr},separators=(',',':')).encode();receipt=json.loads(Path(d['graph_receipt']).read_bytes());assert receipt['error']is None and hashlib.sha256(b).hexdigest()==receipt['out']['variants']['8']['graph_sha256'];out['source_graph_sha256']=hashlib.sha256(b).hexdigest();wires=oo[-1]['wires'][0];orders=fixed_orders(G,wires);out['orders']=orders;prior=json.loads(Path(d['natural_receipt']).read_bytes());assert prior['error']is None;samples=[rng.randrange(1<<G.n)for _ in range(128)];controls=[G.evaluate(x)for x in samples]
 for label in d['orders']:
  for count in d['counts']:
   t=time.monotonic();key=label+'_'+str(count)
   try:
    M,roots=compile_cones(G,wires[:count],orders[label],**d['caps']);degrees,terms=M.degrees_terms(roots)
    for x,v in zip(samples,controls):assert M.evaluate(roots,x)==[v[w]for w in wires[:count]];out['actual_value_checks']+=count
    artifact={'source_graph_sha256':out['source_graph_sha256'],'order':orders[label],'nodes':M.nodes,'roots':roots,'wires':wires[:count]};b=json.dumps(artifact,separators=(',',':')).encode()
    with(P/('diagram_'+key+'.json')).open('xb')as f:f.write(b)
    out['variants'][key]={'status':'complete','seconds':time.monotonic()-t,'degrees':degrees,'term_counts':terms,'stats':M.stats(),'artifact_sha256':hashlib.sha256(b).hexdigest()}
   except ZDDLimit as e:
    out['variants'][key]={'status':'unknown_cap','reason':str(e),'seconds':time.monotonic()-t,'stats':e.stats}
    if label=='natural':assert str(e)==prior['out']['variants'][str(count)]['reason']and e.stats==prior['out']['variants'][str(count)]['stats']
 out['total_seconds']=time.monotonic()-START
except Exception as e:error=repr(e)
finish(out,error,START,'Fixed natural/reverse/frequency order profiles on identical correction wires with unchanged caps and source coordinates.','If allcap,audit frontier liveness and reclaim unreachable intermediates between complete source gates,not inside recursive arithmetic.')
