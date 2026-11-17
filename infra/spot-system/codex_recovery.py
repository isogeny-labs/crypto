"""Resume explicitly enrolled Codex chats on a single fenced Spot owner.

The deployment is opt-in through an operator-owned migration manifest. Original
source release and verified snapshots are prerequisites. No approval requests
are automatically answered, and idle/errored chats are never silently started.
"""
import argparse
import json
import os
from pathlib import Path
import signal
import socket as sockets
import sqlite3
import subprocess
import threading
import time
import urllib.request

from codex_rpc import Client
from codex_snapshot import load_generation, restore, snapshot
from criu_local import atomic_json
from legacy_recovery import acquire, api, instance_id


def resumable(thread):
    source = thread.get('source')
    return (isinstance(source, str) and thread.get('status', {}).get('type') == 'active'
            and not thread['status'].get('activeFlags'))


def interrupted_turn(client, thread):
    latest = client.request('thread/turns/list',
                            {'threadId': thread['id'], 'limit': 1, 'sortDirection': 'desc'})
    turns = latest.get('data', [])
    return bool(turns and turns[0].get('status') in {'interrupted', 'inProgress'})


def start_app_server(socket, command, environment):
    # The desktop may reconnect and launch the same local app server first.
    # Adopt that authenticated local connection instead of creating a competing
    # server or terminating the user's active connection on supervisor exit.
    try:
        client = Client(socket)
    except (OSError, EOFError):
        return subprocess.Popen(command, env=environment, cwd='/home/ec2-user'), None
    try:
        client.initialize()
    except Exception:
        client.close()
        raise
    return None, client


def persisted_parents(local, threads, manifest, unavailable=None, archived=None):
    with sqlite3.connect('file:' + str(local / 'state_5.sqlite') + '?mode=ro', uri=True) as db:
        persisted = dict(db.execute('SELECT id, archived FROM threads'))
    parents, empty = [], []
    for thread in threads:
        if not isinstance(thread.get('source'), str):
            continue
        if persisted.get(thread['id']):
            if archived is not None:
                archived.append(thread['id'])
            continue
        if thread['id'] not in persisted:
            has_rollout = any(thread['id'] in name for name in manifest['files'])
            if thread.get('status', {}).get('type') == 'idle' and not has_rollout:
                empty.append(thread['id'])
                continue
            if unavailable is not None:
                # Preserve unresolved enrollment for inspection without killing
                # the app server and every other recoverable chat.
                unavailable.append(thread)
                continue
            raise ValueError('Enrolled work has no persisted thread: ' + thread['id'])
        parents.append(thread)
    return parents, empty


def dependency_ready(status, now):
    # A preserved research deadline is terminal, not an agent startup failure.
    # The agent must be able to reconcile and report the incomplete workload.
    return (status.get('state') in {'running', 'complete', 'deadline-expired'}
            and 0 <= now - status.get('observed', 0) < 90)


def notify_ready():
    """Release dependent experiment services only after the home mount is ready."""
    address = os.environ.get('NOTIFY_SOCKET')
    if address:
        if address.startswith('@'):
            address = '\0' + address[1:]
        with sockets.socket(sockets.AF_UNIX, sockets.SOCK_DGRAM) as sock:
            sock.connect(address)
            sock.sendall(b'READY=1')


def public_ip():
    opener = urllib.request.build_opener(urllib.request.ProxyHandler({}))
    base = 'http://169.254.169.254/latest/'
    request = urllib.request.Request(base+'api/token', method='PUT',
        headers={'X-aws-ec2-metadata-token-ttl-seconds':'60'})
    token = opener.open(request, timeout=3).read().decode()
    request = urllib.request.Request(base+'meta-data/public-ipv4',
        headers={'X-aws-ec2-metadata-token':token})
    return opener.open(request, timeout=3).read().decode().strip()


