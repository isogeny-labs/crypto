#!/usr/bin/env python3
import datetime, hashlib, json, math, os, re, resource, shutil, signal, stat, subprocess, sys, time, traceback
from pathlib import Path
P=Path(__file__).resolve().parent
from durable_io import atomic_json, flush_logs, load_admission, durable_limits
from panel_acceptance import one_test,verify_driver,verify_probe,NEGATIVES
F=load_admission(P)
W=Path(F['workspace']); OUT=Path(F['durable_run'])/'worker'; SOURCE=W/'source'; STAGE=W/'cache'
START=time.monotonic(); END=float(os.environ['PENCIL_CHECK_END_MONOTONIC']); ACTIVE=None; INTERRUPTED=False;OWNED_OUTPUT=False;PERMIT_REQUIRED=False;PHASE_END=END-600;BUILD_END=END-600
R={'status':'failed','commands':[],'errors':[],'invocation_id':os.environ.get('INVOCATION_ID'),'source_scope':'fixed28 signed +7 Labels; at most11 full739 driver calls,2 forced native probes,4 adapter negatives'}
resource.setrlimit(resource.RLIMIT_AS,(16*1024**3,16*1024**3))
def checkpoint(phase):
 R['phase']=phase;R['elapsed_seconds']=time.monotonic()-START
 if OWNED_OUTPUT:
  atomic_json(OUT/'worker-receipt.json',R);flush_logs(OUT)
def interrupted(sig,frame):
 global INTERRUPTED
 INTERRUPTED=True
 raise RuntimeError('signal '+str(sig))
for sig in (signal.SIGTERM,signal.SIGINT):signal.signal(sig,interrupted)
def retain_permit_anomaly(snapshots, error=None):
 row={'snapshots':snapshots}
 if error is not None:row['error']=repr(error)
 R.setdefault('permit_anomalies',[]).append(row)
 R['permit_anomalies']=R['permit_anomalies'][-32:]
def read_permit_snapshot():
 meta={'read_start_unix':time.time(),'read_start_monotonic':time.monotonic()}
 try:
  with (Path(F['durable_run'])/'supervisor-permit.json').open('rb') as stream:raw=stream.read(4097)
  meta.update(bytes=len(raw),sha256=hashlib.sha256(raw).hexdigest())
  assert len(raw)<=4096,'supervisor permit size'
  permit=json.loads(raw)
  meta['value']=permit
  assert isinstance(permit,dict),'supervisor permit object'
  assert permit['invocation_id']==R['invocation_id'] and permit['lease']==F['lease'],'supervisor permit identity'
  expiry=permit['expires_unix']
  assert isinstance(expiry,(int,float)) and not isinstance(expiry,bool) and math.isfinite(expiry),'supervisor permit expiry format'
  meta.update(read_end_unix=time.time(),read_end_monotonic=time.monotonic())
  return permit,meta
 except BaseException as error:
  meta.update(read_end_unix=time.time(),read_end_monotonic=time.monotonic())
  retain_permit_anomaly([meta],error)
  raise
def check_permit():
 first,one=read_permit_snapshot()
 if time.time()<first['expires_unix']:return
 # No grace period: only a newly read, identity-matched, unexpired permit can pass.
 retry_start=time.monotonic()
 try:
  second,two=read_permit_snapshot()
  assert time.monotonic()-retry_start<=0.25,'supervisor permit reread bound'
  assert not INTERRUPTED and time.monotonic()<END-40,'supervisor permit deadline/signal'
  assert time.time()<second['expires_unix'],'supervisor permit expired'
 except BaseException as error:
  retain_permit_anomaly([one]+([two] if 'two' in locals() else []),error)
  raise
 retain_permit_anomaly([one,two])
def guard():
 if PERMIT_REQUIRED:check_permit()
 if OWNED_OUTPUT:durable_limits(Path(F['durable_run']),F['limits'])
 if INTERRUPTED or time.monotonic()>=min(END-40,PHASE_END):raise RuntimeError('phase/work deadline/signal')
 if datetime.datetime.now(datetime.timezone.utc)>=datetime.datetime.fromisoformat(F['absolute_deadline'].replace('Z','+00:00')):raise RuntimeError('absolute deadline')
 if shutil.disk_usage(W).free<6*1024**3:raise RuntimeError('free disk below6GiB')
