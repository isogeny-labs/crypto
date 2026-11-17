#!/usr/bin/env python3
"""Check distinct overlapping provider runners and reject duplicate hosts."""

import json
from pathlib import Path
from tempfile import TemporaryDirectory

from research.n83_fresh_cold_finalists_20261019 import provider_jobs


def main():
    run_id = 39170000000
    with TemporaryDirectory() as temporary:
        root = Path(temporary)
        strata, jobs = [], []
        for number in range(2):
            folder = root / str(number)
            folder.mkdir()
            host = {
                'runner_name': f'GitHub Actions {100 + number}',
                'recorded_utc': '2026-10-18T20:05:00Z',
                'run_url': f'https://github.com/aburan28/crypto/actions/runs/{run_id}',
                'parent_instance_id': f'gh-{run_id}-1-fresh-cold-finalists-stratum',
            }
            (folder / 'host.json').write_text(json.dumps(host))
            strata.append({'stratum': number, 'host': host,
                           'analysis': {'status': 'STRATUM_ADMITTED'},
                           'local_replay_exit': 0, 'results': str(folder)})
            jobs.append({
                'id': 300 + number, 'name': f'fresh-cold-finalists-stratum ({number})',
                'run_id': run_id, 'run_attempt': 1,
                'status': 'completed', 'conclusion': 'success',
                'runner_id': 100 + number,
                'runner_name': host['runner_name'],
                'started_at': '2026-10-18T20:00:00Z',
                'completed_at': '2026-10-18T21:00:00Z',
            })
        path = root / 'jobs.json'
        path.write_text(json.dumps({'total_count': 2, 'jobs': jobs}))
        result = provider_jobs.verify(path, strata, run_id, 1,
                                      'fresh-cold-finalists-stratum')
        assert result['status'] == 'PASS_TWO_DISTINCT_CONCURRENT_RUNNERS'
        jobs[1]['runner_id'] = jobs[0]['runner_id']
        path.write_text(json.dumps({'total_count': 2, 'jobs': jobs}))
        try:
            provider_jobs.verify(path, strata, run_id, 1,
                                 'fresh-cold-finalists-stratum')
        except ValueError as error:
            assert 'distinct provider' in str(error)
        else:
            raise AssertionError('duplicate provider runner was admitted')
    print(json.dumps({'status': 'PASS_FINAL_EXTENDED_PROVIDER_JOBS',
                      'cases': 2}, sort_keys=True))


if __name__ == '__main__':
    main()
