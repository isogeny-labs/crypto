"""Bounded local Docker metadata readback only."""
import json,subprocess,tempfile
from pathlib import Path
from datetime import datetime,timezone
config=Path(tempfile.mkdtemp(prefix='runtime-replacement-readonly-',dir='/var/tmp'))
prefix=['/usr/bin/sudo','-n','/usr/bin/env','-i','PATH=/usr/bin:/bin','DOCKER_CONFIG='+str(config),'/usr/bin/timeout','--kill-after=5','20','/usr/bin/docker','--host','unix:///var/run/docker.sock']
image='sha256:347667d6ecc9294ab7bfcc83411fb47f7548c8d1ca5f7d315cca23fc4b115476'
fmt='{ "Id":{{json .Id}}, "OnBuild":{{json .Config.OnBuild}}, "User":{{json .Config.User}}, "Layers":{{json .RootFS.Layers}}, "OS":{{json .Os}}, "Architecture":{{json .Architecture}} }'
rows=[]
for args in [['image','inspect','--format',fmt,image],['ps','-a','--no-trunc','--format','{{.ID}}']]:
 q=subprocess.run(prefix+args,capture_output=True,timeout=30)
 rows.append(dict(argv=prefix+args,exit=q.returncode,stdout=q.stdout.decode(),stderr=q.stderr.decode()))
print(json.dumps(dict(schema='runtime.replacement.local-observation/v1',observed_utc=datetime.now(timezone.utc).isoformat(),commands=rows,container_actions=0,builds=0),indent=2))
assert all(r['exit']==0 for r in rows)
b=json.loads(rows[0]['stdout']);assert b['Id']==image and not b['OnBuild'] and b['OS']=='linux' and b['Architecture']=='amd64'
