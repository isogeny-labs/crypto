"""Preserve every Git object while removing only redundant loose storage."""
from pathlib import Path
import hashlib,json,os,re,subprocess,zlib
R=Path('/workspace/rootfinding-pair-pr-stage');C=Path('/workspace/generalized_roots/research/autolab_cycle_20261115_v9')
def git(args,data=None):return subprocess.check_output(['git',*args],cwd=R,input=data)
def sha(p):
    h=hashlib.sha256()
    with p.open('rb')as f:
        for b in iter(lambda:f.read(1048576),b''):h.update(b)
    return h.hexdigest()
def inventory():return set(git(['cat-file','--batch-all-objects','--batch-check=%(objectname)']).decode().splitlines())
def working():
    h=hashlib.sha256()
    for n in git(['ls-files','-z']).split(b'\0'):
        if not n:continue
        p=R/os.fsdecode(n);h.update(n+b'\0')
        h.update(hashlib.sha256(os.fsencode(os.readlink(p))).digest()if p.is_symlink()else bytes.fromhex(sha(p)))
    return h.hexdigest()
def loose():
    d=R/'.git/objects'
    return sorted([p for q in d.iterdir()if q.is_dir()and re.fullmatch('[0-9a-f]{2}',q.name)for p in q.iterdir()if p.is_file()and re.fullmatch('[0-9a-f]{38}',p.name)and p.stat().st_size<=1024],key=lambda p:(p.stat().st_size,str(p)))
def immutable_meta():
    files=[R/'.git/HEAD',R/'.git/index',*sorted((R/'.git/logs').rglob('*'))]
    return {str(p.relative_to(R)):sha(p)for p in files if p.is_file()}
assert not(C/'GIT_PACK_RECEIPT.json').exists()
before=inventory();refs=git(['show-ref']);meta=immutable_meta();work=working();batches=[]
for limit in [256,256,None]:
    selected=loose();selected=selected if limit is None else selected[:limit]
    if not selected:break
    oids=[p.parent.name+p.name for p in selected]
    for p,oid in zip(selected,oids):assert hashlib.sha1(zlib.decompress(p.read_bytes())).hexdigest()==oid
    payload=sum(p.stat().st_size for p in selected);allocated=sum(p.stat().st_blocks*512 for p in selected)
    available=os.statvfs(R).f_bavail*os.statvfs(R).f_frsize
    assert payload*2+len(selected)*40+8192<available,'insufficient temporary pack space'
    digest=git(['pack-objects','--threads=1','--compression=9','--window=10','--depth=10','.git/objects/pack/pack'],('\n'.join(oids)+'\n').encode()).decode().strip()
    idx=R/'.git/objects/pack'/('pack-'+digest+'.idx')
    git(['verify-pack',str(idx)])
    listed=git(['verify-pack','-v',str(idx)]).decode().splitlines()
    packed={v.split()[0]for v in listed if re.match('[0-9a-f]{40} ',v)}
    assert packed==set(oids)
    check=git(['cat-file','--batch-check'],('\n'.join(oids)+'\n').encode()).decode().splitlines()
    assert len(check)==len(oids)and all(' missing'not in line for line in check)
    keep=idx.with_suffix('.keep')
    if not keep.exists():keep.write_text('Preserve all historical objects, including unreachable research evidence.\n')
    dry=git(['prune-packed','-n']).decode();prune={a+b for a,b in re.findall(r'([0-9a-f]{2})/([0-9a-f]{38})',dry)}
    assert prune<=set(oids),'unexpected duplicate storage outside selected objects'
    git(['prune-packed'])
    assert inventory()==before and git(['show-ref'])==refs,'Git logical history changed'
    batches.append(dict(objects=len(oids),loose_payload_bytes=payload,loose_allocated_bytes=allocated,pack_hash=digest,pack_verified=True,unreachable_objects_kept=True,only_packed_duplicates_removed=True))
assert immutable_meta()==meta and working()==work,'working files/index/reflogs changed'
receipt=dict(schema='lossless-git-storage-maintenance/v1',all_object_ids_preserved=len(before),refs_unchanged=True,head_index_reflogs_unchanged=True,all_tracked_working_bytes_unchanged=True,working_tree_sha256=work,batches=batches,free_bytes_after=os.statvfs(R).f_bavail*os.statvfs(R).f_frsize,scientific_files_deleted=False)
with(C/'GIT_PACK_RECEIPT.json').open('x')as f:json.dump(receipt,f,indent=2);f.write('\n')
print(json.dumps(receipt))
