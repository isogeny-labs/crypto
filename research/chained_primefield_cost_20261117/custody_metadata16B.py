"""File/hash/JSON/process-record custody only. Scientific control is shell/Rust."""
from pathlib import Path
from datetime import datetime,timezone
import argparse,hashlib,json,os,shutil,subprocess,sys,tarfile
p=argparse.ArgumentParser();p.add_argument('action');p.add_argument('--hold',required=True);p.add_argument('--temp');p.add_argument('--source-commit');p.add_argument('extra',nargs='*')
raw=sys.argv[1:];cut=raw.index('--') if '--' in raw else len(raw);a=p.parse_args(raw[:cut])
if cut<len(raw):a.extra=raw[cut+1:]
h=Path(a.hold);study=Path(__file__).resolve().parent;repo=study.parents[1]
def sha(x):return hashlib.sha256(x.read_bytes()).hexdigest()
def now():return datetime.now(timezone.utc).isoformat()
def write_new(path,value):
 with path.open('x') as f:json.dump(value,f,indent=2);f.write('\n')
def load_state():return json.loads((h/'all-phases.json').read_text())
def save(state):(h/'all-phases.json').write_text(json.dumps(state,indent=2)+'\n')
if a.action=='init':
 h.mkdir(exist_ok=False);temp=Path(a.temp);temp.mkdir(exist_ok=False)
 write_new(h/'CONTEXT16.json',{'source_commit':a.source_commit,'study':str(study),'repo':str(repo),'compiler_temp':str(temp),'started':now()})
 save({'schema':'native-cost16-custody/v1','source_commit':a.source_commit,'phases':[],'first_failure':None,'scientific_logic':'Rust','build_run_control':'thin shell','Python_role':'file/hash/JSON/process metadata only','compiler_temp_prospectively_excluded':str(temp)})
ctx=json.loads((h/'CONTEXT16.json').read_text());study=Path(ctx['study']);repo=Path(ctx['repo']);temp=Path(ctx['compiler_temp'])
def append(name,exit,detail=None,expected=0,kind='metadata-control'):
 state=load_state();row={'name':name,'recorded':now(),'kind':kind,'exit':exit,'expected_exit':expected,'detail':detail}
 state['phases'].append(row)
 if exit!=expected and state['first_failure'] is None:state['first_failure']=name
 save(state)
def source_lock():return json.loads((study/'SOURCE_LOCK_16B.json').read_text())
def originals():return json.loads((study/'ORIGINAL_INPUTS_BEFORE_NEW_CODE_16.json').read_text())
def primary_guard():
 rows=originals()['original_source_files']
 for row in rows:assert sha(Path(row['original_path']))==row['sha256'],row['original_path']
 return {'full_original_hashes_equal':len(rows)}
def copied_guard():
 for row in source_lock()['files']:
  path=h/'source'/row['path'];assert sha(path)==row['sha256'] and path.stat().st_mode&0o7777==row['mode'],row['path']
 return {'all_copied_sources_equal':len(source_lock()['files'])}
def dependency_guard():
 rows=originals()['exact_prebuilt_Rust1991_dependencies']
 for row in rows:assert sha(h/'deps'/Path(row['member']).name)==row['sha256'],row['member']
 return {'all_full_dependency_hashes_equal':len(rows)}
def tool_guard():
 rows=json.loads((h/'COMPILER_TOOL_BINARY_MANIFEST16.json').read_text())
 for row in rows:assert sha(Path(row['path']))==row['sha256'],row['path']
 return {'all_full_tool_hashes_equal':len(rows)}
def scientific_environment():
 keys=['PATH','HOME','LANG','LC_ALL','TZ','LD_LIBRARY_PATH','RUSTUP_HOME','CARGO_HOME','TMPDIR','RUST_TEST_THREADS','CLIPPY_CONF_DIR']
 return {k:os.environ[k] for k in keys if k in os.environ}
if a.action=='record-start':
 name,expected,*argv=a.extra;state=load_state();assert not any(x['name']==name for x in state['phases'])
 state['phases'].append({'name':name,'kind':'subprocess','started':now(),'expected_exit':int(expected),'exit':None,'argv':argv,'cwd':str(h/'source'),'complete_whitelisted_scientific_environment':scientific_environment(),'stdout':name+'.stdout','stderr':name+'.stderr'});save(state);sys.exit(0)
if a.action=='record-end':
 name,code=a.extra;state=load_state();rows=[x for x in state['phases'] if x['name']==name];assert len(rows)==1 and rows[0]['exit'] is None;row=rows[0];row['exit']=int(code);row['finished']=now()
 if row['exit']!=row['expected_exit'] and state['first_failure'] is None:state['first_failure']=name
 save(state);sys.exit(0)
