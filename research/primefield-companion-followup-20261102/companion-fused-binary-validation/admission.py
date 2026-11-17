"""Verify exact local locks and their published GitHub representation."""
from pathlib import Path, PurePosixPath
import base64
import hashlib
import json
import re
import shutil
import subprocess
import time


def sha(path):
    h = hashlib.sha256()
    with Path(path).open('rb') as f:
        for block in iter(lambda: f.read(1048576), b''): h.update(block)
    return h.hexdigest()


def safe_relative(path):
    p = PurePosixPath(path)
    if not path or p.is_absolute() or '..' in p.parts:
        raise ValueError('unsafe publication path')
    return path


def published(publication, source_lock, input_lock, protocol_bytes, root, deadline, fetch=None):
    if publication['repository'] != 'isogeny-labs/crypto':
        raise ValueError('unexpected publication repository')
    commit = publication['commit']
    if re.fullmatch('[0-9a-f]{40}', commit) is None:
        raise ValueError('immutable publication commit required')
    if set(publication['files']) != set(source_lock):
        raise ValueError('published source scope differs from locked source scope')
    if fetch is None:
        gh = shutil.which('gh')
        if gh is None: raise RuntimeError('gh is required for read-only publication verification')
        def fetch(path):
            remaining = deadline - time.monotonic()
            if remaining <= 0: raise TimeoutError('admission deadline exhausted')
            request = 'repos/isogeny-labs/crypto/contents/' + safe_relative(path) + '?ref=' + commit
            raw = subprocess.check_output([gh, 'api', request], timeout=min(30, remaining))
            obj = json.loads(raw)
            if obj.get('encoding') != 'base64' or obj.get('type') != 'file':
                raise ValueError('publication is not an inline GitHub file')
            return base64.b64decode(obj['content'])
    cache = {}
    def get(path):
        safe_relative(path)
        if path not in cache: cache[path] = fetch(path)
        return cache[path]
    if json.loads(get(publication['source_manifest_path'])) != source_lock:
        raise ValueError('published source manifest differs')
    if json.loads(get(publication['input_manifest_path'])) != input_lock:
        raise ValueError('published input manifest differs')
    if get(publication['protocol_path']) != protocol_bytes:
        raise ValueError('published frozen protocol differs')
    for local, expected in source_lock.items():
        safe_relative(local)
        entry = publication['files'][local]
        if set(entry) == {'path'}:
            if hashlib.sha256(get(entry['path'])).hexdigest() != expected:
                raise ValueError('published source differs: ' + local)
        elif set(entry) == {'manifest_path', 'artifact'} and not local.endswith('.py'):
            manifest = json.loads(get(entry['manifest_path']))
            if manifest.get('schema') != 'artifacts.manifest/v1':
                raise ValueError('artifact manifest schema')
            value = manifest['files'][entry['artifact']]
            if value['sha256'] != expected or value['bytes'] != (root/local).stat().st_size:
                raise ValueError('published artifact binding differs: ' + local)
        else:
            raise ValueError('source scripts must be published directly')
    return dict(repository=publication['repository'], commit=commit,
                verified_source_entries=len(source_lock), verified_input_manifest_entries=len(input_lock),
                fetched_files=len(cache))
