#!/usr/bin/env python3
"""One durable experiment supervisor per shared workspace; no third-party deps.

Jobs MUST implement the checkpoint contract. This supervisor cannot snapshot RAM.
An NFSv4 flock fences overlapping replacement instances. A failure to renew an NFS
lease must not be interpreted as proof that an old worker is dead; jobs that write
external side effects need application-level fencing/idempotency as well.
"""
import argparse
import fcntl
import hashlib
import json
import os
from pathlib import Path
import signal
import socket
import subprocess
import sys
import tempfile
import threading
import time
import urllib.error
import urllib.request
import uuid

RUN_SCHEMA = 'experiment.run/v1'
RESERVED_ENV = {'SPOT_CHECKPOINT_DIR', 'SPOT_CHECKPOINT_MANIFEST', 'SPOT_RESUMED', 'SPOT_OUTPUT_DIR'}


def atomic_json(path, value):
    # Unique, private temporary files also allow readers/admission requests to
    # coexist with the supervisor's status updates.
    with tempfile.NamedTemporaryFile(mode='w', dir=path.parent, delete=False) as f:
        temp = Path(f.name)
        try:
            json.dump(value, f, sort_keys=True)
            f.flush()
            os.fsync(f.fileno())
            os.replace(temp, path)
        finally:
            temp.unlink(missing_ok=True)
    fd = os.open(path.parent, os.O_DIRECTORY)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)


def fingerprint(job):
    return hashlib.sha256(json.dumps(job, sort_keys=True).encode()).hexdigest()


