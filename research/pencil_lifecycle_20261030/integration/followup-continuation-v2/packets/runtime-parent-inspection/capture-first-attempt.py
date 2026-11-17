import json,subprocess,tempfile,resource,time,hashlib
from pathlib import Path
from datetime import datetime,timezone
p=Path(__file__).parent
image="sha256:347667d6ecc9294ab7bfcc83411fb47f7548c8d1ca5f7d315cca23fc4b115476"
private=Path(tempfile.mkdtemp(prefix="pencil-base-inspection-",dir="/var/tmp"))
config=private/'docker-config';config.mkdir(mode=0o700)
base=['/usr/bin/sudo','-n','/usr/bin/env','-i','PATH=/usr/bin:/bin',f'DOCKER_CONFIG={config}','/usr/bin/timeout','--kill-after=5','90','/usr/bin/docker','--host','unix:///var/run/docker.sock']
fmt='{ "Id":{{json .Id}}, "Size":{{json .Size}}, "OnBuild":{{json .Config.OnBuild}}, "Layers":{{json .RootFS.Layers}}, "OS":{{json .Os}}, "Architecture":{{json .Architecture}} }'
q=subprocess.run(base+['image','inspect','--format',fmt,image],capture_output=True,timeout=100)
if q.returncode: raise RuntimeError(q.stderr.decode())
obs=json.loads(q.stdout);assert obs['Id']==image and not obs['OnBuild'] and obs['Size']<700*1024**2
(p/'image-inspection.json').write_text(json.dumps(obs,indent=2)+'\n')
archive=private/'base.tar';args=base+['image','save',image]
def limits(): resource.setrlimit(resource.RLIMIT_FSIZE,(768*1024**2,768*1024**2))
start=time.monotonic()
with archive.open('xb') as out, (private/'save.stderr').open('xb') as err:
 result=subprocess.run(args,stdout=out,stderr=err,preexec_fn=limits,timeout=105)
receipt=dict(schema='runtime.image-save/v1',utc=datetime.now(timezone.utc).isoformat(),image=image,argv=args,exit=result.returncode,elapsed_seconds=time.monotonic()-start,archive=str(archive),bytes=archive.stat().st_size,timeout_seconds=90,output_limit_bytes=768*1024**2,container_actions=0,payload_executions=0,scientific_execution=False)
with archive.open('rb') as f:receipt['sha256']=hashlib.file_digest(f,'sha256').hexdigest()
(p/'save-receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
(p/'save.stderr').write_bytes((private/'save.stderr').read_bytes())
print(json.dumps(receipt))
if result.returncode: raise SystemExit(result.returncode)
