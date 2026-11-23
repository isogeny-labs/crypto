import hashlib,json,os,subprocess,sys,tempfile,time,resource
from pathlib import Path
D=Path(__file__).resolve().parent;name=sys.argv[1];assert name.isidentifier()
resource.setrlimit(resource.RLIMIT_AS,(8192*1024**2,8192*1024**2))
assert len(list(D.glob('*-attempt-*')))<12,'software invocation ceiling'
out=Path(tempfile.mkdtemp(prefix=name+'-attempt-',dir=D));src=D/(name+'.lean');(out/src.name).write_bytes(src.read_bytes())
roots=[D,Path('/var/tmp/pencil-kummer-row-fold-20261101-v1/snapshot'),Path('/var/tmp/pencil-comparator-word-20261101/snapshot'),Path('/var/tmp/independent-product-review-20261101/snapshot'),Path('/var/tmp/pencil-mumford-validity-20261031-v1/dependency-union'),Path('/var/tmp/pencil-sameu-global-smooth-20261031-v1/dependency-union')]
lean=Path('/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain/bin/lean')
files={lean,D/'runner.py',D/'check.sh',D/'PLAN.md',D/'PLAN-amendment.md'}
for root in roots[1:]:files.update(p for p in root.rglob('*') if p.is_file())
files.update(p for p in D.iterdir() if p.is_file() and p.suffix in ('.lean','.olean') and p!=D/(name+'.olean'))
def digest(p):return hashlib.file_digest(p.open('rb'),'sha256').hexdigest() if hasattr(hashlib,'file_digest') else hashlib.sha256(p.read_bytes()).hexdigest()
before={str(p):digest(p) for p in sorted(files)};(out/'inputs.json').write_text(json.dumps(before,indent=2)+'\n')
env=dict(os.environ,LEAN_SYSROOT=str(lean.parent.parent),LEAN_PATH=':'.join(map(str,roots)))
cmd=[str(lean),'-j1','-M3500','-s16384','-o',str(D/(name+'.olean')),str(src)]
with (out/'stdout').open('wb') as so,(out/'stderr').open('wb') as se:
 try:code=subprocess.run(cmd,env=env,cwd=D,stdout=so,stderr=se,timeout=90).returncode
 except subprocess.TimeoutExpired:code=124
changed=[s for s,h in before.items() if not Path(s).exists() or digest(Path(s))!=h]
(out/'receipt.json').write_text(json.dumps({'module':name,'command':cmd,'native_exit':code,'inputs':len(before),'changed_inputs':changed,'gate_exit':126 if changed else code,'outer_wrapper_seconds':120,'kill_reserve_seconds':3,'address_space_mib':8192,'lean_heap_mib':3500,'thread_stack_kib':16384,'threads':1},indent=2)+'\n')
print((out/'stdout').read_text());print((out/'stderr').read_text());print(out);sys.exit(126 if changed else code)
