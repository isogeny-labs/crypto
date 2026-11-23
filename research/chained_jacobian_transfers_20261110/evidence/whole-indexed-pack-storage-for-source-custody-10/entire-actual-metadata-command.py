from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,os,shutil,subprocess
r=Path('/workspace/crypto');stem='pack-52825694c66bbac7be9341b85bd98f6d93ce1754';folder=r/'.git/objects/pack';h=Path('/dev/shm/ct370-entire-existing-indexed-pack');h.mkdir();env=os.environ.copy();env['GIT_OPTIONAL_LOCKS']='0'
def git(*args):return subprocess.check_output(['git',*args],cwd=r,env=env)
head=git('rev-parse','HEAD');tree=git('rev-parse','HEAD^{tree}');ledger=git('show','HEAD:pcc/ledger.jsonl');working=(r/'pcc/ledger.jsonl').read_bytes();index=(r/'.git/index').read_bytes();rows=[]
for f in sorted(folder.glob(stem+'.*')):
 assert f.is_file() and not f.is_symlink();b=f.read_bytes();q=h/f.name;q.write_bytes(b);assert q.read_bytes()==b
 rows.append(dict(path=str(f),bytes=len(b),sha256=hashlib.sha256(b).hexdigest()));f.unlink();f.symlink_to(q);assert f.read_bytes()==b
assert any(x['path'].endswith('.pack') for x in rows)
assert git('rev-parse','HEAD')==head and git('rev-parse','HEAD^{tree}')==tree and git('show','HEAD:pcc/ledger.jsonl')==ledger and (r/'pcc/ledger.jsonl').read_bytes()==working and (r/'.git/index').read_bytes()==index
receipt=dict(at=datetime.now(timezone.utc).isoformat(),entire_pack_and_companions=rows,entire_byte_count=sum(x['bytes'] for x in rows),head_before_after=head.decode().strip(),tree_before_after=tree.decode().strip(),published_and_working_ledger_and_current_index_byte_identical=True,whole_hold=str(h),scope='complete Git object storage relocation only; original paths remain exact byte symlinks, no object/source/raw deletion or scientific rerun')
(h/'actual-whole-indexed-pack-custody.json').write_text(json.dumps(receipt,indent=2)+'\n')
g=r/'research/chained_jacobian_transfers_20261110/evidence/whole-indexed-pack-storage-for-source-custody-10';g.mkdir();shutil.copyfile(h/'actual-whole-indexed-pack-custody.json',g/'actual-whole-indexed-pack-custody.json');shutil.copyfile(__file__,g/'entire-actual-metadata-command.py')
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement',f'Whole existing indexed Git pack{stem} and all companions relocated byte-exact toRAM, original paths preserved as symlinks, total{receipt["entire_byte_count"]}B. Actual c8aa HEAD/tree/published ledger/working ledger/canonical index unchanged; every object byte retained, no source/raw/Git object deletion or scientific rerun. This recovers overlay room for whole guarded-source archive publication. Lost volatile metadata requires re-fetching published immutable objects after preserving working files, not science. Existing normal configured Git read/fresh merge-branch transition works; do not inject invalid shell-token auth. Metadata only, no paid work/signed promotion.' ]
for f in sorted(g.iterdir()):cmd+=['--proof','attestation:'+f.relative_to(r).as_posix()]
print(subprocess.check_output(cmd,cwd=r,env=env,text=True).strip(),json.dumps(receipt))
