#!/usr/bin/env python3
"""Recompute observed axes from retained custody receipts without inferring missing cells."""

import collections
import hashlib
import json
from pathlib import Path


HERE = Path(__file__).resolve().parent
RESEARCH = HERE.parent
INVENTORY = RESEARCH / "ml_dataset_inventory_20261031" / "inventory-v2.json"
PRIME_AUDIT = HERE / "prime-twist-archive-audit.json"
FAMILIES = ("koblitz", "binary", "prime")
METHODS = ("enumerate", "bsgs", "sat", "weil", "f4", "f5", "f6", "root", "pkm", "double-large-prime")
FIELD_SIZES = tuple(range(23, 32)) + tuple(range(47, 62))
ARITIES = tuple(range(2, 16))
CURVE_SEEDS = 16
GENERATORS = 8
KEYS = 64


def read(path):
    return json.loads(path.read_text())


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def require(test, message):
    if not test:
        raise ValueError(message)


def main():
    inventory = read(INVENTORY)
    prime_audit = read(PRIME_AUDIT)
    require(inventory["schema"] == "ml-dataset.local-campaign-inventory/v2", "unexpected inventory")
    require(prime_audit["schema"] == "ml-dataset.prime23-twist-archive-audit/v1" and
            prime_audit["total"] == 64 and len(prime_audit["rows"]) == 64,
            "prime archive twist audit absent or incomplete")
    planned = len(FIELD_SIZES) * len(ARITIES) * len(FAMILIES) * len(METHODS) * CURVE_SEEDS * GENERATORS * KEYS * 2
    require(planned == inventory["default_plan_method_cells"] == 165_150_720, "default plan mismatch")
    counts = collections.Counter()
    explicit_twists = collections.defaultdict(set)
    observed_sizes = set()
    observed_arities = set()
    observed_families = set()
    observed_methods = set()
    unknown_twist_receipts = 0
    native_statuses = collections.Counter()
    campaigns = []
    for row in inventory["campaigns"]:
        name = row["campaign"]
        base = RESEARCH / name
        plan_path = base / "plan.json"
        plan = read(plan_path)
        require(digest(plan_path) == row["plan_file_sha256"]["plan.json"], f"{name}: plan digest")
        if name == "ml_dataset_local_campaign_20261031":
            require(prime_audit["campaign_plan_sha256"] == digest(plan_path),
                    "prime archive twist audit plan binding mismatch")
        require(set(plan["families"]) <= set(FAMILIES), f"{name}: family outside requested axis")
        require(set(plan["field_sizes"]) <= set(FIELD_SIZES), f"{name}: size outside requested axis")
        require(set(plan["arities"]) <= set(ARITIES), f"{name}: arity outside requested axis")
        require(set(plan["solvers"]) <= set(METHODS), f"{name}: method outside requested axis")
        require(len(plan["families"]) == len(plan["field_sizes"]) == len(plan["arities"]) == 1,
                f"{name}: campaign plan axes need per-cell disaggregation")
        family, n, m = plan["families"][0], plan["field_sizes"][0], plan["arities"][0]
        receipts = sorted((base / "checkpoints").glob("shard-*.json"))
        require(len(receipts) == row["observed_receipts"] == row["planned_cells"], f"{name}: denominator")
        campaign_methods = collections.Counter()
        campaign_unknown_twists = 0
        for index, path in enumerate(receipts):
            checkpoint = read(path)
            require(path.name == f"shard-{index:02d}.json", f"{name}: shard gap")
            require(checkpoint["index"] == checkpoint["custody"]["index"] == index,
                    f"{name}: index mismatch")
            native = checkpoint["custody"]["native_status"]
            require(native["verified_and_exported"], f"{name}: absent native export")
            require(checkpoint["published"]["archive_fresh_get"] and
                    checkpoint["published"]["pointer_fresh_get"], f"{name}: absent producer readback")
            spec = native.get("cell_spec")
            if spec is None:
                require(len(plan["solvers"]) == 1, f"{name}: method cannot be disaggregated")
                method = plan["solvers"][0]
                unknown_twist_receipts += 1
                campaign_unknown_twists += 1
                require(family == "prime" and n == 23 and m == 2 and index < 64,
                        f"{name}: no archive twist audit for this cell")
                archive_row = prime_audit["rows"][index]
                require(archive_row["index"] == index and
                        archive_row["archive_sha256"] == checkpoint["custody"]["archive_sha256"] and
                        archive_row["s3_uri"] == checkpoint["custody"]["s3_uri"] and
                        type(archive_row["twist"]) is bool,
                        f"{name}: archive twist audit binding mismatch")
                explicit_twists[family].add(archive_row["twist"])
            else:
                method = spec["solver"]
                require(method in plan["solvers"] and type(spec["twist"]) is bool,
                        f"{name}: invalid cell spec")
                explicit_twists[family].add(spec["twist"])
            counts[(family, n, m, method)] += 1
            campaign_methods[method] += 1
            if "native_status" in native:
                native_statuses[native["native_status"]] += 1
            observed_sizes.add(n)
            observed_arities.add(m)
            observed_families.add(family)
            observed_methods.add(method)
        campaigns.append({"name": name, "receipts": len(receipts),
            "method_receipts": dict(sorted(campaign_methods.items())),
            "twist_not_explicit_in_checkpoint_receipts": campaign_unknown_twists})
    require(sum(counts.values()) == inventory["totals"]["observed_receipts"] == 148,
            "inventory receipt total mismatch")
    require(prime_audit["untwisted"] == prime_audit["twisted"] == 32 and
            explicit_twists["prime"] == {False, True}, "prime twist coverage mismatch")
    grid = [{"family": family, "method": method,
             "observed_receipts": sum(v for (f, _, _, s), v in counts.items() if f == family and s == method)}
            for family in FAMILIES for method in METHODS]
    output = {"schema": "ml-dataset.requested-versus-observed-coverage/v2",
        "inventory_sha256": digest(INVENTORY),
        "prime_twist_archive_audit_sha256": digest(PRIME_AUDIT),
        "claim_scope": "retained local synthetic receipt coverage, not completion of the default plan",
        "requested": {"field_sizes": list(FIELD_SIZES), "arities": list(ARITIES),
            "families": list(FAMILIES), "methods": list(METHODS), "twists": [False, True],
            "curve_seeds_per_axis": CURVE_SEEDS, "generators_per_curve": GENERATORS,
            "keys_per_generator": KEYS, "default_method_cells": planned,
            "specific_prime_modulus": "2^61-1"},
        "observed": {"retained_receipts": sum(counts.values()),
            "independently_solved_ecdlp_cells": inventory["totals"]["independently_solved_ecdlp_cells"],
            "field_sizes": sorted(observed_sizes), "arities": sorted(observed_arities),
            "families": sorted(observed_families), "methods": sorted(observed_methods),
            "explicit_twists_by_family": {f: sorted(explicit_twists[f]) for f in FAMILIES},
            "twist_not_explicit_in_checkpoint_receipts": unknown_twist_receipts,
            "prime_twists_replayed_from_verified_restricted_archives": prime_audit["total"],
            "native_status_counts_where_retained": dict(sorted(native_statuses.items())),
            "family_method_grid": grid, "campaigns": campaigns},
        "unobserved_axes": {"field_sizes": sorted(set(FIELD_SIZES) - observed_sizes),
            "arities": sorted(set(ARITIES) - observed_arities),
            "methods": sorted(set(METHODS) - observed_methods),
            "specific_prime_modulus_2^61-1": "no retained cell",
            "n_47_to_61": "no retained cell"},
        "interpretation": ["Counts are campaign receipts, not unique curve or key identities across campaigns.",
            "The 64 prime twists were read from restricted archive manifests after exact archive hash checks; their Git checkpoints alone omit cell_spec.",
            "Absent family-method intersections are coverage gaps, not evidence that a method is mathematically unsupported.",
            "Native partial, timeout, and budget-exhausted outcomes remain retained rather than promoted to solutions."]}
    target = HERE / "coverage-v2.json"
    data = (json.dumps(output, sort_keys=True, indent=2) + "\n").encode()
    if target.exists() and target.read_bytes() != data:
        raise ValueError("coverage-v2.json differs; review source changes before replacing")
    target.write_bytes(data)
    print(json.dumps({"retained": sum(counts.values()),
        "observed_sizes": sorted(observed_sizes), "observed_arities": sorted(observed_arities),
        "observed_methods": sorted(observed_methods), "unobserved_methods": output["unobserved_axes"]["methods"]},
        sort_keys=True))


if __name__ == "__main__":
    main()
