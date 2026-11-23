#!/usr/bin/env python3
"""Execute and mirror the frozen same-host N83 mixed-orbit panel."""

from __future__ import annotations

import argparse
from datetime import datetime, timezone
import hashlib
import ipaddress
import json
import os
from pathlib import Path
import shlex
import subprocess
import time


REMOTE = Path('/home/ec2-user/n83-mixed')
PREFLIGHTS = [(f'p-{shape.lower()}{fixture}', 'preflight', shape, fixture)
              for fixture in (0, 1) for shape in ('UUUU', 'UUDD', 'DDUU', 'DDDD')]
CONTROLS = [(f'a-{shape.lower()}', 'positive', shape, 0)
            for shape in ('UUUU', 'UUDD', 'DDUU', 'DDDD')]
NATURAL = [(f'n-{shape.lower()}{fixture}', 'natural', shape, fixture)
           for shape, fixture in (('UUUU', 0), ('UUDD', 0), ('DDUU', 0), ('DDDD', 0),
                                  ('DDDD', 1), ('DDUU', 1), ('UUDD', 1), ('UUUU', 1))]
PLAN = PREFLIGHTS + CONTROLS + NATURAL


def require(ok: bool, message: str) -> None:
    if not ok:
        raise RuntimeError(message)


def utc() -> str:
    return datetime.now(timezone.utc).isoformat()


def write_event(path: Path, event: dict) -> None:
    with path.open('a') as f:
        f.write(json.dumps({'utc': utc(), **event}, sort_keys=True) + '\n')
        f.flush()
        os.fsync(f.fileno())


def ssh_base(args: argparse.Namespace) -> list[str]:
    return ['ssh', '-i', str(args.ssh_key), '-o', f'UserKnownHostsFile={args.known_hosts}',
            '-o', 'StrictHostKeyChecking=yes', '-o', 'BatchMode=yes',
            '-o', 'ServerAliveInterval=20', '-o', 'ServerAliveCountMax=4',
            f'ec2-user@{args.host}']


def remote(args: argparse.Namespace, command: list[str], timeout: int = 60) -> subprocess.CompletedProcess[str]:
    return subprocess.run(ssh_base(args) + [shlex.join(command)],
                          capture_output=True, text=True, timeout=timeout)


def scp_dir(args: argparse.Namespace, remote_dir: Path, local_parent: Path) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        ['scp', '-q', '-r', '-i', str(args.ssh_key),
         '-o', f'UserKnownHostsFile={args.known_hosts}',
         '-o', 'StrictHostKeyChecking=yes',
         f'ec2-user@{args.host}:{remote_dir}', str(local_parent)],
        capture_output=True, text=True, timeout=90,
    )


def verify_host_hashes(directory: Path) -> None:
    lines = (directory / 'raw.sha256').read_text().splitlines()
    require(bool(lines), 'empty host raw manifest')
    seen: set[str] = set()
    for line in lines:
        digest, name = line.split('  ', 1)
        name = name.removeprefix('./')
        require(name not in seen and len(digest) == 64 and all(c in '0123456789abcdef' for c in digest),
                'malformed raw manifest')
        seen.add(name)
        path = directory / name
        require(path.is_file() and hashlib.sha256(path.read_bytes()).hexdigest() == digest,
                f'host/local raw mismatch: {name}')
    require({str(p.relative_to(directory)) for p in directory.rglob('*') if p.is_file()}
            == seen | {'raw.sha256'}, 'host raw manifest omits a file')


def run_replay(args: argparse.Namespace, shape: str, raw: Path) -> None:
    env = os.environ.copy()
    env['ICV1_FROZEN_SOURCE_DIR'] = str(args.snapshot)
    basic = [str(args.replay_binary), 'primary-mixed-replay-m4', shape,
             str(args.local_u), str(args.local_d), str(args.local_evidence),
             str(args.local_fixture)]
    worker = raw / 'worker.json'
    for label in ('original', 'changed-point', 'changed-row'):
        current = worker
        output = raw / ('replay-arm64.json' if label == 'original'
                        else f'{label}-unexpected-success.json')
        if label != 'original':
            altered = json.loads(worker.read_text())
            relation = altered['relation']
            if label == 'changed-point':
                relation['point_indices'][0] ^= 1
            else:
                old = relation['dense_decimal_blake3']
                relation['dense_decimal_blake3'] = ('0' if old[0] != '0' else '1') + old[1:]
            current = raw / f'{label}.worker.json'
            current.write_text(json.dumps(altered, sort_keys=True) + '\n')
        result = subprocess.run(basic + [str(current), str(output)],
                                env=env, capture_output=True, text=True, timeout=180)
        (raw / f'{label}.exit.txt').write_text(str(result.returncode) + '\n')
        (raw / f'{label}.stdout.log').write_text(result.stdout)
        (raw / f'{label}.stderr.log').write_text(result.stderr)
        if label == 'original':
            require(result.returncode == 0 and json.loads(output.read_text())['status'] == 'PASS',
                    f'{shape}: independent replay failed')
        else:
            require(result.returncode != 0 and not output.exists(),
                    f'{shape}: forged {label} was accepted')


