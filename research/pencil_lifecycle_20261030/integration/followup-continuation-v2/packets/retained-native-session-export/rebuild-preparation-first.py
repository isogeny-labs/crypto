from pathlib import Path
import subprocess,json,hashlib,re,tarfile,io
R=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029');D=R/'research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/retained-native-session-export';W=Path('/var/tmp/retained-native-export-rebuild-20261103-v1');W.mkdir();S=W/'source';S.mkdir();O=W/'output';O.mkdir();deps=W/'deps';deps.mkdir()
head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip()
paths=['Cargo.toml','Cargo.lock','src','research/pencil_large_prime_20261029/Cargo.toml','research/pencil_large_prime_20261029/src','docs/curves/registry.json','docs/curves/standards/parameters.json','docs/ecbench/schema.sql','docs/ic/boundary_targets.json','docs/ic/calibration.json']
archive=subprocess.check_output(['git','archive',head,'--',*paths],cwd=R)
with tarfile.open(fileobj=io.BytesIO(archive)) as t:t.extractall(S,filter='data')
def sha(p):
 h=hashlib.sha256()
 with p.open('rb') as f:
  for b in iter(lambda:f.read(1048576),b''):h.update(b)
 return h.hexdigest()
source={str(p.relative_to(S)):{'sha256':sha(p),'bytes':p.stat().st_size} for p in S.rglob('*') if p.is_file()}
reuse={}
for p in (R/'target/release/deps').iterdir():
 if p.suffix in ('.rlib','.rmeta','.a','.so') and not p.name.startswith(('libcrypto_lib-','libpencil_symbolic_graph-')):
  (deps/p.name).symlink_to(p);reuse[str(p)]={'sha256':sha(p),'bytes':p.stat().st_size}
fp=json.loads((R/'target/release/.fingerprint/crypto-daf9fdac4ff20898/lib-crypto_lib.json').read_text())
externs={}
for _,name,_,_ in fp['deps']:
 if name=='pencil_symbolic_graph':continue
 ps=list(deps.glob('lib'+name+'-*.rlib'))
 assert len(ps)==1,(name,ps)
 externs[name]=str(ps[0])
rustc='/home/ec2-user/.rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu/bin/rustc'
common=[rustc,'--edition=2021','--crate-type','rlib','--emit=dep-info,metadata,link','-C','opt-level=0','-C','codegen-units=1','-C','debuginfo=0','-L','dependency='+str(deps),'--out-dir',str(O)]
graph=common+['--crate-name','pencil_symbolic_graph',str(S/'research/pencil_large_prime_20261029/src/lib.rs'),'--cfg','feature="reporting"','--cfg','feature="serde"','--cfg','feature="serde_json"']
for name in ['serde','serde_json']:graph+=['--extern',name+'='+externs[name]]
lib=common+['--crate-name','crypto_lib',str(S/'src/lib.rs'),'--cfg','feature="default"','--cfg','feature="redis-native-tls"','--extern','pencil_symbolic_graph='+str(O/'libpencil_symbolic_graph.rlib'),'-L','dependency='+str(O)]
for n,p in externs.items():lib+=['--extern',n+'='+p]
native=[]
for p in (R/'target/release/build').glob('*/out'):
 if any(p.rglob('*.a')):native.append(str(p));lib+=['-L','native='+str(p)]
for d in native:
 for p in Path(d).rglob('*'):
  if p.is_file():reuse[str(p)]={'sha256':sha(p),'bytes':p.stat().st_size}
env={'PATH':'/usr/bin:/bin','HOME':str(W),'TMPDIR':str(O),'CARGO_MANIFEST_DIR':str(S),'CARGO_PKG_VERSION':'0.1.0','CARGO_NET_OFFLINE':'true','CARGO_BUILD_JOBS':'1','LC_ALL':'C','RUST_MIN_STACK':'16777216','CRYPTO_BUILD_GIT_COMMIT':head,'CRYPTO_BUILD_GIT_DIRTY':'false'}
plan={'status':'prospective_not_executed','source_commit':head,'source_root':str(S),'output':str(O),'command_argv':[graph,lib],'environment':env,'rustc_sha256':sha(Path(rustc)),'lock_sha256':sha(S/'Cargo.lock'),'source_files':len(source),'source_bytes':sum(v['bytes'] for v in source.values()),'reuse_files':len(reuse),'reuse_bytes':sum(v['bytes'] for v in reuse.values()),'max_seconds':300,'hard_as_bytes':8*1024**3,'disk_output_stop_threshold_bytes':4*1024**3,'disk_policy':'Prospective monitor threshold, not a filesystem quota; requires final review before execution.','scientific_runs':0,'tests':0}
for name,obj in [('rebuild-source-inputs.json',source),('rebuild-reused-inputs.json',reuse),('rebuild-command.draft.json',plan)]: (D/name).write_text(json.dumps(obj,indent=2)+'\n')
print(json.dumps({k:v for k,v in plan.items() if k not in ('command_argv','environment')},indent=2))