def H(path):
 guard(); h=hashlib.sha256()
 with path.open('rb') as s:
  for b in iter(lambda:s.read(4*1024**2),b''):guard();h.update(b)
 return h.hexdigest()
def verify(rows):
 for path,row in rows.items():
  f=Path(path)
  if 'resolved' in row:assert str(f.resolve())==row['resolved'],path
  else:assert stat.S_ISREG(f.lstat().st_mode),path
  assert H(f)==row['sha256'],path
def traversal_error(error):
 raise error

ENV={'PATH':F['toolchain']+'/bin:/usr/bin:/bin','LANG':'C.UTF-8','LC_ALL':'C.UTF-8','CARGO_HOME':str(STAGE/'cargo-home'),'CARGO_TARGET_DIR':str(STAGE/'target'),'CARGO_BUILD_JOBS':'1','RUSTC':F['toolchain']+'/bin/rustc','TMPDIR':str(STAGE/'tmp'),'RUST_TEST_THREADS':'1','RUST_BACKTRACE':'0','SIGNED_BUILD_END_MONOTONIC':str(BUILD_END),'QUAL_SOURCE_REVISION':'29861c9a55f9ae5a33e1e09b3866aa397fbde605','QUAL_SOURCE_SHA256':F['source_manifest_sha256']}
def command(label,argv,cwd=None):
 global ACTIVE
 guard(); row={'label':label,'argv':argv}; R['commands'].append(row); start=time.monotonic()
 stdout=OUT/(label+'.stdout');stderr=OUT/(label+'.stderr');checkpoint('before-'+label)
 with stdout.open('xb') as o,stderr.open('xb') as e:
  mask=signal.pthread_sigmask(signal.SIG_BLOCK,{signal.SIGTERM,signal.SIGINT})
  try:ACTIVE=subprocess.Popen(argv,cwd=SOURCE if cwd is None else cwd,env=ENV,stdout=o,stderr=e,start_new_session=True)
  finally:signal.pthread_sigmask(signal.SIG_SETMASK,mask)
  row['pid']=ACTIVE.pid;row['pgid']=ACTIVE.pid;last=0;durable_last=0;checkpoint('running-'+label)
  try:
   while ACTIVE.poll() is None:
    guard()
    if time.monotonic()-durable_last>5:
     durable_last=time.monotonic();checkpoint('running-'+label)
    if time.monotonic()-last>1:
     last=time.monotonic(); total=0;target=STAGE/'target'
     try:target_mode=target.lstat().st_mode
     except FileNotFoundError:
      assert label=='materialize','target absent after materialization';target_mode=None
     if target_mode is not None:
      assert stat.S_ISDIR(target_mode),str(target)
      for base,dirs,names in os.walk(target,onerror=traversal_error):
       for name in names:
        f=Path(base)/name
        try:st=f.lstat()
        except FileNotFoundError:continue
        assert stat.S_ISREG(st.st_mode),str(f)
        total+=st.st_size
     row['peak_observed_target_bytes']=max(row.get('peak_observed_target_bytes',0),total)
     assert total<=6*1024**3,'6GiB target threshold'
     workspace_bytes=0
     for base,dirs,names in os.walk(W,followlinks=False,onerror=traversal_error):
      for n in dirs:assert stat.S_ISDIR((Path(base)/n).lstat().st_mode)
      for n in names:
       f=Path(base)/n
       try:st=f.lstat()
       except FileNotFoundError:continue
       assert stat.S_ISREG(st.st_mode),str(f)
       workspace_bytes+=st.st_size
     assert workspace_bytes<=8*1024**3,'private workspace8GiB ceiling'
     logbytes=sum(f.stat().st_size for f in OUT.glob('*.std*'))
     row['peak_observed_log_bytes']=max(row.get('peak_observed_log_bytes',0),logbytes)
     assert logbytes<=256*1024**2,'256MiB log threshold'
    time.sleep(.1)
   row['exit_code']=ACTIVE.returncode
  finally:
   if ACTIVE.poll() is None:os.killpg(ACTIVE.pid,signal.SIGKILL)
   row['reaped_exit_code']=ACTIVE.wait(timeout=3);row['reaped']=True
   # Kill remaining children even after leader exit; unit containment remains final authority.
   try:os.killpg(ACTIVE.pid,signal.SIGKILL)
   except ProcessLookupError:pass
   ACTIVE=None;row['elapsed_seconds']=time.monotonic()-start;checkpoint('finished-'+label)
 assert row['exit_code']==0,label+' failed'
 guard();row['stdout_sha256']=H(stdout);row['stderr_sha256']=H(stderr)
 return stdout.read_text(errors='replace')

