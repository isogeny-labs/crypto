"""Record existing Rust output and custody only; no research calculations."""
from pathlib import Path
from datetime import datetime,timezone
import collections,hashlib,json,os,shutil,tarfile
r=Path('/workspace/crypto-chained-balanced-main');s=r/'research/chained_richelot_map_20261117';h=Path('/dev/shm/ct750-first-map14-F');o=s/'evidence/first-arithmetic-F-09';o.mkdir(parents=True,exist_ok=False)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def meta(p):return {'path':str(p),'bytes':p.stat().st_size,'sha256':sha(p),'mode':p.stat().st_mode&0o7777}
phases=json.loads((h/'all-phases.json').read_text());assert phases['first_failure'] is None and len(phases['phases'])==24
assert all(x['exit']==x['expected_exit'] for x in phases['phases'])
raw=(h/'19-first-frozen-arithmetic.stdout').read_text();assert '1 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out' in raw
# Recognize a prefixed first test line as well as standalone records.
lines=[line[line.index('CT14|'):] for line in raw.splitlines() if 'CT14|' in line]
counts=collections.Counter(line.split('|')[1] for line in lines)
assert dict(counts)=={'normalization':1,'attempt':66,'witness':17,'conjugacy':17,'pair':34,'generator':2,'control':5,'scalar':10,'accepted':1},dict(counts)
accepted='CT14|accepted|witnesses=17|ordered_pairs=34|conjugacies=17|closed_quadratic=2|prime_generators=2|scalar_cases=10|constructor_controls=3|wrong_sign_controls=2|kappa=100|no_DLP_solver|unsigned'
assert accepted in lines
assert lines[0].startswith('CT14|normalization|old_scale=30|s=16|c_normal=76|new_scale=13|kappa=100|')
assert len([x for x in lines if x.startswith('CT14|witness|') and x.endswith('closed_route=Quadratic')])==2
for name in ['19-first-frozen-arithmetic.stdout','19-first-frozen-arithmetic.stderr','all-phases.json','actual-compiler-tool-binary-custody.json','prospectively-excluded-compiler-temp-inventory.json','18-list-exact-tests.stdout']:
 shutil.copy2(h/name,o/name)
files={'hold/'+p.relative_to(h).as_posix():p for p in sorted(h.rglob('*')) if p.is_file()}
for name in ['ct750-first-map14-F-caller.stdout','ct750-first-map14-F-caller.stderr','ct750-first-map14-F-caller.exit','ct748-actual-source-F-commit.json','ct748-source-F-push.stdout','ct748-source-F-push.stderr','ct748-source-F-exact-readback.txt']:
 p=Path('/workspace/scratch')/name;assert p.is_file();files['caller-and-publication/'+name]=p
archive=Path('/run/lock/ct751-whole-first14-F.tar.xz');assert not archive.exists()
with tarfile.open(archive,'w:xz',preset=9) as t:
 for name,p in files.items():t.add(p,arcname=name,recursive=False)
inventory=[]
with tarfile.open(archive,'r:xz') as t:
 assert len(t.getmembers())==len(files)
 for name,p in files.items():
  m=t.getmember(name);b=t.extractfile(m).read();assert b==p.read_bytes() and m.mode==p.stat().st_mode&0o7777
  inventory.append({'member':name,'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest(),'mode':m.mode})
removed=[]
for p in sorted(h.rglob('*')):
 if p.is_file() and (p.is_relative_to(h/'source') or p.is_relative_to(h/'deps') or p.name in ['compiler-output','immutable-qualifier']):
  row=next(x for x in inventory if x['member']=='hold/'+p.relative_to(h).as_posix());assert sha(p)==row['sha256'];removed.append(row);p.unlink()
for p in sorted(h.rglob('*'),key=lambda x:len(x.parts),reverse=True):
 if p.is_dir() and not any(p.iterdir()):p.rmdir()
# Retain the entire public D2 primary container in RAM, making its worktree copy sparse.
old=s/'infrastructure-first-strict-Clippy-D2-07/complete-first-strict-Clippy-D2-source-full-rlibs-compiled-ELF-and-logs.tar.xz'
ram=Path('/dev/shm/ct751-retained-primary-archives');ram.mkdir(exist_ok=False)
oldmeta=meta(old);moved=ram/old.name;shutil.copy2(old,moved);assert old.read_bytes()==moved.read_bytes() and old.stat().st_mode==moved.stat().st_mode;old.unlink()
dest=o/'complete-first-map14-F-source-full-rlibs-both-ELFs-and-all-traces.tar.xz';shutil.copy2(archive,dest);assert sha(dest)==sha(archive);archive.unlink()
(o/'WHOLE_ARCHIVE_MEMBERS.json').write_text(json.dumps(inventory,indent=2)+'\n')
data={'time':datetime.now(timezone.utc).isoformat(),'public_source':'f35867831245560527428e3562ab55d74eefb5a0','scientific_language':'Rust','phase_count':24,'all_recorded_phase_exit_expectations_match':True,'new_test_passed':1,'inherited_tests_compiled_and_filtered':13,'original_full_12317_class_population_not_executed':True,'normalization_record':lines[0],'pre_map_manual_prediction':'PCC-20261117-11b4a9; exact old_scale30,s16,c_normal76,new_scale13,kappa100 matched','record_counts':dict(counts),'accepted_record':accepted,'full_stdout':meta(o/'19-first-frozen-arithmetic.stdout'),'full_archive':dict(meta(dest),members=len(inventory),all_full_bytes_and_modes_compared=True,full_three_rlibs_and_compiled_plus_copied_qualifier=True),'all_mathematical_comparisons_executed_in_Rust':True,'Python_role':'metadata/count/hash/I-O only','scientific_standing':'unsigned; same-engine finite controls, no independent full replay, general Lean or performance result','wall_time_note':'Rust test harness prints1.51s; correctness run only, no benchmark/accounting or ratio claim','fixed_controls':'3 malformed constructors and2 wrong dual signs detected','next_task':'cubic-subfield closed-fibre inverse specialization and separately admitted native comparison'}
(o/'ACTUAL_FIRST_MAP14_SCOPE.json').write_text(json.dumps(data,indent=2)+'\n')
compaction={'time':datetime.now(timezone.utc).isoformat(),'loose_full_archive_equal_source_deps_and_both_ELFs_removed':removed,'full_F_archive':meta(dest),'public_D2_worktree_archive_sparse':{'original_path':oldmeta,'retained_complete_primary_container':meta(moved),'public_commit':'f35867831245560527428e3562ab55d74eefb5a0','index_blob_unchanged':True,'restore_original_file_before_local_proof_file_verification':True},'no_original_shared_objects_or_indices_touched':True}
(o/'ACTUAL_FULL_CUSTODY_STORAGE_LAYOUT.json').write_text(json.dumps(compaction,indent=2)+'\n')
print(json.dumps({'archive_bytes':dest.stat().st_size,'archive_sha256':sha(dest),'members':len(inventory),'phase_count':24,'record_counts':dict(counts),'disk_free':shutil.disk_usage(r).free,'shm_free':shutil.disk_usage(h).free,'scratch_free':shutil.disk_usage('/run/lock').free}))
