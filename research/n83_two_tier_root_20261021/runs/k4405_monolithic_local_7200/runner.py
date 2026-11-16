"""Bounded correctness control for the opt-in N83 indexed five-byte load rule."""

import argparse
import hashlib
import json
import os
import signal
import subprocess
import sys
import time
from pathlib import Path

PINNED = Path('/Volumes/SSD990-2/tmp/n83_highk_retry_20261019')
LOAD80 = Path('/Volumes/SSD990-2/tmp/n83_load80_20261020')
BINARY = Path('/Volumes/SSD990-2/tmp/n83_load80_build_20261020/release/examples/koblitz_orbit_dlp_fast')
sys.path.insert(0, str(PINNED))
from research.n83_highk_online_distribution_20261020 import arms, run as runner  # noqa: E402


def digest(path):
    h = hashlib.sha256()
    with path.open('rb') as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b''):
            h.update(chunk)
    return h.hexdigest()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--load', choices=('75', '80'), required=True)
    parser.add_argument('--k', type=int, choices=(4405, 4500, 4549), default=4405)
    parser.add_argument('--cap-seconds', type=int, default=1800)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    if args.k > 4405 and args.load != '80':
        parser.error('larger K controls require the opt-in load80 request')
    if not 60 <= args.cap_seconds <= 7200:
        parser.error('cap must be between 60 and 3600 seconds')
    root = args.output.resolve()
    root.mkdir(parents=True, exist_ok=False)
    arm = next(a for a in arms.ARMS if a[0] == 'k4405_wide3')
    source = PINNED / 'research/n83_highk_online_distribution_20261020/screen_targets.jsonl'
    answer = PINNED / 'research/n83_highk_online_distribution_20261020/screen_answers.jsonl'
    target = root / 'target.jsonl'
    target.write_text(source.open().readline())
    expected_scalar = int(json.loads(answer.open().readline())['scalar'])
    env = runner.ic_environment(root, arm)
    env['KIC_INDEXED5_LOAD75'] = '1' if args.load == '75' else '0'
    env['KIC_INDEXED5_LOAD80'] = '1' if args.load == '80' else '0'
    env['TMPDIR'] = '/Volumes/SSD990-2/tmp/n83_load80_tmp_20261020'
    command = [str(BINARY), f'construct:83:1:{args.k}', str(target), '29',
               str(root / 'targets_result.jsonl')]
    (root / 'input.json').write_text(json.dumps({
        'schema': 'n83.load80-local-control/v1', 'load': args.load,
        'k': args.k,
        'binary_sha256': digest(BINARY), 'target_sha256': digest(target),
        'source_sha256': digest(LOAD80 / 'examples/koblitz_orbit_dlp_fast.rs'),
        'control_script_sha256': digest(Path(__file__)),
        'source_head': subprocess.check_output(
            ['git', 'rev-parse', 'HEAD'], cwd=LOAD80, text=True).strip(),
        'expected_scalar': expected_scalar, 'command': command,
        'environment_flags': {k: v for k, v in env.items()
                              if k.startswith('KIC_')},
        'cap_seconds': args.cap_seconds, 'rss_cap_bytes': 26 * 1024**3,
    }, indent=2, sort_keys=True) + '\n')
    start = time.monotonic()
    stop_reason = None
    peak_kib = 0
    with (root / 'stdout.jsonl').open('wb') as stdout, (root / 'stderr.log').open('wb') as stderr:
        child = subprocess.Popen(command, cwd=LOAD80, env={**os.environ, **env},
                                 stdout=stdout, stderr=stderr,
                                 start_new_session=True)
        while child.poll() is None:
            if time.monotonic() - start > args.cap_seconds:
                stop_reason = f'TIMEOUT_{args.cap_seconds}S'
                os.killpg(child.pid, signal.SIGTERM)
                break
            ps = subprocess.run(['ps', '-o', 'rss=', '-p', str(child.pid)],
                                capture_output=True, text=True)
            if ps.returncode == 0 and ps.stdout.strip().isdigit():
                peak_kib = max(peak_kib, int(ps.stdout.strip()))
                if peak_kib * 1024 > 26 * 1024**3:
                    stop_reason = 'RSS_CAP_26GIB'
                    os.killpg(child.pid, signal.SIGTERM)
                    break
            time.sleep(2)
        try:
            code = child.wait(timeout=20)
        except subprocess.TimeoutExpired:
            os.killpg(child.pid, signal.SIGKILL)
            code = child.wait()
    result = {'schema': 'n83.load80-local-control-result/v1',
              'status': stop_reason or ('PASS_PROCESS' if code == 0 else 'FAILED_PROCESS'),
              'exit_code': code, 'wall_seconds': time.monotonic() - start,
              'sampled_peak_rss_bytes': peak_kib * 1024}
    (root / 'process_result.json').write_text(json.dumps(result, indent=2,
                                                         sort_keys=True) + '\n')
    print(json.dumps(result, sort_keys=True), flush=True)


if __name__ == '__main__':
    main()
