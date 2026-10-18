#!/usr/bin/env python3
"""Deterministic, digest-pinned worker bundle; publish to the stack artifact bucket."""
import gzip
import hashlib
from pathlib import Path
import tarfile


def package(destination=None, binary_dir=None):
    here = Path(__file__).resolve().parent
    destination = destination or here / 'runtime.tar.gz'
    binary_dir = binary_dir or Path.home() / '.local/bin'
    sources = [(here/name, name) for name in ['runner.py', 'activity.py', 'worker_state.py', 'node_service.py', 'install_services.py']]
    sources += [(binary_dir/name, 'bin/'+name) for name in ['cairn', 'conductor', 'conductord', 'conductor-mcp']]
    for source, _ in sources:
        if not source.is_file():
            raise FileNotFoundError(source)
    with destination.open('wb') as raw, gzip.GzipFile(fileobj=raw, mode='wb', filename='', mtime=0, compresslevel=1) as zipped:
        with tarfile.open(fileobj=zipped, mode='w') as archive:
            for source, name in sources:
                info = archive.gettarinfo(str(source), arcname=name)
                info.mtime = info.uid = info.gid = 0
                info.uname = info.gname = ''
                info.mode = 0o755 if name.startswith('bin/') else 0o644
                with source.open('rb') as data:
                    archive.addfile(info, data)
    digest = hashlib.sha256(destination.read_bytes()).hexdigest()
    destination.with_suffix('.sha256').write_text(digest+'\n')
    return digest


if __name__ == '__main__':
    print(package())
