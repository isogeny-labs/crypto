from pathlib import Path
import json,sys,hashlib,base64,tarfile,io,ast,random,runpy
C=Path(__file__).resolve().parent;R=C.parents[1];sys.path.insert(0,str(R));saved={}
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for name,h in json.loads((C/'RAW_INDEX.json').read_text()).items():
  b=t.extractfile(name).read();assert hashlib.sha256(b).hexdigest()==h,name;saved[name]=b
for name,h in json.loads((C/'SOURCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
checks=runpy.run_path(str(C/'GLOBAL_NAME_PREFLIGHT.py'),run_name='preflight')
for row in json.loads((C/'GLOBAL_NAME_PREFLIGHT.json').read_text())['files']:assert checks['undefined_globals'](R/row['path'])==row['undefined_globals']
from research.unified_chart_20261116_v112b.kernel import union_map,forward,compile_map,literal
from research.frontier_certificate_20261116_v111b.kernel import guarded_charts
from research.local_split_circuit_20261116_v100.kernel import Circuit
P=R/'research/unified_chart_20261116_v112b';source=ast.parse((P/'profile.py').read_text());body=next(n for n in source.body if isinstance(n,ast.Try));loop=next(n for n in body.body if isinstance(n,ast.For));out={'toy_cases':0,'toy_original_assignments':0,'toy_compiled_assignments':0};ns={'rng':random.Random(json.loads((P/'DATA.json').read_text())['seed']),'out':out,'Circuit':Circuit,'union_map':union_map,'forward':forward,'compile_map':compile_map,'literal':literal,'guarded_charts':guarded_charts};exec(compile(ast.Module(body=[loop],type_ignores=[]),'frozen_tiny_loop','exec'),ns)
prior=json.loads(saved['research/unified_chart_20261116_v112b/profile_raw.json']);assert prior['error']=="NameError(\"name 'Path' is not defined\")";assert all(out[k]==prior['out'][k]for k in out);assert prior['out']['actual_controls']==0
assert json.loads(saved['research/chart_oracles_20261116_v112c/oracles_raw.json'])['error']is None
print('C112 immutable evidence/source hashes, static missing-import diagnosis and all64tiny selector union cases replayed;corrected actual profile remains unexecuted.')
