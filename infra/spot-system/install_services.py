#!/usr/bin/env python3
"""Install worker units; --nodes-only also works on the current non-ASG host."""
import argparse
import os
from pathlib import Path
import pwd
import subprocess


def units(runtime, binaries, project, table='', region='us-east-2'):
    common = '[Unit]\nAfter=network-online.target\nRequiresMountsFor=/srv/experiments\n[Service]\nUser=ec2-user\n'
    node = common + f'''RuntimeDirectory=spot-postgresql
RuntimeDirectoryMode=0700
ExecStart=/usr/bin/python3 {runtime}/node_service.py --binary-dir {binaries} --project {project}
Restart=on-failure
RestartSec=15
TimeoutStopSec=90
KillMode=mixed
[Install]
WantedBy=multi-user.target
'''
    environment = f'Environment=AWS_REGION={region}\nEnvironment=SPOT_ACTIVITY_TABLE={table}\n'
    experiment = common + environment + f'''ExecStart=/usr/bin/python3 {runtime}/runner.py --root /srv/experiments
Restart=on-failure
RestartSec=20
TimeoutStopSec=90
KillMode=mixed
[Install]
WantedBy=multi-user.target
'''
    activity = common + environment + f'''ExecStart=/usr/bin/python3 {runtime}/activity.py
Restart=on-failure
RestartSec=15
TimeoutStopSec=30
[Install]
WantedBy=multi-user.target
'''
    return {'spot-nodes': node, 'spot-experiments': experiment, 'spot-activity': activity}


def discovery_units(runtime, search_roots):
    # systemd double-quoted arguments use backslash escaping, not shell quoting.
    def quoted(path):
        value = str(path)
        if any(c in value for c in '\n\r\x00'):
            raise ValueError('Invalid unit argument')
        return '"' + value.replace('\\', '\\\\').replace('"', '\\"').replace('%', '%%').replace('$', '$$') + '"'
    extra = ''.join(' --search-root ' + quoted(root) for root in search_roots)
    service = f'''[Unit]
Description=Discover experiment checkpoints without starting workloads
After=network-online.target
RequiresMountsFor=/srv/experiments
[Service]
Type=oneshot
User=ec2-user
ExecStart={quoted(runtime / 'bin/checkpoint-discovery')} --root /srv/experiments{extra} --output /srv/experiments/discovery/current-host.json
NoNewPrivileges=true
Nice=10
TimeoutStartSec=120
'''
    timer = '''[Unit]
Description=Refresh the durable checkpoint inventory
[Timer]
OnBootSec=30
OnUnitInactiveSec=60
Unit=spot-checkpoint-discovery.service
[Install]
WantedBy=timers.target
'''
    return {'spot-checkpoint-discovery.service': service, 'spot-checkpoint-discovery.timer': timer}


if __name__ == '__main__':
    p = argparse.ArgumentParser()
    p.add_argument('--runtime', type=Path, default=Path('/opt/spot'))
    p.add_argument('--binaries', type=Path, default=Path('/opt/spot/bin'))
    p.add_argument('--project', type=Path, required=True)
    p.add_argument('--table', default='')
    p.add_argument('--region', default='us-east-2')
    p.add_argument('--nodes-only', action='store_true')
    p.add_argument('--discovery-only', action='store_true')
    p.add_argument('--search-root', type=Path, action='append', default=[])
    a = p.parse_args()
    if a.nodes_only and a.discovery_only:
        p.error('Choose only one installation mode')
    if os.geteuid() != 0:
        raise SystemExit('Run with sudo to install system units')
    if not os.path.ismount('/srv/experiments'):
        raise SystemExit('Durable filesystem must be mounted first')
    identity = pwd.getpwnam('ec2-user')
    if not a.discovery_only:
        node_root = Path('/srv/experiments/nodes')
        node_root.mkdir(exist_ok=True, mode=0o700)
        os.chown(node_root, identity.pw_uid, identity.pw_gid)
    names = []
    if not a.discovery_only:
        for name, text in units(a.runtime, a.binaries, a.project, a.table, a.region).items():
            if a.nodes_only and name != 'spot-nodes':
                continue
            Path('/etc/systemd/system/'+name+'.service').write_text(text)
            names.append(name+'.service')
    if not a.nodes_only:
        discovery_root = Path('/srv/experiments/discovery')
        discovery_root.mkdir(exist_ok=True, mode=0o700)
        os.chown(discovery_root, identity.pw_uid, identity.pw_gid)
        for name, text in discovery_units(a.runtime, a.search_root).items():
            Path('/etc/systemd/system/'+name).write_text(text)
        names.append('spot-checkpoint-discovery.timer')
    subprocess.run(['systemctl', 'daemon-reload'], check=True)
    subprocess.run(['systemctl', 'enable', '--now', *names], check=True)
