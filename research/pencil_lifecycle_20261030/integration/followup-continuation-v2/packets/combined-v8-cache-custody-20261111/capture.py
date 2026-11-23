from pathlib import Path
import json,os,stat,hashlib,time,shutil,tarfile,gzip
D=Path(__file__).resolve().parent;P=D.parent;W=Path('/var/tmp/pencil-signed-kummer-combined-v8-20261111');C=W/'cache';R=Path('/home/ec2-user/pencil-validation-receipts/signed-kummer-combined-v8-20261111');S=P/'signed-kummer-combined-runner-v8-20261111';M=P/'root-cache-snapshot-results-20261111/raw/native/current-cache.json'
START=time.monotonic()
def guard():assert time.monotonic()-START<300 and shutil.disk_usage(D).free>6*1024**3

def digest(f):
 h=hashlib.sha256()
 with Path(f).open('rb') as stream:
  for b in iter(lambda:stream.read(1024**2),b''):guard();h.update(b)
 return h.hexdigest()
def put(n,v):(D/n).write_text(json.dumps(v,indent=2)+'\n')
base=json.loads(M.read_text());donor=Path('/var/tmp/pencil-root-materialize-t1953-20261109/cache-stage-v2');modes=json.loads((S/'cache-mode-inventory.json').read_text());assert modes['file_count']==2165
mode_by_path={p:int(modes['mode_table'][int(code,16)],8) for p,code in zip(sorted(base['files']),modes['mode_index_nibbles'])}
baseline={str(Path(p).relative_to(donor)):dict(v,mode=mode_by_path[p]) for p,v in base['files'].items()}
completion=json.loads((R/'completion.json').read_text());assert completion['seal_status']=='passed'
worker=json.loads((R/'worker/worker-receipt.json').read_text());binary=Path(worker['test_binary']['path']);assert binary.is_relative_to(C);binaryname=str(binary.relative_to(C));assert binary.stat().st_size==253684264 and digest(binary)==worker['test_binary']['sha256']=='e641bcf1d5e2be3369b52ca60ee0721fc8a89b734375e5334b026c31389feb88'
inventory={};directories=[]
def error(e):raise e
for root,dirs,names in os.walk(C,followlinks=False,onerror=error):
 guard();directories.append(str(Path(root).relative_to(C)))
 for name in dirs:assert stat.S_ISDIR((Path(root)/name).lstat().st_mode)
 for name in names:
  f=Path(root)/name;st=f.lstat();assert stat.S_ISREG(st.st_mode);inventory[str(f.relative_to(C))]={'sha256':digest(f),'bytes':st.st_size,'mode':stat.S_IMODE(st.st_mode)}
changed=[k for k in sorted(inventory) if k not in baseline or inventory[k]!=baseline[k]];missing=sorted(set(baseline)-set(inventory));delta=[k for k in changed if k!=binaryname];total=sum(inventory[k]['bytes'] for k in delta)+inventory[binaryname]['bytes'];assert total<2*1024**3 and shutil.disk_usage(D).free-total>6*1024**3
put('inventory.json',{'schema':'pencil.cache-inventory/v1','root':str(C),'directories':directories,'files':inventory})
put('preparation.json',{'cache_files':len(inventory),'cache_bytes':sum(v['bytes'] for v in inventory.values()),'changed_or_added':changed,'missing':missing,'archive_uncompressed_bytes':total,'binary_stored_separately_not_duplicated':True,'free_before_archive':shutil.disk_usage(D).free,'deleted':False})
archives={}
for filename,names in [('cache-delta.tar.gz',delta),('root-test-binary.tar.gz',[binaryname])]:
 with (D/filename).open('xb') as raw:
  with gzip.GzipFile(fileobj=raw,mode='wb',mtime=0,filename='',compresslevel=1) as gz:
   with tarfile.open(fileobj=gz,mode='w',dereference=True) as tar:
    for name in names:
     guard();f=C/name;row=inventory[name];assert digest(f)==row['sha256'] and stat.S_IMODE(f.lstat().st_mode)==row['mode'];info=tar.gettarinfo(str(f),arcname=name);info.uid=info.gid=0;info.uname=info.gname='';info.mtime=0
     with f.open('rb') as stream:tar.addfile(info,stream)
 seen=[]
 with tarfile.open(D/filename) as tar:
  for m in tar:
   guard();assert m.isfile() and m.name in names and m.name not in seen;row=inventory[m.name];assert m.size==row['bytes'] and m.mode==row['mode'];hh=hashlib.sha256();stream=tar.extractfile(m)
   for b in iter(lambda:stream.read(1024**2),b''):guard();hh.update(b)
   assert hh.hexdigest()==row['sha256'];seen.append(m.name)
 assert set(seen)==set(names);archives[filename]={'sha256':digest(D/filename),'bytes':(D/filename).stat().st_size,'members':len(seen),'all_member_hashes_and_modes_verified':True}
# Actual source and compiler artifact bindings, without executing the binary.
source=json.loads((S/'expected-source-manifest.json').read_text());assert len(source)==4485
for name,row in source.items():assert (W/'source'/name).stat().st_size==row['bytes'] and digest(W/'source'/name)==row['sha256']
compiles=[x for x in worker['commands'] if x['label']=='compile'];assert len(compiles)==1 and compiles[0]['exit_code']==0 and compiles[0]['reaped'] is True
assert digest(R/'worker/compile.stdout')==compiles[0]['stdout_sha256'];cargo=[]
for line in (R/'worker/compile.stdout').read_text().splitlines():
 try:entry=json.loads(line)
 except json.JSONDecodeError:continue
 if entry.get('reason')=='compiler-artifact' and entry.get('executable')==str(binary):cargo.append(entry)
assert len(cargo)==1 and cargo[0]['target']['name']=='crypto_lib' and cargo[0]['profile']['test'] is True
bindings={}
for label,f in [('completion',R/'completion.json'),('worker-receipt',R/'worker/worker-receipt.json'),('compile.stdout',R/'worker/compile.stdout'),('compile.stderr',R/'worker/compile.stderr'),('materialization',W/'materialization.json'),('expected-source-manifest',S/'expected-source-manifest.json'),('admission',S/'admission.json')]:
 assert f.is_file();dest=D/('binding-'+label+('' if '.' in label else '.json'));shutil.copyfile(f,dest);bindings[label]={'path':str(f),'sha256':digest(f),'bytes':f.stat().st_size,'retained':dest.name}
# Verify original current-cache members have not changed during capture.
for name,row in inventory.items():assert digest(C/name)==row['sha256'] and stat.S_IMODE((C/name).lstat().st_mode)==row['mode']
put('capture.json',{'schema':'pencil.cache-and-binary-custody/v1','status':'captured_and_verified','source_cache':str(C),'input_manifest':{'path':str(M),'sha256':digest(M)},'mode_inventory_sha256':digest(S/'cache-mode-inventory.json'),'cache_files':len(inventory),'cache_bytes':sum(v['bytes'] for v in inventory.values()),'changed_or_added_count':len(changed),'missing':missing,'archives':archives,'binary':dict(inventory[binaryname],path=str(binary),cargo_artifact=cargo[0]),'compiler_command':compiles[0],'exact_source_files_verified':len(source),'bindings':bindings,'workflow_status':completion['status'],'root_test_binary_compilation_succeeded':True,'test_execution_claim':False,'elapsed_seconds':time.monotonic()-START,'cache_modified':False,'donor_modified':False,'deleted':False,'uploaded':False})
print(json.dumps({'files':len(inventory),'changed':len(changed),'missing':len(missing),'archives':archives,'elapsed':time.monotonic()-START}))
