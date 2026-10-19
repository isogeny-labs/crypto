#!/usr/bin/env python3
"""Fail-closed audit of a frozen N83 mixed 2+2 Spot panel or its prefix."""

from __future__ import annotations

import argparse
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path

from panel_supervisor import PLAN

SOURCE = 'cad5fb22d7d768c27ef495e6ef9f84259443f49d'
BINARY = 'a2c2414f1eb52b8982d62c3b58eac1bf768160a569c937824ce6c05290e1b097'
IMAGE = 'sha256:8ed3c0a996841537f75607e7d1de2114d8150391f75792e8da9268738547e73f'
OVERLAP = '032c5cbbf1a55b667ca885a9b7cd766fa86d2926ad1d5762eebf1843bc036b93'
OVERLAP_AUDIT = '1913836ff201eb234127a141ed4512beb31143121b1aa8a9b650e3059f68087d'
CORPUS = '831eaa7be2387634fd610f6706103b2af9b4e274f0db59923c0280af3cbed74b'
BASES = {
    'U': ('1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d',
          '564c6bb72b346b73112240bb0bddc4ef68b7525f7ef2556d6046e52c32f976e0',
          '69ab5c1a90a09f92d9b0a730c78c7db6683f838e06f53d30c072f3e039f4f17c',
          'public_x_hash', 2026100801),
    'D': ('e0c24980207cbb15fa57ec76bcfc6a628a763a80834cba1117ca83ea04678682',
          '8c1c245cda15bdd0019b487a5535a3b7eac642c259c4dc0bae25875d87d345b0',
          'adb3f1545fde96525fda08ba137e25ab71d28bc0822dec155f66177805643a46',
          'direct_subgroup_x_d18', 0),
}
TARGETS = {
    0: ('ad77539c5624533970a6df1abf339a0f4f90e54259da33ba275c740e4262cfb4',
        {'x_decimal': '1834913121178197242856238',
         'y_decimal': '8797964190732846213981377'}),
    1: ('25e15462e2cb86a928dd024c2bb5403218bd14259d893b6d9cfff7f07e06fb7e',
        {'x_decimal': '5809312440517013698248638',
         'y_decimal': '3568442427236746854022671'}),
}
EIGHT_GIB = 8 * 1024**3


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def read_json(path: Path):
    return json.loads(path.read_text())


def verify_mirror(path: Path) -> set[str]:
    lines = (path / 'raw.sha256').read_text().splitlines()
    require(bool(lines), f'{path.name}: empty host manifest')
    seen: set[str] = set()
    for line in lines:
        digest, name = line.split('  ', 1)
        name = name.removeprefix('./')
        require(name not in seen and len(digest) == 64
                and all(c in '0123456789abcdef' for c in digest),
                f'{path.name}: malformed host manifest')
        seen.add(name)
        file = path / name
        require(file.is_file() and hashlib.sha256(file.read_bytes()).hexdigest() == digest,
                f'{path.name}: host raw mismatch {name}')
    files = {str(p.relative_to(path)) for p in path.rglob('*') if p.is_file()}
    extra = files - seen - {'raw.sha256'}
    allowed = {'replay-arm64.json', 'original.exit.txt', 'original.stdout.log',
               'original.stderr.log'}
    for mutation in ('changed-point', 'changed-row'):
        allowed.update({f'{mutation}.worker.json', f'{mutation}.exit.txt',
                        f'{mutation}.stdout.log', f'{mutation}.stderr.log'})
    require(extra <= allowed and seen | {'raw.sha256'} | extra == files,
            f'{path.name}: unexpected local raw file')
    return extra


def stamp(path: Path) -> datetime:
    return datetime.fromisoformat(path.read_text().strip().replace('Z', '+00:00')).astimezone(timezone.utc)