def run_one(args: argparse.Namespace, events: Path, spec: tuple[str, str, str, int]) -> str:
    name, mode, shape, fixture = spec
    raw_root = args.output / 'spot-arms'
    partial_root = args.output / 'partial' / name
    raw_root.mkdir(parents=True, exist_ok=True)
    partial_root.mkdir(parents=True, exist_ok=True)
    require(not (raw_root / name).exists(), f'{name}: output already exists')
    remote_out = REMOTE / 'runs' / name
    command = ['bash', str(REMOTE / 'run-mixed-docker.sh'), mode, shape, str(fixture),
               str(remote_out), str(REMOTE / 'target/release/examples/koblitz_n83_factor_base_export'),
               str(REMOTE / 'inputs/U'), str(REMOTE / 'inputs/D'),
               str(REMOTE / 'source/research/n83_d18_subgroup_orbit_census_20261018'),
               str(REMOTE / 'inputs/U'), str(REMOTE / 'snapshot')]
    write_event(events, {'arm': name, 'event': 'start', 'mode': mode,
                         'shape': shape, 'fixture': fixture})
    started = time.monotonic()
    with (args.output / f'{name}.operator.stdout').open('w') as stdout, \
         (args.output / f'{name}.operator.stderr').open('w') as stderr:
        process = subprocess.Popen(ssh_base(args) + [shlex.join(command)],
                                   stdout=stdout, stderr=stderr)
        next_copy = started + 60
        while process.poll() is None:
            time.sleep(10)
            elapsed = time.monotonic() - started
            if elapsed > 630:
                process.terminate()
                write_event(events, {'arm': name, 'event': 'outer_operator_cap',
                                     'elapsed_s': elapsed})
                raise RuntimeError(f'{name}: outer operator cap')
            if time.monotonic() >= next_copy:
                stamp = f'{int(elapsed):04d}s'
                destination = partial_root / stamp
                destination.mkdir(parents=True, exist_ok=False)
                copy = scp_dir(args, remote_out, destination)
                write_event(events, {'arm': name, 'event': 'partial_mirror',
                                     'elapsed_s': elapsed, 'copy_exit': copy.returncode})
                print(f'{name}: {stamp} partial mirror exit {copy.returncode}', flush=True)
                next_copy += 60
        exit_code = process.returncode
    write_event(events, {'arm': name, 'event': 'host_command_end',
                         'exit': exit_code, 'elapsed_s': time.monotonic() - started})
    if exit_code != 255:
        manifest = remote(args, ['bash', '-lc',
            f'cd {shlex.quote(str(remote_out))} && find . -type f ! -name raw.sha256 -print0 | sort -z | xargs -0 sha256sum > raw.sha256'], 30)
        require(manifest.returncode == 0, f'{name}: host manifest failed: {manifest.stderr}')
        copy = scp_dir(args, remote_out, raw_root)
        require(copy.returncode == 0, f'{name}: final mirror failed: {copy.stderr}')
        raw = raw_root / name
        verify_host_hashes(raw)
    else:
        raise RuntimeError(f'{name}: SSH lost; retain partial mirrors and classify infrastructure')
    host_exit = int((raw / 'host-exit.txt').read_text())
    require(host_exit == exit_code, f'{name}: host/SSH exit mismatch')
    if exit_code == 0:
        worker = json.loads((raw / 'worker.json').read_text())
        status = worker['status']
        if mode == 'preflight':
            require(status == 'PASS_PREFLIGHT', f'{name}: preflight failed: {status}')
        elif mode == 'positive':
            require(status == 'PASS_CONSTRUCTED_CONTROL', f'{name}: positive failed: {status}')
            run_replay(args, shape, raw)
        elif status == 'PASS_NATURAL_RELATION_PENDING_REPLAY':
            run_replay(args, shape, raw)
    elif exit_code == 124 and mode == 'natural':
        cap = json.loads((raw / 'worker.json.cap.json').read_text())
        require(cap['status'] == 'UNKNOWN_budget', f'{name}: cap status differs')
        status = 'UNKNOWN_budget'
    else:
        raise RuntimeError(f'{name}: unexpected host exit {exit_code}')
    write_event(events, {'arm': name, 'event': 'mirrored_verified', 'status': status})
    print(f'{name}: {status} ({time.monotonic() - started:.1f}s)', flush=True)
    return status


def main() -> None:
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--host', required=True)
    p.add_argument('--ssh-key', type=Path, required=True)
    p.add_argument('--known-hosts', type=Path, required=True)
    p.add_argument('--replay-binary', type=Path, required=True)
    p.add_argument('--snapshot', type=Path, required=True)
    p.add_argument('--local-u', type=Path, required=True)
    p.add_argument('--local-d', type=Path, required=True)
    p.add_argument('--local-evidence', type=Path, required=True)
    p.add_argument('--local-fixture', type=Path, required=True)
    p.add_argument('--output', type=Path, required=True)
    args = p.parse_args()
    ipaddress.IPv4Address(args.host)
    require(not args.output.exists(), 'panel output already exists')
    for path in (args.ssh_key, args.known_hosts, args.replay_binary,
                 args.snapshot / 'attestation.json', args.local_u,
                 args.local_d, args.local_evidence, args.local_fixture):
        require(path.exists(), f'missing local prerequisite: {path}')
    args.output.mkdir(parents=True)
    events = args.output / 'panel-events.jsonl'
    try:
        setup = remote(args, ['mkdir', '-p', str(REMOTE / 'runs')])
        require(setup.returncode == 0, f'host run directory setup failed: {setup.stderr}')
        write_event(events, {'event': 'host_run_directory_ready'})
        for spec in PLAN:
            run_one(args, events, spec)
    except Exception as exc:
        write_event(events, {'event': 'panel_stopped', 'error': str(exc)})
        (args.output / 'panel-status.json').write_text(
            json.dumps({'status': 'INCOMPLETE', 'error': str(exc)}, indent=2) + '\n')
        raise
    (args.output / 'panel-status.json').write_text(
        json.dumps({'status': 'COMPLETE', 'arm_count': len(PLAN)}, indent=2) + '\n')
    write_event(events, {'event': 'panel_complete', 'arm_count': len(PLAN)})
    print(json.dumps({'status': 'COMPLETE', 'arm_count': len(PLAN)}), flush=True)


if __name__ == '__main__':
    main()
