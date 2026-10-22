#!/usr/bin/env python3
"""Install worker units; --nodes-only also works on the current non-ASG host."""
import argparse
import os
from pathlib import Path
import pwd
import subprocess


def preserve_database_ipc(directory=Path('/etc/systemd/logind.conf.d')):
    # PostgreSQL runs as the existing workspace user. logind must not remove
    # its shared memory when the last interactive/runuser session exits.
    directory.mkdir(parents=True, exist_ok=True)
    path = directory / '60-spot-postgresql.conf'
    body = '[Login]\nRemoveIPC=no\n'
    if not path.exists() or path.read_text() != body:
        path.write_text(body)
        subprocess.run(['systemctl', 'restart', 'systemd-logind'], check=True)


def units(runtime, binaries, project, table='', region='us-east-2', root=Path('/srv/experiments'), legacy_origin=''):
    common = '[Unit]\nAfter=network-online.target\nRequiresMountsFor=/srv/experiments\n[Service]\nUser=ec2-user\n'
    node = common + f'''RuntimeDirectory=spot-postgresql
RuntimeDirectoryMode=0700
ExecStart=/usr/bin/python3 {runtime}/node_service.py --root {root}/nodes --binary-dir {binaries} --project {project}
Restart=on-failure
RestartSec=15
TimeoutStopSec=90
KillMode=mixed
[Install]
WantedBy=multi-user.target
'''
    environment = f'Environment=AWS_REGION={region}\nEnvironment=SPOT_ACTIVITY_TABLE={table}\n'
    if legacy_origin:
        environment += 'Environment=SPOT_LEGACY_ENABLED=true\n'
    experiment = common + environment + f'''ExecStart=/usr/bin/python3 {runtime}/runner.py --root {root}
Restart=on-failure
RestartSec=20
TimeoutStopSec=90
KillMode=mixed
[Install]
WantedBy=multi-user.target
'''
    activity = common + environment + f'''ExecStart=/usr/bin/python3 {runtime}/activity.py --root {root} --nodes {root}/nodes
Restart=on-failure
RestartSec=15
TimeoutStopSec=30
[Install]
WantedBy=multi-user.target
'''
    result = {'spot-nodes': node, 'spot-experiments': experiment, 'spot-activity': activity}
    result['spot-codex-recovery'] = f'''[Unit]
Description=Recover enrolled Codex chats from verified durable state
After=network-online.target
RequiresMountsFor=/srv/experiments
ConditionPathExists=/srv/experiments/agent-migration/agent.json
[Service]
User=root
Type=notify
NotifyAccess=main
TimeoutStartSec=0
UMask=0077
RuntimeDirectory=spot-legacy
RuntimeDirectoryMode=0755
RuntimeDirectoryPreserve=yes
ExecStart=/usr/bin/python3 {runtime}/codex_recovery.py --config /srv/experiments/agent-migration/agent.json
Restart=on-failure
RestartSec=20
TimeoutStopSec=120
KillMode=control-group
[Install]
WantedBy=multi-user.target
'''
    if legacy_origin:
        # A failed initial notify start cancels dependent start jobs. Queue them
        # again after every successful retry, without waiting on their After=.
        result['spot-codex-recovery'] = result['spot-codex-recovery'].replace(
            'Restart=on-failure',
            'ExecStartPost=/usr/bin/systemctl start --no-block spot-legacy-recovery.service spot-diem-recovery.service\nRestart=on-failure')
        result['spot-legacy-recovery'] = f'''[Unit]
After=network-online.target spot-codex-recovery.service
Requires=spot-codex-recovery.service
[Service]
Type=simple
User=root
UMask=0077
RuntimeDirectory=spot-legacy
RuntimeDirectoryMode=0755
RuntimeDirectoryPreserve=yes
ExecStart=/usr/bin/python3 /opt/spot-criu/legacy_recovery.py --origin {legacy_origin} --table {table} --status /run/spot-legacy/status.json
Restart=on-failure
RestartSec=15
KillMode=process
[Install]
WantedBy=multi-user.target
'''
        result['spot-diem-recovery']=result['spot-legacy-recovery'].replace(
            '--origin '+legacy_origin, '--root /var/lib/spot-criu-diem --owner-key legacy-owner-diem --pointer-key checkpoints/criu/diem/latest.json --backup-timer spot-diem-backup.timer --origin '+legacy_origin).replace('/run/spot-legacy/status.json','/run/spot-legacy/diem.json')
        result['spot-activity']=result['spot-activity'].replace('Environment=SPOT_LEGACY_ENABLED=true','Environment=SPOT_LEGACY_ENABLED=true\nEnvironment=SPOT_LEGACY_STATUS_FILES=/run/spot-legacy/status.json:/run/spot-legacy/diem.json')
    return result


