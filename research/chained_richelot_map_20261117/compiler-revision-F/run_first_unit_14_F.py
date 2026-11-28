"""Protocol14 custody/build orchestration only; all scientific work is Rust.

Use a new custody directory. No source repair, fixture selection or arithmetic
is performed here. Failed phases and all outputs are retained.
"""
from pathlib import Path
from datetime import datetime,timezone
import argparse,hashlib,json,os,shutil,subprocess,sys,tarfile

parser=argparse.ArgumentParser()
parser.add_argument('--source-commit',required=True)
parser.add_argument('--custody-dir',required=True)
parser.add_argument('--compiler-temp',required=True)
args=parser.parse_args()
study=Path(__file__).resolve().parent
repo=study.parents[2]
hold=Path(args.custody_dir);temp=Path(args.compiler_temp)
hold.mkdir(exist_ok=False);temp.mkdir(exist_ok=False)
env={k:os.environ[k] for k in ['PATH','HOME','LANG','LC_ALL','TZ','LD_LIBRARY_PATH','RUSTUP_HOME','CARGO_HOME'] if k in os.environ}
env.update(TMPDIR=str(temp),RUST_TEST_THREADS='1',CLIPPY_CONF_DIR=str(hold/'source'))
records=[];first_failure=None;qualifier=None;qualifier_hash=None;dependency_paths=[]
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def now():return datetime.now(timezone.utc).isoformat()
def save():
 (hold/'all-phases.json').write_text(json.dumps({'schema':'bounded-native-custody14/v1','source_commit':args.source_commit,'started':started,'phases':records,'first_failure':first_failure,'compiler_temp_prospectively_excluded':str(temp),'scientific_language':'Rust','Python_role':'metadata,I/O,hashes,subprocess only','new_native_filter':'nonsplit::map::tests_14::frozen_joint_chart_divisor_map_14','no_cold_dependency_build_or_benchmark':True},indent=2)+'\n')
def local_phase(name,fn):
 global first_failure
 row={'name':name,'kind':'metadata-control','started':now(),'expected_exit':0}
 try:row['detail']=fn();row['exit']=0
 except Exception as e:row['exit']=1;row['complete_error']=repr(e)
 row['finished']=now();records.append(row)
 if row['exit']!=0 and first_failure is None:first_failure=name
 save()
 if row['exit']!=0:raise RuntimeError(name)
 return row.get('detail')
def command(name,argv,expected=0):
 global first_failure
 index=len(records);out=hold/f'{index:02d}-{name}.stdout';err=hold/f'{index:02d}-{name}.stderr'
 row={'name':name,'kind':'subprocess','started':now(),'argv':argv,'cwd':str(hold/'source'),'complete_scientific_environment':env,'stdout':out.name,'stderr':err.name,'expected_exit':expected}
 with out.open('xb') as stdout,err.open('xb') as stderr:
  result=subprocess.run(argv,cwd=hold/'source',env=env,stdout=stdout,stderr=stderr)
 row['exit']=result.returncode;row['finished']=now();records.append(row)
 if result.returncode!=expected and first_failure is None:first_failure=name
 save()
 print(name,result.returncode,'expected',expected,flush=True)
 if result.returncode!=expected:raise RuntimeError(name)
 return out,err