def sidecars(path: Path, name: str, shape: str, mode: str, fixture: int) -> tuple[list[dict], list[dict]]:
    phases = [json.loads(line) for line in (path / 'worker.json.phases.jsonl').read_text().splitlines()]
    progress_path = path / 'worker.json.progress.jsonl'
    progress = [json.loads(line) for line in progress_path.read_text().splitlines()] if progress_path.exists() else []
    require(len(phases) >= 2, f'{name}: absent input/model phases')
    first = shape[0]
    obj, point = BASES[first][:2]
    for kind, rows in [('phase', phases), ('progress', progress)]:
        require([r['sequence'] for r in rows] == list(range(len(rows))), f'{name}: {kind} sequence')
        require(all(a['elapsed_wall_ms'] <= b['elapsed_wall_ms'] for a, b in zip(rows, rows[1:])),
                f'{name}: {kind} clock')
        require(all(r['source_commit'] == SOURCE and r['object_blake3'] == obj
                    and r['point_set_blake3'] == point for r in rows), f'{name}: {kind} identity')
        require(all(0 < r['memory_current_bytes'] <= EIGHT_GIB for r in rows),
                f'{name}: {kind} memory')
    require([r['phase'] for r in phases[:2]] == ['inputs_checked', 'model_domain_done'],
            f'{name}: phase order')
    inputs, model = phases[0]['details'], phases[1]['details']
    require(inputs['shape'] == shape and inputs['mode'] == mode
            and inputs['fixture_index'] == fixture and inputs['binary_sha256'] == BINARY
            and inputs['public_corpus_blake3'] == CORPUS
            and inputs['overlap_sha256'] == OVERLAP
            and inputs['overlap_audit_sha256'] == OVERLAP_AUDIT,
            f'{name}: input identity')
    require(model['ordered_first_pair'] is True
            and 0 < model['variables'] <= 300_000
            and 0 < model['clauses'] <= 8_000_000
            and model['xor_rows'] > 0,
            f'{name}: model limits')
    if mode != 'positive':
        require(inputs['target_blake3'] == TARGETS[fixture][0], f'{name}: public target hash')
    for field in ('completed_decisions', 'completed_conflicts', 'completed_restarts',
                  'completed_xor_passes', 'completed_xor_nanoseconds'):
        require(all(a[field] <= b[field] for a, b in zip(progress, progress[1:])),
                f'{name}: nonmonotone {field}')
    return phases, progress


