#!/usr/bin/env python3
"""Create a deterministic, limited source archive for the rho worker panel."""

import argparse
import gzip
from hashlib import sha256
from io import BytesIO
from pathlib import Path
import re
import tarfile


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
EXPLICIT = [
    'Cargo.toml', 'Cargo.lock',
    'examples/rho_worker_phase_profile.rs',
    'examples/rho_worker_batch_benchmark.rs',
    'experiments/rho-worker-phase-profile-20261017/rho-closeout-q-manifest.json',
    'experiments/rho-worker-phase-profile-20261017/run_tier1_worker_batch.py',
    'experiments/rho-worker-phase-profile-20261017/build_worker_batch_plan.py',
    'experiments/rho-worker-phase-profile-20261017/analyze_tier1_worker_batch.py',
    'experiments/rho-worker-phase-profile-20261017/validate_worker_batch_fixtures.py',
]


def paths():
    result = {ROOT / name for name in EXPLICIT}
    result.update((ROOT / 'src').rglob('*'))
    result.update((ROOT / 'tools' / 'volcano-cli').rglob('*'))
    manifest = (ROOT / 'Cargo.toml').read_text()
    for name in re.findall(r'path\s*=\s*"([^"]+)"', manifest):
        path = ROOT / name
        if path.is_dir():
            result.update(path.rglob('*'))
        else:
            result.add(path)
    files = sorted(path for path in result if path.is_file())
    if not files or any(not path.is_relative_to(ROOT) for path in files):
        raise ValueError('source selection escaped repository')
    if any(not path.is_file() for path in (ROOT / name for name in EXPLICIT)):
        raise ValueError('required source file missing')
    return files


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    files = paths()
    buffer = BytesIO()
    with tarfile.open(fileobj=buffer, mode='w', format=tarfile.PAX_FORMAT) as archive:
        for path in files:
            data = path.read_bytes()
            info = tarfile.TarInfo(path.relative_to(ROOT).as_posix())
            info.size = len(data)
            info.mode = 0o755 if path.suffix == '.py' else 0o644
            info.mtime = 0
            info.uid = info.gid = 0
            info.uname = info.gname = ''
            archive.addfile(info, BytesIO(data))
    args.output.parent.mkdir(parents=True, exist_ok=True)
    with args.output.open('wb') as stream:
        with gzip.GzipFile(filename='', mode='wb', fileobj=stream, mtime=0) as compressed:
            compressed.write(buffer.getvalue())
    print(f'{args.output} {sha256(args.output.read_bytes()).hexdigest()} {len(files)} files')


if __name__ == '__main__':
    main()
