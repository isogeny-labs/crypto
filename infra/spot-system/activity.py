#!/usr/bin/env python3
"""Publish conservative worker activity; a missing/broken probe means unknown."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import time
import urllib.request


def collect(root, nodes, proc=Path('/proc')):
    pending, busy, unknown = 0, False, False
    if os.environ.get('SPOT_LEGACY_ENABLED') == 'true':
        try:
            for path in os.environ.get('SPOT_LEGACY_STATUS_FILES','/run/spot-legacy/status.json').split(':'):
                legacy = json.loads(Path(path).read_text())
                unknown = unknown or not (0 <= time.time() - legacy['observed'] <= 90) or legacy['state'] == 'error'
                pending += int(legacy['pending'])
        except (OSError, ValueError, KeyError, TypeError):
            unknown = True
    try:
        jobs = json.loads((root / 'jobs.json').read_text())
        if not isinstance(jobs, list):
            raise ValueError('Invalid manifest')
        for job in jobs:
            name = job['id']
            if not name or any(c not in 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-' for c in name):
                raise ValueError('Unsafe id')
            fingerprint = hashlib.sha256(json.dumps(job, sort_keys=True).encode()).hexdigest()
            complete = root / 'state' / name / 'complete.json'
            if not complete.exists() or json.loads(complete.read_text()).get('sha256') != fingerprint:
                pending += 1
    except (OSError, ValueError, KeyError, TypeError):
        unknown = True
    # Check legacy workers independently: a missing manifest must not skip this.
    try:
        for p in proc.iterdir():
            if p.name.isdigit():
                try:
                    name = (p / 'comm').read_text().strip()
                    if name.startswith('factor_base_sha'):
                        busy = True
                    if int(p.name) == os.getpid():
                        continue
                    uid_line = next(line for line in (p/'status').read_text().splitlines() if line.startswith('Uid:'))
                    if int(uid_line.split()[1]) != os.getuid():
                        continue
                    # Fresh workers host only these idle shells/system daemons and
                    # our own node/supervisor processes. Any other user process is
                    # conservatively treated as work, including unregistered jobs.
                    if name in {'bash','sh','zsh','fish','sleep','systemd','sd-pam','dbus-daemon',
                                'dbus-broker','sshd','sshd-session','ssh-agent','postgres',
                                'cairn','conductord'}:
                        continue
                    command = (p/'cmdline').read_bytes().split(b'\0')
                    discovery = Path(__file__).with_name('bin') / 'checkpoint-discovery'
                    if (p/'exe').is_symlink() and (p/'exe').resolve() == discovery.resolve():
                        continue
                    own_scripts = {str(Path(__file__).with_name(n)).encode() for n in
                                   ['activity.py','node_service.py','runner.py']}
                    if name.startswith('python') and len(command)>1 and command[1] in own_scripts:
                        continue
                    busy = True
                except FileNotFoundError:
                    pass
    except (OSError, ValueError, StopIteration):
        unknown = True
    try:
        opener = urllib.request.build_opener(urllib.request.ProxyHandler({}))
        with opener.open('http://127.0.0.1:9180/health', timeout=3) as r:
            if r.status != 200:
                raise ValueError('Cairn unhealthy')
        for queue in ['queue', 'running']:
            path = nodes / 'agent' / 'jobs' / queue
            if not path.is_dir():
                raise ValueError('Cairn spool missing')
            pending += sum(p.is_file() for p in path.iterdir())
        # Query the authoritative local database over its private Unix socket.
        # Any schema/connection failure blocks shutdown instead of guessing idle.
        result = subprocess.check_output([
            'psql', 'postgresql:///conductor?host=/run/spot-postgresql', '-At', '-v', 'ON_ERROR_STOP=1',
            '-c', "SELECT count(*) FROM tasks WHERE status NOT IN ('done','cancelled','superseded','failed')"
        ], text=True, timeout=8)
        pending += int(result.strip())
        # A crashed control plane must not be mistaken for an empty task queue.
        subprocess.run(['systemctl', 'is-active', '--quiet', 'spot-nodes.service'],
                       check=True, timeout=5)
    except (OSError, ValueError, subprocess.SubprocessError):
        unknown = True
    return {'Busy': busy or pending > 0, 'Pending': pending, 'Unknown': unknown}


def instance_id():
    opener = urllib.request.build_opener(urllib.request.ProxyHandler({}))
    base = 'http://169.254.169.254/latest/'
    req = urllib.request.Request(base + 'api/token', method='PUT',
        headers={'X-aws-ec2-metadata-token-ttl-seconds': '60'})
    token = opener.open(req, timeout=3).read().decode()
    req = urllib.request.Request(base + 'meta-data/instance-id',
                                headers={'X-aws-ec2-metadata-token': token})
    return opener.open(req, timeout=3).read().decode()


def publish(root, nodes, table, region):
    from worker_state import lease, call
    with lease():
        _publish_locked(root, nodes, call)


def _publish_locked(root, nodes, call):
    now = int(time.time())
    status = collect(root, nodes)
    current = 'activity#' + instance_id()
    prior = call('get-item', key={'Id': {'S': current}}, consistent_read=True).get('Item', {})
    last_busy = now if status['Busy'] or status['Unknown'] else int(prior.get('LastBusy', {}).get('N', str(now)))
    item = {'Id': {'S': current}, 'Observed': {'N': str(now)}, 'LastBusy': {'N': str(last_busy)},
            'Expires': {'N': str(now + 600)}, 'Busy': {'BOOL': status['Busy']},
            'Pending': {'N': str(status['Pending'])}, 'Unknown': {'BOOL': status['Unknown']}}
    call('put-item', item=item)


if __name__ == '__main__':
    p = argparse.ArgumentParser()
    p.add_argument('--root', type=Path, default=Path('/srv/experiments'))
    p.add_argument('--nodes', type=Path, default=Path('/srv/experiments/nodes'))
    p.add_argument('--inspect', action='store_true')
    a = p.parse_args()
    if a.inspect:
        print(json.dumps(collect(a.root, a.nodes)))
    else:
        while True:
            try:
                publish(a.root, a.nodes, os.environ['SPOT_ACTIVITY_TABLE'], os.environ['AWS_REGION'])
            except (OSError, ValueError, KeyError, RuntimeError, subprocess.SubprocessError) as exc:
                print('Activity report failed:', type(exc).__name__, flush=True)
            time.sleep(30)
