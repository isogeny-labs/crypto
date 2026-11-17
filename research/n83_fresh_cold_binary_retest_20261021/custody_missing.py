#!/usr/bin/env python3
"""Replay and S3-custody the two current-source finalist bases absent from a1."""

import argparse
import gzip
import hashlib
import importlib.util
import json
import sys
import tempfile
import zipfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
if sys.path and Path(sys.path[0]).resolve() == HERE:
    sys.path.pop(0)
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))
import subprocess
GROUP_PATH = ROOT / 'research/n83_indexed5_20261013/analyze.py'
GROUP_SHA256 = 'e34d06b3275d86dcb3a659b79adf87dcdc078cebb6f510c4b5c42403d83a1804'
SOURCE_SHA256 = '4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507'
ARTIFACT_SHA256 = 'fa2252377835ec0db62a773001770ad01998a9b64fc2b1924c8e04e45bb49e20'
BUCKET = 'crypto-autoresearcher'
PREFIX = 'factor-bases/icv1/etc/n83_current_source_finalists_20261019/a2/objects'
RUN_ID = 39253164581
ARMS = {2304: 'k2304_root11', 1107: 'k1107_root11'}


def digest(data):
    return hashlib.sha256(data).hexdigest()


def file_digest(path):
    value = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1 << 20), b''):
            value.update(block)
    return value.hexdigest()


def read_json(archive, name):
    return json.loads(archive.read(name))


def aws(*args):
    call = subprocess.run(['aws', *args], capture_output=True, text=True)
    if call.returncode:
        raise RuntimeError(f'AWS command failed: {call.stderr.strip()}')
    return call.stdout


