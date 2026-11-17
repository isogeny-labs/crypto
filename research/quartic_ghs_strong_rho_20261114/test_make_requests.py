#!/usr/bin/env python3
import copy
import datetime as dt
import importlib.util
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("make_requests", HERE / "make_requests.py")
assert SPEC and SPEC.loader
M = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(M)


def receipt(now: dt.datetime) -> dict:
    stamp = now.isoformat().replace("+00:00", "Z")
    h = "a" * 64
    readbacks = []
    for name in sorted(M.DAILY_BUDGETS):
        readbacks.append({
            "name": name,
            "read_ok": True,
            "period": "DAILY",
            "limit_usd": 200,
            "actual_spend_usd": 10,
            "reserved_spend_usd": 0,
            "remaining_usd": 190,
            "retrieved_at": stamp,
        })
    readbacks.append({
        "name": M.MONTHLY_BUDGET,
        "read_ok": True,
        "period": "MONTHLY",
        "limit_usd": 1000,
        "actual_spend_usd": 994.75,
        "reserved_spend_usd": 0,
        "remaining_usd": 5.25,
        "retrieved_at": stamp,
    })
    return {
        "schema": "ghs.strong-rho.preflight/v1",
        "observed_at": stamp,
        "account_id": M.ACCOUNT,
        "region": M.REGION,
        "budgets": {
            "daily_allowance_usd": 200,
            "monthly_allowance_usd": 1000,
            "unreserved_headroom_usd": 5.25,
            "readbacks": readbacks,
        },
        "pool": {
            "pool": "cpu-spot",
            "all_tenant_inventory": True,
            "fits_next_wave": True,
            "requested_cpu": 8,
            "requested_memory": "8Gi",
            "receipt_sha256": h,
        },
        "quota": {"fits_next_wave": True, "receipt_sha256": h},
        "active_jobs": {"recorded": True, "receipt_sha256": h},
        "checkpoint_gate": {"ok": True, "receipt_sha256": h},
        "git_custody": {"ok": True, "receipt_sha256": h},
    }


class RequestTests(unittest.TestCase):
    def setUp(self) -> None:
        self.now = dt.datetime(2026, 11, 14, 23, 0, tzinfo=dt.timezone.utc)

    def test_complete_preflight_passes(self) -> None:
        M.validate_preflight(receipt(self.now), self.now)

    def test_stale_preflight_fails(self) -> None:
        value = receipt(self.now - dt.timedelta(minutes=31))
        with self.assertRaisesRegex(M.Refusal, "stale"):
            M.validate_preflight(value, self.now)

    def test_budget_and_pool_fail_closed(self) -> None:
        for path, value in [
            (("budgets", "unreserved_headroom_usd"), 4.99),
            (("pool", "fits_next_wave"), False),
            (("pool", "all_tenant_inventory"), False),
            (("checkpoint_gate", "ok"), False),
        ]:
            case = copy.deepcopy(receipt(self.now))
            case[path[0]][path[1]] = value
            with self.assertRaises(M.Refusal):
                M.validate_preflight(case, self.now)

    def test_budget_headroom_is_recomputed(self) -> None:
        case = receipt(self.now)
        case["budgets"]["readbacks"][-1]["actual_spend_usd"] = 999
        with self.assertRaisesRegex(M.Refusal, "arithmetic"):
            M.validate_preflight(case, self.now)

    def test_budget_duplicates_are_rejected(self) -> None:
        case = receipt(self.now)
        case["budgets"]["readbacks"].append(copy.deepcopy(case["budgets"]["readbacks"][0]))
        with self.assertRaisesRegex(M.Refusal, "duplicate"):
            M.validate_preflight(case, self.now)

    def test_unknown_budget_and_nonintegral_cpu_are_rejected(self) -> None:
        extra = receipt(self.now)
        row = copy.deepcopy(extra["budgets"]["readbacks"][0])
        row["name"] = "unreviewed-budget"
        extra["budgets"]["readbacks"].append(row)
        with self.assertRaisesRegex(M.Refusal, "unknown budget"):
            M.validate_preflight(extra, self.now)

        fractional = receipt(self.now)
        fractional["pool"]["requested_cpu"] = 8.5
        with self.assertRaisesRegex(M.Refusal, "exactly 8 vCPU"):
            M.validate_preflight(fractional, self.now)

    def test_request_grid_is_frozen(self) -> None:
        revision = "b" * 40
        requests = M.build_requests(
            {"image_uri": "104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:" + "c" * 64},
            revision,
            "d" * 64,
            "e" * 64,
            "f" * 64,
        )
        self.assertEqual({k: v["job"]["seconds"] for k, v in requests.items()}, M.SPLITS)
        self.assertEqual([requests[x]["job"]["cpu"] for x in M.SPLITS], [4, 4, 4, 4])
        self.assertTrue(all(v["job"]["retries"] == 0 for v in requests.values()))
        self.assertTrue(all(v["job"]["isolation_profile"] == "standard" for v in requests.values()))
        self.assertEqual(requests["top"]["job"]["command"][:2], ["/usr/bin/env", "--chdir=/opt/crypto"])
        self.assertEqual(requests["top"]["job"]["command"][-1], "top")


if __name__ == "__main__":
    unittest.main()
