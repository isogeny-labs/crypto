#!/usr/bin/env python3
"""Adapt the frozen rho worker JSON to the strict isolated-bench protocol."""

import base64
import json
import subprocess
import sys


PAIR_FIELDS = (
    'job_id', 'checkin_blake3_time_normalized', 'walkers_done',
    'walk_steps', 'distinguished_points', 'dead_trails',
    'units_completed', 'checkins', 'q_index', 'q_hex',
    'dp_bits', 'step_cap', 'checkin_every',
)


def main():
    if len(sys.argv) != 10:
        raise SystemExit('usage: wrapper BINARY MANIFEST Q_INDEX serial|batch WALKERS DP_BITS STEP_CAP BATCH_WIDTH CHECKIN_EVERY')
    binary, manifest, q_index, mode, walkers, dp_bits, step_cap, batch_width, checkin_every = sys.argv[1:]
    command = [binary, manifest, q_index, mode, walkers, dp_bits, step_cap,
               batch_width, checkin_every]
    result = subprocess.run(command, text=True, capture_output=True, check=False)
    if result.returncode != 0 or result.stderr:
        sys.stderr.write(result.stderr or f'worker exited {result.returncode}\n')
        raise SystemExit(result.returncode or 1)
    data = json.loads(result.stdout)
    if (data.get('schema') != 'rho-worker-batch-benchmark-v1' or
            data.get('status') != 'complete_frozen_work' or
            data.get('verified_distinguished_points') != data.get('distinguished_points') or
            data.get('q_index') != int(q_index) or data.get('mode') != mode or
            data.get('walkers_requested') != int(walkers) or
            data.get('dp_bits') != int(dp_bits) or
            data.get('step_cap') != int(step_cap) or
            data.get('batch_width') != int(batch_width) or
            data.get('checkin_every') != int(checkin_every) or
            data.get('total_charged_ns') != data.get('setup_ns', -1) + data.get('walk_ns', -1)):
        raise SystemExit('worker output failed independent wrapper checks')
    raw = base64.urlsafe_b64encode(result.stdout.encode()).decode()
    fields = {name: data[name] for name in PAIR_FIELDS}
    fields.update(verified=1, online_ms=data['total_charged_ns'] / 1_000_000,
                  peak_rss_kib=data['peak_rss_kib'], raw_b64=raw)
    print(' '.join(f'{key}={value}' for key, value in fields.items()))


if __name__ == '__main__':
    main()
