#!/usr/bin/env python3
"""Restore and hash-check both RunPod raw strata from the deduplicated archive."""

import argparse
import hashlib
import json
import os
import shutil
import tarfile
from pathlib import Path, PurePosixPath

HERE = Path(__file__).resolve().parent


def digest(path):
    h = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            h.update(block)
    return h.hexdigest()


def safe(name):
    p = PurePosixPath(name)
    if p.is_absolute() or not p.parts or any(x in ('', '.', '..') for x in p.parts):
        raise ValueError(f'unsafe archive path: {name}')
    return Path(*p.parts)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('output_dir', type=Path)
    args = parser.parse_args()
    output = args.output_dir.resolve()
    if output.exists():
        raise ValueError(f'output exists: {output}')
    manifest_path = HERE / 'raw_dedup_manifest.json'
    archive = HERE / 'raw_dedup.tar.gz'
    receipt = json.loads((HERE / 'raw_dedup_receipt.json').read_text())
    manifest = json.loads(manifest_path.read_text())
    if (manifest['schema'] != 'n83.root4-runpod-complete-dedup/v1' or
            receipt['schema'] != 'n83.root4-runpod-complete-dedup-receipt/v1' or
            manifest['raw_archive_sha256'] != [
                'd06daffcbdb35d4ba3fc74c94d6eb3c4fe206d5b428f53a3673fce9f46866aca',
                '0abc06f1353ebabb79e5009cd7d9021bd8f37132a721017d3734741999ff54bd'] or
            manifest['raw_file_count_by_stratum'] != [1384, 1384] or
            digest(HERE / 'provider_snapshot.json') != manifest['provider_snapshot_sha256'] or
            digest(HERE / 'validated.json') != manifest['validated_sha256'] or
            digest(HERE / 'selection.json') != manifest['selection_sha256'] or
            [digest(HERE / f'local_replay_s{i}.stdout.json') for i in range(2)]
            != manifest['local_replay_sha256'] or
            digest(manifest_path) != receipt['manifest_sha256'] or
            digest(archive) != receipt['archive_sha256'] or
            archive.stat().st_size != receipt['archive_bytes']):
        raise ValueError('archive identity differs')
    entries = manifest['entries']
    blobs = {item['blob'] for item in entries.values()}
    for name, item in entries.items():
        safe(name)
        safe(item['blob'])
        if item['blob'] not in entries:
            raise ValueError(f'missing canonical blob: {name}')
        canonical = entries[item['blob']]
        if (canonical['blob'] != item['blob'] or
                canonical['sha256'] != item['sha256'] or
                canonical['bytes'] != item['bytes']):
            raise ValueError(f'duplicate binding differs: {name}')
    output.mkdir(parents=True)
    with tarfile.open(archive, 'r:gz') as bundle:
        all_members = bundle.getmembers()
        members = {m.name: m for m in all_members}
        if len(members) != len(all_members) or set(members) != blobs | {'MANIFEST.json'}:
            raise ValueError('stored archive members differ')
        embedded = bundle.extractfile(members['MANIFEST.json'])
        if embedded is None or embedded.read() != manifest_path.read_bytes():
            raise ValueError('embedded manifest differs')
        for name in sorted(blobs):
            member = members[name]
            if not member.isfile() or member.size != entries[name]['bytes']:
                raise ValueError(f'stored blob differs: {name}')
            stream = bundle.extractfile(member)
            if stream is None:
                raise ValueError(f'unreadable blob: {name}')
            dest = output / safe(name)
            dest.parent.mkdir(parents=True, exist_ok=True)
            with stream, dest.open('wb') as sink:
                shutil.copyfileobj(stream, sink, 1024 * 1024)
            if digest(dest) != entries[name]['sha256']:
                raise ValueError(f'stored blob hash differs: {name}')
    for name, item in sorted(entries.items()):
        if name != item['blob']:
            dest = output / safe(name)
            dest.parent.mkdir(parents=True, exist_ok=True)
            os.link(output / safe(item['blob']), dest)
    for name, item in entries.items():
        dest = output / safe(name)
        if not dest.is_file() or dest.stat().st_size != item['bytes'] or digest(dest) != item['sha256']:
            raise ValueError(f'restored file differs: {name}')
    if (len(entries) != receipt['file_count'] or len(entries) != 2768 or
            len(blobs) != receipt['stored_blob_count'] or
            len({item['sha256'] for name, item in entries.items()
                 if name.endswith('/base_dump.jsonl')}) != receipt['unique_base_dump_count']):
        raise ValueError('archive counts differ')
    print(json.dumps({'status': 'PASS', 'file_count': len(entries),
                      'stored_blob_count': len(blobs)}, sort_keys=True))


if __name__ == '__main__':
    main()
