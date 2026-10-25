#!/usr/bin/env python3
"""Stage and recover the two budgeted N83 RunPod strata over provider SSH."""

import argparse
from concurrent.futures import ThreadPoolExecutor
from datetime import datetime, timezone
import ipaddress
import json
from pathlib import Path
import subprocess
import tarfile
import time

from research.n83_crossfamily_current_20261025 import allocate_runpod
from research.n83_crossfamily_current_20261025 import aggregate_runpod
from research.n83_crossfamily_current_20261025 import source_bundle


HERE = Path(__file__).resolve().parent
QUERY = ("query { myself { pods { id name machineId desiredStatus "
         "runtime { ports { ip isIpPublic privatePort publicPort type } } } } }")
REMOTE_BUNDLE = '/workspace/n83-crossfamily-bundle'
REMOTE_SOURCE = '/workspace/crypto'
SSH_OPTIONS = ('-o', 'BatchMode=yes', '-o', 'ConnectTimeout=15',
               '-o', 'ServerAliveInterval=20',
               '-o', 'ServerAliveCountMax=3',
               '-o', 'StrictHostKeyChecking=accept-new')


def write(path, value):
    path.write_text(json.dumps(value, sort_keys=True, indent=2) + '\n')


def sealed(allocation_dir, bundle_dir):
    snapshot = json.loads((allocation_dir / 'provider_snapshot.json').read_text())
    bundle = source_bundle.verify(bundle_dir / 'source.tar.gz',
                                  bundle_dir / 'manifest.json')
    manifest_sha = source_bundle.digest(bundle_dir / 'manifest.json')
    if (snapshot.get('schema') != 'n83.crossfamily-current-runpod-snapshot/v1' or
            snapshot.get('source_head_sha') != bundle['source_head_sha'] or
            snapshot.get('source_archive_sha256') !=
            bundle['source_archive_sha256'] or
            snapshot.get('source_manifest_sha256') != manifest_sha or
            len(snapshot.get('pods', [])) != 2):
        raise ValueError('allocation snapshot differs from source bundle')
    allocations = []
    for index in range(2):
        path = allocation_dir / f'allocation_s{index}.json'
        record = json.loads(path.read_text())
        if (record.get('schema') !=
                'n83.crossfamily-current-runpod-allocation/v1' or
                record.get('stratum') != index or
                record.get('source_head_sha') != bundle['source_head_sha'] or
                record.get('source_archive_sha256') !=
                bundle['source_archive_sha256'] or
                record.get('source_manifest_sha256') != manifest_sha or
                record.get('snapshot_sha256') !=
                source_bundle.digest(allocation_dir / 'provider_snapshot.json') or
                any(record.get(key) != snapshot['pods'][index].get(key)
                    for key in ('pod_id', 'machine_id', 'desired_status',
                                'cost_per_hr', 'image_name'))):
            raise ValueError(f'stratum {index} allocation differs')
        allocations.append(record)
    if (len({r['pod_id'] for r in allocations}) != 2 or
            len({r['machine_id'] for r in allocations}) != 2):
        raise ValueError('two distinct Pods required')
    return bundle, allocations


def endpoints(pods, allocations):
    result = {}
    for record in allocations:
        matches = [pod for pod in pods if pod.get('id') == record['pod_id']]
        if (len(matches) != 1 or
                matches[0].get('machineId') != record['machine_id'] or
                matches[0].get('desiredStatus') != 'RUNNING'):
            raise ValueError('provider Pod state differs from allocation')
        ssh_ports = []
        for port in (matches[0].get('runtime') or {}).get('ports') or []:
            if (port.get('privatePort') == 22 and
                    port.get('isIpPublic') is True and
                    type(port.get('publicPort')) is int and
                    0 < port['publicPort'] <= 65535):
                try:
                    ip = str(ipaddress.ip_address(port.get('ip')))
                except ValueError:
                    continue
                ssh_ports.append((ip, port['publicPort']))
        if len(set(ssh_ports)) != 1:
            raise ValueError('one public provider SSH endpoint required')
        result[record['stratum']] = ssh_ports[0]
    return result


def provider_endpoints(key, allocations):
    for attempt in range(60):
        pods = allocate_runpod.graphql(QUERY, key)['myself']['pods']
        try:
            return endpoints(pods, allocations)
        except ValueError:
            if attempt == 59:
                raise
            time.sleep(2)


def ssh_base(endpoint, known_hosts):
    ip, port = endpoint
    return ['ssh', *SSH_OPTIONS, '-o',
            'UserKnownHostsFile=' + str(known_hosts),
            '-p', str(port), 'root@' + ip]


def scp_base(endpoint, known_hosts):
    _, port = endpoint
    return ['scp', *SSH_OPTIONS, '-o',
            'UserKnownHostsFile=' + str(known_hosts),
            '-P', str(port)]


def remote(endpoint, known_hosts, command, capture=False):
    process = subprocess.run(ssh_base(endpoint, known_hosts) + [command],
                             check=True, capture_output=capture, text=True)
    return process.stdout.strip() if capture else None


