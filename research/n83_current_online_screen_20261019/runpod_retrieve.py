#!/usr/bin/env python3
"""Fetch and verify the sealed N83 RunPod raw parts through its SSH PTY."""

import argparse
import hashlib
import json
import subprocess
import sys
import tarfile
from pathlib import Path

FETCH = Path(__file__).resolve().with_name('runpod_fetch_part.py')


def sha256(path):
    digest = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            digest.update(block)
    return digest.hexdigest()


def fetch(user, remote, output, expected=None):
    if output.exists():
        if expected is None or sha256(output) == expected:
            return
        raise ValueError(f'existing artifact differs: {output}')
    command = [sys.executable, str(FETCH), '--user', user,
               '--remote', remote, '--output', str(output)]
    if expected:
        command.extend(['--sha256', expected])
    for attempt in range(1, 4):
        result = subprocess.run(command, check=False)
        if result.returncode == 0:
            return
        capture = output.with_suffix(output.suffix + '.pty')
        if capture.exists():
            capture.unlink()
        print(f'fetch attempt {attempt} failed for {remote}', flush=True)
    raise RuntimeError(f'could not fetch {remote}')


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--user', required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    root = args.output.resolve()
    root.mkdir(parents=True, exist_ok=True)
    manifest = root / 'raw_parts_manifest.json'
    fetch(args.user, '/workspace/raw_parts_manifest.json', manifest)
    record = json.loads(manifest.read_text())
    if record.get('schema') != 'n83.current-online-screen-runpod-raw-parts/v1':
        raise ValueError('unexpected raw part manifest')
    archive = root / 'n83_screen_raw.tar.gz'
    for part in record['parts']:
        name = part['name']
        if not name.startswith('raw.part.') or not name[9:].isdigit():
            raise ValueError('unexpected part name')
        path = root / name
        fetch(args.user, '/workspace/' + name, path, part['sha256'])
        if path.stat().st_size != part['bytes']:
            raise ValueError(f'part size differs: {name}')
    if not archive.exists():
        with archive.open('wb') as output:
            for part in record['parts']:
                with (root / part['name']).open('rb') as stream:
                    for block in iter(lambda: stream.read(1024 * 1024), b''):
                        output.write(block)
    if (archive.stat().st_size != record['archive_bytes'] or
            sha256(archive) != record['archive_sha256']):
        raise ValueError('reassembled raw archive differs')
    extracted = root / 'extracted'
    extracted.mkdir(exist_ok=True)
    with tarfile.open(archive, 'r:gz') as bundle:
        names = set()
        for member in bundle:
            path = Path(member.name)
            if (path.is_absolute() or '..' in path.parts or
                    not path.parts or path.parts[0] not in ('results', 'evidence') or
                    not (member.isfile() or member.isdir()) or
                    member.name in names):
                raise ValueError(f'unsafe raw archive member: {member.name}')
            names.add(member.name)
        bundle.extractall(extracted)
    print(json.dumps({'archive_sha256': record['archive_sha256'],
                      'archive_bytes': record['archive_bytes'],
                      'parts': len(record['parts']),
                      'extracted': str(extracted)}, sort_keys=True))


if __name__ == '__main__':
    main()
