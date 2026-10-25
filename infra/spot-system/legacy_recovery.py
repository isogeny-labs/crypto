#!/usr/bin/env python3
"""Fenced recovery of the explicitly configured legacy experiment set.

Never fences by time alone: a successor needs EC2-confirmed termination or an
explicit, receipted release of the original workload. Stopped instances alone
are not a fence. All application outputs are restored into private local copies.
"""
import argparse
import base64
import ctypes
import hashlib
import json
import os
import re
from pathlib import Path
import signal
import subprocess
import tarfile
import time
import urllib.request

from criu_local import atomic_json, digest, identity, same_process, validate_generation
from criu_export import BUCKET, REGION, completion


def api(service, operation, **kwargs):
    cmd = ['aws', '--region', REGION, service, operation, '--output', 'json']
    for key, value in kwargs.items():
        if isinstance(value, bool):
            if value:
                cmd += ['--' + key.replace('_', '-')]
            continue
        cmd += ['--' + key.replace('_', '-'), value if isinstance(value, str) else json.dumps(value)]
    result = subprocess.run(cmd, capture_output=True, text=True, timeout=35)
    if result.returncode:
        if 'ConditionalCheckFailed' in result.stderr:
            raise BlockingIOError('Recovery ownership changed')
        raise RuntimeError('AWS ' + service + ' ' + operation + ' failed')
    return json.loads(result.stdout or '{}')


def instance_id():
    opener = urllib.request.build_opener(urllib.request.ProxyHandler({}))
    base = 'http://169.254.169.254/latest/'
    request = urllib.request.Request(base+'api/token',method='PUT',headers={'X-aws-ec2-metadata-token-ttl-seconds':'60'})
    token = opener.open(request,timeout=3).read().decode()
    request = urllib.request.Request(base+'meta-data/instance-id',headers={'X-aws-ec2-metadata-token':token})
    return opener.open(request,timeout=3).read().decode()


def state(ec2, instance):
    reservations = ec2(instance_ids=[instance])['Reservations']
    instances = [i for r in reservations for i in r['Instances']]
    if len(instances) != 1:
        raise RuntimeError('Owner state not established')
    return instances[0]['State']['Name']


def origin_released(record, origin):
    """Only the operator's quiesced-source handoff can replace source death."""
    return (record.get('Origin', {}).get('S') == origin
            and record.get('OriginReleased', {}).get('BOOL') is True
            and re.fullmatch(r'[0-9a-f]{64}', record.get('ReleaseReceipt', {}).get('S', '')) is not None)


def acquire(db, ec2, table, key, origin, owner):
    """A failed heartbeat or an expired timestamp never permits stealing work."""
    if owner==origin:
        return False
    current = db('get-item', table_name=table, key={'Id':{'S':key}}, consistent_read=True).get('Item', {})
    prior = current.get('Owner', {}).get('S', origin)
    released = origin_released(current, origin)
    origin_dead = current.get('OriginTerminated', {}).get('BOOL') is True
    if prior==owner and current.get('Origin',{}).get('S')==origin and (origin_dead or released):
        return True
    prior_dead = (current.get('OwnerTerminated', {}).get('BOOL') is True or
                  (prior == origin and (origin_dead or released)))
    if prior != owner and not prior_dead and state(ec2, prior) != 'terminated':
        return False
    # A node may not resurrect old source work while its source remains alive.
    if (owner != origin and not origin_dead and not released
            and state(ec2, origin) != 'terminated'):
        return False
    kwargs = {'table_name':table, 'item':{'Id':{'S':key}, 'Owner':{'S':owner},
              'Origin':{'S':origin}, 'OriginTerminated':{'BOOL':not released or origin_dead}}}
    if released:
        kwargs['item'].update(OriginReleased={'BOOL':True}, ReleaseReceipt=current['ReleaseReceipt'])
    if current:
        if current.get('Origin', {}).get('S') != origin:
            raise ValueError('Recovery namespace belongs to another origin')
        kwargs.update(condition_expression='#owner = :prior',
                      expression_attribute_names={'#owner':'Owner'},
                      expression_attribute_values={':prior':{'S':prior}})
    else:
        kwargs['condition_expression'] = 'attribute_not_exists(Id)'
    db('put-item', **kwargs)
    return True


