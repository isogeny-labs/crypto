"""EKS worker: restore frozen source, run native arithmetic, preserve every outcome."""
from pathlib import Path
import hashlib,json,os,resource,shutil,subprocess,sys,tarfile,threading,time
source_uri,source_hash,binary_hash,dest=sys.argv[1:]
work=Path('/work');root=work/'prime-terminal-source';raw=work/'results'/'dataset'
launcher_results_uri=os.environ.get('ISOLAB_RESULTS_URI');dest=dest.rstrip('/')
assert dest.startswith('s3://setec-astronomy-artifacts/')
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
def aws(*args):subprocess.run(['aws','s3',*args,'--region','us-east-2','--only-show-errors'],check=True)
archive=work/'source.tar.gz';aws('cp',source_uri,str(archive));assert sha(archive)==source_hash
root.mkdir();raw.mkdir(parents=True)
with tarfile.open(archive,'r:gz') as tf:
 for m in tf.getmembers():
  assert not m.name.startswith('/') and '..' not in Path(m.name).parts and (m.isfile() or m.isdir()),m.name
 tf.extractall(root)
for line in (root/'manifest.sha256').read_text().splitlines():
 digest,name=line.split('  ',1);assert sha(root/name)==digest,name
instrument=Path('/usr/local/bin/prime_terminal');assert sha(instrument)==binary_hash
protocol=json.loads((root/'protocol-prime-terminal.json').read_text());assert protocol['freeze']['status']=='frozen'
assert protocol['arms'][0]['instrument']['binary_sha256']==binary_hash
build=json.loads(Path('/opt/prime-terminal/build-info.json').read_text());assert build['source_revision']==protocol['arms'][0]['instrument']['commit'];assert build['binary_sha256']==binary_hash
for name in ['protocol-prime-terminal.json','PRIME_TERMINAL_DESIGN.md','PRIME_TERMINAL_FORMAT.json']:shutil.copy2(root/name,raw/name)
shutil.copytree(root/'inputs',raw/'inputs');shutil.copytree(root/'development-validation',raw/'development-validation');shutil.copy2(instrument,raw/'instrument');shutil.copy2('/opt/prime-terminal/build-info.json',raw/'build-info.json')
(raw/'host.txt').write_text(subprocess.check_output(['uname','-a'],text=True));stop=threading.Event()
def upload():aws('sync',str(raw),dest+'/raw/')
def periodic():
 while not stop.wait(30):
  try:upload()
  except Exception as e:print('periodic_upload_error',type(e).__name__,flush=True)
thread=threading.Thread(target=periodic,daemon=True);thread.start()
def limits():resource.setrlimit(resource.RLIMIT_AS,(2*1024**3,2*1024**3))
status={'producer_exit_code':125,'replay_exit_code':125,'controls_exit_code':125,'wall_valid':False,'backend':'eks_spot','started_at':time.strftime('%Y-%m-%dT%H:%M:%SZ',time.gmtime())}
code=1
try:
 for name,args,seconds in [('controls',['controls'],60),('producer',['run',str(raw),str(raw/'results')],300),('replay',['replay',str(raw),str(raw/'results')],300)]:
  with (raw/(name+'.log')).open('wb') as out:
   try:r=subprocess.run([str(instrument),*args],stdout=out,stderr=subprocess.STDOUT,timeout=seconds,preexec_fn=limits);status[name+'_exit_code']=r.returncode
   except subprocess.TimeoutExpired:status[name+'_exit_code']=124
  if name=='controls' and status[name+'_exit_code']==0:shutil.copy2(raw/'controls.log',raw/'controls.json')
  (raw/'status.json').write_text(json.dumps(status,indent=2)+'\n');upload()
  assert status[name+'_exit_code']==0,(name,status)
 code=0
finally:
 stop.set();thread.join(timeout=60)
 status.update(exit_code=code,finished_at=time.strftime('%Y-%m-%dT%H:%M:%SZ',time.gmtime()))
 (raw/'terminal.json').write_text(json.dumps(status,indent=2)+'\n')
 files=sorted(p for p in raw.rglob('*') if p.is_file());(raw/'manifest.sha256').write_text(''.join(sha(p)+'  '+str(p.relative_to(raw))+'\n' for p in files))
 upload();packed=work/'results'/'runs.tar.gz'
 with tarfile.open(packed,'w:gz') as tf:tf.add(raw,arcname='.')
 identity={'archive_sha256':sha(packed),'archive_bytes':packed.stat().st_size,'archive_object':dest+'/runs.tar.gz','terminal':status,'source_bundle_sha256':source_hash,'binary_sha256':binary_hash,'manifest_payloads':len(files)}
 receipt=work/'results'/'custody.json';receipt.write_text(json.dumps(identity,indent=2)+'\n');aws('cp',str(packed),dest+'/runs.tar.gz');aws('cp',str(receipt),dest+'/custody.json');print(json.dumps(identity),flush=True)
sys.exit(code)
