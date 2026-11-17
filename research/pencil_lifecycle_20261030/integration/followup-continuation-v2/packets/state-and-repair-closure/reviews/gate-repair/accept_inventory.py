from pathlib import Path
import json,re,hashlib
D=Path(__file__).parent
expected=['GraphFind.run_preserves','GraphFind.run_exists','FindControls.depth_two_run','FindControls.initial_forest','FindControls.initial_values','FindControls.compressed_parent','FindControls.compressed_value','FindControls.all_values_preserved','FindControls.root_fixed','FindControls.potential_reset_wrong']
allowed={'propext','Classical.choice','Quot.sound'}
def check(path):
 i=json.loads(path.read_text());rec=i['receipt'];inv=Path(rec['directory']);reasons=[]
 if i['exit']!=0 or rec['gate_exit']!=0:reasons.append('nonzero process')
 if [x['module'] for x in rec['modules']]!=['Find','Controls','Readback']:reasons.append('wrong module inventory')
 if any(x['native_exit']!=0 for x in rec['modules']):reasons.append('nonzero native result')
 if not rec['bindings_unchanged']:reasons.append('binding change')
 for n in ['Find','Controls','Readback']:
  f=inv/'snapshot'/(n+'.lean');original=D/'subject'/(n+'.lean')
  if not f.exists() or f.read_bytes()!=original.read_bytes():reasons.append('source mismatch '+n)
 text=(inv/'Readback.stdout').read_text() if (inv/'Readback.stdout').exists() else ''
 names=[]
 for line in text.splitlines():
  m=re.fullmatch(r"'([^']+)' (?:depends on axioms: \[([^\]]*)\]|does not depend on any axioms)",line)
  if not m:continue
  names.append(m[1]);a=set(m[2].split(', ')) if m[2] else set()
  if not a<=allowed:reasons.append('unapproved axioms')
 if names!=expected:reasons.append('missing/wrong/duplicate readbacks')
 return {'source':str(path),'accepted':not reasons,'reasons':reasons,'readbacks':names}
results=[check(D/(x+'.json')) for x in ['success1','success2','false','missing','empty','empty-required-readback']]
assert [x['accepted'] for x in results]==[True,True,False,False,False,False]
(D/'inventory-acceptance.json').write_text(json.dumps(results,indent=2)+'\n')
for x in results:print(Path(x['source']).stem,x['accepted'],x['reasons'])
