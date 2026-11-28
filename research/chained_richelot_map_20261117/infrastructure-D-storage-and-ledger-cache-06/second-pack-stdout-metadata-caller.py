"""Compress duplicate Git representations of unchanged PCC ledger data.
No ledger bytes, claims, verifier, policy or shared object files are edited.
"""
from pathlib import Path
from datetime import datetime,timezone
import os,json,hashlib,subprocess,lzma
r=Path('/workspace/crypto-chained-balanced-main');s=r/'research/chained_richelot_map_20261117';p=s/'infrastructure-D-storage-and-ledger-cache-06';p.mkdir(exist_ok=True)
env=dict(os.environ,GIT_OPTIONAL_LOCKS='0',GIT_NO_LAZY_FETCH='1',GIT_INDEX_FILE='/run/lock/chained-transfers-ct669/canonical-current-index')
refs=['233c0cab0e2facffa22769d063450e1bccba00d8','e9d833a53265908311f087ca736bfd0bed3c4f9e','77d9ad43f2a0706b5c76eaf997262f52e1ca380b','ac01a90b6ee997fcf9ea99977d5d4b1d07a5ab1c','98ac7649c22f41c59b0b9c65dc2ac583324529e3','ab9297b229b406a10581b85a176266c90c2da12c','c0a2f94debb9bd26763ebabb0dda746936fdbdd1','0bc406900e2ef9756215d3623a3b34db8b68da2b','c331063303f6275938d0c715101fa4f6e1e7a632']
rows=[];objects={}
for ref in refs:
 x=subprocess.run(['git','rev-parse',ref+':pcc/ledger.jsonl'],cwd=r,env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE);assert x.returncode==0,x.stderr
 oid=x.stdout.decode().strip();data=subprocess.check_output(['git','cat-file','blob',oid],cwd=r,env=env);assert hashlib.sha1(b'blob '+str(len(data)).encode()+b'\0'+data).hexdigest()==oid
 objects[oid]=data;rows.append({'ref':ref,'oid':oid,'bytes':len(data),'sha256':hashlib.sha256(data).hexdigest()})
cache=Path('/run/lock/chained-transfers-ct669/owned-ledger-delta-objects-v2');(cache/'pack').mkdir(parents=True,exist_ok=False)
argv=['git','pack-objects','--stdout','--window=50','--depth=50','--threads=1','--delta-base-offset','--no-reuse-delta']
x=subprocess.run(argv,cwd=r,env=env,input=('\n'.join(objects)+'\n').encode(),stdout=subprocess.PIPE,stderr=subprocess.PIPE)
(p/'second-pack-stdout-process.json').write_text(json.dumps({'argv':argv,'exit':x.returncode,'complete_stderr':x.stderr.decode(),'pack_bytes':len(x.stdout)},indent=2)+'\n')
assert x.returncode==0,x.stderr
payload=x.stdout;assert hashlib.sha1(payload[:-20]).digest()==payload[-20:]
pack_id=payload[-20:].hex();pack=cache/'pack'/('pack-'+pack_id+'.pack');pack.write_bytes(payload);pack.chmod(0o444);idx=pack.with_suffix('.idx')
ix=subprocess.run(['git','index-pack','--strict','-o',str(idx),str(pack)],cwd=r,env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
(p/'second-pack-index-process.json').write_text(json.dumps({'exit':ix.returncode,'complete_stdout':ix.stdout.decode(),'complete_stderr':ix.stderr.decode()},indent=2)+'\n');assert ix.returncode==0,ix.stderr
v=subprocess.run(['git','verify-pack','-v',str(idx)],cwd=r,env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE);assert v.returncode==0,v.stderr
(p/'whole-verified-owned-ledger-delta-pack-inventory.txt.xz').write_bytes(lzma.compress(v.stdout))
isolated=dict(env,GIT_OBJECT_DIRECTORY=str(cache),GIT_ALTERNATE_OBJECT_DIRECTORIES='')
for oid,b in objects.items():
 x=subprocess.run(['git','cat-file','blob',oid],cwd=r,env=isolated,stdout=subprocess.PIPE,stderr=subprocess.PIPE);assert x.returncode==0 and x.stdout==b,(oid,x.stderr)
alternates=Path('/run/lock/chained-transfers-ct669/owned-canonical-gitdir/objects/info/alternates');old=alternates.read_bytes();new=old+(b'' if old.endswith(b'\n') else b'\n')+str(cache).encode()+b'\n';alternates.write_bytes(new);assert new.startswith(old)
before=subprocess.check_output(['git','prune-packed','-n'],cwd=r,env=env)
(p/'normal-prune-packed-dryrun.txt').write_bytes(before)
subprocess.run(['git','prune-packed'],cwd=r,env=env,check=True)
for oid,b in objects.items():assert subprocess.check_output(['git','cat-file','blob',oid],cwd=r,env=env)==b
receipt={'time':datetime.now(timezone.utc).isoformat(),'scope':'owned Git object storage only; exact unchanged historical ledger bytes','all_original_objects_full_byte_equal_from_isolated_new_pack':True,'ledgers':rows,'pack_bytes':pack.stat().st_size,'index_bytes':idx.stat().st_size,'pack_sha256':hashlib.sha256(pack.read_bytes()).hexdigest(),'normal_prune_packed_only':True,'pruned_owned_loose_entries':len(before.splitlines()),'old_alternates_complete_prefix_preserved':True,'new_owned_object_cache':str(cache),'no_ledger_row_rewrite_or_rechain':True,'no_shared_original_objects_or_indices_changed':True,'runlock_available_after':os.statvfs('/run/lock').f_bavail*os.statvfs('/run/lock').f_frsize}
(p/'ACTUAL_OWNED_GIT_LEDGER_STORAGE_COMPACTION.json').write_text(json.dumps(receipt,indent=2)+'\n');print(json.dumps({k:receipt[k] for k in ['pack_bytes','pruned_owned_loose_entries','runlock_available_after']},indent=2))
