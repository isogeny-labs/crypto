#!/usr/bin/env python3
import datetime, hashlib, json, os, re, resource, shutil, signal, stat, subprocess, sys, time, traceback
from pathlib import Path
P=Path(__file__).resolve().parent
from durable_io import atomic_json, flush_logs, load_admission, durable_limits
F=load_admission(P)
W=Path(F['workspace']); OUT=Path(F['durable_run'])/'worker'; SOURCE=W/'source'; STAGE=W/'cache-stage-v2'
START=time.monotonic(); END=float(os.environ['PENCIL_CHECK_END_MONOTONIC']); ACTIVE=None; INTERRUPTED=False;OWNED_OUTPUT=False;PERMIT_REQUIRED=False
R={'status':'failed','commands':[],'errors':[],'invocation_id':os.environ.get('INVOCATION_ID'),'source_scope':'root default-feature library tests, private candidate patch'}
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
def check_permit():
 permit=json.loads((Path(F['durable_run'])/'supervisor-permit.json').read_text())
 assert permit['invocation_id']==R['invocation_id'] and permit['lease']==F['lease'],'supervisor permit identity'
 assert time.time()<permit['expires_unix'],'supervisor permit expired'
def guard():
 if PERMIT_REQUIRED:check_permit()
 if OWNED_OUTPUT:durable_limits(Path(F['durable_run']),F['limits'])
 if INTERRUPTED or time.monotonic()>=END-40:raise RuntimeError('work deadline/signal')
 if datetime.datetime.now(datetime.timezone.utc)>=datetime.datetime.fromisoformat(F['absolute_deadline']):raise RuntimeError('absolute deadline')
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
ENV={'PATH':F['toolchain']+'/bin:/usr/bin:/bin','LANG':'C.UTF-8','LC_ALL':'C.UTF-8','CARGO_HOME':str(STAGE/'cargo-home'),'CARGO_TARGET_DIR':str(STAGE/'target'),'CARGO_BUILD_JOBS':'1','RUSTC':F['toolchain']+'/bin/rustc','TMPDIR':str(STAGE/'tmp'),'RUST_TEST_THREADS':'1','RUST_BACKTRACE':'0'}
def command(label,argv):
 global ACTIVE
 guard(); row={'label':label,'argv':argv}; R['commands'].append(row); start=time.monotonic()
 stdout=OUT/(label+'.stdout');stderr=OUT/(label+'.stderr');checkpoint('before-'+label)
 with stdout.open('xb') as o,stderr.open('xb') as e:
  mask=signal.pthread_sigmask(signal.SIG_BLOCK,{signal.SIGTERM,signal.SIGINT})
  try:ACTIVE=subprocess.Popen(argv,cwd=SOURCE,env=ENV,stdout=o,stderr=e,start_new_session=True)
  finally:signal.pthread_sigmask(signal.SIG_SETMASK,mask)
  row['pid']=ACTIVE.pid;row['pgid']=ACTIVE.pid;last=0;durable_last=0;checkpoint('running-'+label)
  try:
   while ACTIVE.poll() is None:
    guard()
    if time.monotonic()-durable_last>5:
     durable_last=time.monotonic();checkpoint('running-'+label)
    if time.monotonic()-last>1:
     last=time.monotonic(); total=0
     for base,dirs,names in os.walk(STAGE/'target'):
      for name in names:
       f=Path(base)/name
       try:st=f.lstat()
       except FileNotFoundError:continue
       assert stat.S_ISREG(st.st_mode),str(f)
       total+=st.st_size
     row['peak_observed_target_bytes']=max(row.get('peak_observed_target_bytes',0),total)
     assert total<=6*1024**3,'6GiB target threshold'
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
 for path in [SOURCE,STAGE/'vendor',STAGE/'target',STAGE/'cargo-home',STAGE/'tmp']:assert path.is_dir(),str(path)
 R['within_unit_input_visibility']=True;checkpoint('waiting-for-supervisor-permit')
 wait_end=min(END-40,time.monotonic()+10)
 while True:
  try:check_permit();break
  except (FileNotFoundError,AssertionError):
   if time.monotonic()>=wait_end:raise RuntimeError('supervisor ownership permit not received')
   time.sleep(.1)
 PERMIT_REQUIRED=True;checkpoint('containment-and-permit-verified')
 inputs=json.loads((P/'materialized-inputs.json').read_text());assert H(P/'materialized-inputs.json')==F['inputs_sha256']
 for name,h in F['recipe_files'].items():assert H(P/name)==h,name
 verify(inputs['immutable']);verify(inputs['initial_cache']);R['pre_inputs_verified']=True;checkpoint('inputs-verified')
 cargo=F['toolchain']+'/bin/cargo';base=[cargo,'test','--release','--lib','--locked','--offline']
 output=command('compile',base+['--no-run','--message-format=json'])
 binaries=[]
 for line in output.splitlines():
  try:entry=json.loads(line)
  except json.JSONDecodeError:continue
  if entry.get('reason')=='compiler-artifact' and entry.get('target',{}).get('name')=='crypto_lib' and entry.get('profile',{}).get('test') and entry.get('executable'):binaries.append(entry['executable'])
 assert len(set(binaries))==1,'exactly one root library test executable required'
 binary=Path(binaries[0]);assert binary.is_relative_to(STAGE/'target') and binary.is_file()
 R['test_binary']={'path':str(binary),'sha256':H(binary)}
 listing=command('list',[str(binary),'--list']);names=[line[:-6] for line in listing.splitlines() if line.endswith(': test')]
 assert len(names)>0 and len(names)==len(set(names))
 R['test_inventory']=names
 for i,name in enumerate(F['exact_tests']):
  assert names.count(name)==1,name
  out=command('exact-'+str(i),[str(binary),name,'--exact','--test-threads=1','--nocapture'])
  assert len(re.findall(r'test result: ok\. 1 passed; 0 failed;',out))==1,'nonzero exact test result'
  if i==0:assert out.count('ROOT_ADAPTER_POSITIVE_CONTROL recovered=31124 actual_routes=2 deterministic_ledgers_equal=true')==1,'positive control sentinel'
 full=command('full-suite',base+['--','--test-threads=1'])
 summaries=re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out;',full)
 assert len(summaries)==1,'exactly one unfiltered library result'
 passed,failed,ignored,measured,filtered=map(int,summaries[0]);assert passed>0 and failed==filtered==0 and passed+ignored+measured==len(names)
 R['full_suite']={'passed':passed,'failed':failed,'ignored':ignored,'measured':measured,'filtered':filtered}
 assert H(binary)==R['test_binary']['sha256'],'binary changed'
 verify(inputs['immutable']);R['post_inputs_verified']=True
 guard();R['status']='passed'
except BaseException as e:R['errors'].append({'repr':repr(e),'traceback':traceback.format_exc()})
finally:
 if ACTIVE is not None:
  try:
   os.killpg(ACTIVE.pid,signal.SIGKILL);ACTIVE.wait(timeout=3)
  except BaseException as e:R['errors'].append({'cleanup':repr(e)})
  R['status']='failed'
 R['elapsed_seconds']=time.monotonic()-START
 if INTERRUPTED or time.monotonic()>=END-20 or R['errors']:R['status']='failed'
 if OWNED_OUTPUT:checkpoint('terminal')
print(json.dumps({'status':R['status'],'elapsed_seconds':R['elapsed_seconds']}))
raise SystemExit(0 if R['status']=='passed' and not INTERRUPTED and time.monotonic()<END-20 else 1)
