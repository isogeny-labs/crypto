"""Stop the identified local N83 run if the internal scratch disk gets tight."""

import argparse
import json
import os
import shutil
import signal
import subprocess
import time
from pathlib import Path


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--pid', type=int, required=True)
    parser.add_argument('--min-free-gib', type=int, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    started = time.time()
    threshold = args.min_free_gib * 1024**3
    while True:
        result = subprocess.run(['ps', '-p', str(args.pid), '-o', 'comm='],
                                capture_output=True, text=True)
        if result.returncode != 0 or 'koblitz_orbit_dlp_fast' not in result.stdout:
            status = 'CHILD_EXITED_NO_DISK_STOP'
            break
        free = shutil.disk_usage('/private/tmp').free
        if free < threshold:
            status = 'DISK_GUARD_STOP'
            try:
                os.killpg(args.pid, signal.SIGTERM)
            except ProcessLookupError:
                status = 'CHILD_EXITED_NO_DISK_STOP'
            break
        time.sleep(30)
    receipt = {
        'schema': 'n83.local-disk-guard/v1', 'pid': args.pid,
        'min_free_bytes': threshold, 'status': status,
        'observed_free_bytes': shutil.disk_usage('/private/tmp').free,
        'started_unix': started, 'finished_unix': time.time(),
    }
    args.output.write_text(json.dumps(receipt, indent=2, sort_keys=True) + '\n')
    print(json.dumps(receipt, sort_keys=True), flush=True)


if __name__ == '__main__':
    main()
