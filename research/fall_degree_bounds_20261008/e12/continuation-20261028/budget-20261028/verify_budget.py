#!/usr/bin/env python3
"""Recompute a conservative reservation; this does not launch or stop AWS jobs."""
import argparse
import json
from decimal import Decimal
from pathlib import Path


def require(condition, message):
    if not condition:
        raise ValueError(message)


def verify(path, extra=Decimal(0)):
    budget = json.loads(path.read_text())
    if not extra.is_finite() or extra < 0:
        raise ValueError("extra reservation must be finite and nonnegative")
    costs = []
    for job in [*budget["existing_jobs"], budget["proposed_new_worker"]]:
        raw = json.loads((path.parent / "aws-rates" / (job["instance_type"] + ".json")).read_text())
        prices = set()
        for product in map(json.loads, raw["PriceList"]):
            attrs = product["product"]["attributes"]
            require(attrs["instanceType"] == job["instance_type"], "wrong quoted instance")
            require(attrs["regionCode"] == "us-east-1", "wrong quoted region")
            require(attrs["operatingSystem"] == "Linux", "wrong quoted OS")
            require(attrs["tenancy"] == "Shared", "wrong quoted tenancy")
            for term in product["terms"]["OnDemand"].values():
                for dimension in term["priceDimensions"].values():
                    if dimension["unit"] == "Hrs":
                        prices.add(Decimal(dimension["pricePerUnit"]["USD"]))
        require(prices == {Decimal(job["hourly_usd"])}, "rate differs from retained AWS quote")
        hours = Decimal(job["maximum_hours"])
        require(hours.is_finite() and hours > 0, "invalid job hours")
        costs.append(hours * prices.pop())
    worker = budget["proposed_new_worker"]
    require(worker["maximum_instances"] == 1, "only one new worker is reserved")
    require(Decimal(worker["maximum_hours"]) <= 24, "worker exceeds 24-hour reservation")
    require(all(Decimal(v).is_finite() and Decimal(v) >= 0 for v in budget["reserves_usd"].values()), "invalid reserve")
    reserves = sum((Decimal(v) for v in budget["reserves_usd"].values()), Decimal(0))
    total = sum(costs, reserves) + extra
    ceiling = Decimal(budget["total_ceiling"])
    require(ceiling.is_finite() and 0 < ceiling <= 200, "ceiling exceeds user authorization")
    if total > ceiling:
        raise ValueError(f"reservation {total} exceeds authorized ceiling {ceiling}")
    return {"schema": "dataset-budget-check/v1", "reserved_usd": str(total),
            "remaining_usd": str(ceiling - total), "authorized_ceiling_usd": str(ceiling),
            "billing_verified": False, "launch_performed": False}


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--budget", type=Path, default=Path(__file__).with_name("budget.json"))
    parser.add_argument("--extra-reservation-usd", type=Decimal, default=Decimal(0))
    args = parser.parse_args()
    try:
        print(json.dumps(verify(args.budget, args.extra_reservation_usd), indent=2))
    except (AssertionError, ValueError, KeyError) as error:
        parser.exit(1, f"budget check failed: {error}\n")
