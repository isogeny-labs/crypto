from pathlib import Path
import json,hashlib,tarfile,shutil,subprocess,os,resource,signal,re,datetime
repo=Path('/workspace/relation-supervisor-live');s=repo/'research/relation_fusion_20261117';root=Path('/workspace/root-fixture-extracted-393');root.mkdir();receipt=json.loads((s/'ROOT_FIXTURE_ARCHIVE_392.json').read_text());archive=s/receipt['archive'];sha=lambda p:hashlib.file_digest(p.open('rb'),'sha256').hexdigest();assert sha(archive)==receipt['sha256']
with tarfile.open(archive,'r:xz') as t:
 for m in t.getmembers():
  rel=Path(m.name);assert not rel.is_absolute() and '..' not in rel.parts and m.isfile() and not m.issym() and not m.islnk();p=root/rel;p.parent.mkdir(parents=True,exist_ok=True)
  with t.extractfile(m) as f,p.open('xb') as out:shutil.copyfileobj(f,out)
  p.chmod(m.mode)
index=json.loads((root/'index.json').read_text());rows=index['files'];assert len(rows)==receipt['indexed_files'];assert {v['path'] for v in rows}=={str(p.relative_to(root)) for p in root.rglob('*') if p.is_file() and p!=root/'index.json'}
for v in rows:p=root/v['path'];assert p.stat().st_size==v['bytes'] and sha(p)==v['sha256'],v['path']
source=root/'source';harness=source/'research/relation_fusion_20261117/root-fixture-repair'
for n,key in [(385,'files'),(386,'source_files')]:
 for rel,h in json.loads((root/f'study/ROOT_REPAIR_PROTOTYPE_SOURCE_{n}.json' if n==385 else root/f'study/ROOT_GEOMETRIC_SOURCE_{n}.json').read_text())[key].items():
  p=source/rel
  if n==385 and p.name=='main.rs':p=harness/'main-385.rs'
  assert sha(p)==h,(n,rel)
for n in [387,389]:
 c=json.loads((root/f'study/ROOT_FIXTURE_SOURCE_{n}.json').read_text())
 for rel,h in c['root_changes'].items():
  p=source/rel
  if n==387 and p.name=='binary_radical_root.rs':p=harness/'binary_radical_root-unformatted-387.rs'
  assert sha(p)==h,(n,rel)
 assert sha(harness/'actual-module-tests.rs')==c['test_harness_sha256']
old=(harness/'binary_radical_root-before-387.rs').read_bytes();final=(source/'src/cryptanalysis/binary_radical_root.rs').read_bytes();assert old.split(b'#[cfg(test)]')[0]==final.split(b'#[cfg(test)]')[0]
assert json.loads((harness/'old/curve_aliases.json').read_bytes())==json.loads((source/'src/cryptanalysis/curve_aliases.json').read_bytes())
base=json.loads((s/'ROOT_UNIT_SOURCE_348.json').read_text())['source_sha256'];changes=[p for p,h in base.items() if sha(repo/p)!=h];assert changes==['src/cryptanalysis/binary_radical_root.rs'],changes
exe=root/'bin/root-fixture-modules';orig=json.loads((s/'ROOT_FIXTURE_PLAN_390.json').read_text());assert sha(exe)==orig['executable_sha256'];roster=subprocess.run([str(exe),'--list'],capture_output=True,text=True,check=True).stdout;assert roster==(s/'development-01/root-fixture-roster-390.stdout').read_text()
plan={'schema':'root-fixture-extracted-replay-plan/v1','recorded_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'archive_sha256':receipt['sha256'],'indexed_files_verified':len(rows),'source_contexts_bound':[385,386,387,389],'executable_sha256':sha(exe),'all_original_tests':7,'cpu':0,'threads':1,'wall_limit_seconds':600,'address_space_bytes':12*1024**3,'each_output_file_limit_bytes':16*1024**2,'full_root_suite_qualified':False,'performance_comparison':False};(s/'ROOT_FIXTURE_REPLAY_PLAN_393.json').write_text(json.dumps(plan,indent=2)+'\n')
def limits():resource.setrlimit(resource.RLIMIT_AS,(12*1024**3,12*1024**3));resource.setrlimit(resource.RLIMIT_FSIZE,(16*1024**2,16*1024**2))
out=s/'development-01/root-fixture-extracted-393.stdout';err=s/'development-01/root-fixture-extracted-393.stderr'
with out.open('xb') as o,err.open('xb') as e:
 p=subprocess.Popen(['taskset','-c','0',str(exe),'--test-threads=1','--nocapture'],stdout=o,stderr=e,env=dict(os.environ,RAYON_NUM_THREADS='1',OMP_NUM_THREADS='1',OPENBLAS_NUM_THREADS='1'),preexec_fn=limits,start_new_session=True);timeout=False
 try:code=p.wait(timeout=600)
 except subprocess.TimeoutExpired:
  timeout=True;os.killpg(p.pid,signal.SIGTERM);code=p.wait(timeout=5)
raw=out.read_text();summary=re.findall(r'test result: ([^\n]+)',raw);assert code==0 and not timeout and '7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;' in raw,(code,raw[-800:]);assert re.findall(r'^test (.+) \.\.\. ok$',raw,re.M)==re.findall(r'^test (.+) \.\.\. ok$',(s/'development-01/root-fixture-tests-390.stdout').read_text(),re.M)
repack=Path('/workspace/root-fixture-repack-393.tar.xz')
with tarfile.open(repack,'x:xz',format=tarfile.GNU_FORMAT,preset=6) as t:
 for p in sorted(root.rglob('*')):
  if p.is_file():
   info=t.gettarinfo(str(p),str(p.relative_to(root)));info.type=tarfile.REGTYPE;info.linkname='';info.mtime=1795392000;info.uid=info.gid=0;info.uname=info.gname='';info.mode=0o755 if p.stat().st_mode&0o111 else 0o644
   with p.open('rb') as f:t.addfile(info,f)
assert sha(repack)==receipt['sha256'];result={'schema':'root-fixture-extracted-replay-outcome/v1','plan':'ROOT_FIXTURE_REPLAY_PLAN_393.json','archive_sha256':receipt['sha256'],'indexed_files_byte_checked':len(rows),'source_contexts_bound':[385,386,387,389],'root348_changed_pinned_paths':changes,'production_oracle_prefix_unchanged':True,'all_alias_values_metadata_preserved':True,'extracted_module_replay_exit_code':code,'wall_timeout':timeout,'summary':summary,'all7_original_pass_names_match390':True,'stdout_sha256':sha(out),'stderr_sha256':sha(err),'archive_repack_byte_equal':True,'independent_host_replay':False,'signed_execution_verdict':False,'full_root_suite_qualified':False,'performance_comparison':False,'15x_admitted':False};(s/'ROOT_FIXTURE_EXTRACTION_393.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result))
