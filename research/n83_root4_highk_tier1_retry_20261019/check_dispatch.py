#!/usr/bin/env python3
"""Fail closed if the sealed high-K source, fixture or protocol differs."""

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def verify():
    record = json.loads((HERE / 'dispatch.json').read_text())
    if (record.get('schema') != 'n83.highk-retry-dispatch/v1' or
            record.get('status') != 'SEALED_FOR_DISPATCH' or
            record.get('point_count') != 8 or
            record.get('host_strata') != 1 or
            record.get('producer_cap_seconds') != 30600):
        raise ValueError('high-K dispatch metadata differs')
    actual = {name: digest(ROOT / name) for name in record['sha256']}
    if actual != record['sha256']:
        raise ValueError('sealed high-K dispatch file differs')
    return record


if __name__ == '__main__':
    record = verify()
    print(json.dumps({'status': 'PASS_SEALED_DISPATCH',
                      'file_count': len(record['sha256'])}, sort_keys=True))
