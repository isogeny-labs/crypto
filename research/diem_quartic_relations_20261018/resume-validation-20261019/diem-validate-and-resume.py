"""Host-side bounded validation and restart of the existing frozen Diem grid."""
from pathlib import Path
import datetime
import fcntl
import hashlib
import json
import os
import signal
import subprocess

ROOT = Path('/home/ec2-user/crypto-w2a-diem-relations')
BASE = ROOT / 'research/diem_quartic_relations_20261018/resume-20261019'
BASE.mkdir(exist_ok=True)
lock = (BASE / '.driver.lock').open('a+')
fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)

def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

out = BASE / ('run-' + datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ'))
out.mkdir()
state = {'started_at': now(), 'driver_pid': os.getpid(), 'status': 'validating', 'steps': [], 'output': str(out)}

def save():
    data = (json.dumps(state, indent=2) + '\n').encode()
    for path in [out / 'state.json', BASE / 'latest-state.json']:
        tmp = path.with_suffix('.tmp')
        with tmp.open('wb') as f:
            f.write(data)
            f.flush()
            os.fsync(f.fileno())
        os.replace(tmp, path)
        fd = os.open(path.parent, os.O_RDONLY)
        try:
            os.fsync(fd)
        finally:
            os.close(fd)

env = dict(os.environ, CARGO_TARGET_DIR=str(ROOT / 'target'), CARGO_BUILD_JOBS='2', RAYON_NUM_THREADS='1')
deployment = json.loads((BASE / 'deployment.json').read_text())
expected = dict(deployment['deployed'])
expected['src/cryptanalysis/diem_quartic.rs'] = deployment['original_sha256']['src/cryptanalysis/diem_quartic.rs']
state['source_sha256'] = expected
save()

def check_sources():
    for rel, digest in expected.items():
        if hashlib.sha256((ROOT / rel).read_bytes()).hexdigest() != digest:
            raise RuntimeError('source changed during validation: ' + rel)

def run(name, cmd, seconds):
    row = {'name': name, 'command': cmd, 'started_at': now(), 'timeout_seconds': seconds}
    state['steps'].append(row)
    with (out / (name + '.log')).open('wb') as log:
        p = subprocess.Popen(['taskset', '-c', '26,27,28', 'nice', '-n', '10'] + cmd,
                             cwd=ROOT, env=env, stdin=subprocess.DEVNULL,
                             stdout=log, stderr=subprocess.STDOUT, start_new_session=True)
        row['pid'] = p.pid
        save()
        try:
            code = p.wait(timeout=seconds)
        except subprocess.TimeoutExpired:
            os.killpg(p.pid, signal.SIGTERM)
            try:
                p.wait(timeout=10)
            except subprocess.TimeoutExpired:
                os.killpg(p.pid, signal.SIGKILL)
                p.wait()
            code = 124
            row['timed_out'] = True
    row.update(exit_code=code, finished_at=now())
    save()
    if code:
        raise RuntimeError(f'{name} exited {code}; no dependent work launched')

try:
    check_sources()
    run('library-tests', ['cargo', 'test', '--release', '--locked', '--offline', '--lib', 'diem_quartic', '-j', '2', '--', '--test-threads=1'], 1800)
    run('checkpoint-tests', ['cargo', 'test', '--release', '--locked', '--offline', '--example', 'diem_quartic_study', '-j', '2', '--', '--test-threads=1'], 1800)
    run('build', ['cargo', 'build', '--release', '--locked', '--offline', '--example', 'diem_quartic_study', '-j', '2'], 1800)
    check_sources()
    binary = ROOT / 'target/release/examples/diem_quartic_study'
    state['binary_sha256'] = hashlib.sha256(binary.read_bytes()).hexdigest()
    run('preflight', [str(binary), 'gen-status'], 60)
    # A second generator is excluded by the generator's kernel-held lock.
    # Also catch a surviving old binary which predates checkpoint locking.
    for proc in Path('/proc').iterdir():
        if not proc.name.isdigit():
            continue
        try:
            args = (proc / 'cmdline').read_bytes().split(b'\0')
            if args and b'diem_quartic_study' in args[0] and b'gen' in args:
                raise RuntimeError('existing generator detected: ' + proc.name)
        except (FileNotFoundError, PermissionError, ProcessLookupError):
            pass
    state['status'] = 'generating'
    save()
    run('generation', [str(binary), 'gen'], 86400)
    state['status'] = 'generation_complete'
    state['next_gate'] = 'Review and commit the frozen list before starting cost cells.'
except Exception as exc:
    state['status'] = 'failed'
    state['error'] = str(exc)
finally:
    state['finished_at'] = now()
    save()
