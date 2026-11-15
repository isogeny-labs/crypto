#!/usr/bin/env python3
"""Materialize canonical IC1 workload and run rows for the extended panel."""

import argparse
import hashlib
import io
import json
import sys
from contextlib import redirect_stdout
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
if sys.path and Path(sys.path[0]).resolve() == HERE:
    sys.path.pop(0)
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from research.n83_fresh_cold_binary_retest_20261021 import check_dispatch, run, select  # noqa: E402

CURVE_ID = 'EC1N83Ckb1a1haacb69ee9f37'


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(',', ':'),
                      ensure_ascii=False).encode('utf-8')


def digest(path):
    value = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            value.update(block)
    return value.hexdigest()


def write_once(path, value):
    blob = json.dumps(value, sort_keys=True, indent=2) + '\n'
    if path.exists() and path.read_text() != blob:
        raise ValueError(f'sealed IC1 rows differ: {path}')
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(blob)


def materialize(validated_path, selection_path, stratum_dirs, output):
    with redirect_stdout(io.StringIO()):
        check_dispatch.main()
    dispatch = json.loads((HERE / 'dispatch.json').read_text())
    validated = json.loads(validated_path.read_text())
    selection = json.loads(selection_path.read_text())
    ids = dispatch['ordered_candidate_ids']
    if (validated.get('schema') != 'n83.fresh-cold-binary-retest-validated/v1' or
            validated.get('source_sha256') != run.SOURCE_SHA256 or
            validated.get('targets_sha256') != run.TARGETS_SHA256 or
            validated.get('answers_sha256') != run.ANSWERS_SHA256 or
            validated.get('candidate_ids') != ids or
            validated.get('choice') != dispatch['choice'] or
            selection != select.select(validated) or
            len(validated.get('points', [])) != 16 or
            len(validated.get('host_strata', [])) != 2):
        raise ValueError('validated result, dispatch or selection differs')
    public = [json.loads(line) for line in
              (HERE / 'fresh_targets.jsonl').read_text().splitlines()]
    answers = [json.loads(line) for line in
               (HERE / 'fresh_answers.jsonl').read_text().splitlines()]
    if len(public) != 16 or len(answers) != 16:
        raise ValueError('sealed fixture count differs')
    workloads, rows = [], []
    for point in sorted(validated['points'], key=lambda row: row['point_index']):
        index = point['point_index']
        stratum_number = index // 8
        folder = stratum_dirs[stratum_number]
        stratum = validated['host_strata'][stratum_number]
        target_blob = json.dumps(public[index], separators=(',', ':')) + '\n'
        point_file = folder / 'points' / f'p{index:02d}' / 'target.jsonl'
        if (stratum['stratum'] != stratum_number or
                (point_file.exists() and point_file.read_text() != target_blob) or
                answers[index]['fixture_index'] != index):
            raise ValueError('raw point, host or sealed fixture differs')
        workload = {
            'schema_version': 1, 'curve_id': CURVE_ID,
            'target_count': 1, 'public_targets': [public[index]],
            'input_law': {
                'domain': run.generator.DOMAIN.decode(),
                'fixture_index': index,
                'scalar_rule': '1+little_endian(SHA256(domain||u32_le(index))) mod (r-1)',
                'seed_sha256': answers[index]['seed_sha256'],
                'generator_sha256': digest(HERE / 'generate_fresh.py'),
                'answer_sidecar_sha256': digest(HERE / 'fresh_answers.jsonl')},
            'target_sha256': hashlib.sha256(target_blob.encode()).hexdigest(),
            'cold_or_warm': 'one_public_point_per_fresh_process_empty_base_index_and_logs',
        }
        workload_id = hashlib.sha256(canonical(workload)).hexdigest()[:12]
        workloads.append({'workload_id': workload_id, **workload})
        source_path, host_path, analysis_path = (folder / name for name in
                                                  ('source.json', 'host.json',
                                                   'analysis.json'))
        source = json.loads(source_path.read_text())
        host = json.loads(host_path.read_text())
        if (digest(analysis_path) != stratum['analysis_sha256'] or
                source['source_sha256'] != validated['source_sha256'] or
                host['instance_id'] != stratum['instance_id']):
            raise ValueError('raw source, host or replay receipt differs')
        by_id = {arm['candidate_id']: arm for arm in point['arms']}
        for candidate in ids:
            arm = by_id.get(candidate, {'status': 'NOT_RUN'})
            raw = arm.get('raw_replay', {})
            rho = point['rho_by_candidate'].get(candidate,
                                                {'status': 'NOT_RUN'})
            passed = (arm['status'] == 'PASS_INDEPENDENT_REPLAY' and
                      arm.get('resource_status') == 'PASS' and
                      rho['status'] == 'PASS_INDEPENDENT_REPLAY' and
                      rho.get('resource_status') == 'PASS' and
                      stratum['tier_status'] == 'PASS_TIER1')
            rows.append({
                'candidate_id': candidate, 'workload_id': workload_id,
                'run_id': f'{candidate}W{workload_id}R1',
                'point_index': index, 'stratum': stratum_number,
                'producer_target_file_status': 'PRESENT_MATCHING' if
                point_file.exists() else 'NOT_WRITTEN',
                'status': arm['status'],
                'hardware_tier': 'cloud_instance_tier1' if passed else
                                 'cloud_instance_exploratory',
                'candidate_manifest_sha256':
                    dispatch['choice']['candidate_manifest_sha256'][candidate],
                'source_snapshot_sha256': source['source_sha256'],
                'run_head_sha': host['git_sha'],
                'executed_binary_sha256': source['binary_sha256'],
                'rho_binary_sha256': source['rho_binary_sha256'],
                'host_receipt_sha256': digest(host_path),
                'source_receipt_sha256': digest(source_path),
                'correctness_receipt_sha256': digest(analysis_path),
                'raw_artifact_run_id': validated['run_id'],
                'raw_artifact_sha256': stratum['raw_artifact_sha256'],
                'actual_usable_points': raw.get('actual_factor_base_points'),
                'effective_columns': raw.get('effective_columns'),
                'cold_process_ms': arm.get('cold_process_ms') if passed else None,
                'one_target_online_ms': arm.get('one_target_online_ms') if passed else None,
                'paired_rho_online_ms': rho.get('online_ms') if passed else None,
                'online_phases_ms': raw.get('online_phases_ms') if passed else None,
                'setup_ms': raw.get('setup_ms') if passed else None,
                'rank_stage_ms': raw.get('rank_stage_ms') if passed else None,
                'rank_attempts': raw.get('rank_attempts'),
                'rank_relations': raw.get('rank_relations'),
                'target_probes': raw.get('target_probes'),
                'peak_rss_bytes': raw.get('resources', {}).get('peak_rss_bytes'),
                'pair_calibration': point['calibration_by_candidate'].get(
                    candidate, {'status': 'NOT_RUN'}),
                'raw_evidence': raw,
            })
    expected = 16 * len(ids)
    if (len(workloads) != 16 or
            len({item['workload_id'] for item in workloads}) != 16 or
            len(rows) != expected or
            len({item['run_id'] for item in rows}) != expected):
        raise ValueError('canonical IC1 workload or row count differs')
    result = {
        'schema': 'n83.fresh-cold-binary-retest-ic1/v1',
        'provider_run_id': validated['run_id'],
        'run_attempt': validated['run_attempt'],
        'run_head_sha': validated['head_sha'],
        'validated_rows_sha256': digest(validated_path),
        'selection_sha256': digest(selection_path),
        'dispatch_sha256': digest(HERE / 'dispatch.json'),
        'host_strata': validated['host_strata'],
        'workloads': workloads, 'rows': rows,
        'online_selection': selection['online'],
        'cold_selection': selection['cold'],
    }
    write_once(output, result)
    return {'status': 'PASS', 'workloads': 16, 'rows': expected}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('validated_rows', type=Path)
    parser.add_argument('selection', type=Path)
    parser.add_argument('stratum0_results', type=Path)
    parser.add_argument('stratum1_results', type=Path)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(materialize(args.validated_rows.resolve(),
                                 args.selection.resolve(),
                                 (args.stratum0_results.resolve(),
                                  args.stratum1_results.resolve()),
                                 args.output.resolve()), sort_keys=True))


if __name__ == '__main__':
    main()
