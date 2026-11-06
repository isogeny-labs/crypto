"""Bounded full-suite replay using the just-validated release test executable."""
from pathlib import Path
import datetime
import fcntl
import hashlib
import json
import os
import signal
import subprocess

root = Path('/home/ec2-user/crypto-w2a-diem-relations')
out = root / 'research/diem_quartic_relations_20261018/resume-20261019'
lock = (out / '.full-integration.lock').open('a+')
fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
receipt = out / 'full-integration.json'
if receipt.exists():
    raise SystemExit('Existing full-suite receipt: inspect it before any rerun')
binary = root / 'target/release/deps/crypto_lib-565bcb20cb4d5bb4'
def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()
state = {'started_at': now(), 'status': 'running', 'driver_pid': os.getpid(),
         'binary': str(binary), 'binary_sha256': hashlib.sha256(binary.read_bytes()).hexdigest(),
         'timeout_seconds': 1800, 'cpu_affinity': [24, 25],
         'command': [str(binary), '--test-threads=2']}
def save():
    temp = receipt.with_suffix('.tmp')
    with temp.open('w') as f:
        json.dump(state, f, indent=2)
        f.flush()
        os.fsync(f.fileno())
    os.replace(temp, receipt)
    fd = os.open(out, os.O_RDONLY)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)
with (out / 'full-integration.log').open('xb') as log:
    p = subprocess.Popen(['taskset', '-c', '24,25', 'nice', '-n', '15'] + state['command'],
                         cwd=root, env=dict(os.environ, RAYON_NUM_THREADS='1'),
                         stdin=subprocess.DEVNULL, stdout=log, stderr=subprocess.STDOUT,
                         start_new_session=True)
    state['pid'] = p.pid
    save()
    try:
        code = p.wait(timeout=1800)
    except subprocess.TimeoutExpired:
        os.killpg(p.pid, signal.SIGTERM)
        try:
            p.wait(timeout=10)
        except subprocess.TimeoutExpired:
            os.killpg(p.pid, signal.SIGKILL)
            p.wait()
        code = 124
        state['timed_out'] = True
state.update(exit_code=code, finished_at=now(), status='passed' if code == 0 else 'failed')
save()
