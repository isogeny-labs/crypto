"""Archive/file/hash IO ONLY: never dispatches or analyzes research computations.

Plain raw objects share an xz dictionary. A gzip9 recipe is used only after
ACTUAL byte-identical reconstruction. Other files retain their literal bytes.
"""
import argparse, gzip, hashlib, io, json, tarfile, tempfile, zlib
from pathlib import Path

p=argparse.ArgumentParser()
p.add_argument('input',type=Path)
p.add_argument('archive',type=Path)
p.add_argument('--remove-verified-new-copies',action='store_true')
a=p.parse_args();source=a.input.resolve();archive=a.archive.resolve()
assert source.is_dir() and not archive.exists() and source not in archive.parents
assert str(source).startswith('/tmp/twist-first-row-run')
if a.remove_verified_new_copies:
    assert source.parent==Path('/tmp/twist-first-row-run01-20261116')
    assert source.name.startswith(('A-','B-','false-'))
sha=lambda b:hashlib.sha256(b).hexdigest()
files={};objects={}
for path in sorted(source.rglob('*')):
    assert not path.is_symlink()
    if not path.is_file():continue
    data=path.read_bytes();stored=data;recipe='literal';header=''
    if data[:4]==b'\x1f\x8b\x08\x00':
        try:
            raw=gzip.decompress(data)
            rebuilt=data[:10]+gzip.compress(raw,compresslevel=9,mtime=0)[10:]
            if rebuilt==data:stored=raw;recipe='gzip9';header=data[:10].hex()
        except (OSError,EOFError):pass
    h=sha(stored)
    if h in objects:assert objects[h]==stored
    else:objects[h]=stored
    files[str(path.relative_to(source))]={'sha256':sha(data),'bytes':len(data),'stored_sha256':h,'recipe':recipe,'gzip_header_hex':header}
manifest=(json.dumps({'schema':'twist.lossless-raw-codec/v1','zlib_runtime':zlib.ZLIB_RUNTIME_VERSION,'files':files},sort_keys=True,indent=2)+'\n').encode()
def pack(destination,contents):
    with tarfile.open(destination,'w:xz',preset=3,format=tarfile.PAX_FORMAT)as tf:
        for name,data in sorted({'FILES.json':manifest,**{'objects/'+h:b for h,b in contents.items()}}.items()):
            info=tarfile.TarInfo(name);info.size=len(data);info.mode=0o600;info.mtime=0;tf.addfile(info,io.BytesIO(data))
def restore(row,stored):
    assert sha(stored)==row['stored_sha256']
    if row['recipe']=='gzip9':
        data=bytes.fromhex(row['gzip_header_hex'])+gzip.compress(stored,compresslevel=9,mtime=0)[10:]
    else:data=stored
    assert len(data)==row['bytes'] and sha(data)==row['sha256']
    return data
pack(archive,objects)
with tarfile.open(archive,'r:xz')as tf:
    assert tf.extractfile('FILES.json').read()==manifest
    recovered={m.name.split('/')[1]:tf.extractfile(m).read()for m in tf if m.isfile()and m.name.startswith('objects/')}
for h,b in recovered.items():assert sha(b)==h
with tempfile.TemporaryDirectory(prefix='counter-byte-restore-',dir='/workspace')as temporary:
    for name,row in files.items():
        rel=Path(name);assert not rel.is_absolute()and'..'not in rel.parts
        dest=Path(temporary)/rel;dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(restore(row,recovered[row['stored_sha256']]))
        assert dest.read_bytes()==(source/rel).read_bytes();dest.unlink()
    repack=Path(temporary)/'repack.tar.xz';pack(repack,recovered);assert repack.read_bytes()==archive.read_bytes()
receipt={'schema':'twist.lossless-raw-codec-receipt/v1','source':str(source),'archive':str(archive),'archive_bytes':archive.stat().st_size,'archive_sha256':sha(archive.read_bytes()),'raw_paths':len(files),'unique_objects':len(objects),'gzip_recipe_paths':sum(x['recipe']=='gzip9'for x in files.values()),'zlib_runtime':zlib.ZLIB_RUNTIME_VERSION,'all_original_compressed_and_raw_bytes_actually_restored_hashed':True,'deterministic_repack_exact':True,'old_inputs_untouched':True,'removed_only_verified_new_copies':False}
if a.remove_verified_new_copies:
    actual={str(x.relative_to(source))for x in source.rglob('*')if x.is_file()};assert actual==set(files)
    for name,row in files.items():assert sha((source/name).read_bytes())==row['sha256']
    for name in files:(source/name).unlink()
    for directory in sorted(source.rglob('*'),key=lambda x:len(x.parts),reverse=True):
        if directory.is_dir():directory.rmdir()
    source.rmdir();receipt['removed_only_verified_new_copies']=True
Path(str(archive)+'.receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps(receipt))