def get_object(key, target, expected=None):
    if not key.startswith('checkpoints/criu/') and not key.startswith('checkpoints/criu-validation/'):
        raise ValueError('Unsupported checkpoint prefix')
    subprocess.run(['aws', '--region', REGION, 's3', 'cp', 's3://' + BUCKET + '/' + key,
                    str(target), '--only-show-errors'], check=True)
    if expected is not None and digest(target) != expected:
        raise ValueError('Downloaded checkpoint hash mismatch')


def extract(archive, target, max_bytes=68719476736):
    """Streaming tar reader; no links, devices, escaped paths or overwrite."""
    target.mkdir(mode=0o700)
    directories = []
    total = 0
    with subprocess.Popen(['zstd', '-dc', str(archive)], stdout=subprocess.PIPE) as process:
        with tarfile.open(fileobj=process.stdout, mode='r|') as stream:
            for entry in stream:
                rel = Path(entry.name)
                if rel.is_absolute() or '..' in rel.parts or not (entry.isdir() or entry.isfile()):
                    raise ValueError('Unsafe checkpoint archive member')
                actual = target / rel
                if not actual.resolve().is_relative_to(target.resolve()):
                    raise ValueError('Escaped checkpoint archive')
                if entry.isdir():
                    # Tar writers commonly include "." as a root-directory
                    # record. Keep the private staging directory's ownership
                    # and mode instead of applying archive metadata to it.
                    if rel == Path('.'):
                        continue
                    actual.mkdir(parents=True, exist_ok=True, mode=0o700)
                    directories.append((actual, entry))
                    continue
                total += entry.size
                if total > max_bytes or actual.exists():
                    raise ValueError('Oversized or duplicate checkpoint member')
                actual.parent.mkdir(parents=True, exist_ok=True, mode=0o700)
                with actual.open('xb') as output:
                    data = stream.extractfile(entry)
                    for chunk in iter(lambda:data.read(1048576), b''):
                        output.write(chunk)
                actual.chmod(entry.mode & 0o777)
                os.chown(actual, entry.uid, entry.gid)
        process.stdout.close()
        if process.wait() != 0:
            raise ValueError('Checkpoint decompression failed')
    for path, entry in reversed(directories):
        path.chmod(entry.mode & 0o777)
        os.chown(path, entry.uid, entry.gid)


def discover(root, pointer_key, origin):
    get_object(pointer_key, root / 'pointer.json')
    pointer = json.loads((root / 'pointer.json').read_text())
    if pointer.get('schema') != 'spot.criu-export-pointer/v1' or not 1 <= len(pointer['indexes']) <= 3:
        raise ValueError('Invalid discovery pointer')
    rejected = []
    for record in pointer['indexes']:
        try:
            get_object(record['key'], root / 'index.json', record['sha256'])
            index = json.loads((root / 'index.json').read_text())
            if (not index.get('committed') or index.get('schema') != 'spot.criu-export/v1'
                    or index['recovery']['origin_instance_id'] != origin):
                raise ValueError('Checkpoint origin/commit mismatch')
            jobs = index['recovery']['jobs']
            records = index['objects'] + index['completed']
            if len(records) != len(jobs) or {r['pid'] for r in records} != {j['pid'] for j in jobs}:
                raise ValueError('Incomplete logical experiment coverage')
            # Download and verify every required archive before accepting a batch.
            for record in index['objects']:
                directory = root / str(record['pid']) / index['batch']
                if not directory.exists():
                    directory.parent.mkdir(exist_ok=True, mode=0o700)
                    archive = directory.with_suffix('.tar.zst')
                    get_object(record['key'], archive, record['sha256'])
                    extract(archive, directory)
                manifest = validate_generation(directory)
                expected = next(j for j in jobs if j['pid'] == record['pid'])
                if (manifest['job']['command_sha256'] != expected['command_sha256'] or
                        manifest['job']['pid'] != expected['pid'] or not (directory / 'clocks.json').is_file()):
                    raise ValueError('Checkpoint identity or clock state missing')
                record['directory'] = str(directory)
            return index
        except (OSError, ValueError, KeyError, TypeError, subprocess.SubprocessError) as exc:
            rejected.append(type(exc).__name__)
    raise ValueError('No fully verified recovery batch: ' + ','.join(rejected))


