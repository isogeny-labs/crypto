#!/usr/bin/env python3
"""Append the exact-size shifted-interval selection and source-pinned preflights."""

import argparse
import hashlib
import json
import os
import subprocess
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
EXPECTED_BINARY = "1ae5170f9b3362f75a636909e1f5d01fb0ead0fd4886627d6012e2b9de9d8d08"
P, A, B, WIDTH, BASE_SIZE = 16187, 1, 12, 32, 21
INSTANCE = "icv1-fp14-tm43-158b6914"
ORDER = 16231
SEED = 20261301
SPECS = {
    "interval": "prime-interval:t=5,lo=118",
    "candidate": "prime-isogeny:t=5,seed=5,aux=ordinary",
}


def append(path, row):
    with path.open("a", encoding="utf-8") as stream:
        stream.write(json.dumps(row, sort_keys=True) + "\n")
        stream.flush()
        os.fsync(stream.fileno())


def scan():
    carriers = [int((rhs := (x * x * x + A * x + B) % P) == 0 or
                    pow(rhs, (P - 1) // 2, P) == 1) for x in range(P)]
    count = sum(carriers[:WIDTH])
    matches = []
    for lo in range(P):
        if count == BASE_SIZE:
            matches.append(lo)
        count += carriers[(lo + WIDTH) % P] - carriers[lo]
    return matches


def command(binary, spec):
    return [str(binary), "bench", "--bits", "14", "--family", "generic",
            "--seed", str(SEED), "--repeats", "1", "--targets", "walk",
            "--factor-base", spec, "--oracle", "subtract", "--max-trials", "1",
            "--rho-runs", "0", "--json"]


def preflight(binary, spec):
    argv = command(binary, spec)
    start = time.monotonic()
    try:
        proc = subprocess.run(argv, capture_output=True, text=True, timeout=15)
        code, stdout, stderr = proc.returncode, proc.stdout, proc.stderr
        timed_out = False
    except subprocess.TimeoutExpired as exc:
        code = 124
        stdout = exc.stdout.decode(errors="replace") if isinstance(exc.stdout, bytes) else (exc.stdout or "")
        stderr = exc.stderr.decode(errors="replace") if isinstance(exc.stderr, bytes) else (exc.stderr or "")
        timed_out = True
    try:
        result = json.loads(stdout)
    except json.JSONDecodeError:
        result = {}
    rows = result.get("rows") or []
    row = rows[0] if rows else {}
    return {"command": argv, "exit_code": code, "timeout": timed_out,
            "elapsed_s": time.monotonic() - start, "stdout": stdout, "stderr": stderr,
            "instance": result.get("instance"), "skipped": result.get("configurations_skipped"),
            "abscissae": (row.get("factor_base") or {}).get("abscissae"),
            "order": row.get("r"), "row_count": len(rows)}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bin", type=Path, required=True)
    parser.add_argument("--store", type=Path, default=HERE / "equal21_preflight.jsonl")
    args = parser.parse_args()
    digest = hashlib.sha256(args.bin.read_bytes()).hexdigest()
    if digest != EXPECTED_BINARY:
        parser.error(f"binary SHA-256 {digest} differs from frozen {EXPECTED_BINARY}")
    rows = {}
    if args.store.exists():
        for line in args.store.read_text(encoding="utf-8").splitlines():
            item = json.loads(line)
            if item["id"] in rows:
                raise ValueError(f"duplicate record {item['id']}")
            rows[item["id"]] = item
    if "header" not in rows:
        item = {"id": "header", "schema": "pkm.equal21-preflight/v1",
                "protocol": "PROTOCOL_F4_DEGREE32_EQUAL21_14BIT_20261125.md",
                "binary_sha256": digest, "curve": [P, A, B, ORDER], "seed": SEED}
        append(args.store, item)
        rows[item["id"]] = item
    if rows["header"]["binary_sha256"] != digest:
        raise ValueError("store binary differs from this executable")
    if "scan" not in rows:
        matches = scan()
        item = {"id": "scan", "width": WIDTH, "wanted_abscissae": BASE_SIZE,
                "searched_shifts": P, "match_count": len(matches),
                "first_match": matches[0] if matches else None,
                "match_sha256": hashlib.sha256(json.dumps(matches).encode()).hexdigest()}
        append(args.store, item)
        rows[item["id"]] = item
    if rows["scan"]["first_match"] != 118:
        raise ValueError("frozen first shift does not reproduce")
    for name, spec in SPECS.items():
        key = f"preflight:{name}"
        if key not in rows:
            item = {"id": key, "spec": spec, **preflight(args.bin, spec)}
            append(args.store, item)
            rows[key] = item
    passed = all(rows[f"preflight:{name}"]["instance"] == INSTANCE and
                 rows[f"preflight:{name}"]["abscissae"] == BASE_SIZE and
                 rows[f"preflight:{name}"]["order"] == ORDER and
                 rows[f"preflight:{name}"]["row_count"] == 1 and
                 rows[f"preflight:{name}"]["skipped"] == [] and
                 rows[f"preflight:{name}"]["exit_code"] in (0, 1)
                 for name in SPECS)
    if "gate" not in rows:
        append(args.store, {"id": "gate", "passed": passed,
                            "arms": list(SPECS), "matched_instance": INSTANCE,
                            "matched_abscissae": BASE_SIZE})
    print(json.dumps({"gate": passed, "first_match": rows["scan"]["first_match"],
                      "match_count": rows["scan"]["match_count"]}, sort_keys=True))
    if not passed:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