def remote_spec(endpoint, path):
    ip, _ = endpoint
    host = '[' + ip + ']' if ':' in ip else ip
    return f'root@{host}:{path}'


def stage_one(index, endpoint, bundle_dir, allocation_dir, known_hosts):
    allocation = allocation_dir / f'allocation_s{index}.json'
    remote(endpoint, known_hosts,
           f'mkdir -p {REMOTE_BUNDLE} {REMOTE_SOURCE} '
           f'/workspace/n83-crossfamily-s{index}')
    for path in (bundle_dir / 'source.tar.gz',
                 bundle_dir / 'manifest.json', allocation):
        destination = (f'{REMOTE_BUNDLE}/{path.name}' if path != allocation else
                       f'{REMOTE_BUNDLE}/allocation_s{index}.json')
        subprocess.run(scp_base(endpoint, known_hosts) +
                       [str(path), remote_spec(endpoint, destination)], check=True)
    remote(endpoint, known_hosts,
           f'tar -C {REMOTE_SOURCE} -xzf {REMOTE_BUNDLE}/source.tar.gz')
    launcher = (f'{REMOTE_SOURCE}/research/'
                'n83_crossfamily_current_20261025/launch_on_pod.sh')
    remote(endpoint, known_hosts,
           f'nohup bash {launcher} {index} {REMOTE_BUNDLE} '
           f'{REMOTE_BUNDLE}/allocation_s{index}.json '
           f'> /workspace/n83-crossfamily-s{index}/dispatch.log '
           '2>&1 < /dev/null &')
    return {'stratum': index, 'status': 'LAUNCHED',
            'pod_endpoint': f'{endpoint[0]}:{endpoint[1]}'}


def retire_failed_stage(output, allocations, key, failures, launched=()):
    cleanup = {}
    for record in allocations:
        try:
            cleanup[record['pod_id']] = allocate_runpod.terminate(
                record['pod_id'], key)
        except Exception:
            cleanup[record['pod_id']] = False
    write(output / 'dispatch_failure.json',
          {'launched': list(launched), 'failures': failures,
           'terminated_and_absent': cleanup})


def stage(allocation_dir, bundle_dir, output, key):
    bundle, allocations = sealed(allocation_dir, bundle_dir)
    if output.exists():
        raise ValueError('dispatch output already exists')
    output.mkdir(parents=True)
    try:
        addresses = provider_endpoints(key, allocations)
    except Exception as error:
        retire_failed_stage(output, allocations, key,
                            [{'phase': 'provider_endpoint',
                              'error_type': type(error).__name__}])
        raise
    with ThreadPoolExecutor(max_workers=2) as workers:
        futures = [workers.submit(stage_one, i, addresses[i], bundle_dir,
                                  allocation_dir,
                                  output / f'known_hosts_s{i}') for i in range(2)]
        launched, failures = [], []
        for index, future in enumerate(futures):
            try:
                launched.append(future.result())
            except Exception as error:
                failures.append({'stratum': index,
                                 'error_type': type(error).__name__})
    if failures:
        retire_failed_stage(output, allocations, key, failures, launched)
        raise ValueError('RunPod staging failed; inspect dispatch failure receipt')
    record = {'schema': 'n83.crossfamily-runpod-dispatch/v1',
              'source_head_sha': bundle['source_head_sha'],
              'source_archive_sha256': bundle['source_archive_sha256'],
              'strata': launched}
    write(output / 'dispatch.json', record)
    return record


def download_one(index, endpoint, output, known_hosts):
    base = f'/workspace/n83-crossfamily-s{index}'
    status = remote(endpoint, known_hosts, f'cat {base}/run.status', True)
    if not status.startswith('EXIT '):
        return {'stratum': index, 'status': 'IN_PROGRESS', 'remote_status': status}
    target = output / f'stratum{index}'
    target.mkdir(exist_ok=True)
    for name in ('run.status', 'source_check.json', 'preflight.json',
                 'apt.log', 'rustup.log', 'toolchain.log', 'rustc.txt',
                 'ic_tests.log', 'rho_tests.log', 'build.log',
                 'cpu_affinity_ids.txt', 'producer.log', 'producer.exit',
                 'replay.log', 'replay.exit', 'dispatch.log',
                 'raw_results.sha256', 'raw_results.tar.gz'):
        destination = target / name
        if destination.exists():
            continue
        process = subprocess.run(scp_base(endpoint, known_hosts) +
                                 [remote_spec(endpoint, f'{base}/{name}'),
                                  str(destination)], capture_output=True)
        if process.returncode != 0:
            destination.unlink(missing_ok=True)
    archive = target / 'raw_results.tar.gz'
    checksum = target / 'raw_results.sha256'
    if archive.is_file() and checksum.is_file():
        if checksum.read_text().split()[0] != source_bundle.digest(archive):
            raise ValueError('transferred raw archive checksum differs')
        results = target / 'results'
        if not results.exists():
            with tarfile.open(archive, 'r:gz') as stream:
                for member in stream:
                    path = Path(member.name)
                    if (path.is_absolute() or '..' in path.parts or
                            not path.parts or path.parts[0] != 'results' or
                            not (member.isfile() or member.isdir())):
                        raise ValueError('unsafe raw archive member')
                stream.extractall(target, filter='data')
        bound = aggregate_runpod.bind_archive(archive, results)
        return {'stratum': index, 'status': 'RECOVERED', **bound}
    remote_archive = remote(endpoint, known_hosts,
                            f'if test -f {base}/raw_results.tar.gz; '
                            'then echo PRESENT; else echo ABSENT; fi', True)
    if remote_archive != 'ABSENT':
        return {'stratum': index, 'status': 'RECOVERY_PENDING',
                'remote_status': status}
    return {'stratum': index, 'status': 'FAILED_BEFORE_RAW_ARCHIVE',
            'remote_status': status}


