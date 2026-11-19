#!/usr/bin/env python3
"""CT232: independent stdlib-only adjudication of frozen native CT231 output.

This audits a single GitHub-hosted producer invocation; it does not rerun
Cantor arithmetic or constitute signed independent PCC. [pcc:unverified]
Reproduce:
 python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct232_single_lp_cost_audit.py --self-test
"""
from __future__ import annotations
import copy
import json
import sys
from pathlib import Path

DATA = Path(__file__).with_name("ct232_single_lp_native_panel.jsonl")
SOURCE = "4c5ef4e4d65d62b93501b0b3a1417001aab0f14f"
RUN = "44509580066"
JOB = "135509986267"
KEYS = {
    "basis", "rank", "candidates", "offsets", "full", "single_lp",
    "multi_lp_discard", "pair_rows", "distinct_lp", "target_recovered",
    "scalar", "target_candidates", "setup_ns", "relations_ns", "la_ns",
    "verification_ns", "target_ns", "total_ns", "bsgs_ns", "cantor_adds",
    "target_adds", "model", "pcc",
}
# Counts transcribed from the original, un-echoed four CT231_PANEL_JSON lines.
EXPECTED = {
    24: (17, 6540, 60, 3, 46, 204, 24, 22, False, 0, 6632, 0),
    36: (36, 2615, 24, 16, 50, 38, 34, 16, True, 3, 2639, 4),
    48: (48, 1569, 15, 40, 21, 6, 16, 5, True, 3, 1584, 4),
    54: (54, 1421, 14, 62, 0, 0, 0, 0, True, 3, 1435, 4),
}


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def parse(payload: str) -> tuple[list[str], list[dict]]:
    comments, records = [], []
    for line in payload.splitlines():
        if line.startswith("#"):
            comments.append(line)
        elif line.strip():
            record = json.loads(line)
            require(type(record) is dict and set(record) == KEYS, "schema mismatch")
            records.append(record)
    require(len(records) == 4, "four and only four native samples required")
    require([r["basis"] for r in records] == [24, 36, 48, 54], "wrong arm order")
    comments_text = "\n".join(comments)
    for provenance in (RUN, JOB, SOURCE, "NOT four extra independent samples"):
        require(provenance in comments_text, "missing exact source provenance: " + provenance)
    return comments, records


def audit(payload: str) -> dict:
    _, records = parse(payload)
    for r in records:
        b = r["basis"]
        got = tuple(r[k] for k in (
            "rank", "candidates", "offsets", "full", "single_lp",
            "multi_lp_discard", "pair_rows", "distinct_lp", "target_recovered",
            "target_candidates", "cantor_adds", "target_adds",
        ))
        require(got == EXPECTED[b], "native counters changed at basis " + str(b))
        require(r["model"] == "F101_signed_one_lp", "unregistered model")
        require(r["pcc"] == "unverified", "unauthorized PCC promotion")
        require(type(r["target_recovered"]) is bool, "nonboolean target status")
        for field in KEYS - {"target_recovered", "scalar", "model", "pcc"}:
            require(type(r[field]) is int and r[field] >= 0, "invalid numeric field " + field)
        require(r["rank"] <= b, "rank exceeds columns")
        require(r["candidates"] <= 6540 and r["offsets"] <= 60, "budget overrun")
        require(r["full"] + r["single_lp"] + r["multi_lp_discard"] <= r["candidates"],
                "support classification inconsistent")
        require(r["pair_rows"] == r["single_lp"] - r["distinct_lp"],
                "first-occurrence pair count inconsistent")
        require(r["distinct_lp"] <= 54 - b, "too many excluded symbols")
        require((r["offsets"] - 1) * 109 < r["candidates"] <= r["offsets"] * 109,
                "offset/candidate mismatch")
        require(r["cantor_adds"] >= r["candidates"], "native composition undercount")
        require(r["bsgs_ns"] > 0 and r["relations_ns"] > 0, "missing baseline or relations")
        phases = sum(r[k] for k in (
            "setup_ns", "relations_ns", "la_ns", "verification_ns", "target_ns",
        ))
        require(0 <= r["total_ns"] - phases < 1000, "missing cold-cost phase")
        require(r["target_recovered"] == (r["rank"] == b), "rank/solve inconsistency")
        if r["target_recovered"]:
            require(r["scalar"] == 37, "incorrect source scalar")
            require(r["target_candidates"] > 0 and r["target_adds"] > 0,
                    "missing target cost")
            require(r["total_ns"] > r["bsgs_ns"], "unsupported speedup record")
        else:
            require(r["scalar"] is None and r["target_candidates"] == 0 and
                    r["target_adds"] == 0, "failed arm assigned a completed solve")
    baseline = records[-1]
    require((baseline["rank"], baseline["full"], baseline["candidates"]) ==
            (54, 62, 1421), "CT226 full-base control drift")
    # The tested 36/48 single-LP arms are completed but remain slower than
    # the 54-variable arm in this ONE unisolated CI invocation.
    require(all(r["total_ns"] > baseline["total_ns"] for r in records[1:3]),
            "performance ordering differs from original record")
    return {
        "run": int(RUN), "job": int(JOB), "record_count": 4,
        "failed_bases": [24],
        "complete_bases": [36, 48, 54],
        "36_vs_54_slowdown": records[1]["total_ns"] / baseline["total_ns"],
        "48_vs_54_slowdown": records[2]["total_ns"] / baseline["total_ns"],
        "36_vs_bsgs_slowdown": records[1]["total_ns"] / records[1]["bsgs_ns"],
        "48_vs_bsgs_slowdown": records[2]["total_ns"] / records[2]["bsgs_ns"],
        "54_vs_bsgs_slowdown": baseline["total_ns"] / baseline["bsgs_ns"],
        "independent_pcc": False,
    }


