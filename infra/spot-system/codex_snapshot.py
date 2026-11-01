"""Checksummed Codex recovery generations; live SQLite stays on local storage.

SQLite's backup API includes committed WAL data. For migration, quiesce the
source before the final generation. Periodic online backups can lose work since
the last committed generation and are not a snapshot of running tool processes.
Authentication export is opt-in and must be authorized by the operator.
"""
import argparse
from contextlib import closing
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import sqlite3
import tempfile
import time
import uuid

from criu_local import atomic_json, digest

TREES = ('sessions', 'archived_sessions', 'attachments', 'skills', 'plugins', 'packages',
         'rules', 'visualizations')
FILES = ('config.toml', 'installation_id', 'session_index.jsonl', '.sandbox_migration', 'hooks.json')


def safe_relative(name):
    path = Path(name)
    if path.is_absolute() or '..' in path.parts or not path.parts:
        raise ValueError('Unsafe snapshot path')
    return path


def load_generation(root):
    pointer = json.loads((root / 'current.json').read_text())
    generation = safe_relative(pointer['generation'])
    if len(generation.parts) != 1 or not generation.name.startswith('gen-'):
        raise ValueError('Unsafe generation name')
    directory = root / generation
    if directory.is_symlink() or digest(directory / 'manifest.json') != pointer['sha256']:
        raise ValueError('Snapshot manifest integrity mismatch')
    manifest = json.loads((directory / 'manifest.json').read_text())
    if manifest.get('schema') != 'spot.codex-snapshot/v1':
        raise ValueError('Unsupported snapshot schema')
    return directory, manifest


def snapshot(home, root, threads, include_auth=False, retain=3):
    if retain < 2:
        raise ValueError('Keep at least two committed recovery generations')
    root.mkdir(parents=True, exist_ok=True, mode=0o700)
    if root.is_symlink() or root.stat().st_mode & 0o077:
        raise ValueError('Snapshot root must be private')
    previous, old_files = None, {}
    if (root / 'current.json').exists():
        previous, manifest = load_generation(root)
        old_files = manifest['files']
    generation = 'gen-'+str(time.time_ns())+'-'+uuid.uuid4().hex[:8]
    staging = root / ('.'+generation)
    staging.mkdir(mode=0o700)
    files = {}
    selected = [home / name for name in FILES if (home / name).is_file()]
    if include_auth and (home / 'auth.json').exists():
        selected.append(home / 'auth.json')
    for tree in TREES:
        if (home / tree).is_symlink():
            raise ValueError('Snapshot source tree is a symlink: '+tree)
        selected.extend(p for p in (home / tree).rglob('*') if p.is_file() or p.is_symlink())
    selected.extend(p for p in home.glob('*.sqlite') if not p.name.startswith('logs_'))
    for source in sorted(set(selected)):
        name = source.relative_to(home).as_posix()
        destination = staging / name
        destination.parent.mkdir(parents=True, exist_ok=True, mode=0o700)
        if source.is_symlink():
            resolved = source.resolve(strict=True)
            if not resolved.is_relative_to(home.resolve()):
                raise ValueError('Snapshot symlink leaves Codex home: '+name)
            target = os.path.relpath(resolved, source.parent.resolve())
            destination.symlink_to(target)
            files[name] = {'symlink': target}
            continue
        if source.suffix == '.sqlite':
            # SQLite backup inherits the source journal mode. Keep its database
            # and WAL operations on local storage, then publish a closed,
            # standalone file to EFS; NFS is not a live SQLite/WAL destination.
            with tempfile.TemporaryDirectory(prefix='.codex-backup-', dir=home.parent) as temporary:
                backup = Path(temporary) / source.name
                with closing(sqlite3.connect('file:'+str(source)+'?mode=ro', uri=True, timeout=15)) as src:
                    with closing(sqlite3.connect(backup)) as dst:
                        src.backup(dst)
                        if dst.execute('PRAGMA quick_check').fetchone()[0] != 'ok':
                            raise ValueError('SQLite snapshot integrity check failed: '+name)
                        dst.execute('PRAGMA journal_mode=DELETE')
                shutil.copyfile(backup, destination)
        else:
            # A changed file is copied into this generation, never over a hard
            # link from an earlier immutable generation.
            sha = digest(source)
            mode = 0o700 if source.stat().st_mode & 0o111 else 0o600
            if (previous and old_files.get(name, {}).get('sha256') == sha
                    and old_files[name].get('mode') == mode):
                old = previous / name
                if old.is_symlink() or digest(old) != sha:
                    raise ValueError('Previous snapshot file integrity mismatch')
                os.link(old, destination)
            else:
                shutil.copyfile(source, destination)
        destination.chmod(0o700 if source.stat().st_mode & 0o111 else 0o600)
        with destination.open('rb') as f:
            os.fsync(f.fileno())
        files[name] = {'sha256': digest(destination), 'bytes': destination.stat().st_size,
                       'mode': destination.stat().st_mode & 0o777}
    manifest = {'schema': 'spot.codex-snapshot/v1', 'captured_utc': time.time(),
                'auth_included': 'auth.json' in files, 'threads': threads, 'files': files}
    atomic_json(staging / 'manifest.json', manifest)
    final = root / generation
    os.rename(staging, final)
    atomic_json(root / 'current.json', {'generation': generation, 'sha256': digest(final / 'manifest.json')})
    # SQLite backups are full copies. Bound storage growth only after committing
    # the new pointer, and retain the current generation even if the clock moved.
    generations = sorted((p for p in root.iterdir() if p.is_dir() and not p.is_symlink()
                          and re.fullmatch(r'gen-[0-9]+-[0-9a-f]{8}', p.name)
                          and p != final), key=lambda p: p.name, reverse=True)
    prune_errors = 0
    for old in generations[retain - 1:]:
        try:
            shutil.rmtree(old)
        except OSError:
            prune_errors += 1
    return {'generation': generation, 'files': len(files), 'threads': len(threads),
            'auth_included': manifest['auth_included'], 'prune_errors': prune_errors}


