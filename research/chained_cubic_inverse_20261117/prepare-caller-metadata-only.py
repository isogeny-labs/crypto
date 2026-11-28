"""Build exact metadata-only custody caller/lock; no scientific execution."""
from pathlib import Path
from datetime import datetime,timezone
import ast,hashlib,json,shutil
r=Path('/workspace/crypto-chained-balanced-main');s=r/'research/chained_cubic_inverse_20261117';f=r/'research/chained_richelot_map_20261117/compiler-revision-F'
inputs=json.loads((s/'ORIGINAL_INPUTS_BEFORE_NEW_CODE_15.json').read_text());flat=dict(inputs['old_input_and_full_dependency_manifest'])
flat['original_source_files']=flat['original_source_files']+inputs['all_original_F_declared_files']+[inputs['original_F_manifest_lock']]
for row in flat['original_source_files']:
 if 'original_path' not in row:row['original_path']=row.pop('path')
flat['pinned_Lean_compiler_and_direct_imports']=inputs['pinned_Lean_compiler_and_direct_imports']
(s/'ORIGINAL_INPUTS_BEFORE_CODE_15.json').write_text(json.dumps(flat,indent=2)+'\n')
caller=(f/'run_first_unit_14_F.py').read_text()
caller=caller.replace('repo=study.parents[2]','repo=study.parents[1]').replace('SOURCE_LOCK_14F.json','SOURCE_LOCK_15.json').replace('ORIGINAL_INPUTS_BEFORE_CODE_14.json','ORIGINAL_INPUTS_BEFORE_CODE_15.json')
caller=caller.replace('bounded-native-custody14/v1','bounded-native-custody15/v1').replace('nonsplit::map::tests_14::frozen_joint_chart_divisor_map_14','nonsplit::map::tests_14::frozen_cubic_inverse_15').replace('chained_richelot_operational_map_14','chained_cubic_inverse_15')
caller=caller.replace("records=[];first_failure=None;qualifier=None;qualifier_hash=None;dependency_paths=[]","records=[];first_failure=None;qualifier=None;qualifier_hash=None;dependency_paths=[];tool_rows=[];proof_path=None;proof_hash=None")
caller=caller.replace('  rows=[]\n  for name', '  global tool_rows\n  rows=[]\n  for name',1)
caller=caller.replace("  (hold/'actual-compiler-tool-binary-custody.json').write_text", "  for row in originals['pinned_Lean_compiler_and_direct_imports']:\n   path=Path(row['path']);assert sha(path)==row['sha256'];rows.append(dict(row,tool='Lean-compiler-or-import'))\n  tool_rows=rows\n  (hold/'actual-compiler-tool-binary-custody.json').write_text",1)
marker=" local_phase('copied-source-before-arithmetic',copied_guard)\n"
assert caller.count(marker)==1
formal=''' local_phase('copied-source-before-arithmetic',copied_guard)
 def prepare_proof():
  global proof_path,proof_hash
  folder=hold/'immutable-proof-source';folder.mkdir();proof_path=folder/'Inverse15.lean'
  shutil.copy2(hold/'source/formal/Inverse15.lean',proof_path);proof_path.chmod(0o444);proof_hash=sha(proof_path)
  assert proof_hash==sha(study/'formal/Inverse15.lean')
  return {'path':str(proof_path),'sha256':proof_hash,'mode':'0444'}
 local_phase('immutable-proof-source-before',prepare_proof)
 lean=originals['pinned_Lean_compiler_and_direct_imports'][0]['path']
 command('Lean-version',[lean,'--version'])
 (hold/'proof-output').mkdir()
 out,_=command('first-frozen-Lean-proof',['timeout','--signal=TERM','--kill-after=5s','120s',lean,'-DwarningAsError=true','-o',str(hold/'proof-output/Inverse15.olean'),str(proof_path)])
 def axioms_record():
  text=out.read_text();names=['norm_cubic','inverse_by_square','cubic_inverse_pair']
  assert all('ChainedCubicInverse15.'+name in text for name in names),text
  assert 'sorryAx' not in text and 'ofReduceBool' not in text,text
  allowed={'propext','Classical.choice','Quot.sound'}
  import re
  for group in re.findall(r'depends on axioms:\\s*\\[([^]]*)\\]',text):
   actual={x.strip() for x in group.split(',') if x.strip()};assert actual<=allowed,actual
  assert sha(proof_path)==proof_hash and proof_path.stat().st_mode&0o7777==0o444
  return {'complete_printed_axioms':text,'standard_logical_axioms_only':True,'conditional_standard_Field_no_Rust_Field_instance':True}
 local_phase('complete-Lean-axioms-and-source-record',axioms_record)
'''
caller=caller.replace(marker,formal)
old=" command('list-exact-tests',[str(qualifier),'--list'])"
new=" out,_=command('list-exact-tests',[str(qualifier),'--list'])\n assert out.read_text().count('nonsplit::map::tests_14::frozen_cubic_inverse_15: test')==1,out.read_text()"
assert old in caller;caller=caller.replace(old,new)
marker=" if qualifier is not None:\n"
after=''' if tool_rows:
  try:
   def tools_after():
    for row in tool_rows:assert sha(Path(row['path']))==row['sha256'],row['path']
    return {'all_full_compiler_and_Lean_import_hashes_equal':True,'files':len(tool_rows)}
   local_phase('compiler-tools-and-Lean-imports-after',tools_after)
  except Exception:pass
 if proof_path is not None:
  try:
   def proof_after():
    assert sha(proof_path)==proof_hash and proof_path.stat().st_mode&0o7777==0o444
    return {'same_full_proof_source_hash_and_mode':True}
   local_phase('immutable-proof-source-after',proof_after)
  except Exception:pass
 if qualifier is not None:
'''
assert caller.count(marker)==1;caller=caller.replace(marker,after)
ast.parse(caller);(s/'run_first_unit_15.py').write_text(caller)
# Keep an exact JSON encoding of the complete raw source diff; no whitespace edit.
p=s/'ACTUAL_EXPLICIT_NATIVE_CHANGES.diff';b=p.read_bytes();(s/'FULL_EXPLICIT_NATIVE_CHANGES.json').write_text(json.dumps({'entire_raw_diff':b.decode(),'raw_bytes':len(b),'raw_sha256':hashlib.sha256(b).hexdigest()},indent=2)+'\n')
raw=Path('/run/lock/ct776-entire-native15-source-diff.txt');shutil.copy2(p,raw);assert raw.read_bytes()==b;p.unlink()
shutil.copy2('/workspace/scratch/ct775_prepare_native15_source.py',s/'prepare-native-source-metadata-only.py')
shutil.copy2(Path(__file__),s/'prepare-caller-metadata-only.py')
files=[]
roots=['src','vendor','reused-core','new-code','formal','primary-inputs','unchanged-baseline-inputs']
for name in roots:
 for p in sorted((s/name).rglob('*')):
  if p.is_file():files.append(p)
