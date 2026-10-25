#!/usr/bin/env python3
"""Replay two high-K RunPod archives and apply the sealed selector."""

import argparse
import hashlib
import json
import subprocess
import sys
import tarfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from research.n83_crossfamily_current_20261025 import run as runner
from research.n83_crossfamily_current_20261025 import runpod_provider
from research.n83_crossfamily_current_20261025 import select as selector


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
    archived_analysis = (results / 'analysis.json').read_bytes()
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
    if ((results / 'analysis.json').read_bytes() != archived_analysis or
            json.loads(process.stdout) != analysis or
            (process.returncode == 0) !=
            (analysis['status'] == 'STRATUM_ADMITTED')):
        raise ValueError(f'stratum {stratum}: replay output differs')
    return {'stratum': stratum, 'results': results, 'analysis': analysis,
            'host': json.loads((results / 'host.json').read_text()),
            'source': json.loads((results / 'source.json').read_text()),
            **archive_receipt}


def aggregate(strata, snapshot, head_sha):
    first = strata[0]
    keys = ('hashes', 'source_sha256', 'binary_sha256', 'rho_binary_sha256',
            'runner_sha256', 'targets_sha256', 'answers_sha256',
            'fixture_manifest_sha256', 'arms', 'choice', 'rho_seed_rule',
            'calibration_seed', 'calibration_repetitions_per_side',
            'calibration_comparison', 'calibration_target_sha256',
            'calibration_answer_sha256', 'global_producer_cap_seconds',
            'ic_cap_seconds', 'layout_receipt_sha256',
            'runpod_adapter_sha256')
    same_source = all(item['source'][key] == first['source'][key]
                      for item in strata for key in keys)
    proof = runpod_provider.verify(
        snapshot, [item['results'] for item in strata], head_sha)
    points = sorted(
        [point for item in strata for point in item['analysis']['points']],
        key=lambda item: item['point_index'])
    ids = [arm[3] for arm in first['source']['arms']]
    base_hashes = {
        candidate: {arm['raw_replay'].get('base_hash') for point in points
                    for arm in point['arms']
                    if arm['candidate_id'] == candidate and
                    arm['status'] == 'PASS_INDEPENDENT_REPLAY'}
        for candidate in ids}
    same_base = all(len(hashes) == 1 and None not in hashes
                    for hashes in base_hashes.values())
    coverage = (len(points) == 16 and
                {point['point_index'] for point in points} == set(range(16)))
    evidence = ('PASS_INDEPENDENT_REPLAY' if
                same_source and same_base and coverage and
                all(item['analysis']['evidence_status'] ==
                    'PASS_INDEPENDENT_REPLAY' for item in strata) else
                'VERIFICATION_FAILED')
    validated = {
        'schema': 'n83.crossfamily-current-validated/v1',
        'provider': 'runpod',
        'source_sha256': runner.SOURCE_SHA256,
        'targets_sha256': runner.TARGETS_SHA256,
        'answers_sha256': runner.ANSWERS_SHA256,
        'candidate_ids': ids, 'choice': first['source']['choice'],
        'evidence_status': evidence, 'head_sha': head_sha,
        'same_source_and_binary': same_source,
        'different_cloud_instances': proof['status'] ==
                                     'PASS_TWO_DISTINCT_CONCURRENT_RUNPOD_MACHINES',
        'provider_runner_identity': proof,
        'same_candidate_bases_across_strata': same_base,
        'host_strata': [
            {'stratum': item['stratum'],
             'point_indices': item['analysis']['point_indices'],
             'tier_status': item['analysis']['tier_status'],
             'aa_status': item['analysis']['aa_status'],
             'instance_id': item['host']['instance_id'],
             'raw_archive_sha256': item['archive_sha256'],
             'raw_file_count': item['file_count']}
            for item in strata],
        'points': points,
    }
    return validated, selector.select(validated)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--snapshot', type=Path, required=True)
    parser.add_argument('--stratum0', type=Path, required=True)
    parser.add_argument('--stratum1', type=Path, required=True)
    parser.add_argument('--archive0', type=Path, required=True)
    parser.add_argument('--archive1', type=Path, required=True)
    parser.add_argument('--head-sha', required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=False)
    strata = [replay(i, results, archive, args.output) for
              i, (results, archive) in enumerate(
                  ((args.stratum0, args.archive0),
                   (args.stratum1, args.archive1)))]
    validated, selection = aggregate(strata, args.snapshot, args.head_sha)
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
