from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,os,shutil,subprocess
r=Path('/workspace/crypto');stem='pack-8752eafa63fdc065af5da0a83f436aff97bfdd41'
hold=Path('/dev/shm/ct268-entire-recently-fetched-Git-pack');hold.mkdir()
env=os.environ.copy();env['GIT_OPTIONAL_LOCKS']='0'
def git(*args):return subprocess.check_output(['git',*args],cwd=r,env=env)
before=dict(head=git('rev-parse','HEAD').decode().strip(),tree=git('rev-parse','HEAD^{tree}').decode().strip(),
    published_ledger_sha256=hashlib.sha256(git('show','HEAD:pcc/ledger.jsonl')).hexdigest(),
    working_ledger_sha256=hashlib.sha256((r/'pcc/ledger.jsonl').read_bytes()).hexdigest())
assert before['head']=='2d54889ac1032d59fe8b5410ad26ff705cec5bd7'
records=[]
for suffix in ['pack','idx','rev','promisor']:
    source=r/'.git/objects/pack'/(stem+'.'+suffix)
    assert source.is_file() and not source.is_symlink()
    data_hash=hashlib.sha256(source.read_bytes()).hexdigest();size=source.stat().st_size
    target=hold/source.name;shutil.copyfile(source,target)
    assert target.stat().st_size==size and hashlib.sha256(target.read_bytes()).hexdigest()==data_hash
    temporary=source.with_name(source.name+'.whole-RAM-custody-link')
    temporary.symlink_to(target);os.replace(temporary,source)
    assert source.is_symlink() and hashlib.sha256(source.read_bytes()).hexdigest()==data_hash
    records.append(dict(original_path=str(source),whole_RAM_file=str(target),bytes=size,sha256=data_hash))
after=dict(head=git('rev-parse','HEAD').decode().strip(),tree=git('rev-parse','HEAD^{tree}').decode().strip(),
    published_ledger_sha256=hashlib.sha256(git('show','HEAD:pcc/ledger.jsonl')).hexdigest(),
    working_ledger_sha256=hashlib.sha256((r/'pcc/ledger.jsonl').read_bytes()).hexdigest())
assert before==after
g=r/'research/chained_jacobian_transfers_20261110/evidence/fetched-Git-pack-storage-custody-02';g.mkdir()
shutil.copyfile(__file__,g/'original-command.py')
(g/'whole-byte-storage-custody.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),before=before,after=after,whole_files=records,
    reason='original read-only genuine5ae1 commit lookup fetched this32.7MB pack and its indices; overlay reached3.3MiB free',
    all_git_object_bytes_preserved=True,no_scientific_or_raw_deletion=True,
    recovery='if volatile Git cache is lost preserve all working data, re-fetch published5ae1/2d548 immutable commits through the existing authorized Git transport; never rerun science'),indent=2)+'\n')
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','Near-full overlay Git-cache storage recovery: all4 complete files of the recently auto-fetched32.7MB genuine-merge pack and its indices/promisor metadata are copied whole with SHA/size equality to RAM, original paths atomically become exact whole-byte symlinks. Original/current published HEAD/tree/published-ledger/working-ledger identities remain exact before/after, no source/research/raw/Git object bytes deleted or histories rewritten. Earlier split-index custodybfeb59 remains unchanged. The pack was fetched by original genuine5ae1 read, not a new research computation. Cache loss requires authenticated re-fetch of published immutable objects, preserving working data and never repeating scientific populations.']
for f in sorted(g.iterdir()):cmd+=['--proof','attestation:'+f.relative_to(r).as_posix()]
print(subprocess.check_output(cmd,cwd=r,env=env,text=True).strip())
