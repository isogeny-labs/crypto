"""Acquire and compare whole public source archives; file/hash metadata only."""
from pathlib import Path
import hashlib,json,shutil,tarfile,urllib.request,traceback
from datetime import datetime,timezone
R=Path('/dev/shm/chained-cubic-coordinate-20261127/crypto')
S=R/'research/chained_cubic_coordinate_20261127'
H=Path('/dev/shm/ct27-H-cargo-home')
A=Path('/dev/shm/ct27-H-official-archives')
I='index.crates.io-1949cf8c6b5b557f'
A.mkdir(exist_ok=False)
H.mkdir(exist_ok=False)
packages=json.loads(Path('/dev/shm/ct27-H-dependencies.json').read_text())
records=[]
try:
    shutil.copytree(Path('/home/agent/.cargo/registry/index')/I,H/'registry/index'/I)
    for pkg in packages:
        name=pkg['name']+'-'+pkg['version']
        url='https://static.crates.io/crates/'+pkg['name']+'/'+name+'.crate'
        dest=A/(name+'.crate')
        with urllib.request.urlopen(url,timeout=60) as response:raw=response.read()
        dest.write_bytes(raw)
        got=hashlib.sha256(raw).hexdigest()
        assert got==pkg['checksum'],name
        prior=Path('/home/agent/.cargo/registry/cache')/I/(name+'.crate')
        assert prior.read_bytes()==raw,name
        cached=H/'registry/cache'/I/(name+'.crate');cached.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(dest,cached)
        src=Path('/home/agent/.cargo/registry/src')/I/name
        copied=H/'registry/src'/I/name
        shutil.copytree(src,copied)
        with tarfile.open(dest,'r:gz') as archive:
            for member in archive.getmembers():
                if member.isfile():
                    original=archive.extractfile(member).read()
                    installed=copied/Path(member.name).relative_to(name)
                    assert installed.read_bytes()==original,str(installed)
        records.append({'package':pkg['name'],'version':pkg['version'],'url':url,'whole_bytes':len(raw),'sha256':got,'archive':str(dest),'installed_source':str(copied),'whole_official_archive_readback':True})
    inputs={}
    for base in [H/'registry/index'/I,H/'registry/cache'/I,H/'registry/src'/I,A]:
        for p in base.rglob('*'):
            if p.is_file():
                b=p.read_bytes();inputs[str(p)]={'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest(),'mode':p.stat().st_mode&0o777}
    record={'schema':'chained-identity-official-dependency-inputs/v1','recorded':datetime.now(timezone.utc).isoformat(),'cargo_home':str(H),'packages':records,'files':inputs,'policy':'Official whole original crates fetched/read and compared to lock checksum and installed extracted source. No repackaging, source substitution or numerical research in Python.'}
    (S/'DEPENDENCY_INPUTS_H.json').write_text(json.dumps(record,indent=2)+'\n')
    print(json.dumps({'whole_public_archives':len(records),'guarded_files':len(inputs),'record_bytes':(S/'DEPENDENCY_INPUTS_H.json').stat().st_size}))
except BaseException:
    (A/'acquisition-failure.stderr').write_text(traceback.format_exc())
    raise
