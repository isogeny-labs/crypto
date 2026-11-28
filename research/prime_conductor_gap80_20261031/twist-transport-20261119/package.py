#!/usr/bin/env python3
"""Package complete raw directories; verify extraction/repack byte identity."""
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile

HERE=Path(__file__).resolve().parent
MEMBERS=['results','qualification-a',*[p.name for p in sorted(HERE.glob('negative-*')) if p.is_dir()]]


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def pack(root,out):
    with out.open('wb') as sink:
        tar=subprocess.Popen(['tar','--sort=name','--mtime=2026-11-19T00:00:00Z',
            '--owner=0','--group=0','--numeric-owner','--format=gnu',
            '--mode=u+rwX,go+rX,go-w','-cf','-',*MEMBERS],cwd=root,stdout=subprocess.PIPE)
        subprocess.run(['xz','-6','-T1'],stdin=tar.stdout,stdout=sink,check=True)
        tar.stdout.close()
        if tar.wait()!=0:raise RuntimeError('tar failed')


archive=HERE/'raw-results.tar.xz'
pack(HERE,archive)
files=[{'path':str(p.relative_to(HERE)),'sha256':sha(p),'bytes':p.stat().st_size}
       for name in MEMBERS for p in sorted((HERE/name).rglob('*')) if p.is_file()]
with tempfile.TemporaryDirectory(prefix='conductor-twist-repack-') as tmp:
    restored=Path(tmp)
    subprocess.run(['tar','-xJf',str(archive),'-C',tmp],check=True)
    for f in files:
        p=restored/f['path'];assert sha(p)==f['sha256'] and p.stat().st_size==f['bytes']
    repack=restored/'repack.tar.xz';pack(restored,repack);assert sha(repack)==sha(archive)
digest=sha(archive)
receipt={'schema':'conductor.raw-archive/v1','archive':archive.name,'sha256':digest,
         'bytes':archive.stat().st_size,'extraction_check':'pass','repack_identical':True,
         'uploaded':False,'remote_verified':False,
         'desired_s3_uri':f's3://setec-astronomy-artifacts/store/sha256/{digest[:2]}/{digest}',
         'files':files}
(HERE/'archive-info.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps({k:v for k,v in receipt.items() if k!='files'},indent=2))
