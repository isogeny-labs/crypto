#!/usr/bin/env python3
"""Exact three-sum coverage on the frozen PKM roster; diagnostic, not timing."""

import hashlib
import json
from pathlib import Path

from verify_roster import add, factor_base, first_curve, first_zeta, multiply, selected_primes, target_scalar


HERE = Path(__file__).resolve().parent


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def rows():
    for t in (3, 4, 5):
        width = 1 << t
        for index, p in enumerate(selected_primes(t)[1]):
            a, b, r, generator, rejected, least_roots = first_curve(p)
            zeta = first_zeta(p, width)
            _, points = factor_base(p, a, b, width, zeta, least_roots)
            pairs = {add(left, right, a, p) for left in points for right in points}
            triples = {add(pair, point, a, p) for pair in pairs for point in points}
            used = set()
            targets = []
            for j in range(16):
                _, _, scalar = target_scalar(t, index, j, used, r)
                target = multiply(scalar, generator, a, p)
                targets.append({"index": j, "scalar": scalar, "direct_three_sum": target in triples})
            yield {"t": t, "curve_index": index, "p": p, "a": a, "b": b,
                   "r": r, "generator": list(generator), "rejected_pairs": rejected,
                   "zeta": zeta, "kummer_abscissae": width,
                   "lifted_points": len(points), "distinct_pair_sums": len(pairs),
                   "distinct_three_sums": len(triples), "group_order": r,
                   "fixed_targets_directly_reachable": sum(x["direct_three_sum"] for x in targets),
                   "fixed_targets": targets}


def main():
    result = {"schema": "pkm-roster-reachability/v1",
              "meaning": "exact direct three-point sums; excludes relation shifts, costs and solver timing",
              "selection_rule_sha256": digest(HERE / "ROSTER_SELECTION_20261125.md"),
              "verifier_sha256": digest(HERE / "verify_roster.py"),
              "diagnostic_source_sha256": digest(Path(__file__)),
              "curves": list(rows())}
    print(json.dumps(result, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
