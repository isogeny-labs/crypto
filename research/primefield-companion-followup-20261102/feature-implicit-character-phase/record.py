from pathlib import Path
import json,hashlib
p=Path(__file__).resolve().parent;sha=lambda f:hashlib.sha256(f.read_bytes()).hexdigest()
rows=json.loads((p/'generation.json').read_text())['copied']
for row in rows:assert sha(p/row['local'])==row['sha256']==sha(p.parent/row['source'])
for version,build,run in [(1,1,None),(2,0,101),(3,0,0)]:
 v=p/f'validation0{version}';deps={}
 if build==0:
  dep=Path(f'/tmp/t1925-v{version}-check.d')
  for x in dep.read_text().splitlines()[0].split(': ',1)[1].split():
   f=(p/x).resolve();actual=v/'observation.rs.snapshot' if version==2 and f.name=='observation.rs' else f
   deps[str(f)]={'sha256':sha(actual),'artifact':str(actual)}
 if version==3:
  assert not(v/'check.stderr').read_bytes()
  out=(v/'check.stdout').read_text();assert 'observation_fiber_controls=36' in out and 'squarefree_intersection_equalities=2112' in out
 receipt={'scope':'Standalone exact quotient-phase and compressed-observation controls; no end-to-end recovery or timing claim','build_exit':build,'check_exit':run,'limits_seconds':{'build':90,'check':60},'commands':[f'rustc --edition=2021 -O -C overflow-checks=yes --emit=link,dep-info driver.rs -o /tmp/t1925-v{version}-check',f'/tmp/t1925-v{version}-check' if run is not None else 'not executed'],'source_hashes':deps,'logs':{f.name:sha(f) for f in v.iterdir() if f.suffix in ['.stdout','.stderr','.snapshot']},'copied_sources':len(rows)}
 if build==0:receipt['binary_sha256']=sha(Path(f'/tmp/t1925-v{version}-check'))
 (v/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print('copied_sources',len(rows),'final_compiler_dependencies',len(deps))
