#!/usr/bin/env python3
"""Package observations, never qualify clocks or recompute mathematical prices."""
import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path

PHASES = {'target_query', 'target_PDP', 'target_relation_check', 'target_descent', 'target_recovery_check'}


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def bound(binding):
    path = Path(binding['path']).resolve()
    if digest(path) != binding['sha256']:
        raise ValueError(f'hash mismatch: {path}')
    return path


def load(path):
    return json.loads(Path(path).read_text())


def canonical(binding):
    path = bound(binding)
    spec = importlib.util.spec_from_file_location('canonical_protocol', path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def integer(value):
    return isinstance(value, int) and not isinstance(value, bool) and value >= 0


def online_observation(record):
    online = record.get('online') or {}
    phases = online.get('phases_ns') or {}
    reasons = []
    if set(phases) != PHASES:
        reasons.append('phase_names_not_exact_five')
    if not integer(online.get('wall_ns')) or any(not integer(v) for v in phases.values()):
        reasons.append('invalid_clock_integer')
    elif sum(phases.values()) != online['wall_ns'] or sum(phases.values()) > 2**64 - 1:
        reasons.append('invalid_phase_sum')
    diag = record.get('time', {}).get('online_clock_diagnostics')
    if not isinstance(diag, dict) or 'error' not in diag or diag['error'] is not None:
        reasons.append('online_diagnostics_absent_or_error')
    return reasons


def package(request, output_dir):
    if request.get('schema') != 'pencil-canonical-package-input/v1':
        raise ValueError('unsupported packaging request')
    tool = canonical(request['canonical_tool'])
    protocol_path = bound(request['protocol'])
    protocol = load(protocol_path)
    findings = tool.schema_errors(protocol, 'experiment.protocol/v1') or tool.lint_protocol(protocol, protocol_path.parent)
    if findings or protocol['freeze']['status'] != 'frozen':
        raise ValueError('requires valid frozen protocol: ' + '; '.join(findings))
    packaging_pin = protocol['extensions'].get('canonical_packaging', {})
    if packaging_pin.get('adapter_sha256') != digest(__file__) or packaging_pin.get('canonical_tool_sha256') != request['canonical_tool']['sha256']:
        raise ValueError('packaging adapter/helper not pinned by protocol')
    native_path = bound(request['native_analysis'])
    native = load(native_path)
    if native.get('schema') != 'pencil-price-cohort-output/v1':
        raise ValueError('unsupported native output')
    manifest = native['input_manifest']
    manifest_hash = hashlib.sha256(json.dumps(manifest, sort_keys=True, separators=(',', ':'), ensure_ascii=False).encode()).hexdigest()
    if native.get('input_manifest_integrity_sha256') != manifest_hash:
        raise ValueError('native manifest integrity mismatch')
    pins = protocol['extensions']['analysis_support_sha256']
    for field, source in [('aggregate_source_sha256','native_price/src/aggregate.rs'), ('paired_export_source_sha256','paired_export/src/lib.rs'), ('paired_export_plan_source_sha256','paired_export/src/plan.rs')]:
        if native.get(field) != pins[source]:
            raise ValueError('native analysis source identity mismatch')
    if native.get('mapping', {}).get('mapping_sha256') != protocol['extensions']['finite_cost_model']['mapping_sha256']:
        raise ValueError('native mapping identity mismatch')
    if manifest['protocol_sha256'] != protocol['freeze']['protocol_sha256']:
        raise ValueError('native protocol identity mismatch')
    if manifest['schema'] != 'pencil-price-cohort-input/v1':
        raise ValueError('unsupported native input manifest')
    binary = manifest['binary_sha256']
    if any(a['instrument']['binary_sha256'] != binary for a in protocol['arms']):
        raise ValueError('instrument binary identity mismatch')
    analysis_script = bound(request['analysis_script'])
    if digest(analysis_script) != protocol['analysis_plan']['script']['sha256']:
        raise ValueError('analysis script identity mismatch')
    outdir = Path(output_dir).resolve()
    e = tool.export_skeleton(protocol, os.path.relpath(protocol_path, outdir), 'ecbench', request['export_id'])
    e['source'].update(commit=protocol['arms'][0]['instrument']['commit'], dirty=None)
    for arm in e['arms']:
        arm.update(binary_sha256=binary, commit=e['source']['commit'])
    attached = {}

    def attach(path, kind='raw'):
        path = Path(path).resolve()
        entry = tool.file_entry(outdir, os.path.relpath(path, outdir), kind)
        attached[str(path)] = entry
        return entry

    attach(protocol_path, 'protocol')
    attach(bound(request['canonical_tool']), 'analysis')
    schema_dir = bound(request['canonical_tool']).parent.parent / 'schemas'
    for schema in ('experiment-protocol.v1.json', 'experiment-export.v1.json'):
        attach(schema_dir / schema, 'other')
    attach(Path(__file__), 'analysis')
    for sidecar in request.get('additional_sidecars', []):
        attach(bound(sidecar), 'raw')
    e['analysis']['script'] = attach(analysis_script, 'analysis')
    e['analysis']['outputs'] = [attach(native_path, 'summary')]
    receipt = load(bound(manifest['prepared_plan_validation']))
    attach(bound(manifest['prepared_plan_validation']), 'summary')
    if receipt.get('schema') != 'pencil-prepared-plan-validation/v1' or receipt.get('passed') is not True:
        raise ValueError('invalid plan validation receipt')
    review = manifest.get('coverage_review')
    if review is not None:
        for source in review.get('source_files', []):
            attach(bound(source), 'other')
        if review.get('independent_report') is not None:
            attach(bound(review['independent_report']), 'report')
    resources = manifest.get('separate_resources')
    if resources is not None:
        attach(bound(resources), 'raw')
        if load(bound(resources)) != native.get('separate_resource_observations'):
            raise ValueError('resource sidecar differs from native observation')
    history_path = bound(request['execution_history'])
    attach(history_path, 'receipt')
    history = load(history_path)
    if history.get('schema') != 'pencil-execution-history/v1' or history.get('protocol_sha256') != protocol['freeze']['protocol_sha256'] or history.get('binary_sha256') != binary:
        raise ValueError('execution history identity mismatch')
    histories = {}
    for event in history['records']:
        key = event['record_id']
        if key in histories:
            raise ValueError('duplicate execution history')
        if not integer(event.get('attempts')) or event['attempts'] < 1 or not integer(event.get('resumed')):
            raise ValueError('explicit known attempts/resumed counts required')
        evidence = event.get('evidence', [])
        if not evidence:
            raise ValueError('execution history requires bound evidence')
        for source in evidence:
            attach(bound(source), 'receipt')
        if event['resumed'] > 0:
            checkpoint = event.get('checkpoint')
            if not isinstance(checkpoint, dict) or checkpoint.get('policy') in (None, 'none') or not integer(checkpoint.get('restored_from_generation')):
                raise ValueError('restored history requires checkpoint generation')
        histories[key] = event
    missing, diagnostics = [], []
    seen_sessions, seen_workloads, seen_records, seen_plans = set(), set(), set(), set()
    expected_count = 0
    roles = {a['id']: a['role'] for a in protocol['arms']}
    for entry in manifest['plans']:
        name = entry['name']
        if name in seen_plans or receipt['files'].get(name) != entry['sha256']:
            raise ValueError('plan set/digest differs from receipt')
        seen_plans.add(name)
        plan_path = bound(entry)
        attach(plan_path, 'protocol')
        plan = load(plan_path)
        if plan.get('schema') != 'ecbench.plan/v1':
            raise ValueError('invalid plan schema')
        sid = entry['session_id']
        if sid in seen_sessions:
            raise ValueError('duplicate session')
        seen_sessions.add(sid)
        for w in plan['workloads']:
            if w['workload_id'] in seen_workloads:
                raise ValueError('duplicate workload across plans')
            seen_workloads.add(w['workload_id'])
        records = {}
        if entry.get('records') is not None:
            raw = bound(entry['records'])
            attach(raw)
            for line in raw.read_text().splitlines():
                if not line.strip():
                    continue
                r = json.loads(line)
                if r['seq'] in records or r['record_id'] in seen_records:
                    raise ValueError('duplicate record; retries belong in resource sidecar')
                if r.get('schema') != 'ecbench.record/v1' or r['session_id'] != sid or r['binary_sha256'] != binary:
                    raise ValueError('record identity mismatch')
                seen_records.add(r['record_id'])
                records[r['seq']] = r
        for ex in plan['executions']:
            expected_count += 1
            arm, workload = plan['arms'][ex['arm']], plan['workloads'][ex['workload']]
            r = records.pop(ex['seq'], None)
            if r is None:
                missing.append({'session_id':sid, 'seq':ex['seq'], 'arm':arm['name'], 'workload_id':workload['workload_id'], 'round':ex['round'], 'warmup':ex['warmup'], 'algorithm_seed':str(ex['algorithm_seed'])})
                continue
            if (r['arm'] != arm['name'] or r['arm'] not in roles or r['workload'] != workload or r['method'] != arm['method'] or r['round'] != ex['round'] or r['warmup'] != ex['warmup'] or str(r['algorithm_seed']) != str(ex['algorithm_seed'])):
                raise ValueError('record differs from exact plan identity')
            event = histories.pop(r['record_id'], None)
            if event is None or event.get('session_id') != sid or event.get('seq') != ex['seq']:
                raise ValueError('missing or mismatched explicit execution history')
            t, outcome = r.get('time', {}), r.get('outcome', {})
            status = {'verified':'succeeded', 'timeout':'timeout', 'interrupted':'interrupted', 'cancelled':'cancelled', 'infra_error':'infra_error'}.get(outcome.get('status'), 'failed')
            verified = outcome.get('status') == 'verified' and outcome.get('matches_target') is True and outcome.get('matches_planted') is not False and 'error' in outcome and outcome['error'] is None
            if status == 'succeeded' and not verified:
                status = 'failed'
            counters = {}
            for key, val in t.items():
                if integer(val):
                    counters['time.' + key] = val
            online = r.get('online') or {}
            if integer(online.get('wall_ns')):
                counters['online.wall_ns'] = online['wall_ns']
            for key, val in (online.get('phases_ns') or {}).items():
                if integer(val):
                    counters['online.phases_ns.' + key] = val
            for key, val in r.get('counters', {}).items():
                if integer(val) or isinstance(val, str):
                    counters['native.' + key] = val
            cold = t.get('method_call_wall_ns')
            if cold is not None and not integer(cold):
                raise ValueError('invalid method-call clock')
            reasons = online_observation(r)
            diagnostics.append({'record_id':r['record_id'], 'online_structure_reasons':reasons, 'qualification':'not_established_by_packaging', 'raw_outcome':outcome, 'attempt_and_restore_history':'explicit_hash_bound_history_not_independently_reviewed'})
            e['runs'].append(tool.run_row(run_id=r['record_id'], arm=r['arm'], instance_id=workload['workload_id'], seed=str(r['algorithm_seed']), replicate=ex['round'], warmup=ex['warmup'], status=status, attempts=event['attempts'], resumed=event['resumed'], checkpoint=event.get('checkpoint'),
                measurements={'counted':{}, 'wall_seconds':None if cold is None else cold/1e9, 'wall_valid':False, 'cpu_user_seconds':None if t.get('user_ns') is None else t['user_ns']/1e9, 'cpu_sys_seconds':None if t.get('sys_ns') is None else t['sys_ns']/1e9, 'max_rss_bytes':None if t.get('max_rss_kib') is None else t['max_rss_kib']*1024, 'counters':counters},
                isolation={'tier':r.get('isolation', {}).get('level'), 'contended':None, 'blockers':['independent clock qualification not established by packaging']},
                verification={'status':'verified' if verified else 'no_claim', 'verifier':'raw ecbench outcome; independent review not inferred'}, backend_ref=r['record_id']))
        if records:
            raise ValueError('record not in plan')
    if histories:
        raise ValueError('execution history references absent records; retain unmatched attempts in separate resources')
    if seen_plans != set(receipt['files']):
        raise ValueError('incomplete plan set')
    if expected_count != native['expected_executions'] or len(e['runs']) != native['observed_executions'] or len(missing) != len(native['missing_executions']):
        raise ValueError('native flow differs from actual records/plans')
    e['files'] = list(attached.values())
    e['flow'] = tool.compute_flow(e['runs'])
    e['backend_record'] = {'schema':'pencil-canonical-packaging/v1', 'qualified':False, 'expected_execution_slots':expected_count, 'missing_slots':missing, 'diagnostics':diagnostics, 'resource_observations_present':resources is not None, 'flow_scope':'Canonical rows represent observed records with explicit known attempts/resumed counts and hash-bound evidence. No missing/unknown history is projected. Missing slots are not fabricated attempted rows; separate infrastructure attempts remain in resource sidecars.', 'native_analysis_sha256':digest(native_path)}
    errors = tool.schema_errors(e, 'experiment.export/v1')
    if errors:
        raise ValueError('; '.join(errors))
    return e, tool.lint_export(e, protocol, outdir, verify_files=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('request')
    parser.add_argument('--out', required=True)
    args = parser.parse_args()
    out = Path(args.out)
    e, findings = package(load(args.request), out.parent)
    with out.open('x') as f:
        json.dump(e, f, indent=2)
        f.write('\n')
    print(json.dumps({'export':str(out), 'canonical_findings':findings, 'qualified':False}))
    return 1 if findings else 0


if __name__ == '__main__':
    raise SystemExit(main())
