"""Owned subprocess group with finite deadline and capped combined stdout log."""
from pathlib import Path
import math
import os
import selectors
import signal
import subprocess
import sys
import time


def run(argv, cwd, log, *, wall_seconds, memory_bytes, log_bytes, cpu=None,
        file_bytes=4*1024**3, on_start=None):
    if wall_seconds <= 0 or min(memory_bytes, log_bytes, file_bytes) <= 0:
        raise ValueError('positive limits required')
    cpu = min(os.sched_getaffinity(0)) if cpu is None else cpu
    if cpu not in os.sched_getaffinity(0): raise ValueError('CPU is outside allowed affinity')
    command = [sys.executable, str(Path(__file__).with_name('limits_exec.py')),
               str(cpu), str(memory_bytes), str(math.ceil(wall_seconds)), str(file_bytes), *argv]
    start = time.monotonic()
    deadline = start + wall_seconds
    process = None
    reason = None
    written = 0
    discarded = 0

    def terminate(why):
        nonlocal reason
        if reason is not None: return
        reason = why
        try: os.killpg(process.pid, signal.SIGKILL)
        except ProcessLookupError: pass

    with Path(log).open('xb') as output:
        try:
            process = subprocess.Popen(command, cwd=cwd, stdout=subprocess.PIPE,
                                       stderr=subprocess.STDOUT, start_new_session=True)
            if on_start is not None: on_start(process.pid)
            with selectors.DefaultSelector() as selector:
                selector.register(process.stdout, selectors.EVENT_READ)
                while selector.get_map():
                    if time.monotonic() >= deadline: terminate('deadline')
                    for key, _ in selector.select(0.05):
                        data = os.read(key.fileobj.fileno(), 65536)
                        if not data:
                            selector.unregister(key.fileobj)
                            continue
                        keep = min(len(data), log_bytes - written)
                        output.write(data[:keep])
                        output.flush()
                        written += keep
                        discarded += len(data) - keep
                        if discarded: terminate('log_budget')
            try: code = process.wait(timeout=max(0, deadline-time.monotonic()))
            except subprocess.TimeoutExpired:
                terminate('deadline')
                code = process.wait()
        except BaseException:
            if process is not None:
                terminate('supervisor_error')
                process.wait()
            raise
        finally:
            if process is not None and process.stdout is not None: process.stdout.close()
    return dict(pid=process.pid, native_exit=code,
                exit_code=124 if reason=='deadline' else 125 if reason=='log_budget' else code,
                stop_reason=reason or 'process_exit', log_bytes=written,
                discarded_log_bytes=discarded, cpu=cpu, memory_limit_bytes=memory_bytes,
                wall_limit_seconds=wall_seconds, retries=0)
