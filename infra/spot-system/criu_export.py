#!/usr/bin/env python3
"""Export explicitly authorized legacy memory checkpoints to private S3.

This is infrastructure orchestration, not an application-state implementation.
An index is committed only after every archive has been downloaded and hashed.
"""
import argparse
from datetime import datetime, timezone
import fcntl
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import uuid

from criu_local import atomic_json, digest, discover, validate_generation

BUCKET = 'spot-experiments-runtimebucket-g8acwaz3kewm'
REGION = 'us-east-2'


def aws(*args):
    return subprocess.check_output(['aws', '--region', REGION, *args], text=True)


def downloaded_digest(key):
    process = subprocess.Popen(['aws', '--region', REGION, 's3', 'cp',
        's3://' + BUCKET + '/' + key, '-', '--only-show-errors'], stdout=subprocess.PIPE)
    h = hashlib.sha256()
    for chunk in iter(lambda: process.stdout.read(1024 * 1024), b''):
        h.update(chunk)
    process.stdout.close()
    if process.wait() != 0:
        raise RuntimeError('S3 verification download failed')
    return h.hexdigest()


def upload_verified(path, key):
    expected = digest(path)
    aws('s3', 'cp', str(path), 's3://' + BUCKET + '/' + key,
        '--sse', 'AES256', '--metadata', 'sha256=' + expected, '--only-show-errors')
    head = json.loads(aws('s3api', 'head-object', '--bucket', BUCKET, '--key', key))
    if (head['ContentLength'] != path.stat().st_size or
            head.get('ServerSideEncryption') != 'AES256' or
            head.get('Metadata', {}).get('sha256') != expected or
            downloaded_digest(key) != expected):
        raise ValueError('Uploaded archive failed encryption, size or SHA-256 verification')
    return {'key': key, 'bytes': path.stat().st_size, 'sha256': expected,
            'server_side_encryption': 'AES256', 'download_sha256_verified': True}


def export(root, staging, authorized_jobs, authorized_generations=None):
    # Serialize exporters; snapshot selection/copy also takes the producer lock.
    # After copying, hashing/compression/upload do not delay later snapshots.
    with (root / '.export.lock').open('a') as export_lock:
        fcntl.flock(export_lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        name = datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%SZ-') + uuid.uuid4().hex[:12]
        batch = staging / name
        batch.mkdir(mode=0o700)
        selected = []
        with (root / '.producer.lock').open('a') as producer_lock:
            fcntl.flock(producer_lock, fcntl.LOCK_EX)
            registry = json.loads((root / 'registry.json').read_text())
            if registry['jobs'] != authorized_jobs:
                raise ValueError('Registered process identities differ from export authorization')
            for job in registry['jobs']:
                choice = discover(root, job['pid'])
                if not choice['selected']:
                    raise ValueError('No valid checkpoint for PID ' + str(job['pid']))
                source = Path(choice['selected'])
                if (authorized_generations is not None and
                        authorized_generations.get(str(job['pid'])) != source.name):
                    raise ValueError('New generation requires recurring memory-export authorization')
                target = batch / str(job['pid'])
                subprocess.run(['cp', '-a', '--reflink=auto', str(source), str(target)], check=True)
                selected.append((job, target, source.name))
        index = {'schema': 'spot.criu-export/v1', 'bucket': BUCKET, 'batch': name,
                 'application_restore_verified': False, 'objects': []}
        for job, directory, generation in selected:
            manifest = validate_generation(directory)
            if manifest['job'] != job:
                raise ValueError('Copied generation identity mismatch')
            archive = batch / (str(job['pid']) + '.tar.zst')
            subprocess.run(['tar', '--use-compress-program=zstd -T2 -1', '-cf', str(archive),
                            '-C', str(directory), '.'], check=True)
            record = upload_verified(archive, 'checkpoints/criu/' + name + '/' + archive.name)
            record.update(pid=job['pid'], generation=generation,
                          captured_utc=manifest['captured_utc'], kernel=manifest['kernel'])
            index['objects'].append(record)
            atomic_json(batch / 'progress.json', index)
            print(json.dumps({'pid': job['pid'], 'state': 'private-s3-download-verified',
                              'bytes': record['bytes']}), flush=True)
            # Preserve staging until the whole batch is committed. Failed batches
            # remain private for diagnosis rather than appearing recoverable.
        index['committed_utc'] = datetime.now(timezone.utc).isoformat()
        index['committed'] = True
        atomic_json(batch / 'index.json', index)
        index_record = upload_verified(batch / 'index.json', 'checkpoints/criu/' + name + '/index.json')
        previous = root / 'last-export.json'
        history = json.loads(previous.read_text()).get('indexes', []) if previous.exists() else []
        pointer = {'schema': 'spot.criu-export-pointer/v1', 'indexes': [index_record, *history[:2]]}
        atomic_json(batch / 'latest.json', pointer)
        upload_verified(batch / 'latest.json', 'checkpoints/criu/latest.json')
        index['indexes'] = pointer['indexes']
        atomic_json(root / 'last-export.json', index)
        # A completed batch is recoverable in S3 and local producer generations
        # remain; remove only the known temporary copies after all verification.
        shutil.rmtree(batch)
        print(json.dumps({'state': 'private-s3-batch-committed', 'batch': name,
                          'jobs': len(index['objects'])}), flush=True)
        return index


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, required=True)
    parser.add_argument('--staging', type=Path, required=True)
    parser.add_argument('--authorization', type=Path, required=True,
                        help='Root-owned 0600 JSON recording explicit memory-export authorization')
    args = parser.parse_args()
    if os.geteuid() != 0:
        parser.error('Run as root; memory snapshots must remain private')
    os.umask(0o077)
    for path in (args.root, args.staging):
        path.mkdir(mode=0o700, parents=True, exist_ok=True)
        if path.is_symlink() or path.stat().st_uid != 0 or path.stat().st_mode & 0o077:
            parser.error('Roots must be private root-owned real directories')
    config = args.authorization
    if config.is_symlink() or config.stat().st_uid != 0 or config.stat().st_mode & 0o077:
        parser.error('Authorization must be a private root-owned real file')
    auth = json.loads(config.read_text())
    if (auth.get('memory_export_authorized') is not True or auth.get('bucket') != BUCKET
            or not auth.get('jobs')):
        parser.error('Explicit memory-export authorization for this bucket is required')
    generations = None if auth.get('recurring_memory_export_authorized') is True else auth.get('generations')
    if generations is None and auth.get('recurring_memory_export_authorized') is not True:
        parser.error('One-time authorization must bind exact generations; recurring export requires separate permission')
    export(args.root, args.staging, auth['jobs'], generations)


if __name__ == '__main__':
    main()
