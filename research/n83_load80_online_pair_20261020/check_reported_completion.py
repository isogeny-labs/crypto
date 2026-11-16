#!/usr/bin/env python3
"""Check frozen producer completion; this is not arithmetic replay or admission."""

import argparse
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
PROTOCOL_SHA256 = "a913da83dd35b558f96e6b8b70a62d1eb606d798e1fde51da421db20092d2091"


def read(path):
    return json.loads(path.read_text())


def check(root, protocol_path=HERE / "protocol.json"):
    try:
        blob = protocol_path.read_bytes()
        if hashlib.sha256(blob).hexdigest() != PROTOCOL_SHA256:
            raise ValueError("frozen protocol hash differs")
        protocol = json.loads(blob)
        events = protocol["events"]
        source = read(root / "source.json")
        summary = read(root / "summary.json")
        if (source.get("protocol_sha256") != PROTOCOL_SHA256
                or source.get("protocol") != protocol):
            raise ValueError("producer source does not bind the frozen protocol")
        if (summary.get("schema") != "n83.load80-online-pair-summary/v1"
                or summary.get("status") != "COMPLETE_REPORTED_OUTPUT"
                or summary.get("expected_events") != len(events)
                or not isinstance(summary.get("events"), list)
                or len(summary["events"]) != len(events)):
            raise ValueError("schedule is incomplete")
        if (root / "producer_stop.json").exists():
            raise ValueError("producer recorded a stop")
        for expected, row in zip(events, summary["events"]):
            if (row.get("event") != expected
                    or row.get("process") != "PASS_PROCESS"
                    or row.get("resource") != "PASS"
                    or row.get("reported_verification") != "PASS_REPORTED_OUTPUT"):
                raise ValueError(f"failed or out-of-order event: {expected['name']}")
            folder = root / "runs" / expected["name"]
            usage = read(folder / "usage.json")
            reported = read(folder / "reported_verification.json")
            if (usage.get("name") != expected["name"]
                    or usage.get("status") != "PASS_PROCESS"
                    or type(usage.get("exit_code")) is not int
                    or usage["exit_code"] != 0
                    or usage.get("resource_status") != "PASS"
                    or reported != {"event": expected, "status": "PASS_REPORTED_OUTPUT"}):
                raise ValueError(f"event receipts disagree: {expected['name']}")
    except (OSError, ValueError, KeyError, TypeError, AttributeError) as error:
        return {"status": "BOUNDED_UNKNOWN", "reason": str(error),
                "independent_arithmetic_replay": False}
    return {"status": "PASS_REPORTED_COMPLETION", "events": len(events),
            "independent_arithmetic_replay": False}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("results", type=Path)
    args = parser.parse_args()
    result = check(args.results)
    print(json.dumps(result, sort_keys=True))
    return 0 if result["status"] == "PASS_REPORTED_COMPLETION" else 2


if __name__ == "__main__":
    raise SystemExit(main())