def discovery_units(runtime, search_roots, root=Path('/srv/experiments')):
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
ExecStart={quoted(runtime / 'bin/checkpoint-discovery')} --root {root}{extra} --max-hash-bytes 68719476736 --output {root}/discovery/current-host.json
NoNewPrivileges=true
Nice=10
TimeoutStartSec=1800
'''
    timer = '''[Unit]
Description=Refresh the durable checkpoint inventory
[Timer]
OnBootSec=30
OnUnitInactiveSec=900
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
    p.add_argument('--root', type=Path, default=Path('/srv/experiments'))
    p.add_argument('--table', default='')
    p.add_argument('--region', default='us-east-2')
    p.add_argument('--stack', help='load the managed-wake function from this stack output')
    p.add_argument('--legacy-origin', default='')
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
    a.root = a.root.resolve(strict=True)
    if not a.root.is_relative_to(Path('/srv/experiments')) or not a.root.is_dir():
        raise SystemExit('Worker root must be a directory within the durable mount')
    identity = pwd.getpwnam('ec2-user')
    if not a.discovery_only:
        preserve_database_ipc()
        node_root = a.root / 'nodes'
        node_root.mkdir(exist_ok=True, mode=0o700)
        os.chown(node_root, identity.pw_uid, identity.pw_gid)
    names = []
    if not a.discovery_only:
        for name, text in units(a.runtime, a.binaries, a.project, a.table, a.region, a.root, a.legacy_origin).items():
            if a.nodes_only and name != 'spot-nodes':
                continue
            Path('/etc/systemd/system/'+name+'.service').write_text(text)
            names.append(name+'.service')
    if not a.nodes_only:
        if not a.discovery_only:
            entrypoint = Path('/usr/local/bin/experiment-runner')
            target = a.runtime / 'experiment-runner'
            if entrypoint.exists() or entrypoint.is_symlink():
                if not entrypoint.is_symlink() or entrypoint.resolve() != target.resolve():
                    raise SystemExit('Existing experiment-runner belongs to another installation')
            else:
                entrypoint.symlink_to(target)
        if a.table:
            import json
            from runner import atomic_json
            config = a.root / 'runner-config.json'
            settings = {'schema':'experiment.worker-config/v1', 'region':a.region, 'activity_table':a.table}
            if a.stack:
                outputs = json.loads(subprocess.check_output(['aws','--region',a.region,'cloudformation',
                    'describe-stacks','--stack-name',a.stack,'--query','Stacks[0].Outputs','--output','json'],text=True))
                settings['lifecycle_function'] = next(o['OutputValue'] for o in outputs if o['OutputKey']=='LifecycleFunctionArn')
            atomic_json(config, settings)
            os.chown(config, identity.pw_uid, identity.pw_gid)
        discovery_root = a.root / 'discovery'
        discovery_root.mkdir(exist_ok=True, mode=0o700)
        os.chown(discovery_root, identity.pw_uid, identity.pw_gid)
        for name, text in discovery_units(a.runtime, a.search_root, a.root).items():
            Path('/etc/systemd/system/'+name).write_text(text)
        names.append('spot-checkpoint-discovery.timer')
    subprocess.run(['systemctl', 'daemon-reload'], check=True)
    subprocess.run(['systemctl', 'enable', '--now', *names], check=True)