def prepare_home(durable, local, home=Path('/home/ec2-user')):
    """Keep SQLite and sockets local, while workspaces survive worker loss."""
    workspace = durable / 'home'
    if not workspace.is_dir() or workspace.is_symlink():
        raise ValueError('Durable workspace is missing')
    if not os.path.ismount(home):
        # Preserve only the public access key installed by EC2's launch keypair.
        # No source SSH private keys are migrated.
        authorized = home/'.ssh/authorized_keys'
        if authorized.is_file() and not (workspace/'.ssh/authorized_keys').exists():
            import shutil
            directory = workspace/'.ssh'
            directory.mkdir(mode=0o700, exist_ok=True)
            shutil.copyfile(authorized, directory/'authorized_keys')
            (directory/'authorized_keys').chmod(0o600)
            os.chown(directory,1000,1000)
            os.chown(directory/'authorized_keys',1000,1000)
        subprocess.run(['mount', '--bind', str(workspace), str(home)], check=True)
    # NFS can report a bind's SOURCE as server:/subdirectory with FSROOT=/.
    # Compare the actual filesystem/inode identity instead of display strings.
    if not os.path.ismount(home) or not os.path.samefile(home, workspace):
        raise ValueError('User home is not the enrolled durable workspace')
    codex_home = home / '.codex'
    if codex_home.is_symlink():
        if codex_home.resolve() != local.resolve():
            raise ValueError('Existing Codex home belongs to another deployment')
    elif codex_home.exists():
        raise ValueError('Refuse to replace existing Codex home')
    else:
        codex_home.symlink_to(local)


