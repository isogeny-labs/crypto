#!/usr/bin/env python3
"""Replay scoped weak-curve criteria against the exact curve registry.

The output deliberately distinguishes an arithmetic witness, a geometric
certificate, a conditional subgroup map, and a costed attack.  An absent
witness within the selected bound is not a global nonexistence statement.
"""

from __future__ import annotations

import argparse
from collections import Counter
import hashlib
import json
import math
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
REGISTRY = ROOT / "docs/curves/registry.json"
COVERS = ROOT / "docs/curves/covers.json"
TRANSFERS = ROOT / "docs/curves/jacobian-transfers.json"
SUMMARY = Path(__file__).with_name("summary.json")
PAIRING_BOUND = 20


def canonical_bytes(value: object) -> bytes:
    return (json.dumps(value, sort_keys=True, separators=(",", ":")) + "\n").encode()


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def unique_by_slug(rows: list[dict], label: str) -> dict[str, dict]:
    result = {}
    for row in rows:
        slug = row["slug"]
        if slug in result:
            raise ValueError(f"duplicate {label} slug: {slug}")
        result[slug] = row
    return result


def bounded_embedding_degree(q: int, r: int, bound: int) -> int | None:
    """Exact least positive k <= bound with r dividing q**k - 1."""
    if q < 2 or r < 2 or bound < 1 or math.gcd(q, r) != 1:
        raise ValueError("q, r, bound must be valid and gcd(q,r)=1")
    residue = q % r
    for k in range(1, bound + 1):
        if residue == 1:
            return k
        residue = residue * q % r
    return None


def field_bits(q: int) -> int:
    """Ceiling of log2(q), including the exact-power-of-two boundary."""
    if q < 2:
        raise ValueError("field cardinality must exceed one")
    return (q - 1).bit_length()


