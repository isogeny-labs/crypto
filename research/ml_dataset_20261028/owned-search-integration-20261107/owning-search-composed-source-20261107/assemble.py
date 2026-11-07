#!/usr/bin/env python3
"""Materialize exact bound crate inputs and a reviewed overlay; never build/run."""
import argparse
import hashlib
import json
from pathlib import Path, PurePosixPath
import subprocess


def read_bound(root, name, digest):
    rel = PurePosixPath(name)
    if rel.is_absolute() or '..' in rel.parts or not rel.parts:
        raise ValueError('unsafe bound path: ' + name)
    p = root
    for part in rel.parts:
        p = p / part
        if p.is_symlink():
            raise ValueError('symlink bound path: ' + name)
    data = p.read_bytes()
    if hashlib.sha256(data).hexdigest() != digest:
        raise ValueError('hash mismatch: ' + name)
    return data


def assemble(base, packet, out):
    plan = json.loads((packet / 'ASSEMBLY.json').read_text())
    manifest_data = read_bound(packet, 'BASE_HASHES.json', plan['base_manifest_sha256'])
    manifest = json.loads(manifest_data)
    # Refuse overwrite even after an interrupted materialization. Preserve it.
    out.mkdir(exist_ok=False)
    receipts = []
    def write(rel, data, origin):
        target = out / rel
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(data)
        receipts.append({'path': rel, 'sha256': hashlib.sha256(data).hexdigest(),
                         'bytes': len(data), 'origin': origin})
    for rel, digest in sorted(manifest.items()):
        write(rel, read_bound(base, rel, digest), 'bound-base')
    for entry in plan['overlays']:
        rel = entry['target']
        if PurePosixPath(rel).is_absolute() or '..' in PurePosixPath(rel).parts:
            raise ValueError('unsafe overlay destination')
        write(rel, read_bound(packet, entry['source'], entry['sha256']), 'bound-overlay')
    for entry in plan['patches']:
        data = read_bound(packet, entry['source'], entry['sha256'])
        # All patch bytes are reviewed and hash-bound. No network or repository operation.
        for check in [True, False]:
            cmd = ['git', 'apply', '--whitespace=nowarn']
            if check: cmd.append('--check')
            subprocess.run(cmd + ['-'], cwd=out, input=data, check=True,
                           stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    final = {}
    for f in sorted(out.rglob('*')):
        if f.is_file():
            final[str(f.relative_to(out))] = hashlib.sha256(f.read_bytes()).hexdigest()
    for rel, digest in plan['expected_outputs'].items():
        if final.get(rel) != digest:
            raise ValueError('unexpected composed output: ' + rel)
    (packet / 'ASSEMBLED_HASHES.json').write_text(json.dumps(final, indent=2) + '\n')
    receipt = {'base_files': len(manifest), 'final_files': len(final),
               'inputs_verified': True, 'expected_outputs_verified': True,
               'compiled': False, 'executed': False,
               'base_manifest_sha256': plan['base_manifest_sha256']}
    (packet / 'ASSEMBLY_RECEIPT.json').write_text(json.dumps(receipt, indent=2) + '\n')
    print(json.dumps(receipt))

if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--base', type=Path, required=True)
    p.add_argument('--output', type=Path, required=True)
    a = p.parse_args()
    assemble(a.base.resolve(), Path(__file__).resolve().parent, a.output.resolve())
