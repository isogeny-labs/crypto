#!/usr/bin/env python3
"""Preserve the independently replayed RunPod stratum0 with each base once."""

import argparse
import gzip
import hashlib
import io
import json
import tarfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
RAW_SHA = 'd06daffcbdb35d4ba3fc74c94d6eb3c4fe206d5b428f53a3673fce9f46866aca'


def digest_stream(stream):
    h = hashlib.sha256()
    for block in iter(lambda: stream.read(1024 * 1024), b''):
        h.update(block)
    return h.hexdigest()


def digest(path):
    with path.open('rb') as stream:
        return digest_stream(stream)


def canonical(value):
    return (json.dumps(value, sort_keys=True, separators=(',', ':')) + '\n').encode()


def write_once(path, data):
    if path.exists() and path.read_bytes() != data:
        raise ValueError(f'sealed artifact differs: {path}')
    path.write_bytes(data)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('raw_archive', type=Path)
    parser.add_argument('extracted_results', type=Path)
    args = parser.parse_args()
    raw = args.raw_archive.resolve()
    results = args.extracted_results.resolve()
    if digest(raw) != RAW_SHA:
        raise ValueError('original raw archive differs')
    analysis = json.loads((results / 'analysis.json').read_text())
    replay = json.loads((HERE / 'local_replay_s0.stdout.json').read_text())
    if (analysis != replay or analysis['status'] != 'STRATUM_ADMITTED' or
            analysis['stratum'] != 0 or
            (HERE / 'local_replay_s0.exit.txt').read_text().strip() != '0' or
            (HERE / 'local_replay_s0.stderr.txt').read_bytes()):
        raise ValueError('independent stratum0 replay differs')
    files = {}
    with tarfile.open(raw, 'r:gz') as bundle:
        for member in bundle:
            p = Path(member.name)
            if (p.is_absolute() or '..' in p.parts or not p.parts or
                    p.parts[0] != 'results' or
                    not (member.isfile() or member.isdir())):
                raise ValueError(f'unsafe original archive member: {member.name}')
            if member.isdir():
                continue
            local = results / Path(*p.parts[1:])
            stream = bundle.extractfile(member)
            if (stream is None or not local.is_file() or
                    digest_stream(stream) != digest(local)):
                raise ValueError(f'original archive differs: {member.name}')
            files[member.name] = local
    if len(files) != 1384:
        raise ValueError('stratum0 original file count differs')
    first_by_sha = {}
    entries = {}
    for name, path in sorted(files.items()):
        sha = digest(path)
        blob = first_by_sha.setdefault(sha, name) if name.endswith('/base_dump.jsonl') else name
        entries[name] = {'sha256': sha, 'bytes': path.stat().st_size, 'blob': blob}
    manifest = {'schema': 'n83.root4-runpod-partial-dedup/v1',
                'raw_archive_sha256': RAW_SHA,
                'provider_snapshot_sha256': digest(HERE / 'provider_snapshot.json'),
                'local_replay_sha256': digest(HERE / 'local_replay_s0.stdout.json'),
                'entries': entries}
    manifest_bytes = canonical(manifest)
    write_once(HERE / 'raw_s0_dedup_manifest.json', manifest_bytes)
    output = HERE / 'raw_s0_dedup.tar.gz'
    if output.exists() or output.with_name(output.name + '.part').exists():
        raise FileExistsError(output)
    temporary = output.with_name(output.name + '.part')
    blobs = {item['blob'] for item in entries.values()}
    with temporary.open('wb') as sink:
        with gzip.GzipFile(fileobj=sink, mode='wb', filename='', mtime=0, compresslevel=6) as gz:
            with tarfile.open(fileobj=gz, mode='w') as tar:
                for name in sorted(blobs):
                    path = files[name]
                    info = tarfile.TarInfo(name)
                    info.size = path.stat().st_size
                    info.mode = 0o644
                    info.mtime = 0
                    with path.open('rb') as stream:
                        tar.addfile(info, stream)
                info = tarfile.TarInfo('MANIFEST.json')
                info.size = len(manifest_bytes)
                info.mode = 0o644
                info.mtime = 0
                tar.addfile(info, io.BytesIO(manifest_bytes))
    temporary.replace(output)
    receipt = {'schema': 'n83.root4-runpod-partial-dedup-receipt/v1',
               'archive_sha256': digest(output), 'archive_bytes': output.stat().st_size,
               'manifest_sha256': digest(HERE / 'raw_s0_dedup_manifest.json'),
               'file_count': len(entries), 'stored_blob_count': len(blobs),
               'unique_base_dump_count': len(first_by_sha)}
    write_once(HERE / 'raw_s0_dedup_receipt.json', canonical(receipt))
    print(json.dumps(receipt, sort_keys=True))


if __name__ == '__main__':
    main()
