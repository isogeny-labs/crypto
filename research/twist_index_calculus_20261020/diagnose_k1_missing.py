#!/usr/bin/env python3
"""Post-run scalar-keyed signed-set differences; never fed to the IC solver."""
from __future__ import annotations

import collections
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent


def exact_set(logs: list[int], line0: int, n: int,
              pairs: dict[int, list[tuple[tuple[int, int], tuple[int, int]]]],
              target: int) -> set[tuple[tuple[int, int], ...]]:
    out: set[tuple[tuple[int, int], ...]] = set()
    for j in range(line0, len(logs)):
        for sj in (-1, 1):
            for i in range(line0):
                for si in (-1, 1):
                    key = (target - sj * logs[j] - si * logs[i]) % n
                    for pair in pairs[key]:
                        if i == pair[0][0] or i == pair[1][0]:
                            continue
                        out.add(tuple(sorted((*pair, (i, si), (j, sj)))))
    return out


def check(p: int, seed: int) -> dict:
    source = ROOT / f"quartic_twist_k1_scale_p{p}_seed{seed}_v1.json"
    auditor = ROOT / f"quartic_twist_k1_scale_p{p}_seed{seed}_audit_v1.json"
    rec = json.loads(source.read_text())
    audit = json.loads(auditor.read_text())
    assert audit["base_logs_verified"] == len(rec["physical_factor_logs"])
    assert audit["recovered"] == 271
    n = rec["n"]
    logs = rec["physical_factor_logs"]
    line0 = sum(pt[0][2] == 0 for pt in rec["base_coordinates"])
    pairs: dict[int, list[tuple[tuple[int, int], tuple[int, int]]]] = collections.defaultdict(list)
    for i in range(line0):
        for j in range(i + 1, line0):
            for si in (-1, 1):
                for sj in (-1, 1):
                    pairs[(si * logs[i] + sj * logs[j]) % n].append(((i, si), (j, sj)))
    differences = []
    expected_total = polynomial_total = 0
    for item in rec["trace"]:
        a, b = item["a"], item["b"]
        target = (a + 271 * b) % n
        expected = exact_set(logs, line0, n, pairs, target)
        polynomial = {tuple(tuple(term) for term in rel)
                      for rel in item["patterns"][0]["signed_relations"]}
        assert not polynomial - expected, "polynomial emitted a false relation"
        expected_total += len(expected)
        polynomial_total += len(polynomial)
        if expected != polynomial:
            missing = sorted(expected - polynomial)
            differences.append({"query_index": item["query_index"], "a": a, "b": b,
                                "expected": len(expected), "polynomial": len(polynomial),
                                "failed_systems": item["patterns"][0]["system_failures"],
                                "missing_terms": [[list(term) for term in rel] for rel in missing],
                                "missing_l2_peels": sorted({(term[0], term[1])
                                                            for rel in missing for term in rel
                                                            if term[0] >= line0})})
    assert expected_total == audit["expected_signed_relations"]
    assert polynomial_total == audit["polynomial_signed_relations"]
    assert expected_total - polynomial_total == audit["missing_signed_relations"]
    return {"p": p, "seed": seed, "processed_queries": len(rec["trace"]),
            "expected_signed_relations": expected_total,
            "polynomial_signed_relations": polynomial_total,
            "differences": differences,
            "source_sha256": hashlib.sha256(source.read_bytes()).hexdigest(),
            "audit_sha256": hashlib.sha256(auditor.read_bytes()).hexdigest()}


if __name__ == "__main__":
    runs = [check(p, seed) for p in (73, 97) for seed in range(3)]
    out = {"status": "pass", "method": "post-run scalar-keyed diagnosis using independently BSGS-verified logs",
           "runs": runs, "missing_signed_relations": sum(
               r["expected_signed_relations"] - r["polynomial_signed_relations"] for r in runs)}
    (ROOT / "quartic_twist_k1_missing_diagnostic_v1.json").write_text(
        json.dumps(out, sort_keys=True, indent=2) + "\n")
    print(json.dumps({"status": out["status"], "missing_signed_relations": out["missing_signed_relations"],
                      "different_queries": sum(len(r["differences"]) for r in runs)}, sort_keys=True))
