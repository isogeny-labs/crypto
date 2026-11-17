from pathlib import Path
import json,sys,hashlib,base64,tarfile,io,os
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R));saved={}
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for name,h in json.loads((C/'RAW_INDEX.json').read_text()).items():
  b=t.extractfile(name).read();assert hashlib.sha256(b).hexdigest()==h,name;saved[name]=b
for name,h in json.loads((C/'SOURCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
# Restore only already archived exact inputs if absent on a fresh checkout.
def restore(cycle,name):
 p=R/name
 if not p.exists():
  a=R/('research/autolab_cycle_20261116_'+cycle)
  with tarfile.open(fileobj=io.BytesIO(base64.b64decode((a/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:b=t.extractfile(name).read()
  assert hashlib.sha256(b).hexdigest()==json.loads((a/'RAW_INDEX.json').read_text())[name]
  with p.open('xb')as f:f.write(b)
restore('v111','research/frontier_certificate_20261116_v111b/charts.json')
from research.chart_compiler_20261116_v112.kernel import source_graph,compile_all as separate_compile
cfg=json.loads((R/'research/chart_compiler_20261116_v112/DATA.json').read_text());sourcepath=R/cfg['source_graph']
if not sourcepath.exists():
 restore('v105','research/trace_zero_predicate_20261116_v105/templates.json')
 from research.predicate_dispatch_20261116_v105b.kernel import compile_all as old_compile
 g,o,*_=old_compile(json.loads((R/'research/predicate_dispatch_20261116_v105b/DATA.json').read_text()));b=json.dumps({'nodes':g.nodes,'outputs':o},separators=(',',':')).encode();assert hashlib.sha256(b).hexdigest()==cfg['source_graph_sha256']
 with sourcepath.open('xb')as f:f.write(b)
from research.unified_chart_20261116_v112b.kernel import compile_map as unified_compile
from research.parity_chart_20261116_v113b.kernel import compile_map as recount_compile
from research.parity_chart_20261116_v113d.kernel import compile_map as incremental_compile
from research.local_split_circuit_20261116_v100.kernel import cnf
mapping=json.loads(saved['research/unified_chart_20261116_v112d/mapping.json']);source,outputs=source_graph(cfg);graphs={}
for i,((g,o),chart)in enumerate(separate_compile(cfg)):graphs['separate-'+str(i)]=json.dumps({'nodes':g.nodes,'outputs':o},separators=(',',':')).encode()
g,o=unified_compile(source,outputs,mapping);graphs['unified']=json.dumps({'nodes':g.nodes,'outputs':o},separators=(',',':')).encode()
for name,compiler,namespace in [('recount',recount_compile,'parity_chart_20261116_v113b'),('incremental',incremental_compile,'parity_chart_20261116_v113d')]:
 d=json.loads((R/('research/'+namespace+'/DATA.json')).read_text());g,o,stats=compiler(source,outputs,mapping,d['round_cap'],d['event_cap']);b=json.dumps({'nodes':g.nodes,'outputs':o},separators=(',',':')).encode();prior=json.loads(saved['research/'+namespace+'/profile_raw.json'])['out'];assert hashlib.sha256(b).hexdigest()==prior['graph_sha256'];assert len(g.nodes)-2==prior['variables']and len(cnf(g,o))==prior['clauses'];graphs[name]=b
for name,h in json.loads((C/'DERIVED_GRAPH_RECEIPT.json').read_text())['omitted_public_duplicate_files'].items():
 if 'separate-0' in name:key='separate-0'
 elif 'separate-1' in name:key='separate-1'
 elif 'v113d' in name:key='incremental'
 elif 'v113b' in name:key='recount'
 else:key='unified'
 assert hashlib.sha256(graphs[key]).hexdigest()==h,name
print('C113 immutable evidence/source hashes and every omitted exact graph regenerated;47661variables/175144clauses verified;prepared native search not executed.')
