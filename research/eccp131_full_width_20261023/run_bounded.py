#!/usr/bin/env python3
"""Bounded process runner with sampled RSS and immutable per-run files."""
import argparse
import json
import subprocess
import time
from pathlib import Path


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--wall-seconds', type=float, required=True)
    ap.add_argument('--rss-mib', type=int, required=True)
    ap.add_argument('--stdout', type=Path, required=True)
    ap.add_argument('--stderr', type=Path, required=True)
    ap.add_argument('--receipt', type=Path, required=True)
    ap.add_argument('command', nargs=argparse.REMAINDER)
    a = ap.parse_args()
    command = a.command[1:] if a.command[:1] == ['--'] else a.command
    assert command and not a.stdout.exists() and not a.stderr.exists() and not a.receipt.exists()
    start = time.monotonic()
    max_rss_kib = 0
    reason = None
    with a.stdout.open('wb') as out, a.stderr.open('wb') as err:
        proc = subprocess.Popen(command, stdout=out, stderr=err)
        while proc.poll() is None:
            ps = subprocess.run(['ps', '-o', 'rss=', '-p', str(proc.pid)], capture_output=True, text=True)
            if ps.returncode == 0 and ps.stdout.strip():
                max_rss_kib = max(max_rss_kib, int(ps.stdout.strip()))
            elapsed = time.monotonic() - start
            if elapsed > a.wall_seconds:
                reason = 'wall_limit'
            elif max_rss_kib > a.rss_mib * 1024:
                reason = 'rss_limit'
            if reason:
                proc.kill()
                break
            time.sleep(0.2)
        code = proc.wait()
    receipt = {'command': command, 'wall_seconds': time.monotonic()-start,
               'max_rss_kib_sampled': max_rss_kib, 'sample_interval_seconds': 0.2,
               'wall_limit_seconds': a.wall_seconds, 'rss_limit_mib': a.rss_mib,
               'exit_code': code, 'stop_reason': reason}
    a.receipt.write_text(json.dumps(receipt, indent=2)+'\n')
    if code != 0 or reason:
        raise SystemExit(1)


if __name__ == '__main__':
    main()
