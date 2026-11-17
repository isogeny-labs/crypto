"""Metadata-only authoritative intake; preserve the complete old source fork."""
from pathlib import Path
from datetime import datetime, timezone
import hashlib, json, lzma, os, subprocess, sys, tarfile

repo = Path('/workspace/crypto-chained-balanced-main')
packet = repo/'research/chained_richelot_joint_chart_20261117/source-fork-before-e742-intake-06'
base = sys.argv[1] if len(sys.argv)>1 else 'e7425e39f1b12148f272f94113f7a12cf2cd89c2'
env = dict(os.environ, GIT_INDEX_FILE='/run/lock/chained-transfers-ct669/canonical-current-index', GIT_OPTIONAL_LOCKS='0', GIT_NO_LAZY_FETCH='1')
def git(*args):
    return subprocess.check_output(['git', '-c', 'gc.auto=0', *args], cwd=repo, env=env)
def blob_hash(b):
    return hashlib.sha1(b'blob '+str(len(b)).encode()+b'\0'+b).hexdigest()
assert git('rev-parse','HEAD').decode().strip() == base
assert git('write-tree','--missing-ok').strip() == git('rev-parse',base+'^{tree}').strip()
archive=packet/'entire-original-source-PCC-journal-policy-tools-and-index.tar.xz'
custody=json.loads((packet/'OLD_SOURCE_FORK_CUSTODY.json').read_text())
assert hashlib.sha256(archive.read_bytes()).hexdigest()==custody['archive_sha256']
scope=['pcc','tools/pcc','tools/artifacts','tools/lessons.py','tools/autonomy_report.py','tools/goal_brief.py','.artifacts/config.json','docs/lessons','benchmarks/agent-autonomy','docs/session-logs/2026-11-17.md','docs/ACTIVE.md','AGENTS.md']
raw=git('ls-tree','-r','-z',base,'--',*scope)
entries=[]
for row in raw.split(b'\0'):
    if row:
        meta,p=row.split(b'\t',1);mode,kind,oid=meta.decode().split()
        assert kind=='blob' and mode in ('100644','100755')
        entries.append((p.decode(),mode,oid))
base_paths={p for p,_,_ in entries}
legacy=[]
with tarfile.open(archive,'r:xz') as t:
    members={m.name:m for m in t.getmembers() if m.isfile()}
    for p in sorted((repo/'pcc/claims').glob('*.json')):
        rel=p.relative_to(repo).as_posix()
        if rel not in base_paths:
            assert rel in members,rel
            b=p.read_bytes();assert t.extractfile(members[rel]).read()==b
            dest=packet/'legacy-claims'/p.name;dest.parent.mkdir(exist_ok=True)
            assert not dest.exists();p.rename(dest)
            legacy.append({'original_path':rel,'preserved_path':dest.relative_to(repo).as_posix(),'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest()})
roots=[Path('/workspace/crypto'),Path('/workspace/cryptanalysis'),Path('/workspace/crypto-chained-main-validated')]
inventory=[];changes=[];fetches=[]
for rel,mode,oid in entries:
    dest=repo/rel;b=dest.read_bytes() if dest.is_file() else None
    method='existing exact authoritative bytes'
    if b is None or blob_hash(b)!=oid:
        before=b;source=None
        for root in roots:
            p=root/rel
            if p.is_file():
                candidate=p.read_bytes()
                if blob_hash(candidate)==oid:b=candidate;source=p;break
        else:
            p=subprocess.run(['git','cat-file','blob',oid],cwd=repo,env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
            if p.returncode:
                narrow=dict(env);narrow.pop('GIT_NO_LAZY_FETCH')
                p=subprocess.run(['git','-c','gc.auto=0','cat-file','blob',oid],cwd=repo,env=narrow,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
                fetches.append({'path':rel,'oid':oid,'exit':p.returncode,'entire_stderr':p.stderr.decode(errors='replace')})
            assert p.returncode==0,(rel,p.stderr)
            b=p.stdout
        assert blob_hash(b)==oid,rel
        dest.parent.mkdir(parents=True,exist_ok=True)
        # Copy-on-write: old claims can be hardlinked to protected checkouts.
        if dest.exists():dest.unlink()
        if source is not None and rel.startswith('pcc/claims/'):
            os.link(source,dest);method='exact immutable claim hardlink'
        else:
            dest.write_bytes(b);dest.chmod(0o755 if mode=='100755' else 0o644);method='exact authoritative blob copy'
        changes.append({'path':rel,'old_bytes':len(before) if before is not None else None,'new_bytes':len(b),'method':method})
    assert blob_hash(dest.read_bytes())==oid,rel
    inventory.append({'path':rel,'git_blob':oid,'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest(),'method':method})
ledger=(repo/'pcc/ledger.jsonl').read_bytes()
receipt={'time':datetime.now(timezone.utc).isoformat(),'canonical_base':base,'complete_index_tree_equal_to_base_before_feature':True,'old_complete_public_source_checkpoint':custody['entire_public_source_checkpoint'],'old_ledger_preserved_without_rechain':True,'all_authoritative_files_checked':len(inventory),'all_authoritative_claims_checked':sum(x['path'].startswith('pcc/claims/') for x in inventory),'base_ledger_rows':len(ledger.splitlines()),'base_ledger_bytes':len(ledger),'base_ledger_sha256':hashlib.sha256(ledger).hexdigest(),'all_authoritative_blob_inventory':inventory,'changes':changes,'normal_narrow_fetches':fetches,'old_fork_claims_moved_whole':legacy,'no_new_claim_created':True}
out=packet/('EXACT_MAIN_'+base[:12]+'_METADATA_INTAKE.json.xz');out.write_bytes(lzma.compress((json.dumps(receipt,indent=2)+'\n').encode()))
assert json.loads(lzma.decompress(out.read_bytes()))==receipt
(packet/('NORMAL_FRESH_BASE_'+base[:12]+'_INDEX.json')).write_text(json.dumps({'time':receipt['time'],'head':base,'entire_index_tree':git('write-tree','--missing-ok').decode().strip(),'base_tree':git('rev-parse',base+'^{tree}').decode().strip(),'partial_clone_object_existence':'write-tree --missing-ok permits unmaterialized unrelated blobs; complete exact tree equality, plus physical custody of all in-scope metadata','old_checkpoint':custody['entire_public_source_checkpoint']},indent=2)+'\n')
print(json.dumps({k:receipt[k] for k in ['canonical_base','all_authoritative_files_checked','all_authoritative_claims_checked','base_ledger_rows','base_ledger_bytes']},indent=2));print('changed',len(changes),'normal_fetches',len(fetches),'legacy_claims',len(legacy))