def verify_private_source():
 expected=json.loads((P/'expected-source-manifest.json').read_text());actual=set()
 for base,dirs,names in os.walk(SOURCE,followlinks=False,onerror=traversal_error):
  guard()
  for n in dirs:assert stat.S_ISDIR((Path(base)/n).lstat().st_mode)
  for n in names:
   f=Path(base)/n;name=str(f.relative_to(SOURCE));assert name in expected,name
   assert stat.S_ISREG(f.lstat().st_mode) and f.stat().st_size==expected[name]['bytes']
   assert H(f)==expected[name]['sha256'],name;actual.add(name)
 assert actual==set(expected),'private source membership'
def verify_driver_source():
 rows=json.loads((W/'driver-source-manifest.json').read_text());root=W/'driver-source';actual=set()
 for base,dirs,names in os.walk(root,followlinks=False,onerror=traversal_error):
  guard()
  for n in dirs:assert stat.S_ISDIR((Path(base)/n).lstat().st_mode)
  for n in names:
   path=Path(base)/n;rel=str(path.relative_to(root));assert rel in rows
   assert stat.S_ISREG(path.lstat().st_mode) and path.stat().st_size==rows[rel]['bytes']
   assert H(path)==rows[rel]['sha256'];actual.add(rel)
 assert actual==set(rows)
def select_binary(output,target_name,target_kind,source_path):
 binaries=[];finished=[]
 for line in output.splitlines():
  try:entry=json.loads(line)
  except json.JSONDecodeError:continue
  if entry.get('reason')=='build-finished':finished.append(entry.get('success'))
  t=entry.get('target',{})
  if entry.get('reason')=='compiler-artifact' and t.get('name')==target_name and target_kind in t.get('kind',[]) and t.get('src_path')==str(source_path) and entry.get('profile',{}).get('test') and entry.get('executable'):binaries.append(entry['executable'])
 assert finished==[True] and len(set(binaries))==1,'exact build artifact'
 path=Path(binaries[0]);assert path.is_absolute() and path.is_relative_to(STAGE/'target') and stat.S_ISREG(path.lstat().st_mode)
 return {'path':str(path),'sha256':H(path)}

def json_rows(output):
 rows=[]
 for line in output.splitlines():
  if '{' not in line:continue
  try:row=json.loads(line[line.index('{'):])
  except json.JSONDecodeError:continue
  if isinstance(row,dict):rows.append(row)
 return rows
