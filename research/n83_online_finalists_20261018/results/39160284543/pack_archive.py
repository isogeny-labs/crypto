#!/usr/bin/env python3
"""Bind and deduplicate the four provider ZIPs for the N83 six-arm result."""

import argparse
import gzip
import hashlib
import io
import json
import tarfile
import zipfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
RUN_ID = 39160284543
HEAD_SHA = 'f152eb7d5e5de660fb16902d2abfe7cbd3ff331a'


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
    parser.add_argument('artifact_root', type=Path)
    args = parser.parse_args()
    root = args.artifact_root.resolve()
    receipts = json.loads((root / 'provider_zip_sha256.json').read_text())
    validated = json.loads((HERE / 'validated_rows.json').read_text())
    selection = json.loads((HERE / 'selection.json').read_text())
    if (receipts['run_id'] != RUN_ID or len(receipts['archives']) != 4 or
            validated['run_id'] != RUN_ID or validated['head_sha'] != HEAD_SHA or
            validated['evidence_status'] != 'PASS_INDEPENDENT_REPLAY' or
            selection['admission_status'] != 'PASS'):
        raise ValueError('upstream result is not admitted')
    files = {}
    providers = []
    for i, item in enumerate(sorted(receipts['archives'], key=lambda x: x['stratum'])):
        if item['stratum'] != i:
            raise ValueError('provider stratum coverage differs')
        archive = Path(item['path'])
        if archive.stat().st_size != item['bytes'] or digest(archive) != item['sha256']:
            raise ValueError(f'provider ZIP {i} differs')
        parent = root / f'stratum{i}'
        results = parent / 'results'
        with zipfile.ZipFile(archive) as z:
            members = [x for x in z.infolist() if not x.is_dir()]
            if len(members) != item['member_count']:
                raise ValueError(f'provider ZIP {i} member count differs')
            for member in members:
                relative = Path(member.filename)
                if relative.is_absolute() or '..' in relative.parts:
                    raise ValueError('unsafe provider ZIP member')
                local = parent / relative
                with z.open(member) as stream:
                    zip_sha = digest_stream(stream)
                if not local.is_file() or digest(local) != zip_sha:
                    raise ValueError(f'provider ZIP {i} extraction differs: {relative}')
                files[f'stratum{i}/{relative.as_posix()}'] = local
        log = parent / 'local_replay.stdout.txt'
        if (int((parent / 'local_replay.exit_status.txt').read_text()) != 0 or
                (parent / 'local_replay.stderr.txt').read_bytes() or
                json.loads(log.read_text()) != json.loads((results / 'analysis.json').read_text())):
            raise ValueError(f'independent replay {i} differs')
        for name in ('local_replay.stdout.txt', 'local_replay.stderr.txt',
                     'local_replay.exit_status.txt'):
            files[f'stratum{i}/{name}'] = parent / name
        providers.append({'stratum': i, 'artifact_id': item.get('artifact_id'),
                          'zip_sha256': item['sha256'], 'zip_bytes': item['bytes'],
                          'zip_member_count': item['member_count']})
    first_by_sha = {}
    entries = {}
    for name, path in sorted(files.items()):
        sha = digest(path)
        blob = first_by_sha.setdefault(sha, name) if name.endswith('/base_dump.jsonl') else name
        entries[name] = {'sha256': sha, 'bytes': path.stat().st_size, 'blob': blob}
    manifest = {'schema': 'n83.online-finalists-dedup-archive/v1',
                'run_id': RUN_ID, 'head_sha': HEAD_SHA,
                'validated_sha256': digest(HERE / 'validated_rows.json'),
                'selection_sha256': digest(HERE / 'selection.json'),
                'providers': providers, 'entries': entries}
    manifest_bytes = canonical(manifest)
    write_once(HERE / 'raw_dedup_manifest.json', manifest_bytes)
    archive = HERE / 'raw_dedup.tar.gz'
    if archive.exists():
        raise FileExistsError(archive)
    temporary = archive.with_suffix('.tar.gz.part')
    if temporary.exists():
        raise FileExistsError(temporary)
    blobs = {item['blob'] for item in entries.values()}
    with temporary.open('wb') as output:
        with gzip.GzipFile(fileobj=output, mode='wb', filename='', mtime=0, compresslevel=6) as compressed:
            with tarfile.open(fileobj=compressed, mode='w') as tar:
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
    temporary.replace(archive)
    receipt = {'schema': 'n83.online-finalists-dedup-receipt/v1',
               'archive_sha256': digest(archive), 'archive_bytes': archive.stat().st_size,
               'manifest_sha256': digest(HERE / 'raw_dedup_manifest.json'),
               'file_count': len(entries), 'stored_blob_count': len(blobs),
               'unique_base_dump_count': len(first_by_sha)}
    write_once(HERE / 'raw_dedup_receipt.json', canonical(receipt))
    print(json.dumps(receipt, sort_keys=True))


if __name__ == '__main__':
    main()
