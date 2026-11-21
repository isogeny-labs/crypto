from pathlib import Path
from datetime import datetime,timezone
import hashlib,importlib.util,json,shutil,subprocess,sys,re
r=Path('/workspace/crypto');p=r/'research/chained_jacobian_transfers_20261110';g=p/'evidence/cubic-word-final-raw-value-proof-03';g.mkdir()
source=p/'lean/CubicNormWordBridge01.lean';b=source.read_bytes();prior=(p/'lean/CubicNormDescent.lean').read_bytes();assert hashlib.sha256(prior).hexdigest()=='18384e633660a419a4d7f671a409b9e2858b0cf5e234b896cc427f5961fa6c41' and b.startswith(prior)
assert b==Path('/workspace/cryptanalysis/.resume/cubic-word-proof-source-20261116-02/CubicNormWordBridge01.lean').read_bytes()
counts=[]
for label,h in [('Root435',Path('/dev/shm/ct329-cubic-word-final-Root435')),('Consumer434',Path('/dev/shm/ct329-cubic-word-final-Consumer434'))]:
 assert (h/'actual.exit').read_text().strip()=='0'
 text=(h/'entire-caller.stdout').read_text();m=re.search(r'PASS\s+(\d+) thm\s+0 trusted',text);assert m;counts.append(int(m[1]));shutil.copytree(h,g/label)
assert counts[0]==counts[1]
shutil.copytree('/dev/shm/ct328-cubic-word-raw-output-first-proof',g/'entire-final-raw-compile')
for f in g.rglob('*.lean'):f.rename(f.with_name(f.name+'.txt'))
shutil.copyfile(source,g/'CubicNormWordBridge01.lean')
for label,repo in [('Root',r),('Consumer',Path('/workspace/cryptanalysis'))]:
 d=g/('whole-original-'+label+'-proof-tools');d.mkdir()
 for name in ['proof-gate.sh','AuditAxioms.lean','trusted-native.txt']:
  f=repo/'tools/lean'/name
  if f.exists():shutil.copyfile(f,d/(name+'.txt'))
common=p/'evidence/cubic-norm-descent-final-04/entire-original-common.rs.txt';assert hashlib.sha256(common.read_bytes()).hexdigest()=='832fd28810909194875a9438e164ec1b45dcefbaa83257b4e3fad3095ef865d6';shutil.copyfile(common,g/'entire-original-common.rs.txt')
shutil.copyfile(__file__,g/'entire-actual-capture-and-original-binding-command.py')
receipt=dict(at=datetime.now(timezone.utc).isoformat(),actual_source_sha256=hashlib.sha256(b).hexdigest(),immutable_prior_source_sha256=hashlib.sha256(prior).hexdigest(),exact_prior_bytes_retained_as_prefix=True,original_Root435_gate_exit=0,original_Consumer434_gate_exit=0,audited_each=counts[0],trusted_each=0,new_named_statements=10,original_common_sha256=hashlib.sha256(common.read_bytes()).hexdigest(),scope='exact Lean UInt64 coefficient-square operators/raw output values at q223,c3 and canonical inputs; counter updates/constructor/compiler/binary/actual field/Jacobian/global/cost outside scope',first_failure='5cb553 complete nested-mod omega failure',intermediate='5a85de decoded-only dual22/0 source remains whole')
(g/'actual-final-proof-scope.json').write_text(json.dumps(receipt,indent=2)+'\n')
statement=f'Final additive cubic UInt64 coefficient bridge: both unchanged original RootLean4.35/Consumer4.34.1 gates pass{counts[0]} audited declarations/0 trusted on byte-identical source{receipt["actual_source_sha256"]}, exact old18384e proof source retained as prefix. Ten new named statements give canonical product bound49284, decoded square coordinate equality with original natural/modular routine, decoded12/24 exclusions/control, all-UInt64 output canonicality, exact raw natural output values, raw UInt64 output12/24 exclusions and raw square4 control. Each ordinary Rust-style multiplication/addition before reduction avoids wrap via explicit product/pair/triple and scaled/final modulo2^64 lemmas (largest unreduced canonical triple147852; final reduced sum888). No223cubed enumeration, trusted native_decide or new axioms; only propext/Classical.choice/Quot.sound. Whole first failed source/output5cb553 and intermediate22/0 source5a85de remain intact. Original common.rs832fd288 is fully retained, but this formal coefficient model does not cover operation counters, constructor selection, compilation/native binary execution, actual field/Jacobian/global geometry or cost. New exact statement bindings follow original PCC declaration finder; all claims unsigned. No native historical population or paid work.'
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement',statement]
for f in sorted(g.rglob('*')):
 if f.is_file():cmd+=['--proof','attestation:'+f.relative_to(r).as_posix()]
