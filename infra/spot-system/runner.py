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
import subprocess
import threading
import time
import urllib.error
import urllib.request


def atomic_json(path, value):
    temp = path.with_suffix(".tmp")
    with temp.open("w") as f:
        json.dump(value, f, sort_keys=True)
        f.flush()
        os.fsync(f.fileno())
    os.replace(temp, path)
    fd = os.open(path.parent, os.O_DIRECTORY)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)


def read_jobs(root):
    data = json.loads((root / "jobs.json").read_text())
    if not isinstance(data, list):
        raise ValueError("jobs.json must contain a list")
    seen = set()
    for j in data:
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
                isinstance(a, str) for a in j["argv"]):
            raise ValueError("argv must be a nonempty string list")
        cwd = (root / j["cwd"]).resolve()
        if not cwd.is_relative_to(root.resolve()) or not cwd.is_dir():
            raise ValueError("cwd must be an existing directory within the durable workspace")
    return data


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
    fingerprint = hashlib.sha256(json.dumps(job, sort_keys=True).encode()).hexdigest()
    identity = state / "spec.json"
    if identity.exists() and json.loads(identity.read_text())["sha256"] != fingerprint:
        raise RuntimeError(f"{job['id']}: specification changed; use a new job id")
    atomic_json(identity, {"sha256": fingerprint, "spec": job})
    if (state / "complete.json").exists():
        if json.loads((state / 'complete.json').read_text()).get('sha256') != fingerprint:
            raise RuntimeError(f"{job['id']}: completion marker does not match the job")
        return
    checkpoint = state / "checkpoint"
    checkpoint.mkdir(exist_ok=True)
    selected = None
    if job['checkpoint_contract'] == 'manifest-sha256-v1':
        binary = os.environ.get('SPOT_CHECKPOINT_DISCOVERY_BIN',
                                str(Path(__file__).with_name('bin') / 'checkpoint-discovery'))
        discovery = subprocess.run([binary, '--root', str(root), '--job', job['id'],
                                    '--require-valid'], capture_output=True, text=True, timeout=120)
        if discovery.returncode:
            raise RuntimeError(f"{job['id']}: checkpoint discovery rejected recovery; inspect the inventory")
        report = json.loads(discovery.stdout)
        selected = report['jobs'][0]['selected']
    env = dict(os.environ, SPOT_CHECKPOINT_DIR=str(checkpoint),
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
        if job['checkpoint_contract'] == 'manifest-sha256-v1':
            atomic_json(state / 'attempt.json', {'sha256': fingerprint, 'started': time.time()})
        child = subprocess.Popen(job["argv"], cwd=root / job["cwd"], env=env,
                                 stdout=out, stderr=err, start_new_session=True)
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
                return
        if child.returncode:
            raise RuntimeError(f"{job['id']}: exited {child.returncode}; inspect stderr.log")
        if not stop.is_set():
            atomic_json(state / "complete.json", {"sha256": fingerprint, "completed": time.time()})


def supervise(root, monitor=True, once=False):
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
                stop.wait(2)
        if stop.is_set():
            return
        while not stop.is_set():
            # Sequential by default: each job may use all 32 vCPUs / 128 GiB.
            for job in read_jobs(root):
                if stop.is_set():
                    return
                run_job(root, job, stop)
            if once:
                return
            stop.wait(10)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, required=True)
    parser.add_argument("--no-imds", action="store_true", help="local test only")
    parser.add_argument("--once", action="store_true")
    parser.add_argument("--validate", action="store_true", help="validate the manifest without starting work")
    args = parser.parse_args()
    if args.validate:
        read_jobs(args.root.resolve())
    else:
        supervise(args.root, monitor=not args.no_imds, once=args.once)
