"""Verify both archival Lean attempts and the accepted theorem audit."""
from pathlib import Path
import hashlib, json, subprocess

h = Path(__file__).resolve().parent
repo = h.parents[3]
failed = h / 'runs/quadratic-lean76-01'
accepted = h / 'runs/quadratic-lean76-02'
assert json.loads((failed / 'failure.json').read_text()) == {
    'attempt': 'quadratic-lean76-01', 'status': 'rejected_before_compile',
    'reason': 'driver referenced nonexistent FiveFiber76.lean due mechanical rename; raw copy retained',
    'compile_started': False}
assert not (failed / 'compile.exit').exists()
assert (accepted / 'compile.exit').read_text().strip() == '0'
assert (accepted / 'audit.exit').read_text().strip() == '0'
assert (accepted / 'mathlib-revision.txt').read_text().strip() == 'd13f23b723b8a846827a245b89c10fc7d3f11612'
audit = (accepted / 'audit.log').read_text()
assert 'ok    Quadratic76.norm_R  ' in audit
assert 'ok    Quadratic76.norm_R_add_one  ' in audit
assert 'ok    Quadratic76.rational_zero_degrees  ' in audit
assert 'audited 3 theorems in [Quadratic76]; 0 failing, 0 trusted (native_decide)' in audit
assert not any(s.startswith(('FAIL  ', 'TRUST ')) for s in audit.splitlines())
rev = (accepted / 'source-revision.txt').read_text().strip()
digest = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
pins=[]
for local,source in [('driver.sh','LEAN_DRIVER.sh'),('launcher.sh','REPLAY_LEAN.sh'),
                     ('Quadratic76.lean','lean/Quadratic76.lean'),('LEAN-PROTOCOL.md','LEAN-PROTOCOL.md')]:
    path = str((h/source).relative_to(repo))
    assert subprocess.check_output(['git','show',rev+':'+path],cwd=repo) == (accepted/local).read_bytes()
    pins.append({'artifact':local,'git_source':path,'revision':rev,'sha256':digest(accepted/local)})
assert subprocess.check_output(['git','show',rev+':tools/lean/AuditAxioms.lean'],cwd=repo) == (accepted/'AuditAxioms.lean').read_bytes()
pins.append({'artifact':'AuditAxioms.lean','git_source':'tools/lean/AuditAxioms.lean','revision':rev,'sha256':digest(accepted/'AuditAxioms.lean')})
results={'schema':'koblitz.lean-results/v1','round':76,'attempts':[
    {'attempt':failed.name,'status':'rejected_before_compile','reason':'mechanical launcher filename error'},
    {'attempt':accepted.name,'status':'accepted','revision':rev,'mathlib_revision':(accepted/'mathlib-revision.txt').read_text().strip(),
     'compile_exit':0,'audit_exit':0,'theorems':3,'failing':0,'trusted':0,
     'artifacts':[{'path':str(p.relative_to(h)),'sha256':digest(p),'bytes':p.stat().st_size}
                  for p in sorted(accepted.iterdir()) if p.is_file()]}],
    'source_pins':pins}
(h/'lean-results.json').write_text(json.dumps(results,indent=2)+'\n')
print(json.dumps({'accepted':1,'rejected_before_compile':1,'theorems':3,'failing':0,'trusted':0,'source_pins':5}))
