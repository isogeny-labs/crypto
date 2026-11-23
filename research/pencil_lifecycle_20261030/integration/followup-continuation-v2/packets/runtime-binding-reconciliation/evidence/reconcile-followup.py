from pathlib import Path
import subprocess,datetime,json
D=Path(__file__).parent; patterns=['/var/tmp/pencil-copy-only-assembly-supervisor-20261031-v1/assemble.py','/var/tmp/pencil-runtime-only-recovery-supervisor-20261031-v1/recover.py','/var/tmp/pencil-runtime-qualification-supervisor-20261031-v5/supervise.py'];tags=['pencil-local-base:467e8502-46d8-4ab9-a13b-8d2375dd2ca2','pencil-local-runtime:467e8502-46d8-4ab9-a13b-8d2375dd2ca2'];matches=[]
for p in Path('/proc').glob('[0-9]*'):
 try:
  args=[x.decode(errors='replace') for x in (p/'cmdline').read_bytes().split(b'\0')];hits=[x for x in patterns+tags if x in args]
  if hits:matches.append({'pid':int(p.name),'matched_exact_arguments':hits,'comm':(p/'comm').read_text().strip(),'stat':(p/'stat').read_text()})
 except (FileNotFoundError,PermissionError,ProcessLookupError):pass
(D/'process-observation-corrected.json').write_text(json.dumps({'observed_at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'exact_script_paths':patterns,'exact_tags':tags,'matches':matches,'correction':'initial observation used supervise.py for two supervisors whose actual names are assemble.py and recover.py; initial narrow observation preserved'},indent=2))
config=str(D/'empty-docker-config');base=['/usr/bin/sudo','-n','/usr/bin/env','-i','PATH=/usr/bin:/bin','HOME='+config,'DOCKER_CONFIG='+config,'/usr/bin/docker','--host','unix:///var/run/docker.sock'];fmt='{"Id":{{json .Id}},"Name":{{json .Name}},"Image":{{json .Image}},"State":{{json .State.Status}},"Pid":{{json .State.Pid}},"Owner":{{json (index .Config.Labels "isolab.runtime-qualification.owner")}}}'
for c in json.loads((D/'exact-containers.json').read_text()):
 out=D/'observations'/('container-name-'+c['case']);out.mkdir();cmd=base+['container','inspect','--format',fmt,c['name']];p=subprocess.run(cmd,stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=15,env={'PATH':'/usr/bin:/bin','LANG':'C'});(out/'stdout').write_bytes(p.stdout);(out/'stderr').write_bytes(p.stderr);(out/'receipt.json').write_text(json.dumps({'argv':cmd,'exit':p.returncode,'observed_at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'timeout_seconds':15},indent=2));print(c['case'],p.returncode)