def prepare():
 ram=shutil.disk_usage('/dev/shm').free;scratch=shutil.disk_usage(temp).free
 assert ram>=8*1024*1024,('RAM preflight',ram)
 assert scratch>=3*1024*1024,('scratch preflight',scratch)
 lock=study/'SOURCE_LOCK_16B.json';E=dict(os.environ,GIT_OPTIONAL_LOCKS='0',GIT_NO_LAZY_FETCH='1')
 def blob(path):
  argv=['git','cat-file','blob',ctx['source_commit']+':'+path.relative_to(repo).as_posix()]
  result=subprocess.run(argv,cwd=repo,env=E,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
  if result.returncode!=0:
   for suffix,b in [('stdout',result.stdout),('stderr',result.stderr)]:
    with (h/('FAILED_PUBLIC_GIT16.'+suffix)).open('xb') as stream:stream.write(b)
   write_new(h/'FAILED_PUBLIC_GIT16.json',{'argv':argv,'exit':result.returncode,'cwd':str(repo),'original_full_stdout':'FAILED_PUBLIC_GIT16.stdout','original_full_stderr':'FAILED_PUBLIC_GIT16.stderr'})
   raise subprocess.CalledProcessError(result.returncode,argv,output=result.stdout,stderr=result.stderr)
  return result.stdout
 assert blob(lock)==lock.read_bytes()
 rows=source_lock()['files']
 for row in rows:
  src=study/row['path'];assert sha(src)==row['sha256'] and blob(src)==src.read_bytes(),row['path']
 primary_guard();(h/'source').mkdir()
 for row in rows:
  src=study/row['path'];dst=h/'source'/row['path'];dst.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(src,dst);dst.chmod(row['mode']);assert sha(dst)==row['sha256']
 shutil.copy2(lock,h/'source'/lock.name);copied_guard()
 before=originals();archive=repo/before['entire_prior_dependency_archive']['path'];assert sha(archive)==before['entire_prior_dependency_archive']['sha256'];(h/'deps').mkdir()
 with tarfile.open(archive,'r:xz') as tar:
  for row in before['exact_prebuilt_Rust1991_dependencies']:
   m=tar.getmember(row['member']);b=tar.extractfile(m).read();assert hashlib.sha256(b).hexdigest()==row['sha256'] and len(b)==row['bytes'];dst=h/'deps'/Path(row['member']).name;dst.write_bytes(b);dst.chmod(m.mode)
 dependency_guard();tools=[]
 for name in ['rustc','clippy-driver','rustfmt']:
  out=subprocess.run(['rustup','which','--toolchain','1.99.1',name],stdout=subprocess.PIPE,stderr=subprocess.PIPE,check=True);path=Path(out.stdout.decode().strip());tools.append({'tool':name,'path':str(path),'bytes':path.stat().st_size,'sha256':sha(path)})
 write_new(h/'COMPILER_TOOL_BINARY_MANIFEST16.json',tools)
 write_new(h/'BUILD_INPUT_MANIFEST16.json',{'recorded':now(),'RAM_free':ram,'scratch_free':scratch,'fixed_RAM_floor':8*1024*1024,'fixed_scratch_floor':3*1024*1024,'public_source_commit':ctx['source_commit'],'source_lock_sha256':sha(lock),'copied_source_files':len(rows),'originals':primary_guard(),'dependencies':dependency_guard(),'tools':tools,'scientific_call_has_not_occurred':True})
 return {'RAM_free':ram,'scratch_free':scratch,'exact_public_copied_source_files':len(rows),'original_files':len(before['original_source_files']),'full_prebuilt_dependencies':len(before['exact_prebuilt_Rust1991_dependencies'])}
def frontend():
 assert (h/'rustc-version.stdout').read_text().startswith('rustc 1.99.1 ')
 error=(h/'clippy-negative.stderr').read_text();assert 'clippy::eq-op' in error and 'equal expressions as operands' in error
 return {'Rust_version_and_canonical_negative_diagnostic_match':True}
def build_pre_call():
 primary_guard();copied_guard();dependency_guard();tool_guard();q=h/'immutable-qualifier';assert not q.exists();shutil.copy2(h/'compiler-output',q);q.chmod(0o555);assert sha(q)==sha(h/'compiler-output')
 value={'recorded_before_any_qualifier_call':now(),'public_source_commit':ctx['source_commit'],'qualifier':str(q),'bytes':q.stat().st_size,'sha256':sha(q),'mode':q.stat().st_mode&0o7777,'all_original_copied_dependencies_tools_equal':True,'BUILD_manifest_sha256':sha(h/'BUILD_INPUT_MANIFEST16.json')}
 write_new(h/'PRE_CALL_MANIFEST16.json',value);return value
def inventory():
 text=(h/'list-tests.stdout').read_text();target='nonsplit::map::tests_14::frozen_prime_cost_16: test';assert text.count(target)==1;assert len([x for x in text.splitlines() if x.endswith(': test')])==16
 return {'new_exact_filter_present_once':True,'tests_compiled':16,'old_tests_not_executed_by_listing':True}
def finish():
 failures=[];facts={}
 for name,guard in [('primary',primary_guard),('copied',copied_guard),('dependencies',dependency_guard),('tools',tool_guard)]:
  try:facts[name]=guard()
  except Exception as error:failures.append({'guard':name,'complete_error':repr(error)})
 pre=h/'PRE_CALL_MANIFEST16.json'
 if pre.exists():
  saved=json.loads(pre.read_text());q=Path(saved['qualifier']);same=q.exists() and sha(q)==saved['sha256'] and q.stat().st_mode&0o7777==saved['mode'];facts['qualifier_full_hash_and_mode_equal']=same
  if not same:failures.append({'guard':'immutable qualifier','complete_error':'hash/mode mismatch'})
 leftovers=[{'path':str(x.relative_to(temp)),'bytes':x.stat().st_size,'sha256':sha(x)} for x in sorted(temp.rglob('*')) if x.is_file()]
 write_new(h/'PROSPECTIVELY_EXCLUDED_COMPILER_TEMP16.json',leftovers)
 write_new(h/'POST_CALL_MANIFEST16.json',{'recorded_after_calls':now(),'source_commit':ctx['source_commit'],'guards':facts,'all_complete_guard_failures':failures,'pre_call_manifest_replaced':False})
 if failures:raise RuntimeError(failures)
 return facts
actions={'init':prepare,'frontend':frontend,'pre-call':build_pre_call,'test-inventory':inventory,'finish':finish}
try:detail=actions[a.action]();append(a.action,0,detail)
except Exception as error:
 append(a.action,1,{'complete_error':repr(error)});print(a.action,repr(error),file=sys.stderr);sys.exit(1)
if a.action=='finish' and load_state()['first_failure'] is not None:sys.exit(1)
