#!/usr/bin/env python3
"""The shortcut applies only after an identical successful full replay."""

from pathlib import Path
from types import SimpleNamespace
import unittest

from research.n83_fresh_cold_binary_retest_20261021.base_replay_cache import (
    ValidatedBaseCache,
)


def group(calls):
    def full(dump, summary):
        calls.append((dump, summary))
        if dump.get('invalid'):
            raise ValueError('full replay rejected base')
        return [tuple(point) for point in dump['factor_base_point_coordinates']]

    return SimpleNamespace(__file__=str(Path(__file__).resolve()),
                           replay_base=full,
                           point_from_json=tuple)


class ValidatedBaseCacheTest(unittest.TestCase):
    def test_identical_input_reuses_one_successful_proof(self):
        calls = []
        cache = ValidatedBaseCache()
        first, second = group(calls), group(calls)
        cache.install(first)
        cache.install(second)
        dump = {'factor_base_point_coordinates': [[1, 2], [3, 4]],
                'base_hash': 'known'}
        summary = {'factor_base_points': 2, 'orbit_columns': 1,
                   'base_hash': 'known'}
        self.assertEqual(first.replay_base(dump, summary), [(1, 2), (3, 4)])
        reordered = dict(reversed(list(dump.items())))
        self.assertEqual(second.replay_base(reordered, summary),
                         [(1, 2), (3, 4)])
        self.assertEqual(len(calls), 1)
        self.assertEqual(cache.receipt()['identical_reuses'], 1)

    def test_changed_dump_or_summary_runs_full_proof(self):
        calls = []
        cache = ValidatedBaseCache()
        module = group(calls)
        cache.install(module)
        dump = {'factor_base_point_coordinates': [[1, 2]]}
        summary = {'factor_base_points': 1, 'orbit_columns': 1,
                   'base_hash': 'one'}
        module.replay_base(dump, summary)
        module.replay_base({'factor_base_point_coordinates': [[2, 3]]}, summary)
        module.replay_base(dump, {**summary, 'base_hash': 'two'})
        self.assertEqual(len(calls), 3)
        self.assertEqual(cache.receipt()['identical_reuses'], 0)

    def test_failed_proof_is_never_cached(self):
        calls = []
        cache = ValidatedBaseCache()
        module = group(calls)
        cache.install(module)
        dump = {'factor_base_point_coordinates': [], 'invalid': True}
        summary = {'factor_base_points': 0, 'orbit_columns': 0,
                   'base_hash': 'bad'}
        for _ in range(2):
            with self.assertRaisesRegex(ValueError, 'full replay rejected'):
                module.replay_base(dump, summary)
        self.assertEqual(len(calls), 2)
        self.assertEqual(cache.receipt()['unique_proved_inputs'], 0)


if __name__ == '__main__':
    unittest.main()
