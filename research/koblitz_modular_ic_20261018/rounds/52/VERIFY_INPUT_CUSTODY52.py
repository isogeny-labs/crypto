"""Fresh direct GET/hash for the two immutable large donor objects; no math."""
from pathlib import Path
import json,hashlib,subprocess
h=Path(__file__).resolve().parent;repo=h.parents[3];manifest=json.loads((h/'source-data/.artifacts.json').read_text());rows=[]
for name,meta in manifest['files'].items():
 digest=meta['sha256'];key='store/sha256/'+digest[:2]+'/'+digest
 fresh=Path('/var/tmp/koblitz-r52-input-'+name+'.fresh');assert not fresh.exists(),'reconcile existing GET before repeating'
 p=subprocess.run(['aws','s3api','get-object','--bucket','setec-astronomy-artifacts','--key',key,'--checksum-mode','ENABLED',str(fresh)],capture_output=True,text=True)
 (h/'publication'/('input-'+name+'.get.json')).write_text(json.dumps({'exit':p.returncode,'stdout':p.stdout,'stderr':p.stderr,'key':key},indent=2)+'\n');assert p.returncode==0
 assert fresh.stat().st_size==meta['bytes'] and hashlib.sha256(fresh.read_bytes()).hexdigest()==digest
 local=h/'source-data'/name;assert local.read_bytes()==fresh.read_bytes()
 source=str(local.relative_to(repo));birth=subprocess.check_output(['git','log','--diff-filter=A','--format=%H','--',source],cwd=repo,text=True).splitlines()[-1]
 historical=subprocess.check_output(['git','show',birth+':'+source],cwd=repo);assert historical==fresh.read_bytes()
 rows.append({'name':name,'sha256':digest,'bytes':meta['bytes'],'fresh_GetObject_hash_match':True,'local_bytes_unchanged':True,'historical_freeze_revision':birth,'historical_git_bytes_match':True})
(h/'input-custody52.json').write_text(json.dumps({'round':52,'bucket':'setec-astronomy-artifacts','objects':rows,'history_rewritten':False},indent=2)+'\n')
print(json.dumps({'objects_verified':len(rows),'fresh_downloads':True,'original_freeze_bytes_preserved':True}))
