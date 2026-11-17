#!/usr/bin/env python3
"""Restore absent records from frozen archive pointers via the canonical store.

The archive locator remains provenance, never an extraction instruction. Original
archive members are published by digest before this runs. Existing files and Git
source ownership remain authoritative; the subsequent ecbench audit is unchanged.
"""
import argparse
import json
from pathlib import Path, PurePosixPath
import re
import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / 'tools/artifacts'))
from artifacts import Config, Remote, ensure_cached, git, git_path_key, place_copy, rel


POINTER = re.compile(r'([0-9a-f]{64})  (\S+)  ([0-9]+) bytes  in (s3://[^\s()]+) \(([^\s()]+)\)\n?')


def pointer(text, destination):
    match = POINTER.fullmatch(text)
    if not match:
        raise ValueError('invalid records pointer')
    digest, logical, size, archive, member = match.groups()
    for value in (logical, member):
        path = PurePosixPath(value)
        if path.is_absolute() or '..' in path.parts or '\\' in value or path.name != 'records.jsonl':
            raise ValueError('unsafe records pointer path')
    parts = PurePosixPath(logical).parts
    if destination.parts[-len(parts):] != parts:
        raise ValueError('pointer names another records path')
    return digest, int(size)


def hydrate(root, sessions):
    root = root.resolve()
    cfg = Config.load(root)
    owned = {git_path_key(p) for p in git(root, 'ls-files', '-z').decode(errors='surrogateescape').split('\0')}
    remote = None
    restored = 0
    for name in sessions:
        path = PurePosixPath(name)
        if path.is_absolute() or '..' in path.parts or not path.parts or '\\' in name:
            raise ValueError('unsafe session path')
        directory = root / path
        if not directory.resolve().is_relative_to(root):
            raise ValueError('session directory outside repository')
        dest = directory / 'records.jsonl'
        source = directory / 'records.jsonl.pointer'
        if not source.exists():
            continue
        if source.is_symlink():
            raise ValueError('symbolic records pointer')
        digest, size = pointer(source.read_text(), path / 'records.jsonl')
        capsule = json.loads((directory / 'session.json').read_text())
        if capsule.get('schema') != 'ecbench.session/v1' or capsule.get('records_sha256') != digest:
            raise ValueError('pointer digest differs from session capsule')
        if dest.exists() or dest.is_symlink() or git_path_key(rel(root, dest)) in owned:
            continue
        if remote is None:
            remote = Remote(cfg)
        cached = ensure_cached(cfg, remote, digest, size)
        place_copy(cached, dest)
        restored += 1
    return restored


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('list', type=Path)
    args = parser.parse_args()
    root = Path(git(Path.cwd(), 'rev-parse', '--show-toplevel').decode().strip())
    count = hydrate(root, args.list.read_text().splitlines())
    print(f'Restored {count} archive-pointer records files; original capsules and pointers unchanged.')


if __name__ == '__main__':
    main()