for name in ['Cargo.toml','Cargo.lock','fixtures-12.csv','run_first_unit_15.py','ORIGINAL_INPUTS_BEFORE_CODE_15.json','ORIGINAL_INPUTS_BEFORE_NEW_CODE_15.json','PROTOCOL_15.md','PROOF_15.md','ACTUAL_NATIVE_SOURCE_PREPARATION_15.json','FULL_EXPLICIT_NATIVE_CHANGES.json','prepare-native-source-metadata-only.py','prepare-caller-metadata-only.py']:files.append(s/name)
rows=[{'path':str(p.relative_to(s)),'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'mode':p.stat().st_mode&0o7777} for p in sorted(files)]
lock={'schema':'source-input-lock15/v1','time':datetime.now(timezone.utc).isoformat(),'public_prospective_protocol':'8dfcbd23ec70b3aa8baa497fe35dd3dfa1e57e74','prospective_claim':'PCC-20261117-b0ba92','files':rows,'native_filter':'nonsplit::map::tests_14::frozen_cubic_inverse_15','expected_native_test_scope':'one new pass and14 inherited filtered; complete original criteria unchanged','first_native_or_formal_execution_has_not_occurred':True}
(s/'SOURCE_LOCK_15.json').write_text(json.dumps(lock,indent=2)+'\n')
print(json.dumps({'locked_files':len(rows),'original_source_guards':len(flat['original_source_files']),'native_driver':'Rust','formal_driver':'Lean','caller_role':'metadata/hash/I-O/subprocess only','no_scientific_call':True}))
