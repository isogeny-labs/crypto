#!/usr/bin/env python3
"""Freeze the new points against every fixture in the preceding finalist audit."""

import hashlib
import json
import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
PRIOR = ROOT / 'research/n83_fresh_cold_finalists_20261019'
AUDIT = HERE / 'fresh_exclusion_audit.json'
FINALISTS_SHA256 = '4eb139e73c99f0921041a911d30698997913d1e71155a8e09035ffa647e02a58'


def digest(data):
    return hashlib.sha256(data).hexdigest()


def points(data):
    rows = []
    for line in data.decode().splitlines():
        try:
            row = json.loads(line)
            if (isinstance(row, list) and len(row) == 2 and
                    all(isinstance(value, (str, int)) for value in row)):
                rows.append(tuple(int(value) for value in row))
        except (ValueError, TypeError, json.JSONDecodeError):
            continue
    return rows


def repository_blob(relative):
    path = ROOT / relative
    if path.exists():
        return path.read_bytes()
    return subprocess.check_output(['git', '-C', str(ROOT), 'show',
                                    f'HEAD:{relative}'])


def audit():
    new_blob = (HERE / 'fresh_targets.jsonl').read_bytes()
    fresh = points(new_blob)
    if len(fresh) != 16 or len(set(fresh)) != 16:
        raise ValueError('new fixture count or distinctness differs')
    old = json.loads((PRIOR / 'fresh_exclusion_audit.json').read_text())
    if old['status'] != 'PASS_ZERO_OVERLAP':
        raise ValueError('prior fixture audit was not admitted')
    prior_paths = [item['path'] for item in old['prior_files']]
    prior_paths.append('research/n83_fresh_cold_finalists_20261019/fresh_targets.jsonl')
    if len(prior_paths) != len(set(prior_paths)):
        raise ValueError('duplicate prior fixture path')
    prior_files, previous = [], set()
    for relative in prior_paths:
        blob = repository_blob(relative)
        rows = points(blob)
        prior_files.append({'path': relative, 'sha256': digest(blob),
                            'point_rows': len(rows)})
        previous.update(rows)
    old_targets = (PRIOR / 'fresh_targets.jsonl').read_bytes()
    if (digest(old_targets) != old['fresh_targets_sha256'] or
            len(previous) < old['prior_unique_point_count'] + 16):
        raise ValueError('previous finalist fixture not fully included')
    historical = (HERE / 'historical_finalist_targets.jsonl').read_bytes()
    if digest(historical) != FINALISTS_SHA256:
        raise ValueError('historical finalist fixture differs')
    result = {
        'schema': 'n83.fresh-cold-binary-retest-exclusion/v1',
        'fresh_targets_sha256': digest(new_blob),
        'fresh_count': len(fresh),
        'prior_audit_sha256': digest((PRIOR / 'fresh_exclusion_audit.json').read_bytes()),
        'prior_files': prior_files,
        'prior_unique_point_count': len(previous),
        'prior_overlap_count': len(set(fresh) & previous),
        'historical_finalist_sha256': FINALISTS_SHA256,
        'historical_finalist_count': len(points(historical)),
        'historical_finalist_overlap_count': len(set(fresh) & set(points(historical))),
        'status': 'PASS_ZERO_OVERLAP' if not set(fresh) & previous and
                  not set(fresh) & set(points(historical)) else 'OVERLAP_FOUND',
    }
    blob = json.dumps(result, sort_keys=True, indent=2) + '\n'
    if AUDIT.exists() and AUDIT.read_text() != blob:
        raise ValueError('sealed exclusion audit differs')
    if not AUDIT.exists():
        AUDIT.write_text(blob)
    print(json.dumps({key: result[key] for key in (
        'status', 'fresh_targets_sha256', 'fresh_count',
        'prior_unique_point_count', 'prior_overlap_count',
        'historical_finalist_overlap_count')}, sort_keys=True))


if __name__ == '__main__':
    audit()
