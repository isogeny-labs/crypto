#!/usr/bin/env python3
"""Bind both RunPod raw archives and pack their exact files with shared bases."""

import argparse
import gzip
import hashlib
import io
import json
import tarfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
RAW_SHA = (
    'd06daffcbdb35d4ba3fc74c94d6eb3c4fe206d5b428f53a3673fce9f46866aca',
    '0abc06f1353ebabb79e5009cd7d9021bd8f37132a721017d3734741999ff54bd',
)


def digest(path):
    h = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            h.update(block)
    return h.hexdigest()


def canonical(value):
    return (json.dumps(value, sort_keys=True, separators=(',', ':')) + '\n').encode()


def write_once(path, blob):
    if path.exists() and path.read_bytes() != blob:
        raise ValueError(f'sealed artifact differs: {path}')
    path.write_bytes(blob)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('raw0', type=Path)
    parser.add_argument('raw1', type=Path)
    args = parser.parse_args()
    raw = (args.raw0.resolve(), args.raw1.resolve())
    validated = json.loads((HERE / 'validated.json').read_text())
    selection = json.loads((HERE / 'selection.json').read_text())
    if (validated['evidence_status'] != 'PASS_INDEPENDENT_REPLAY' or
            selection['admission_status'] != 'PASS' or
            [item['raw_archive_sha256'] for item in validated['host_strata']]
            != list(RAW_SHA)):
        raise ValueError('two-stratum independent replay or raw hashes differ')
    for i in range(2):
        if (digest(raw[i]) != RAW_SHA[i] or
                (HERE / f'local_replay_s{i}.exit.txt').read_text().strip() != '0' or
                (HERE / f'local_replay_s{i}.stderr.txt').read_bytes() or
                json.loads((HERE / f'local_replay_s{i}.stdout.json').read_text())
                ['status'] != 'STRATUM_ADMITTED'):
            raise ValueError(f'stratum {i}: raw archive or replay differs')
    entries = {}
    blobs = {}
    base_by_sha = {}
    counts = []
    for i, archive in enumerate(raw):
        count = 0
        with tarfile.open(archive, 'r:gz') as bundle:
            for member in bundle:
                path = Path(member.name)
                if (path.is_absolute() or '..' in path.parts or not path.parts or
                        path.parts[0] != 'results' or
                        not (member.isfile() or member.isdir())):
                    raise ValueError(f'unsafe stratum {i} member: {member.name}')
                if member.isdir():
                    continue
                if member.name == 'results/analysis.json':
                    analysis_stream = bundle.extractfile(member)
                    if (analysis_stream is None or
                            json.loads(analysis_stream.read()) !=
                            json.loads((HERE / f'local_replay_s{i}.stdout.json').read_text())):
                        raise ValueError('producer and local replay analyses differ')
                stream = bundle.extractfile(member)
                if stream is None:
                    raise ValueError('unreadable raw archive member')
                data = stream.read()
                name = f'stratum{i}/{member.name}'
                if name in entries:
                    raise ValueError('duplicate raw member')
                sha = hashlib.sha256(data).hexdigest()
                blob = (base_by_sha.setdefault(sha, name)
                        if name.endswith('/base_dump.jsonl') else name)
                entries[name] = {'sha256': sha, 'bytes': len(data), 'blob': blob}
                if blob == name:
                    blobs[name] = data
                count += 1
        counts.append(count)
    if counts != [1384, 1384] or len(entries) != 2768:
        raise ValueError('two full raw strata required')
    manifest = {
        'schema': 'n83.root4-runpod-complete-dedup/v1',
        'raw_archive_sha256': list(RAW_SHA),
        'raw_file_count_by_stratum': counts,
        'provider_snapshot_sha256': digest(HERE / 'provider_snapshot.json'),
        'validated_sha256': digest(HERE / 'validated.json'),
        'selection_sha256': digest(HERE / 'selection.json'),
        'local_replay_sha256': [digest(HERE / f'local_replay_s{i}.stdout.json')
                                for i in range(2)],
        'entries': entries,
    }
    manifest_bytes = canonical(manifest)
    manifest_path = HERE / 'raw_dedup_manifest.json'
    write_once(manifest_path, manifest_bytes)
    archive_path = HERE / 'raw_dedup.tar.gz'
    if archive_path.exists():
        raise FileExistsError(archive_path)
    temporary = HERE / 'raw_dedup.tar.gz.part'
    if temporary.exists():
        raise FileExistsError(temporary)
    with temporary.open('wb') as sink:
        with gzip.GzipFile(fileobj=sink, mode='wb', filename='', mtime=0,
                           compresslevel=6) as gz:
            with tarfile.open(fileobj=gz, mode='w') as tar:
                for name in sorted(blobs):
                    info = tarfile.TarInfo(name)
                    info.size = len(blobs[name])
                    info.mode = 0o644
                    info.mtime = 0
                    tar.addfile(info, io.BytesIO(blobs[name]))
                info = tarfile.TarInfo('MANIFEST.json')
                info.size = len(manifest_bytes)
                info.mode = 0o644
                info.mtime = 0
                tar.addfile(info, io.BytesIO(manifest_bytes))
    temporary.replace(archive_path)
    receipt = {
        'schema': 'n83.root4-runpod-complete-dedup-receipt/v1',
        'archive_sha256': digest(archive_path),
        'archive_bytes': archive_path.stat().st_size,
        'manifest_sha256': digest(manifest_path),
        'file_count': len(entries), 'stored_blob_count': len(blobs),
        'unique_base_dump_count': len(base_by_sha),
    }
    write_once(HERE / 'raw_dedup_receipt.json', canonical(receipt))
    print(json.dumps(receipt, sort_keys=True))


if __name__ == '__main__':
    main()
