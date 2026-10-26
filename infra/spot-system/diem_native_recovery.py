"""Fenced continuation of an explicitly enrolled native Diem generation.

Completed instance receipts live on EFS. An interrupted unfinished instance is
recomputed by the pinned producer; original deadlines survive replacements.
"""
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import threading
import time

from criu_local import atomic_json, digest
from legacy_recovery import acquire, api, instance_id


def receipts(config):
    root = Path(config['results'])
    records = root / 'instance-checkpoints-v1'
    manifest = records / 'manifest.json'
    if digest(manifest) != config['manifest_sha256']:
        raise ValueError('Diem checkpoint manifest changed')
    bound = json.loads(manifest.read_text())
    tasks = {tuple(task) for task in bound['tasks']}
    if len(tasks) != config['expected_records']:
        raise ValueError('Diem task grid differs')
    expected = {'k%02d-i%02d.json' % task: task for task in tasks}
    payloads = {}
    for path in records.glob('*.json'):
        if path.name == 'manifest.json':
            continue
        if path.name not in expected:
            raise ValueError('Unexpected Diem checkpoint')
        row = json.loads(path.read_text())
        payload = row['payload']
        value = json.loads(payload)
        if (row['schema'] != 1 or row['source_sha256'] != bound['source_sha256']
                or hashlib.sha256(payload.encode()).hexdigest() != row['payload_sha256']
                or (value['k'], value['idx']) != expected[path.name]
                or '\n' in payload or '\r' in payload):
            raise ValueError('Diem checkpoint identity or checksum differs')
        payloads[expected[path.name]] = payload
    complete = False
    final = root / 'instances.jsonl'
    if final.exists():
        body = ''.join(payloads[t] + '\n' for t in sorted(tasks))
        if len(payloads) != len(tasks) or final.read_text() != body:
            raise ValueError('Diem final list differs from committed receipts')
        frozen = root / 'FROZEN.sha256'
        complete = frozen.is_file() and frozen.read_text().split()[0] == digest(final)
    return len(payloads), complete


def run(args, config_path):
    config = json.loads(config_path.read_text())
    if (config.get('schema') != 'spot.diem-native/v1' or config.get('enabled') is not True
            or config['origin_instance_id'] != args.origin
            or config.get('workspace_verified') is not True):
        raise ValueError('Native Diem enrollment is not verified')
    if not os.path.ismount('/home/ec2-user'):
        raise ValueError('Durable home is not mounted')
    owner = instance_id()
    db = lambda operation, **kw: api('dynamodb', operation, **kw)
    ec2 = lambda **kw: api('ec2', 'describe-instances', **kw)
    stop = threading.Event()
    for sig in (signal.SIGTERM, signal.SIGINT):
        signal.signal(sig, lambda *_: stop.set())
    def report(state, pending=True, **extra):
        args.status.parent.mkdir(parents=True, exist_ok=True)
        atomic_json(args.status, dict(state=state, pending=pending,
                    observed=int(time.time()), native=True, **extra))
        args.status.chmod(0o644)
    while not stop.is_set():
        if (acquire(db, ec2, args.table, args.host_owner_key, args.origin, owner)
                and acquire(db, ec2, args.table, args.owner_key, args.origin, owner)):
            break
        report('waiting-for-source')
        stop.wait(10)
    if stop.is_set():
        return
    process = None
    try:
        for path, expected in config['files_sha256'].items():
            if digest(Path(path)) != expected:
                raise ValueError('Pinned Diem input changed: ' + path)
        count, complete = receipts(config)
        binary = config['binary']
        if binary not in config['files_sha256']:
            raise ValueError('Diem executable is not pinned')
        env = dict(os.environ, HOME='/home/ec2-user', RAYON_NUM_THREADS='1')
        launch = ['runuser', '-u', 'ec2-user', '--', 'taskset', '-c', '26,27,28',
                  'nice', '-n', '10', binary]
        subprocess.run(launch + ['gen-status'], cwd=config['cwd'], env=env,
                       check=True, timeout=60)
        logpath = Path(config['results']).parent / ('spot-generation-' + owner + '.log')
        with logpath.open('ab', buffering=0) as log:
            if not complete and time.time() < config['deadline_utc']:
                process = subprocess.Popen(launch + ['gen'], cwd=config['cwd'], env=env,
                    stdin=subprocess.DEVNULL, stdout=log, stderr=log, start_new_session=True)
            while not stop.is_set():
                count, complete = receipts(config)
                alive = process is not None and process.poll() is None
                expired = time.time() >= config['deadline_utc']
                if not alive and not complete and not expired:
                    raise RuntimeError('Native Diem exited without a complete frozen list')
                terminal = complete or expired
                report('complete' if complete else ('deadline-expired' if expired else 'running'),
                       pending=not terminal, durable_records=count,
                       expected_records=config['expected_records'],
                       launcher_pid=process.pid if alive else None,
                       deadline_utc=config['deadline_utc'])
                db('update-item', table_name=args.table, key={'Id':{'S':args.owner_key}},
                   update_expression='SET Ready = :yes, Complete = :complete, Failed = :failed',
                   condition_expression='#owner = :owner',
                   expression_attribute_names={'#owner':'Owner'},
                   expression_attribute_values={':yes':{'BOOL':True}, ':complete':{'BOOL':terminal},
                       ':failed':{'BOOL':expired and not complete}, ':owner':{'S':owner}})
                api('ec2', 'create-tags', resources=[owner], tags=[{'Key':'LegacyReady','Value':'true'}])
                if terminal:
                    return
                stop.wait(20)
    except Exception as exc:
        report('error', error=str(exc)[:200])
        raise
    finally:
        if process is not None and process.poll() is None:
            os.killpg(process.pid, signal.SIGTERM)
            try:
                process.wait(timeout=30)
            except subprocess.TimeoutExpired:
                os.killpg(process.pid, signal.SIGKILL)
                process.wait()