try:
 OUT.mkdir(mode=0o700,exist_ok=False);OWNED_OUTPUT=True;checkpoint('worker-start')
 assert os.getuid()==F['uid'] and os.getgid()==F['gid']
 assert R['invocation_id'] and re.fullmatch('[a-f0-9]{32}',R['invocation_id'])
 cgline=Path('/proc/self/cgroup').read_text().strip();assert cgline.startswith('0::/') and '\n' not in cgline
 cg=Path('/sys/fs/cgroup')/cgline[3:].lstrip('/');assert cg.name==F['unit']
 controls={n:(cg/n).read_text().strip() for n in ['cpu.max','memory.max','memory.swap.max','pids.max']}
 quota,period=controls['cpu.max'].split();assert quota!='max' and int(quota)<=2*int(period)
 assert controls['memory.max']==str(16*1024**3) and controls['memory.swap.max']=='0' and controls['pids.max']=='512'
 R['cgroup']={'path':str(cg),'inode':cg.stat().st_ino,'controls':controls}
 assert not SOURCE.exists() and not STAGE.exists(),'fresh source/cache required'
 checkpoint('waiting-for-supervisor-permit');wait_end=min(END-40,time.monotonic()+10)
 while True:
  try:check_permit();break
  except (FileNotFoundError,AssertionError):
   if time.monotonic()>=wait_end:raise RuntimeError('supervisor ownership permit not received')
   time.sleep(.1)
 PERMIT_REQUIRED=True;checkpoint('containment-and-permit-verified')
 R['build_deadline_monotonic']=BUILD_END
 for name,h in F['packet_files'].items():assert H(P/name)==h,name
 command('materialize',['/usr/bin/python3','-B',str(P/'materialize.py')],cwd=P)
 materialized=json.loads((W/'materialization.json').read_text());assert materialized['status']=='passed' and materialized['build_end_monotonic']==BUILD_END
 assert materialized['cache_mode_inventory_sha256']==H(P/'cache-mode-inventory.json')
 assert materialized['cache_mode_files']==2165 and materialized['cache_executable_files']==82
 assert materialized['cache_modes_preserved'] is True
 R['materialization_receipt_sha256']=H(W/'materialization.json');verify_private_source();R['pre_inputs_verified']=True
 cargo=F['toolchain']+'/bin/cargo'
 output=command('compile',[cargo,'test','--release','--lib','--locked','--offline','--no-run','--message-format=json'])
 binaries=[];finished=[]
 for line in output.splitlines():
  try:entry=json.loads(line)
  except json.JSONDecodeError:continue
  if entry.get('reason')=='build-finished':finished.append(entry.get('success'))
  target=entry.get('target',{})
  if entry.get('reason')=='compiler-artifact' and target.get('name')=='crypto_lib' and 'lib' in target.get('kind',[]) and target.get('src_path')==str(SOURCE/'src/lib.rs') and entry.get('profile',{}).get('test') and entry.get('executable'):binaries.append(entry['executable'])
 assert finished==[True],'one successful Cargo build-finished required'
 assert len(set(binaries))==1,'exactly one root library test executable required'
 binary=Path(binaries[0]);assert binary.is_absolute() and binary.is_relative_to(STAGE/'target') and stat.S_ISREG(binary.lstat().st_mode)
 R['test_binary']={'path':str(binary),'sha256':H(binary)};guard()
 command('stage-driver',['/usr/bin/python3','-B',str(P/'stage_driver.py')],cwd=P)
 verify_driver_source()
 driver_root=W/'driver-source';driver_spec=json.loads((P/'driver-inputs.json').read_text())
 driver_crate=driver_root/driver_spec['crate'];driver_manifest=driver_crate/'Cargo.toml'
 driver_source=driver_crate/'src/pruned.rs'
 ENV.update(KUMMER_TARGET_CAP='1',KUMMER_START_INDEX='0',KUMMER_VALIDATE_TRANSITIONS='0',KUMMER_DRIVER_SOURCE_SHA256=H(driver_source),KUMMER_DRIVER_REVISION=F['published_source_revision'],KUMMER_DRIVER_CONTROL_OUTPUT=str(OUT/'driver-panel'))
 driver_output=command('compile-driver',[cargo,'test','--manifest-path',str(driver_manifest),'--release','--bin','equivalence','--locked','--offline','--no-run','--message-format=json'],cwd=driver_root)
 driver_binary=select_binary(driver_output,'equivalence','bin',driver_crate/'src/check.rs');R['driver_binary']=driver_binary
 adapter_output=command('compile-adapter',[cargo,'test','--manifest-path',str(driver_root/'research/pencil_large_prime_20261029/Cargo.toml'),'--features','reporting','--release','--lib','--locked','--offline','--no-run','--message-format=json'],cwd=driver_root)
 adapter_binary=select_binary(adapter_output,'pencil_symbolic_graph','lib',driver_root/'research/pencil_large_prime_20261029/src/lib.rs');R['adapter_binary']=adapter_binary
 verify_driver_source()
 R['build_phase_finished_monotonic']=time.monotonic()
 END=min(END,time.monotonic()+600);PHASE_END=END-40
 R['execution_end_monotonic']=END;R['execution_deadline_monotonic']=PHASE_END;checkpoint('execution-phase-start')
 panel=json.loads((SOURCE/'research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/signed-producer-qualification-20261109/signed-panel.json').read_text())
 for i,name in enumerate(F['exact_tests']):
  assert H(binary)==R['test_binary']['sha256'],'binary changed before test'
  out=command('exact-'+str(i),[str(binary),name,'--exact','--test-threads=1','--nocapture'])
  assert len(re.findall(r'test result: ok\. 1 passed; 0 failed; 0 ignored; 0 measured;',out))==1,'one nonzero exact passing result required'
  rows=json_rows(out)
  if i==0:
   cases=[x for x in rows if isinstance(x.get('case'),dict)];summaries=[x for x in rows if 'completed' in x]
   assert [x['case'] for x in cases]==panel and all(x.get('pass') is True for x in cases),'exact28 fixed signed rows'
   assert len(summaries)==1 and summaries[0].get('pass') is True and summaries[0]['completed']==28,'final completed28 after fault reruns required'
   for key,flag in [('identity_cases','identity'),('duplicate_cases','duplicate')]:
    count=sum(x[flag] is True for x in cases);assert count>0 and summaries[0][key]==count,'fixed native branch coverage'
   R['signed_summary']=summaries[0]
  else:
   names=['informative_same','informative_opposite','redundant_same','redundant_opposite','invalid_zero_target','invalid_wrong_candidate','noncanonical']
   cases=[x for x in rows if isinstance(x.get('case'),str)]
   assert [x['case'] for x in cases]==names and all(x.get('pass') is True for x in cases),'exact7label rows'
   R['kummer_cases']=cases
 for index,name in enumerate(['pruned::driver_controls::actual_driver_original_disabled_observed_and_fixed_faults','pruned::driver_controls::actual_forced_fallback_and_zero_vector_lift_are_recorded']):
  assert H(Path(driver_binary['path']))==driver_binary['sha256']
  output=command('driver-exact-'+str(index),[driver_binary['path'],name,'--exact','--test-threads=1','--nocapture'],cwd=driver_root);one_test(output)
  fixture=json.loads((driver_crate/'fixture.json').read_text())
  R['driver_summary' if index==0 else 'native_probe_summary']=verify_driver(OUT/'driver-panel',fixture,F['published_source_revision'],H(driver_source)) if index==0 else verify_probe(OUT/'driver-panel',fixture,F['published_source_revision'],H(driver_source))
 R['adapter_negatives']=[]
 for index,suffix in enumerate(NEGATIVES):
  assert H(Path(adapter_binary['path']))==adapter_binary['sha256']
  name='relation_stream::kummer_driver_protocol_controls::'+suffix
  output=command('adapter-negative-'+str(index),[adapter_binary['path'],name,'--exact','--test-threads=1','--nocapture'],cwd=driver_root);one_test(output)
  R['adapter_negatives'].append({'name':name,'passed':True})
 assert len(R['adapter_negatives'])==4
 assert H(Path(driver_binary['path']))==driver_binary['sha256'] and H(Path(adapter_binary['path']))==adapter_binary['sha256']
 verify_driver_source()
 assert H(binary)==R['test_binary']['sha256'],'binary changed after tests'
 verify_private_source()
 manifest=Path(F['immutable_manifest']['path']);assert H(manifest)==F['immutable_manifest']['sha256'];inputs=json.loads(manifest.read_text());verify(inputs['immutable'])
 config_key=F['donor_workspace']+'/cache-stage-v2/cargo-home/config.toml';assert H(STAGE/'cargo-home/config.toml')==inputs['immutable'][config_key]['sha256']
 R['post_inputs_verified']=True;guard();R['status']='passed'
except BaseException as e:R['errors'].append({'repr':repr(e),'traceback':traceback.format_exc()})
finally:
 if ACTIVE is not None:
  try:os.killpg(ACTIVE.pid,signal.SIGKILL);ACTIVE.wait(timeout=3)
  except BaseException as e:R['errors'].append({'cleanup':repr(e)})
  R['status']='failed'
 R['elapsed_seconds']=time.monotonic()-START
 if INTERRUPTED or time.monotonic()>=END-20 or R['errors']:R['status']='failed'
 if OWNED_OUTPUT:checkpoint('terminal')
print(json.dumps({'status':R['status'],'elapsed_seconds':R['elapsed_seconds']}))
raise SystemExit(0 if R['status']=='passed' and not INTERRUPTED and time.monotonic()<END-20 else 1)
