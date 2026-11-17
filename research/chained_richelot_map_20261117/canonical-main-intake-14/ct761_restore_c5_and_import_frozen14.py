"""Exact authoritative metadata intake, immutable artifact references, journal union."""
from pathlib import Path
from datetime import datetime,timezone
import collections,hashlib,json,os,re,shutil,subprocess,tarfile
r=Path('/workspace/crypto-chained-balanced-main');s=r/'research/chained_richelot_map_20261117';o=s/'canonical-main-intake-14';o.mkdir(exist_ok=True)
base='c5cbb67b19fe39864da7f8c9b29726696ba01580';tracking='6f1815b9f90b71f407f68e77901d3cc6f12cb8f9';old='265b97cabcc41ad60ba86d2d769b4cc24836c4ea'
env=dict(os.environ,GIT_INDEX_FILE='/run/lock/chained-transfers-ct669/canonical-current-index',GIT_OPTIONAL_LOCKS='0',GIT_NO_LAZY_FETCH='1')
def run(argv,check=True,lazy=False):
 e=dict(env)
 if lazy:e.pop('GIT_NO_LAZY_FETCH',None)
 p=subprocess.run(argv,cwd=r,env=e,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
 if check:assert p.returncode==0,(argv,p.returncode,p.stderr)
 return p
def git(*a,lazy=False):return run(['git','-c','gc.auto=0',*a],lazy=lazy).stdout
def blob(b):return hashlib.sha1(b'blob '+str(len(b)).encode()+b'\0'+b).hexdigest()
def digest(b):return hashlib.sha256(b).hexdigest()
assert git('rev-parse','HEAD').decode().strip()==tracking
assert git('diff','--name-status',base,tracking).decode().strip()=='M\tdocs/ACTIVE.md'
assert git('write-tree','--missing-ok').strip()==git('rev-parse',tracking+'^{tree}').strip()
assert Path('/workspace/scratch/ct758-old14-custody-exact-readback.txt').read_text().split()[0]==old
source_journal=(r/'docs/session-logs/2026-11-17.md').read_bytes()
assert source_journal==git('cat-file','blob',old+':docs/session-logs/2026-11-17.md')
scopes=['pcc','tools/pcc','tools/artifacts','tools/lessons.py','tools/autonomy_report.py','tools/goal_brief.py','.artifacts/config.json','docs/lessons','benchmarks/agent-autonomy','docs/session-logs/2026-11-17.md','AGENTS.md']
raw=git('ls-tree','-r','-z',base,'--',*scopes);entries=[]
for record in raw.split(b'\0'):
 if record:
  meta,path=record.split(b'\t',1);mode,kind,oid=meta.decode().split();assert kind=='blob';entries.append((path.decode(),mode,oid))
paths={x[0] for x in entries};legacy=[]
for p in sorted((r/'pcc/claims').glob('*.json')):
 rel=str(p.relative_to(r))
 if rel not in paths:
  b=p.read_bytes();assert git('cat-file','blob',old+':'+rel)==b
  dest=o/'legacy-claims'/p.name;dest.parent.mkdir(exist_ok=True);assert not dest.exists();p.rename(dest)
  legacy.append({'old_path':rel,'new_preserved_path':str(dest.relative_to(r)),'bytes':len(b),'sha256':digest(b)})
inventory=[];changes=[];fetches=[]
for rel,mode,oid in entries:
 dest=r/rel;b=dest.read_bytes() if dest.is_file() else None;method='existing exact canonical bytes'
 if b is None or blob(b)!=oid:
  before=b;source=None
  for root in [Path('/workspace/crypto'),Path('/workspace/cryptanalysis'),Path('/workspace/crypto-chained-main-validated')]:
   p=root/rel
   if p.is_file() and blob(p.read_bytes())==oid:b=p.read_bytes();source=p;break
  else:
   p=run(['git','cat-file','blob',oid],False)
   if p.returncode:
    p=run(['git','-c','gc.auto=0','cat-file','blob',oid],False,lazy=True);fetches.append({'path':rel,'oid':oid,'exit':p.returncode,'stderr':p.stderr.decode(errors='replace')})
   assert p.returncode==0,(rel,p.stderr);b=p.stdout
  assert blob(b)==oid
  dest.parent.mkdir(parents=True,exist_ok=True)
  if dest.exists():dest.unlink()
  if source is not None and rel.startswith('pcc/claims/'):os.link(source,dest);method='exact immutable canonical claim hardlink'
  else:dest.write_bytes(b);dest.chmod(0o755 if mode=='100755' else 0o644);method='exact canonical blob copy'
  changes.append({'path':rel,'old_bytes':len(before) if before is not None else None,'new_bytes':len(b),'method':method})
 assert blob(dest.read_bytes())==oid
 inventory.append({'path':rel,'Git_blob':oid,'bytes':len(b),'sha256':digest(b),'method':method})
ledger=(r/'pcc/ledger.jsonl').read_bytes();assert ledger==git('cat-file','blob',base+':pcc/ledger.jsonl')
verify=run(['python3','tools/pcc/pcc.py','ledger','verify','--base',base],False)
(o/'EXACT_C5_BASELINE_PCC.stdout').write_bytes(verify.stdout);(o/'EXACT_C5_BASELINE_PCC.stderr').write_bytes(verify.stderr)
assert not verify.stderr
baseline=[x for x in verify.stdout.decode().splitlines() if x.startswith('claim ')]
assert verify.returncode in [0,1];assert baseline or verify.returncode==0
receipt={'time':datetime.now(timezone.utc).isoformat(),'canonical_base':base,'initial_tracking_commit':tracking,'complete_old_public_source_fork':old,'physical_canonical_metadata_files':len(inventory),'physical_canonical_claims':sum(x['path'].startswith('pcc/claims/') for x in inventory),'base_ledger_rows':len(ledger.splitlines()),'base_ledger_bytes':len(ledger),'base_ledger_sha256':digest(ledger),'baseline_PCC_exit':verify.returncode,'baseline_existing_claim_diagnostics':len(baseline),'all_authoritative_file_bytes_checked':inventory,'changes':changes,'normal_narrow_metadata_fetches':fetches,'old_raw_claims_preserved_without_rechain':legacy,'keyring_policy_verifiers_unchanged_by_us':True,'new_draft_tracking_rule_read_and_applied':True}
(o/'EXACT_C5_METADATA_INTAKE.json').write_text(json.dumps(receipt,indent=2)+'\n')
# Preserve every normalized full timestamp block with maximum multiplicity.
base_journal=(r/'docs/session-logs/2026-11-17.md').read_text();source=source_journal.decode()
pat=re.compile(r'(?m)^(?:#{1,3} )?2026-11-17(?:T|[ ~])[0-9~][^\n]*$')
def split(text):
 starts=[m.start() for m in pat.finditer(text)];assert starts
 def norm(x):return re.sub(r'\n\s*---\s*$','',x.strip()).strip()
 return text[:starts[0]].strip(),[norm(text[a:b]) for a,b in zip(starts,starts[1:]+[len(text)])]
pref,a=split(base_journal);other_pref,b=split(source);ca=collections.Counter(a);cb=collections.Counter(b);want=ca|cb
def stamp(block):
 line=block.splitlines()[0];m=re.search(r'2026-11-17(?:T|[ ~]+)(\d\d):(\d\d)(?::(\d\d))?',line);return (tuple(map(int,[m[1],m[2],m[3] or '00'])) if m else (0,0,0),line)
if other_pref and other_pref not in pref:pref+='\n\n'+other_pref
merged=pref+'\n\n---\n\n'+'\n\n---\n\n'.join(sorted(want.elements(),key=stamp))+'\n';assert collections.Counter(split(merged)[1])==want
(r/'docs/session-logs/2026-11-17.md').write_text(merged)
(o/'FULL_JOURNAL_MAX_MULTISET_UNION.json').write_text(json.dumps({'time':datetime.now(timezone.utc).isoformat(),'base':base,'source':old,'base_bytes':len(base_journal.encode()),'source_bytes':len(source_journal),'base_blocks':len(a),'source_blocks':len(b),'union_blocks':sum(want.values()),'full_block_max_multiplicities_roundtrip_equal':True,'all_normalized_full_blocks':[{'sha256':digest(block.encode()),'base':ca[block],'source':cb[block],'output':want[block]} for block in sorted(want)]},indent=2)+'\n')
# Import the entire frozen source tree, then omit inline containers from the new PR.
rel=str(s.relative_to(r));assert not git('ls-tree',base,'--',rel)
git('read-tree','--prefix='+rel+'/',old+':'+rel)
raw=git('ls-tree','-r','-z',old,'--',rel);artifacts=[]
for record in raw.split(b'\0'):
 if not record:continue
 meta,path=record.split(b'\t',1);mode,kind,oid=meta.decode().split();path=path.decode()
 if path.endswith('.xz'):
  p=r/path;candidates=[p,Path('/dev/net/chained-transfers-owned-storage-20261117')/p.name,Path('/run/lock/ct758-entire-old-source-PCC-journal-tools-and-index.tar.xz')]
  payload=None
  for candidate in candidates:
   if candidate.is_file():
    b=candidate.read_bytes()
    if blob(b)==oid:payload=b;break
  if payload is None:payload=git('cat-file','blob',oid,lazy=True)
  assert blob(payload)==oid
  artifacts.append({'path':path,'Git_blob':oid,'bytes':len(payload),'sha256':digest(payload),'immutable_public_commit':old,'raw_url':'https://raw.githubusercontent.com/isogeny-labs/crypto/'+old+'/'+path,'complete_primary_bytes_remain_public':True,'S3_migration':'pending; not claimed uploaded'})
  git('update-index','--force-remove','--',path)
(s/'IMMUTABLE_RAW_ARTIFACTS.json').write_text(json.dumps({'schema':'chained.immutable-artifact-references/v1','source_public_checkpoint':old,'all_complete_containers_retained_without_primary_data_deletion':True,'canonical_main_artifact_policy_not_changed':True,'artifacts':artifacts},indent=2)+'\n')
# This file lists local hydration caches, so a later add cannot inline them accidentally.
(s/'.gitignore').write_text('*.xz\n__pycache__/\n')
active=r/'docs/ACTIVE.md';txt=active.read_text();assert 'branch lane/chained-richelot-map14; in progress' in txt;active.write_text(txt.replace('branch lane/chained-richelot-map14; in progress','branch lane/chained-richelot-map14, PR #4544; in progress'))
print(json.dumps({'canonical_files':len(inventory),'canonical_claims':receipt['physical_canonical_claims'],'base_ledger_rows':receipt['base_ledger_rows'],'baseline_diagnostics':len(baseline),'legacy_claims':len(legacy),'journal_base_blocks':len(a),'journal_source_blocks':len(b),'journal_union_blocks':sum(want.values()),'complete_external_containers':len(artifacts),'new_intake_claim_not_yet_written':True},indent=2))
