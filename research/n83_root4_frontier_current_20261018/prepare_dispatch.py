#!/usr/bin/env python3
"""Seal the fixed root4 frontier after native pilot replay."""

import json
import subprocess
import sys
import tarfile
from pathlib import Path
from tempfile import TemporaryDirectory

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
if sys.path and Path(sys.path[0]).resolve() == HERE:
    sys.path.pop(0)
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from research.n83_root4_frontier_current_20261018 import arms, check_dispatch, run  # noqa: E402


def main():
    archive = HERE / 'pilots/k3596.tar.gz'
    with TemporaryDirectory(prefix='n83-k3596-dispatch-replay-') as temporary:
        with tarfile.open(archive, 'r:gz') as stream:
            if any(name != 'pilot_work' and not name.startswith('pilot_work/') for name in
                   stream.getnames()):
                raise ValueError('pilot archive has an unexpected member')
            stream.extractall(temporary, filter='data')
        pilot = Path(temporary) / 'pilot_work'
        before = (pilot / 'verification.json').read_bytes()
        process = subprocess.run(
            [sys.executable, '-m',
             'research.n83_root4_frontier_current_20261018.verify_pilot',
             str(pilot)], cwd=ROOT, capture_output=True, text=True,
            check=False)
        if (process.returncode != 0 or process.stderr or
                (pilot / 'verification.json').read_bytes() != before or
                json.loads(process.stdout)['status'] != 'PASS'):
            raise ValueError('archived K3596 pilot fails independent replay')
    chosen, choice = arms.choose()
    paths = check_dispatch.required_paths(chosen)
    hashes = {path: check_dispatch.digest(ROOT / path) for path in sorted(paths)}
    dispatch = {
        'schema': 'n83.root4-frontier-dispatch/v1',
        'status': 'SEALED_FOR_DISPATCH',
        'point_count': 16, 'host_strata': 2, 'points_per_host': 8,
        'producer_cap_seconds': run.GLOBAL_PRODUCER_CAP_SECONDS,
        'job_cap_minutes': 330, 'source_sha256': run.SOURCE_SHA256,
        'targets_sha256': run.TARGETS_SHA256,
        'answers_sha256': run.ANSWERS_SHA256,
        'ordered_candidate_ids': [item[3] for item in chosen],
        'choice': choice, 'sha256': hashes,
    }
    path = HERE / 'dispatch.json'
    blob = json.dumps(dispatch, sort_keys=True, indent=2) + '\n'
    if path.exists() and path.read_text() != blob:
        raise ValueError('sealed dispatch differs')
    path.write_text(blob)
    check_dispatch.main()


if __name__ == '__main__':
    main()
