#!/usr/bin/env python3
"""Independently replay one RunPod root4 archive and apply the sealed selector."""

import argparse
import hashlib
import json
import subprocess
import sys
import tarfile
from pathlib import Path

from research.n83_root4_highk_tier1_retry_20261019 import run as runner
from research.n83_root4_highk_tier1_retry_20261019 import runpod_provider
from research.n83_root4_highk_tier1_retry_20261019 import select as selector


HERE = Path(__file__).resolve().parent


def sha256(path):
    value = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            value.update(block)
    return value.hexdigest()


def bind_archive(archive, results):
    """Require every archived byte to match the independently replayed tree."""
    expected = {}
    with tarfile.open(archive, 'r:gz') as bundle:
        for member in bundle:
            path = Path(member.name)
            if (path.is_absolute() or '..' in path.parts or
                    not path.parts or path.parts[0] != 'results' or
                    member.issym() or member.islnk() or
                    not (member.isfile() or member.isdir())):
                raise ValueError('unsafe or unexpected raw archive member')
            if member.isdir():
                continue
            relative = Path(*path.parts[1:])
            if relative in expected:
                raise ValueError('duplicate raw archive member')
            stream = bundle.extractfile(member)
            if stream is None:
                raise ValueError('unreadable raw archive member')
            expected[relative] = hashlib.sha256(stream.read()).hexdigest()
    actual = {path.relative_to(results): sha256(path) for path in
              results.rglob('*') if path.is_file()}
    if not expected or expected != actual:
        raise ValueError('raw archive and extracted result bytes differ')
    return {'archive_sha256': sha256(archive),
            'file_count': len(expected)}


def replay(stratum, results, archive, replay_dir):
    archive_receipt = bind_archive(archive, results)
    process = subprocess.run(
        [sys.executable, str(HERE / 'verify.py'), str(results),
         '--stratum', str(stratum)], cwd=HERE.parents[1],
        capture_output=True, text=True, check=False)
    (replay_dir / f'stratum{stratum}.stdout.txt').write_text(process.stdout)
    (replay_dir / f'stratum{stratum}.stderr.txt').write_text(process.stderr)
    (replay_dir / f'stratum{stratum}.exit.txt').write_text(
        f'{process.returncode}\n')
    if process.returncode not in (0, 2) or process.stderr:
        raise ValueError(f'stratum {stratum}: independent replay failed')
    analysis = json.loads((results / 'analysis.json').read_text())
    if (json.loads(process.stdout) != analysis or
            (process.returncode == 0) !=
            (analysis['status'] == 'STRATUM_ADMITTED')):
        raise ValueError(f'stratum {stratum}: replay output differs')
    return {'stratum': stratum, 'results': results, 'analysis': analysis,
            'host': json.loads((results / 'host.json').read_text()),
            'source': json.loads((results / 'source.json').read_text()),
            **archive_receipt}


def aggregate(item, snapshot, head_sha):
    proof = runpod_provider.verify(snapshot, item['results'], head_sha)
    points = item['analysis']['points']
    ids = [arm[3] for arm in item['source']['arms']]
    base_hashes = {
        candidate: {arm['raw_replay'].get('base_hash') for point in points
                    for arm in point['arms']
                    if arm['candidate_id'] == candidate and
                    arm['status'] == 'PASS_INDEPENDENT_REPLAY'}
        for candidate in ids}
    same_base = all(len(hashes) == 1 and None not in hashes
                    for hashes in base_hashes.values())
    coverage = (len(points) == 8 and
                {point['point_index'] for point in points} == set(range(8)))
    evidence = ('PASS_INDEPENDENT_REPLAY' if
                same_base and coverage and
                item['analysis']['evidence_status'] ==
                'PASS_INDEPENDENT_REPLAY' else 'VERIFICATION_FAILED')
    validated = {
        'schema': 'n83.highk-retry-validated/v1',
        'provider': 'runpod',
        'source_sha256': runner.SOURCE_SHA256,
        'targets_sha256': runner.TARGETS_SHA256,
        'answers_sha256': runner.ANSWERS_SHA256,
        'candidate_ids': ids, 'choice': item['source']['choice'],
        'evidence_status': evidence, 'head_sha': head_sha,
        'single_cloud_instance': proof['status'] ==
                                 'PASS_SINGLE_RECORDED_RUNPOD_ALLOCATION',
        'provider_runner_identity': proof,
        'same_candidate_bases': same_base,
        'host_strata': [{
            'stratum': 0,
            'point_indices': item['analysis']['point_indices'],
            'tier_status': item['analysis']['tier_status'],
            'aa_status': item['analysis']['aa_status'],
            'instance_id': item['host']['instance_id'],
            'raw_archive_sha256': item['archive_sha256'],
            'raw_file_count': item['file_count'],
        }],
        'points': points,
    }
    return validated, selector.select(validated)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--snapshot', type=Path, required=True)
    parser.add_argument('--results', type=Path, required=True)
    parser.add_argument('--archive', type=Path, required=True)
    parser.add_argument('--head-sha', required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=False)
    item = replay(0, args.results, args.archive, args.output)
    validated, selection = aggregate(item, args.snapshot, args.head_sha)
    for name, value in (('validated.json', validated),
                        ('selection.json', selection)):
        (args.output / name).write_text(json.dumps(value, sort_keys=True,
                                                  indent=2) + '\n')
    print(json.dumps({'status': selection['admission_status'],
                      'selected_online_candidate_id':
                      selection['online'].get('selected_candidate_id'),
                      'selected_cold_candidate_id':
                      selection['cold'].get('selected_candidate_id')},
                     sort_keys=True))


if __name__ == '__main__':
    main()
