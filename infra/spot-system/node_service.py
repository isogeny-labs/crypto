#!/usr/bin/env python3
"""Single writer for durable node state, with graceful database shutdown."""
import argparse
import fcntl
import os
from pathlib import Path
import signal
import subprocess
import threading
import time


def run(root, binary_dir, project):
    root.mkdir(parents=True, exist_ok=True)
    root.chmod(0o700)
    stop = threading.Event()
    for sig in [signal.SIGTERM, signal.SIGINT]:
        signal.signal(sig, lambda *_: stop.set())
    children = []
    database_started = False
    conductor_state = root / 'conductor'
    conductor_state.mkdir(exist_ok=True, mode=0o700)
    if (Path.home()/'.conductor/secret.key').exists() and not (conductor_state/'secret.key').exists():
        raise RuntimeError('Existing Conductor encryption key needs an explicit migration before changing state paths')
    env = dict(os.environ, PATH=str(binary_dir) + ':/usr/local/bin:/usr/bin:/bin',
               DATABASE_URL='postgresql:///conductor?host=/run/spot-postgresql',
               CONDUCTOR_STATE_DIR=str(conductor_state))
    with (root / '.writer.lock').open('a+') as lock:
        while not stop.is_set():
            try:
                fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
                break
            except BlockingIOError:
                stop.wait(2)
        if stop.is_set():
            return
        try:
            data = root / 'postgres'
            if not (data / 'PG_VERSION').exists():
                subprocess.run(['initdb', '-D', str(data), '--auth-local=peer', '--auth-host=reject'],
                               env=env, check=True, stdout=subprocess.DEVNULL)
            subprocess.run(['pg_ctl', '-D', str(data), '-l', str(root / 'postgres.log'),
                            '-o', "-k /run/spot-postgresql -h ''", '-w', 'start'], env=env, check=True)
            database_started = True
            exists = subprocess.check_output(['psql', 'postgresql:///postgres?host=/run/spot-postgresql',
                '-At', '-v', 'ON_ERROR_STOP=1', '-c', "SELECT 1 FROM pg_database WHERE datname='conductor'"],
                env=env, text=True).strip()
            if exists != '1':
                subprocess.run(['createdb', '-h', '/run/spot-postgresql', 'conductor'], env=env, check=True)
            # Bootstrap stores local credentials itself. Never print its token output.
            with (root / 'bootstrap.log').open('ab') as out:
                subprocess.run([str(binary_dir / 'conductord'), 'bootstrap', '--org', 'spot',
                    '--project', 'experiments', '--principal', 'ec2-user', '--repo', str(project),
                    '--endpoint', 'http://127.0.0.1:8080'], env=env, check=True, stdout=out, stderr=out)
            for queue in ['queue', 'running', 'done']:
                (root / 'agent/jobs' / queue).mkdir(parents=True, exist_ok=True)
            cairn_env = dict(env, CAIRN_DATA=str(root / 'cairn'), CAIRN_KEY=str(root / 'cairn.key'),
                CAIRN_NETWORK='private', CAIRN_SEEDS='off', CAIRN_PORTMAP='off',
                CAIRN_BEACON_PORT='off', CAIRN_ROLES='relay', CAIRN_MCP_MAX_SPEND='0')
            commands = [([str(binary_dir / 'conductord'), '--addr', '127.0.0.1:8080', '--database', 'local'], env),
                        ([str(binary_dir / 'cairn'), '--root', str(project), 'run', '--no-mcp',
                          '--listen', '127.0.0.1:9100', '--serve', '127.0.0.1:9180'], cairn_env)]
            for command, child_env in commands:
                children.append(subprocess.Popen(command, env=child_env, cwd=root, start_new_session=True))
            while not stop.wait(1):
                if any(child.poll() is not None for child in children):
                    raise RuntimeError('Node process exited; restarting the node bundle')
        finally:
            for child in children:
                if child.poll() is None:
                    os.killpg(child.pid, signal.SIGTERM)
            for child in children:
                try:
                    child.wait(timeout=20)
                except subprocess.TimeoutExpired:
                    os.killpg(child.pid, signal.SIGKILL)
                    child.wait()
            if database_started:
                subprocess.run(['pg_ctl', '-D', str(root / 'postgres'), '-m', 'fast', '-w', 'stop'],
                               env=env, check=True, timeout=30)


if __name__ == '__main__':
    p = argparse.ArgumentParser()
    p.add_argument('--root', type=Path, default=Path('/srv/experiments/nodes'))
    p.add_argument('--binary-dir', type=Path, default=Path('/opt/spot/bin'))
    p.add_argument('--project', type=Path, required=True)
    a = p.parse_args()
    run(a.root, a.binary_dir, a.project)