def write_completed(record, contract):
    for output in record['outputs']:
        if output['path'] not in {contract['stdout'], contract['stderr'],contract.get('completion_file')}:
            raise ValueError('Unexpected completed output path')
        payload = base64.b64decode(output['data_base64'], validate=True)
        if len(payload) > contract.get('max_output_bytes',1048576) or hashlib.sha256(payload).hexdigest() != output['sha256']:
            raise ValueError('Completed output hash mismatch')
        path = Path(output['path'])
        path.parent.mkdir(parents=True, exist_ok=True)
        if path.exists() and digest(path) != output['sha256']:
            raise ValueError('Refuse to overwrite different completed output')
        if not path.exists():
            path.write_bytes(payload)
            path.chmod(0o600)
            os.chown(path, 1000, 1000)


def clock_exec(directory, command):
    clocks = json.loads((directory / 'clocks.json').read_text())
    libc = ctypes.CDLL(None, use_errno=True)
    if libc.unshare(0x80) != 0:
        raise OSError(ctypes.get_errno(), 'Cannot create restore time namespace')
    offsets = []
    for name, clock in [('monotonic',time.CLOCK_MONOTONIC), ('boottime',time.CLOCK_BOOTTIME)]:
        offset = clocks[name + '_ns'] - time.clock_gettime_ns(clock)
        seconds, nanos = divmod(offset, 1000000000)
        offsets.append('%s %s %s\n' % (name, seconds, nanos))
    Path('/proc/self/timens_offsets').write_text(''.join(offsets))
    child = os.fork()
    if child:
        _, status = os.waitpid(child, 0)
        os._exit(os.waitstatus_to_exitcode(status))
    os.execv(command[0], command)


def restore(record, contract, criu):
    directory = Path(record['directory'])
    manifest = validate_generation(directory)
    pid = record['pid']
    if Path('/proc', str(pid)).exists():
        raise ValueError('Restore PID occupied; refuse to kill an existing process')
    if os.uname().release != manifest['kernel']:
        raise ValueError('Restore kernel differs from source')
    for name in json.loads((directory / 'backing-paths.json').read_text()):
        source = directory / 'filesystem' / name.lstrip('/')
        target = Path(name)
        if name.startswith(('/usr/lib64/', '/lib64/')):
            if digest(source) != digest(target):
                raise ValueError('Restore library differs from source')
        elif name in {manifest['job']['executable'],contract['stdout'],contract['stderr']}:
            if target.exists():
                if digest(target) != digest(source):
                    raise ValueError('Refuse to overwrite different application backing file')
            else:
                import shutil
                target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(source, target)
                info = source.stat()
                os.chown(target, info.st_uid, info.st_gid)
        else:
            raise ValueError('Unexpected backing file')
    for name, info in json.loads((directory / 'backing-directories.json').read_text()).items():
        if name=='/tmp/claude-1000' or name.startswith(('/home/ec2-user/','/tmp/claude-1000/')):
            Path(name).mkdir(parents=True, exist_ok=True)
            os.chmod(name, info['mode'])
            os.chown(name, info['uid'], info['gid'])
    command = [str(criu), 'restore', '-D', str(directory / 'images'), '--shell-job',
               '--cpu-cap=all', '--restore-detached', '--pidfile', str(directory / 'restored.pid'),
               '-o', 'automatic-restore.log', '-v2']
    # CLONE_NEWTIME affects children, preserving this supervisor's host clock.
    # Saved monotonic time excludes preemption downtime; UTC launcher deadlines
    # remain enforced by the supervisor outside that namespace.
    with (directory / 'restore-command.log').open('ab') as log:
        subprocess.run(['/usr/bin/python3',str(Path(__file__).resolve()),'--clock-exec',str(directory),*command], stdout=log, stderr=log,
                       check=True, timeout=90)
    actual = identity(pid)
    if actual['command_sha256'] != manifest['job']['command_sha256'] or not same_process(actual):
        raise ValueError('Restored experiment disappeared')
    return actual