def audit_join_equations() -> None:
    # Independent F109 identity with ALL four LP coefficient sign pairs.
    mod, logs, lp = 109, (11, 19, 23), 43
    a, b = (2, -5, 4), (7, 0, -3)
    for s0 in (-1, 1):
        for s1 in (-1, 1):
            t0 = (sum(x * y for x, y in zip(a, logs)) + s0 * lp) % mod
            t1 = (sum(x * y for x, y in zip(b, logs)) + s1 * lp) % mod
            joined = [(s0 * y - s1 * x) % mod for x, y in zip(a, b)]
            rhs = (s0 * t1 - s1 * t0) % mod
            require(sum(x * y for x, y in zip(joined, logs)) % mod == rhs,
                    "signed LP cancellation failed")
            recovered_lp = (s0 * (t0 - sum(x * y for x, y in zip(a, logs)))) % mod
            require(recovered_lp == lp, "signed LP recovery failed")


def self_test(payload: str) -> None:
    audit_join_equations()
    audit(payload)
    comments, original = parse(payload)
    def serialize(rows: list[dict], notes: list[str] = comments) -> str:
        return "\n".join(notes + [json.dumps(x, separators=(",", ":")) for x in rows]) + "\n"
    mutations = []
    def altered(basis: int, field: str, value: object) -> str:
        rows = copy.deepcopy(original)
        next(r for r in rows if r["basis"] == basis)[field] = value
        return serialize(rows)
    mutations += [
        serialize(original[:-1]), serialize(original + [original[-1]]),
        altered(24, "rank", 24), altered(36, "pair_rows", 33),
        altered(36, "scalar", 38), altered(48, "total_ns", 1200),
        altered(54, "bsgs_ns", 0), altered(24, "target_recovered", True),
        altered(36, "pcc", "verified"), altered(48, "model", "oracle"),
        payload.replace(SOURCE, "0" * 40, 1),
    ]
    for i, bad in enumerate(mutations):
        try:
            audit(bad)
        except (ValueError, KeyError, TypeError, json.JSONDecodeError):
            continue
        raise ValueError(f"mutation {i} passed audit")
    print(f"CT232_SELFTEST_PASS four_sign_pairs {len(mutations)}_mutations")


if __name__ == "__main__":
    raw = DATA.read_text(encoding="utf8")
    if sys.argv[1:] == ["--self-test"]:
        self_test(raw)
    elif not sys.argv[1:]:
        print(json.dumps(audit(raw), indent=2, sort_keys=True))
    else:
        raise SystemExit("usage: ct232_single_lp_cost_audit.py [--self-test]")
