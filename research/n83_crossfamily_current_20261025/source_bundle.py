#!/usr/bin/env python3
"""Build and replay a private, exact-commit source archive for the N83 Pods."""

import argparse
import gzip
import hashlib
import json
from pathlib import Path, PurePosixPath
import shutil
import subprocess
import tarfile

from research.n83_crossfamily_current_20261025 import arms, run


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
BASE_PATHS = (
    'Cargo.toml', 'Cargo.lock', 'src', 'examples',
    'docs/curves/registry.json', 'docs/curves/traits.json',
    'docs/ic/boundary_targets.json', 'docs/ic/calibration.json',
    'docs/ecbench/schema.sql',
    'docs/identity-certificates/schoenhage-10mult.json',
    'tests/fixtures/ecbench_manifest_small.json',
)
RESEARCH_DIRS = (
    'n83_cold_factor_base_20261016',
    'n83_compact4_crosslayout_20261016',
    'n83_compact4_crosslayout_results_20261016',
    'n83_crossfamily_current_20261025',
    'n83_dense_root_20261013',
    'n83_e2e_factor_base_20261016',
    'n83_e2e_final_holdout_20261016',
    'n83_e2e_finegrid_20261016',
    'n83_e2e_holdout_20261016',
    'n83_factor_base_100_results_20261014',
    'n83_highk_beyond4405_20261021',
    'n83_indexed5_20261013',
    'n83_k3072_current_source_20261017',
    'n83_k3300_current_source_prefix_20261025',
    'n83_k3596_current_source_full_20261025',
    'n83_nearby_full_grid_20261024',
    'n83_nearby_grid_confirm16_20261024',
    'n83_nearby_grid_tier1_20261024',
    'n83_online_finalists_20261018',
    'n83_root11_dynamic_20261016',
    'n83_root4_20261014',
    'n83_smallk_challenge_20261017',
)
PATHS = BASE_PATHS + tuple('research/' + name for name in RESEARCH_DIRS)


def digest(path):
    value = hashlib.sha256()
    with path.open('rb') as source:
        for block in iter(lambda: source.read(1024 * 1024), b''):
            value.update(block)
    return value.hexdigest()


def safe_name(name):
    path = PurePosixPath(name)
    return (name and not name.startswith('/') and
            all(part not in ('', '.', '..') for part in path.parts))


def members(archive):
    result = {}
    with tarfile.open(archive, 'r:gz') as source:
        for item in source:
            if not safe_name(item.name):
                raise ValueError('unsafe source archive path')
            if item.isdir():
                continue
            if not item.isfile() or item.name in result:
                raise ValueError('unsupported or repeated source archive member')
            data = source.extractfile(item)
            if data is None:
                raise ValueError('unreadable source archive member')
            result[item.name] = hashlib.sha256(data.read()).hexdigest()
    return result


def verify(archive, manifest_path, root=None):
    manifest = json.loads(manifest_path.read_text())
    if (manifest.get('schema') != 'n83.crossfamily-source-bundle/v1' or
            not isinstance(manifest.get('source_head_sha'), str) or
            len(manifest['source_head_sha']) != 40 or
            manifest.get('source_archive_sha256') != digest(archive) or
            manifest.get('paths') != list(PATHS) or
            not isinstance(manifest.get('members_sha256'), dict) or
            members(archive) != manifest['members_sha256']):
        raise ValueError('source archive or manifest differs')
    if root is not None:
        for name, expected in manifest['members_sha256'].items():
            path = root / name
            if not path.is_file() or path.is_symlink() or digest(path) != expected:
                raise ValueError(f'extracted source differs: {name}')
    return manifest


def build(output):
    if output.exists():
        raise ValueError('source bundle output already exists')
    head = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT,
                                   text=True).strip()
    if subprocess.check_output(['git', 'status', '--porcelain'], cwd=ROOT,
                               text=True):
        raise ValueError('source checkout must be clean')
    run.fixture()
    arms.choose()
    expected, replayed = run.generator.produce()
    if any((HERE / name).read_bytes() != blob for name, blob in expected.items()):
        raise ValueError('historical fixture replay differs')
    if replayed != json.loads((HERE / 'fixture_manifest.json').read_text()):
        raise ValueError('historical manifest replay differs')
    output.mkdir(parents=True)
    archive = output / 'source.tar.gz'
    command = ['git', 'archive', '--format=tar', 'HEAD', '--', *PATHS]
    producer = subprocess.Popen(command, cwd=ROOT, stdout=subprocess.PIPE)
    try:
        with archive.open('wb') as destination:
            with gzip.GzipFile(fileobj=destination, mode='wb', filename='',
                               mtime=0, compresslevel=6) as compressed:
                shutil.copyfileobj(producer.stdout, compressed)
        if producer.wait() != 0:
            raise ValueError('Git source archive failed')
    finally:
        producer.stdout.close()
    record = {
        'schema': 'n83.crossfamily-source-bundle/v1',
        'source_head_sha': head,
        'source_archive_sha256': digest(archive),
        'paths': list(PATHS),
        'members_sha256': members(archive),
    }
    manifest_path = output / 'manifest.json'
    manifest_path.write_text(json.dumps(record, sort_keys=True,
                                        separators=(',', ':')) + '\n')
    verify(archive, manifest_path)
    return {'source_head_sha': head,
            'source_archive_sha256': record['source_archive_sha256'],
            'source_manifest_sha256': digest(manifest_path),
            'file_count': len(record['members_sha256'])}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--build', type=Path)
    parser.add_argument('--verify', type=Path)
    parser.add_argument('--root', type=Path)
    args = parser.parse_args()
    if (args.build is None) == (args.verify is None):
        parser.error('specify exactly one of --build or --verify')
    if args.build is not None:
        result = build(args.build.resolve())
    else:
        bundle = args.verify.resolve()
        record = verify(bundle / 'source.tar.gz', bundle / 'manifest.json',
                        args.root.resolve() if args.root else None)
        result = {'source_head_sha': record['source_head_sha'],
                  'source_archive_sha256': record['source_archive_sha256'],
                  'source_manifest_sha256': digest(bundle / 'manifest.json'),
                  'file_count': len(record['members_sha256'])}
    print(json.dumps(result, sort_keys=True))


if __name__ == '__main__':
    main()