def run(args):
    native = Path('/srv/experiments/agent-migration/diem.json')
    if args.owner_key == 'legacy-owner-diem' and native.is_file():
        from diem_native_recovery import run as native_run
        return native_run(args, native)
    owner = instance_id()
    root = args.root
    root.mkdir(parents=True, exist_ok=True, mode=0o700)
    if root.is_symlink() or root.stat().st_uid != 0 or root.stat().st_mode & 0o077:
        raise ValueError('Recovery root must be private and root-owned')
    db = lambda operation, **kw:api('dynamodb', operation, **kw)
    ec2 = lambda **kw:api('ec2', 'describe-instances', **kw)
    while True:
        try:
            ownership = db('get-item', table_name=args.table, key={'Id':{'S':args.owner_key}}, consistent_read=True).get('Item', {})
            if ownership.get('Complete', {}).get('BOOL') is True:
                atomic_json(args.status, {'state':'failed' if ownership.get('Failed',{}).get('BOOL') else 'complete','pending':False,'observed':int(time.time())})
                api('ec2','create-tags', resources=[owner], tags=[{'Key':'LegacyReady','Value':'true'}])
            elif (not acquire(db,ec2,args.table,args.host_owner_key,args.origin,owner)
                  or not acquire(db, ec2, args.table, args.owner_key, args.origin, owner)):
                atomic_json(args.status, {'state':'waiting-for-source', 'pending':False, 'observed':int(time.time())})
            else:
                atomic_json(args.status, {'state':'recovering', 'pending':True, 'observed':int(time.time())})
                committed = root / 'restored.json'
                if not committed.exists():
                    index = discover(root, args.pointer_key, args.origin)
                    config = index['recovery']
                    for record in index['completed']:
                        write_completed(record, config['contracts'][str(record['pid'])])
                    registry = {j['pid']:j for j in config['jobs']}
                    failed = []
                    for record in index['objects']:
                        if time.time() >= (config['contracts'][str(record['pid'])].get('deadline_utc') or float('inf')):
                            failed.append(record['pid'])
                            continue
                        started = root / ('started-' + str(record['pid']) + '.json')
                        if started.exists():
                            previous = json.loads(started.read_text())
                            if not same_process(previous) and completion(previous, config['contracts'][str(record['pid'])]) is None:
                                raise ValueError('Partial recovery process disappeared without completion')
                            registry[record['pid']] = previous
                        else:
                            registry[record['pid']] = restore(record, config['contracts'][str(record['pid'])], args.criu)
                            atomic_json(started, registry[record['pid']])
                    config['backup_source_instance_id'] = owner
                    atomic_json(root / 'recovery.json', config)
                    atomic_json(root / 'registry.json', {'schema':'spot.criu-registry/v1','jobs':list(registry.values())})
                    atomic_json(root / 'export-authorization.json', {'memory_export_authorized':True,
                        'recurring_memory_export_authorized':True, 'bucket':BUCKET, 'jobs':list(registry.values())})
                    atomic_json(committed, {'batch':index['batch'], 'jobs':list(registry.values()), 'config':config,'failed':failed})
                    # Enable the exact same private backup loop on this owner.
                    subprocess.run(['systemctl','enable','--now',args.backup_timer], check=True)
                restored = json.loads(committed.read_text())
                active = 0
                failed = restored.get('failed',[])
                for job in restored['jobs']:
                    if job['pid'] in failed:
                        continue
                    contract = restored['config']['contracts'][str(job['pid'])]
                    if same_process(job):
                        if time.time() >= (contract.get('deadline_utc') or float('inf')):
                            os.kill(job['pid'], signal.SIGTERM)
                            failed.append(job['pid'])
                            restored['failed'] = failed
                            atomic_json(committed, restored)
                        active += 1
                    elif completion(job, contract) is None:
                        raise ValueError('Recovered experiment exited without its terminal records')
                atomic_json(args.status, {'state':'running' if active else ('failed' if failed else 'complete'),
                    'pending':bool(active), 'active':active,'failed':len(failed), 'observed':int(time.time())})
                api('ec2','create-tags', resources=[owner], tags=[{'Key':'LegacyReady','Value':'true'}])
                db('update-item',table_name=args.table,key={'Id':{'S':args.owner_key}},
                   update_expression='SET Ready = :yes',condition_expression='#owner = :owner',
                   expression_attribute_names={'#owner':'Owner'},
                   expression_attribute_values={':yes':{'BOOL':True},':owner':{'S':owner}})
                if not active:
                    if failed:
                        from criu_export import upload_verified
                        receipt = root/'terminal.json'
                        atomic_json(receipt, {'state':'launcher-deadline-expired','failed':failed,
                            'last_recoverable_batch':restored['batch'],'original_checkpoints_retained':True})
                        prefix = restored['config'].get('prefix','checkpoints/criu')
                        upload_verified(receipt,prefix+'/terminal/'+owner+'.json')
                        subprocess.run(['systemctl','disable','--now',args.backup_timer],check=True)
                    else:
                        subprocess.run(['/usr/bin/python3',str(Path(__file__).with_name('criu_export.py')),
                            '--root',str(root),'--staging',str(root)+'-export',
                            '--authorization',str(root/'export-authorization.json')], check=True)
                    db('update-item', table_name=args.table, key={'Id':{'S':args.owner_key}},
                       update_expression='SET Complete = :yes, Failed = :failed', condition_expression='#owner = :owner',
                       expression_attribute_names={'#owner':'Owner'},
                       expression_attribute_values={':yes':{'BOOL':True},':failed':{'BOOL':bool(failed)},':owner':{'S':owner}})
            args.status.chmod(0o644)
        except (OSError, ValueError, KeyError, RuntimeError, subprocess.SubprocessError) as exc:
            atomic_json(args.status, {'state':'error','pending':True,'observed':int(time.time()),'error':str(exc)[:200]})
            args.status.chmod(0o644)
            print('Legacy recovery failed: ' + type(exc).__name__, flush=True)
        if args.once:
            return
        time.sleep(20)


if __name__ == '__main__':
    import sys
    if len(sys.argv) > 2 and sys.argv[1] == '--clock-exec':
        clock_exec(Path(sys.argv[2]), sys.argv[3:])
        raise SystemExit(0)
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--root', type=Path, default=Path('/var/lib/spot-criu-managed'))
    p.add_argument('--status', type=Path, required=True)
    p.add_argument('--origin', required=True)
    p.add_argument('--table', required=True)
    p.add_argument('--owner-key', default='legacy-owner')
    p.add_argument('--host-owner-key')
    p.add_argument('--backup-timer',default='spot-legacy-backup.timer')
    p.add_argument('--pointer-key', default='checkpoints/criu/latest.json')
    p.add_argument('--criu', type=Path, default=Path('/opt/spot-criu/criu'))
    p.add_argument('--once', action='store_true')
    a = p.parse_args()
    if not a.host_owner_key:
        a.host_owner_key='legacy-owner-test-host-'+a.origin if a.owner_key.startswith('legacy-owner-test-') else 'legacy-owner-host'
    if os.geteuid() != 0:
        p.error('Root is required for private process recovery')
    os.umask(0o077)
    run(a)
