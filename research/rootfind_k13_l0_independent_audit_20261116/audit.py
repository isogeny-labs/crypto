#!/usr/bin/env python3
"""Post-hoc arithmetic and paired-record audit of the frozen K13 L0 session.

This deliberately uses only Python integer arithmetic and the archived JSONL.
It does not call the Rust solver, its replay implementation, or its cost model.
"""

from __future__ import annotations

import argparse
import hashlib
import json
from collections import defaultdict
from copy import deepcopy
from pathlib import Path


RECORDS_SHA256 = "dc16c97ae21a250cc1d47237231d9642960c100dc3fdb68f4dd210af171bff44"
RECORDS_BYTES = 453126
SESSION = "ECBS1hc322a24b6610"
BINARY_SHA256 = "dba933709f466966dc13f069b9bf0e91def033284f8b1cb779b25c8c9b3448c1"
CURVE_ID = "ICV1:f2m-13-752c6463:181:8012:0x1:-7:unk:r:515ee5696294"
FB_SHA256 = "cc3c0ee034174a528849ca7efd95d70459404f6cc1e9361f0b4da7f6c3da2ae3"
POINTS_SHA256 = "9dfd936503b02863275833a548f4ae1e13559834b1aad6fe1f77f6c2c9789648"
ARMS = (
    "rho-strong", "matrix-f4", "matrix-f5", "inherited-f4", "f6-ic",
    "rootfind-m3", "batch-m3", "batch-m3-aa",
)
BASELINES = tuple(arm for arm in ARMS if arm != "batch-m3")
N = 13
R = 2003
O = None


def poly_rem(a: int, b: int) -> int:
    while a and a.bit_length() >= b.bit_length():
        a ^= b << (a.bit_length() - b.bit_length())
    return a


def poly_gcd(a: int, b: int) -> int:
    while b:
        a, b = b, poly_rem(a, b)
    return a


def field_mul(a: int, b: int, modulus: int) -> int:
    out = 0
    while b:
        if b & 1:
            out ^= a
        b >>= 1
        a <<= 1
        if a & (1 << N):
            a ^= modulus
    return out & ((1 << N) - 1)


def field_pow(a: int, exponent: int, modulus: int) -> int:
    out = 1
    while exponent:
        if exponent & 1:
            out = field_mul(out, a, modulus)
        a = field_mul(a, a, modulus)
        exponent >>= 1
    return out


def irreducible_degree_13(modulus: int) -> bool:
    if modulus.bit_length() != N + 1 or not modulus & 1:
        return False
    x = 2
    for i in range(1, N + 1):
        x = field_mul(x, x, modulus)
        if i == 1 and poly_gcd(x ^ 2, modulus) != 1:
            return False
    return x == 2


def selected_modulus() -> int:
    # Independent sparse search: least bitmask among trinomials and
    # pentanomials, matching the declared Koblitz constructor selection law.
    for low in range(1, 1 << N, 2):
        if low.bit_count() <= 4 and irreducible_degree_13((1 << N) | low):
            return (1 << N) | low
    raise ValueError("no sparse irreducible degree-13 polynomial")


def field_inv(a: int, modulus: int) -> int:
    if a == 0:
        raise ValueError("division by zero in binary field")
    return field_pow(a, (1 << N) - 2, modulus)


def on_curve(point: tuple[int, int] | None, modulus: int) -> bool:
    if point is O:
        return True
    x, y = point
    if x >= (1 << N) or y >= (1 << N):
        return False
    left = field_mul(y, y, modulus) ^ field_mul(x, y, modulus)
    right = field_mul(field_mul(x, x, modulus), x, modulus) ^ 1
    return left == right


def add(p: tuple[int, int] | None, q: tuple[int, int] | None, modulus: int):
    if p is O:
        return q
    if q is O:
        return p
    x1, y1 = p
    x2, y2 = q
    if x1 == x2:
        if y1 ^ y2 == x1:
            return O
        if p != q or x1 == 0:
            raise ValueError("invalid equal-abscissa operands")
        slope = x1 ^ field_mul(y1, field_inv(x1, modulus), modulus)
    else:
        slope = field_mul(y1 ^ y2, field_inv(x1 ^ x2, modulus), modulus)
    x3 = field_mul(slope, slope, modulus) ^ slope ^ x1 ^ x2
    y3 = field_mul(slope, x1 ^ x3, modulus) ^ x3 ^ y1
    out = (x3, y3)
    if not on_curve(out, modulus):
        raise ValueError("independent addition left the curve")
    return out


def scalar_mul(p: tuple[int, int], scalar: int, modulus: int):
    result = O
    while scalar:
        if scalar & 1:
            result = add(result, p, modulus)
        p = add(p, p, modulus)
        scalar >>= 1
    return result


def point(raw: list[str]) -> tuple[int, int]:
    if len(raw) != 2:
        raise ValueError("point needs two coordinates")
    return tuple(int(coord, 16) for coord in raw)


