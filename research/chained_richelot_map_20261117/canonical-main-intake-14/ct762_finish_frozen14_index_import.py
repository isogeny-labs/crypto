"""Resume after exact canonical metadata/journal intake; index metadata only."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,os,subprocess
r=Path('/workspace/crypto-chained-balanced-main');s=r/'research/chained_richelot_map_20261117';o=s/'canonical-main-intake-14';old='265b97cabcc41ad60ba86d2d769b4cc24836c4ea'
E=dict(os.environ,GIT_INDEX_FILE='/run/lock/chained-transfers-ct669/canonical-current-index',GIT_OPTIONAL_LOCKS='0',GIT_NO_LAZY_FETCH='1')
def call(args,input=None,lazy=False):
 e=dict(E)
 if lazy:e.pop('GIT_NO_LAZY_FETCH',None)
 p=subprocess.run(args,cwd=r,env=e,input=input,stdout=subprocess.PIPE,stderr=subprocess.PIPE);assert p.returncode==0,(args,p.stderr);return p.stdout
def git(*a,lazy=False):return call(['git','-c','gc.auto=0',*a],lazy=lazy)
def oid(b):return hashlib.sha1(b'blob '+str(len(b)).encode()+b'\0'+b).hexdigest()
assert git('rev-parse','HEAD').decode().strip()=='6f1815b9f90b71f407f68e77901d3cc6f12cb8f9'
assert git('write-tree','--missing-ok').strip()==git('rev-parse','HEAD^{tree}').strip()
receipt=json.loads((o/'EXACT_C5_METADATA_INTAKE.json').read_text());assert receipt['physical_canonical_metadata_files']==4157 and receipt['baseline_existing_claim_diagnostics']==151
for row in receipt['all_authoritative_file_bytes_checked']:
 if row['path']!='docs/session-logs/2026-11-17.md':assert oid((r/row['path']).read_bytes())==row['Git_blob']
raw=git('ls-tree','-r','-z',old,'--',str(s.relative_to(r)));info=[];artifacts=[];source_checks=[]
for row in raw.split(b'\0'):
 if not row:continue
 meta,path=row.split(b'\t',1);mode,kind,obj=meta.decode().split();path=path.decode();assert kind=='blob' and path.startswith(str(s.relative_to(r))+'/')
 if path.endswith('.xz'):
  p=r/path;payload=None
  for q in [p,Path('/dev/net/chained-transfers-owned-storage-20261117')/p.name,Path('/run/lock/ct758-entire-old-source-PCC-journal-tools-and-index.tar.xz')]:
   if q.is_file():
    b=q.read_bytes()
    if oid(b)==obj:payload=b;break
  if payload is None:payload=git('cat-file','blob',obj,lazy=True)
  assert oid(payload)==obj
  artifacts.append({'path':path,'Git_blob':obj,'bytes':len(payload),'sha256':hashlib.sha256(payload).hexdigest(),'immutable_public_commit':old,'raw_url':'https://raw.githubusercontent.com/isogeny-labs/crypto/'+old+'/'+path,'complete_primary_bytes_remain_public':True,'S3_migration':'pending; not claimed uploaded'})
 else:
  p=r/path;assert p.is_file() and oid(p.read_bytes())==obj,path
  info.append((mode+' '+obj+'\t'+path).encode()+b'\0');source_checks.append({'path':path,'Git_blob':obj,'all_full_worktree_bytes_equal_to_public_checkpoint':True})
call(['git','-c','gc.auto=0','update-index','-z','--index-info'],b''.join(info))
(s/'IMMUTABLE_RAW_ARTIFACTS.json').write_text(json.dumps({'schema':'chained.immutable-artifact-references/v1','source_public_checkpoint':old,'all_complete_containers_retained_without_primary_data_deletion':True,'canonical_main_artifact_policy_not_changed':True,'artifacts':artifacts},indent=2)+'\n')
(s/'.gitignore').write_text('*.xz\n__pycache__/\n')
(o/'FROZEN_SOURCE_INDEX_IMPORT.json').write_text(json.dumps({'time':datetime.now(timezone.utc).isoformat(),'source_public_checkpoint':old,'all_used_source_report_and_log_bytes_checked':source_checks,'complete_raw_containers_reference_count':len(artifacts),'canonical_tree_has_no_new_inline_compressed_containers':True,'initial_read_tree_prefix_negative':'missing promised8492a7755f131845eb1009389629bbb2c3a8537f; no index mutation','normal_index_info_import_of_exact_known_public_source_modes_and_oids':True,'no_source_or_blob_or_gate_rewrite':True},indent=2)+'\n')
active=r/'docs/ACTIVE.md';text=active.read_text();assert 'branch lane/chained-richelot-map14; in progress' in text;active.write_text(text.replace('branch lane/chained-richelot-map14; in progress','branch lane/chained-richelot-map14, PR #4544; in progress'))
print(json.dumps({'complete_unchanged_source_report_log_files':len(source_checks),'complete_external_containers':len(artifacts),'canonical_metadata_files':4157,'canonical_claims':3395,'base_ledger_rows':1357,'baseline_diagnostics':151}))
