#!/usr/bin/env python3
"""Bounded current-source K4405 wide3 Bloom diagnostic run on the frozen control point."""
import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import re
import signal
import subprocess
import time

ROOT = Path('/Volumes/SSD990/crypto/worktrees/n83-root4-frontier-20261018')
SOURCE = ROOT / 'examples/koblitz_orbit_dlp_fast.rs'
TARGETS = Path('/private/tmp/n83_wide3_fresh4096_20261019/targets.jsonl')
SOURCE_SHA = '4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507'
TARGETS_SHA = '281a088cc49658973375ab88359d65a3b4d51d2819a297872b5fe5da01783895'
CAP = 2100
RSS_CAP = 24 * 1024**3


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def utc():
    return datetime.now(timezone.utc).isoformat()


def write(path, value):
    path.write_text(json.dumps(value, sort_keys=True, indent=2) + '\n')


def memory_free():
    output = subprocess.check_output(['/usr/bin/memory_pressure', '-Q'], text=True)
    total = int(re.search(r'The system has (\d+)', output).group(1))
    free = int(re.search(r'System-wide memory free percentage:\s*(\d+)%', output).group(1))
    return total, free, output


def rss(pid):
    result = subprocess.run(['/bin/ps', '-o', 'rss=', '-p', str(pid)],
                            capture_output=True, text=True)
    if result.returncode or not result.stdout.strip().isdigit():
        raise RuntimeError('RSS telemetry unavailable')
    return int(result.stdout.strip()) * 1024


def stop(process):
    try:
        os.killpg(process.pid, signal.SIGTERM)
    except ProcessLookupError:
        return
    try:
        process.wait(timeout=5)
    except subprocess.TimeoutExpired:
        os.killpg(process.pid, signal.SIGKILL)
        process.wait()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--binary', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    binary = args.binary.resolve(); output = args.output.resolve()
    if output.exists():
        raise FileExistsError(output)
    if sha(SOURCE) != SOURCE_SHA or sha(TARGETS) != TARGETS_SHA:
        raise ValueError('pinned source or target differs')
    if not binary.is_file():
        raise FileNotFoundError(binary)
    total, free, pressure = memory_free()
    if total < 40 * 1024**3 or free < 50:
        raise ValueError('pilot host memory gate failed')
    output.mkdir(parents=True)
    flags = {
        'KIC_PACKED_ROOT_TABLE':'1', 'KIC_PACKED_ROOT_TABLE_DENSE':'0',
        'KIC_PACKED_ROOT_TABLE_LOAD75':'0', 'KIC_STATE28':'0',
        'KIC_STATE26':'0', 'KIC_ROOT_ONLY22':'0', 'KIC_ROOT_ONLY21':'0',
        'KIC_ROOT_ONLY11':'1', 'KIC_INDEXED_ROOT5':'1',
        'KIC_INDEXED_ROOT4':'0', 'KIC_ROOT_BLOOM':'1',
        'KIC_ROOT_BLOOM_PROFILE':'wide3', 'KIC_SYMMETRIC_S3':'1',
        'KIC_INDEXED5_LOAD75':'1', 'KIC_INDEXED5_FIXED30':'0',
        'KIC_INDEXED5_ID30':'1', 'KIC_INDEXED4_FIXED31':'0',
        'KIC_SPARSE_CANONICAL':'1', 'KIC_RANK_THREADS':'4',
        'KIC_TARGET_THREADS':'2', 'KIC_TARGET_CANCEL':'1',
        'KIC_INDEX_ROOT_BATCH':'1',
        'KIC_DUMP_BASE':str(output/'base_dump.jsonl'),
    }
    env={key:value for key,value in os.environ.items() if not key.startswith('KIC_')}
    env.update(flags)
    command=[str(binary),'construct:83:1:4405',str(TARGETS),'29',
             str(output/'targets_result.jsonl')]
    write(output/'source.json', {'source_sha256':SOURCE_SHA,
          'binary_sha256':sha(binary),'target_sha256':TARGETS_SHA,
          'runner_sha256':sha(Path(__file__)), 'source_commit':'08660bd57ae1ea79f1d2d34639a0d3d36d0286fa',
          'algorithm_flags':flags, 'command':command})
    write(output/'host_before.json', {'recorded_utc':utc(),
          'physical_memory_bytes':total,'free_percent':free,
          'memory_pressure':pressure,'rss_cap_bytes':RSS_CAP})
    began=utc(); t0=time.monotonic(); peak=0; minimum=free; status='FAILED'
    with (output/'stdout.jsonl').open('wb') as stdout, (output/'stderr.log').open('wb') as stderr:
        process=subprocess.Popen(command,cwd=ROOT,env=env,stdout=stdout,
                                 stderr=stderr,start_new_session=True)
        while process.poll() is None:
            if time.monotonic()-t0 > CAP:
                status='TIMEOUT';stop(process);break
            try:
                peak=max(peak,rss(process.pid))
                _, current, _=memory_free()
                minimum=min(minimum,current)
            except (RuntimeError, subprocess.CalledProcessError):
                if process.poll() is not None: break
                status='TELEMETRY_FAILURE';stop(process);break
            if peak>RSS_CAP:
                status='RSS_CAP';stop(process);break
            if current<15:
                status='HOST_MEMORY_PRESSURE';stop(process);break
            time.sleep(1)
        else:
            status='PASS_PRODUCER' if process.returncode==0 else 'FAILED'
    if status=='FAILED' and process.returncode==0:
        status='PASS_PRODUCER'
    _, finish, finish_pressure=memory_free()
    write(output/'usage.json',{'status':status,'exit_code':process.returncode,
          'started_utc':began,'finished_utc':utc(),'wall_seconds':time.monotonic()-t0,
          'peak_sampled_rss_bytes':peak,'min_sampled_free_percent':minimum,
          'finish_free_percent':finish,'memory_pressure_after':finish_pressure,
          'cap_seconds':CAP,'hardware_tier':'local_correctness_diagnostic'})
    print(json.dumps({'status':status,'exit_code':process.returncode,
          'peak_sampled_rss_bytes':peak,'min_sampled_free_percent':minimum},sort_keys=True))
    return 0 if status=='PASS_PRODUCER' else 1


if __name__=='__main__':
    raise SystemExit(main())
