#!/usr/bin/env python3
"""Independently terminate one identified RunPod Pod before its spending cap."""

import argparse
from datetime import datetime, timedelta, timezone
from decimal import Decimal, InvalidOperation
import getpass
import json
from pathlib import Path
import signal
import time
import urllib.error
import urllib.request


BASE = "https://rest.runpod.io/v1"
MAX_RATE = Decimal("0.35")
MAX_SECONDS = 12 * 3600
STOP_MARGIN = timedelta(minutes=10)
STOP_REQUESTED = False


def utc_now():
    return datetime.now(timezone.utc)


def parse_time(value):
    parsed = datetime.fromisoformat(value.replace("Z", "+00:00"))
    if parsed.tzinfo is None:
        raise ValueError("allocation time lacks timezone")
    return parsed.astimezone(timezone.utc)


def gate_allocation(data):
    if data.get("schema") != "n83.maxk-runpod-allocation/v1":
        raise ValueError("allocation schema differs")
    pod_id, name = data.get("pod_id"), data.get("pod_name")
    if (not isinstance(pod_id, str) or not pod_id.isalnum() or
            not isinstance(name, str) or not name.startswith("n83-maxk-")):
        raise ValueError("Pod identity differs")
    created = parse_time(data["created_utc"])
    rate = Decimal(str(data["cost_per_hr_usd"]))
    cap = Decimal(str(data["approved_cap_usd"]))
    lifetime = data["max_lifetime_seconds"]
    if (not rate.is_finite() or not Decimal("0") < rate <= MAX_RATE or
            not cap.is_finite() or cap <= 0 or cap > Decimal("5") or
            not isinstance(lifetime, int) or isinstance(lifetime, bool) or
            not 0 < lifetime <= MAX_SECONDS or
            rate * Decimal(lifetime) / Decimal(3600) > cap):
        raise ValueError("rate, cap, or provider lifetime exceeds approval")
    spend_seconds = int((cap / rate * Decimal(3600)).to_integral_value(rounding="ROUND_FLOOR"))
    deadline = created + timedelta(seconds=min(lifetime, spend_seconds)) - STOP_MARGIN
    return pod_id, name, deadline


def request(key, method, path):
    req = urllib.request.Request(BASE + path, method=method, headers={
        "Authorization": "Bearer " + key,
        "User-Agent": "cryptanalysis-n83-maxk-guard/1.0",
    })
    try:
        with urllib.request.urlopen(req, timeout=30) as response:
            payload = response.read()
            return response.status, json.loads(payload) if payload else None
    except urllib.error.HTTPError as error:
        return error.code, None


def write_receipt(path, event):
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("a") as output:
        output.write(json.dumps(event, sort_keys=True) + "\n")
        output.flush()


def observe(key, pod_id, name):
    code, pod = request(key, "GET", "/pods/" + pod_id)
    if code == 404:
        listing_code, listing = request(key, "GET", "/pods")
        if listing_code != 200 or not isinstance(listing, list):
            return "API_ERROR", {"pod_http": code, "inventory_http": listing_code}
        if any(item.get("id") == pod_id or item.get("name") == name
               for item in listing):
            return "POD_PRESENT", {"pod_http": code, "inventory_http": listing_code}
        return "POD_ABSENT", {"pod_http": code, "inventory_http": listing_code}
    if code != 200 or not isinstance(pod, dict):
        return "API_ERROR", {"pod_http": code}
    if pod.get("id") != pod_id or pod.get("name") != name:
        return "IDENTITY_MISMATCH", {"pod_http": code}
    return "POD_PRESENT", {
        "pod_http": code,
        "desired_status": pod.get("desiredStatus"),
        "machine_id": pod.get("machineId"),
        "cost_per_hr": pod.get("costPerHr"),
    }


def on_signal(_signum, _frame):
    global STOP_REQUESTED
    STOP_REQUESTED = True


def run(allocation, receipt_path, key, interval):
    pod_id, name, deadline = gate_allocation(allocation)
    signal.signal(signal.SIGINT, on_signal)
    signal.signal(signal.SIGTERM, on_signal)
    deleting = False
    while True:
        now = utc_now()
        try:
            status, detail = observe(key, pod_id, name)
        except (TimeoutError, OSError, ValueError) as error:
            status, detail = "API_ERROR", {"error_type": type(error).__name__}
        if status == "POD_ABSENT":
            write_receipt(receipt_path, {"at_utc": now.isoformat(),
                                         "status": "VERIFIED_POD_ABSENT",
                                         "pod_id": pod_id, **detail})
            return 0
        if status == "IDENTITY_MISMATCH":
            write_receipt(receipt_path, {"at_utc": now.isoformat(),
                                         "status": status, "pod_id": pod_id, **detail})
            return 2
        live_rate = detail.get("cost_per_hr") if status == "POD_PRESENT" else None
        try:
            over_rate = (status == "POD_PRESENT" and
                         (live_rate is None or
                          not Decimal(str(live_rate)).is_finite() or
                          Decimal(str(live_rate)) >
                          Decimal(str(allocation["cost_per_hr_usd"]))))
        except (InvalidOperation, ValueError, TypeError):
            over_rate = True
        if now >= deadline or STOP_REQUESTED or over_rate:
            deleting = True
        if deleting:
            try:
                delete_code, _ = request(key, "DELETE", "/pods/" + pod_id)
            except (TimeoutError, OSError, ValueError):
                delete_code = "API_ERROR"
            write_receipt(receipt_path, {"at_utc": now.isoformat(),
                                         "status": "DELETE_REQUESTED",
                                         "pod_id": pod_id,
                                         "delete_http": delete_code,
                                         "observed": status})
        else:
            write_receipt(receipt_path, {"at_utc": now.isoformat(),
                                         "status": status, "pod_id": pod_id,
                                         "deadline_utc": deadline.isoformat(),
                                         **detail})
        time.sleep(interval)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--allocation", type=Path, required=True)
    parser.add_argument("--receipt", type=Path, required=True)
    parser.add_argument("--poll-seconds", type=int, default=30)
    args = parser.parse_args()
    if not 5 <= args.poll_seconds <= 60:
        parser.error("poll interval must be 5..60 seconds")
    allocation = json.loads(args.allocation.read_text())
    gate_allocation(allocation)
    if args.receipt.exists():
        parser.error("guard receipt already exists")
    key = getpass.getpass("RunPod key for independent guard: ")
    if not key:
        parser.error("empty API key")
    return run(allocation, args.receipt, key, args.poll_seconds)


if __name__ == "__main__":
    raise SystemExit(main())
