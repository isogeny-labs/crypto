#!/usr/bin/env python3
"""Prospective wrapper: not executed by its author. No publication/image actions."""
import argparse,fcntl,hashlib,json,os,re,shutil,signal,subprocess,time
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('--source-root',required=True);p.add_argument('--commit',required=True);p.add_argument('--seconds',required=True,type=int);p.add_argument('--receipt-dir',required=True);a=p.parse_args()
assert re.fullmatch('[0-9a-f]{40}',a.commit) and 1<=a.seconds<=3600
D=Path(a.receipt_dir);assert str(D).startswith('/var/tmp/');D.mkdir(exist_ok=False)
ORIGINAL=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029');R=Path(a.source_root);T=Path('/home/ec2-user/pencil-musl-target-b985-20261031');M=Path('/var/tmp/pencil-musl-private-20261031');B=T/'x86_64-unknown-linux-musl/release/ecbench'
sha=lambda q:hashlib.sha256(q.read_bytes()).hexdigest()
receipt={'schema':'software.bounded-musl-build/v1','source_commit_requested':a.commit,'seconds_cap':a.seconds,'start':time.time(),'binary_sha256':None,'image_qualified':False,'scientific_execution':False};proc=None

def save(): (D/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
def command(name,args,timeout=120):
 r=subprocess.run(args,cwd=R,capture_output=True,text=True,timeout=timeout,env={**os.environ,'GIT_OPTIONAL_LOCKS':'0'});(D/(name+'.stdout')).write_text(r.stdout);(D/(name+'.stderr')).write_text(r.stderr);receipt.setdefault('checks',[]).append({'name':name,'command':args,'exit_code':r.returncode});save();assert r.returncode==0,(name,r.returncode);return r.stdout

def ownership():
 out=[]
 for q in Path('/proc').iterdir():
  if not q.name.isdigit():continue
  try:
   comm=(q/'comm').read_text().strip()
   if comm not in ('cargo','rustc'):continue
   cwd=str((q/'cwd').resolve());ev=(q/'environ').read_bytes().split(b'\0');target=next((v.split(b'=',1)[1].decode() for v in ev if v.startswith(b'CARGO_TARGET_DIR=')),None)
   if cwd==str(R)or target==str(T):out.append({'pid':int(q.name),'comm':comm,'cwd':cwd,'target':target})
  except (OSError,ValueError):continue
 return out

def interrupted(signum,frame):raise InterruptedError('supervisor signal '+str(signum))
for sig in (signal.SIGINT,signal.SIGTERM):signal.signal(sig,interrupted)
try:
 assert R.is_absolute() and R==R.resolve() and R.parent==Path('/home/ec2-user') and R.name.startswith('pencil-musl-source-') and R!=ORIGINAL
 lock=(T/'.pencil-musl-owner.lock').open('a+');fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
 receipt['conflicting_owners']=ownership();assert not receipt['conflicting_owners']
 assert command('source-top-level',['git','rev-parse','--show-toplevel']).strip()==str(R)
 assert command('source-detached',['git','rev-parse','--abbrev-ref','HEAD']).strip()=='HEAD'
 receipt['source_root']=str(R)
 head=command('head-before',['git','rev-parse','HEAD']).strip();assert head==a.commit
 receipt['source_tree']=command('tree',['git','rev-parse','HEAD^{tree}']).strip()
 command('tracked-clean-before',['git','diff-index','--quiet','HEAD','--'])
 untracked=command('untracked-before',['git','ls-files','--others']);assert not untracked.strip(),'untracked inputs require source review'
 receipt['source_manifest']={'commit':head,'tree':receipt['source_tree'],'Cargo.toml':sha(R/'Cargo.toml'),'Cargo.lock':sha(R/'Cargo.lock')}
 receipt['toolchain']=command('rustc-version',['rustc','+1.99.1','-Vv'])
 receipt['compiler_inputs']={str(q):sha(q)for q in [M/'prefix/bin/musl-gcc',M/'prefix/lib/musl-gcc.specs',M/'prefix/lib/libc.a',M/'manifest.json',Path('/usr/bin/gcc')]}
 expected=json.loads((M/'status.json').read_text());assert receipt['compiler_inputs'][str(M/'prefix/bin/musl-gcc')]==expected['wrapper_sha256'];assert receipt['compiler_inputs'][str(M/'prefix/lib/musl-gcc.specs')]==expected['specs_sha256'];assert receipt['compiler_inputs'][str(M/'prefix/lib/libc.a')]==expected['libc_a_sha256']
 env={k:os.environ[k] for k in ['PATH','HOME','CARGO_HOME','RUSTUP_HOME','TMPDIR'] if k in os.environ};env['LC_ALL']='C'
 binding={'CARGO_TARGET_DIR':str(T),'CARGO_BUILD_JOBS':'2','RUSTFLAGS':'-C target-cpu=x86-64','REALGCC':'/usr/bin/gcc','CC_x86_64_unknown_linux_musl':str(M/'prefix/bin/musl-gcc'),'CRYPTO_BUILD_GIT_COMMIT':head,'CRYPTO_BUILD_GIT_DIRTY':'0'};env.update(binding)
 cmd=['timeout','-k','5s',str(a.seconds)+'s','cargo','+1.99.1','rustc','--locked','--offline','--release','--no-default-features','--target','x86_64-unknown-linux-musl','--bin','ecbench','-vv','--','--print','link-args','-Clink-arg=-Wl,-Map='+str(D/'ecbench.map')]
 receipt.update(command=cmd,build_environment=env,build_started=time.time());save()
 with (D/'build.stdout').open('w')as out,(D/'build.stderr').open('w')as err:
  proc=subprocess.Popen(cmd,cwd=R,env=env,stdout=out,stderr=err,start_new_session=True);receipt['supervised_process_group']=proc.pid;save();receipt['build_exit_code']=proc.wait(timeout=a.seconds+15)
 receipt['build_ended']=time.time();save();assert receipt['build_exit_code']==0
 assert command('head-after',['git','rev-parse','HEAD']).strip()==head
 command('tracked-clean-after',['git','diff-index','--quiet','HEAD','--'])
 assert not command('untracked-after',['git','ls-files','--others']).strip()
 assert B.is_file() and (D/'ecbench.map').is_file();shutil.copy2(B,D/'ecbench');receipt['binary_sha256']=sha(D/'ecbench')
 h=command('elf-header',['readelf','-h',str(D/'ecbench')]);ph=command('elf-programs',['readelf','-l',str(D/'ecbench')]);dyn=command('elf-dynamic',['readelf','-d',str(D/'ecbench')]);assert 'Advanced Micro Devices X86-64'in h and 'INTERP'not in ph and 'NEEDED'not in dyn
 text=(D/'ecbench.map').read_text(errors='replace')+(D/'build.stdout').read_text(errors='replace');paths=sorted(set(re.findall(r'/[^\s\"\(\)]+\.(?:a|o)',text)));selected={v:sha(Path(v))for v in paths if Path(v).is_file() and (Path(v).name=='libc.a' or 'crt'in Path(v).name)};(D/'crt-libc-candidates.json').write_text(json.dumps(selected,indent=2)+'\n')
 receipt['static_elf_checked']=True;receipt['link_map_sha256']=sha(D/'ecbench.map');receipt['crt_libc_binding_status']='candidate paths extracted; independent map/invocation review required';receipt['status']='built_static_candidate'
except BaseException as e:
 receipt['status']='failed_or_interrupted';receipt['error']=repr(e)
finally:
 if proc is not None and proc.poll()is None:
  os.killpg(proc.pid,signal.SIGTERM)
  try:proc.wait(timeout=5)
  except subprocess.TimeoutExpired:os.killpg(proc.pid,signal.SIGKILL);proc.wait()
 receipt['end']=time.time();save()
raise SystemExit(0 if receipt['status']=='built_static_candidate'else 1)
