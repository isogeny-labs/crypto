#!/usr/bin/env python3
"""Build the deterministic public source bundle for the fresh-Q panel."""

import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent))
import make_worker_source_bundle as base  # noqa: E402

PREFIX = 'experiments/rho-worker-phase-profile-20261017/tier0-ec2-20261019'
EXTRA = [
    f'{PREFIX}/isolated_bench.py',
    f'{PREFIX}/CONFIRMATION_PLAN.md',
    f'{PREFIX}/analyze.py',
    f'{PREFIX}/analyze_confirmation.py',
    f'{PREFIX}/build_confirmation_plan.py',
    f'{PREFIX}/build_manifest.py',
    f'{PREFIX}/configure_host.sh',
    f'{PREFIX}/isolated_worker_runner.py',
    f'{PREFIX}/make_confirmation_source_bundle.py',
    f'{PREFIX}/worker_wrapper.py',
]


def main():
    fixtures = sorted((HERE / 'confirmation-fixtures').glob('*.json'))
    if len(fixtures) != 34:  # 32 Q files, manifest, independent verification.
        raise ValueError('confirmation fixtures incomplete')
    base.EXPLICIT.extend(EXTRA)
    base.EXPLICIT.extend(str(path.relative_to(base.ROOT)) for path in fixtures)
    base.main()


if __name__ == '__main__':
    main()