def audit_arm(path: Path, name: str, mode: str, shape: str, fixture: int) -> dict:
    local_extra = verify_mirror(path)
    require((path / 'shape.txt').read_text().strip() == shape
            and (path / 'mode.txt').read_text().strip() == mode
            and int((path / 'fixture-index.txt').read_text()) == fixture,
            f'{name}: wrapper identity')
    start, end = stamp(path / 'start.utc'), stamp(path / 'end.utc')
    require(start < end, f'{name}: host interval')
    exit_code = int((path / 'host-exit.txt').read_text())
    containers = read_json(path / 'container-inspect.json')
    require(len(containers) == 1, f'{name}: container count')
    container = containers[0]
    state, host = container['State'], container['HostConfig']
    require(state['ExitCode'] == exit_code and state['OOMKilled'] is False,
            f'{name}: container exit/OOM')
    require(host['Memory'] == host['MemorySwap'] == EIGHT_GIB
            and host['NanoCpus'] == 1_000_000_000 and host['CpusetCpus'] == '7'
            and host['NetworkMode'] == 'none' and host['ReadonlyRootfs'] is True
            and host['CapDrop'] == ['ALL'] and host['PidsLimit'] == 256
            and 'no-new-privileges' in host['SecurityOpt'], f'{name}: resource/security envelope')
    mounts = {m['Destination']: m for m in container['Mounts']}
    require(set(mounts) == {'/opt/worker', '/inputs/u', '/inputs/d', '/inputs/evidence',
                            '/inputs/fixture', '/snapshot', '/out'}, f'{name}: mount set')
    require(all(m['RW'] is False for key, m in mounts.items() if key != '/out')
            and mounts['/out']['RW'] is True
            and mounts['/out']['Source'].endswith(f'/runs/{name}')
            and mounts['/inputs/u']['Source'].endswith('/inputs/U')
            and mounts['/inputs/d']['Source'].endswith('/inputs/D')
            and mounts['/inputs/fixture']['Source'].endswith('/inputs/U'),
            f'{name}: mount bindings')
    require(container['Config']['Image'].endswith(IMAGE), f'{name}: image digest')
    image = read_json(path / 'image-inspect.json')
    require(len(image) == 1 and any(d.endswith(IMAGE) for d in image[0]['RepoDigests']),
            f'{name}: pulled image digest')
    phases, progress = sidecars(path, name, shape, mode, fixture)
    cap_path, worker_path = path / 'worker.json.cap.json', path / 'worker.json'
    if cap_path.exists():
        require(mode == 'natural' and exit_code == 124 and not worker_path.exists(),
                f'{name}: cap/exit mismatch')
        receipt = read_json(cap_path)
        require(receipt['schema'] == 'n83.mixed-orbit-m4-cap/v1'
                and receipt['status'] == 'UNKNOWN_budget'
                and receipt['shape'] == shape and receipt['mode'] == mode
                and receipt['fixture_index'] == fixture and receipt['summands'] == 4
                and receipt['budget_seconds'] == 600
                and 599_000 <= receipt['elapsed_process_wall_ms'] <= 610_000,
                f'{name}: deadline receipt')
        status = 'UNKNOWN_budget'
    else:
        require(exit_code == 0 and worker_path.exists(), f'{name}: missing worker receipt')
        receipt = read_json(worker_path)
        require(receipt['schema'] == 'n83.mixed-orbit-m4-worker/v1'
                and receipt['source_commit'] == SOURCE and receipt['binary_sha256'] == BINARY
                and receipt['source_attestation_mode'] == 'supervised_snapshot'
                and receipt['shape'] == shape and receipt['mode'] == mode
                and receipt['fixture_index'] == fixture and receipt['summands'] == 4
                and receipt['public_corpus_blake3'] == CORPUS
                and receipt['overlap_sha256'] == OVERLAP
                and receipt['overlap_audit_sha256'] == OVERLAP_AUDIT,
                f'{name}: worker identity')
        for arm in ('U', 'D'):
            item = receipt['identities'][arm]
            if arm in shape:
                obj, point, manifest, policy, seed = BASES[arm]
                require(item['object_blake3'] == obj and item['point_set_blake3'] == point
                        and item['manifest_blake3'] == manifest
                        and item['policy'] == policy and item['seed'] == seed
                        and item['columns'] == 16_627 and item['points'] == 2_760_082,
                        f'{name}: {arm} base identity')
            else:
                require(item is None, f'{name}: unused {arm} base imported')
        require(phases[0]['details']['identities'] == receipt['identities'],
                f'{name}: phase/worker base identity')
        built_clauses = phases[1]['details']['clauses']
        final_clauses = receipt['sat_clauses']
        clauses_valid = (final_clauses == built_clauses if mode == 'preflight'
                         else final_clauses == built_clauses + 530 if mode == 'positive'
                         else built_clauses <= final_clauses <= 8_000_000)
        require(receipt['columns'] == (33_254 if len(set(shape)) == 2 else 16_627)
                and receipt['sat_variables'] == phases[1]['details']['variables']
                and clauses_valid
                and receipt['sat_xor_rows'] == phases[1]['details']['xor_rows']
                and receipt['ordered_first_pair'] is True,
                f'{name}: model receipt')
        if mode == 'positive':
            require(phases[2]['phase'] == 'positive_choices_pinned'
                    and phases[2]['details'] == {'selectors': 4, 'phases': 28},
                    f'{name}: constructed selector/phase pins')
        require(0 < receipt['memory_cgroup_peak_bytes'] <= EIGHT_GIB
                and receipt['memory_cgroup_limit_bytes'] == EIGHT_GIB,
                f'{name}: memory receipt')
        status = receipt['status']
        allowed = {'preflight': {'PASS_PREFLIGHT'},
                   'positive': {'PASS_CONSTRUCTED_CONTROL'},
                   'natural': {'UNKNOWN_search_cap', 'UNVERIFIED_MODEL_UNSAT',
                               'INVALID_MODEL', 'INVALID_GROUP_LIFT', 'INVALID_RELATION',
                               'PASS_NATURAL_RELATION_PENDING_REPLAY'}}[mode]
        require(status in allowed, f'{name}: status {status}')
        if mode == 'preflight':
            require(receipt['solver_stage_executed'] is False
                    and receipt['public_point'] == TARGETS[fixture][1]
                    and receipt['target'] == TARGETS[fixture][1]
                    and receipt['target_blake3'] == TARGETS[fixture][0],
                    f'{name}: preflight target')
        elif mode == 'natural':
            require(receipt['target'] == TARGETS[fixture][1]
                    and receipt['target_blake3'] == TARGETS[fixture][0],
                    f'{name}: natural target')
    require(receipt['candidate_id'] is None
            and receipt['selected_best_total_runtime'] is None,
            f'{name}: premature selection')
    usage = receipt['process_usage']
    require(0 < usage['cpu_seconds'] <= 610
            and 0 < usage['peak_rss_bytes'] <= EIGHT_GIB,
            f'{name}: process usage')
    if mode == 'positive' or status == 'PASS_NATURAL_RELATION_PENDING_REPLAY':
        require(len(local_extra) == 12, f'{name}: incomplete local replay files')
        relation = receipt['relation']
        require(relation['rank_after_insert'] == 1 and relation['group_verified'] is True
                and relation['tamper_rejected'] is True,
                f'{name}: relation check')
        if mode == 'positive':
            require(relation['matches_constructed_row'] is True
                    and relation['matches_constructed_indices'] is True,
                    f'{name}: constructed relation differs')
        replay = read_json(path / 'replay-arm64.json')
        require(replay['schema'] == 'n83.mixed-orbit-m4-replay/v1'
                and replay['status'] == 'PASS'
                and replay['worker_source_commit'] == SOURCE
                and replay['worker_sha256'] == hashlib.sha256(worker_path.read_bytes()).hexdigest()
                and replay['rank_after_insert'] == 1
                and replay['changed_point_rejected'] is True
                and replay['changed_row_rejected'] is True,
                f'{name}: independent replay')
        for mutation in ('changed-point', 'changed-row'):
            require(int((path / f'{mutation}.exit.txt').read_text()) != 0
                    and not (path / f'{mutation}-unexpected-success.json').exists(),
                    f'{name}: accepted changed {mutation}')
    else:
        require(not local_extra, f'{name}: replay files without checked relation')
    return {'name': name, 'mode': mode, 'shape': shape, 'fixture_index': fixture,
            'status': status, 'host_exit': exit_code,
            'start_utc': start.isoformat(), 'end_utc': end.isoformat(),
            'charged_arm_host_wall_s': (end - start).total_seconds(),
            'base_import_ms': phases[0]['details']['base_import_ms'],
            'model_domain_ms': phases[1]['details']['model_ms'],
            'sat_variables': phases[1]['details']['variables'],
            'sat_clauses': phases[1]['details']['clauses'],
            'last_progress_ms': progress[-1]['elapsed_wall_ms'] if progress else None,
            'sampled_memory_peak_bytes': max((r['memory_current_bytes'] for r in progress), default=None),
            'memory_cgroup_peak_bytes': receipt['memory_cgroup_peak_bytes'],
            'candidate_id': None, 'selected_best_total_runtime': None}


