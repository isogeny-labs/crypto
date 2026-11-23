#!/usr/bin/env python3
"""Read-only selected-file verification; writes only explicitly named private receipt."""
from pathlib import Path
import json,hashlib,stat,sys
proposal=Path(__file__).parent;root=Path(sys.argv[1]);dest=Path(sys.argv[2]);assert str(dest).startswith('/var/tmp/') and not dest.exists()
rows=json.loads((proposal/'selected.json').read_text())['files'];out=[];errors=[]
for row in rows:
 p=root/row['path']
 try:
  s=p.lstat();assert stat.S_ISREG(s.st_mode),'not ordinary file';data=p.read_bytes();obj=hashlib.sha1(b'blob '+str(len(data)).encode()+b'\0'+data).hexdigest();assert obj==row['object'],'Git blob mismatch';assert bool(s.st_mode&0o111)==(row['mode']=='100755'),'executable-mode mismatch';out.append({'path':row['path'],'sha256':hashlib.sha256(data).hexdigest(),'git_blob':obj,'bytes':len(data)})
 except Exception as e:errors.append({'path':row['path'],'error':repr(e)})
dest.write_text(json.dumps({'source_root':str(root),'selected_inventory_sha256':hashlib.sha256((proposal/'selected.json').read_bytes()).hexdigest(),'files':out,'errors':errors,'pass':not errors},indent=2)+'\n');sys.exit(bool(errors))
