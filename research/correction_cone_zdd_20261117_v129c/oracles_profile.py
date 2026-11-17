from pathlib import Path
import json,time,random,hashlib
from .common import P,finish
from .kernel import Manager,compile_cones,ZDDLimit
from research.correction_cone_anf_checks_20261117_v129b.kernel import expand
from research.correction_basis_checks_20261117_v128b.kernel import compile_all
from research.local_split_circuit_20261116_v100.kernel import Circuit
START=time.monotonic();out={'circuits':0,'small_assignments':0,'value_checks':0,'polynomial_checks':0,'cap_checks':0,'variants':{},'actual_value_checks':0,'original_complete':False};error=None
try:
 d=json.loads((P/'DATA.json').read_bytes());rng=random.Random(d['seed'])
 for case in range(48):
  n=3+case%5;C=Circuit(n)
  for _ in range(48):
   kind=rng.choice(['and','xor','xor','not']);a=rng.randrange(len(C.nodes));b=rng.randrange(len(C.nodes));C.op(kind,a,None if kind=='not'else b)
  x=C.xor(2,3);y=C.xor(C.xor(2,4),C.xor(3,4));z=C.op('and',x,y);outputs=[0,1,x,y,z]+[rng.randrange(len(C.nodes))for _ in range(5)];before=tuple(C.nodes);M,roots=compile_cones(C,outputs,max_nodes=100000,max_cache=500000,max_work=2000000);polys,stats=expand(C,outputs,max_degree=8,max_monomials=512,max_stored=1000000,max_work=20000000);assert tuple(C.nodes)==before
  for r,poly in zip(roots,polys):assert M.monomials(r)==poly;out['polynomial_checks']+=1
  degrees,terms=M.degrees_terms(roots);assert terms==[len(poly)for poly in polys];assert degrees==[max((m.bit_count()for m in poly),default=-1)for poly in polys];out['circuits']+=1
  for value in range(1<<n):
   wanted=C.evaluate(value);assert M.evaluate(roots,value)==[wanted[w]for w in outputs];out['small_assignments']+=1;out['value_checks']+=len(outputs)
 for reason in ['nodecap','workcap','cachecap']:
  try:
   if reason=='nodecap':M=Manager(2,max_nodes=2);M.make(0,0,1)
   elif reason=='workcap':M=Manager(2,max_work=0);M.xor(0,1)
   else:M=Manager(2,max_cache=0);a=M.make(0,0,1);b=M.make(1,0,1);M.xor(a,b)
  except ZDDLimit as e:assert str(e)==reason;out['cap_checks']+=1
  else:raise AssertionError('missingcap:'+reason)
 cfg=json.loads(Path(d['config']).read_bytes());chart=json.loads(Path(d['cover']).read_bytes())['records'][1]['chart'];G,oo,rr,meta=compile_all(cfg,chart,8);raw=json.dumps({'nodes':G.nodes,'outputs':oo,'roots':rr},separators=(',',':')).encode();receipt=json.loads(Path(d['graph_receipt']).read_bytes());assert receipt['error']is None and hashlib.sha256(raw).hexdigest()==receipt['out']['variants']['8']['graph_sha256'];flat=json.loads(Path(d['flat_receipt']).read_bytes());assert flat['error']is None;out['source_graph_sha256']=hashlib.sha256(raw).hexdigest();wires=oo[-1]['wires'][0];samples=[rng.randrange(1<<G.n)for _ in range(128)];controls=[G.evaluate(x)for x in samples]
 for count in d['counts']:
  t=time.monotonic()
  try:
   M,roots=compile_cones(G,wires[:count],**d['caps']);degrees,terms=M.degrees_terms(roots)
   for x,values in zip(samples,controls):assert M.evaluate(roots,x)==[values[w]for w in wires[:count]];out['actual_value_checks']+=count
   artifact={'source_graph_sha256':out['source_graph_sha256'],'input_bits':G.n,'nodes':M.nodes,'roots':roots,'wires':wires[:count]};b=json.dumps(artifact,separators=(',',':')).encode()
   with(P/('zdd_'+str(count)+'.json')).open('xb')as f:f.write(b)
   out['variants'][str(count)]={'status':'complete','seconds':time.monotonic()-t,'degrees':degrees,'term_counts':terms,'stats':M.stats(),'artifact_sha256':hashlib.sha256(b).hexdigest()}
  except ZDDLimit as e:out['variants'][str(count)]={'status':'unknown_cap','reason':str(e),'seconds':time.monotonic()-t,'stats':e.stats}
 out['total_seconds']=time.monotonic()-START
except Exception as e:error=repr(e)
finish(out,error,START,'CompressedBooleanpolynomial oracle and changed fixed-order correction-cone profiles.','If capped,preserve representation bound and study exact partial assignments or block variable orders;no final degree or cell-pruning claim from caps.')
