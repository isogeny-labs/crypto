#!/usr/bin/env python3
"""Explicitly registered, leave-running local CRIU backups. No memory export.

This producer never restores, restarts or terminates an experiment. A registry
binds each PID to its start ticks and command digest to prevent PID reuse.
"""
import argparse
from datetime import datetime, timezone
import fcntl
import hashlib
import json
import os
from pathlib import Path
import platform
import shlex
import shutil
import stat
import subprocess
import sys
import time

EXECUTABLE = Path('/home/ec2-user/crypto/target/release/examples/factor_base_shape_ladder')
LIBRARIES = (Path('/usr/lib64'), Path('/lib64'))


def digest(path):
    h = hashlib.sha256()
    with path.open('rb') as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b''):
            h.update(chunk)
    return h.hexdigest()


def atomic_json(path, value):
    temporary = path.with_name(path.name + '.tmp')
    with open(temporary, 'w', opener=lambda p, f: os.open(p, f, 0o600)) as f:
        json.dump(value, f, sort_keys=True)
        f.write('\n')
        f.flush()
        os.fsync(f.fileno())
    os.replace(temporary, path)
    with open_directory(path.parent) as directory:
        os.fsync(directory)


def open_directory(path):
    class Directory:
        def __enter__(self):
            self.fd = os.open(path, os.O_DIRECTORY | os.O_RDONLY)
            return self.fd
        def __exit__(self, *_):
            os.close(self.fd)
    return Directory()


def identity(pid, proc=Path('/proc')):
    p = proc / str(pid)
    # Fields after the final ') ' begin at field 3; starttime is field 22.
    start = p.joinpath('stat').read_text().rsplit(') ', 1)[1].split()[19]
    executable = p.joinpath('exe').resolve(strict=True)
    if executable != EXECUTABLE:
        raise ValueError('Only the explicitly supported factor-base executable may be registered')
    return {'pid': int(pid), 'start_ticks': start, 'executable': str(executable),
            'command_sha256': hashlib.sha256(p.joinpath('cmdline').read_bytes()).hexdigest(),
            'cwd': str(p.joinpath('cwd').resolve(strict=True))}


def same_process(job):
    try:
        return identity(job['pid']) == job
    except (OSError, ValueError, IndexError):
        return False


def backing_paths(job, proc=Path('/proc')):
    p = proc / str(job['pid'])
    paths = {EXECUTABLE.resolve(strict=True)}
    cwd = Path(job['cwd'])
    for line in p.joinpath('maps').read_text().splitlines():
        fields = line.split(None, 5)
        if len(fields) != 6 or not fields[5].startswith('/'):
            continue
        path = Path(fields[5])
        resolved = path.resolve(strict=True)
        if resolved != EXECUTABLE and not any(resolved.is_relative_to(root.resolve()) for root in LIBRARIES):
            raise ValueError('Mapped backing file is outside the executable/library allowlist')
        paths.add(resolved)
    for fd in p.joinpath('fd').iterdir():
        path = Path(os.readlink(fd))
        if path == Path('/dev/null'):
            continue
        if fd.name not in {'1', '2'} or not path.is_absolute() or not path.resolve().is_relative_to(cwd):
            raise ValueError('Unsupported external descriptor; do not guess restore requirements')
        resolved = path.resolve(strict=True)
        if not resolved.is_file():
            raise ValueError('Output descriptor is not a regular file')
        paths.add(resolved)
    return sorted(str(path) for path in paths)


def capture(directory):
    if os.environ.get('CRTOOLS_SCRIPT_ACTION') != 'post-dump':
        return
    directories = {}
    for name in json.loads((directory / 'backing-paths.json').read_text()):
        source = Path(name)
        if not source.is_absolute() or '..' in source.parts or not stat.S_ISREG(source.stat().st_mode):
            raise ValueError('Invalid backing file')
        target = directory / 'filesystem' / source.relative_to('/')
        target.parent.mkdir(parents=True, exist_ok=True)
        for parent in source.parents:
            copied_parent = directory / 'filesystem' / parent.relative_to('/')
            copied_parent.mkdir(parents=True, exist_ok=True)
            info = parent.stat()
            shutil.copystat(parent, copied_parent)
            os.chown(copied_parent, info.st_uid, info.st_gid)
            directories[str(parent)] = {'mode': stat.S_IMODE(info.st_mode), 'uid': info.st_uid, 'gid': info.st_gid}
        shutil.copy2(source, target)
        info = source.stat()
        os.chown(target, info.st_uid, info.st_gid)
    atomic_json(directory / 'backing-directories.json', directories)


def validate_generation(directory):
    manifest = json.loads((directory / 'manifest.json').read_text())
    if manifest.get('schema') != 'spot.criu-local/v1' or not manifest.get('committed'):
        raise ValueError('Uncommitted CRIU generation')
    paths = [entry['path'] for entry in manifest['files']]
    if len(paths) != len(set(paths)) or 'images/inventory.img' not in paths or 'backing-directories.json' not in paths or not any(p.startswith('images/pages-') for p in paths):
        raise ValueError('Incomplete CRIU generation')
    for entry in manifest['files']:
        path = Path(entry['path'])
        if path.is_absolute() or '..' in path.parts:
            raise ValueError('Invalid checkpoint path')
        actual = directory / path
        if any(p.is_symlink() for p in [actual, *actual.parents] if p != directory.parent) or not actual.resolve().is_relative_to(directory.resolve()):
            raise ValueError('Checkpoint symlink or escaped path')
        if not actual.is_file() or actual.stat().st_size != entry['bytes'] or digest(actual) != entry['sha256']:
            raise ValueError('Checkpoint integrity check failed')
    return manifest


