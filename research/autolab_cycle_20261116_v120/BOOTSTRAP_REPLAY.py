from pathlib import Path
import os,sys,json,hashlib,base64,tarfile,io,runpy
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R))
template='research/trace_zero_predicate_20261116_v105/templates.json';expected='dc4f32d05c83f23654c725415db7c02107c7a8fa2539fdfc90d92f3ca4f60baf';p=R/template
if not p.exists():
 archive=R/'research/autolab_cycle_20261116_v105/RAW_EVIDENCE.tar.xz.b64'
 with tarfile.open(fileobj=io.BytesIO(base64.b64decode(archive.read_bytes())),mode='r:xz')as t:b=t.extractfile(template).read()
 assert hashlib.sha256(b).hexdigest()==expected;p.parent.mkdir(parents=True,exist_ok=True)
 with p.open('xb')as f:f.write(b)
assert hashlib.sha256(p.read_bytes()).hexdigest()==expected
cfg=json.loads((R/'research/chart_compiler_20261116_v112/DATA.json').read_text());matrix=R/cfg['matrix'];assert hashlib.sha256(matrix.read_bytes()).hexdigest()==cfg['matrix_sha256'];graph=R/cfg['source_graph']
if not graph.exists()or '--rebuild-check'in sys.argv:
 from research.predicate_dispatch_20261116_v105b.kernel import compile_all
 d=json.loads((R/'research/predicate_dispatch_20261116_v105b/DATA.json').read_text());G,outputs,choices=compile_all(d);b=json.dumps({'nodes':G.nodes,'outputs':outputs},separators=(',',':')).encode();assert hashlib.sha256(b).hexdigest()==cfg['source_graph_sha256']
 if not graph.exists():
  graph.parent.mkdir(parents=True,exist_ok=True)
  with graph.open('xb')as f:f.write(b)
assert hashlib.sha256(graph.read_bytes()).hexdigest()==cfg['source_graph_sha256'];runpy.run_path(str(C/'REPLAY.py'),run_name='__main__')
