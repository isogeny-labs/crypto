"""Check imported bytes and frozen source locks; never execute kernels."""
from pathlib import Path
import hashlib
import json


def digest(path):
    h = hashlib.sha256()
    with path.open('rb') as stream:
        for chunk in iter(lambda: stream.read(1048576), b''):
            h.update(chunk)
    return h.hexdigest()


def main():
    root = Path(__file__).resolve().parent
    manifest = json.loads((root / 'publication-manifest.json').read_text())
    entries = list(manifest['entries'])
    for component in ('companion-generator-compression', 'companion-rank-solver',
                      'companion-cached-pivot', 'companion-border-cache', 'companion-compact-lift', 'companion-shared-border', 'companion-metric-pivot', 'companion-metric-solver', 'companion-paired-transport', 'companion-metric-views', 'companion-zero-prefix', 'companion-zero-delete'):
        extension = root / component / 'publication.json'
        if extension.exists():
            added = json.loads(extension.read_text())
            for entry in added['entries']:
                entries.append(dict(entry, path=component + '/' + entry['path']))
    seen = set()
    for entry in entries:
        rel = Path(entry['path'])
        if rel.is_absolute() or '..' in rel.parts or str(rel) in seen:
            raise ValueError('unsafe or duplicate publication path')
        seen.add(str(rel))
        path = root / rel
        if path.is_symlink() or not path.is_file():
            raise ValueError('hydrate missing original file: ' + str(rel))
        if path.stat().st_size != entry['bytes'] or digest(path) != entry['sha256']:
            raise ValueError('original bytes differ: ' + str(rel))
    checked = 0
    for component in ('companion-fused-binary-validation',
                      'companion-fused-validation',
                      'companion-schur-fused-validation'):
        locks = json.loads((root / component / 'source-lock.json').read_text())
        for rel, expected in locks.items():
            if rel not in seen or digest(root / rel) != expected:
                raise ValueError('frozen source lock differs: ' + rel)
            checked += 1
    print(json.dumps({'files_verified': len(entries),
                      'source_locks_verified': checked,
                      'numerical_trials': 0}))


if __name__ == '__main__':
    main()