def load_and_audit(
    registry_path: Path = REGISTRY,
    covers_path: Path = COVERS,
    transfers_path: Path = TRANSFERS,
    pairing_bound: int = PAIRING_BOUND,
) -> tuple[dict, list[dict]]:
    registry = json.loads(registry_path.read_text())
    covers = json.loads(covers_path.read_text())
    transfers = json.loads(transfers_path.read_text())
    if covers["registry_sha256"] != sha256(registry_path):
        raise ValueError("cover report is bound to a different registry")
    models = unique_by_slug(registry["curves"], "registry")
    cover_rows = unique_by_slug(covers["curves"], "cover")
    transfer_rows = unique_by_slug(transfers["curves"], "transfer")
    if models.keys() != cover_rows.keys():
        raise ValueError("cover and registry model sets differ")

    model_genus = Counter()
    subgroup_genus = Counter()
    pairing_degrees = Counter()
    large_field_window = Counter()
    route_rows = Counter()
    records = []
    seen_ec1 = set()
    for slug, model in models.items():
        cover = cover_rows[slug]
        if cover["model_sha256"] != hashlib.sha256(model["model_json"].encode()).hexdigest():
            raise ValueError(f"model hash mismatch: {slug}")
        if cover["status"] != "verified_over_declared_field" or not cover["exists"]:
            continue
        cert = cover["certificate"]
        genus, degree = int(cert["genus"]), int(cert["degree"])
        if genus < 2 or genus > 8 or degree < 2:
            continue
        model_genus[genus] += 1
        model_params = json.loads(model["model_json"])
        if "p" in model_params:
            model_characteristic = int(model_params["p"])
            model_degree = int(model_params.get("k", 1))
        else:
            model_characteristic = 2
            model_degree = int(model_params["modulus"], 16).bit_length() - 1
        for rep in model["representations"]:
            ec1 = rep["ec1"]
            if ec1 in seen_ec1:
                raise ValueError(f"duplicate EC1 representation: {ec1}")
            seen_ec1.add(ec1)
            field = rep["field"]
            if (int(field["characteristic"]), int(field["degree"])) != (
                model_characteristic, model_degree
            ):
                raise ValueError(f"selected subgroup/field mismatch: {ec1}")
            q = int(field["characteristic"]) ** int(field["degree"])
            r = int(rep["curve"]["subgroup_order"])
            cofactor = int(rep["curve"]["cofactor"])
            if r < 2 or cofactor < 1 or r * cofactor != int(model["order"]):
                raise ValueError(f"selected subgroup/order mismatch: {ec1}")
            k = bounded_embedding_degree(q, r, pairing_bound)
            coprime = math.gcd(degree, r) == 1
            subgroup_genus[(genus, "coprime" if coprime else "noncoprime")] += 1
            pairing_degrees[str(k) if k is not None else "above_bound"] += 1
            if 192 <= field_bits(q) <= 252:
                large_field_window[(genus, "witness" if k is not None else "above_bound")] += 1
            records.append(
                {
                    "slug": slug,
                    "icv1": model["icv1"],
                    "ec1": ec1,
                    "field_cardinality": str(q),
                    "selected_order": str(r),
                    "cover": {
                        "genus": genus,
                        "degree": degree,
                        "construction": cert["construction"],
                        "geometry": "verified_over_declared_field",
                        "subgroup_gate": "coprime_degree" if coprime else "unresolved",
                        "base_field_change": False,
                    },
                    "pairing": {
                        "bound": pairing_bound,
                        "first_divisibility_degree": k,
                        "arithmetic": "witness" if k is not None else "absent_through_bound",
                        "mov_reduction": "requires_pairing_and_cost_check",
                        "frey_ruck_reduction": "requires_pairing_and_cost_check",
                    },
                }
            )

    for slug, curve in transfer_rows.items():
        if slug not in models or curve["icv1"] != models[slug]["icv1"]:
            raise ValueError(f"transfer identity mismatch: {slug}")
        for route in curve["routes"]:
            raw_genus = route.get("destination", {}).get("genus")
            if raw_genus is None or not str(raw_genus).isdecimal():
                continue
            genus = int(raw_genus)
            if 2 <= genus <= 8:
                route_rows[(genus, route["kind"], route["evidence"], route["transport"])] += 1

    records.sort(key=lambda row: (row["slug"], row["ec1"]))
    summary = {
        "schema": "weak-curve-criteria-audit/v1",
        "inputs_sha256": {
            "registry": sha256(registry_path),
            "covers": sha256(covers_path),
            "transfers": sha256(transfers_path),
        },
        "pairing_bound": pairing_bound,
        "model_count": len(models),
        "selected_representations": len(records),
        "certified_same_field_models_by_genus": {str(k): v for k, v in sorted(model_genus.items())},
        "selected_cover_degree_gate": [
            {"genus": g, "gate": gate, "count": count}
            for (g, gate), count in sorted(subgroup_genus.items())
        ],
        "pairing_first_divisibility_degree": dict(sorted(pairing_degrees.items())),
        "selected_field_bits_192_252": [
            {"genus": g, "pairing_arithmetic": outcome, "count": count}
            for (g, outcome), count in sorted(large_field_window.items())
        ],
        "enumerated_routes_genus_2_8": [
            {"genus": g, "kind": kind, "evidence": evidence, "transport": transport, "count": count}
            for (g, kind, evidence, transport), count in sorted(route_rows.items())
        ],
        "interpretation": {
            "cover": "The cover certificate is same-field geometry; coprime degree gives injectivity conditional on the standard push-pull identity and recorded subgroup order.",
            "pairing": "The least k through the bound is exact divisibility arithmetic. A MOV or Frey-Ruck transfer and relative cost require further checks.",
            "descent": "Routes retain their source evidence and transport labels. A proposed or exact-invariant row is not promoted to a constructed subgroup-preserving map.",
        },
    }
    return summary, records


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="compare against committed summary")
    parser.add_argument("--query", help="print one exact slug, ICV1, or EC1 record")
    parser.add_argument("--all-jsonl", action="store_true", help="stream all selected representation rows")
    parser.add_argument("--genus", type=int, choices=range(2, 9), help="stream enumerated routes at this genus")
    parser.add_argument("--pairing", nargs=2, metavar=("Q", "R"), type=int,
                        help="compute exact bounded divisibility degree for field size Q and selected order R")
    parser.add_argument("--bound", type=int, default=PAIRING_BOUND)
    args = parser.parse_args()
    if args.pairing:
        q, r = args.pairing
        k = bounded_embedding_degree(q, r, args.bound)
        print(json.dumps({"field_cardinality": str(q), "selected_order": str(r),
                          "bound": args.bound, "first_divisibility_degree": k,
                          "arithmetic": "witness" if k is not None else "absent_through_bound"},
                         sort_keys=True))
        return
    summary, records = load_and_audit(pairing_bound=args.bound)
    if args.check:
        if args.bound != PAIRING_BOUND:
            raise SystemExit("--check uses the frozen pairing bound")
        if SUMMARY.read_bytes() != canonical_bytes(summary):
            raise SystemExit("criteria summary differs from current inputs")
        for row in records:
            q, r = int(row["field_cardinality"]), int(row["selected_order"])
            direct = next((k for k in range(1, PAIRING_BOUND + 1) if pow(q, k, r) == 1), None)
            if direct != row["pairing"]["first_divisibility_degree"]:
                raise SystemExit(f"independent modular-power replay failed: {row['ec1']}")
        print("WEAK_CRITERIA_REPLAY_PASS", len(records))
    elif args.query:
        matches = [r for r in records if args.query in (r["slug"], r["icv1"], r["ec1"])]
        route_index = unique_by_slug(json.loads(TRANSFERS.read_text())["curves"], "transfer")
        if not matches:
            models = json.loads(REGISTRY.read_text())["curves"]
            covers = unique_by_slug(json.loads(COVERS.read_text())["curves"], "cover")
            for model in models:
                if args.query in (model["slug"], model["icv1"]):
                    cover = covers[model["slug"]]
                    matches = [{
                        "slug": model["slug"], "icv1": model["icv1"],
                        "selected_representations": 0,
                        "cover": {"geometry": cover["status"],
                                  "genus": cover["certificate"]["genus"] if cover["certificate"] else None,
                                  "degree": cover["certificate"]["degree"] if cover["certificate"] else None},
                        "pairing": {"arithmetic": "unknown_without_selected_order"},
                    }]
                    break
        for match in matches:
            match["enumerated_routes_genus_2_8"] = [
                route for route in route_index.get(match["slug"], {}).get("routes", [])
                if str(route.get("destination", {}).get("genus", "")).isdecimal()
                and 2 <= int(route["destination"]["genus"]) <= 8
            ]
        print(json.dumps(matches, indent=2))
    elif args.genus is not None:
        for curve in json.loads(TRANSFERS.read_text())["curves"]:
            for route in curve["routes"]:
                if str(route.get("destination", {}).get("genus")) == str(args.genus):
                    print(json.dumps({"slug": curve["slug"], "icv1": curve["icv1"],
                                      "kind": route["kind"], "evidence": route["evidence"],
                                      "transport": route["transport"],
                                      "genus": args.genus}, sort_keys=True))
    elif args.all_jsonl:
        for record in records:
            print(json.dumps(record, sort_keys=True, separators=(",", ":")))
    else:
        print(json.dumps(summary, indent=2))


if __name__ == "__main__":
    main()
