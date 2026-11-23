"""Preserve entire first source/publication failure; metadata and bytes only."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,os,shutil,subprocess,tarfile
r=Path('/workspace/crypto-chained-balanced-main');s=r/'research/chained_primefield_cost_20261117';h=Path('/dev/shm/ct807-first-cost16A');o=s/'first-source-A-publication-negative-03';o.mkdir(exist_ok=False)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
j=json.loads((h/'all-phases.json').read_text());assert j['first_failure']=='init' and len(j['phases'])==2 and not (h/'source').exists() and not (h/'deps').exists()
lock=json.loads((s/'SOURCE_LOCK_16.json').read_text())
for row in lock['files']:assert sha(s/row['path'])==row['sha256'],row['path']
E=dict(os.environ,GIT_OPTIONAL_LOCKS='0',GIT_NO_LAZY_FETCH='1');argv=['git','cat-file','blob','8f3d314c9f6ba549f27d660776888b65b63e29f1:research/chained_primefield_cost_20261117/Cargo.lock'];p=subprocess.run(argv,cwd=r,env=E,stdout=subprocess.PIPE,stderr=subprocess.PIPE);assert p.returncode==128
(o/'POST_CALL_DIAGNOSTIC_RECHECK.stdout').write_bytes(p.stdout);(o/'POST_CALL_DIAGNOSTIC_RECHECK.stderr').write_bytes(p.stderr)
(o/'POST_CALL_DIAGNOSTIC_RECHECK.json').write_text(json.dumps({'argv':argv,'exit':p.returncode,'scope':'post-call read-only check on exact immutable A commit; not the lost original captured Git stderr','original_Git_stderr_gap':'A helper saved only CalledProcessError repr, not exception.stdout/stderr; preserve gap explicitly','no_scientific_rerun':True},indent=2)+'\n')
files={('failed-custody/'+p.relative_to(h).as_posix()):p for p in sorted(h.rglob('*')) if p.is_file()}
for row in lock['files']:files['entire-declared-A-source/'+row['path']]=s/row['path']
files['entire-declared-A-source/SOURCE_LOCK_16.json']=s/'SOURCE_LOCK_16.json'
for ext in ['stdout','stderr']:
 p=Path('/workspace/scratch/ct807-first-cost16A-caller.'+ext);files['outer-caller/'+p.name]=p;shutil.copy2(p,o/('outer-caller.'+ext))
(o/'outer-caller.exit').write_text('1\n');files['outer-caller/caller.exit']=o/'outer-caller.exit'
for p in sorted(o.glob('POST_CALL_DIAGNOSTIC_RECHECK.*')):files['post-call-read-only-diagnostic/'+p.name]=p
a=Path('/dev/net/chained-transfers-owned-storage-20261117/native15-complete-packets/entire-first16A-declared-source-and-publication-guard-failure.tar.xz');assert not a.exists()
with tarfile.open(a,'w:xz',preset=6) as t:
 for n,p in files.items():t.add(p,arcname=n,recursive=False)
inventory=[]
with tarfile.open(a,'r:xz') as t:
 assert len(t.getmembers())==len(files)
 for n,p in files.items():
  m=t.getmember(n);b=t.extractfile(m).read();assert b==p.read_bytes() and m.mode==p.stat().st_mode&0o7777;inventory.append({'member':n,'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest(),'mode':m.mode})
for p in h.iterdir():
 if p.is_file():shutil.copy2(p,o/p.name)
data={'recorded':datetime.now(timezone.utc).isoformat(),'source_A':'8f3d314c9f6ba549f27d660776888b65b63e29f1','caller_exit':1,'first_failure':'init/exact-public-source/Cargo.lock','Cargo_lock_ignored_while_physical_source_lock_included_it':True,'source_copy_dependency_copy_compilation_and_native_not_reached':True,'original85_sources_checked_only_after_call':True,'post_call_missing_artifact_guards':['copied source','dependencies','tools'],'complete_declared_source_in_packet':len(lock['files'])+1,'full_dependency_rlibs_or_ELFs_not_copied_or_created':True,'original_metadata_Git_stderr_was_not_retained':True,'separate_post_call_diagnostic_check_preserved_not_relabelled':True,'whole_packet':{'path':str(a),'bytes':a.stat().st_size,'sha256':sha(a),'members':len(files),'all_full_bytes_and_modes_compared':True},'next_prospective_B':'force-add exact unchanged Cargo.lock; separate helper/caller/lock revision retains full failed Git stdout/stderr before raising; all original math/oracles/population/limits unchanged','artifact_policy':'whole new raw packet private, no inline compressed data'}
(o/'ACTUAL_FIRST16A_PUBLICATION_NEGATIVE.json').write_text(json.dumps(data,indent=2)+'\n');(o/'WHOLE_PACKET_INVENTORY.json').write_text(json.dumps(inventory,indent=2)+'\n');shutil.copy2(Path(__file__),o/'seal-complete-metadata-only.py');print(json.dumps(data,indent=2))