def restore(root, home):
    if home.is_symlink() or (home.exists() and any(home.iterdir())):
        raise ValueError('Refuse to overwrite existing Codex state')
    directory, manifest = load_generation(root)
    # Validate the entire generation before making any destination changes.
    for name, metadata in manifest['files'].items():
        path = directory / safe_relative(name)
        if 'symlink' in metadata:
            if (not path.is_symlink() or os.readlink(path) != metadata['symlink']
                    or not path.resolve().is_relative_to(directory.resolve())):
                raise ValueError('Snapshot symlink integrity mismatch: '+name)
            continue
        if (path.is_symlink() or not path.resolve().is_relative_to(directory.resolve())
                or digest(path) != metadata['sha256'] or path.stat().st_size != metadata['bytes']):
            raise ValueError('Snapshot file integrity mismatch: '+name)
    home.mkdir(parents=True, exist_ok=True, mode=0o700)
    for name, metadata in manifest['files'].items():
        target = home / name
        target.parent.mkdir(parents=True, exist_ok=True, mode=0o700)
        if 'symlink' in metadata:
            target.symlink_to(metadata['symlink'])
            continue
        shutil.copyfile(directory / name, target)
        target.chmod(metadata['mode'] & 0o700)
    atomic_json(home / 'spot-restore.json', {'generation': directory.name, 'restored_utc': time.time()})
    return manifest['threads']


if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('action', choices=['snapshot', 'restore'])
    p.add_argument('--home', type=Path, required=True)
    p.add_argument('--root', type=Path, required=True)
    p.add_argument('--threads', type=Path)
    p.add_argument('--include-auth', action='store_true')
    a = p.parse_args()
    os.umask(0o077)
    if a.action == 'snapshot':
        if not a.threads:
            p.error('Snapshot requires an explicit thread inventory')
        print(json.dumps(snapshot(a.home, a.root, json.loads(a.threads.read_text()), a.include_auth)))
    else:
        print(json.dumps({'threads': restore(a.root, a.home)}))
