"""Package retained process bytes; administrative only, no numeric analysis."""
import hashlib
import json
import shutil
import subprocess
import sys
import tarfile
from pathlib import Path

root = Path(__file__).resolve().parents[2]
study = root / sys.argv[1]
prefix = sys.argv[2]
runs = study / 'runs'
assert not runs.exists(), 'Refuse to overwrite a retained tree'
runs.mkdir()
for label in ['primary', 'pilot', 'calibration-final']:
    src = Path('/workspace/scratch') / f'trace-{prefix}-{label}'
    assert src.is_dir(), src
    shutil.copytree(src, runs / label)
if (study / 'profiles').exists():
    for src in sorted((study / 'profiles').glob('*/callgrind.out')):
        dst = runs / 'discovery' / src.parent.name
        dst.mkdir(parents=True)
        shutil.copy2(src, dst / src.name)
members = []
for p in sorted(runs.rglob('*')):
    if p.is_file():
        raw = p.read_bytes()
        members.append({'archive_member': str(p.relative_to(study)),
                        'bytes': len(raw), 'sha256': hashlib.sha256(raw).hexdigest()})
manifest = {'schema': 'trace-zero-raw-archive/v1', 'files': members,
            'scope': 'Exact primary, pilot, final calibration and available discovery counter bytes; no solver statistics computed here.'}
(study / 'raw-tree-manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
tar_args = ['tar', '--sort=name', '--mtime=2026-11-15 00:00:00Z', '--owner=0',
            '--group=0', '--numeric-owner', '--mode=u+rwX,go+rX,go-w',
            '--format=gnu', '-cf', '-', 'runs']
def pack(cwd, dest):
    with dest.open('wb') as out:
        a = subprocess.Popen(tar_args, cwd=cwd, stdout=subprocess.PIPE)
        b = subprocess.run(['xz', '-6', '-T1'], stdin=a.stdout, stdout=out, check=True)
        a.stdout.close()
        assert a.wait() == 0
archive = study / 'runs.tar.xz'
pack(study, archive)
digest = hashlib.sha256(archive.read_bytes()).hexdigest()
(study / 'runs.tar.xz.sha256').write_text(digest + '  runs.tar.xz\n')
checkdir = Path('/workspace/scratch') / f'trace-{prefix}-archive-check'
assert not checkdir.exists()
checkdir.mkdir()
with tarfile.open(archive) as tf:
    tf.extractall(checkdir, filter='data')
for m in members:
    data = (checkdir / m['archive_member']).read_bytes()
    assert len(data) == m['bytes']
    assert hashlib.sha256(data).hexdigest() == m['sha256']
repacked = checkdir / 'repacked.tar.xz'
pack(checkdir, repacked)
assert hashlib.sha256(repacked.read_bytes()).hexdigest() == digest
(study / 'checks/archive-check.txt').write_text(
    f'{len(members)} file members extracted and SHA-256 matched; deterministic repack byte-identical.\nArchive SHA-256 {digest}\n')
print(study.name, len(members), digest)
