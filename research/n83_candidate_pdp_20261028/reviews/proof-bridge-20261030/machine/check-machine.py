import pathlib,subprocess,json,hashlib,re,shutil
r=pathlib.Path('/home/ec2-user/n83-candidate-recovery/proof-bridge-machine-check')
allowed={'propext','Classical.choice','Quot.sound'}
def audit(log,expected):
 parsed={}
 for line in log.splitlines():
  m=re.fullmatch(r"'([^']+)' (?:depends on axioms: \[(.*)\]|does not depend on any axioms)",line)
  if m:
   assert m[1] not in parsed
   parsed[m[1]]=[] if not m[2] else [a.strip() for a in m[2].split(',')]
 return set(parsed)==set(expected) and all(set(a)<=allowed for a in parsed.values()),parsed
source=(r/'LefschetzBoundary.lean').read_text();expected=re.findall(r'#print axioms (\w+)',source);expected=['N83LefschetzBoundary.'+n for n in expected]
ok,axioms=audit((r/'lean.log').read_text(),expected);assert ok and len(expected)==8
controls={
 'known_counterexample':('import Std\ntheorem removedBoundCounterexample : (0+4+13=(17:Nat)) ∧ (13≤(13:Nat)) ∧ ¬ (0+2+2*13≤(27:Nat)) := by decide\n#print axioms removedBoundCounterexample\n',0,['removedBoundCounterexample'],True),
 'false_bound':('import Std\nexample : (0+2+2*13≤(27:Nat)) := by omega\n',1,[],False),
 'missing_declaration':('import Std\n#check N83LefschetzBoundary.noSuchDeclaration\n',1,[],False),
 'sorry_hole':('import Std\ntheorem deliberateHole : False := by sorry\n#print axioms deliberateHole\n',0,['deliberateHole'],False),
 'custom_axiom':('import Std\naxiom deliberateUntrusted : False\ntheorem customAxiomControl : False := deliberateUntrusted\n#print axioms customAxiomControl\n',0,['customAxiomControl'],False),
 'native_bypass':('import Std\ntheorem nativeBypassControl : (1:Nat)=1 := by native_decide\n#print axioms nativeBypassControl\n',0,['nativeBypassControl'],False)
}
results=[]
for name,(text,exit_kind,names,want_audit) in controls.items():
 p=r/(name+'.lean');p.write_text(text)
 proc=subprocess.run(['lean','+leanprover/lean4:v4.34.1',str(p)],capture_output=True,text=True,timeout=120)
 log=proc.stdout+proc.stderr;(r/(name+'.log')).write_text(log)
 passed,got=audit(log,names) if names else (False,{})
 assert (proc.returncode==0)==(exit_kind==0),(name,proc.returncode,log)
 assert passed==want_audit,(name,passed,got)
 results.append({'control':name,'compiler_exit_code':proc.returncode,'audit_accepted':passed,'expected_audit_acceptance':want_audit,'reported_axioms':got})
lean=pathlib.Path('/home/ec2-user/.elan/toolchains/leanprover--lean4---v4.34.1/bin/lean')
out={'schema':'n83.proof-bridge-machine-review/v1','producer_revision':'c64f02c176787dbf40d455f95b75bd2dd5bfe5bf','source_sha256':hashlib.sha256((r/'LefschetzBoundary.lean').read_bytes()).hexdigest(),'lean_binary_sha256':hashlib.sha256(lean.read_bytes()).hexdigest(),'toolchain_version':(r/'lean-version.txt').read_text().strip(),'declarations':axioms,'named_declarations_checked':8,'controls':results,'allowed_axioms':sorted(allowed),'lean4checker_available':bool(shutil.which('lean4checker')),'trust_boundary':'Fresh Lean compiler/kernel run using bundled Std. No independent binary/kernel rechecker. This file audits eight arithmetic declarations, not the full polynomial claim.'}
(r/'verification.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({'named_checks':len(axioms),'controls':results},indent=2))
