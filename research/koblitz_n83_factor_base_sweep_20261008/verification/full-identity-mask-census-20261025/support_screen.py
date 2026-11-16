#!/usr/bin/env python3
"""Exact combinatorial target-coverage ceilings for unexcluded N83 masks."""

import argparse
from decimal import Decimal, localcontext
import hashlib
import json
import math
from pathlib import Path
import sys

from pair_screen import GROUP_ARITHMETIC_SHA256
from run import HERE, STUDY, file_sha, require

sys.path.insert(0, str(STUDY))
from chain_search_supervisor import PRIMARY_ORDER  # noqa: E402


def count_sums(base_points: int, residual_points: int,
               base_summands: int, residual_summands: int, mask: int) -> int:
    require(base_points > 0 and residual_points > 0
            and (base_summands, residual_summands) in ((5, 1), (4, 2))
            and mask in (0, 1, 2), "frozen unexcluded N83 pattern")
    # Mask 1 makes the first two base summands cancel. Mask 2 makes the
    # first three base summands sum to O. Count only the remaining terms;
    # ignoring whether the cancelled prefix exists makes this an upper bound.
    removed = {0: 0, 1: 2, 2: 3}[mask]
    base_left = base_summands - removed
    return (math.comb(base_points + base_left - 1, base_left)
            * math.comb(residual_points + residual_summands - 1,
                        residual_summands))


def scientific(numerator: int, denominator: int) -> str:
    with localcontext() as context:
        context.prec = 18
        return f"{Decimal(numerator) / Decimal(denominator):.6E}"


def screen(pair: dict, pair_sha256: str) -> dict:
    require(file_sha(STUDY / "chain_search_supervisor.py")
            == GROUP_ARITHMETIC_SHA256
            and PRIMARY_ORDER == 2417851639230796216685689,
            "pinned primary subgroup order and source")
    require(pair["schema"] == "n83.full-identity-mask-pair-screen/v1"
            and pair["selected_best_total_runtime"] is None
            and len(pair["results"]) == 6,
            "exact archive-backed point-count input")
    rows = []
    for source in pair["results"]:
        require(source["base_points"] == 99600
                and source["residual_points"]
                == (99600 if source["pool"] == "hash" else 16102)
                and source["base_plus_residual_ordered_pair_hits"] == 0
                and source["residual_plus_residual_ordered_pair_hits"] == 0,
                "same exact pair-gated base and residual sets")
        for base_summands, residual_summands in ((5, 1), (4, 2)):
            for mask in (0, 1, 2):
                count = count_sums(source["base_points"],
                                   source["residual_points"],
                                   base_summands, residual_summands, mask)
                clipped = min(count, PRIMARY_ORDER - 1)
                rows.append({
                    "trial": source["trial"], "pool": source["pool"],
                    "base_summands": base_summands,
                    "residual_summands": residual_summands,
                    "identity_mask": mask,
                    "remaining_base_summands": base_summands - {0: 0, 1: 2, 2: 3}[mask],
                    "remaining_residual_summands": residual_summands,
                    "unordered_multiset_sum_ceiling": str(count),
                    "nonidentity_target_coverage_ceiling": {
                        "numerator": str(clipped),
                        "denominator": str(PRIMARY_ORDER - 1),
                        "decimal": scientific(clipped, PRIMARY_ORDER - 1),
                    },
                })
    require(len(rows) == 36, "all twelve strata and three unexcluded masks")
    return {"schema": "n83.full-identity-mask-support-screen/v1",
            "pair_screen_sha256": pair_sha256,
            "subgroup_order": str(PRIMARY_ORDER),
            "interpretation": "Upper bound on the fraction of nonidentity subgroup targets representable by each residual sumset; not an outcome or probability for a pinned trial target",
            "rows": rows,
            "relation_search_executed": False,
            "selected_best_total_runtime": None}


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--pair", type=Path, default=HERE / "pair-screen.json")
    args = parser.parse_args()
    raw = args.pair.read_bytes()
    result = screen(json.loads(raw), hashlib.sha256(raw).hexdigest())
    print(json.dumps(result, indent=2, sort_keys=True))