def cleanup(allocations, output, key):
    statuses = {}
    for record in allocations:
        index, pod_id = record['stratum'], record['pod_id']
        path = output / f'termination_s{index}.json'
        if path.is_file() and json.loads(path.read_text()).get(
                'terminated_and_absent') is True:
            statuses[index] = True
            continue
        try:
            statuses[index] = allocate_runpod.terminate(pod_id, key)
        except Exception:
            statuses[index] = False
        write(path, {'pod_id': pod_id,
                     'terminated_and_absent': statuses[index]})
    return statuses


def collect(allocation_dir, bundle_dir, output, key):
    _, allocations = sealed(allocation_dir, bundle_dir)
    if not (output / 'dispatch.json').is_file():
        raise ValueError('dispatch receipt is absent')
    prior = output / 'collection.json'
    if prior.is_file():
        recorded = json.loads(prior.read_text())['strata']
        if all(item['status'] in ('RECOVERED', 'FAILED_BEFORE_RAW_ARCHIVE')
               for item in recorded):
            return {'strata': recorded,
                    'terminated_and_absent': cleanup(allocations, output, key)}
    addresses = provider_endpoints(key, allocations)
    with ThreadPoolExecutor(max_workers=2) as workers:
        futures = [workers.submit(download_one, i, addresses[i], output,
                                  output / f'known_hosts_s{i}')
                   for i in range(2)]
        recovered = [future.result() for future in futures]
    write(output / 'collection.json', {'strata': recovered})
    finished = all(item['status'] in
                   ('RECOVERED', 'FAILED_BEFORE_RAW_ARCHIVE')
                   for item in recovered)
    terminations = cleanup(allocations, output, key) if finished else None
    return {'strata': recovered, 'terminated_and_absent': terminations}


def watch(allocation_dir, bundle_dir, output, key, poll_seconds=30):
    plan = json.loads((allocation_dir / 'allocation_plan.json').read_text())
    if plan.get('schema') != 'n83.crossfamily-current-allocation-plan/v1':
        raise ValueError('allocation deadline plan differs')
    deadline = datetime.fromisoformat(
        plan['terminate_at_utc'].replace('Z', '+00:00'))
    if deadline.tzinfo is None:
        raise ValueError('allocation deadline has no timezone')
    errors = []
    while datetime.now(timezone.utc) < deadline:
        try:
            state = collect(allocation_dir, bundle_dir, output, key)
            if state['terminated_and_absent'] == {0: True, 1: True}:
                status = ('PASS_ARCHIVES_AND_POD_ABSENCE' if
                          all(item['status'] == 'RECOVERED' for item in
                              state['strata']) else
                          'PODS_ABSENT_WITH_FAILED_STAGE')
                write(output / 'watch.json',
                      {'status': status,
                       'strata': state['strata'], 'errors': errors})
                return state
        except Exception as error:
            errors.append({'utc': datetime.now(timezone.utc).isoformat(),
                           'error_type': type(error).__name__})
            write(output / 'watch_errors.json', errors)
        time.sleep(poll_seconds)
    write(output / 'watch.json', {'status': 'DEADLINE_WITHOUT_COMPLETE_CUSTODY',
                                 'errors': errors})
    raise ValueError('RunPod deadline reached before complete custody')


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--allocation-dir', type=Path, required=True)
    parser.add_argument('--source-bundle', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    action = parser.add_mutually_exclusive_group(required=True)
    action.add_argument('--stage', action='store_true')
    action.add_argument('--collect', action='store_true')
    action.add_argument('--watch', action='store_true')
    action.add_argument('--stage-and-watch', action='store_true')
    args = parser.parse_args()
    key = allocate_runpod.configured_key()
    if args.stage or args.stage_and_watch:
        result = stage(args.allocation_dir.resolve(),
                       args.source_bundle.resolve(), args.output.resolve(), key)
        if args.stage_and_watch:
            result = watch(args.allocation_dir.resolve(),
                           args.source_bundle.resolve(), args.output.resolve(), key)
    elif args.watch:
        result = watch(args.allocation_dir.resolve(),
                       args.source_bundle.resolve(), args.output.resolve(), key)
    else:
        result = collect(args.allocation_dir.resolve(),
                         args.source_bundle.resolve(), args.output.resolve(), key)
    print(json.dumps(result, sort_keys=True))


if __name__ == '__main__':
    main()
