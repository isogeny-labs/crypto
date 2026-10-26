#!/usr/bin/env python3
"""Bind a RunPod raw archive and independently replay the N83 screen."""

import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import tarfile

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
IDS = {
    'k3450_wide3': 'IC1N83Ckb1a1fb572700PDP4rootRCguidedLAgaussTDdirectISO0hc7264b83bbd1',
    'k3500_root4': 'IC1N83Ckb1a1fb581000PDP4rootRCguidedLAgaussTDdirectISO0hd41c74f97eea',
    'k3596_root4': 'IC1N83Ckb1a1fb596936PDP4rootRCguidedLAgaussTDdirectISO0h3da4e8b31c3f',
}


def digest(path):
    value = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            value.update(block)
    return value.hexdigest()


def archive_binding(archive, results, manifest):
    parts = json.loads(manifest.read_text())
    if (parts.get('schema') != 'n83.current-online-screen-runpod-raw-parts/v1'
            or digest(archive) != parts['archive_sha256'] or
            archive.stat().st_size != parts['archive_bytes']):
        raise ValueError('raw archive or part manifest differs')
    for part in parts['parts']:
        path = manifest.parent / part['name']
        if (not part['name'].startswith('raw.part.') or
                '/' in part['name'] or not path.is_file() or
                path.stat().st_size != part['bytes'] or
                digest(path) != part['sha256']):
            raise ValueError('raw transfer part differs')
    expected = {}
    with tarfile.open(archive, 'r:gz') as bundle:
        for member in bundle:
            path = Path(member.name)
            if (path.is_absolute() or '..' in path.parts or not path.parts or
                    path.parts[0] not in ('results', 'evidence') or
                    not (member.isfile() or member.isdir())):
                raise ValueError('unsafe raw archive member')
            if member.isdir():
                continue
            if member.name in expected:
                raise ValueError('duplicate archive member')
            stream = bundle.extractfile(member)
            if stream is None:
                raise ValueError('unreadable archive member')
            expected[member.name] = hashlib.sha256(stream.read()).hexdigest()
    actual = {f'results/{path.relative_to(results)}': digest(path)
              for path in results.rglob('*') if path.is_file()}
    result_expected = {path: value for path, value in expected.items()
                       if path.startswith('results/')}
    if not result_expected or actual != result_expected:
        raise ValueError('extracted results differ from raw archive')
    return {'archive_sha256': parts['archive_sha256'],
            'archive_file_count': len(expected),
            'result_file_count': len(result_expected)}


def provider_binding(allocation_path, results):
    allocation = json.loads(allocation_path.read_text())
    embedded = results / 'runpod_allocation.json'
    if not embedded.exists() or digest(embedded) != digest(allocation_path):
        raise ValueError('RunPod allocation differs from embedded result')
    host = json.loads((results / 'host.json').read_text())
    source = json.loads((results / 'source.json').read_text())
    if (allocation.get('schema') !=
            'n83.current-online-screen-runpod-allocation/v1' or
            allocation.get('desired_status') != 'RUNNING' or
            not 0 < allocation.get('cost_per_hr', 0) <= 0.40 or
            host.get('provider') != 'runpod' or
            host.get('instance_id') !=
            f"rp-{allocation['pod_id']}-{allocation['machine_id']}" or
            host.get('git_sha') != allocation['source_head_sha'] or
            host.get('runpod_allocation_sha256') != digest(embedded) or
            source.get('source_head_sha') != allocation['source_head_sha'] or
            source.get('runpod_allocation_sha256') != digest(embedded)):
        raise ValueError('RunPod Pod, machine, source or cost binding differs')
    return {'pod_id': allocation['pod_id'],
            'machine_id': allocation['machine_id'],
            'source_head_sha': allocation['source_head_sha'],
            'cost_per_hr': allocation['cost_per_hr'],
            'allocation_sha256': digest(allocation_path)}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--allocation', type=Path, required=True)
    parser.add_argument('--archive', type=Path, required=True)
    parser.add_argument('--manifest', type=Path, required=True)
    parser.add_argument('--results', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=False)
    archive = archive_binding(args.archive, args.results, args.manifest)
    provider = provider_binding(args.allocation, args.results)
    process = subprocess.run([sys.executable, str(HERE / 'runpod_verify.py'),
                              str(args.results)], cwd=ROOT,
                             capture_output=True, text=True, check=False)
    (args.output / 'local_replay.stdout.txt').write_text(process.stdout)
    (args.output / 'local_replay.stderr.txt').write_text(process.stderr)
    (args.output / 'local_replay.exit.txt').write_text(
        f'{process.returncode}\n')
    if process.returncode not in (0, 1) or process.stderr:
        raise ValueError('independent arithmetic replay failed')
    analysis = json.loads((args.results / 'analysis.json').read_text())
    if json.loads(process.stdout) != analysis:
        raise ValueError('producer and local analysis differ')
    selection = analysis['selected_pipeline']
    validated = {
        'schema': 'n83.current-online-screen-runpod-validated/v1',
        'evidence_status': ('PASS_INDEPENDENT_REPLAY' if
                            process.returncode == 0 and
                            analysis['status'] == 'COMPLETE' else
                            'INCOMPLETE_OR_RESOURCE_REJECTED'),
        'provider': provider, 'archive': archive,
        'target_count': analysis['target_count'],
        'selection_status': ('SELECTED' if selection else 'BOUNDED_UNKNOWN'),
        'selected_pipeline': selection,
        'selected_online_candidate_id': IDS[selection] if selection else None,
        'balanced_comparison': analysis['balanced_comparison'],
    }
    (args.output / 'validated.json').write_text(
        json.dumps(validated, indent=2, sort_keys=True) + '\n')
    print(json.dumps({'evidence_status': validated['evidence_status'],
                      'selection_status': validated['selection_status'],
                      'selected_online_candidate_id':
                      validated['selected_online_candidate_id']}, sort_keys=True))


if __name__ == '__main__':
    main()