def discover(root, pid):
    registry = json.loads((root / 'registry.json').read_text())
    jobs = [job for job in registry['jobs'] if job['pid'] == pid]
    if len(jobs) != 1:
        raise ValueError('PID must identify one registered job')
    rejected = []
    for directory in sorted((root / str(pid)).glob('gen-*'), reverse=True):
        try:
            manifest = validate_generation(directory)
            if manifest['job'] != jobs[0]:
                raise ValueError('Checkpoint belongs to another process identity')
            return {'pid': pid, 'selected': str(directory), 'integrity': 'verified', 'rejected': rejected,
                    'application_restore_verified': False}
        except (OSError, ValueError, KeyError, TypeError) as error:
            rejected.append({'generation': directory.name, 'reason': str(error)})
    return {'pid': pid, 'selected': None, 'rejected': rejected, 'application_restore_verified': False}


def snapshot(root, criu):
    registry = json.loads((root / 'registry.json').read_text())
    with (root / '.producer.lock').open('a') as lock:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        for job in registry['jobs']:
            if not same_process(job):
                print(json.dumps({'pid': job['pid'], 'state': 'not-the-registered-live-process'}))
                continue
            parent = root / str(job['pid'])
            parent.mkdir(exist_ok=True, mode=0o700)
            directory = parent / ('gen-' + str(time.time_ns()))
            directory.mkdir(mode=0o700)
            (directory / 'images').mkdir(mode=0o700)
            atomic_json(directory / 'backing-paths.json', backing_paths(job))
            action = directory / 'capture.sh'
            action.write_text('#!/bin/bash\nset -eu\nexec /usr/bin/python3 ' +
                shlex.quote(str(Path(__file__).resolve())) + ' post-dump --root ' + shlex.quote(str(directory)) + '\n')
            action.chmod(0o700)
            subprocess.run([str(criu), 'dump', '-t', str(job['pid']), '-D', str(directory / 'images'),
                '--leave-running', '--shell-job', '--cpu-cap=all', '--action-script', str(action), '-o', 'dump.log', '-v2'], check=True)
            files = []
            for path in sorted(directory.rglob('*')):
                if path.is_file() and not path.is_symlink() and path.name not in {'capture.sh', 'dump.log'}:
                    with path.open('rb') as f:
                        os.fsync(f.fileno())
                    files.append({'path': str(path.relative_to(directory)), 'bytes': path.stat().st_size,
                                  'sha256': digest(path)})
            atomic_json(directory / 'manifest.json', {'schema': 'spot.criu-local/v1', 'committed': True,
                'job': job, 'kernel': platform.release(), 'criu_version': '4.2.1',
                'captured_utc': datetime.now(timezone.utc).isoformat(), 'files': files,
                'source_still_running': same_process(job), 'off_host_export': False})
            committed = sorted(p for p in parent.iterdir() if (p / 'manifest.json').is_file())
            for old in committed[:-2]:
                shutil.rmtree(old)
            print(json.dumps({'pid': job['pid'], 'state': 'local-checkpoint-committed', 'generation': directory.name}))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=['register', 'snapshot', 'list', 'verify', 'discover', 'post-dump'])
    parser.add_argument('--root', type=Path, required=True)
    parser.add_argument('--pid', type=int, action='append', default=[])
    parser.add_argument('--criu', type=Path, default=Path('/home/ec2-user/.local/bin/criu-4.2.1'))
    args = parser.parse_args()
    if os.geteuid() != 0:
        parser.error('Root is required; process memory images must stay private')
    os.umask(0o077)
    if args.action == 'post-dump':
        capture(args.root)
        return
    args.root.mkdir(mode=0o700, parents=True, exist_ok=True)
    if args.root.is_symlink() or args.root.stat().st_uid != 0 or args.root.stat().st_mode & 0o077:
        parser.error('Checkpoint root must be a private real directory')
    if args.action == 'register':
        if not args.pid or (args.root / 'registry.json').exists():
            parser.error('Register explicit PIDs once; use a new root for another set')
        atomic_json(args.root / 'registry.json', {'schema': 'spot.criu-registry/v1',
                    'jobs': [identity(pid) for pid in args.pid], 'off_host_export': False})
    elif args.action == 'snapshot':
        snapshot(args.root, args.criu)
    elif args.action == 'verify':
        print(json.dumps({'pid': validate_generation(args.root)['job']['pid'], 'integrity': 'verified'}))
    elif args.action == 'discover':
        if len(args.pid) != 1:
            parser.error('Discover requires one --pid')
        result = discover(args.root, args.pid[0])
        print(json.dumps(result, indent=2))
        if not result['selected']:
            sys.exit(2)
    else:
        registry = json.loads((args.root / 'registry.json').read_text())
        print(json.dumps({'jobs': [{'pid': j['pid'], 'same_live_process': same_process(j),
            'generations': [str(p) for p in sorted((args.root / str(j['pid'])).glob('gen-*')) if (p/'manifest.json').is_file()]}
            for j in registry['jobs']], 'off_host_export': False}, indent=2))


if __name__ == '__main__':
    main()
