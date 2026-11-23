#!/usr/bin/env python3
"""Independently reparse N83 U/D18 archives using full-orbit point digests."""

from __future__ import annotations

import argparse
from collections import Counter
from decimal import Decimal, localcontext
from math import comb
import gzip
import hashlib
import json
from pathlib import Path
import signal
import time


ORDER = 2417851639230796216685689
K = 16627
N = 83
PINNED = {
    "U": ("2457a977ea9c05a11cc60878bdb3b29f6d905dc8db9aa26e52913a702d308369",
          "public_x_hash", 2026100801),
    "D": ("347d465bef217ae5f76e9b664710440a735a26211bf3957c4be4bcb4b46345a7",
          "direct_subgroup_x_d18", 0),
}


def check(condition: bool, reason: str) -> None:
    if not condition:
        raise ValueError(reason)


def file_hash(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        while chunk := f.read(1 << 20):
            h.update(chunk)
    return h.hexdigest()


def replay_archive(path: Path, arm: str, k: int = K,
                   expected_sha256: str | None = None) -> tuple[dict[bytes, tuple[int, tuple[int, int]]], dict]:
    expected_hash, policy, seed = PINNED[arm]
    check(file_hash(path) == (expected_sha256 or expected_hash), f"{arm} archive SHA-256 differs")
    keyed: dict[bytes, tuple[int, tuple[int, int]]] = {}
    with gzip.open(path, "rb") as f:
        header = json.loads(f.readline())
        check(header["schema"] == "n83.factor-base/v1"
              and header["curve_slug"] == "icv1-f2m83-tm6151469093347-debefd74"
              and header["modulus_low_terms"] == [0, 1, 2, 45]
              and header["curve_a"] == 0 and int(header["curve_b"]) == 1
              and int(header["cofactor"]) == 4
              and int(header["subgroup_order"]) == ORDER
              and header["closure"] == "signed_frobenius"
              and header["policy"] == policy and header["seed"] == seed
              and header["orbit_columns"] == k
              and header["point_count"] == k * 166,
              f"{arm} archive header differs")
        powers = [pow(int(header["frobenius_lambda"]), i, ORDER) for i in range(N)]
        for col in range(k):
            orbit: list[tuple[int, int]] = []
            labels: set[tuple[int, bool]] = set()
            xs: Counter[int] = Counter()
            for _ in range(166):
                line = f.readline()
                check(bool(line), f"{arm} archive truncated")
                row = json.loads(line)
                phase, neg = row["phase"], row["negative"]
                check(row["column"] == col and type(phase) is int and 0 <= phase < N
                      and type(neg) is bool, f"{arm} malformed label")
                check((phase, neg) not in labels, f"{arm} repeated phase/sign")
                labels.add((phase, neg))
                check(int(row["coefficient"]) == ((-powers[phase] if neg else powers[phase]) % ORDER),
                      f"{arm} coefficient differs")
                point = tuple(int(value) for value in row["point"])
                check(len(point) == 2 and all(0 <= v < 1 << N for v in point),
                      f"{arm} malformed point")
                orbit.append(point)
                xs[point[0]] += 1
            check(len(labels) == 166 and len(set(orbit)) == 166
                  and len(xs) == N and set(xs.values()) == {2},
                  f"{arm} incomplete orbit")
            h = hashlib.sha256()
            for x, y in sorted(orbit):
                h.update(x.to_bytes(11, "big"))
                h.update(y.to_bytes(11, "big"))
            digest = h.digest()
            check(digest not in keyed, f"{arm} repeated orbit digest")
            keyed[digest] = (col, min(orbit))
        check(not f.readline(), f"{arm} archive has trailing rows")
    check(len(keyed) == k, f"{arm} orbit count differs")
    return keyed, header



def multiset_count(size: int, degree: int) -> int:
    return 1 if degree == 0 else (comb(size + degree - 1, degree) if size else 0)


def support_count(u_only: int, d_only: int, shared: int, u_picks: int, d_picks: int) -> int:
    total = 0
    degree = u_picks + d_picks
    for u_used in range(degree + 1):
        for d_used in range(degree - u_used + 1):
            common_used = degree - u_used - d_used
            if u_used <= u_picks <= u_used + common_used and d_used <= d_picks <= d_used + common_used:
                total += (multiset_count(u_only, u_used) * multiset_count(d_only, d_used)
                          * multiset_count(shared, common_used))
    return total


def formatted_ratio(numerator: int) -> str:
    with localcontext() as ctx:
        ctx.prec = 25
        return format(Decimal(numerator) / Decimal(ORDER), ".12g")


def audit(u_path: Path, d_path: Path, receipt_path: Path) -> dict:
    started = time.monotonic()
    receipt = json.loads(receipt_path.read_text())
    check(receipt["schema"] == "n83.u-d18-exact-orbit-overlap/v1"
          and receipt["status"] == "PASS_PRODUCER_PENDING_INDEPENDENT_AUDIT"
          and receipt["candidate_id"] is None
          and receipt["selected_best_total_runtime"] is None,
          "producer receipt identity differs")
    check(receipt["producer_sha256"] == file_hash(receipt_path.parent / "overlap.py"),
          "producer source changed")
    u, uh = replay_archive(u_path, "U", k=K)
    d, dh = replay_archive(d_path, "D", k=K)
    check(all(uh[key] == dh[key] for key in
              ("generator", "field_encoding", "frobenius_lambda", "curve_slug")),
          "archive curve identity differs")
    shared = set(u) & set(d)
    count = len(shared)
    checks = {"shared_orbits": count, "u_only_orbits": K - count,
              "d_only_orbits": K - count, "union_orbits": 2 * K - count,
              "shared_points": 166 * count, "u_only_points": 166 * (K - count),
              "d_only_points": 166 * (K - count),
              "union_points": 166 * (2 * K - count)}
    for name, value in checks.items():
        check(receipt[name] == value, f"forged producer {name}")
    check(receipt["input_compressed_sha256"] == {"U": file_hash(u_path), "D": file_hash(d_path)},
          "producer input hashes differ")
    witnesses = sorted((u[key][1], u[key][0], d[key][0]) for key in shared)[:16]
    wanted = [{"u_column": ui, "d_column": di,
               "canonical_point": [str(point[0]), str(point[1])]}
              for point, ui, di in witnesses]
    check(receipt["shared_witnesses_first_16"] == wanted,
          "producer shared-orbit witnesses differ")
    point_keys = sorted(u[key][1] for key in shared)
    canonical_bytes = json.dumps([[str(x), str(y)] for x, y in point_keys],
                                 separators=(",", ":")).encode()
    check(receipt["shared_canonical_point_sha256"] == hashlib.sha256(canonical_bytes).hexdigest(),
          "producer shared-orbit digest differs")
    for a, b in ((2, 2), (3, 2), (2, 3), (3, 3), (4, 2), (2, 4)):
        support = support_count(166 * (K - count), 166 * (K - count), 166 * count, a, b)
        observed = receipt["mixed_support"][f"{a}+{b}"]
        check(observed == {"distinct_point_multisets": str(support),
                           "support_over_subgroup_order": formatted_ratio(support),
                           "uniform_target_union_bound": formatted_ratio(min(support, ORDER))},
              f"forged mixed support {a}+{b}")
    for m in (4, 5, 6):
        check(receipt["same_base_support"][str(m)] == str(multiset_count(166 * K, m)),
              f"forged same-base support m={m}")
    return {"schema": "n83.u-d18-overlap-independent-audit/v1",
            "status": "PASS", "producer_receipt_sha256": file_hash(receipt_path),
            "input_compressed_sha256": receipt["input_compressed_sha256"],
            "shared_orbits_by_full_point_digest": count,
            "u_only_orbits": K - count, "d_only_orbits": K - count,
            "first_16_witnesses": wanted,
            "audit_source_sha256": file_hash(Path(__file__)),
            "wall_s": time.monotonic() - started,
            "candidate_id": None, "selected_best_total_runtime": None}


def deadline_expired(_signum: int, _frame: object) -> None:
    raise TimeoutError("1200-second audit cap")


def main() -> None:
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--u", type=Path, required=True)
    p.add_argument("--d", type=Path, required=True)
    p.add_argument("--receipt", type=Path, required=True)
    p.add_argument("--output", type=Path, required=True)
    args = p.parse_args()
    check(not args.output.exists(), "audit output already exists")
    signal.signal(signal.SIGALRM, deadline_expired)
    signal.alarm(1200)
    try:
        result = audit(args.u, args.d, args.receipt)
        args.output.write_text(json.dumps(result, sort_keys=True, indent=2) + "\n")
    except Exception as exc:
        args.output.with_suffix(".error.json").write_text(
            json.dumps({"status": "INCOMPLETE_ERROR", "error": str(exc)}, indent=2) + "\n")
        raise
    print(json.dumps({"status": result["status"],
                      "shared_orbits": result["shared_orbits_by_full_point_digest"],
                      "audit_sha256": file_hash(args.output)}))


if __name__ == "__main__":
    main()
