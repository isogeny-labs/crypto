"""Reuse a completed base proof only for an identical canonical input."""

import hashlib
import json
from pathlib import Path


class ValidatedBaseCache:
    def __init__(self):
        self._proved = set()
        self.hits = 0
        self.misses = 0

    def install(self, group):
        original = group.replay_base
        group_source_sha256 = hashlib.sha256(
            Path(group.__file__).read_bytes()).hexdigest()

        def replay(dump, summary):
            canonical = json.dumps(dump, sort_keys=True, separators=(',', ':'),
                                   ensure_ascii=False).encode('utf-8')
            key = (group_source_sha256, hashlib.sha256(canonical).digest(),
                   summary['factor_base_points'], summary['orbit_columns'],
                   summary['base_hash'])
            if key in self._proved:
                self.hits += 1
                return [group.point_from_json(point)
                        for point in dump['factor_base_point_coordinates']]
            points = original(dump, summary)
            self._proved.add(key)
            self.misses += 1
            return points

        group.replay_base = replay

    def receipt(self):
        return {'schema': 'n83.verified-base-replay-cache/v1',
                'proofs_executed': self.misses, 'identical_reuses': self.hits,
                'unique_proved_inputs': len(self._proved),
                'identity': 'sha256(canonical_dump),group_source_sha256,'
                            'factor_base_points,orbit_columns,base_hash'}