def validate_record(row: dict, modulus: int, generator: tuple[int, int]) -> None:
    if row["session_id"] != SESSION or row["binary_sha256"] != BINARY_SHA256:
        raise ValueError("session or binary mismatch")
    if row["arm"] not in ARMS or row["outcome"]["status"] != "verified":
        raise ValueError("arm or outcome mismatch")
    workload = row["workload"]
    curve = workload["curve"]
    if curve["icv1"] != CURVE_ID or curve["r"] != R or point(curve["generator"]) != generator:
        raise ValueError("curve identity mismatch")
    if workload["target_seed"] != 202611160511 or workload["target_index"] not in (0, 1):
        raise ValueError("target law mismatch")
    target = point(workload["target"])
    if not on_curve(target, modulus):
        raise ValueError("target is off curve")
    recovered = int(row["outcome"]["recovered"])
    if recovered >= R or recovered < 0 or scalar_mul(generator, recovered, modulus) != target:
        raise ValueError("recovered scalar does not reproduce target")
    if row["arm"] != "rho-strong":
        if (row["factor_base"]["fb_sha256"] != FB_SHA256
                or row["factor_base"]["points_sha256"] != POINTS_SHA256):
            raise ValueError("factor-base identity mismatch")
    if row["isolation"]["level"] != "L0":
        raise ValueError("unexpected isolation level")
    if not isinstance(row["time"]["process_wall_ns"], int) or row["time"]["process_wall_ns"] <= 0:
        raise ValueError("missing complete-process clock")


def audit(path: Path) -> dict:
    data = path.read_bytes()
    if len(data) != RECORDS_BYTES or hashlib.sha256(data).hexdigest() != RECORDS_SHA256:
        raise ValueError("raw record bytes do not match the archived manifest")
    modulus = selected_modulus()
    generator = (0x12B9, 0x1BC4)
    if modulus != 0x201B or not on_curve(generator, modulus):
        raise ValueError("field or generator mismatch")
    if scalar_mul(generator, R, modulus) is not O:
        raise ValueError("generator order mismatch")
    rows = [json.loads(line) for line in data.splitlines()]
    if len(rows) != 48:
        raise ValueError("unexpected record count")
    keys = set()
    groups: dict[tuple[int, int], dict[str, dict]] = defaultdict(dict)
    targets: dict[int, tuple] = {}
    for row in rows:
        validate_record(row, modulus, generator)
        index = row["workload"]["target_index"]
        identity = (row["workload"]["workload_sha256"], tuple(row["workload"]["target"]))
        if index in targets and targets[index] != identity:
            raise ValueError("target identity changes across arms")
        targets[index] = identity
        key = (index, row["round"], row["arm"])
        if key in keys:
            raise ValueError("duplicate target-round-arm record")
        keys.add(key)
        if not row["warmup"]:
            groups[(index, row["round"])][row["arm"]] = row
    if len(groups) != 4 or any(set(arms) != set(ARMS) for arms in groups.values()):
        raise ValueError("measured target-round panel is incomplete")
    warmups = [row for row in rows if row["warmup"]]
    if len(warmups) != 16:
        raise ValueError("warmup panel is incomplete")
    first = next(row for row in rows if not row["warmup"] and row["arm"] == "batch-m3")
    scalar_control = deepcopy(first)
    scalar_control["outcome"]["recovered"] = str((int(first["outcome"]["recovered"]) + 1) % R)
    clock_control = deepcopy(first)
    clock_control["time"]["process_wall_ns"] = 0
    for corrupt in (scalar_control, clock_control):
        try:
            validate_record(corrupt, modulus, generator)
        except ValueError:
            pass
        else:
            raise ValueError("mutation control was accepted")
    pairs = []
    for (index, round_id), arms in sorted(groups.items()):
        seed = {str(row["algorithm_seed"]) for row in arms.values()}
        if len(seed) != 1:
            raise ValueError("algorithm seed mismatch within matched pair")
        candidate = arms["batch-m3"]["time"]["process_wall_ns"]
        pairs.append({
            "target_index": index, "round": round_id,
            "workload_sha256": arms["batch-m3"]["workload"]["workload_sha256"],
            "algorithm_seed": next(iter(seed)),
            "batch_process_wall_ns": candidate,
            "reference_process_wall_ns": {
                arm: arms[arm]["time"]["process_wall_ns"] for arm in BASELINES
            },
            "batch_over_reference": {
                arm: candidate / arms[arm]["time"]["process_wall_ns"] for arm in BASELINES
            },
        })
    summed = {
        arm: sum(p["batch_process_wall_ns"] for p in pairs)
        / sum(p["reference_process_wall_ns"][arm] for p in pairs)
        for arm in BASELINES
    }
    return {
        "schema": "rootfind-k13-l0-independent-audit/v1",
        "raw_sha256": RECORDS_SHA256, "raw_bytes": RECORDS_BYTES,
        "session_id": SESSION, "binary_sha256": BINARY_SHA256,
        "curve_id": CURVE_ID, "field_polynomial": hex(modulus),
        "generator": [hex(x) for x in generator], "subgroup_order": R,
        "records_checked": len(rows), "measured_records": len(rows) - len(warmups),
        "distinct_targets": len(targets), "matched_pairs": len(pairs),
        "wrong_scalar_control_rejected": True, "zero_clock_control_rejected": True,
        "pairs": pairs, "summed_wall_ratios": summed,
        "isolation": "L0", "status": "arithmetic_and_pairing_checks_passed",
        "claim_boundary": "Post-hoc same-producer L0 raw-record audit; no admitted L2 timing or execution-proof verdict",
    }


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--records", required=True, type=Path)
    args = parser.parse_args()
    print(json.dumps(audit(args.records), indent=2, sort_keys=True))
