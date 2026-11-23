#!/usr/bin/env python3
"""Prospective, one-shot offline resolution only; no compiler, payload or tests."""
import hashlib,json,os,pathlib,subprocess,sys,time,datetime
P=pathlib.Path(__file__).resolve().parent
W=pathlib.Path('/var/tmp/pencil-kummer-lock-20261109')
start=time.monotonic(); end=start+45
sha=lambda b:hashlib.sha256(b).hexdigest()
def check():
 if time.monotonic()>=end:raise TimeoutError('55-second worker deadline')
def verified(path,digest):
 check();p=pathlib.Path(path)
 if p.is_symlink():raise ValueError('symlink input')
 b=p.read_bytes()
 if sha(b)!=digest:raise ValueError('input hash mismatch: '+str(p))
 return b
release=json.loads((P/'release.json').read_text())
assert release['released'] is True and release['publication_commit'] and release['publication_readback_sha256']
assert release['reserved_worker_seconds']==60 and release['serial_owner_confirmed'] is True
assert datetime.datetime.now(datetime.timezone.utc)<datetime.datetime.fromisoformat('2026-11-10T05:11:39+00:00')-datetime.timedelta(seconds=60)
inputs=json.loads(verified(P/'lock-inputs.json',release['inputs_sha256']))
verified(__file__,release['worker_sha256'])
assert json.loads((W/'caller-owner.json').read_text())['unit']=='pencil-kummer-lock-20261109.service'
assert set(p.name for p in W.iterdir()) <= {'caller-owner.json','unit.stdout','unit.stderr','launch.stdout','launch.stderr','owner.json'}
result={'status':'failed','compile_invoked':False,'network_allowed':False,'lock':None}
try:
 immutable=json.loads(verified(inputs['immutable_manifest'],inputs['immutable_manifest_sha256']))['immutable']
 vendor=pathlib.Path(inputs['registry_vendor']); expected={k:v for k,v in immutable.items() if k.startswith(str(vendor)+'/')}
 def walkerror(e):raise e
 actual=set()
 for root,dirs,files in os.walk(vendor,followlinks=False,onerror=walkerror):
  for n in dirs:
   if (pathlib.Path(root)/n).is_symlink():raise ValueError('vendor directory symlink')
  for n in files:actual.add(str(pathlib.Path(root)/n))
 assert actual==set(expected)
 for p,r in expected.items():verified(p,r['sha256'])
 cargo='/home/ec2-user/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/cargo'
 verified(cargo,immutable[cargo]['sha256'])
 for row in inputs['files']:
  dst=W/'source'/row['destination'];dst.parent.mkdir(parents=True,exist_ok=True)
  dst.write_bytes(verified(row['source'],row['sha256']))
 home=W/'cargo-home';home.mkdir();(home/'config.toml').write_text('[source.crates-io]\nreplace-with="vendored-sources"\n[source.vendored-sources]\ndirectory='+json.dumps(str(vendor))+'\n[net]\noffline=true\n')
 env={'PATH':'/usr/bin:/bin','CARGO_HOME':str(home),'CARGO_NET_OFFLINE':'true','RUSTC':'/bin/false','RUSTDOC':'/bin/false','HOME':str(W),'LANG':'C.UTF-8'}
 argv=[cargo,'generate-lockfile','--offline','--manifest-path',str(W/'source'/inputs['crate']/'Cargo.toml')]
 result['argv']=argv
 with (W/'stdout').open('xb') as out,(W/'stderr').open('xb') as err:
  run=subprocess.run(argv,env=env,cwd=W,stdout=out,stderr=err,timeout=max(.01,end-time.monotonic()),check=False)
 result['exit_code']=run.returncode
 lock=W/'source'/inputs['crate']/'Cargo.lock'
 if lock.exists():
  b=lock.read_bytes();(W/'Cargo.lock').write_bytes(b);result['lock']={'sha256':sha(b),'bytes':len(b),'path':str(W/'Cargo.lock')}
 if run.returncode!=0:raise RuntimeError('offline resolution failed; retained stderr')
 assert result['lock'];check();result['status']='generated_pending_publication'
except BaseException as e:
 result['error']=type(e).__name__+': '+str(e)
finally:
 result['elapsed_seconds']=time.monotonic()-start
 (W/'result.json').write_text(json.dumps(result,indent=2)+'\n')
 if result['status']=='failed':sys.exit(1)