data=subprocess.check_output(cmd,cwd=r,text=True).strip()
spec=importlib.util.spec_from_file_location('original_pcc_binding_word',r/'tools/pcc/pcc.py');m=importlib.util.module_from_spec(spec);sys.modules[spec.name]=m;spec.loader.exec_module(m)
names=['canonical_product_bound','word_square_coordinates','word_square_ne12','word_square_ne24','word_square_control','word_square_canonical','word_square_exact_values','word_square_raw_ne12','word_square_raw_ne24','word_square_raw_control'];bindings=[]
for name in names:
 qualified='CubicNormWordBridge01.'+name;actual=m.lean_find_declaration([g/'CubicNormWordBridge01.lean'],qualified);assert actual is not None;normalized=actual[1]
 proof=dict(path=(g/'CubicNormWordBridge01.lean').relative_to(r).as_posix(),theorem=qualified,decl_sha256=hashlib.sha256(normalized.encode()).hexdigest())
 cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','theorem','--claim-type','interpretive','--statement',normalized,'--scope','Exact canonical-input q223/c3 coefficient-word model. Original dual-gate component '+data+'; counters/constructor/compiler/binary/field/Jacobian/global/cost separate. Unsigned until original PCC standing.','--proof','lean:'+json.dumps(proof)]
 claim=subprocess.check_output(cmd,cwd=r,text=True).strip();bindings.append(dict(theorem=qualified,claim=claim,normalized_statement=normalized,proof=proof))
(g/'exact-ten-new-statement-bindings.json').write_text(json.dumps(dict(data=data,bindings=bindings),indent=2)+'\n')
entry=f'\n---\n\n{datetime.now(timezone.utc).isoformat()}\n# Final raw-value UInt64 square bridge\n\n## What was done\n{data}: both unchanged original Lean gates{counts[0]} audited/0 trusted on identical final source;10 exact original PCC bindings cover raw/decoded coordinate equality, canonical outputs,12/24 exclusions and allowed4 controls. Whole original18384e prefix/source/common.rs, first failure5cb553 and intermediate5a85de remain exact. [pcc:unverified]\n\n## Decisions made\nPublish additive written derivation/report and new dedicated dual-gate source CI without changing norm01 assets. Raw values now equal the original Nat/modular routine; Rust counters/constructor/compiler/native binary and full geometric/cost bridges remain separate. [pcc:unverified]\n\n## Open threads\nSource publication after current4415 genuine metadata integration/checks; runtime packaging source/context is metadata-only and paid work remains stopped. Current observations11:05/11:10. [pcc:unverified]\n\n## Errors\nOnly first nested-mod tactic attempt failed; exact bounded simplification fixes proof without changing its statement/model. No trusted native_decide or new axiom/signed standing. [pcc:unverified]\n'
for n in ['docs/session-logs/2026-11-16.md','.resume/RESUME.md']:
 with (r/n).open('a') as f:f.write(entry)
print(data,json.dumps(receipt));print(json.dumps([(b['theorem'],b['claim']) for b in bindings]))