def head_or_none(key):
    call = subprocess.run(['aws', 's3api', 'head-object', '--bucket', BUCKET,
                           '--key', key, '--output', 'json'],
                          capture_output=True, text=True)
    if call.returncode == 0:
        return json.loads(call.stdout)
    if '404' in call.stderr or 'Not Found' in call.stderr:
        return None
    raise RuntimeError(f'S3 HEAD failed: {call.stderr.strip()}')


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--artifact', type=Path, required=True)
    parser.add_argument('--output-dir', type=Path, required=True)
    parser.add_argument('--receipt', type=Path, required=True)
    parser.add_argument('--upload', action='store_true')
    args = parser.parse_args()
    if file_digest(args.artifact) != ARTIFACT_SHA256 or \
            file_digest(GROUP_PATH) != GROUP_SHA256:
        raise ValueError('artifact or independent group source differs')
    spec = importlib.util.spec_from_file_location('n83_missing_custody_group', GROUP_PATH)
    group = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(group)
    answers = [json.loads(x) for x in (ROOT /
        'research/n83_final_extended_20261018/fresh_answers.jsonl').read_text().splitlines()]
    with zipfile.ZipFile(args.artifact) as archive:
        analysis = read_json(archive, 'results/analysis.json')
        if (analysis['status'] != 'STRATUM_ADMITTED' or
                analysis['evidence_status'] != 'PASS_INDEPENDENT_REPLAY' or
                analysis['point_indices'] != list(range(8))):
            raise ValueError('archived stratum is not independently admitted')
        rows = []
        args.output_dir.mkdir(parents=True, exist_ok=True)
        for k, name in ARMS.items():
            base_path = f'results/points/p00/runs/{name}/'
            raw = archive.read(base_path + 'base_dump.jsonl')
            summary = read_json(archive, base_path + 'stdout.jsonl')
            target = read_json(archive, 'results/points/p00/target.jsonl')
            result = read_json(archive, base_path + 'targets_result.jsonl')
            manifests = list((ROOT / 'research/n83_online_finalists_20261018').glob(
                f'candidate_*fb{166*k}PDP4rootRCguidedLAgaussTDdirectISO0h*.json'))
            if len(manifests) != 1:
                raise ValueError(f'K{k}: ambiguous manifest')
            manifest = json.loads(manifests[0].read_text())
            spec_base = manifest['factor_base']
            candidate = manifest['candidate_id']
            archived_arm = next((arm for arm in analysis['points'][0]['arms']
                                 if arm['candidate_id'] == candidate), None)
            if (archived_arm is None or
                    archived_arm['status'] != 'PASS_INDEPENDENT_REPLAY' or
                    manifest['implementation']['source_sha256']['ic'] != SOURCE_SHA256 or
                    spec_base['effective_columns'] != k or
                    spec_base['actual_usable_points'] != 166*k or
                    spec_base['ordered_base_dump_sha256'] != digest(raw) or
                    spec_base['source_base_hash_blake3'] != summary['base_hash'] or
                    summary['rank'] != k or summary['factor_base_points'] != 166*k or
                    archived_arm['raw_replay']['base_hash'] != summary['base_hash']):
                raise ValueError(f'K{k}: source, base or archive evidence differs')
            coords = group.replay_base(json.loads(raw), summary)
            point = group.point_from_json(target)
            scalar = int(result['recovered_scalar'])
            relation = None
            indices = result['point_indices']
            if (len(coords) != 166*k or len(indices) != 4 or
                    any(not isinstance(i, int) or i < 0 or i >= len(coords)
                        for i in indices)):
                raise ValueError(f'K{k}: base or relation indices differ')
            for i in indices:
                relation = group.add(relation, coords[i])
            if (scalar != int(answers[0]['scalar']) or
                    group.multiply(group.G, scalar) != point or
                    group.multiply(point, group.R) is not None or
                    relation != point or result['target'] != target or
                    result['group_verified'] is not True or
                    result['exit_code'] != 0):
                raise ValueError(f'K{k}: scalar or relation replay differs')
            packed = gzip.compress(raw, compresslevel=9, mtime=0)
            packed_sha = digest(packed)
            local = args.output_dir / f'{packed_sha}.jsonl.gz'
            if local.exists() and local.read_bytes() != packed:
                raise ValueError(f'K{k}: local object collision')
            local.write_bytes(packed)
            key = f'{PREFIX}/{local.name}'
            row = {'k': k, 'candidate_id': candidate,
                   'candidate_manifest_sha256': file_digest(manifests[0]),
                   'source_base_hash_blake3': summary['base_hash'],
                   'raw_sha256': digest(raw), 'raw_bytes': len(raw),
                   'gzip_sha256': packed_sha, 'gzip_bytes': len(packed),
                   's3_uri': f's3://{BUCKET}/{key}',
                   'independent_replay': 'PASS_BASE_RELATION_SCALAR',
                   'status': 'LOCAL_ONLY'}
            if args.upload:
                metadata = f"raw-sha256={digest(raw)},base-hash={summary['base_hash']},role=k{k}root11"
                existing = head_or_none(key)
                if existing is None:
                    aws('s3', 'cp', str(local), row['s3_uri'], '--no-progress',
                        '--only-show-errors', '--metadata', metadata)
                head = head_or_none(key)
                if (head['ContentLength'] != len(packed) or
                        head.get('Metadata', {}).get('raw-sha256') != digest(raw) or
                        head.get('Metadata', {}).get('base-hash') != summary['base_hash']):
                    raise ValueError(f'K{k}: S3 HEAD differs')
                with tempfile.TemporaryDirectory(dir=args.output_dir) as folder:
                    copy = Path(folder) / 'download.jsonl.gz'
                    aws('s3', 'cp', row['s3_uri'], str(copy), '--no-progress',
                        '--only-show-errors')
                    if (file_digest(copy) != packed_sha or
                            digest(gzip.decompress(copy.read_bytes())) != digest(raw)):
                        raise ValueError(f'K{k}: downloaded object differs')
                row['status'] = 'PASS_DOWNLOADED_SHA256'
                row['s3_etag'] = head['ETag']
                row['s3_last_modified'] = head['LastModified']
            rows.append(row)
            print(f'K{k}: {row["status"]}', flush=True)
    receipt = {'schema': 'n83.current-source-missing-base-custody/v1',
               'status': 'PASS_S3_ROUNDTRIP' if args.upload else 'LOCAL_ONLY',
               'provider_run_id': RUN_ID, 'stratum0_artifact_sha256': ARTIFACT_SHA256,
               'ic_source_sha256': SOURCE_SHA256,
               'independent_group_source_sha256': GROUP_SHA256,
               'objects': rows}
    blob = json.dumps(receipt, sort_keys=True, indent=2) + '\n'
    if args.receipt.exists() and args.receipt.read_text() != blob:
        raise ValueError('existing custody receipt differs')
    args.receipt.write_text(blob)
    print(json.dumps({'status': receipt['status'], 'object_count': len(rows)}))


if __name__ == '__main__':
    main()
