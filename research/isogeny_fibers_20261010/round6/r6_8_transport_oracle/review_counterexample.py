#!/usr/bin/env python3
"""Exact finite witnesses for gaps in the superseded transport proof."""

import itertools
import json


def main():
    original = {
        sum(pair) % 5 for pair in itertools.combinations_with_replacement((0, 1), 2)
    }
    without_identity = {
        sum(pair) % 5 for pair in itertools.combinations_with_replacement((1,), 2)
    }
    assert original == {0, 1, 2}
    assert without_identity == {2}
    assert 1 in original and 1 not in without_identity
    assert len(original - {0}) > len(without_identity - {0})
    assert 1 < 2 / 1  # Claimed image-size lower bound for the identity map.

    points = ((1, 0), (1, 1))
    target = (2, 1)
    lifts = [
        pair
        for pair in itertools.combinations_with_replacement(points, 2)
        if (
            sum(point[0] for point in pair) % 5,
            sum(point[1] for point in pair) % 2,
        )
        == target
    ]
    assert lifts == [((1, 0), (1, 1))]
    chosen_lift = (1, 0)
    arbitrary_lift_sum = (2 * chosen_lift[0] % 5, 2 * chosen_lift[1] % 2)
    assert arbitrary_lift_sum != target
    assert arbitrary_lift_sum[0] == target[0]
    print(
        json.dumps(
            {
                "status": "PASS_COUNTEREXAMPLES",
                "identity_removal": {
                    "group_order": 5,
                    "arity": 2,
                    "original_sums": sorted(original),
                    "identity_removed_sums": sorted(without_identity),
                    "lost_nonzero_target": 1,
                },
                "arbitrary_lift": {
                    "group": "C5 x C2",
                    "projection": "first coordinate",
                    "target": target,
                    "valid_lifts": lifts,
                    "arbitrary_lift_sum": arbitrary_lift_sum,
                },
                "general_simulation_status": "NOT_ESTABLISHED",
            },
            indent=2,
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