def audit(root: Path) -> dict:
    raw = root / 'spot-arms'
    present = [spec for spec in PLAN if (raw / spec[0]).is_dir()]
    require(present and present == PLAN[:len(present)], 'raw arms are not a frozen prefix')
    require({p.name for p in raw.iterdir() if p.is_dir()} == {s[0] for s in present},
            'unexpected raw arm')
    rows = [audit_arm(raw / name, name, mode, shape, fixture)
            for name, mode, shape, fixture in present]
    require(all(a['end_utc'] <= b['start_utc'] for a, b in zip(rows, rows[1:])),
            'overlapping or misordered arms')
    panel = read_json(root / 'panel-status.json')
    complete = len(rows) == len(PLAN)
    require((panel['status'] == 'COMPLETE') == complete, 'panel completion flag differs')
    entries = [f'{hashlib.sha256(p.read_bytes()).hexdigest()}  {p.relative_to(root)}'
               for p in sorted(raw.rglob('*')) if p.is_file()]
    result = {'schema': 'n83.mixed-orbit-m4-panel-audit/v1',
              'status': 'AUDITED_BOUNDED' if complete else 'AUDITED_INCOMPLETE',
              'source_commit': SOURCE, 'binary_sha256': BINARY,
              'rows': rows, 'raw_file_count': len(entries),
              'charged_arm_host_wall_s': sum(r['charged_arm_host_wall_s'] for r in rows),
              'candidate_id': None, 'total_index_calculus_runtime_ms': None,
              'selected_best_total_runtime': None}
    (root / 'raw-files.sha256').write_text('\n'.join(entries) + '\n')
    (root / 'audit.json').write_text(json.dumps(result, sort_keys=True, indent=2) + '\n')
    return result


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('panel', type=Path)
    args = parser.parse_args()
    report = audit(args.panel)
    print(json.dumps({'status': report['status'], 'arms': [(r['name'], r['status'])
                                                         for r in report['rows']]}))