started=now()
try:
 def storage():
  free_ram=shutil.disk_usage('/dev/shm').free;free_temp=shutil.disk_usage(temp).free
  assert free_ram>=8*1024*1024,('RAM preflight',free_ram)
  assert free_temp>=3*1024*1024,('compiler scratch preflight',free_temp)
  return {'RAM_available':free_ram,'scratch_available':free_temp,'minimum_RAM':8*1024*1024,'minimum_scratch':3*1024*1024}
 local_phase('frozen-storage-preflight',storage)
 lock=study/'SOURCE_LOCK_14F.json'
 git_env=dict(env,GIT_OPTIONAL_LOCKS='0',GIT_NO_LAZY_FETCH='1')
 def exact_public_source():
  subject=lock.relative_to(repo).as_posix()
  p=subprocess.run(['git','cat-file','blob',args.source_commit+':'+subject],cwd=repo,env=git_env,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
  assert p.returncode==0 and p.stdout==lock.read_bytes(),p.stderr
  manifest=json.loads(p.stdout)
  for row in manifest['files']:
   src=study/row['path'];assert sha(src)==row['sha256'],row['path']
   b=subprocess.run(['git','cat-file','blob',args.source_commit+':'+src.relative_to(repo).as_posix()],cwd=repo,env=git_env,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
   assert b.returncode==0 and b.stdout==src.read_bytes(),row['path']
  return {'all_source_commit_blobs_equal':True,'files':len(manifest['files'])}
 local_phase('exact-public-original-source',exact_public_source)
 manifest=json.loads(lock.read_text());originals=json.loads((study/'ORIGINAL_INPUTS_BEFORE_CODE_14.json').read_text())
 def primary_guard():
  for row in originals['original_source_files']:
   assert sha(Path(row['original_path']))==row['sha256'],row['original_path']
  return {'primary_sources':len(originals['original_source_files'])}
 local_phase('primary-original-source-before',primary_guard)
 def copy_source():
  (hold/'source').mkdir()
  for row in manifest['files']:
   src=study/row['path'];dst=hold/'source'/row['path'];dst.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(src,dst)
   assert sha(dst)==row['sha256']
  shutil.copy2(lock,hold/'source'/lock.name)
  return {'copied_files':len(manifest['files'])+1,'no_history_tree_copy':True}
 local_phase('copy-exact-declared-source',copy_source)
 def copied_guard():
  for row in manifest['files']:assert sha(hold/'source'/row['path'])==row['sha256'],row['path']
  return {'all_copied_source_hashes_equal':True}
 local_phase('copied-source-before-build',copied_guard)
 def dependency_guard():
  assert len(dependency_paths)==len(originals['exact_prebuilt_Rust1991_dependencies'])
  for path,digest in dependency_paths:assert sha(path)==digest,path
  return {'all_full_dependency_hashes_equal':True,'files':len(dependency_paths)}
 def dependency_copy():
  archive=repo/originals['entire_prior_dependency_archive']['path']
  assert sha(archive)==originals['entire_prior_dependency_archive']['sha256']
  (hold/'deps').mkdir()
  with tarfile.open(archive,'r:xz') as tar:
   for row in originals['exact_prebuilt_Rust1991_dependencies']:
    member=tar.getmember(row['member']);b=tar.extractfile(member).read();assert hashlib.sha256(b).hexdigest()==row['sha256'] and len(b)==row['bytes']
    dst=hold/'deps'/Path(row['member']).name;dst.write_bytes(b);dst.chmod(member.mode);dependency_paths.append((dst,row['sha256']))
  return {'full_dependency_binaries':len(dependency_paths),'no_dependency_build':True}
 local_phase('full-locked-prebuilt-dependencies',dependency_copy)
 local_phase('dependency-binary-before',dependency_guard)
 for name in ['rustc','clippy-driver','rustfmt']:
  out,_=command(name+'-version',['rustup','run','1.99.1',name,'--version'])
  if name=='rustc':assert out.read_text().startswith('rustc 1.99.1 ')
 def binary_custody():
  rows=[]
  for name in ['rustc','clippy-driver','rustfmt']:
   p=subprocess.run(['rustup','which','--toolchain','1.99.1',name],env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE,check=True)
   path=Path(p.stdout.decode().strip());rows.append({'tool':name,'path':str(path),'bytes':path.stat().st_size,'sha256':sha(path)})
  (hold/'actual-compiler-tool-binary-custody.json').write_text(json.dumps(rows,indent=2)+'\n');return rows
 local_phase('compiler-tool-binary-custody',binary_custody)
 for label,source in [('positive','fn main() { std::hint::black_box(1_u64); }\n'),('negative','fn main() { let a = 1_u64; std::hint::black_box(a == a); }\n')]:
  fixture=hold/'source'/('clippy-sentinel-'+label+'.rs');fixture.write_text(source)
  _,err=command('clippy-sentinel-'+label,['timeout','--signal=TERM','--kill-after=5s','120s','rustup','run','1.99.1','clippy-driver','--crate-name','clippy_sentinel_'+label,'--edition=2021','--emit=metadata','-D','clippy::eq_op',str(fixture),'-o',str(hold/('clippy-sentinel-'+label+'.rmeta'))],0 if label=='positive' else 1)
  if label=='negative':
   diagnostic=err.read_text()
   assert 'clippy::eq-op' in diagnostic and 'equal expressions as operands' in diagnostic
 deps=hold/'deps'
 bigint=next(deps.glob('libnum_bigint-*.rlib'));traits=next(deps.glob('libnum_traits-*.rlib'))
 common=['--crate-name','chained_richelot_operational_map_14','--test','--edition=2021','src/lib.rs','-L','dependency='+str(deps),'--extern','num_bigint='+str(bigint),'--extern','num_traits='+str(traits),'-C','opt-level=3','-C','codegen-units=1','-C','strip=symbols']
 command('compile-only',['timeout','--signal=TERM','--kill-after=5s','120s','rustup','run','1.99.1','rustc',*common,'-o',str(hold/'compiler-output')])
 command('strict-direct-clippy',['timeout','--signal=TERM','--kill-after=5s','120s','rustup','run','1.99.1','clippy-driver',*common,'--emit=metadata','-D','warnings','-o',str(hold/'strict-clippy.rmeta')])
 command('format',['rustup','run','1.99.1','rustfmt','--check','--edition','2021','src/lib.rs'])
 local_phase('copied-source-before-arithmetic',copied_guard)
 def make_qualifier():
  global qualifier,qualifier_hash
  qualifier=hold/'immutable-qualifier';shutil.copy2(hold/'compiler-output',qualifier);qualifier.chmod(0o555);qualifier_hash=sha(qualifier)
  assert qualifier_hash==sha(hold/'compiler-output')
  return {'path':str(qualifier),'sha256':qualifier_hash,'mode':oct(qualifier.stat().st_mode&0o7777),'bytes':qualifier.stat().st_size}
 local_phase('immutable-qualifier-before',make_qualifier)
 command('list-exact-tests',[str(qualifier),'--list'])
 command('first-frozen-arithmetic',['timeout','--signal=TERM','--kill-after=5s','120s',str(qualifier),'nonsplit::map::tests_14::frozen_joint_chart_divisor_map_14','--exact','--nocapture','--test-threads=1'])
except Exception as error:
 if first_failure is None:first_failure='orchestration-exception'
 (hold/'whole-orchestration-exception.txt').write_text(repr(error)+'\n')
finally:
 for name,fn in [('primary-original-source-after',lambda: primary_guard()),('copied-source-after',lambda: copied_guard()),('dependency-binary-after',lambda: dependency_guard())]:
  try:
   if ('primary_guard' if name.startswith('primary') else 'copied_guard' if name.startswith('copied') else 'dependency_guard') in globals():local_phase(name,fn)
  except Exception:pass
 if qualifier is not None:
  try:
   def qualifier_after():
    assert sha(qualifier)==qualifier_hash and qualifier.stat().st_mode&0o7777==0o555
    return {'same_full_qualifier_hash_and_mode':True}
   local_phase('immutable-qualifier-after',qualifier_after)
  except Exception:pass
 leftovers=[{'path':str(p.relative_to(temp)),'bytes':p.stat().st_size,'sha256':sha(p)} for p in sorted(temp.rglob('*')) if p.is_file()]
 (hold/'prospectively-excluded-compiler-temp-inventory.json').write_text(json.dumps(leftovers,indent=2)+'\n')
 save()
 print('first_failure',first_failure,'phases',len(records),flush=True)
sys.exit(1 if first_failure is not None else 0)
