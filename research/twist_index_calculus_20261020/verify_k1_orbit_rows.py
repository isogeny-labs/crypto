#!/usr/bin/env python3
"""Check that every selected twist orbit of folded relation rows has rank one."""
from __future__ import annotations

import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent


def row(terms: list[list[int]], columns: list[int], coefficients: list[int],
        anchor_log: int, a: int, b: int, n: int, folded_columns: int) -> tuple[list[int], int]:
    sums = [0] * folded_columns
    for index, sign in terms:
        col = columns[index]
        sums[col] = (sums[col] + sign * coefficients[index]) % n
    return sums[1:] + [(-b) % n], (a - anchor_log * sums[0]) % n


def check(p: int, seed: int) -> dict:
    source = ROOT / f"quartic_twist_k1_scale_p{p}_seed{seed}_v1.json"
    audit_source = ROOT / f"quartic_twist_k1_scale_p{p}_seed{seed}_audit_v1.json"
    rec = json.loads(source.read_text())
    audit = json.loads(audit_source.read_text())
    assert audit["status"] in ("pass", "partial")
    assert audit["base_logs_verified"] == rec["base_size"]
    n = rec["n"]
    frob = rec["lambda_frob"]
    cubic = rec["lambda_cubic"]
    actions = {(pow(frob, i, n) * pow(cubic, j, n)) % n for i in range(8) for j in range(3)}
    assert len(actions) == 24
    assert pow(frob, 4, n) == n - 1 and pow(frob, 8, n) == 1
    assert pow(cubic, 3, n) == 1 and cubic != 1
    logs = rec["physical_factor_logs"]
    columns = rec["orbit_column"]
    coefficients = rec["orbit_coefficient"]
    folded = rec["folded_columns"]
    anchor = rec["anchor_log"]
    assert len(logs) == len(columns) == len(coefficients) == rec["base_size"]
    by_log: dict[int, tuple[int, int]] = {}
    for index, log in enumerate(logs):
        assert log not in by_log and (-log) % n not in by_log
        by_log[log] = (index, 1)
        by_log[(-log) % n] = (index, -1)
    closure_checks = 0
    for lam in actions:
        for log in logs:
            assert (lam * log) % n in by_log
            closure_checks += 1
    checks = 0
    for selected in rec["selected_relations"]:
        terms = selected["terms"]
        a, b = selected["a"], selected["b"]
        original, rhs = row(terms, columns, coefficients, anchor, a, b, n, folded)
        assert original == selected["row"] and rhs == selected["rhs"]
        assert sum(sign * logs[index] for index, sign in terms) % n == (a + 271 * b) % n
        assert any(original)
        for lam in actions:
            transformed = [list(by_log[(lam * sign * logs[index]) % n]) for index, sign in terms]
            transformed.sort()
            got, got_rhs = row(transformed, columns, coefficients, anchor,
                               lam * a % n, lam * b % n, n, folded)
            assert got == [lam * value % n for value in original]
            assert got_rhs == lam * rhs % n
            checks += 1
    return {"p": p, "seed": seed, "selected_relations": len(rec["selected_relations"]),
            "actions": len(actions), "factor_action_closure_checks": closure_checks,
            "rank_one_row_checks": checks,
            "raw_sha256": hashlib.sha256(source.read_bytes()).hexdigest(),
            "audit_sha256": hashlib.sha256(audit_source.read_bytes()).hexdigest()}


if __name__ == "__main__":
    runs = [check(p, seed) for p in (73, 97) for seed in range(3)]
    out = {"status": "pass", "runs": runs,
           "selected_relations": sum(r["selected_relations"] for r in runs),
           "rank_one_row_checks": sum(r["rank_one_row_checks"] for r in runs),
           "factor_action_closure_checks": sum(r["factor_action_closure_checks"] for r in runs)}
    (ROOT / "quartic_twist_k1_orbit_row_audit_v1.json").write_text(
        json.dumps(out, sort_keys=True, indent=2) + "\n")
    print(json.dumps({key: value for key, value in out.items() if key != "runs"}, sort_keys=True))
