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


if __name__ == '__main__':
    p = argparse.ArgumentParser()
    p.add_argument('--runtime', type=Path, default=Path('/opt/spot'))
    p.add_argument('--binaries', type=Path, default=Path('/opt/spot/bin'))
    p.add_argument('--project', type=Path, required=True)
    p.add_argument('--table', default='')
    p.add_argument('--region', default='us-east-2')
    p.add_argument('--nodes-only', action='store_true')
    a = p.parse_args()
    if os.geteuid() != 0:
        raise SystemExit('Run with sudo to install system units')
    if not os.path.ismount('/srv/experiments'):
        raise SystemExit('Durable filesystem must be mounted first')
    identity = pwd.getpwnam('ec2-user')
    node_root = Path('/srv/experiments/nodes')
    node_root.mkdir(exist_ok=True, mode=0o700)
    os.chown(node_root, identity.pw_uid, identity.pw_gid)
    names = []
    for name, text in units(a.runtime, a.binaries, a.project, a.table, a.region).items():
        if a.nodes_only and name != 'spot-nodes':
            continue
        Path('/etc/systemd/system/'+name+'.service').write_text(text)
        names.append(name+'.service')
    subprocess.run(['systemctl', 'daemon-reload'], check=True)
    subprocess.run(['systemctl', 'enable', '--now', *names], check=True)