def validate_jobs(root, data):
    if not isinstance(data, list):
        raise ValueError("jobs.json must contain a list")
    seen = set()
    for j in data:
        if not isinstance(j, dict) or not isinstance(j.get('cwd'), str):
            raise ValueError('Job must be an object with a cwd string')
        name = j["id"]
        if not isinstance(name, str) or not name or any(c not in
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-" for c in name):
            raise ValueError("Job id must contain only letters, digits, _ and -")
        if name in seen:
            raise ValueError("Duplicate job id")
        seen.add(name)
        if j.get("checkpoint_contract") not in {"atomic-files-and-sigterm-v1", "manifest-sha256-v1"}:
            raise ValueError(f"{name}: explicit checkpoint contract required")
        if not isinstance(j.get("argv"), list) or not j["argv"] or not all(
                isinstance(a, str) and '\0' not in a for a in j["argv"]):
            raise ValueError("argv must be a nonempty string list")
        env = j.get('env', {})
        if (not isinstance(env, dict) or any(not isinstance(k, str) or not k or
                k[0] not in 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_' or
                any(c not in 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_' for c in k) or
                k in RESERVED_ENV or not isinstance(v, str) or '\0' in v for k, v in env.items())):
            raise ValueError('env must contain explicit string settings, without reserved checkpoint variables')
        cwd = (root / j["cwd"]).resolve()
        if not cwd.is_relative_to(root.resolve()) or not cwd.is_dir():
            raise ValueError("cwd must be an existing directory within the durable workspace")
    return data


def read_jobs(root):
    return validate_jobs(root, json.loads((root / 'jobs.json').read_text()))


def run_paths(job):
    state = 'state/' + job['id']
    return {name: state + '/' + leaf for name, leaf in
            [('record', 'run.json'), ('checkpoint', 'checkpoint'), ('output', 'output'),
             ('stdout', 'stdout.log'), ('stderr', 'stderr.log')]}


def run_record(root, job, phase, selected=None, **status):
    paths = run_paths(job)
    checkpoint = {'contract': job['checkpoint_contract'], 'selection': 'latest-at-resume',
                  'validation': 'sha256-manifest' if job['checkpoint_contract'] == 'manifest-sha256-v1'
                  else 'application-managed', 'last_selected': None}
    if selected:
        checkpoint['last_selected'] = {'manifest': str(Path(selected['manifest']).relative_to(root)),
                                       'generation': selected['generation']}
    return {'schema': RUN_SCHEMA, 'id': job['id'], 'spec_sha256': fingerprint(job), 'job': job,
            'workspace': str(root), 'paths': paths, 'checkpoint': checkpoint,
            'status': {'phase': phase, 'observed_unix': time.time(), **status},
            'resume': {'policy': 'resume-only', 'argv': ['experiment-runner',
                '--root', str(root), '--resume', paths['record']]}}


def publish_record(root, job, phase, selected=None, required=False, **status):
    record = run_record(root, job, phase, selected, **status)
    try:
        atomic_json(root / record['paths']['record'], record)
    except OSError:
        if required:
            raise
        # Observational metadata cannot orphan a running child by killing its
        # supervisor. Checkpoint/completion commits retain their strict checks.
        print('Run record update failed for ' + job['id'], file=sys.stderr, flush=True)
    return record


def read_run_record(root, path):
    record = json.loads(path.read_text())
    if record.get('schema') != RUN_SCHEMA:
        raise ValueError('Unsupported experiment run schema')
    job = validate_jobs(root, [record['job']])[0]
    if (record.get('id') != job['id'] or record.get('spec_sha256') != fingerprint(job)
            or record.get('paths') != run_paths(job)):
        raise ValueError('Run record identity or paths differ from its specification')
    state = root / 'state' / job['id']
    identity = state / 'spec.json'
    if not state.resolve().is_relative_to(root) or identity.is_symlink():
        raise ValueError('Run identity must stay within the workspace')
    saved = json.loads(identity.read_text())
    if saved.get('sha256') != fingerprint(job) or saved.get('spec') != job:
        raise ValueError('Run record differs from durable spec.json; a new job id is required')
    return job


def completed(root, job):
    path = root / 'state' / job['id'] / 'complete.json'
    if not path.exists():
        return False
    if json.loads(path.read_text()).get('sha256') != fingerprint(job):
        raise RuntimeError(job['id'] + ': completion marker does not match the job')
    return True


def select_checkpoint(root, job, resume_only=False):
    checkpoint = root / 'state' / job['id'] / 'checkpoint'
    selected = None
    if job['checkpoint_contract'] == 'manifest-sha256-v1':
        binary = os.environ.get('SPOT_CHECKPOINT_DISCOVERY_BIN',
                                str(Path(__file__).with_name('bin') / 'checkpoint-discovery'))
        discovery = subprocess.run([binary, '--root', str(root), '--job', job['id'],
                                    '--require-valid', '--max-hash-bytes', str(64 * 1024**3)],
                                   capture_output=True, text=True, timeout=1800)
        if discovery.returncode:
            raise RuntimeError(job['id'] + ': checkpoint discovery rejected recovery; inspect the inventory')
        selected = json.loads(discovery.stdout)['jobs'][0]['selected']
        if selected and not Path(selected['manifest']).resolve().is_relative_to(checkpoint.resolve()):
            raise RuntimeError('Selected checkpoint is outside this run')
        if resume_only and not selected:
            raise RuntimeError(job['id'] + ': resume requires a valid checkpoint; fresh start refused')
    elif resume_only and (not checkpoint.is_dir() or not any(checkpoint.iterdir())):
        raise RuntimeError(job['id'] + ': resume requires application checkpoint files; fresh start refused')
    return selected


def submit_resume(root, path):
    job = read_run_record(root, path)
    if completed(root, job):
        if (root / 'jobs.json').exists():
            current = next((j for j in read_jobs(root) if j['id'] == job['id']), None)
            if current is not None and current != job:
                raise ValueError('Current jobs.json differs from the completed run record')
        publish_record(root, job, 'complete')
        return job
    managed_wake = os.environ.get('SPOT_LIFECYCLE_FUNCTION')
    if os.environ.get('SPOT_ACTIVITY_TABLE') and not managed_wake:
        from worker_state import admit
        admit(root)
    # This admission lock is independent of the long-lived supervisor lock.
    # A client can submit work while the system service is supervising another
    # job; the manifest is replaced atomically and existing entries are preserved.
    with (root / '.jobs.lock').open('a+') as lock:
        fcntl.flock(lock, fcntl.LOCK_EX)
        jobs = read_jobs(root) if (root / 'jobs.json').exists() else []
        existing = next((j for j in jobs if j['id'] == job['id']), None)
        if existing is not None and existing != job:
            raise ValueError('Current jobs.json differs from the run record')
        # Persist strict intent BEFORE publishing a newly admitted job.
        atomic_json(root / 'state' / job['id'] / 'resume-request.json',
                    {'schema': 'experiment.resume-request/v1', 'spec_sha256': fingerprint(job),
                     'requested_unix': time.time()})
        if existing is None:
            atomic_json(root / 'jobs.json', [*jobs, job])
    select_checkpoint(root, job, resume_only=True)
    if managed_wake:
        from wake import invoke
        response = invoke({'function': managed_wake, 'region': os.environ['AWS_REGION']},
                          'wake', managed_work=True)
        if response.get('state') != 'starting':
            raise RuntimeError('Managed worker wake was not accepted')
    return job


def configure_worker(root):
    # Nonsecret infrastructure settings live with the workspace, so an SSH/Codex
    # caller does not need to inherit the original launcher's environment.
    path = root / 'runner-config.json'
    if path.exists():
        config = json.loads(path.read_text())
        if config.get('schema') != 'experiment.worker-config/v1':
            raise ValueError('Unsupported runner configuration')
        for key, name in [('AWS_REGION', 'region'), ('SPOT_ACTIVITY_TABLE', 'activity_table')]:
            value = config[name]
            if not isinstance(value, str) or not value:
                raise ValueError('Invalid worker configuration')
            os.environ[key] = value
        if config.get('lifecycle_function'):
            os.environ['SPOT_LIFECYCLE_FUNCTION'] = config['lifecycle_function']
        else:
            os.environ.pop('SPOT_LIFECYCLE_FUNCTION', None)


def list_runs(root, export=False):
    records = []
    jobs = {j['id']: j for j in read_jobs(root)} if (root / 'jobs.json').exists() else {}
    state_root = root / 'state'
    if state_root.is_dir():
        for state in sorted(state_root.iterdir()):
            identity = state / 'spec.json'
            if state.is_symlink() or not state.is_dir() or not identity.exists():
                continue
            if identity.is_symlink():
                raise ValueError('Run identity must not be a symlink')
            saved = json.loads(identity.read_text())
            job = validate_jobs(root, [saved['spec']])[0]
            if state.name != job['id'] or saved.get('sha256') != fingerprint(job):
                raise ValueError('Stored run identity does not match its state directory')
            if job['id'] in jobs and jobs[job['id']] != job:
                raise ValueError('Current manifest differs from a stored run')
            jobs[job['id']] = job
    for job in jobs.values():
        path = root / run_paths(job)['record']
        if path.exists():
            read_run_record(root, path)
            records.append(json.loads(path.read_text()))
        else:
            phase = 'complete' if completed(root, job) else 'unknown'
            record = run_record(root, job, phase)
            if export:
                # Backfill only runs that already have a durable identity. This
                # never creates spec/attempt state or launches a fresh workload.
                identity = root / 'state' / job['id'] / 'spec.json'
                if identity.exists():
                    saved = json.loads(identity.read_text())
                    if saved != {'sha256': fingerprint(job), 'spec': job}:
                        raise ValueError('Cannot export a mismatched durable run')
                    atomic_json(path, record)
            records.append(record)
    return {'schema': 'experiment.run-index/v1', 'workspace': str(root), 'runs': records}


def interrupted():
    """IMDSv2 direct local access, independent of HTTP proxy environment."""
    opener = urllib.request.build_opener(urllib.request.ProxyHandler({}))
    base = "http://169.254.169.254/latest/"
    request = urllib.request.Request(base + "api/token", method="PUT",
                                    headers={"X-aws-ec2-metadata-token-ttl-seconds": "60"})
    token = opener.open(request, timeout=2).read().decode()
    request = urllib.request.Request(base + "meta-data/spot/instance-action",
                                    headers={"X-aws-ec2-metadata-token": token})
    try:
        with opener.open(request, timeout=2) as response:
            return response.status == 200
    except urllib.error.HTTPError as e:
        if e.code == 404:
            return False
        raise


def run_job(root, job, stop):
    state = root / "state" / job["id"]
    state.mkdir(parents=True, exist_ok=True)
    digest = fingerprint(job)
    identity = state / "spec.json"
    if identity.exists() and json.loads(identity.read_text())["sha256"] != digest:
        raise RuntimeError(f"{job['id']}: specification changed; use a new job id")
    atomic_json(identity, {"sha256": digest, "spec": job})
    if completed(root, job):
        publish_record(root, job, 'complete')
        return
    checkpoint = state / "checkpoint"
    checkpoint.mkdir(exist_ok=True)
    publish_record(root, job, 'preparing', required=True)
    strict = state / 'resume-request.json'
    try:
        if strict.exists() and json.loads(strict.read_text()).get('spec_sha256') != digest:
            raise ValueError('Resume request does not match the run')
        selected = select_checkpoint(root, job, resume_only=strict.exists())
    except (OSError, ValueError, RuntimeError, subprocess.SubprocessError) as exc:
        publish_record(root, job, 'blocked', reason=str(exc))
        raise
    env = dict(os.environ, **job.get('env', {}))
    env.update(SPOT_CHECKPOINT_DIR=str(checkpoint),
               SPOT_RESUMED="1" if (selected if job['checkpoint_contract'] == 'manifest-sha256-v1'
                                    else any(checkpoint.iterdir())) else "0",
               SPOT_OUTPUT_DIR=str(state / "output"))
    # Clear inherited selections: only the current job's validated generation
    # may be passed to an application implementing the new manifest contract.
    env.pop('SPOT_CHECKPOINT_MANIFEST', None)
    if selected:
        env['SPOT_CHECKPOINT_MANIFEST'] = selected['manifest']
    (state / "output").mkdir(exist_ok=True)
    with (state / "stdout.log").open("ab", buffering=0) as out, \
            (state / "stderr.log").open("ab", buffering=0) as err:
        if os.environ.get('SPOT_ACTIVITY_TABLE'):
            from worker_state import admit
            admit(root)
        attempt = {'id': uuid.uuid4().hex, 'sha256': digest, 'started': time.time()}
        atomic_json(state / 'attempt.json', attempt)
        try:
            child = subprocess.Popen(job["argv"], cwd=root / job["cwd"], env=env,
                                 stdout=out, stderr=err, start_new_session=True)
        except (OSError, ValueError) as exc:
            publish_record(root, job, 'blocked', selected, reason=str(exc))
            raise
        status = {'attempt_id': attempt['id'], 'started_unix': attempt['started'],
                  'host': socket.gethostname(), 'pid': child.pid}
        publish_record(root, job, 'running', selected, **status)
        heartbeat = time.monotonic() + 30
        while child.poll() is None:
            if stop.wait(1):
                try:
                    os.killpg(child.pid, signal.SIGTERM)
                    child.wait(timeout=75)
                except subprocess.TimeoutExpired:
                    os.killpg(child.pid, signal.SIGKILL)
                    child.wait()
                except ProcessLookupError:
                    pass
                publish_record(root, job, 'interrupted', selected, **status, exit_code=child.returncode)
                return
            if time.monotonic() >= heartbeat:
                publish_record(root, job, 'running', selected, **status)
                heartbeat = time.monotonic() + 30
        if child.returncode:
            publish_record(root, job, 'failed', selected, **status, exit_code=child.returncode)
            raise RuntimeError(f"{job['id']}: exited {child.returncode}; inspect stderr.log")
        if not stop.is_set():
            atomic_json(state / "complete.json", {"sha256": digest, "completed": time.time()})
            publish_record(root, job, 'complete', selected, **status, exit_code=0)
        else:
            publish_record(root, job, 'interrupted', selected, **status, exit_code=child.returncode)


def supervise(root, monitor=True, once=False, resume_job=None):
    root = root.resolve()
    stop = threading.Event()
    for sig in (signal.SIGINT, signal.SIGTERM):
        signal.signal(sig, lambda *_: stop.set())
    def watch():
        while not stop.wait(5):
            try:
                if interrupted():
                    stop.set()
            except (OSError, urllib.error.URLError):
                # Periodic application checkpoints cover missing notice/metadata.
                pass
    if monitor:
        threading.Thread(target=watch, daemon=True).start()
    with (root / ".supervisor.lock").open("a+") as lock:
        while not stop.is_set():
            try:
                fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
                break
            except BlockingIOError:
                if resume_job is not None:
                    return {'state': 'submitted', 'id': resume_job['id'],
                            'run_record': str(root / run_paths(resume_job)['record'])}
                stop.wait(2)
        if stop.is_set():
            return
        while not stop.is_set():
            # Sequential by default: each job may use all 32 vCPUs / 128 GiB.
            for job in [resume_job] if resume_job is not None else read_jobs(root):
                if stop.is_set():
                    return
                run_job(root, job, stop)
            if once:
                return
            stop.wait(10)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, help='workspace; defaults to the run record location hint for --resume')
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument('--resume', type=Path, help='resume a run.json; never start an empty run')
    mode.add_argument('--list-runs', action='store_true', help='emit the harness-independent run index')
    mode.add_argument('--export-runs', action='store_true', help='backfill run.json for existing durable identities')
    parser.add_argument("--no-imds", action="store_true", help="local test only")
    parser.add_argument("--once", action="store_true")
    parser.add_argument("--validate", action="store_true", help="validate the manifest without starting work")
    args = parser.parse_args()
    try:
        if args.resume:
            path = args.resume if args.resume.is_absolute() or not args.root else args.root / args.resume
            root = (args.root or Path(json.loads(path.read_text())['workspace'])).resolve(strict=True)
            configure_worker(root)
            job = submit_resume(root, path)
            if completed(root, job):
                response = {'state': 'complete', 'id': job['id']}
            elif not args.once:
                response = {'state': 'submitted', 'id': job['id'],
                            'run_record': str(root / run_paths(job)['record'])}
            else:
                response = supervise(root, monitor=not args.no_imds, once=True, resume_job=job)
            print(json.dumps(response or {'state': 'complete' if completed(root, job) else 'interrupted',
                                         'id': job['id'], 'run_record': str(root / run_paths(job)['record'])}))
        elif args.root is None:
            parser.error('--root is required unless --resume supplies a workspace')
        elif args.list_runs or args.export_runs:
            print(json.dumps(list_runs(args.root.resolve(strict=True), export=args.export_runs)))
        elif args.validate:
            read_jobs(args.root.resolve())
        else:
            configure_worker(args.root)
            supervise(args.root, monitor=not args.no_imds, once=args.once)
        return 0
    except (OSError, ValueError, KeyError, TypeError, RuntimeError, subprocess.SubprocessError) as exc:
        print(json.dumps({'state': 'refused', 'error': str(exc)}))
        return 1


if __name__ == '__main__':
    raise SystemExit(main())
