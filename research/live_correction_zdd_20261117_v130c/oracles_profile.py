from pathlib import Path
import json,time,random,hashlib
from .common import P,finish
from .kernel import LiveManager,compile_live
from research.correction_cone_zdd_20261117_v129c.kernel import ZDDLimit
from research.correction_cone_anf_checks_20261117_v129b.kernel import expand
from research.correction_basis_checks_20261117_v128b.kernel import compile_all
from research.local_split_circuit_20261116_v100.kernel import Circuit
START=time.monotonic();out={'circuits':0,'assignments':0,'value_checks':0,'polynomial_checks':0,'oracle_collections':0,'retry_control':0,'variants':{},'actual_value_checks':0,'original_complete':False};error=None
try:
 d=json.loads((P/'DATA.json').read_bytes());rng=random.Random(d['seed'])
 for case in range(32):
  n=3+case%5;C=Circuit(n)
  for _ in range(48):
   kind=rng.choice(['and','xor','xor','not']);a=rng.randrange(len(C.nodes));b=rng.randrange(len(C.nodes));C.op(kind,a,None if kind=='not'else b)
  outputs=[0,1]+[rng.randrange(len(C.nodes))for _ in range(8)];before=tuple(C.nodes);polys,stats=expand(C,outputs,max_degree=8,max_monomials=512,max_stored=1000000,max_work=20000000);M,roots=compile_live(C,outputs,collect_every=4,max_nodes=100000,max_cache=500000,max_work=20000000,max_total_nodes=500000,max_gc_visits=5000000);assert tuple(C.nodes)==before;out['circuits']+=1;out['oracle_collections']+=M.collections
  for root,poly in zip(roots,polys):assert M.monomials(root)==poly;out['polynomial_checks']+=1
  for x in range(1<<n):
   values=C.evaluate(x);assert M.evaluate(roots,x)==[values[w]for w in outputs];out['assignments']+=1;out['value_checks']+=len(outputs)
 M=LiveManager(4,max_nodes=8);forms={2:M.make(0,0,1),3:M.make(1,0,1)};c=M.make(2,0,1);e=M.make(3,0,1);M.xor(c,e);M.xor(forms[2],c);assert len(M.nodes)==8
 try:M.xor(forms[2],forms[3])
 except ZDDLimit as ex:assert str(ex)=='nodecap';M.collect(forms)
 else:raise AssertionError('retrycontrol did not cap')
 root=M.xor(forms[2],forms[3]);assert M.reclaimed>0
 for x in range(16):assert M.evaluate([root],x)==[((x>>0)^(x>>1))&1]
 out['retry_control']+=1
 cfg=json.loads(Path(d['config']).read_bytes());chart=json.loads(Path(d['cover']).read_bytes())['records'][1]['chart'];G,oo,rr,meta=compile_all(cfg,chart,8);b=json.dumps({'nodes':G.nodes,'outputs':oo,'roots':rr},separators=(',',':')).encode();receipt=json.loads(Path(d['graph_receipt']).read_bytes());orders=json.loads(Path(d['order_receipt']).read_bytes());assert receipt['error']is None and orders['error']is None and hashlib.sha256(b).hexdigest()==receipt['out']['variants']['8']['graph_sha256'];out['source_graph_sha256']=hashlib.sha256(b).hexdigest();wires=oo[-1]['wires'][0];samples=[rng.randrange(1<<G.n)for _ in range(128)];controls=[G.evaluate(x)for x in samples]
 for count in d['counts']:
  t=time.monotonic()
  try:
   M,roots=compile_live(G,wires[:count],collect_every=d['collect_every'],**d['caps']);degrees,terms=M.degrees_terms(roots)
   for x,values in zip(samples,controls):assert M.evaluate(roots,x)==[values[w]for w in wires[:count]];out['actual_value_checks']+=count
   artifact={'source_graph_sha256':out['source_graph_sha256'],'nodes':M.nodes,'roots':roots,'wires':wires[:count]};b=json.dumps(artifact,separators=(',',':')).encode()
   with(P/('diagram_'+str(count)+'.json')).open('xb')as f:f.write(b)
   out['variants'][str(count)]={'status':'complete','seconds':time.monotonic()-t,'degrees':degrees,'term_counts':terms,'stats':M.stats(),'artifact_sha256':hashlib.sha256(b).hexdigest()}
  except ZDDLimit as e:out['variants'][str(count)]={'status':'unknown_cap','reason':str(e),'seconds':time.monotonic()-t,'stats':e.stats}
 out['total_seconds']=time.monotonic()-START
except Exception as e:error=repr(e)
finish(out,error,START,'Exact live-frontier ZDD collection with forced collection/retry controls,then changed natural-order correction profiles.','Retain all caps;completed polynomials may support a separately proven restriction pruner;unknowns give no final size/degree or root count.')
