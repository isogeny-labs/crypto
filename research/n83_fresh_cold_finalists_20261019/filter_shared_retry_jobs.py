#!/usr/bin/env python3
"""Bind a three-job shared-binary retry to the frozen two-stratum provider check."""

import argparse
import hashlib
import json
from datetime import datetime
from pathlib import Path

STEM = 'fresh-cold-finalists-stratum'
NAMES = {'shared-binary-build', f'{STEM} (0)', f'{STEM} (1)'}


def instant(value):
    result = datetime.fromisoformat(value.replace('Z', '+00:00'))
    if result.tzinfo is None:
        raise ValueError('provider timestamp lacks timezone')
    return result


def filter_jobs(raw, raw_sha256, run_id, attempt, head_sha):
    jobs = raw.get('jobs')
    if (raw.get('total_count') != 3 or not isinstance(jobs, list) or
            len(jobs) != 3 or {job.get('name') for job in jobs} != NAMES):
        raise ValueError('expected exactly one build and two stratum jobs')
    by_name = {job['name']: job for job in jobs}
    if any(job.get('run_id') != run_id or
           job.get('run_attempt') != attempt or
           job.get('head_sha') != head_sha or
           job.get('status') != 'completed' or
           not isinstance(job.get('id'), int) or job['id'] <= 0
           for job in jobs):
        raise ValueError('provider run identity or completion differs')
    build = by_name['shared-binary-build']
    if build.get('conclusion') != 'success':
        raise ValueError('shared binary build did not succeed')
    strata = [by_name[f'{STEM} ({number})'] for number in range(2)]
    if any(instant(build['completed_at']) > instant(job['started_at'])
           for job in strata):
        raise ValueError('stratum job started before shared build completed')
    if len({job['id'] for job in jobs}) != 3:
        raise ValueError('provider job IDs are not distinct')
    return {
        'total_count': 2,
        'jobs': strata,
        'shared_retry_filter': {
            'schema': 'n83.shared-binary-provider-filter/v1',
            'raw_provider_jobs_sha256': raw_sha256,
            'build_job_id': build['id'],
            'build_runner_id': build.get('runner_id'),
            'build_completed_at': build['completed_at'],
            'build_conclusion': build['conclusion'],
        },
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('raw_provider_jobs', type=Path)
    parser.add_argument('--run-id', type=int, required=True)
    parser.add_argument('--run-attempt', type=int, default=1)
    parser.add_argument('--head-sha', required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    raw_blob = args.raw_provider_jobs.read_bytes()
    result = filter_jobs(json.loads(raw_blob), hashlib.sha256(raw_blob).hexdigest(),
                         args.run_id, args.run_attempt, args.head_sha)
    blob = json.dumps(result, indent=2, sort_keys=True) + '\n'
    if args.output.exists() and args.output.read_text() != blob:
        raise ValueError('filtered provider jobs differ from sealed output')
    args.output.write_text(blob)
    print(json.dumps({'status': 'PASS_SHARED_BUILD_AND_TWO_STRATA',
                      'build_job_id': result['shared_retry_filter']['build_job_id'],
                      'stratum_job_ids': [job['id'] for job in result['jobs']]},
                     sort_keys=True))


if __name__ == '__main__':
    main()