def main(args):
    config = json.loads(args.config.read_text())
    if config.get('schema') != 'spot.codex-migration/v1' or config.get('enabled') is not True:
        raise ValueError('Codex migration is not enabled')
    if config.get('authentication_transfer_authorized') is not True:
        raise ValueError('Authentication migration has not been authorized')
    if config.get('resume_active_threads_authorized') is not True:
        raise ValueError('Active task continuation has not been authorized')
    if config.get('workspace_verified') is not True:
        raise ValueError('Final quiesced workspace migration has not been verified')
    source, table = config['origin_instance_id'], config['table']
    owner = instance_id()
    durable = args.config.parent
    root = durable / 'codex-snapshots'
    local = Path('/var/lib/spot-codex/home')
    status = Path('/run/spot-legacy/codex.json')
    status.parent.mkdir(parents=True, exist_ok=True)
    stop = threading.Event()
    for sig in (signal.SIGTERM, signal.SIGINT):
        signal.signal(sig, lambda *_: stop.set())
    db = lambda operation, **kw: api('dynamodb', operation, **kw)
    ec2 = lambda **kw: api('ec2', 'describe-instances', **kw)
    def report(state, pending=True, **extra):
        atomic_json(status, {'state': state, 'pending': pending, 'observed': int(time.time()), **extra})
        status.chmod(0o644)
    while not stop.is_set():
        if (acquire(db, ec2, table, 'legacy-owner-host', source, owner)
                and acquire(db, ec2, table, 'legacy-owner-codex', source, owner)):
            break
        report('waiting-for-source')
        stop.wait(10)
    if stop.is_set():
        return
    _, manifest = load_generation(root)
    if not manifest.get('auth_included'):
        raise ValueError('Committed snapshot has no authorized authentication state')
    local.parent.mkdir(mode=0o755, parents=True, exist_ok=True)
    local.parent.chmod(0o755)
    if not local.exists():
        threads = restore(root, local)
        subprocess.run(['chown', '-R', 'ec2-user:ec2-user', str(local)], check=True)
    else:
        # A same-host service restart uses the latest local state. The initial
        # thread inventory remains the authoritative interrupted-work list.
        threads = manifest['threads']
    prepare_home(durable, local)
    binary = local / config['codex_binary_relative']
    if (not binary.resolve().is_relative_to(local.resolve()) or not binary.is_file()
            or not os.access(binary, os.X_OK)):
        raise ValueError('Enrolled Codex binary is unavailable')
    environment = os.environ.copy()
    environment.update(HOME='/home/ec2-user', CODEX_HOME='/home/ec2-user/.codex')
    socket = local/'app-server-control/app-server-control.sock'
    process, client = start_app_server(socket,
        ['runuser', '-u', 'ec2-user', '--', str(binary),
         *config.get('codex_arguments', []), 'app-server', '--listen', 'unix://'], environment)
    try:
        if client is None:
            for _ in range(60):
                if process.poll() is not None:
                    raise RuntimeError('Codex app-server exited during startup')
                try:
                    client = Client(socket)
                    client.initialize()
                    break
                except (OSError, EOFError):
                    stop.wait(1)
            if client is None:
                raise TimeoutError('Codex app-server did not become ready')
        # Dependent experiment recovery needs the home mount before it can
        # start. Release that ordering barrier before continuing agent turns.
        notify_ready()
        for required in config.get('wait_for_recovery_status', []):
            while not stop.is_set():
                try:
                    ready = json.loads(Path(required).read_text())
                    if dependency_ready(ready, time.time()):
                        break
                except (OSError, ValueError):
                    pass
                stop.wait(2)
            if stop.is_set():
                return
        if config.get('wait_for_public_ip'):
            # The research API can restrict access to the stable Elastic IP.
            # Signal server readiness so the fenced controller can move it,
            # then wait for attachment before continuing any agent work.
            api('dynamodb', 'update-item', table_name=table, key={'Id':{'S':'legacy-owner-codex'}},
                update_expression='SET Ready = :yes', condition_expression='#owner = :owner',
                expression_attribute_names={'#owner':'Owner'},
                expression_attribute_values={':yes':{'BOOL':True}, ':owner':{'S':owner}})
            while not stop.is_set():
                try:
                    if public_ip() == config['wait_for_public_ip']:
                        break
                except OSError:
                    pass
                report('waiting-for-endpoint')
                stop.wait(5)
            if stop.is_set():
                return
        unavailable, archived = [], []
        parents, omitted_empty = persisted_parents(local, threads, manifest, unavailable, archived)
        if unavailable:
            atomic_json(durable / ('codex-unavailable-' + owner + '.json'),
                        {'owner': owner, 'observed': int(time.time()), 'threads': unavailable})
        resumed = []
        for thread in parents:
            # Spawned children belong to their original parent; starting them
            # independently could duplicate tools when the parent reconciles.
            if not isinstance(thread.get('source'), str):
                continue
            # Histories can exceed the RPC frame limit. Resume the original
            # persisted chat while requesting only metadata in the response.
            current = client.request('thread/resume', {'threadId': thread['id'],
                                                       'excludeTurns': True})['thread']
            resumed.append(thread['id'])
            if resumable(thread) and current.get('status', {}).get('type') != 'active':
                # Retained inventory can outlive a completed/failed turn. Only
                # continue work whose persisted turn was actually interrupted.
                if not interrupted_turn(client, thread):
                    continue
                client.request('turn/start', {'threadId': thread['id'], 'input': [{'type':'text',
                    'text':'Continue the existing user-authorized task after this Spot host migration. '
                           'First reconcile saved artifacts and any external jobs or side effects '
                           'before repeating interrupted work. Preserve the original scope, budgets, '
                           'approval rules, and completion criteria. If it already completed, '
                           'report that evidence without starting new work. '
                           + config.get('migration_context', '')}]})
        api('dynamodb', 'update-item', table_name=table, key={'Id':{'S':'legacy-owner-codex'}},
            update_expression='SET Ready = :yes', condition_expression='#owner = :owner',
            expression_attribute_names={'#owner':'Owner'},
            expression_attribute_values={':yes':{'BOOL':True}, ':owner':{'S':owner}})
        notify_ready()
        while not stop.is_set():
            if process is not None and process.poll() is not None:
                raise RuntimeError('Codex app-server exited')
            active = client.inventory()
            saved = snapshot(local, root, active, include_auth=True)
            report('running', pending=any(t.get('status',{}).get('type') == 'active' for t in active),
                   loaded=len(active), resumed=len(resumed), omitted_empty=omitted_empty, server_pid=process.pid if process is not None else None,
                   unavailable_threads=[t['id'] for t in unavailable],
                   archived_threads=archived,
                   snapshot_prune_errors=saved['prune_errors'])
            stop.wait(config.get('snapshot_interval_seconds', 60))
        # Retain a final committed generation before the OS stops the process.
        snapshot(local, root, client.inventory(), include_auth=True)
    finally:
        if client:
            client.close()
        if process is not None and process.poll() is None:
            process.terminate()
            try:
                process.wait(timeout=30)
            except subprocess.TimeoutExpired:
                process.kill()


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--config', type=Path, required=True)
    args = parser.parse_args()
    os.umask(0o077)
    if os.geteuid() != 0:
        parser.error('Root is required for workload ownership and local state restore')
    main(args)
